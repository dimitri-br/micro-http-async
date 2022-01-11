use std::collections::HashMap;

use regex::Regex;


/// # Http Methods
///
/// An enum with the types of method that a user can request
///
/// It is up to the user to specify in the callbacks which
/// methods to accept
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HttpMethod {
    Get,
    Post,
    Head,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

/// # Request
///
/// Handles requests from users, and returns it in a friendly, safe way.
///
/// Can be used to gather POST and GET info, user-agent information and more
///
/// If you have custom headers, and want to access them, use `raw_request` to access the
/// raw unmodified request
#[derive(Debug)]
pub struct Request {
    /// Method stores the method used to
    /// make the request
    pub method: Option<HttpMethod>,
    /// URI contains the URI of the request
    pub uri: String,
    /// User Agent stores the user agent of the user
    pub user_agent: String,
    /// User Addr stores the users IP address
    pub user_addr: std::net::SocketAddr,
    /// Get Request stores the data of the get request.
    /// 
    /// It is a `HashMap<String, String>`
    /// 
    /// The key of the hashmap is equal to the name of the
    /// form field name.
    pub get_request: HashMap<String, String>,
    /// Post Request stores the data of the post request.
    /// 
    /// It is a `HashMap<String, PostRequest>`
    /// 
    /// The key of the hashmap is equal to the name of the
    /// form field name.
    pub post_request: HashMap<String, PostRequest>,
    /// Raw Request stores the raw request without any modifications.
    pub raw_request: Vec<String>,
    /// Did the request come from a secure connection?
    pub secure: bool,
}

impl Request {
    /// # New
    ///
    /// Create a new request struct.
    ///
    /// Takes an input string (Which should be
    /// the request).
    ///
    /// It will then construct itself and return, ready to use.
    pub async fn new(request: String, user_addr: std::net::SocketAddr, is_secure: bool) -> Result<Self, &'static str> {
        let request = Request::split_to_row(request).await;

        let method = Request::get_method(&request).await;

        let uri = Request::get_uri(&request).await;

        let user_agent = Request::get_user_agent(&request).await;

        let (get_request, uri) = Request::get_vars(&uri).await;

        let post_request = Request::get_post_request(&request).await;

        Ok(Self {
            method,
            uri,
            user_agent,
            user_addr,
            get_request,
            post_request,
            raw_request: request,
            secure: is_secure,
        })
    }

    /// # Split To Row
    ///
    /// This function splits a string into rows for every new line
    async fn split_to_row(string: String) -> Vec<String> {
        let u_strings: Vec<String> = string.split("\r\n").map(|x| x.to_string()).collect();

        let mut strings = Vec::<String>::new();
        for string in u_strings {
            if string.contains(";") {
                let mut sts: Vec<String> = string.split(";").map(|x| x.to_string()).collect();
                strings.append(&mut sts);
            } else {
                strings.push(string.clone());
            }
        }
        strings
    }

    /// # Get Method
    ///
    /// This function gets the method from a http request (Eg, POST)
    async fn get_method(strings: &Vec<String>) -> Option<HttpMethod> {
        let mut method: Option<HttpMethod> = None;
        for string in strings.iter() {
            for substring in string.split(" ") {
                match substring {
                    "GET" => {
                        method = Some(HttpMethod::Get);
                        break;
                    }
                    "POST" => {
                        method = Some(HttpMethod::Post);
                        break;
                    }
                    "HEAD" => {
                        method = Some(HttpMethod::Head);
                        break;
                    }
                    "PUT" => {
                        method = Some(HttpMethod::Put);
                        break;
                    }
                    "DELETE" => {
                        method = Some(HttpMethod::Delete);
                        break;
                    }
                    "CONNECT" => {
                        method = Some(HttpMethod::Connect);
                        break;
                    }
                    "OPTIONS" => {
                        method = Some(HttpMethod::Options);
                        break;
                    }
                    "TRACE" => {
                        method = Some(HttpMethod::Trace);
                        break;
                    }
                    _ => continue,
                }
            }
            if !method.is_none() {
                break;
            }
        }

        method
    }

    /// # Get uri
    ///
    /// This funcion gets the URI of the request
    ///
    /// The URI is the requested route (eg, /about)
    async fn get_uri(strings: &Vec<String>) -> String {
        let string = &strings[0];

        let strings: Vec<String> = string.split(" ").map(|x| x.to_string()).collect();

        let uri = if strings.len() > 1 {
            strings[1].clone()
        } else {
            "/error".to_string()
        };

        uri
    }

    /// # Get Vars
    ///
    /// This function takes in a URI and extracts the GET parameters, returning them as a hashmap
    ///
    /// This can then be used by the callback
    async fn get_vars(uri: &String) -> (HashMap<String, String>, String) {
        let split_uri: Vec<String> = uri.split("?").map(|x| x.to_string()).collect();

        let mut hash_vals = HashMap::<String, String>::new();

        if uri.contains("?") && split_uri.len() > 1 {
            let get_vals = &split_uri[1];

            for val in get_vals.split("&") {
                let split_vals: Vec<String> = val.split("=").map(|x| x.to_string()).collect();
                if split_vals.len() > 1 {
                    hash_vals.insert(
                        split_vals[0].clone(),
                        split_vals[1].clone().replace("+", " "),
                    );
                }
            }
        }

        (hash_vals, split_uri[0].clone())
    }

    /// # Get post request
    ///
    /// Takes in the split request, returns a hashmap with the `variables` and `values`.
    ///
    /// This currently only works for the standard `application/x-www-form-urlencoded` form type,
    /// and doesn't support `multipart/form-data` yet.
    async fn get_post_request(strings: &Vec<String>) -> HashMap<String, PostRequest> {
        //println!("{:?}", strings);

        if !strings.contains(&"Content-Type: application/x-www-form-urlencoded".to_string())
            && !strings.contains(&"Content-Type: multipart/form-data".to_string())
        {
            return HashMap::new();
        }

        let index_of_post = strings.iter().position(|x| x == "").unwrap();

        let mut post_req = HashMap::new();

        if strings.contains(&"Content-Type: application/x-www-form-urlencoded".to_string()) {
            if strings.len() - 1 > index_of_post {
                for n in 0..(strings.len() - 1) - index_of_post {
                    let val = &strings[index_of_post + n + 1];
                    let args = val.split("&");
                    for val in args {
                        let mut split_val = val.split("=");
                        let p_var = split_val.next().unwrap();
                        let p_val = split_val.next().unwrap();
                        let p_r =
                            PostRequest::new(p_var.to_string(), "".to_string(), p_val.into()).await;
                        post_req.insert(p_var.to_string(), p_r);
                    }
                }
            }
        } else {
            // It is multipart, so extract the data from it and store it in post_req

            let mut boundary = String::new();
            
            let mut name = String::new();

            let mut filename = String::new();

            let mut data = Vec::<u8>::new();

            

            for string in strings[index_of_post + 1..].iter() {
                // Check for the boundaries
                if string.contains("-----------------------------") {
                    // Now, we need to extract the boundary hash with regex, eg boundary=---------------------------179249680738152359603687021860
                    // becomes 179249680738152359603687021860
                    let re = Regex::new(r"\w*?([\d|\.]+)\w*?([\d{1,4}]+).*").unwrap();
                    let re_boundary = match re.find(string){
                        Some(v) => {v.as_str().to_string()},
                        None => {panic!("Error - multipart form data is invalid!")}
                    };         
                    if re_boundary == boundary{

                        // Create a post request based on the data
                        data.remove(0); // The first item in the data is literally empty bytes, we don't need it.
                        let p_r = PostRequest::new(name.clone(), filename.clone(), data.clone()).await;
                        post_req.insert(name.clone(), p_r);
                        
                        // We've reached the end of the form data, so reset the data
                        name.clear();
                        filename.clear();
                        data.clear();
                    }
                    boundary = re_boundary;
                    
                    continue;
                }

                if !boundary.is_empty(){
                    // We're in the middle of the form data, so here we extract the data
                    
                    // Check for content disposition. Skip if we match
                    if string.contains("Content-Disposition: form-data"){
                        continue;
                    }

                    // Check if there is a filename
                    if string.contains("filename="){
                        let re = Regex::new(r#""([^"].+)""#).unwrap();
                        let re_filename = match re.find(string){
                            Some(v) => {v.as_str().to_string()},
                            None => {panic!("Error - multipart form data is invalid!")}
                        };
                        // Remove quotes
                        let re_filename = re_filename.replace("\"", "");
                        filename = re_filename;

                        continue;
                    }

                    // Check for the name
                    if string.contains("name="){
                        let re = Regex::new(r#""([^"].+)""#).unwrap();
                        let re_name = match re.find(string){
                            Some(v) => {v.as_str().to_string()},
                            None => {panic!("Error - multipart form data is invalid!")}
                        };
                        // Remove quotes
                        let re_name = re_name.replace("\"", "");
                        name = re_name;

                        continue;
                    }

                    // Check for content type. If we match, just skip
                    if string.contains("Content-Type:"){
                        continue;
                    }

                    // If we've finished, then we can add the data to the hashmap
                    data.extend((format!("{}\n", string)).as_bytes());
                }
            }

            // Save the last value in the hashmap, as we've reached the end of the form data. 
            data.remove(0); // The first item in the data is literally empty bytes, we don't need it.
            data.remove(data.len() - 1); // We also remove the last value as it's garbage (TODO: FIX THIS)
            let p_r = PostRequest::new(name.clone(), filename.clone(), data.clone()).await;
            post_req.insert(name.clone(), p_r);
        }

        post_req
    }
    

    /// # Get user agent
    ///
    /// This function gets the user agent from the request
    ///
    /// Can be useful when doing dynamic page downloads (eg, specific downloads for macos or android)
    async fn get_user_agent(strings: &Vec<String>) -> String {
        let mut agent = "none".to_string();
        for string in strings.iter() {
            if string.contains("User-Agent:") {
                let agent_raw: Vec<String> = string
                    .split("User-Agent: ")
                    .map(|x| x.to_string())
                    .collect();
                agent = agent_raw[1].clone();
            }
            if agent != "none".to_string() {
                break;
            }
        }

        agent
    }
}

/// # Post Request
///
/// A representation of a post request
#[derive(Debug)]
pub struct PostRequest {
    pub name: String,
    pub file_name: String,
    pub data: Vec<u8>,
}

impl PostRequest {
    pub async fn new(name: String, file_name: String, data: Vec<u8>) -> Self {
        Self {
            name,
            file_name,
            data,
        }
    }

    pub async fn get_data<T: std::convert::From<std::vec::Vec<u8>>>(&self) -> T {
        self.data.clone().into()
    }
}
