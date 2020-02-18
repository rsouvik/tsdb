use crate::wal::WAL;
use crate::logger::SimpleLogger;
use std::sync::RwLock;

pub struct Engine {
    current_segment_id: i32,
    segment_size: i32,
    mu: RwLock<i32>,
    logger: SimpleLogger,
}

//NewEngine initialises a new storage engine, including a series file, index and
// TSM engine.
fn NewEngine(id : i32) -> Engine {
    //let n_wal : WAL = WAL { current_segment_id: (id), logger: SimpleLogger, mu: RwLock::new(5), segment_size: 1024};
    //return n_wal;

    //WAL { current_segment_id: (id), logger: SimpleLogger, mu: RwLock::new(5), segment_size: 1024}
    //Initialize WAL
    let wal : Box<WAL> = NewWAL();
}