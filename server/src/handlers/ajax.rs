use super::db;
use crate::{
    config::{self, KeyPathKind},
    models::{NewRecipe, NewRule, Recipe, RecipeCascaded, Rule},
    DbPool,
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data, Json, Path},
    HttpResponse, Result,
};
use anyhow::bail;
use log::debug;
use std::convert::TryInto;
use uuid::Uuid;

#[actix_web::get("/ajax/recipe/")]
pub(crate) async fn list_recipes(db: Data<DbPool>) -> Result<HttpResponse> {
    let (_, recipes): (_, Vec<Recipe>) =
        web::block(move || db::load_recipes(&db, super::DEFAULT_OFFSET, super::DEFAULT_LIMIT))
            .await
            .map_err(ErrorInternalServerError)?;
    let recipes: Vec<shared::Recipe> = recipes
        .into_iter()
        .map(Recipe::try_into)
        .collect::<anyhow::Result<_>>()
        .map_err(ErrorInternalServerError)?;
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
    let payload = payload.to_string();
    let (recipe, rules) = if let Some(id) = id {
        use shared::Rule::*;
        web::block(move || {
            let count = db::update_recipe(&db, id, url, payload)?;
            if count == 1 {
                let (to_retain, to_create): (Vec<shared::Rule>, Vec<shared::Rule>) =
                    rules.into_iter().partition(|rule| match rule {
                        Authenticated { id, .. } | Subject { id, .. } | HttpMethod { id, .. } => {
                            id.is_some()
                        }
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

#[actix_web::delete("/ajax/recipe/{id}")]
pub(crate) async fn delete_recipe(db_pool: Data<DbPool>, path: Path<Uuid>) -> Result<HttpResponse> {
    web::block(move || db::delete_recipe(&db_pool, path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().finish())
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
