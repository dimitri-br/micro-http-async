use std::collections::HashMap;
use std::future::Future;
use crate::Request;
use tokio::io::AsyncReadExt;
use chunked_transfer::Encoder;
use futures::future::BoxFuture;
use std::io::Write;

/// # RouteDef
///
/// This trait creates a helpful function that can convert an asynchrynous function without much user input.
///
/// This cleans up the API quite a bit, only requiring the user to Box the function they want to use.
///
/// Hopefully I figure out macros soon so I can simplify the whole process further to a single macro.
pub trait RouteDef{
    fn call(&self, request: Request) -> BoxFuture<'static, Result<String, String>>;
}
impl<T, F> RouteDef for T where T: Fn(Request) -> F, F: Future<Output = Result<String, String>> + Send + 'static{
    /// # Call
    /// Run the function (defined as being a future of type T), taking in the `request` we want to use
    fn call(&self, request: Request) -> BoxFuture<'static, Result<String, String>> {
       Box::pin(self(request))
    }
}

/// # Route
///
/// This struct defines a `Route`. A route is an address defined on a webserver by a `/`. For example, `localhost/search` - `/search` is the route.
///
/// This struct will store a reference to a function or closure, and will run it automatically when a user visits the corresponding route defined to the function.
pub struct Route{
    function: Box<dyn RouteDef>
}

impl Route{
    /// # New
    /// 
    /// Create a new route, taking in a Boxed function as its input. 
    pub fn new(function: Box<dyn RouteDef>) -> Self{
        Self{
            function
        }
    }

    /// # Run
    ///
    /// Run the function, taking in the request as its input. It will return a corresponding DataType based on the output of the function.
    pub async fn run(&self, request: Request) -> DataType{
        // Check that our function returned an Ok result, and unwrap it after it executes
        if let Ok(v) = self.function.call(request).await{
            DataType::Text(v)
        }else{
            DataType::Text(String::new()) // Err returned, just return nothing
        }
    }
}

/// # DataType
/// 
/// This returns the data type of the response, wrapping the response as well
/// 
/// Used mostly for returning static images as bytes
/// 
/// for example, if you're requesting for a static image from say `/static/img.png`,
/// you would want `Bytes(content)` instead of `Text(content)`. The API already handles
/// this for you, but it is worth keeping in mind how it works behind the scenes
pub enum DataType{
    Text(String),
    Bytes(Vec<u8>)
}

/// # Routes
/// 
/// This struct defines the routes. It uses a hashmap to do this.
/// 
/// `HashMap<Route, Content>` where content is the return content (ie, html or json).
pub struct Routes{
    routes: HashMap::<String, Route>
}

impl Routes{
    /// # New
    /// 
    /// Create a new `Route` struct
    pub async fn new() -> Self{
        Self{
            routes: HashMap::<String, Route>::new()
        }
    }

    /// # Add Route
    /// 
    /// Adds a new route to the routes hashmap. If the route already exists,
    /// its value is updated
    pub async fn add_route(&mut self, route: String, content: Route){
        self.routes.insert(route, content);
    }

    /// # Get Route
    /// 
    /// This function takes in the response string from the `TcpStream` and searches the hashmap
    /// for the callback function associated with the route. It then checks that the route is valid,
    /// and runs it asynchrynously (using the request so that the callback can make use of the request data)
    /// 
    /// This function only runs the callback - handling POST and GET requests is up to the callback.
    /// 
    /// If this function detects a request for static content - which it can only detect if the data is stored in
    /// `/static/`, then it will return early with the static content, and not run any functions.
    pub async fn get_route(&self, request: String, user_addr: std::net::SocketAddr) -> Result<DataType, &str>{
        let request = Request::new(request, user_addr);

        // Handle static files - check if theyre binary or text, and handle appropriately.
        // Probably not the best method but it *works*
        if request.uri.contains("static"){
            let file_path = format!(".{}", request.uri);
            return match tokio::fs::File::open(file_path).await{
                Ok(mut file_handle) => {
                    let mut contents = vec![];
                    file_handle.read_to_end(&mut contents).await.unwrap();
                    
                    let result = String::from("HTTP/1.1 {}\r\nContent-type: image/jpeg;\r\nTransfer-Encoding: chunked\r\n\r\n");
                    let mut result = result.into_bytes();

                    // We split the data into chunks so we don't allocate a ton of data to the stack
                    let chunks = contents.chunks(5);
                    let mut iter_chunks = Vec::<std::io::IoSlice<>>::new();
                    for chunk in chunks{
                        iter_chunks.push(std::io::IoSlice::new(chunk));
                    }

                    // TODO: we need to figure out how to write chunked to the buffer using non-nightly features
                    // Also, it might be worth moving to HTTP 2 for this as chunked is http 1 only
                    let mut encoded = Vec::new();
                    {
                        let mut encoder = Encoder::with_chunks_size(&mut encoded, 8);
                        encoder.write_all_vectored(&mut iter_chunks).unwrap();
                    }
                    result.extend(&encoded);


                    match String::from_utf8(result.clone()){
                        Ok(_) => {
                            let result = String::from("HTTP/1.1 {} {}\r\nContent-type: text/css;\r\nTransfer-Encoding: chunked\r\n\r\n");
                            let mut result = result.into_bytes();
                            result.extend(&encoded);
                            let v = String::from_utf8(result).expect("This should work");
                            return Ok(DataType::Text(v))
                        }
                        Err(_) => {
                            return Ok(DataType::Bytes(result))
                        }
                    }
                }
                Err(e) => {
                    println!("Error loading static content: {}", e);
                    Ok(DataType::Text(String::from("ERROR - CONTENT NOT AVAILABLE")))
                }
            };
        }

        // If not static, handle the request
        let func = match self.routes.get(&request.uri){
            Some(v) => v,
            None => {
                println!("Error - user requested '{}', which does not exist on this server.", request.uri);
                self.routes.get(&"err".to_string()).unwrap()// we assume we've got an error handler
            } 
        };
           
        let result = func.run(request).await;

        Ok(result)
    }
}