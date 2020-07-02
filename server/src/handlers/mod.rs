use crate::{
    models::{NewRecipe, NewRule, Recipe, RecipeCascaded, Rule},
    DbPool,
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data, Json, Path},
    HttpRequest, HttpResponse, Result,
};
use anyhow::bail;
use diesel::prelude::*;
use log::{debug, trace};
use std::convert::TryInto;
use uuid::Uuid;

#[actix_web::get("/ajax/recipe/")]
pub(crate) async fn list_recipes(db: Data<DbPool>) -> Result<HttpResponse> {
    let recipes: Vec<shared::Recipe> = web::block(move || load_recipes(&db))
        .await
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(Recipe::into)
        .collect();
    Ok(HttpResponse::Ok().json(recipes))
}

#[actix_web::get("/ajax/recipe/{id}")]
pub(crate) async fn get_recipe(path: Path<Uuid>, db: Data<DbPool>) -> Result<HttpResponse> {
    let (recipe, rules) = web::block(move || find_recipe(&db, path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)?;
    let body: shared::Recipe = RecipeCascaded(recipe, rules)
        .try_into()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(body))
}

#[actix_web::post("/ajax/recipe/")]
pub(crate) async fn upsert_recipe(
    db: Data<DbPool>,
    recipe: Json<shared::Recipe>,
) -> Result<HttpResponse> {
    let shared::Recipe {
        id,
        url,
        payload,
        rules,
        ..
    } = recipe.into_inner();
    let (recipe, rules) = if let Some(id) = id {
        use shared::Rule::*;
        web::block(move || {
            let count = update_recipe(&db, id, url, payload)?;
            if count == 1 {
                let (to_retain, to_create): (Vec<shared::Rule>, Vec<shared::Rule>) =
                    rules.into_iter().partition(|rule| match rule {
                        Authenticated { id, .. } | Subject { id, .. } => id.is_some(),
                    });
                let to_retain = to_retain
                    .into_iter()
                    .map(|rule| (id, rule).try_into())
                    .collect::<anyhow::Result<Vec<Rule>>>()?;
                let to_create = to_create
                    .into_iter()
                    .map(|rule| (id, rule).into())
                    .collect::<Vec<NewRule>>();
                delete_rules(&db, id, &to_retain)?;
                update_rules(&db, to_retain)?;
                create_rules(&db, &to_create)?;
                find_recipe(&db, id)
            } else {
                bail!("Unable to update recipe, {}", id)
            }
        })
        .await
        .map_err(ErrorInternalServerError)?
    } else {
        let to_created = NewRecipe { url, payload };
        web::block(move || {
            create_recipe(&db, to_created).and_then(|recipe| {
                let to_create: Vec<NewRule> = rules
                    .into_iter()
                    .map(|rule| (recipe.id, rule).into())
                    .collect();
                create_rules(&db, &to_create).map(|rules| (recipe, rules))
            })
        })
        .await
        .map_err(ErrorInternalServerError)?
    };
    let upserted: shared::Recipe = RecipeCascaded(recipe, rules)
        .try_into()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(upserted))
}

#[actix_web::get("/api{tail:.*}")]
pub(crate) async fn serve_recipe(request: HttpRequest, db: Data<DbPool>) -> Result<HttpResponse> {
    let cx_info = request.connection_info();
    let scheme = cx_info.scheme();
    let host = cx_info.host();
    trace!("Scheme {}", scheme);
    trace!("Host {:?}", host);
    let uri = request.uri();
    trace!("URI {:?}", uri);
    let key = format!(
        "{}://{}{}",
        request.connection_info().scheme(),
        host,
        uri.path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or_else(|| "")
    );
    debug!("Recipe key {}", key);
    let to_find = key.clone();
    let recipes = web::block(move || find_recipe_by_url(&db, to_find))
        .await
        .map_err(ErrorInternalServerError)?;
    let recipes = recipes
        .into_iter()
        .map(|(recipe, rules)| recipe.evaluate_rules(&rules, &request))
        // in order for collect to transpose Vec and Result we need the right hint, here, that
        // matches the T and E generic arguments returned by the closure in the map in the line
        // above
        .collect::<anyhow::Result<Vec<Option<String>>>>()
        .map_err(ErrorInternalServerError)?
        // filter map after map_err and ? so that any short circuiting errors bubble out; the
        // result of the remaining chain calls is a Vec of valid, matching recipes
        .into_iter()
        .filter_map(|payload| payload)
        // due to the extended chaining, the compiler needs more help inferring the final type of
        // the whole expression
        .collect::<Vec<String>>();
    if let Some(payload) = recipes.first() {
        Ok(HttpResponse::Ok().body(payload))
    } else {
        Ok(HttpResponse::NotFound().body(format!(
            "Could not find a recipe for requested URI, {}",
            key
        )))
    }
}

