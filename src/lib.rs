//! Rust implementation of [`grpc-web`](https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-WEB.md) protocol that allows
//! using [`tonic`](https://crates.io/crates/tonic) in browsers via webassembly.
//!
//! # Usage
//!
//! To use `tonic-web-wasm-client`, you need to add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tonic-web-wasm-client = "0.2"
//! ```
//!
//! ## Example
//! To use `tonic` gRPC clients in browser, compile your code with tonic's `transport` feature disabled (this will disable
//! the default transport layer of tonic). Then initialize the query client as follows:
//!
//! ```rust,ignore
//! use tonic_web_wasm_client::Client;
//!
//! let base_url = "http://localhost:9001"; // URL of the gRPC-web server
//! let query_client = QueryClient::new(Client::new(base_url)); // `QueryClient` is the client generated by tonic
//!
//! let response = query_client.status().await; // Execute your queries the same way as you do with defaule transport layer
//! ```
//!
//! ## Building
//!
//! Since `tonic-web-wasm-client` is primarily intended for use in browsers, a crate that uses `tonic-web-wasm-client`
//! can only be built for `wasm32` target architectures:
//!
//! ```shell
//! cargo build --target wasm32-unknown-unknown
//! ```
//!
//! Other option is to create a `.cargo/config.toml` in your crate repository and add a build target there:
//!
//! ```toml
//! [build]
//! target = "wasm32-unknown-unknown"
//! ```
mod client;
mod error;
mod grpc_response;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use self::{client::Client, error::ClientError};
