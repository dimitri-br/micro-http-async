
/// # Http Methods
/// 
/// An enum with the types of method that a user can request
/// 
/// It is up to the user to specify in the callbacks which 
/// methods to accept
#[derive(Debug)]
pub enum HttpMethod{
    Get,
    Post,
    Head,
    Put,
    Delete,
    Connect,
    Options,
    Trace
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
pub struct Request{
    pub method: Option<HttpMethod>,
    pub uri: String,
    pub user_agent: String,
    pub user_addr: std::net::SocketAddr,
    pub raw_request: Vec::<String>
}

impl Request{
    /// # New
    /// 
    /// Create a new request struct. 
    /// 
    /// Takes an input string (Which should be
    /// the request).
    /// 
    /// It will then construct itself and return, ready to use.
    pub fn new(request: String, user_addr: std::net::SocketAddr) -> Self{
        
        let request = Request::split_to_row(request);

        let method = Request::get_method(&request);

        let uri = Request::get_uri(&request);

        let user_agent = Request::get_user_agent(&request);


        Self{
            method,
            uri, 
            user_agent,
            user_addr,
            raw_request: request
        }
    }

    /// # Split To Row
    /// 
    /// This function splits a string into rows for every new line
    fn split_to_row(string: String) -> Vec::<String>{
        let strings: Vec::<String> = string.split("\r\n").map(|x| x.to_string()).collect();

        strings
    }

    /// # Get Method
    /// 
    /// This function gets the method from a http request (Eg, POST)
    fn get_method(strings: &Vec::<String>) -> Option<HttpMethod>{
        let mut method: Option<HttpMethod> = None;
        for string in strings.iter(){
            for substring in string.split(" "){
                match substring{
                    "GET" => {
                        method = Some(HttpMethod::Get);
                        break
                    },
                    "POST" => {
                        method = Some(HttpMethod::Post);
                        break;
                    },
                    "HEAD" => {
                        method = Some(HttpMethod::Head);
                        break;
                    },
                    "PUT" => {
                        method = Some(HttpMethod::Put);
                        break;
                    },
                    "DELETE" => {
                        method = Some(HttpMethod::Delete);
                        break;
                    },
                    "CONNECT" => {
                        method = Some(HttpMethod::Connect);
                        break;
                    },
                    "OPTIONS" => {
                        method = Some(HttpMethod::Options);
                        break;
                    },
                    "TRACE" => {
                        method = Some(HttpMethod::Trace);
                        break;
                    }
                    _ => continue
                }
            }
            if !method.is_none(){
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
    fn get_uri(strings: &Vec::<String>) -> String{
        let string = &strings[0];

        let strings: Vec::<String> = string.split(" ").map(|x| x.to_string()).collect();

        let uri = if strings.len() > 1{
            strings[1].clone()
        }else{
            "/error".to_string()
        };
        
        uri
    }

    /// # Get user agent
    /// 
    /// This function gets the user agent from the request
    /// 
    /// Can be useful when doing dynamic page downloads (eg, specific downloads for macos or android)
    fn get_user_agent(strings: &Vec::<String>) -> String{
        let mut agent = "none".to_string();
        for string in strings.iter(){
            if string.contains("User-Agent:"){
                let agent_raw: Vec::<String> = string.split("User-Agent: ").map(|x| x.to_string()).collect();
                agent = agent_raw[1].clone();
            }
            if agent != "none".to_string(){
                break;
            }
        }

        agent
    }
    
}