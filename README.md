Empholite
=========

A modern mock server. A mock server is designed to stand in for other services when performing integration test. It should be able to return all kinds of canned responses for success, failure and edge cases. A modern mock server should be lightweight, easily configurable, and highly scalable. In looking at the field of mock servers already available, none of them seem very modern, being built on heavy weight technologies, requiring a stiff learning curve to get a simple test case running.

empholite is relatively stateless. You can spin up and configure as needed. It has a UI and will have a REST API for describing call and response patterns. SDETs can build up patterns interactively and then use them as part of exploratory testing. Or test runners can make simple REST calls to set needed response patterns, individual or in bulk, as part of test setup and tear down. empholite is able to mock multiple services so that tests can focus on the service under test more easily instead of compounding the costs, in effort and resources, of providing a live test service ecosystem or even multiple mock servers for each replaced live service.

## How It Works

empholite has an endpoint that responds to any call with a path starting with `/api`. empholite looks up applicable recipes based on the full path and on the requested host. You may use host headers or host aliasing, both will work. All rules for a matching path will be tried in order from the most specific to the least specific. Specificity is based right now on the number of rules--more rules means more specific. Each rule is tested, if all rules pass, then the associated payload for the recipe is served.

## Recipes

A recipe has an endpoint, which is a full url including scheme and hostname so that matches can be done based on host aliasing. For instance, `http://foo.com/api/foo` and `http://bar.com/api/foo` are distinct; even though they have the same patch, only calls to `foo.com` will match the recipe including that hostname in the specified endpoint. A recipe may have zero or more rules. Rules are evaluated against the request, if a rule fails, evaluation halts and empholite tries the next recipe that for the given endpoint, if there is one, or if no recipes match based on the evaluated rules, a 404 is returned. Recipes include a payload which right now are literal JSON.

## Rules

* Authenticated call - In order for this rule to match, it expects an "Authorization" header whose value is "Bearer <a base64 encoded JWT>". You must specific a public key for this rule. If `KEY_PATH_KIND` is "file" then `KEY_PATH` must be the location to a JSON file and the rule must have a valid property path, for example `public.auth.001`, to a PEM encoded string value of the key. If `KEY_PATH_KIND` is "directory" then `KEY_PATH` must be a directory and the rule must have a value that is a relative path from this directory to a PEM encoded public key file. The public key is used to verify the signature on the JWT.
* Subject - In order for this rule to match, it expects an "Authorization" header, just like the authenticated call rule. The rule must have a subject value and will only match if the decoded JWT from the auth header contains a "subject" claim that matches the rule's subject value.

## To Do

* [ ] Add REST API
* [ ] Add the ability to proxy between two live services, recording calls and responses which can be copied and edited to create new recipes.
* [ ] Add ability to use arbitrary response status codes in a recipe.
* [ ] Add a rule to match arbitrary header matches.
* [ ] Add support for path parameters.
* [ ] Add support for variable replacement in the payload, for instance to use a path parameter as a value.
* [ ] Add other content types for payloads.
* [ ] Improve the mock endpoint; use a middleware instead so any path may be used.

## Setup

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

### Database

empholite requires a backing database and currently only supports postgres. You can run postgres natively or use the official Docker image. Once you have a running database server, use `createuser` to create a user to own the empholite database, then use `createdb` to create a database named `empholite`. Set DATABASE_URL to something like `postgres://user:password@localhost:5432/empholite` then from the root of the project, to set up the database run

```
$ cargo install --no-default-features diesel_cli --features postgres
$ diesel migration run
```

## Config

empholite can be configured with a few environment variables.

* `DATABASE_URL` - This is required and of the form `postgres://user:password@localhost:5432/empholite`.
* `KEY_PATH` - This is optional and used with the JWT verification rule. This may point to a directory or a file. See `KEY_PATH_KIND` and `KEY_REF` for more.
* `KEY_PATH_KIND` - Set this to either "file" or "directory", defaults to "directory".
* `HOST` - Optional, defaults to "0.0.0.0".
* `PORT` - Optional, defaults to "8989".
* `STATIC_PATH` - Optional, path to static assets required by the client. Defaults to the expected path in a local working copy of the git repo. Provided in case you create your own Docker image.
* `CLIENT_PATH` - Optional, path to client bundle and associated files. Defaults to the expected path in a local working copy of the git repo. Provided in case you create your own Docker image.

## Usage

You need to build the client bundle before running.

```bash
$ make client/pkg/bundle.js
```

This rule will use `wasm-pack` to build the client Rust code as a WebAssembly library then use `rollup` to create a bundle that can be loaded in the browser.

```bash
$ cargo run -p empholite
```

This will run the server which will serve the static assets and the client bundle out of the local working copy. The server listens on port 8989 by default so you can access it at [http://localhost:8989](http://localhost:8989).

## Docker

You can pull the image, `cmdln/empholite`. The image contains the binary for the server, the client bundle, and all static assets. All configuration except `DATABASE_URL` are set based on the contents of the image.

## Logo

Topaz Crystal by Joshua Nichol from the Noun Project
