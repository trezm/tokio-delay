use std::convert::Infallible;

use hyper::server::conn::Http;
use hyper::server::Builder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use std::net::ToSocketAddrs;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();


    let server = Builder::new(
        hyper::server::accept::from_stream(rx),
        Http::new(),
    )
    .serve(make_svc);

    println!("Listening...");
    
    let addr = ("127.0.0.1", 3000)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let mut listener = TcpListener::bind(&addr).await.unwrap();
    let mut incoming = listener.incoming();

    tokio::spawn(async move {
	let _ = server.await;
    });
    
    while let Some(item) = incoming.next().await {
        let tx = tx.clone();

        tokio::spawn(async move {
            tokio::time::delay_for(std::time::Duration::from_millis(5000)).await;

            let _ = tx.send(item);
        });
    }

    Ok(())
}
