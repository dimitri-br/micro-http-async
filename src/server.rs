use std::io::BufReader;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::io::{self};
use tokio::net::{TcpListener, TcpStream}; // Async versions of the stdlib implementation // :D

// TLS stuff, so we can support HTTPS
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use rustls_pemfile::{certs, rsa_private_keys};

use crate::{Connection, connection::ConnectionType};
use crate::Routes;

/// # HTTP Server
///
/// This struct stores the listener, which listens for incoming connections and handles them
///
/// **Example**:
///
/// ```
/// let http_server = HttpServer::new("127.0.0.1", "8080").await.unwrap(); // Create a new http listener
/// ```
pub struct HttpServer {
    listener: TcpListener,
    pub routes: Routes,

    /* Hidden parameters */
    read_buffer_size: usize,

    // TLS stuff
    tls_acceptor: Option<TlsAcceptor>,
}

impl HttpServer {
    /// # New
    ///
    /// Create a new server, with a given IP and port
    ///
    /// **Example**
    /// ```
    /// let http_server = HttpServer::new("127.0.0.1", "8080").await.unwrap();
    /// ```
    pub async fn new(ip: &str, port: &str) -> io::Result<Self> {
        let address = format!("{}:{}", ip, port);
        println!("Listening on {}", address);
        Ok(Self {
            listener: TcpListener::bind(&address).await?,
            routes: Routes::new().await,
            read_buffer_size: 8192,
            tls_acceptor: None,
        })
    }

    pub async fn new_tls(ip: &str, port: &str, cert_path: PathBuf, key_path: PathBuf) -> io::Result<Self> {
        let address = format!("{}:{}", ip, port);
        println!("Listening on {}", address);
        let certs = load_certs(&cert_path)?;
        let mut keys = load_keys(&key_path)?;

        let config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, keys.remove(0))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        let acceptor = TlsAcceptor::from(Arc::new(config));

        Ok(Self {
            listener: TcpListener::bind(&address).await?,
            routes: Routes::new().await,
            read_buffer_size: 8192,
            tls_acceptor: Some(acceptor),
        })
    }

    /// # Listen
    ///
    /// Listen for new connections.
    ///
    /// Run `handle_connection` upon connection.
    pub async fn listen(&mut self) -> Result<(), &'static str> {
        loop {
            let (socket, addr) = self.listener.accept().await.unwrap(); // Accept an incoming connection

            let acceptor = self.tls_acceptor.clone();

            // Check if we need to use TLS
            if let Some(ref tls_acceptor) = acceptor {
                let tls_socket = tls_acceptor.accept(socket).await.unwrap();
                self.handle_connection_tls(tls_socket, addr).await.unwrap();
            }else{
                self.handle_connection(socket, addr).await.unwrap(); // Handle it
            }
        }
    }

    /// # Handle Connection
    ///
    /// This function takes a `TcpStream`, and runs all the necessary functions to read the request,
    /// handle the response and write it back to the user.
    ///
    /// This function should only be called by the `HttpServer`, as it should only be run upon accepting
    /// a new connection
    ///
    /// We define the content to return using the `Routes` struct in `HttpServer`
    ///
    /// It returns a Result for better error handling if something goes wrong at any point during I/O operations
    async fn handle_connection(
        &mut self,
        stream: TcpStream,
        addr: std::net::SocketAddr,
    ) -> Result<(), &str> {
        let mut connection = Connection::new(ConnectionType::Plain(Some(stream)), self.read_buffer_size); // Create our connection handler

        let request_str = connection.read_to_string().await.unwrap(); // get a string value from the recieved data

        // only needs the request and address as it constructs a `Request` to get the route and more info
        let ret_str = self.routes.get_route(request_str, addr).await.unwrap();

        match ret_str {
            crate::DataType::Text(text) => {
                connection.write_string(text).await.unwrap();
            }
            crate::DataType::Bytes(bytes) => {
                connection.write_bytes(bytes).await.unwrap();
            }
        }

        Ok(()) // Return the future
    }

    async fn handle_connection_tls(
        &mut self,
        stream: TlsStream<TcpStream>,
        addr: std::net::SocketAddr,
    ) -> Result<(), &str> {
        let mut connection = Connection::new(ConnectionType::TLS(Some(stream)), self.read_buffer_size); // Create our connection handler

        let request_str = connection.read_to_string().await.unwrap(); // get a string value from the recieved data

        // only needs the request and address as it constructs a `Request` to get the route and more info
        let ret_str = self.routes.get_route(request_str, addr).await.unwrap();

        match ret_str {
            crate::DataType::Text(text) => {
                connection.write_string(text).await.unwrap();
            }
            crate::DataType::Bytes(bytes) => {
                connection.write_bytes(bytes).await.unwrap();
            }
        }

        Ok(()) // Return the future
    }

    /// # Set Read Buffer Size
    /// 
    /// Set the read buffer size for the server. The default value is 8192 bytes.
    pub async fn set_read_buffer_size(&mut self, size: usize) -> Result<(), &'static str> {
        self.read_buffer_size = size;
        
        Ok(())
    }
}


// Helpful TLS functions for reading certificates and keys

/// # Load Certificates
/// 
/// Load certificates from a given path.
/// 
/// Returns a `Vec<Certificate>`
fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    let buf_reader = &mut BufReader::new(File::open(path).unwrap());
    certs(buf_reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}


/// # Load Keys
/// 
/// Load keys from a given path.
/// 
/// Returns a `Vec<PrivateKey>`
fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let buf_reader = &mut BufReader::new(File::open(path).unwrap());
    rsa_private_keys(buf_reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}