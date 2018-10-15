use futures;
use futures::future::Future;
use futures::Stream;
use hyper;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Request, Response, Service};
use serde_json::{self, Value};
use tokio_core::reactor::Core;

use std::io;
use std::fs::{self, DirEntry};
use std::net::SocketAddr;

use error::*;

#[derive(Clone)]
struct Recipe {
    addr: SocketAddr,
    recipe: String,
}

impl Recipe {
    fn new(port: io::Result<DirEntry>) -> Result<Self> {
        debug!("Examining recipes for port, {:?}", port);
        let port = port.chain_err(|| format!("Could not unwrap port directory"))?;
        if let Some(port) = port.path().file_name() {
            let port = port.to_string_lossy();
            let port: usize = port.parse()
                .chain_err(|| format!("Could not parse port, {}", port))?;
            let addr = format!("127.0.0.1:{}", port)
                .parse()
                .chain_err(|| "Could not parse address.")?;
            let recipe: Value = json!({
                "message": "Hello"
            });
            let recipe = serde_json::to_string(&recipe)
                .chain_err(|| "Could not stringify")?;
            Ok(Self { addr, recipe })
        } else {
            bail!("Could not find or use port directory")
        }
    }

    fn into_replay(self) -> ReplayResponse {
        ReplayResponse { recipe: self.recipe }
    }
}

#[derive(Clone)]
struct ReplayResponse {
    recipe: String,
}

impl ReplayResponse {
    fn len(&self) -> usize {
        self.recipe.len()
    }

    fn body(&self) -> String {
        self.recipe.clone()
    }
}

impl Service for ReplayResponse {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(futures::future::ok(Response::new()
                                         .with_header(ContentLength(self.len() as u64))
                                         .with_header(ContentType::json())
                                         .with_body(self.body())))
    }
}

pub fn init_responses() -> Result<()> {
    let mut core = Core::new().chain_err(|| "Could not instantiate new core")?;
    let handle = core.handle();
    let ports = fs::read_dir("recipes")
        .chain_err(|| "Could not scan recipes directory")?;
    println!("Created core and handle");
    let http = Http::new();
    println!("Created handle and http");
    let work = ports
        .filter_map(|port| Recipe::new(port).ok())
        .map(|recipe| {
            let addr = recipe.addr.clone();
            println!("Binding replay service, {:?}", addr);
            // convert the recipe so it can be moved into the closure
            let replay = recipe.into_replay();
            // the closure satisfies Fn, i.e. able to be called everywhere, because the moved
            // ReplayResponse is immutable and it clones that struct in order to share it
            // with each call on its particular port
            http.serve_addr_handle(&addr, &handle, move || Ok(replay.clone()))
                .chain_err(|| "Could not bind new http listener")
        })
        .filter_map(|server| server.ok())
        .for_each(|server| core.run(server.for_each(|_| Ok(()))).unwrap());
    println!("Mapped ports into bound listeners");
    //core.run(work).unwrap();
    println!("Core completed");
    Ok(())
}
