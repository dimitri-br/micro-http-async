/// Small example to show the functionings of the crate. Read the comments to see how everything 
/// functions

// All the imports we need
use micro_http_async::{HttpServer, JSONResponse};
use micro_http_async::Request;
use micro_http_async::HtmlConstructor;
use micro_http_async::Vars;
use micro_http_async::Variable;
use micro_http_async::Response;
use micro_http_async::Route;

// Macros
use micro_http_async::create_route;

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
async fn main_handler(request: Request) -> Result<String, String>{    
    println!("{:?} -> {:?} {:?}", request.user_addr, request.method.unwrap(), request.uri);


    // Setup vars, which will define how vars are set in the page
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

    // Construct the page. We need the response code and page to submit, as well as vars to set. It returns the full page including headers.
    let page = HtmlConstructor::construct_page(Response::from(200), "./templates/index.html", vars).await;

    // Return the page as a Result
    Ok(page) 
}


/// We have to define a custom error handler, which defines what to do when we have a 404
/// 
/// Not doing this WILL result in an unrecoverable panic.
async fn error_handler(request: Request) -> Result<String, String>{ 
    println!("{:?} -> {:?} {:?}", request.user_addr, request.method.unwrap(), request.uri);

    let mut vars = Vars::new();
    let test_string = format!("Could not load webpage at <code>127.0.0.1:8080{}</code>", request.uri);
    vars.insert("uri".to_string(), Variable::String(test_string));

    let page = HtmlConstructor::construct_page(Response::ClientErr, "./templates/err.html", vars).await;
    
    Ok(page) 
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
async fn json_response_handler(request: Request) -> Result<String, String>{ 
    println!("{:?} -> {:?} {:?}", request.user_addr, request.method.unwrap(), request.uri);
  
    let json = serde_json::json!(
        TestResponse{
            name: "Hello, world!".into()
        }
    );

    // This differs from the HTMLConstructor, as we don't take vars as an input
    let page = JSONResponse::construct_response(Response::Ok, json.to_string()).await;
    Ok(page) 
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
    
    http_server.routes.add_route("/".to_string(), create_route!(main_handler)).await; // Use the macro
    http_server.routes.add_route("err".to_string(), Route::new(Box::new(error_handler))).await; // Do it manually
    http_server.routes.add_route("/json".to_string(), create_route!(json_response_handler)).await;

    http_server.listen().await;
}