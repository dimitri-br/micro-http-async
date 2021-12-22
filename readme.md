# micro_http_async
## What is it for?
A small, lightweight crate using async to serve web pages or web apis with high performance and low overhead.
## How do I use it?
Firstly, install the crate and dependencies:
```
[dependencies]
micro_http_async = "*"
tokio = "1.11.0"
```
And if you want to support JSON:
```
serde_json = "1.0"
serde = {version = "1.0", features = ["derive"]}
```
This crate is designed to abstract away many of the low level code required to run a safe, asynchronous web server
Here is a small example which shows how to route, use asynchronous call backs and load webpage templates from HTML files.
For the HTML files included, please go to the [repository](https://github.com/dimitribobkov/micro-http-async/) and check the `templates` folder.
Static files also included.
To run the included example (which is the example seen below), run `cargo run --example hello_world`, and visit [127.0.0.1:8080](http://127.0.0.1:8080)
Please note this is probably not the final API

[Examples](https://github.com/dimitribobkov/micro-http-async/tree/master/examples)

This crate aims only to simplify web api or lightweight web creation - not intended to run full scale web apps like chatrooms
or other high intensity applications. It implements a simple asynchronous routing system (Made using hash maps for speed and efficiency)
as well as asynchronous file loading and more.
The demo above uses 0% CPU under no load, and less than 10mb of memory under usage.
It compiles in 1m 34s on an i5 5500u (release) from scratch and sits at 700kb.

Changelog v0.1.3:
Post requests now work fully. Check templates and examples for examples on usage.
hello_world.rs contains a functioning program that can read in post request data, save it to a file and so on. 
The read buffer size for the server is now modifiable by the server. Check hello_world to see how it is used.
A future aim is to add smaller, bite sized examples.
