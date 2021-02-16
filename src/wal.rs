
//use log::{Record, Level, Metadata};

//struct SimpleLogger;
use std::sync::RwLock;
use crate::logger::SimpleLogger;

const DefaultSegmentSize : i32 = 10 * 1024 * 1024;

const WALFileExtension : &str = "wal";

// WALFilePrefix is the prefix on all wal segment files.
const WALFilePrefix : &str = "_";

// walEncodeBufSize is the size of the wal entry encoding buffer
const walEncodeBufSize : i32 = 4 * 1024 * 1024;

const float64EntryType : i32  = 1;
const integerEntryType : i32  = 2;
const booleanEntryType : i32  = 3;
const stringEntryType :  i32  = 4;
const unsignedEntryType : i32 = 5;


pub struct WAL {
    current_segment_id: i32,
    segment_size: i32,
    mu: RwLock<i32>,
    logger: SimpleLogger,
    path: String,
}

pub fn NewWAL(path: String, id: i32) -> Box<WAL> {
    //let n_wal : WAL = WAL { current_segment_id: (id), logger: SimpleLogger, mu: RwLock::new(5), segment_size: 1024};
    //return n_wal;

    Box::new(WAL { current_segment_id: (id), logger: SimpleLogger, mu: RwLock::new(5), segment_size: 1024, path })
}