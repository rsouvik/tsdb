use crate::memtable::flushState;
use crate::vector::VectorTable;

pub trait MemTableStore<K,V> {
    fn insert_key(&mut self, key: K, val: V);
    fn insert(&mut self, key: K, val: V);
    fn insert_concurrently(&mut self);
    fn insert_key_concurrently(&mut self, key: K, val: V);
    fn contains(&mut self, key: K) -> bool;
    fn get(&mut self, key: K) -> Option<&V>;
}

pub trait MemTableStoreFactory<K,V> {

    fn create_mem_table_store(&mut self) -> Box<dyn MemTableStore<K,V>>;

}

pub struct SkipListStore {

    path: String,
    flush_state: flushState

}

impl<K,V> MemTableStore<K,V> for SkipListStore {
    fn insert_key(&mut self, key: K, val: V) {

        unimplemented!()
    }

    fn insert(&mut self, key: K, val: V) {


        unimplemented!()
    }

    fn insert_concurrently(&mut self) {


        unimplemented!()
    }

    fn insert_key_concurrently(&mut self, key: K, val: V) {
        unimplemented!()
    }

    fn contains(&mut self, key: K) -> bool{
        unimplemented!()
    }

    fn get(&mut self, key: K) -> Option<&V>{
        unimplemented!()
    }
}

impl<K,V> MemTableStoreFactory<K,V> for SkipListStore {

    fn create_mem_table_store(&mut self) -> Box<dyn MemTableStore<K,V>> {
        unimplemented!()
    }
    //fn create_mem_table_store<'a>(&'a mut self) -> _ {
      //  unimplemented!()
    //}
}

pub fn create_skiplist_store() -> Box<dyn MemTableStore<String,String>> {
    Box::new(SkipListStore{ path: "".to_string(), flush_state: flushState::FlushNotReq })
}


