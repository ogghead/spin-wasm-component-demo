# Spin WASM Component Demo

This package is intended as a demo on how to use [Fermyon Spin](https://www.fermyon.com/spin) to seamlessly integrate HTTP middleware components cross-language on Spin components that follow the [WASI HTTP 0.2 specification](https://github.com/WebAssembly/wasi-http)

## Setup

* Install [cargo component](https://github.com/bytecodealliance/cargo-component) to build Rust WASM components
* Set up Spin following the [install guide](https://developer.fermyon.com/spin/v2/install)
  * [Set up Rust component support](https://developer.fermyon.com/spin/v2/rust-components)
  * [Set up Python component support](https://developer.fermyon.com/spin/v2/python-components)

## Building

* Build the auth component with `cargo component build --release`
* Build spin components with `spin build`
* Start the app with `spin up` -- you should now be able to hit the /rust-api and /py-api endpoints to demonstrate linked auth middleware for both.

## Future Goals

* Upload and download WASM component from an OCI registry like <https://www.fermyon.com/blog/announcing-spin-2-6>
* Optimize WASM components + run perf tests
