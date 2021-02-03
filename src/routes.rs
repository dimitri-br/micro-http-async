use std::collections::HashMap;
use crate::Request;


/// # Routes
/// 
/// This struct defines the routes. It uses a hashmap to do this.Connection
/// 
/// `HashMap<Route, Content>` where content is the return content (ie, html or json).
/// 
/// TODO: get and post
pub struct Routes<F: Fn(Request) -> String + Send>{
    routes: HashMap<String, F>,
}

impl<F: Fn(Request) -> String + Send> Routes<F>{
    /// # New
    /// 
    /// Create a new `Route` struct
    pub async fn new() -> Self{
        Self{
            routes: HashMap::<String, F>::new(),
        }
    }

    /// # Add Route
    /// 
    /// Adds a new route to the routes hashmap. If the route already exists,
    /// its value is updated
    pub async fn add_route(&mut self, route: String, content: F) -> Result<(), &str>{
        self.routes.insert(route, content);
        Ok(())
    }

    /// # Get Route
    /// 
    /// Dynamically generate a new response for a route based on the request.
    /// 
    /// Define your own functions to handle this - it MUST return a string
    pub async fn get_route(&self, request: String) -> Result<String, &str>{
        let request = Request::new(request);

        let func = match self.routes.get(&request.uri){
            Some(v) => v,
            None => self.routes.get(&"err".to_string()).unwrap(), // we assume we've got an error handler
        };
           
        let result = func(request);
        Ok(result)
    }
}