use crate::memtable;
use crate::memtable::flushState;
use crate::memtablestore::MemTableStore;
use std::sync::{Arc, Mutex};
use core::borrow::BorrowMut;
use std::ops::Deref;
use std::thread::spawn;

pub struct VectorTable { //updated

    path: String,
    flush_state: flushState,
    //mem: Mutex<Vec<[String;2]>>
    mem: Vec<[String;2]>

}

//impl Deref for VectorTable{
  //  type Target = Vec<[String;2]>;

    //fn deref(&self) -> &Self::Target {
      //  &self.mem
    //}
//}

impl MemTableStore for VectorTable {
    fn insert_key(&mut self, key: String, val: String) {

        self.mem.push([key,val]);
    }

    fn insert(&mut self, key: String, val: String) {
       // self.mem.push([])
    }

    fn insert_concurrently(&mut self) {

        unimplemented!()
    }

    fn insert_key_concurrently(&mut self, key: String, val: String) {

        self.mem.push([key,val]);

        /*
        //let my_vector = Arc::new(&self);
        //{
        let my_vector = Arc::new(Mutex::new(self)).clone();

        let file_feed = spawn(move || {
            let mut tx_guard = my_vector.lock().unwrap();
            tx_guard.mem.push([key,val])
        });
        file_feed.join().unwrap();


            //let my_vector = Arc::new(&self.mem);
            //let mem_update = my_vector.clone();
            //{
            //let mut tx_guard = my_vector.lock().unwrap();
            //let mut tx_guard = mem_update.lock().unwrap();
            //tx_guard.push([key, val])
            //tx_guard.push([key,val])
        //tx_guard.push([key,val])
        //}
        //}
        //tx_guard.push([key,val]);

        */

    }

    fn contains(&mut self, key: String) -> bool{
        for i in &mut self.mem {
            if (*i)[0] == key {
                return true
            }
        }
        false
    }

    fn get(&mut self, key: String) -> String
    {
        //return self.mem.get(key);
        for i in &mut self.mem {
            if (*i)[0] == key {
                return (*(i[1])).parse().unwrap();
            }
        }
        " ".parse().unwrap()
    }
}

pub fn create_vector_store() -> Box<MemTableStore> {
    Box::new(VectorTable{
        path: "".to_string(),
        flush_state: flushState::FlushNotReq,
        mem: vec![]
    })
}

#[cfg(test)]
mod tests {
    use crate::vector::VectorTable;
    use crate::memtablestore::MemTableStore;
    use crate::memtable::flushState::FlushNotReq;
    use std::sync::{Mutex, Arc};
    use std::thread::spawn;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn it_works_insert_key() {
        let mut store = Box::new(VectorTable{
            path: "".to_string(),
            flush_state: FlushNotReq,
            //mem: Vec::from((["Souvik", "1"], ["Ray", "2"]))
            //mem: Vec::from(vec![["Souvik", "1"], ["Ray", "2"]])
            //mem: vec![["Souvik".to_string(), "1".to_string()]]
            mem: vec![["Souvik".to_string(), "1".to_string()]]
        });

        store.insert_key("Sou".parse().unwrap(), "Ray".parse().unwrap());

        assert_eq!(store.get("Souvik".parse().unwrap()), "1");
        assert_eq!(store.get("Sou".parse().unwrap()), "Ray")
    }

    #[test]
    fn it_works_insert_concurrent_key() {

        let mut store =Box::new(VectorTable{
            path: "".to_string(),
            flush_state: FlushNotReq,
            mem: vec![["Souvik".to_string(), "1".to_string()]]
        });

        //{
            let my_vector = Arc::new(Mutex::new(store)).clone();
            let update_thread = spawn(move || {
                let mut tx_guard = my_vector.lock().unwrap();
                //tx_guard.mem.push([key,val])
                tx_guard.insert_key_concurrently("Sou".parse().unwrap(), "2".parse().unwrap());
                assert_eq!(tx_guard.get("Sou".parse().unwrap()), "2");
                println!("{:?}",tx_guard.get("Sou".parse().unwrap()));
            });
            update_thread.join().unwrap();
        //}
        assert_eq!(store.get("Sou".parse().unwrap()), "2");
    }
}