use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::{Service};
use hyper::{Request, Response, body::Incoming as IncomingBody};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Read this from config
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    // TODO: abstract to avoid tight coupling with the library
    // We create a TcpListener and bind it to 127.0.0.1:8000
    let listener = TcpListener::bind(addr).await?;

    // accept connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            // TODO: add http2 listener
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(stream, Handler {})
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

struct Handler {

}

impl Service<Request<IncomingBody>> for Handler {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        // TODO: add router here

        let res = match req.uri().path() {
            "/" => mk_response(format!("home! ")),
            _ => return Box::pin(async { mk_response("oh no! not found".into()) }),
        };

        Box::pin(async { res })
    }
}
