use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::{Error, Router, RouterService};
// Import `RequestMultipartExt` trait.
use routerify_multipart::RequestMultipartExt;
use std::net::SocketAddr;

// A handler to handle file uploading in `multipart/form-data` content-type.
async fn file_upload_handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Convert the request into a `Multipart` instance.
    let mut multipart = match req.into_multipart() {
        Ok(m) => m,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Bad Request: {}", err)))
                .unwrap());
        }
    };

    // Iterate over the fields.
    while let Some(mut field) = multipart.next_field().await.map_err(|err| Error::wrap(err))? {
        // Get the field name.
        let name = field.name();
        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name();

        println!("Name {:?}, File name: {:?}", name, file_name);

        // Process the field data chunks e.g. store them in a file.
        while let Some(chunk) = field.chunk().await.map_err(|err| Error::wrap(err))? {
            // Do something with field chunk.
            println!("Chunk: {:?}", chunk);
        }
    }

    Ok(Response::new(Body::from("Success")))
}

// Create a router.
fn router() -> Router<Body, Error> {
    // Register the handlers.
    Router::builder().post("/upload", file_upload_handler).build().unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    // The address on which the server will be listening.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Create a server by passing the created service to `.serve` method.
    let server = Server::bind(&addr).serve(service);

    println!("App is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
