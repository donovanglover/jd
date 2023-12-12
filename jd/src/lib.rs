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

#[derive(Debug)]
pub struct Index {
    pub areas: Vec<Area>,
    pub categories: Vec<Category>,
    pub ids: Vec<Id>,
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
            area: chars[0..5].iter().collect(),
            name: chars[6..chars.len()].iter().collect(),
        })
    }
}

impl PartialEq for Area {
    fn eq(&self, other: &Area) -> bool {
        self.area == other.area
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
            category: chars[0..2].iter().collect(),
            area: [chars[0], '0', '-', chars[0], '9'].iter().collect(),
            name: chars[3..chars.len()].iter().collect(),
        })
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Category) -> bool {
        self.category == other.category
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
            id: chars[0..5].iter().collect(),
            category: chars[0..2].iter().collect(),
            area: [chars[0], '0', '-', chars[0], '9'].iter().collect(),
            name: chars[6..chars.len()].iter().collect(),
        })
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.id == other.id
    }
}

impl Index {
    pub fn from_str(str: &str) -> Result<Index, &str> {
        let mut areas: Vec<Area> = vec![];
        let mut categories: Vec<Category> = vec![];
        let mut ids: Vec<Id> = vec![];

        for line in str.lines() {
            let line = line.trim_start_matches(' ');

            if let Ok(id) = Id::from_str(line) {
                if ids.contains(&id) {
                    return Err("Duplicate ids are not allowed");
                }

                if !categories.iter().any(|c| c.category == id.category) {
                    return Err("A given id has no associated category");
                }

                ids.push(id);
                continue;
            }

            if let Ok(category) = Category::from_str(line) {
                if categories.contains(&category) {
                    return Err("Duplicate categories are not allowed");
                }

                if !areas.iter().any(|a| a.area == category.area) {
                    return Err("A given category has no associated area");
                }

                categories.push(category);
                continue;
            }

            if let Ok(area) = Area::from_str(line) {
                if areas.contains(&area) {
                    return Err("Duplicate areas are not allowed");
                }

                areas.push(area);
                continue;
            }

            return Err("Given value was neither an Area, Category, or Id")
        }

        Ok(Index {
            areas,
            categories,
            ids,
        })
    }
}
