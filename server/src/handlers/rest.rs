use super::db;
use crate::{
    models::{NewRecipe, NewRule, RecipeCascaded},
    DbPool,
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data, Json},
    HttpResponse, Result,
};
use std::convert::TryInto;

#[actix_web::post("/api/recipe")]
pub(crate) async fn create_recipe(
    db_pool: Data<DbPool>,
    recipe: Json<shared::Recipe>,
) -> Result<HttpResponse> {
    let shared::Recipe {
        url,
        payload,
        rules,
        ..
    } = recipe.into_inner();
    let payload = serde_json::to_string(&payload).map_err(ErrorInternalServerError)?;
    let (recipe, rules) = {
        let to_create = NewRecipe { url, payload };
        web::block(move || {
            db::create_recipe(&db_pool, to_create).and_then(|recipe| {
                let to_create: Vec<NewRule> = rules
                    .into_iter()
                    .map(|rule| (recipe.id, rule).into())
                    .collect();
                db::create_rules(&db_pool, &to_create).map(|rules| (recipe, rules))
            })
        })
        .await
        .map_err(ErrorInternalServerError)?
    };
    let created: shared::Recipe = RecipeCascaded(recipe, rules)
        .try_into()
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(created))
}
