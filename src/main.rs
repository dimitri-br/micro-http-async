use http_server::HttpServer;
use http_server::Request;

/// # main handler
/// 
/// main handler is a test to test our route and function callbacks work
/// 
/// And it does!
/// 
/// The way it works is that we run test_handler when we recieve a connection. 
/// 
/// Then, this handler manipulates the request (for post info, or other info etc)
/// 
/// after, we return the response as a string. It is then served to the user.
fn main_handler(_request: Request) -> String{
    let header = "HTTP/1.1 200 OK\r\n\r\n";
    let head = r#"
    <head>
        <title>Async Server</title>
        <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-giJF6kkoqNQ00vy+HMDP7azOuL0xtbfIcaT9wjKHr8RbDVddVHyTfAAsrekwKmP1" crossorigin="anonymous" \>
    </head>"#;
    let body = r#"
        <body class="bg-dark text-light align-middle text-center">
            <h1>Data recieved successfully!</h1>
            <p>Thanks for testing my asynchrynous web server</p>
            <p>This is running from the function!</p>
        </body>"#;

    let ret_str = format!("{}{}{}", header, head, body);

    return ret_str;
}

/// We have to define a custom error handler, which defines what to do when we have a 404
/// 
/// Not doing this WILL result in an unrecoverable panic.
fn error_handler(_request: Request) -> String{
    let header = "HTTP/1.1 404 ERR\r\n\r\n";
    let head = r#"
    <head>
        <title>Async Server</title>
        <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-giJF6kkoqNQ00vy+HMDP7azOuL0xtbfIcaT9wjKHr8RbDVddVHyTfAAsrekwKmP1" crossorigin="anonymous" \>
    </head>"#;
    let body = r#"
        <body class="bg-dark text-light align-middle text-center">
            <h1>Error 404</h1>
            <p>Thanks for testing my asynchrynous web server</p>
            <p>Unfortunately we ran into an issue :(</p>
        </body>"#;

    let ret_str = format!("{}{}{}", header, head, body);

    return ret_str;
}

/// # main
/// 
/// Does what it says, just sets up the server and routes
/// 
/// then listens for incoming connections
#[tokio::main]
pub async fn main() {
    let mut http_server = HttpServer::new("127.0.0.1", "8080").await.unwrap();
    
    // must be placed on heap so it can be allocated at runtime (alternative is static)
    http_server.routes.add_route("/".to_string(), Box::new(main_handler)).unwrap();
    http_server.routes.add_route("err".to_string(), Box::new(error_handler)).unwrap();

    http_server.listen().await;
}