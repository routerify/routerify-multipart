use hyper::{Body, Request, Response, Server};
use routerify::{Error, Router, RouterService};
use routerify_multipart::RequestMultipartExt;
use std::net::SocketAddr;

async fn file_upload_handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    let mut multipart = req.into_multipart().map_err(|err| Error::wrap(err))?;

    while let Some(field) = multipart.next_field().await.map_err(|err| Error::wrap(err))? {
        let name = field.name();
        let file_name = field.file_name();

        println!("Name {:?}, File name: {:?}", name, file_name);

        let content = field.text().await.map_err(|err| Error::wrap(err))?;
        println!("Content {:?}", content);
    }

    Ok(Response::new(Body::from("Success")))
}

fn router() -> Router<Body, Error> {
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
