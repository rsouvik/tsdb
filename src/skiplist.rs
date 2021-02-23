pub struct SkipList {

    path: String,
    flush_state: flushState

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

    fn insertKeyConcurrently() {
        self.mem.push([key,val]);
        unimplemented!()
    }

    fn contains() {
        unimplemented!()
    }

    fn get() {
        unimplemented!()
    }
}