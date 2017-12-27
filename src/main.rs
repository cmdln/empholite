extern crate empholite;
extern crate dotenv;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;
extern crate router;
extern crate staticfile;

use empholite::{IndexHandler, Result, ResultExt};
use dotenv::dotenv;
use iron::prelude::*;
use logger::Logger;
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::env;
use std::path::Path;

fn main() {
    match bootstrap() {
        Ok(_) => {}
        Err(e) => {
            debug!("Error: {:?}", e);
            error!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn bootstrap() -> Result<()> {
    dotenv().ok();

    env_logger::init().unwrap();

    // TODO set up diesel middleware

    bootstrap_config()?;
    // TODO bootstrap another listener for mock routes
    Ok(())
}

fn bootstrap_config() -> Result<()> {
    let host: String = env::var("EMPHOLITE_CONFIG_HOST").unwrap_or("0.0.0.0".into());
    let port: i32 = env::var("EMPHOLITE_CONFIG_PORT")
        .unwrap_or("8080".into())
        .parse()
        .chain_err(|| "Could not parse port as a number.")?;
    let client_path = env::var("EMPHOLITE_CLIENT_PATH").unwrap_or("./target/client".to_owned());

    let mut router = Router::new();

    router.get("/", IndexHandler::new("./static/index.html")?, "index");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/images", Static::new(Path::new("./static/images/")))
        .mount("/client", Static::new(Path::new(&client_path)));

    // set up request logging
    let mut chain = Chain::new(mount);

    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let address: &str = &format!("{}:{}", host, port);
    Iron::new(chain)
        .http(address)
        .chain_err(|| "Could not start server")?;
    Ok(())
}
