/// An `id`, also known as a `JohnnyDecimal` number or an `ACID` (Area Category ID) number
pub struct Id {
    area: u8,
    category: u8,
    id: u8,
}

impl Id {
    pub fn new(id: u8) -> Option<Id> {
        Some(Id {
            area: 0,
            category: 0,
            id,
        })
    }
}
