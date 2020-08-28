pub(crate) mod ajax;
mod db;
pub(crate) mod rest;

use crate::{models::Recipe, DbPool};
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use diesel::prelude::*;
use log::{debug, trace};
use serde_json::{json, Value};
use std::convert::TryInto;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_OFFSET: i64 = 0;
const DEFAULT_LIMIT: i64 = 25;

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

#[actix_web::get("/health")]
pub(crate) async fn health_check(db: Data<DbPool>) -> Result<HttpResponse> {
    let _ = web::block(move || health_query(&db))
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(format!("All is well! ({})", VERSION)))
}

fn health_query(db: &DbPool) -> anyhow::Result<usize> {
    let conn = db.get()?;
    diesel::sql_query("select 1 = 1")
        .execute(&conn)
        .map_err(anyhow::Error::from)
}

fn list_recipes_offset_limit(
    db_pool: Data<DbPool>,
    offset: i64,
    limit: i64,
) -> anyhow::Result<Value> {
    let (total, recipes): (i64, Vec<Recipe>) = db::load_recipes(&db_pool, offset, limit)?;
    let recipes: Vec<shared::Recipe> = recipes
        .into_iter()
        .map(Recipe::try_into)
        .collect::<anyhow::Result<_>>()?;
    Ok(json! {{
        "total": total,
        "offset": offset,
        "limit": limit,
        "recipes": recipes,
    }})
}
