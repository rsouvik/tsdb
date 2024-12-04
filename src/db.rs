


pub trait openDB {
    fn open(&mut self);
    fn open_for_read_only(&mut self);
    fn close(&mut self);
    fn get(&mut self);
    fn put(&mut self);
}

OpenForReadOnly();

pub struct Db {

    path: string,

}

impl openDB for Db {
    fn open(&mut self) {
        unimplemented!()
    }

    fn open_for_read_only(&mut self) {
        unimplemented!()
    }

    fn close(&mut self) {
        unimplemented!()
    }

    fn get(&mut self) {
        unimplemented!()
    }

    fn put(&mut self) {
        unimplemented!()
    }
}