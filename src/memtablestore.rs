use crate::memtable::flushState;

pub trait MemTableStore {
    fn insert_key(&mut self, key: String, val: String);
    fn insert(&mut self, key: String, val: String);
    fn insert_concurrently(&mut self);
    fn insert_key_concurrently(&mut self, key: String, val: String);
    fn contains(&mut self, key: String) -> bool;
    fn get(&mut self, key: String) -> String;
}

pub trait MemTableStoreFactory {

    fn create_mem_table_store(&mut self) -> Box<MemTableStore>;

}

pub struct SkipListStore {

    path: String,
    flush_state: flushState

}

impl MemTableStore for SkipListStore {
    fn insert_key(&mut self, key: String, val: String) {

        unimplemented!()
    }

    fn insert(&mut self, key: String, val: String) {


        unimplemented!()
    }

    fn insert_concurrently(&mut self) {


        unimplemented!()
    }

    fn insert_key_concurrently(&mut self, key: String, val: String) {
        unimplemented!()
    }

    fn contains(&mut self, key: String) -> bool{
        unimplemented!()
    }

    fn get(&mut self, key: String) -> String{
        unimplemented!()
    }
}

impl MemTableStoreFactory for SkipListStore {
    fn create_mem_table_store(&mut self) -> Box<MemTableStore> {
        unimplemented!()
    }
    //fn create_mem_table_store<'a>(&'a mut self) -> _ {
      //  unimplemented!()
    //}
}

pub fn create_skiplist_store() -> Box<MemTableStore> {
    Box::new(SkipListStore{ path: "".to_string(), flush_state: flushState::FlushNotReq })
}

