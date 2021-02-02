use tokio::net::{TcpListener, TcpStream}; // Async versions of the stdlib implementation
use tokio::io; // :D

use async_trait::async_trait;


/// # HTTP Server
/// 
/// This struct stores the listener, which listens for incoming connections and handles them
/// 
/// **Example**:
/// 
/// ```
/// let http_server = HttpServer::new("127.0.0.1", "8080").await.unwrap(); // Create a new http listener
/// ```
pub struct HttpServer{
    listener: TcpListener
}


impl HttpServer{

    /// # New
    /// 
    /// Create a new server, with a given IP and port
    /// 
    /// **Example**
    /// ```
    /// let http_server = HttpServer::new("127.0.0.1", "8080").await.unwrap();
    /// ```
    pub async fn new(ip: &str, port: &str) -> io::Result<Self>{
        let address = format!("{}:{}", ip, port);
        Ok(Self{
            listener: TcpListener::bind(&address).await?
        })
    }

    /// # Listen
    /// 
    /// Listen for new connections. 
    /// 
    /// Run `handle_connection` upon connection.
    pub async fn listen(&mut self){
        loop{
            let (socket, addr) = self.listener.accept().await.unwrap(); // Accept an incoming connection
            println!("Recieved new connection from {:?}", addr); // Let us know who it is
            self.handle_connection(socket).await.unwrap(); // Handle it
        }
    }
}


/// # Connection Handler
/// 
/// This method is required.
/// 
/// This method takes a TcpStream (from the tokio crate) and should both read the request and write a response.
/// 
/// **Example**
/// 
/// ```
/// #[async_trait]
/// impl ConnectionHandler for HttpServer{
///    /// This function handles a connection using the `Connection` struct and its functions
///    /// Please note this is not the best way to do it! 
///    async fn handle_connection(&mut self, stream: TcpStream) -> Result<(), &str>{
///        
///        let mut connection = Connection::new(stream); // Create our connection handler
///
///        let _recv_value = connection.read_to_string().await; // get a string value from the recieved data
///
///        let header = "HTTP/1.1 200 OK\r\n\r\n";
///        let head = r#"
///        <head>
///            <title>Async Server</title>
///            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-giJF6kkoqNQ00vy+HMDP7azOuL0xtbfIcaT9wjKHr8RbDVddVHyTfAAsrekwKmP1" crossorigin="anonymous" \>
///        </head>"#;
///        let body = r#"
///            <body class="bg-dark text-light align-middle">
///                <h1>Data recieved successfully!</h1>
///                <p>Thanks for testing my asynchrynous web server</p>
///            </body>"#;
///
///        let ret_str = format!("{}{}{}", header, head, body);
///
///        connection.write_string(ret_str).await.unwrap();
///
///        Ok(()) // Return the future
///    }
/// }
/// ```
#[async_trait]
pub trait ConnectionHandler{
    /// # Handle Connection
    /// Handle a TcpStream connection. 
    /// 
    /// Gets called every time a connection is made.
    async fn handle_connection(&mut self, stream: TcpStream) -> Result<(), &str>;
}