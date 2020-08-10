use super::db;
use crate::{
    config::{self, KeyPathKind},
    models::{NewRecipe, NewRule, Recipe, RecipeCascaded, Rule},
    DbPool,
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data, Json, Path},
    HttpRequest, HttpResponse, Result,
};
use anyhow::bail;
use log::{debug, trace};
use std::convert::TryInto;
use uuid::Uuid;

#[actix_web::get("/ajax/recipe/")]
pub(crate) async fn list_recipes(db: Data<DbPool>) -> Result<HttpResponse> {
    let recipes: Vec<shared::Recipe> = web::block(move || db::load_recipes(&db))
        .await
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(Recipe::into)
        .collect();
    Ok(HttpResponse::Ok().json(recipes))
}

#[actix_web::get("/ajax/recipe/{id}")]
pub(crate) async fn get_recipe(path: Path<Uuid>, db: Data<DbPool>) -> Result<HttpResponse> {
    let (recipe, rules) = web::block(move || db::find_recipe(&db, path.into_inner()))
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
            let count = db::update_recipe(&db, id, url, payload)?;
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
                db::delete_rules(&db, id, &to_retain)?;
                db::update_rules(&db, to_retain)?;
                db::create_rules(&db, &to_create)?;
                db::find_recipe(&db, id)
            } else {
                bail!("Unable to update recipe, {}", id)
            }
        })
        .await
        .map_err(ErrorInternalServerError)?
    } else {
        let to_create = NewRecipe { url, payload };
        web::block(move || {
            db::create_recipe(&db, to_create).and_then(|recipe| {
                let to_create: Vec<NewRule> = rules
                    .into_iter()
                    .map(|rule| (recipe.id, rule).into())
                    .collect();
                db::create_rules(&db, &to_create).map(|rules| (recipe, rules))
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
    let recipes = web::block(move || db::find_recipe_by_url(&db, to_find))
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

#[actix_web::get("/ajax/config")]
pub(crate) async fn get_config() -> Result<HttpResponse> {
    debug!("KEY_PATH_KIND={:?}", std::env::var("KEY_PATH_KIND"));
    debug!("KeyPathKind={:?}", *config::KEY_PATH_KIND);
    let config = shared::Config {
        key_path_kind: if let KeyPathKind::Directory(_) = *config::KEY_PATH_KIND {
            shared::KeyPathKind::Directory
        } else {
            shared::KeyPathKind::File
        },
    };
    Ok(HttpResponse::Ok().json(config))
}
