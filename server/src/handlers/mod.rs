use crate::{models::Recipe, DbPool};
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data, Json},
    HttpRequest, HttpResponse, Result,
};
use diesel::prelude::*;
use log::{debug, trace};
use std::{collections::HashMap, sync::Mutex};

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

#[actix_web::post("/ajax/recipe/")]
pub(crate) async fn save_recipe(
    recipe: Json<shared::Recipe>,
    data: Data<Mutex<HashMap<String, String>>>,
) -> Result<HttpResponse> {
    let mut data = data.lock().unwrap();
    let shared::Recipe { url, payload, .. } = recipe.into_inner();
    data.insert(url, payload);
    Ok(HttpResponse::Ok().body("Success!"))
}

#[actix_web::get("/api{tail:.*}")]
pub(crate) async fn serve_recipe(
    request: HttpRequest,
    data: Data<Mutex<HashMap<String, String>>>,
) -> Result<HttpResponse> {
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
    let data = data.lock().unwrap();
    trace!("Recipes {:?}", data);
    if let Some(payload) = data.get(&key) {
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
