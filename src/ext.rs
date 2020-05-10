use crate::Multipart;
use hyper::{header, Request};

/// An extension trait which extends [`hyper::Request<Body>`](https://docs.rs/hyper/0.13.5/hyper/struct.Request.html)
/// to add some methods to parse request body as `multipart/form-data`.
pub trait RequestMultipartExt {
    /// Convert the request body to [`Multipart`](./struct.Multipart.html) if the `content-type` is `multipart/form-data`.
    ///
    /// # Errors
    ///
    /// This method fails if the request body is not `multipart/form-data` and in this case, you could send a `400 Bad Request` status.
    fn into_multipart(self) -> routerify::Result<Multipart>;
}

impl RequestMultipartExt for Request<hyper::Body> {
    fn into_multipart(self) -> routerify::Result<Multipart> {
        let boundary = self
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|val| val.to_str().ok())
            .and_then(|val| multer::parse_boundary(val).ok());

        if let Some(boundary) = boundary {
            Ok(Multipart::new(self.into_body(), boundary))
        } else {
            Err(routerify::Error::new("Content-Type is not multipart/form-data"))
        }
    }
}
