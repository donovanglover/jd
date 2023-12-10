pub struct Category {
    area: u8,
    category: u8,
}

impl Category {
    pub fn new() -> Option<Category> {
        Some(Category {
            area: 0,
            category: 0,
        })
    }
}
