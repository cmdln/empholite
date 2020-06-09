///! The server that supports the client developers will use for configuration and the dynamic
///! endpoint other services will call.
mod config;

use actix_files::{Files, NamedFile};
use actix_web::{
    middleware,
    web::{get, Data, Json},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use dotenv::dotenv;
use env_logger::Builder;
use log::{debug, info, trace, LevelFilter};
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, env, io::prelude::*, sync::Mutex};
use time::OffsetDateTime;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    bootstrap();

    let recipes: HashMap<String, String> = HashMap::new();

    let data = Data::new(Mutex::new(recipes));

    let config::ServerConfig {
        bind_address,
        client_bundle_path,
        static_file_path,
    } = config::server_config().unwrap_or_else(|error| panic!("{}", error));
    info!("Starting server, listening at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .route("/favicon", get().to(favicon))
            .route("/favicon.ico", get().to(favicon))
            .route("/pkg/client_bg.wasm", get().to(wasm))
            .service(save_recipe)
            .service(serve_recipe)
            .service(Files::new("/client", &client_bundle_path))
            .service(Files::new("/batch{tail:.*}", &static_file_path).index_file("index.html"))
            .service(Files::new("/draft{tail:.*}", &static_file_path).index_file("index.html"))
            .service(Files::new("/inventory{tail:.*}", &static_file_path).index_file("index.html"))
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
                let today = OffsetDateTime::now_utc();
                let timestamp = today.format("%FT%T");
                let result = json!({
                    "time": timestamp,
                    "@timestamp": format!("{}.{}Z", timestamp, today.nanosecond()),
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

#[derive(Deserialize)]
struct Recipe {
    url: String,
    payload: String,
}

#[actix_web::post("/ajax/recipe/")]
async fn save_recipe(
    recipe: Json<Recipe>,
    data: Data<Mutex<HashMap<String, String>>>,
) -> Result<HttpResponse> {
    let mut data = data.lock().unwrap();
    let Recipe { url, payload } = recipe.into_inner();
    data.insert(url, payload);
    Ok(HttpResponse::Ok().body("Success!"))
}

#[actix_web::get("/api{tail:.*}")]
async fn serve_recipe(
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
