use crate::{
    models::{NewRecipe, Recipe},
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
use std::{collections::HashMap, sync::Mutex};
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
    let recipe: shared::Recipe = web::block(move || find_recipe(&db, path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)?
        .into();
    Ok(HttpResponse::Ok().json(recipe))
}

#[actix_web::post("/ajax/recipe/")]
pub(crate) async fn upsert_recipe(
    db: Data<DbPool>,
    recipe: Json<shared::Recipe>,
) -> Result<HttpResponse> {
    let shared::Recipe {
        id, url, payload, ..
    } = recipe.into_inner();
    let upserted: shared::Recipe = if let Some(id) = id {
        web::block(move || {
            let count = update_recipe(&db, id, url, payload)?;
            if count == 1 {
                find_recipe(&db, id)
            } else {
                bail!("Unable to update recipe, {}", id)
            }
        })
        .await
        .map_err(ErrorInternalServerError)?
        .into()
    } else {
        let to_created = NewRecipe { url, payload };
        web::block(move || create_recipe(&db, to_created))
            .await
            .map_err(ErrorInternalServerError)?
            .into()
    };
    Ok(HttpResponse::Ok().json(upserted))
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

fn find_recipe(db: &DbPool, to_find: Uuid) -> anyhow::Result<Recipe> {
    use crate::schema::recipes::dsl::*;

    let conn = db.get()?;

    recipes
        .find(to_find)
        .first::<Recipe>(&conn)
        .map_err(anyhow::Error::from)
}

fn create_recipe(db: &DbPool, to_create: NewRecipe) -> anyhow::Result<Recipe> {
    use crate::schema::recipes;

    let conn = db.get()?;

    diesel::insert_into(recipes::table)
        .values(to_create)
        .get_result(&conn)
        .map_err(anyhow::Error::from)
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
