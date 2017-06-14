Empholite
=========
A mock server.

The idea of a mock server is that you can configure arbitrary responses to arbitrary endpoints. When working with micro-services you do not always want or need to bootstrap the whole ecosystem. A mock server is a useful tool when working on a single microservice that relies on others.

## Install

If you do not have the Rust [toolchain](https://www.rustup.rs/), install by running the following in your terminal:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Windows users: follow the [link](https://www.rustup.rs/) to the rustup project for installation options.

You will need `npm` to build the client bundle, on a Mac you can use Homebrew; on Linux, you can use apt.

```bash
$ npm install
```

This will install all the dependencies for the client bundle.

## Usage

```bash
$ npm start
```

This will build the client bundle then compile the server and start it.

## Logo
Topaz Crystal by Joshua Nichol from the Noun Project