fn load_recipes(db: &DbPool) -> anyhow::Result<Vec<Recipe>> {
    use crate::schema::recipes::dsl::*;

    let conn = db.get()?;

    recipes.load::<Recipe>(&conn).map_err(anyhow::Error::from)
}

fn find_recipe(db: &DbPool, to_find: Uuid) -> anyhow::Result<(Recipe, Vec<Rule>)> {
    use crate::schema::recipes::dsl::*;

    let conn = db.get()?;

    let recipe = recipes
        .find(to_find)
        .first::<Recipe>(&conn)
        .map_err(anyhow::Error::from)?;

    let rules: Vec<(Rule, Recipe)> = Rule::belonging_to(&recipe)
        .inner_join(recipes)
        .load::<(Rule, Recipe)>(&conn)?;
    let rules: Vec<Rule> = rules.into_iter().map(|(rule, _)| rule).collect();

    Ok((recipe, rules))
}

fn find_recipe_by_url<S: AsRef<str>>(
    db: &DbPool,
    to_find: S,
) -> anyhow::Result<Vec<(Recipe, Vec<Rule>)>> {
    use crate::schema::{recipes, rules};

    let conn = db.get()?;

    let joined: Vec<(Recipe, Rule)> = recipes::dsl::recipes
        .filter(recipes::dsl::url.eq(to_find.as_ref()))
        .inner_join(rules::dsl::rules)
        .load::<(Recipe, Rule)>(&conn)?;

    // the query returns a denormalized Vec, meaning that while the rule have of each tuple is
    // distinct, the associated recipe may be repeated; unzip here to re-normalize before returning
    let (mut recipes, rules): (Vec<_>, Vec<_>) = joined.into_iter().unzip();

    // sort followed by deduplicating yields a Vec of recipes that are unique by ID
    recipes.sort_by_key(|recipe| recipe.id);
    recipes.dedup_by_key(|recipe| recipe.id);

    // this fold produces the result Vec who members are a tuple containing each recipe and its
    // associated rules
    let (recipes, _) = recipes
        .into_iter()
        // the accumulator is a tuple, the left half is the result Vec containing tuples of recipes
        // and their related rules, the right half is the leftover unprocessed rules, if any
        .fold((Vec::new(), rules), move |acc, recipe| {
            let (mut recipes, rules) = acc;

            // partitioning on the ID means the left half of the resulting tuple is only rules with
            // this iterations recipe ID; the right half is the remaining rules which we pass along
            // in the accumulator
            let (for_recipe, rules) = rules
                .into_iter()
                .partition(|rule| rule.recipe_id == recipe.id);

            // push a new tuple for this iteration and the rules we matched to the recipe
            recipes.push((recipe, for_recipe));

            // return the accumulator to keep working
            (recipes, rules)
        });
    Ok(recipes)
}

fn create_recipe(db: &DbPool, to_create: NewRecipe) -> anyhow::Result<Recipe> {
    use crate::schema::recipes;

    let conn = db.get()?;

    diesel::insert_into(recipes::table)
        .values(to_create)
        .get_result(&conn)
        .map_err(anyhow::Error::from)
}

fn create_rules(db: &DbPool, to_create: &[NewRule]) -> anyhow::Result<Vec<Rule>> {
    use crate::schema::rules;

    let conn = db.get()?;

    diesel::insert_into(rules::table)
        .values(to_create)
        .get_results(&conn)
        .map_err(anyhow::Error::from)
}

fn delete_rules(db: &DbPool, parent: Uuid, to_retain: &[Rule]) -> anyhow::Result<usize> {
    use crate::schema::rules::dsl::*;

    let conn = db.get()?;

    let ids: Vec<Uuid> = to_retain.iter().map(|rule| rule.id).collect();

    diesel::delete(rules.filter(recipe_id.eq(parent).and(id.ne_all(ids))))
        .execute(&conn)
        .map_err(anyhow::Error::from)
}

fn update_rules(db: &DbPool, to_update: Vec<Rule>) -> anyhow::Result<usize> {
    use crate::schema::rules::dsl::*;

    let conn = db.get()?;

    let mut count = 0;
    for rule in to_update {
        count += diesel::update(rules.find(rule.id))
            .set((
                rule_type.eq(rule.rule_type),
                subject.eq(rule.subject),
                key_path.eq(rule.key_path),
            ))
            .execute(&conn)?;
    }
    Ok(count)
}

fn update_recipe(
    db: &DbPool,
    to_update: Uuid,
    changed_url: String,
    changed_payload: String,
) -> anyhow::Result<usize> {
    use crate::schema::recipes::dsl::*;

    let conn = db.get()?;

    let count = diesel::update(recipes.find(to_update))
        .set((url.eq(changed_url), payload.eq(changed_payload)))
        .execute(&conn)?;
    Ok(count)
}
