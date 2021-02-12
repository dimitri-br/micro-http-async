use crate::Response;

pub struct JSONResponse;

impl JSONResponse{
    pub async fn construct_response(response_code: Response, data: String) -> String{
        // This should be changed over to support all response types
        let header_code = match response_code{
            Response::Ok => {format!("HTTP/1.1 {} {}\r\n\r\n", 200, "OK")}
            Response::Redirect => {format!("HTTP/1.1 {} {}\r\n\r\n", 301, "MOVED PERMANENTLY")}
            Response::ClientErr => {format!("HTTP/1.1 {} {}\r\n\r\n", 404, "NOT FOUND")}
            Response::ServerErr => {format!("HTTP/1.1 {} {}\r\n\r\n", 500, "INTERNEL SERVER ERROR")}
        };
        let file = format!("{}{}", header_code, data);
        return file;
    }
}