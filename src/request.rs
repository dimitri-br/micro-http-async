use std::collections::HashMap;

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
    pub method: Option<HttpMethod>,
    pub uri: String,
    pub user_agent: String,
    pub user_addr: std::net::SocketAddr,
    pub get_request: HashMap<String, String>,
    pub post_request: HashMap<String, PostRequest>,
    pub raw_request: Vec<String>,
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
    pub async fn new(request: String, user_addr: std::net::SocketAddr) -> Result<Self, &'static str> {
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
            if strings.len() - 1 > index_of_post {
                let mut form_data = Vec::new();

                let mut should_append = false;
                for value in strings.iter() {
                    if value.contains("-----------------------------") {
                        should_append = true;
                        continue;
                    }

                    if value.contains("Content-") {
                        continue;
                    }

                    if should_append {
                        form_data.push(value);
                    }
                }
                let mut is_new_item = false;
                let mut name = String::new();
                let mut filename = String::new();
                let mut data = Vec::new();

                for value in form_data.iter() {
                    // Is a name, so we get the name. We also check the last given data so we can write the old post data.
                    if value.contains(" name=") {
                        if data.len() > 0 && !is_new_item {
                            let p_r =
                                PostRequest::new(name.clone(), filename.clone(), data.clone())
                                    .await;
                            data.clear();
                            post_req.insert(name.clone(), p_r);
                        }

                        let mut split_name = value.split(" name=");
                        split_name.next();
                        name = split_name.next().unwrap().to_string();
                        name.remove_matches("\"");

                        is_new_item = true;
                        continue;
                    }

                    // Is a file, so we get the filename
                    if value.contains(" filename=") {
                        let mut split_name = value.split(" filename=");
                        split_name.next();
                        filename = split_name.next().unwrap().to_string();
                        filename.remove_matches("\"");

                        continue;
                    }

                    // is data
                    is_new_item = false;
                    data.append(&mut value.clone().as_bytes().to_vec());
                }

                // We make sure the last bit of form data is saved
                if data.len() > 0 && !is_new_item {
                    let p_r = PostRequest::new(name.clone(), filename.clone(), data.clone()).await;
                    data.clear();
                    post_req.insert(name.clone(), p_r);
                }
            }
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
