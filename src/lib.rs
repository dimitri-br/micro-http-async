//! # micro_http_async
//! ## What is it for?
//! 
//! A small, lightweight crate using async to serve web pages or webapis with high performance and low overhead.
//! 
//! ## How do I use it?
//! 
//! Firstly, install the crate and dependencies:
//! 
//! ```
//! [dependencies]
//! micro_http_async = "*"
//! tokio = "1.1"
//! ```
//! This crate is designed to abstract away many of the low level code required to run a safe, asynchrynous web server
//! 
//! Here is a small example which shows how to route, use asynchrynous callbacks and load webpage templates from HTML files.
//! 
//! For the HTML files included, please go to the [repository](https://github.com/dimitribobkov/micro-http-async/) and check the `templates` folder.
//! 
//! Static files also included.
//! 
//! To run the included example (which is the example seen below), run `cargo run --example hello_world`, and visit [127.0.0.1:8080](http://127.0.0.1:8080)
//! 
//! Please note this is probably not the final API
//! 
//! **Example**
//! ```
//! /// Small example to show the functionings of the crate. Read the comments to see how everything 
//! /// functions
//! 
//! use micro_http_async::HttpServer;
//! use micro_http_async::Request;
//! use micro_http_async::HtmlConstructor;
//! use micro_http_async::Vars;
//! use micro_http_async::Variable;
//! use micro_http_async::Response;
//! 
//! /// # main handler
//! /// 
//! /// main handler is a test to test our route and function callbacks work
//! /// 
//! /// And it does!
//! /// 
//! /// The way it works is that we run test_handler when we recieve a connection. 
//! /// 
//! /// Then, this handler manipulates the request (for post info, or other info etc)
//! /// 
//! /// after, we return the response as a string. It is then served to the user.
//! /// 
//! /// The syntax is a bit weird but if it works it works. I'll try fix it :')
//! /// 
//! /// It should return a pinned box future result that implements send
//! fn main_handler(request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{
//!     println!("REQ: {:?}", request.raw_request);
//!     // We wrap the return_str as a future, so we can return it for our routing system to call await on
//!     // This works better than making the whole function a future, since doing that causes race errors.
//!     // By returning a Pinned Boxed future, we define it as a future so it works. Just looks a bit odd
//!     let return_future = async move { 
//!         let mut vars = Vars::new();
//!         let test_string = "This string will be outputted dynamically to the web page!".to_string();
//!         
//!         vars.insert("test_var".to_string(), Variable::String(test_string));
//! 
//!         // This part will check we have a get request parameter with "name"
//!         // If we do, we will set a dynamic variable to the key value.
//!         // It will show how to handle get request parameters
//!         if request.get_request.contains_key("name"){
//!             let name = format!("Hello, {}!", request.get_request.get("name").unwrap().to_string());
//!             vars.insert("name".to_string(), Variable::String(name));
//!         }else{
//!             vars.insert("name".to_string(), Variable::String("".to_string()));
//!         }
//! 
//! 
//!         let page = HtmlConstructor::construct_page(Response::from(200), "./templates/index.html", vars).await;
//! 
//!         Ok(page) 
//!     };
//! 
//!     return Box::pin(return_future);
//! }
//! 
//! 
//! /// We have to define a custom error handler, which defines what to do when we have a 404
//! /// 
//! /// Not doing this WILL result in an unrecoverable panic.
//! fn error_handler(request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{
//!     println!("Connection error!");
//! 
//!     println!("Get: {:?}", request.raw_request);
//!     let return_future = async move {      
//!         let mut vars = Vars::new();
//!         let test_string = format!("Could not load webpage at <code>127.0.0.1:8080{}</code>", request.uri);
//!         vars.insert("uri".to_string(), Variable::String(test_string));
//! 
//!         let page = HtmlConstructor::construct_page(Response::ClientErr, "./templates/err.html", vars).await;
//!         
//!         Ok(page) 
//!     };
//! 
//!     return Box::pin(return_future);
//! }
//! 
//! /// # main
//! /// 
//! /// Does what it says, just sets up the server and routes
//! /// 
//! /// then listens for incoming connections
//! #[tokio::main]
//! pub async fn main() {
//!     let mut http_server = HttpServer::new("127.0.0.1", "8080").await.expect("Error binding to IP/Port");
//!     
//!     // must be placed on heap so it can be allocated at runtime (alternative is static)
//!     http_server.routes.add_route("/".to_string(), Box::pin(main_handler)).await;
//!     http_server.routes.add_route("err".to_string(), Box::pin(error_handler)).await;
//! 
//!     http_server.listen().await;
//! }
//! ```
//! 
//! This crate aims only to simplify webapi or lightweight web creation - not intended to run full scale web apps like chatrooms
//! or other high intensity applications. It implements a simple asynchrynous routing system (Made using hashmaps for speed and efficiency)
//! as well as asynchrynous file loading and more. 
//! 
//! The demo above uses 0% CPU under no load, and less than 10mb of memory under usage.
//! 
//! It compiles in 1m 34s on an i5 5500u (release) from scratch and sits at 700kb.
//! 
//! Changelog v0.1.0:
//! 
//! Fixed some issues with stackoverflows when loading static content like images. 
//! A workaround was found, however for the time being you will need to use `nightly`
//! for this crate. Also I've not figured out a better way to store futures yet, but do 
//! feel free to open an issue/contribute if you know a better way!
#![doc(test(attr(deny(warnings))))]
#![doc(test(no_crate_inject))]

// This is a workaround while we wait for the feature to become stable
#![feature(write_all_vectored)]

mod server;
mod connection;
mod routes;
mod request;
mod html_loader;
mod json_response;
mod response;

pub use server::HttpServer;
pub use connection::Connection;
pub use routes::Routes;
pub use request::{Request, HttpMethod};
pub use html_loader::{Variable, HtmlConstructor, FileLoader, Vars};
pub use response::Response;
pub use routes::DataType;
pub use json_response::JSONResponse;