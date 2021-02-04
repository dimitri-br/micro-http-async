use micro_http_async::HttpServer;
use micro_http_async::Request;
use micro_http_async::HtmlConstructor;
use micro_http_async::Vars;
use micro_http_async::Variable;

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
/// 
/// The syntax is a bit weird but if it works it works. I'll try fix it :')
/// 
/// It should return a pinned box future result that implements send
fn main_handler(_request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{
    // We wrap the return_str as a future, so we can return it for our routing system to call await on
    // This works better than making the whole function a future, since doing that causes race errors.
    // By returning a Pinned Boxed future, we define it as a future so it works. Just looks a bit odd
    let return_future = async move { 
        let mut vars = Vars::new();
        let test_string = "This string will be outputted dynamically to the web page!".to_string();

        vars.insert("test_var".to_string(), Variable::String(test_string));

        let header = "HTTP/1.1 200 OK\r\n\r\n";
        let body = HtmlConstructor::construct_page("./templates/index.html", vars).await;
        let page = format!("{}{}", header , body);
        Ok(page) 
    };

    return Box::pin(return_future);
}

/// We have to define a custom error handler, which defines what to do when we have a 404
/// 
/// Not doing this WILL result in an unrecoverable panic.
fn error_handler(request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{

    let return_future = async move {      
        let mut vars = Vars::new();
        let test_string = format!("Could not load webpage at <code>127.0.0.1:8080{}</code>", request.uri);
        vars.insert("uri".to_string(), Variable::String(test_string));

        let header = "HTTP/1.1 404 ERR\r\n\r\n";
        let body = HtmlConstructor::construct_page("./templates/err.html", vars).await;
        let page = format!("{}{}", header , body);
        Ok(page) 
    };

    return Box::pin(return_future);
}

/// # main
/// 
/// Does what it says, just sets up the server and routes
/// 
/// then listens for incoming connections
#[tokio::main]
pub async fn main() {
    let mut http_server = HttpServer::new("127.0.0.1", "8080").await.expect("Error binding to IP/Port");
    
    // must be placed on heap so it can be allocated at runtime (alternative is static)
    http_server.routes.add_route("/".to_string(), Box::pin(main_handler)).await;
    http_server.routes.add_route("err".to_string(), Box::pin(error_handler)).await;

    http_server.listen().await;
}