Empholite
=========
A modern mock server. A mock server is designed to stand in for other services when performing integration test. It should be able to return all kinds of canned responses for success, failure and edge cases. A modern mock server should be lightweight, easily configurable, and highly scalable. In looking at the field of mock servers already available, none of them seem very modern, being built on heavy weight technologies, requiring a stiff learning curve to get a simple test case running.

The plan for empholite is for it to be relatively stateless, spun up and configured as needed. It will have both a UI and a REST API for describing call and response patterns. SDETs can build up patterns interactively then export configs to be version controlled and bulk loaded when needed. Or test runners can make simple REST calls to set needed response patterns, individual or in bulk, as part of test setup and tear down. It will be able to mock multiple services so that tests can focus on the service under test more easily instead of compounding the costs, in effort and resources, of providing a live test service ecosystem or even multiple mock servers for each replaced live service.

## Install

If you do not have the Rust [toolchain](https://www.rustup.rs/), install by running the following in your terminal:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Windows users: follow the [link](https://www.rustup.rs/) to the rustup project for installation options.

You will need `wasm-pack` and `rollup` to build and bundle the client.

```bash
$ cargo install wasm-pack
$ npm i -g rollup
```

## Usage

```bash
$ make client/pkg/bundle.js
```

This rule will use `wasm-pack` to build the client Rust code as a WebAssembly library then use `rollup` to create a bundle that can be loaded in the browser.

```bash
$ cargo run -p empholite
```

This will run the server which will serve the static assets and the client bundle out of the local working copy. The server listens on port 8989 by default so you can access it at [http://localhost:8989](http://localhost:8989).

## Logo

Topaz Crystal by Joshua Nichol from the Noun Project
