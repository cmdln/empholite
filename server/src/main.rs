///! The server that supports the client developers will use for configuration and the dynamic
///! endpoint other services will call.
#[macro_use]
extern crate diesel;

mod config;
mod handlers;
mod models;
mod schema;

use actix_files::{Files, NamedFile};
use actix_web::{
    middleware,
    web::{get, Data},
    App, HttpServer, Result,
};
use chrono::Utc;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use env_logger::Builder;
use log::{info, LevelFilter};
use serde_json::json;
use std::{collections::HashMap, env, io::prelude::*, sync::Mutex};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    bootstrap();

    let recipes: HashMap<String, String> = HashMap::new();

    let data = Data::new(Mutex::new(recipes));

    let config::ServerConfig {
        bind_address,
        database_url,
        client_bundle_path,
        static_file_path,
    } = config::server_config().unwrap_or_else(|error| panic!("{}", error));

    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(database_url);
    let pool = Pool::new(manager)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;

    info!("Starting server, listening at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .app_data(data.clone())
            .route("/favicon", get().to(favicon))
            .route("/favicon.ico", get().to(favicon))
            .route("/pkg/client_bg.wasm", get().to(wasm))
            .service(handlers::get_recipe)
            .service(handlers::save_recipe)
            .service(handlers::list_recipes)
            .service(handlers::serve_recipe)
            .service(Files::new("/client", &client_bundle_path))
            .service(Files::new("/add{tail:.*}", &static_file_path).index_file("index.html"))
            .service(Files::new("/view{tail:.*}", &static_file_path).index_file("index.html"))
            .service(Files::new("/", &static_file_path).index_file("index.html"))
    })
    .bind(bind_address)?
    .run()
    .await
}

const ENABLE_JSON_LOGGING: &str = "JSON_LOGGING";

fn bootstrap() {
    // no need for direnv, reads .env as well as any exported environment variables
    dotenv().ok();

    // force init to make errors clear on start up
    config::init();

    let mut builder = Builder::new();
    if let Ok(json_logging) = env::var(ENABLE_JSON_LOGGING) {
        if json_logging.parse().unwrap_or(false) {
            builder.format(|buf, record| {
                let today = Utc::now();
                let time = today.format("%FT%T.%f").to_string();
                let timestamp = today.format("%FT%T").to_string();
                let result = json!({
                    "time": time,
                    "@timestamp": timestamp,
                    "args": format!("{}", record.args()),
                    "level": format!("{}", record.level()),
                    "location": format!("{}:{}",
                                        record.file().unwrap_or("none"),
                                        record.line().unwrap_or(0)),
                    "target": record.target(),
                });
                let line: String = serde_json::to_string(&result)
                    .unwrap_or_else(|_| String::from("Could not translate log to JSON string"));
                writeln!(buf, "{}", line)
            });
        }
    }
    builder.filter(None, LevelFilter::Info);
    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    }
    // initializes logging to stderr
    builder.init();
}

async fn favicon() -> Result<NamedFile> {
    NamedFile::open(config::FAVICON.as_str()).map_err(Into::into)
}

async fn wasm() -> Result<NamedFile> {
    NamedFile::open(config::WASM.as_str()).map_err(Into::into)
}
