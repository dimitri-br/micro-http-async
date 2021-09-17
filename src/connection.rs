use std::error::Error;
use std::io;
use tokio::net::TcpStream;

/// # Connection
///
/// This struct is a helpful struct to handle the nitty gritty of
/// connections, such as reading and writing to the stream
pub struct Connection(TcpStream);

impl Connection {
    /// # New
    ///
    /// Create a new connection handler from a `TcpStream`
    pub fn new(stream: TcpStream) -> Self {
        Connection(stream)
    }

    /// # Read To String
    ///
    /// Read the `TcpStream` to a `String`
    pub async fn read_to_string(&self) -> String {
        let mut string = String::from_utf8(
            self.read_to_vec()
                .await
                .expect("Error reading vec from stream"),
        )
        .expect("Error decoding stream to utf-8");
        trim_newline(&mut string);

        string
    }

    /// # Read To Vec
    ///
    /// Read the `TcpStream` to a `Vec<u8>`. Returns a `Result` as we cannot guarantee a successful read.
    pub async fn read_to_vec(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = [0; 2048]; // Create our buffer

        let stream: &TcpStream = &self.0; // Get a reference to the stream

        // We loop while we're waiting for a read
        loop {
            stream.readable().await?; // we await for the stream to be readable

            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_read(&mut buffer) {
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
            .to_vec()
            .iter()
            .filter(|x| **x != 0x0u8)
            .map(|x| *x)
            .collect(); // Remove all empty bytes (to avoid trailing whitespaces)
        Ok(buffer) // return our vec
    }

    /// # Write String
    ///
    /// Write a `String` value to the `TcpStream`. Returns a `Result` as we cannot guarantee a successful write.
    pub async fn write_string(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        let stream: &mut TcpStream = &mut self.0; // Get a reference to the stream

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

        Ok(())
    }

    /// # Write Bytes
    ///
    /// Write bytes to the TCP stream, useful for sending data for things such as images or downloadable binary files
    pub async fn write_bytes(&mut self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let stream: &mut TcpStream = &mut self.0; // Get a reference to the stream

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
