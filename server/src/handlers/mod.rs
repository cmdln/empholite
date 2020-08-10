pub(crate) mod ajax;
mod db;
pub(crate) mod rest;

use crate::DbPool;
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Data},
    HttpResponse, Result,
};
use diesel::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
