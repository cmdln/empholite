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
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryInto;
use uuid::Uuid;

// TODO remove after re-factoring Recipe::payload into serde_json::Value
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Recipe {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub url: String,
    pub rules: Vec<shared::Rule>,
    pub payload: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
}

#[actix_web::post("/api/recipe")]
pub(crate) async fn create_recipe(
    db_pool: Data<DbPool>,
    recipe: Json<Recipe>,
) -> Result<HttpResponse> {
    let Recipe {
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
