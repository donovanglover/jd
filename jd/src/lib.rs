use std::path::PathBuf;

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
impl Area {
    pub fn from_path(path: &PathBuf) -> Result<Area, &str> {
        if let Some(filename) = path.file_name() {
            if let Some(str) = filename.to_str() {
                if let Ok(area) = get_area(str) {
                    return Ok(area);
                } else { Err("Not a valid area" ) }
            } else { Err("Couldn't convert path to string") }
        } else { Err("Couldn't get filename from path") }
    }
}

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

pub fn get_area(str: &str) -> Result<Area, &'static str> {
    let chars: Vec<char> = str.chars().collect();

    if chars.len() < 4 {
        return Err("Given area is too short to follow a0-a9")
    }

    if !chars[0].is_ascii_digit() || chars[1] != '0' {
        return Err("Given area does not have valid starting digits a0")
    }

    if chars[2] != '-' {
        return Err("Given area does not have dash separator in a0-a9")
    }

    if chars[3] != chars[0] || !chars[4].is_ascii_digit() {
        return Err("Given area does not have valid ending digits a9")
    }

    if chars.len() < 6 || chars[5] != ' ' {
        return Err("Given area is too short to have a name")
    }

    Ok(Area {
        area: chars[0..4].into_iter().collect(),
        name: chars[5..chars.len()].into_iter().collect(),
    })
}

// fn is_valid_acid(chars: Vec<char>) -> Result<bool, &'static str> {
//     if chars.len() < 4 {
//         return Err("Given string is too short to follow AC.ID")
//     }
//
//     if !chars[0].is_ascii_digit() || !chars[1].is_ascii_digit() {
//         return Err("Given string does not have digits for Area/Category in AC.ID")
//     }
//
//     if chars[2] != '.' {
//         return Err("Given string does not have period separator in AC.ID")
//     }
//
//     if !chars[3].is_ascii_digit() || !chars[4].is_ascii_digit() {
//         return Err("Given string does not have digits for ID in AC.ID")
//     }
//
//     Ok(true)
// }
