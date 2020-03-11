extern crate hyper;
use crate::engine::Engine;
use crate::logger::SimpleLogger;

pub struct Launcher {
    engine: Option<Box<Engine>>,
    log_level: &'static str,
    //web_server: hyper::server::Server<&'static str, &'static str>,
    port: i32
}

pub fn new_launcher(id: i32) -> Box<Launcher> {
    let mut launcher: Box<Launcher> = Box::new (Launcher{
        engine: None,
        log_level: "WARN",
        //web_server: hyper::server::Server,
        port: 443
    });
    run_http_server();

    return launcher;
}

pub fn run_http_server() {
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new().bind()
        .unwrap();
}
