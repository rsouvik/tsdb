

pub trait MgmtTable {
    fn open(&mut self);
    fn add(&mut self);
}


pub enum flushState {
    FlushNotReq,
    FlushReq,
    FlushSch
}

pub struct MemTable {

    path: String,
    flush_state: flushState

}

pub fn create_mem_table() -> Box<MemTable> {

    Box::new(MemTable{ path: String::from(("")), flush_state: flushState::FlushNotReq })
}

impl MgmtTable for MemTable {
    fn open(&mut self) {
        unimplemented!()
    }

    fn add(&mut self) {
        unimplemented!()
    }
}