use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

mod lscmd;
use crate::lscmd::lscmd;
use crate::lscmd::Service;

async fn handle(_: Request<Body>) -> std::result::Result<hyper::Response<Body>, hyper::http::Error>
{
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<Service>>(1);
    tokio::spawn(async move {
        lscmd("http://192.168.0.200:40772/api/services", tx).unwrap();
    });
    let services = rx.recv().await.unwrap();
    let ul = services.into_iter()
        .map(|s| format!("<dt>{}</dt><dd>{}</dd>", s.name, s.id))
        .collect::<Vec<_>>();
    println!("{:?}", ul);
    Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(Body::from(r#"
            <h1>Services</h1>
        "#))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

}
