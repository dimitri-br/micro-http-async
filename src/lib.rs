//! # micro_http_async
//!
//! ## What is it for?
//! 
//! A small, lightweight crate using async to serve web pages or webapis with high performance and low overhead.
//! 
//! ## How do I use it?
//! 
//! This crate is designed to abstract away many of the low level code required to run a safe, asynchrynous web server
//! 
//! Here is a small example which shows how to route, use asynchrynous callbacks and load webpage templates from HTML files.
//! 
//! Please note this is probably not the final API
//! 
//! ```
//! use micro_http_async::HttpServer;
//! use micro_http_async::Request;
//! use micro_http_async::HtmlConstructor;

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
//! fn main_handler(_request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{
//!    // We wrap the return_str as a future, so we can return it for our routing system to call await on
//!    // This works better than making the whole function a future, since doing that causes race errors.
//!    // By returning a Pinned Boxed future, we define it as a future so it works. Just looks a bit odd
//!    let ret_str = async move { 
//!        let header = "HTTP/1.1 200 OK\r\n\r\n";
//!        let body = HtmlConstructor::construct_page("./templates/index.html").await;
//!        let page = format!("{}{}", header , body);
//!        Ok(page) 
//!    };
//!
//!    return Box::pin(ret_str);
//! }
//!
//! /// We have to define a custom error handler, which defines what to do when we have a 404
//! /// 
//! /// Not doing this WILL result in an unrecoverable panic.
//! fn error_handler(_request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{
//!
//!    let ret_str = async move {       
//!        let header = "HTTP/1.1 404 ERR\r\n\r\n";
//!        let body = HtmlConstructor::construct_page("./templates/err.html").await;
//!        let page = format!("{}{}", header , body);
//!        Ok(page) 
//!    };
//!
//!    return Box::pin(ret_str);
//! }
//!
//! /// # main
//! /// 
//! /// Does what it says, just sets up the server and routes
//! /// 
//! /// then listens for incoming connections
//! #[tokio::main]
//! pub async fn main() {
//!    // Bind the server to a port and IP
//!    let mut http_server = HttpServer::new("127.0.0.1", "8080").await.expect("Error binding to IP/Port");
//!    
//!    // Bind the routes to the callbacks
//!    http_server.routes.add_route("/".to_string(), Box::pin(main_handler)).await;
//!    http_server.routes.add_route("err".to_string(), Box::pin(error_handler)).await;
//! 
//!    // Listen for new connections
//!    http_server.listen().await;
//!}
//! ```
//! 
//! This crate aims only to simplify webapi or lightweight web creation - not intended to run full scale web apps like chatrooms
//! or other high intensity applications. It implements a simple asynchrynous routing system (Made using hashmaps for speed and efficiency)
//! as well as asynchrynous file loading and more. 
//! 
//! The demo above uses 0% CPU under no load, and less than 10mb of memory under usage
#![doc(test(attr(deny(warnings))))]
#![doc(test(no_crate_inject))]

mod server;
mod connection;
mod routes;
mod request;
mod html_loader;

pub use server::HttpServer;
pub use connection::Connection;
pub use routes::Routes;
pub use request::{Request, HttpMethod};
pub use html_loader::{Variable, HtmlConstructor, FileLoader};