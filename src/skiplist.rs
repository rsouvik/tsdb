use rand::Rng;
use std::cell::RefCell;
use std::clone::Clone;
use std::cmp::{Ord, Ordering};
use std::fmt::Display;
use std::option::Option;
use std::rc::{Rc, Weak};

pub struct SkipList {
    path: String,
    flush_state: flushState,
    mem: Vec<[String; 2]>,
}

/*
Skip Node implementation
*/

type Link<K, V> = Option<Rc<RefCell<SkipNode<K, V>>>>;
type WeakLink<K, V> = Option<Weak<RefCell<SkipNode<K, V>>>>;

pub struct SkipNode<K, V> {
    key: K,
    value: V,
    right: Link<K, V>,
    down: Link<K, V>,
    left: WeakLink<K, V>,
    up: WeakLink<K, V>,
}

impl<K, V> SkipNode<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            right: None,
            down: None,
            left: None,
            up: None,
        }
    }

    fn cmp(&self, value: &K) -> Ordering {
        self.key.cmp(value)
    }
}

struct Level<K, V> {
    size: usize,
    head: Link<K, V>,
}

impl<K, V> Level<K, V>
where
    K: Ord + Clone,
    V: Clone,
{

    fn new() -> Level<K, V> {
        Level {
            size: 0,
            head: None,
        }
    }

    fn iter(&self) -> Iter<K, V> {
        Iter {
            next: self.head.as_ref().map(Rc::clone),
        }
    }

    fn bisect(&mut self, key: &K) -> Link<K, V> {
        let maybe_marker = self.iter().find(|node_ref| {
            return match node_ref.borrow().cmp(key) {
                Ordering::Greater => true,
                Ordering::Less | Ordering::Equal => false,
            };
        });
        if maybe_marker.is_some() {
            let marker = maybe_marker.unwrap();
            return marker.borrow().left.as_ref().and_then(Weak::upgrade);
        }
        self.iter().last()
    }

    fn insert(&mut self, key: K, value: V) -> Rc<RefCell<SkipNode<K, V>>>{

    }

}




#[cfg(test)]
mod tests {

}

impl MemTableStore for SkipList {
    fn insertKey(&mut self) {
        unimplemented!()
    }

    fn insert(&mut self) {
        unimplemented!()
    }

    fn insertConcurrently() {
        unimplemented!()
    }

    fn insertKeyConcurrently(&self) {
        self.mem.push([key, val]);
        unimplemented!()
    }

    fn contains() {
        unimplemented!()
    }

    fn get() {
        unimplemented!()
    }
}
