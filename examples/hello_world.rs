/// Small example to show the functionings of the crate. Read the comments to see how everything 
/// functions

use micro_http_async::{HttpServer, JSONResponse};
use micro_http_async::Request;
use micro_http_async::HtmlConstructor;
use micro_http_async::Vars;
use micro_http_async::Variable;
use micro_http_async::Response;

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
fn main_handler(request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{    
    // We wrap the return_str (the actual response we want to return to the user) as a future, so we can return it for our routing 
    // system to use asynchrynously.
    // Hopefully in future this will work better, but this is the only reliable way I found to store futures in the routing system.
    // By returning a Pinned Boxed future, we define it as a future so it works. Just looks a bit odd
    let return_future = async move { 
        println!("{:?} -> {:?} {:?}", request.user_addr, request.method.unwrap(), request.uri);


        let mut vars = Vars::new();
        let test_string = "This string will be outputted dynamically to the web page!".to_string();
        
        vars.insert("test_var".to_string(), Variable::String(test_string));

        // This part will check we have a get request parameter with "name"
        // If we do, we will set a dynamic variable to the key value.
        // It will show how to handle get request parameters
        if request.get_request.contains_key("name"){
            let name = format!("Hello, {}!", request.get_request.get("name").unwrap().to_string());
            vars.insert("name".to_string(), Variable::String(name));
        }else{
            vars.insert("name".to_string(), Variable::String("".to_string()));
        }


        let page = HtmlConstructor::construct_page(Response::from(200), "./templates/index.html", vars).await;

        Ok(page) 
    };

    return Box::pin(return_future);
}


/// We have to define a custom error handler, which defines what to do when we have a 404
/// 
/// Not doing this WILL result in an unrecoverable panic.
fn error_handler(request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{
    let return_future = async move {      
        println!("{:?} -> {:?} {:?}", request.user_addr, request.method.unwrap(), request.uri);

        let mut vars = Vars::new();
        let test_string = format!("Could not load webpage at <code>127.0.0.1:8080{}</code>", request.uri);
        vars.insert("uri".to_string(), Variable::String(test_string));

        let page = HtmlConstructor::construct_page(Response::ClientErr, "./templates/err.html", vars).await;
        
        Ok(page) 
    };

    return Box::pin(return_future);
}

// If we choose to use JSON (eg, for APIs), we can use the following.
// We define the JSON as a rust struct, allowing us to represent it through rust. We serialize this
// using serde.
#[derive(serde::Serialize, serde::Deserialize)]
struct TestResponse{
    pub name: String,
}

// Then, when we handle the response, we convert the Struct using serde_json. We use the JSONResponse class to create
// a response we can send back to the user, using that weird looking return_future method and the box::pins lol.
fn json_response_handler(request: Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send>>{
    let return_future = async move {    
        println!("{:?} -> {:?} {:?}", request.user_addr, request.method.unwrap(), request.uri);
  
        let json = serde_json::json!(
            TestResponse{
                name: "Hello, world!".into()
            }
        );
        let page = JSONResponse::construct_response(Response::Ok, json.to_string()).await;
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
    http_server.routes.add_route("/json".to_string(), Box::pin(json_response_handler)).await;

    http_server.listen().await;
}