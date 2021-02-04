use std::collections::HashMap;
use std::future::Future;
use crate::Request;

/// # Routes
/// 
/// This struct defines the routes. It uses a hashmap to do this.
/// 
/// `HashMap<Route, Content>` where content is the return content (ie, html or json).
pub struct Routes{
    routes: HashMap::<String, std::pin::Pin<Box<dyn Fn(Request) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + Send>>>>>
}

impl Routes{
    /// # New
    /// 
    /// Create a new `Route` struct
    pub async fn new() -> Self{
        Self{
            routes: HashMap::<String, std::pin::Pin<Box<dyn Fn(Request) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + Send>>>>>::new()
        }
    }

    /// # Add Route
    /// 
    /// Adds a new route to the routes hashmap. If the route already exists,
    /// its value is updated
    pub async fn add_route(&mut self, route: String, content: std::pin::Pin<Box<dyn Fn(Request) -> std::pin::Pin<Box<dyn Future<Output = Result<String, String>> + Send>>>>){
        self.routes.insert(route, content);
    }

    /// # Get Route
    /// 
    /// This function takes in the response string from the `TcpStream` and searches the hashmap
    /// for the callback function associated with the route. It then checks that the route is valid,
    /// and runs it asynchrynously (using the request so that the callback can make use of the request data)
    /// 
    /// This function only runs the callback - handling POST and GET requests is up to the callback.
    pub async fn get_route(&self, request: String, user_addr: std::net::SocketAddr) -> Result<String, &str>{
        let request = Request::new(request, user_addr);

        let func = match self.routes.get(&request.uri){
            Some(v) => v,
            None => self.routes.get(&"err".to_string()).unwrap(), // we assume we've got an error handler
        };
           
        // Check that our function returned an Ok result, and unwrap it after it executes
        let result: String = if let Ok(v) = func(request).await{
            return Ok(v);
        }else{
            String::new() // Err returned, just return nothing
        };

        Ok(result)
    }
}