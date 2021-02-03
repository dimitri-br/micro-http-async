//! # http_server
//! 
//! This crate aims to simplify webapi work for my own personal uses
//! 
//! Every function and struct has at least some documentation/explaination
//! 
//! Here is a simple example to help you get started: 
//! 
//! ```
//! use http_server::HttpServer;
//! 
//! fn main_handler() -> String{
//!     let header = "HTTP/1.1 200 OK\r\n\r\n";
//!     let head = r#"
//!        <head>
//!             <title>Async Server</title>
//!             <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-giJF6kkoqNQ00vy+HMDP7azOuL0xtbfIcaT9wjKHr8RbDVddVHyTfAAsrekwKmP1" crossorigin="anonymous" \>
//!        </head>"#;
//!     let body = r#"
//!        <body class="bg-dark text-light align-middle text-center">
//!             <h1>Data recieved successfully!</h1>
//!             <p>Thanks for testing my asynchrynous web server</p>
//!             <p>This is running from the function!</p>
//!        </body>"#;
//!
//!   let ret_str = format!("{}{}{}", header, head, body);
//!
//!   return ret_str;
//! }
//! 
//! fn error_handler(_request: Request) -> String{
//!         let header = "HTTP/1.1 404 ERR\r\n\r\n";
//!     let head = r#"
//!     <head>
//!         <title>Async Server</title>
//!         <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-giJF6kkoqNQ00vy+HMDP7azOuL0xtbfIcaT9wjKHr8RbDVddVHyTfAAsrekwKmP1" crossorigin="anonymous" \>
//!     </head>"#;
//!     let body = r#"
//!         <body class="bg-dark text-light align-middle text-center">
//!             <h1>Error 404</h1>
//!             <p>Thanks for testing my asynchrynous web server</p>
//!             <p>Unfortunately we ran into an issue :(</p>
//!         </body>"#;
//! 
//!     let ret_str = format!("{}{}{}", header, head, body);
//! 
//!     return ret_str;
//! }
//! 
//! #[tokio::main]
//! pub async fn main() {
//!     let mut http_server = HttpServer::new("127.0.0.1", "8080").await.unwrap();
//!     
//!     // must be placed on heap so it can be allocated at runtime (alternative is static)
//!     http_server.routes.add_route("/".to_string(), Box::new(main_handler)).unwrap();
//!     http_server.routes.add_route("err".to_string(), Box::new(error_handler)).unwrap();
//! 
//!     http_server.listen().await;
//! }
//! ```
//! 
//! This example shows how to use callbacks (which are functions that run when a route is called), how to run a new server and the general gist of 
//! how everything works.
//! 
//! The documentation per struct/method goes much more in depth.


mod server;
mod connection;
mod routes;
mod request;

pub use server::HttpServer;
pub use connection::Connection;
pub use routes::Routes;
pub use request::{Request, HttpMethod};