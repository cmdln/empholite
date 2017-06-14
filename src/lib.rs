///! The server for the empholite mock service.
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

pub use error::EmpholiteError;
pub use handlers::index::IndexHandler;

mod error;
mod handlers;

pub type Result<T> = std::result::Result<T, EmpholiteError>;
