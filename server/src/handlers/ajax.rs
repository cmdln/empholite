use super::db;
use crate::{
    config::{self, KeyPathKind},
    models::{NewRecipe, NewRule, RecipeCascaded, Rule},
    DbPool,
};
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    web::{self, Data, Json, Path},
    HttpResponse, Result,
};
use anyhow::{bail, format_err};
use log::debug;
use serde_json::{self, Map, Value};
use std::{convert::TryInto, fs, path::PathBuf};
use uuid::Uuid;

#[actix_web::get("/ajax/recipe/offset/{offset}")]
pub(crate) async fn list_recipes_page(db: Data<DbPool>, offset: Path<i64>) -> Result<HttpResponse> {
    let json = web::block(move || {
        super::list_recipes_offset_limit(db, offset.into_inner(), super::DEFAULT_LIMIT)
    })
    .await
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json))
}

#[actix_web::get("/ajax/recipe/")]
pub(crate) async fn list_recipes(db: Data<DbPool>) -> Result<HttpResponse> {
    let json = web::block(move || {
        super::list_recipes_offset_limit(db, super::DEFAULT_OFFSET, super::DEFAULT_LIMIT)
    })
    .await
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json))
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

#[actix_web::get("/ajax/key_path/{tail:.*}")]
pub(crate) async fn complete_key_path(selected_path: Path<String>) -> Result<HttpResponse> {
    debug!("Key path: {:?}", selected_path);
    let selected_path = selected_path.into_inner();
    let selected: Vec<String> = if selected_path.is_empty() {
        Vec::new()
    } else {
        selected_path
            .split('/')
            .map(ToOwned::to_owned)
            .filter(|c| !c.trim().is_empty())
            .collect()
    };
    debug!("Parsed: {:?}", selected);
    let entries = match config::KEY_PATH_KIND.clone() {
        KeyPathKind::Directory(mut key_path) => {
            let selected_path = PathBuf::from(selected.join("/"));
            key_path.push(selected_path);
            let entries = fs::read_dir(key_path)?;
            let candidates: Result<Vec<shared::KeyPathComponent>> = entries
                .map(|entry| {
                    let entry = entry.map_err(ErrorInternalServerError)?;
                    let leaf = entry.path().is_dir();
                    entry
                        .path()
                        .file_name()
                        .ok_or_else(|| {
                            format_err!(
                                "Could not determine filename for {}",
                                entry.path().display()
                            )
                        })
                        .map(|os_str| os_str.to_os_string())
                        .and_then(|os_string| {
                            os_string.into_string().map_err(|error| {
                                format_err!(
                                    "Filename contained a non-Unicode character! {:?}",
                                    error
                                )
                            })
                        })
                        .map(|component| shared::KeyPathComponent { leaf, component })
                        .map_err(ErrorInternalServerError)
                })
                .collect();
            let candidates = candidates?;
            shared::KeyPathCompletions {
                candidates,
                selected,
                kind: shared::KeyPathKind::Directory,
            }
        }
        KeyPathKind::File(ref key_path) => {
            let key_file = fs::read_to_string(key_path)?;
            let key_material: Value = serde_json::from_str(&key_file)?;
            let key_ref = selected.clone();
            if key_ref.is_empty() {
                let key_material = key_material
                    .as_object()
                    .ok_or_else(|| {
                        format_err!(
                            "The root of the JSON Key file, {}, must be an object!",
                            key_path.display()
                        )
                    })
                    .map_err(ErrorInternalServerError)?;
                into_completions(&key_material, selected)
            } else {
                let nested = key_ref
                    .iter()
                    .fold(Ok(&key_material), |nested, prop| {
                        nested.and_then(|nested| {
                            nested.get(prop).ok_or_else(|| {
                                format_err!(
                                    "\"{}\" is not a valid key reference!",
                                    key_ref.join(".")
                                )
                            })
                        })
                    })
                    .map_err(ErrorBadRequest)?;
                if let Value::Object(nested) = nested {
                    into_completions(nested, selected)
                } else {
                    return Err(ErrorBadRequest(format_err!(
                        "\"{}\" is not a valid key reference!",
                        key_ref.join(".")
                    )));
                }
            }
        }
    };
    Ok(HttpResponse::Ok().json(entries))
}

fn into_completions(
    key_material: &Map<String, Value>,
    selected: Vec<String>,
) -> shared::KeyPathCompletions {
    // TODO set leaf based on value not being an object, i.e. being a scalar or an array
    let candidates = key_material
        .keys()
        .map(|key| shared::KeyPathComponent {
            leaf: false,
            component: key.clone(),
        })
        .collect();
    shared::KeyPathCompletions {
        candidates,
        selected,
        kind: shared::KeyPathKind::File,
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
