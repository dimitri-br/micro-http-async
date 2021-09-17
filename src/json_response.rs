use crate::Response;

/// # JSONResponse
///
/// Helpful tool for converting a serde_json json string into a response.
///
/// It will take in the response code, and create a response string with the headers required.
///
/// See the examples for usage
pub struct JSONResponse;

impl JSONResponse {
    /// This function takes the response code and data (the json) to write, and returns a string that can be used
    /// as a response.
    pub async fn construct_response(response_code: Response, data: String) -> String {
        // This should be changed over to support all response types
        let header_code = match response_code {
            Response::Ok => {
                format!("HTTP/1.1 {} {}\r\n\r\n", 200, "OK")
            }
            Response::Redirect => {
                format!("HTTP/1.1 {} {}\r\n\r\n", 301, "MOVED PERMANENTLY")
            }
            Response::ClientErr => {
                format!("HTTP/1.1 {} {}\r\n\r\n", 404, "NOT FOUND")
            }
            Response::ServerErr => {
                format!("HTTP/1.1 {} {}\r\n\r\n", 500, "INTERNEL SERVER ERROR")
            }
        };
        let file = format!("{}{}", header_code, data);
        return file;
    }
}
