use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::env;

mod lscmd;
use crate::lscmd::lscmd;
use crate::lscmd::Service;

async fn handle(
    _: Request<Body>,
) -> std::result::Result<hyper::Response<Body>, hyper::http::Error> {
    let (tx, rx) = tokio::sync::oneshot::channel::<Vec<Service>>();
    tokio::spawn(async move {
        let lsurl = env::var("LS_MIRA_SERCIVES_LSURL").unwrap();
        lscmd(&lsurl, tx).unwrap();
    });
    let services = rx.await.unwrap();

    let lsurl = env::var("LS_MIRA_SERCIVES_LSURL").unwrap();
    let suffix = env::var("LS_MIRA_SERCIVES_SUFFIX").unwrap();
    let ls = services
        .into_iter()
        .map(|s| (s.name, format!("{}/{}{}", &lsurl, s.id, &suffix)))
        .map(|(name, url)| format!(r#"<dt>{0}</dt><dd><a href="{1}">{1}</a></dd>"#, name, url))
        .collect::<Vec<_>>();
    let content = ls.iter().fold(String::new(), |acc, x| acc + x);

    Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(Body::from(
            r#"
             <h1>Services</h1>
            "#
            .to_string()
                + &content,
        ))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
