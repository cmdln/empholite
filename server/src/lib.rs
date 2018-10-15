///! The server for the empholite mock service.
extern crate failure;

mod error;
//mod handlers;
//mod replay;

pub use error::Result;
//pub use handlers::index::IndexHandler;
//pub use replay::init_responses;

pub fn bootstrap() -> Result<()> {
    dotenv().ok();

    env_logger::init().unwrap();

    // TODO set up diesel middleware

    bootstrap_config()?;
    // TODO bootstrap another listener for mock routes
    Ok(())
}

fn bootstrap_config() -> Result<()> {
    let client_path = env::var("EMPHOLITE_CLIENT_PATH").unwrap_or("./target/client".to_owned());

    //let mut router = Router::new();

    //router.get("/", IndexHandler::new("./static/index.html")?, "index");

    //let mut mount = Mount::new();
    //mount
    //    .mount("/", router)
    //    .mount("/images", Static::new(Path::new("./static/images/")))
    //    .mount("/client", Static::new(Path::new(&client_path)));

    //// set up request logging
    //let mut chain = Chain::new(mount);

    //let (logger_before, logger_after) = Logger::new(None);
    //chain.link_before(logger_before);
    //chain.link_after(logger_after);

    //debug!("Spinning up web server thread");

    //// iron's listener blocks the current thread so spin it up in its own thread or else we cannot
    //// set up the replay response thread
    //let handler: thread::JoinHandle<Result<()>> = thread::spawn(|| {
    //    let host: String = env::var("EMPHOLITE_CONFIG_HOST").unwrap_or("0.0.0.0".into());
    //    let port: i32 = env::var("EMPHOLITE_CONFIG_PORT")
    //        .unwrap_or("8080".into())
    //        .parse()
    //        .chain_err(|| "Could not parse port as a number.")?;
    //    let address: &str = &format!("{}:{}", host, port);
    //    Iron::new(chain)
    //        .http(address)
    //        .chain_err(move || "Could not start server")?;
    //    Ok(())
    //});

    //debug!("Initializing");
    //// set up a replay response process
    //init_responses()?;

    //// on interruption of the replay process, report any error from the web serving thread
    //if let Ok(result) = handler.join() {
    //    result
    //} else {
    //    bail!("Problem with the main web server thread.")
    //}
}
