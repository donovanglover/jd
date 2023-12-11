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
