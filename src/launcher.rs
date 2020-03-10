extern crate hyper;
use crate::engine::Engine;
use crate::logger::SimpleLogger;

pub struct Launcher {
    engine: Option<Box<Engine>>,
    logLevel: str,
    webServer: hyper::server,
    port: i32
}

impl Launcher {}

pub fn NewLauncher(id: i32) -> Box<Launcher> {
    None
}
