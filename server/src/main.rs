extern crate empholite;
#[macro_use]
extern crate error_chain;
extern crate dotenv;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mount;
extern crate router;
extern crate staticfile;

use dotenv::dotenv;
use iron::prelude::*;
use logger::Logger;
use mount::Mount;
use router::Router;
use staticfile::Static;

use std::env;
use std::path::Path;
use std::thread;

use empholite::{self, Result};

fn main() {
    match empholite::bootstrap() {
        Ok(_) => {}
        Err(e) => {
            debug!("Error: {:?}", e);
            error!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
