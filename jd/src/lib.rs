pub enum Acid {
    Area,
    Category,
    Id,
}

#[derive(Debug, Default)]
pub struct Area {
    pub area: String,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct Category {
    pub area: String,
    pub category: String,
    pub name: String,
}

/// An `id`, also known as a `JohnnyDecimal` number or an `ACID` (Area Category ID) number
#[derive(Debug, Default)]
pub struct Id {
    pub area: String,
    pub category: String,
    pub id: String,
    pub name: String,
}

impl Area {
    /// TODO: Allow for alternatives to the dash separator and space separator?
    pub fn from_str(str: &str) -> Result<Self, &'static str> {
        let chars: Vec<char> = str.chars().collect();

        if chars.len() < 5 {
            return Err("Given area is too short to follow a0-a9")
        }

        if !chars[0].is_ascii_digit() {
            return Err("Given area does not have valid starting digit a in a0")
        }

        if chars[1] != '0' {
            return Err("Given area does not have a starting digit that ends in 0 in a0-a9")
        }

        if chars[2] != '-' {
            return Err("Given area does not have dash separator in a0-a9")
        }

        if chars[3] != chars[0] {
            return Err("Given area does not have an ending digit equal to a in a0-a9")
        }

        if chars[4] != '9' {
            return Err("Given area does not end with 9 in a0-a9")
        }

        if chars.len() < 7 {
            return Err("Given area is too short to have a name")
        }

        if chars[5] != ' ' {
            return Err("Given area does not have a space separator")
        }

        Ok(Area {
            area: chars[0..5].into_iter().collect(),
            name: chars[6..chars.len()].into_iter().collect(),
        })
    }
}


impl Category {
    pub fn from_str(str: &str) -> Result<Self, &'static str> {
        let chars: Vec<char> = str.chars().collect();

        if chars.len() < 2 {
            return Err("Given category is too short to follow [00-99]")
        }

        if !chars[0].is_ascii_digit() || !chars[1].is_ascii_digit() {
            return Err("Given category does not have valid starting digits [00-99]")
        }

        if chars.len() < 4 {
            return Err("Given category is too short to have a name")
        }

        if chars[2] != ' ' {
            return Err("Given category does not have a space separator")
        }

        Ok(Category {
            category: chars[0..2].into_iter().collect(),
            area: [chars[0], '0', '-', chars[0], '9'].into_iter().collect(),
            name: chars[3..chars.len()].into_iter().collect(),
        })
    }
}

impl Id {
    pub fn from_str(str: &str) -> Result<Self, &'static str> {
        let chars: Vec<char> = str.chars().collect();

        if chars.len() < 5 {
            return Err("Given id is too short to follow ac.id")
        }

        if !chars[0].is_ascii_digit() {
            return Err("Given id does not have a valid starting area digit a in ac.id")
        }

        if !chars[1].is_ascii_digit() {
            return Err("Given id does not have a valid category ac in ac.id")
        }

        if chars[2] != '.' {
            return Err("Given id does not have a decimal separator in ac.id")
        }

        if !chars[3].is_ascii_digit() || !chars[4].is_ascii_digit() {
            return Err("Given id does not have valid digits id to follow ac.id")
        }

        if chars.len() < 7 {
            return Err("Given id is too short to have a name")
        }

        if chars[5] != ' ' {
            return Err("Given id does not have a space separator")
        }

        Ok(Id {
            id: chars[0..5].into_iter().collect(),
            category: chars[0..2].into_iter().collect(),
            area: [chars[0], '0', '-', chars[0], '9'].into_iter().collect(),
            name: chars[6..chars.len()].into_iter().collect(),
        })
    }
}
