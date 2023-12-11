pub mod area;
pub mod category;
pub mod id;

pub enum Acid {
    Area,
    Category,
    Id,
}

// use std::path::PathBuf;

pub struct Area {
    area: u8,
}

// Strict mode?
impl Area {
    pub fn new(n: u8) -> Option<Area> {
        Some(Area {
            area: n,
        })
    }

    pub fn from_string(str: &str) -> Result<Area, &str> {
        let range: [u8; 2] = [0, 0];
        let str = str.split(' ');
        dbg!(str);
        Err("lol")
    }

    pub fn from_acid_a(a: char) -> Result<Area, &'static str> {
        if let Some(number) = a.to_digit(10) {
            return Ok(Area {
                area: 4
            })
        }

        Err("Given Area in AC.ID was not a digit")
    }

    // pub fn from_path(path: PathBuf) -> Result<Area, &str> {
    //
    // }
}

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

/// An `id`, also known as a `JohnnyDecimal` number or an `ACID` (Area Category ID) number
pub struct Id {
    area: u8,
    category: u8,
    id: u8,
    // acid: &'static str,
    // name: &'static str,
}

impl Id {
    pub fn from_array(arr: [u8; 2]) -> Option<Id> {
        Some(Id {
            area: 0,
            category: 0,
            id: 3,
        })
    }

    pub fn from_string(str: &str) -> Result<Id, &str> {
        let chars: Vec<char> = str.chars().collect();

        is_valid_acid(chars)?;

        Ok(Id {
            area: 0,
            category: 0,
            id: 0,
        })
        // Ok(Id {
        //     area: Area::from_acid_a(chars[0].to_digit(10).unwrap()),
        //     category: Category::from_acid_ac(chars[0].to_digit(10).unwrap(), chars[1].to_digit(10).unwrap()),
        //     id: chars[3] as u8,
        // })
    }
}

fn is_valid_acid(chars: Vec<char>) -> Result<bool, &'static str> {
    if chars.len() < 4 {
        return Err("Given string is too short to follow AC.ID")
    }

    if !chars[0].is_ascii_digit() || !chars[1].is_ascii_digit() {
        return Err("Given string does not have digits for Area/Category in AC.ID")
    }

    if chars[2] != '.' {
        return Err("Given string does not have period separator in AC.ID")
    }

    if !chars[3].is_ascii_digit() || !chars[4].is_ascii_digit() {
        return Err("Given string does not have digits for ID in AC.ID")
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        assert!(Id::from_string("Just a regular folder").is_err());
        assert!(Id::from_string("12.03 Johnny Decimal ID").is_ok());
        assert!(Id::from_string("1.03 Johnny Decimal ID").is_err());
    }

    // #[test]
    // fn test_is_area() {
    //     assert_eq!(is_area("10-19 Area"), true);
    //     assert_eq!(is_area("10-30 Area"), false);
    // }
    //
    // #[test]
    // fn test_is_category() {
    //     assert_eq!(is_category("11 Category"), true);
    //     assert_eq!(is_category("120 Category"), false);
    // }
    //
    // #[test]
    // fn test_is_id() {
    //     assert_eq!(is_id("12.04 Name"), true);
    // }
}
