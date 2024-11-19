extern crate syn;

mod wal;
mod logger;
mod engine;
mod launcher;
mod memtable;
mod vector;
mod memtablestore;
mod skiplist;
mod stor;

use wal::WAL;
use memtable::create_mem_table;
use crate::engine::{NewEngine, addWAL};
use crate::launcher::{Launcher, new_launcher, start_launcher, stop_launcher};
use crate::memtablestore::{SkipListStore, create_skiplist_store};
use crate::vector::create_vector_store;

/*
Create in-mem sketches for ML
Block store
*/

fn main() {
    println!("Hello1, world!");

    let mut mem_table = create_mem_table();
    let mut storage_eng = NewEngine(0);
    storage_eng.addNewWAL();

    //start launcher
    //let mut launcher = new_launcher(0);
    //start_launcher(Launcher::from(launcher));

    //stop_launcher(launcher);

    //Add key/value
    let mut skip_ls_st = create_skiplist_store();

    let mut vec_st = create_vector_store();
    vec_st.insert_key_concurrently("Sou".parse().unwrap(), "Ray".parse().unwrap());
    vec_st.insert_key_concurrently("Sou1".parse().unwrap(), "Ray1".parse().unwrap());
    vec_st.insert_key("Sou1".parse().unwrap(), "Ray1".parse().unwrap());

    //skip_ls_st.insert();
    //skip_ls_st.insert_key();
    //skip_ls_st.insert_key_concurrently();




}
