use crate::memtable;
use crate::memtable::flushState;
use crate::memtablestore::MemTableStore;
use std::sync::{Arc, Mutex};
use core::borrow::BorrowMut;
use std::ops::Deref;
use std::thread::spawn;
use std::clone::Clone;
use std::cmp::{Ord, Ordering};
use syn::parse::{Parse, ParseStream, Result};
use std::option::Option::{self, Some, None};
//use syn::parse::Unexpected::{None, Some};

#[derive(Debug, PartialEq)]
pub struct VNode<K,V> {
    key: K,
    value: V,
}

impl<K, V> VNode<K, V>
    where
        K: Ord + Clone,
        V: Clone,
{
    /*fn new(key: K, value: V) -> VNode<K,V>{
        VNode{
            key,
            value
        }
    }*/

    fn new_node(key: K, value: V) -> Self{
        Self{
            key,
            value,
        }
    }

    fn cmp(&self, value: &K) -> Ordering {
        self.key.cmp(value)
    }

    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }

    /*fn clone(&self) -> VNode<K,V> {
        VNode {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }*/

}

pub struct VectorTable<K,V> { //updated

    path: String,
    flush_state: flushState,
    //mem: Mutex<Vec<[String;2]>>
    mem: Vec<VNode<K,V>>,

}

/*impl<K,V> Clone for VectorTable<K,V> {
    fn clone(&self)->Self{
        VectorTable{
            path: self.path.clone(),
            flush_state: self.flush_state.clone(),
            mem: self.mem.clone(),
        }
    }
}*/

//impl Deref for VectorTable{
  //  type Target = Vec<[String;2]>;

    //fn deref(&self) -> &Self::Target {
      //  &self.mem
    //}
//}

impl<K,V> MemTableStore<K,V> for VectorTable<K,V>
    where
        K: Ord + Clone,
        V: Clone,
{
    fn insert_key(&mut self, key: K, val: V) {
        unimplemented!()
    }

    fn insert(&mut self, key: K, val: V) {
        unimplemented!()
    }

    fn insert_concurrently(&mut self) {
        unimplemented!()
    }

    /*fn insert_key(&mut self, key: K, val: V) {

        //self.mem.push([key,val]);
        //self.mem.push(Box::new(VNode{ key: (key), value: (val) }));
        self.mem.push(VNode::new(key,val));
    }

    fn insert(&mut self, key: K, val: V) {
       // self.mem.push([])
    }

    fn insert_concurrently(&mut self) {

        unimplemented!()
    }*/

    fn insert_key_concurrently(&mut self, key: K, value: V) {

        //let node = Rc::new(RefCell::new(SkipNode::new(key, value)));

        //let node = Rc::new(RefCell::new(VNode::new(key,val)));
        //self.mem.push(VNode::new(key.clone(),val.clone()));

        let node = Box::new(VNode::new_node(key,value));

        self.mem.push(*node);

        //self.mem.push(Rc::new(RefCell::new(VNode::new(key,val))));

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

    fn contains(&mut self, key: K) -> bool {
        unimplemented!()
    }

    //fn get(&mut self, key: K) -> V {
      //  unimplemented!()
    //}
    /*
        fn contains(&mut self, keyv: K) -> bool{
            for i in &mut self.mem {
                //if (*i)[0] == key {
                  //  return true
                //}

                if i.key.cmp(&keyv) == Ordering::Equal {
                    return true
                }
            }
            false
        }
*/
        fn get(&mut self, keyv: K) -> Option<&V>
        {
            //return self.mem.get(key);
            //for i in &mut self.mem {
            //    if (*i)[0] == key {
            //        return (*(i[1])).parse().unwrap();
            //    }
            //}
            for i in &mut self.mem {
                //if (*i).key == key {
                if i.key.cmp(&keyv) == Ordering::Equal {
                    //return (*i).value.parse().unwrap();
                    //return Some((*i).value);
                    return Option::Some(&(*i).value);
                    //return (*i).value;
                }
            }
            //return self.mem.last();
            //return (*i).value;
            //" ".parse().unwrap()
            //" "
            //String::from(" ");
            return Option::None;
        }
}

pub fn create_vector_store() -> Box<dyn MemTableStore<String,String>> {
    Box::new(VectorTable{
        path: "".to_string(),
        flush_state: flushState::FlushNotReq,
        mem: vec![]
    })
}

#[cfg(test)]
mod tests {
    use crate::vector::VectorTable;
    use crate::vector::VNode;
    use crate::memtablestore::MemTableStore;
    use crate::memtable::flushState::FlushNotReq;
    use std::sync::{Mutex, Arc};
    use std::thread::spawn;
    use std::cell::RefCell;
    use std::rc::Rc;
    use syn::parse::Parser;
//    use vector::VNode;

    /*#[test]
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
    }*/

    #[test]
    fn it_works_insert_concurrent_key() {

        let node = Box::new(VNode::new_node("sOUVIK","rAY"));
        let mut store =Box::new(VectorTable{
            path: "".to_string(),
            flush_state: FlushNotReq,
            mem: vec![*node],
        });

        //{
            let my_vector = Arc::new(Mutex::new(store)).clone();
            let update_thread = spawn(move || {
                let mut tx_guard = my_vector.lock().unwrap();
                //tx_guard.mem.push([key,val])
                //let k = String::from("Sou");
                //let v = String::from("2");
                let k = "sOUVIK";
                let v = "2";
                //tx_guard.insert_key_concurrently("Sou".to_owned().parse().unwrap(), "2".to_owned().parse().unwrap());
                //tx_guard.insert_key_concurrently(k.parse().unwrap(), v.parse().unwrap());
                tx_guard.insert_key_concurrently(k, v);

                //assert_eq!(tx_guard.get("Sou".parse().unwrap()), "2");
                //println!("{:?}",tx_guard.get("Sou".parse().unwrap()));
                println!("{:?}",tx_guard.get(&k));

            });
            update_thread.join().unwrap();
        //}
        //assert_eq!(store.get("Sou".parse().unwrap()), "2");
    }
}

