
mod wal;
mod logger;
mod engine;
mod launcher;

use wal::WAL;
use crate::engine::{NewEngine, addWAL};
use crate::launcher::{Launcher, new_launcher};

fn main() {
    println!("Hello1, world!");
    let mut storage_eng = NewEngine(0);
    storage_eng.addNewWAL();

    //start launcher
    let mut launcher = new_launcher(0);


}
