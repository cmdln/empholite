///! The server for the empholite mock service.
#[macro_use]
extern crate error_chain;
extern crate futures;
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
mod replay;

pub use error::{Result, ResultExt};
pub use handlers::index::IndexHandler;
pub use replay::replay_response;
