/// # Response
/// 
/// This enum simplifies return codes for HTTP responses
/// ```
/// let ok = Response::Ok;
/// // OR
/// let ok = Response::from(200);
/// ```
pub enum Response{
    /// All Ok
    Ok,
    /// Link/Page has moved, redirect
    Redirect,
    /// Client error - something went wrong that was caused by the client
    ClientErr,
    /// Server error - something went wrong server side
    ServerErr,
}

impl std::convert::From<u32> for Response{
    fn from(v: u32) -> Response{
        match v{
            200..=299 => { return Response::Ok; }
            300..=399 => { return Response::Redirect; }
            400..=499 => { return Response::ClientErr; }
            500..=599 => { return Response::ServerErr; }
            _ => { return Response::ServerErr; }
        }
    }
}