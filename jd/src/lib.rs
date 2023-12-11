pub enum Acid {
    Area,
    Category,
    Id,
}

#[derive(Debug, Default)]
pub struct Area {
    area: String,
    name: String,
}

#[derive(Debug, Default)]
pub struct Category {
    area: String,
    category: String,
    name: String,
}

/// An `id`, also known as a `JohnnyDecimal` number or an `ACID` (Area Category ID) number
#[derive(Debug, Default)]
pub struct Id {
    area: String,
    category: String,
    id: String,
    name: String,
    // acid: &'static str,
    // name: &'static str,
}

// Strict mode?
// impl Area {
//     pub fn from_str(str: &str) -> Result<Area, &str> {
//     }
// }

// impl Category {
//     pub fn new() -> Option<Category> {
//         Some(Category {
//             area: 0,
//             category: 0,
//         })
//     }
// }
//
// impl Id {
//     pub fn from_array(arr: [u8; 2]) -> Option<Id> {
//         Some(Id {
//             area: 0,
//             category: 0,
//             id: 3,
//         })
//     }
//
//     pub fn from_string(str: &str) -> Result<Id, &str> {
//         let chars: Vec<char> = str.chars().collect();
//
//         is_valid_acid(chars)?;
//
//         Ok(Id {
//             area: 0,
//             category: 0,
//             id: 0,
//         })
//         // Ok(Id {
//         //     area: Area::from_acid_a(chars[0].to_digit(10).unwrap()),
//         //     category: Category::from_acid_ac(chars[0].to_digit(10).unwrap(), chars[1].to_digit(10).unwrap()),
//         //     id: chars[3] as u8,
//         // })
//     }
// }

/// TODO: Allow for alternatives to the dash separator and space separator?
pub fn get_area(str: &str) -> Result<Area, &'static str> {
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

pub fn get_category(str: &str) -> Result<Category, &'static str> {
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

pub fn get_id(str: &str) -> Result<Id, &'static str> {
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
