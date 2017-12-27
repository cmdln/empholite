use futures;
use futures::future::Future;
use hyper;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Request, Response, Service};
use serde_json::{self, Value};

use error::*;

// TODO allow contruction with specific json, details
struct ReplayResponse;

impl Service for ReplayResponse {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let json: Value = json!({
            "message": "Hello"
        });
        let json = serde_json::to_string(&json).chain_err(|| "Could not stringify");
        // TODO figure out error handling
        //if let Err(e) = json {
        //    return Box::new(futures::future::err(e));
        //}
        let json = json.unwrap();
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(json.len() as u64))
                .with_header(ContentType::json())
                .with_body(json)
        ))
    }
}
 pub fn replay_response() {
     // TODO get port, route from replay details
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(ReplayResponse)).unwrap();
    // TODO allow for a shutdown signal
    debug!("Started replay service of 3000");
    // TODO figure out how to effectively background this service
    server.run().unwrap();
 }
