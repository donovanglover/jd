pub struct Area {
    number: u8,
}

// Strict mode?
impl Area {
    pub fn new(n: u8) -> Option<Area> {
        Some(Area {
            number: n,
        })
    }
}
