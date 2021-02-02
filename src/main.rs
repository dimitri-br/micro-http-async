pub mod server;
pub mod connection;

use async_trait::async_trait;

use tokio::net::TcpStream;

use std::future::Future;

use server::{HttpServer, ConnectionHandler};
use connection::Connection;



// Define a connection callback for the HttpServer struct. Can be anything you want, as long as it returns a result and is async
#[async_trait]
impl ConnectionHandler for HttpServer{
    async fn handle_connection(&mut self, stream: TcpStream) -> Result<(), &str>{
        
        let mut connection = Connection::new(stream); // Create our connection handler

        let _recv_value = connection.read_to_string().await; // get a string value from the recieved data

        let header = "HTTP/1.1 200 OK\r\n\r\n";
        let head = r#"
        <head>
            <title>Async Server</title>
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-giJF6kkoqNQ00vy+HMDP7azOuL0xtbfIcaT9wjKHr8RbDVddVHyTfAAsrekwKmP1" crossorigin="anonymous" \>
        </head>"#;
        let body = r#"
            <body class="bg-dark text-light align-middle">
                <h1>Data recieved successfully!</h1>
                <p>Thanks for testing my asynchrynous web server</p>
                <p>This is running from the trait!</p>
            </body>"#;

        let ret_str = format!("{}{}{}", header, head, body);

        connection.write_string(ret_str).await.unwrap();

        Ok(()) // Return the future
    }
}


#[tokio::main]
pub async fn main() {

    callback_caller(callback).await;

    let mut http_server = HttpServer::new("127.0.0.1", "8080").await.unwrap();

    http_server.listen().await;
}

pub async fn callback(){
    println!("Callback!");
}

async fn callback_caller<F, Fut>(f: F) where F: FnOnce() -> Fut, Fut: Future<Output = ()>,
{
    f().await;
}