
mod wal;
mod logger;
mod engine;
mod launcher;
mod memtable;
mod vector;
mod memtablestore;

use wal::WAL;
use memtable::create_mem_table;
use crate::engine::{NewEngine, addWAL};
use crate::launcher::{Launcher, new_launcher};
use crate::memtablestore::{SkipListStore, create_skiplist_store};
use crate::vector::create_vector_store;

/*
Create in-mem sketches for ML

*/

fn main() {
    println!("Hello1, world!");

    let mut mem_table = create_mem_table();
    //let mut storage_eng = NewEngine(0);
    //storage_eng.addNewWAL();

    //start launcher
    let mut launcher = new_launcher(0);
    //Add key/value
    let mut skip_ls_st = create_skiplist_store();

    let mut vec_st = create_vector_store();
    //skip_ls_st.insert();
    //skip_ls_st.insert_key();
    //skip_ls_st.insert_key_concurrently();




}
