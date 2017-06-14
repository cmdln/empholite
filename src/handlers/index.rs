use hyper::header::ContentType;
use iron::middleware::Handler;
use iron::modifiers;
use iron::prelude::*;
use iron::status;
use Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct IndexHandler {
    contents: String,
}

impl IndexHandler {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<IndexHandler> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
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
