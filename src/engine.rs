

pub struct Engine {
    current_segment_id: i32,
    segment_size: i32,
    mu: RwLock<i32>,
    logger: SimpleLogger,
}