[![Github Actions Status](https://github.com/routerify/routerify-multipart/workflows/Test/badge.svg)](https://github.com/routerify/routerify-multipart/actions)
[![crates.io](https://img.shields.io/crates/v/routerify-multipart.svg)](https://crates.io/crates/routerify-multipart)
[![Documentation](https://docs.rs/routerify-multipart/badge.svg)](https://docs.rs/routerify-multipart)
[![MIT](https://img.shields.io/crates/l/routerify-multipart.svg)](./LICENSE)

# routerify-multipart

A `multipart/form-data` parser for [`Routerify`](https://github.com/routerify/routerify).

It's using [multer](https://github.com/rousan/multer-rs) to parse the `multipart/form-data` content.

[Docs](https://docs.rs/routerify-multipart)

## Install

Add this to your `Cargo.toml` file:

```toml
[dependencies]
routerify = "1.1"
routerify-multipart = "1.1"
```

## Example

```rust
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
``` 

## Prevent DDoS Attack
This crate also provides some APIs to prevent potential `DDoS attack` with fine grained control. It's recommended to add some constraints
on field (specially text field) size to avoid potential `DDoS attack` from attackers running the server out of memory.

An example:

```rust
use hyper::{Body, Request, Response, Server, StatusCode};
use routerify::{Error, Router, RouterService};
// Import `RequestMultipartExt` trait and other types.
use routerify_multipart::{RequestMultipartExt, Constraints, SizeLimit};
use std::net::SocketAddr;

// A handler to handle file uploading in `multipart/form-data` content-type.
async fn file_upload_handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Create some constraints to be applied to the fields to prevent DDoS attack.
     let constraints = Constraints::new()
         // We only accept `my_text_field` and `my_file_field` fields,
         // For any unknown field, we will throw an error.
         .allowed_fields(vec!["my_text_field", "my_file_field"])
         .size_limit(
             SizeLimit::new()
                 // Set 15mb as size limit for the whole stream body.
                 .whole_stream(15 * 1024 * 1024)
                 // Set 10mb as size limit for all fields.
                 .per_field(10 * 1024 * 1024)
                 // Set 30kb as size limit for our text field only.
                 .for_field("my_text_field", 30 * 1024),
          );

    // Convert the request into a `Multipart` instance.
    let mut multipart = match req.into_multipart_with_constraints(constraints) {
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
```

## Contributing

Your PRs and suggestions are always welcome.
