use crate::{
    models::{NewRecipe, NewRule, Recipe, Rule},
    DbPool,
};
use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

pub(super) fn load_recipes(db: &DbPool) -> Result<Vec<Recipe>> {
    use crate::schema::recipes::dsl::*;

    let conn = db.get()?;

    recipes.load::<Recipe>(&conn).map_err(anyhow::Error::from)
}

pub(super) fn find_recipe(db: &DbPool, to_find: Uuid) -> Result<(Recipe, Vec<Rule>)> {
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

pub(super) fn find_recipe_by_url<S: AsRef<str>>(
    db: &DbPool,
    to_find: S,
) -> Result<Vec<(Recipe, Vec<Rule>)>> {
    use crate::schema::{recipes, rules};

    let conn = db.get()?;

    let joined: Vec<(Recipe, Option<Rule>)> = recipes::dsl::recipes
        .filter(recipes::dsl::url.eq(to_find.as_ref()))
        .left_join(rules::dsl::rules)
        .load::<(Recipe, Option<Rule>)>(&conn)?;

    // the query returns a denormalized Vec, meaning that while the rule have of each tuple is
    // distinct, the associated recipe may be repeated; unzip here to re-normalize before returning
    let (mut recipes, rules): (Vec<_>, Vec<_>) = joined.into_iter().unzip();

    // sort followed by deduplicating yields a Vec of recipes that are unique by ID
    recipes.sort_by_key(|recipe| recipe.id);
    recipes.dedup_by_key(|recipe| recipe.id);

    // this fold produces the result Vec who members are a tuple containing each recipe and its
    // associated rules
    let (mut recipes, _) = recipes
        .into_iter()
        // the accumulator is a tuple, the left half is the result Vec containing tuples of recipes
        // and their related rules, the right half is the leftover unprocessed rules, if any
        .fold((Vec::new(), rules), move |acc, recipe| {
            let (mut recipes, rules) = acc;

            // partitioning on the ID means the left half of the resulting tuple is only rules with
            // this iterations recipe ID; the right half is the remaining rules which we pass along
            // in the accumulator
            let (for_recipe, rules): (Vec<Option<_>>, Vec<Option<_>>) =
                rules.into_iter().partition(|rule| {
                    if let Some(rule) = rule {
                        rule.recipe_id == recipe.id
                    } else {
                        false
                    }
                });
            let for_recipe: Vec<Rule> = for_recipe.into_iter().filter_map(|rule| rule).collect();

            // push a new tuple for this iteration and the rules we matched to the recipe
            recipes.push((recipe, for_recipe));

            // return the accumulator to keep working
            (recipes, rules)
        });

    // sort by the number of rules
    recipes.sort_by_key(|(_, rules)| rules.len());
    // descending so the most specific is evaluated first
    recipes.reverse();
    Ok(recipes)
}

pub(super) fn create_recipe(db: &DbPool, to_create: NewRecipe) -> Result<Recipe> {
    use crate::schema::recipes;

    let conn = db.get()?;

    diesel::insert_into(recipes::table)
        .values(to_create)
        .get_result(&conn)
        .map_err(anyhow::Error::from)
}

pub(super) fn create_rules(db: &DbPool, to_create: &[NewRule]) -> Result<Vec<Rule>> {
    use crate::schema::rules;

    let conn = db.get()?;

    diesel::insert_into(rules::table)
        .values(to_create)
        .get_results(&conn)
        .map_err(anyhow::Error::from)
}

pub(super) fn delete_rules(db: &DbPool, parent: Uuid, to_retain: &[Rule]) -> Result<usize> {
    use crate::schema::rules::dsl::*;

    let conn = db.get()?;

    let ids: Vec<Uuid> = to_retain.iter().map(|rule| rule.id).collect();

    diesel::delete(rules.filter(recipe_id.eq(parent).and(id.ne_all(ids))))
        .execute(&conn)
        .map_err(anyhow::Error::from)
}

pub(super) fn update_rules(db: &DbPool, to_update: Vec<Rule>) -> Result<usize> {
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

pub(super) fn update_recipe(
    db: &DbPool,
    to_update: Uuid,
    changed_url: String,
    changed_payload: String,
) -> Result<usize> {
    use crate::schema::recipes::dsl::*;

    let conn = db.get()?;

    let count = diesel::update(recipes.find(to_update))
        .set((url.eq(changed_url), payload.eq(changed_payload)))
        .execute(&conn)?;
    Ok(count)
}
