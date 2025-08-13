pub struct Road {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub lane_count: u8,
}

impl Road {
    pub fn new(start: (i32, i32), end: (i32, i32), lane_count: u8) -> Self {
        Self { start, end, lane_count }
    }
}
