use crate::wal::{WAL, NewWAL};
use crate::logger::SimpleLogger;
use std::sync::RwLock;
use std::ptr::null;
use std::option::Option::{self, Some, None};

pub trait addWAL {
    fn addNewWAL(&mut self);
}

pub struct Engine {
    current_segment_id: i32,
    segment_size: i32,
    mu: RwLock<i32>,
    logger: SimpleLogger,
    //wal: Box<WAL>,
    wal: Option<Box<WAL>>
}

impl addWAL for Engine {

    fn addNewWAL(&mut self) {
        self.wal = Option::from(NewWAL(String::from(""), 5));
    }
}

//NewEngine initialises a new storage engine, including a series file, index and
// TSM engine.
pub fn NewEngine(id : i32) -> Box<Engine> {
    //let n_wal : WAL = WAL { current_segment_id: (id), logger: SimpleLogger, mu: RwLock::new(5), segment_size: 1024};
    //return n_wal;`

    //WAL { current_segment_id: (id), logger: SimpleLogger, mu: RwLock::new(5), segment_size: 1024}
    //Initialize WAL
    //let wal : Box<WAL> = NewWAL(" ".parse().unwrap(), id);
    //return
    let mut eng: Box<Engine> = Box::new(Engine{
        current_segment_id: 0,
        segment_size: 0,
        mu: RwLock::new(5),
        logger: SimpleLogger,
        wal: Option::None
    });
    eng.wal = Option::from(NewWAL(String::from(""), 5));
    return eng
}