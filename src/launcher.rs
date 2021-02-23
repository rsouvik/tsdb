extern crate hyper;
//extern crate tokio;

use tokio::net::TcpListener;
use tokio::prelude::*;

use crate::engine::{Engine, NewEngine};
use crate::logger::SimpleLogger;

use hyper::{Request,Response,Result};
use self::hyper::{Body, Server};
use std::net::SocketAddr;
use std::convert::Infallible;
use self::hyper::service::{service_fn, make_service_fn};
//use geno::{add, randnum};
//use geno::hel;

pub struct Launcher {
    engine: Option<Box<Engine>>,
    log_level: &'static str,
    //web_server: hyper::server::Server<&'static str, &'static str>,
    address: String,
    port: i32
}

pub fn new_launcher(id: i32) -> Box<Launcher> {
    let mut launcher: Box<Launcher> = Box::new (Launcher{
        engine: Option::from(NewEngine(0)),
        log_level: "WARN",
        //web_server: hyper::server::Server,
        address: "".parse().unwrap(),
        port: 443
    });
    run_http_server();

    return launcher;
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello World")))
}

#[tokio::main]
pub async fn run_http_server() {
    //let address = "127.0.0.1:8080".parse().unwrap();
    //let server = hyper::server::Http::new().bind(address)
      //  .unwrap();

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&address).serve(make_service);

    //let result = add(1,2, 3);
    //let result1 = randnum();
    //let result2 = hel();

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    //let server = hyper::server::Server::bind(&address);
    //server.run();
}
