//! # micro_http_async
//! ## What is it for?
//! A small, lightweight crate using async to serve web pages or web apis with high performance and low overhead.
//! ## How do I use it?
//! Firstly, install the crate and dependencies:
//! ```
//! [dependencies]
//! micro_http_async = "*"
//! tokio = "1.11.0"
//! ```
//! And if you want to support JSON:
//! ```
//! serde_json = "1.0"
//! serde = {version = "1.0", features = ["derive"]}
//! ```
//! This crate is designed to abstract away many of the low level code required to run a safe, asynchronous web server
//! Here is a small example which shows how to route, use asynchronous call backs and load webpage templates from HTML files.
//! For the HTML files included, please go to the [repository](https://github.com/dimitribobkov/micro-http-async/) and check the `templates` folder.
//! Static files also included.
//! To run the included example (which is the example seen below), run `cargo run --example hello_world`, and visit [127.0.0.1:8080](http://127.0.0.1:8080)
//! Please note this is probably not the final API
//! 
//! [Examples](https://github.com/dimitribobkov/micro-http-async/tree/master/examples)
//! 
//! This crate aims only to simplify web api or lightweight web creation - not intended to run full scale web apps like chatrooms
//! or other high intensity applications. It implements a simple asynchronous routing system (Made using hash maps for speed and efficiency)
//! as well as asynchronous file loading and more.
//! 
//! It also supports TLS if security is a requirement through the [rustls](https://github.com/rustls/rustls) and [tokio-rustls](https://github.com/rustls/rustls) crates.
//! 
//! Changelog v0.1.4:
//! TLS support is now available. See the hello_world example for an example on usage.


#![doc(test(attr(deny(warnings))))]
#![doc(test(no_crate_inject))]
// This is a workaround while we wait for the feature to become stable
#![feature(write_all_vectored)]
#![feature(string_remove_matches)]

mod connection;
mod html_loader;
mod json_response;
mod request;
mod response;
mod routes;
mod server;

pub use connection::Connection;
pub use html_loader::{FileLoader, HtmlConstructor, Variable, Vars};
pub use json_response::JSONResponse;
pub use request::{HttpMethod, Request};
pub use response::Response;
pub use routes::Routes;
pub use routes::{DataType, Route, RouteDef};
pub use server::HttpServer;

/* Define Macros */

/// # Create Route
///
/// This macro takes in an async function, and outputs a Route that can be used when setting up routing
#[macro_export]
macro_rules! create_route {
    ($inc:expr) => {{
        use micro_http_async::Route;
        Route::new(Box::new($inc))
    }};
}
