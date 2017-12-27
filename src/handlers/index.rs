use iron::headers::ContentType;
use iron::middleware::Handler;
use iron::modifiers;
use iron::prelude::*;
use iron::status;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use error::*;

pub struct IndexHandler {
    contents: String,
}

impl IndexHandler {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<IndexHandler> {
        let mut file = File::open(file_path)
            .chain_err(|| "Could not open index.html")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .chain_err(|| "Could not read index.html")?;
        Ok(IndexHandler { contents: contents })
    }
}

impl Handler for IndexHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((modifiers::Header(ContentType::html()),
                           status::Ok,
                           self.contents.as_str())))
    }
}
