///! The server for the empholite mock service.
extern crate hyper;
#[macro_use]
extern crate iron;
#[macro_use]
extern crate log;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub use error::EmpholiteError;
pub use index::IndexHandler;

mod error;
mod index;

pub type Result<T> = std::result::Result<T, EmpholiteError>;
