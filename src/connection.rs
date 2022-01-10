use std::error::Error;
use std::io;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use tokio::io::{split, AsyncWriteExt, AsyncReadExt};



pub enum ConnectionType{
    Plain(Option<TcpStream>),
    TLS(Option<TlsStream<TcpStream>>)
}

/// # Connection
///
/// This struct is a helpful struct to handle the nitty gritty of
/// connections, such as reading and writing to the stream
pub struct Connection{
    /// The connection type - plain or TLS
    connection_type: ConnectionType,
    /// The read buffer size
    read_buffer_size: usize,

    /// We need to store the TLS stream so we don't run into copy/clone issues
    tls_stream: Option<TlsStream<TcpStream>>,
}

impl Connection {
    /// # New
    ///
    /// Create a new connection handler from a `TcpStream`
    pub fn new(mut connection_type: ConnectionType, read_buffer_size: usize) -> Self {
        // Check if we need to use TLS
        let tls_stream = match connection_type {
            ConnectionType::TLS(ref mut tls_stream) => {
                let stream = tls_stream.take().unwrap();
                Some(stream)
            },
            _ => None,
        };
        Connection{
            connection_type,
            read_buffer_size,
            tls_stream,
        }
    }

    /// # Read To String
    ///
    /// Read the `TcpStream` to a `String`
    pub async fn read_to_string(&mut self) -> Result<String, &'static str> {
        let mut string = unsafe {
            String::from_utf8_unchecked(
                self.read_to_vec()
                    .await
                    .expect("Error reading vec from stream"),
            )
        };
        //.expect("Error decoding stream to utf-8");
        trim_newline(&mut string);

        Ok(string)
    }

    /// # Read To Vec
    ///
    /// Read the `TcpStream` to a `Vec<u8>`. Returns a `Result` as we cannot guarantee a successful read.
    pub async fn read_to_vec(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = Vec::with_capacity(self.read_buffer_size);

        
        match self.connection_type {
            ConnectionType::Plain(ref mut stream) => {
                let stream = stream.as_mut().unwrap();
                // We loop while we're waiting for a read
                loop {
                    stream.readable().await?; // we await for the stream to be readable

                    // Try to read data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match stream.try_read_buf(&mut buffer) {
                        Ok(0) => break, // No data recieved
                        Ok(_) => {
                            break; // we recieved some data, just break the loop and return it
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue; // The IO is busy and would block - just try again
                        }
                        Err(e) => {
                            return Err(e.into()); // Some other error - quit
                        }
                    }
                }
                let buffer: Vec<u8> = buffer
                    .iter()
                    .filter(|x| **x != 0x0u8)
                    .map(|x| *x)
                    .collect(); // Remove all empty bytes (to avoid trailing whitespaces)

                return Ok(buffer) // return our vector;

            },
            ConnectionType::TLS(_) => {
                // Unfortunately, the TLS stream doesn't implement `AsyncRead`
                // so we have to use the `split` combinator to split the stream
                // This is a bit of a hack, but it works. We also lose 
                // async/await support, but that's ok for now.
                let tls_stream = self.tls_stream.take().unwrap();
                let (mut reader, writer) = split(tls_stream);

                // Try to read data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                reader.read_buf(&mut buffer).await.unwrap();

                let buffer: Vec<u8> = buffer
                    .iter()
                    .filter(|x| **x != 0x0u8)
                    .map(|x| *x)
                    .collect(); // Remove all empty bytes (to avoid trailing whitespaces)

                // Reconnect the stream
                self.tls_stream = Some(reader.unsplit(writer));

                return Ok(buffer) // return our vector;
            }
        };
    }

    /// # Write String
    ///
    /// Write a `String` value to the `TcpStream`. Returns a `Result` as we cannot guarantee a successful write.
    pub async fn write_string(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        match self.connection_type{
            ConnectionType::Plain(ref mut stream) => {
                let stream = stream.as_mut().unwrap();

                loop {
                    // Wait for the socket to be writable
                    stream.writable().await?;
        
                    // See `read_to_vec` for more explaination what happens here
                    //
                    // Try to write data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match stream.try_write(data.as_bytes()) {
                        Ok(_) => {
                            break;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            },
            ConnectionType::TLS(_) =>{
                // Unfortunately, the TLS stream doesn't implement `AsyncWrite`
                // so we have to use the `sink` combinator to split the stream
                // This is a bit of a hack, but it works. We also lose 
                // async/await support, but that's ok for now.
                let tls_stream = self.tls_stream.take().unwrap();
                let (reader, mut writer) = split(tls_stream);

                // Try to write data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                writer.write_all(data.as_bytes()).await?;

                // Reconnect the stream
                self.tls_stream = Some(reader.unsplit(writer));
            }
        }

        Ok(())
    }

    /// # Write Bytes
    ///
    /// Write bytes to the TCP stream, useful for sending data for things such as images or downloadable binary files
    pub async fn write_bytes(&mut self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        match self.connection_type{
            ConnectionType::Plain(ref mut stream) => {
                let stream = stream.as_mut().unwrap();

                loop {
                    // Wait for the socket to be writable
                    stream.writable().await?;
        
                    // See `read_to_vec` for more explaination what happens here
                    //
                    // Try to write data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match stream.try_write(&data) {
                        Ok(_) => {
                            break;
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            },
            ConnectionType::TLS(_) => {
                // Unfortunately, the TLS stream doesn't implement `AsyncWrite`
                // so we have to use the `sink` combinator to split the stream
                // This is a bit of a hack, but it works. We also lose 
                // async/await support, but that's ok for now.
                let tls_stream = self.tls_stream.take().unwrap();
                let (reader, mut writer) = split(tls_stream);

                // Try to write data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                writer.write_all(&data).await?;

                // Reconnect the stream
                self.tls_stream = Some(reader.unsplit(writer));
            }
        };

        Ok(())
    }
}

/// Trim the ends of the `String` we got from the `TcpStream` so we don't waste buffer space with whitespace
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
