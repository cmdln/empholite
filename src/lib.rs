///! The server for the empholite mock service.
#[macro_use]
extern crate error_chain;
extern crate hyper;
#[macro_use]
extern crate iron;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod error;
mod handlers;

pub use handlers::index::IndexHandler;
pub use error::{Result, ResultExt};
