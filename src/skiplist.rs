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

    //bisect after
    fn bisect_after(&self, node: &Rc<RefCell<Node<K, V>>>, target: &K) -> Link<K, V> {
        if node.borrow().key.cmp(target) == Ordering::Greater {
            return None;
        }
        let mut maybe_current = Some(Rc::clone(node));
        let mut prev: Link<K, V> = node.borrow().left.as_ref().and_then(Weak::upgrade);
        let mut output = None;
        while maybe_current.is_some() {
            let current = maybe_current.take().unwrap();
            prev = Some(Rc::clone(&current));
            match current.borrow().cmp(target) {
                Ordering::Less => {
                    maybe_current = current.borrow().right.as_ref().map(Rc::clone);
                }
                Ordering::Equal => {
                    maybe_current = current.borrow().right.as_ref().map(Rc::clone);
                }
                Ordering::Greater => {
                    output = current.borrow().left.as_ref().and_then(Weak::upgrade);
                }
            };
            if output.is_some() {
                break;
            }
        }
        // found insertion point
        if output.is_some() {
            return output;
        }
        return prev;
    }

    fn insert_after(&mut self, key: K,
                    value: V,
                    after: Rc<RefCell<Node<K, V>>>
    ) -> Rc<RefCell<Node<K, V>>> {
        //Create the node first
        let node = Rc::new(RefCell::new(SkipNode::new(key,value)));
        let maybe_next_node = after.borrow_mut().right.take1();






    }

    fn insert(&mut self, key: K, value: V) -> Rc<RefCell<Node<K, V>>> {
        let mut head: Link<K, V> = self.head.as_ref().map(Rc::clone);
        let mut maybe_prev_node = Option::None;
        while head.is_some() {
            let node = head.take().unwrap();
            match node.borrow().cmp(&key) {
                Ordering::Less | Ordering::Equal => {
                    maybe_prev_node = Some(Rc::clone(&node));
                    head = node.borrow().right.as_ref().map(Rc::clone);
                }
                Ordering::Greater => {
                    break;
                }
            };
        }
        return match maybe_prev_node {
            // insert at head
            None => {
                let maybe_prev_head_ref: Option<Rc<RefCell<Node<K, V>>>> =
                    self.head.as_ref().map(Rc::clone);
                if maybe_prev_head_ref.is_some() {
                    let prev_head_ref = maybe_prev_head_ref.unwrap();
                    let new_head = Rc::new(RefCell::new(Node::new(key, value)));
                    new_head.borrow_mut().right = self.head.take();
                    self.head = Some(new_head);
                    prev_head_ref.borrow_mut().left = self.head.as_ref().map(Rc::downgrade);
                } else {
                    self.head = Some(Rc::new(RefCell::new(Node::new(key, value))));
                }
                self.size += 1;
                Rc::clone(self.head.as_ref().unwrap())
            }
            Some(prev_node) => {
                let maybe_next_node: Option<Rc<RefCell<Node<K, V>>>> =
                    prev_node.borrow().right.as_ref().map(Rc::clone);
                let new_node = Rc::new(RefCell::new(Node::new(key, value)));
                if maybe_next_node.is_some() {
                    // handle insert in the middle
                    let next_node = maybe_next_node.unwrap();
                    next_node.borrow_mut().left = Some(Rc::downgrade(&new_node));
                    new_node.borrow_mut().right = prev_node.borrow_mut().right.take();
                    new_node.borrow_mut().left = Some(Rc::downgrade(&prev_node));
                    prev_node.borrow_mut().right = Some(new_node);
                    self.size += 1;
                } else {
                    // handle insert at tail
                    new_node.borrow_mut().left = Some(Rc::downgrade(&prev_node));
                    prev_node.borrow_mut().right = Some(new_node);
                    self.size += 1;
                }
                Rc::clone(prev_node.borrow().right.as_ref().unwrap())
            }
        };
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
