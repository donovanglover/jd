#![warn(missing_docs)]

//! This library implements the [Johnny.Decimal Index Specification].
//!
//! There are 3 main structs: `Area`, `Category`, and `Id`.
//!
//! For simplicity, these struct fields are stored and compared as `String`s.
//!
//! # Example
//!
//! ```
//! use johnnydecimal::{Area, Category, Id};
//!
//! if let Ok(area) = Area::new("10-19 Area") {
//!     assert!(area.get_name() == "Area");
//! }
//!
//! assert!(Category::new("11 Area").is_ok());
//! ```
//!
//! # Methodology
//!
//! The `struct` implementations perform validation of given `&str`s.
//!
//! If validation is successful, an `Ok` is returned with the fields of that struct.
//!
//! Otherwise, a friendly error message you can show to your users is returned in `Err`.
//!
//! The use of `String` makes it easy to use this library in both Rust and JavaScript (through
//! WebAssembly/Wasm) without having to worry about custom types.
//!
//! # Usage
//!
//! Since I haven't published this crate to `crates.io`, you'll need to source the Git repository
//! directly.
//!
//! ```toml
//! [dependencies]
//! jd = { git = "https://github.com/donovanglover/jd.git" }
//! ```
//!
//! # Performance
//!
//! This crate uses no external dependencies. All functionality is achieved with Rust's standard
//! library, making it extremely fast to compile and use in other projects.
//!
//! [Johnny.Decimal Index Specification]: https://github.com/johnnydecimal/jdcm.al__index-spec

use std::cmp::Ordering;

/// `10-19 Area`
///
/// An `Area` is derived from a `&str` in the format `a0-a9 <title>` where `a` = `[0..9]`.
///
/// <https://github.com/johnnydecimal/jdcm.al__index-spec#areas>
///
/// # Example
///
/// ```
/// use johnnydecimal::Area;
///
/// if let Ok(area) = Area::new("20-29 My Area") {
///     assert_eq!(area.get_area(), "20-29");
///     assert_eq!(area.get_name(), "My Area");
/// } else {
///     panic!("Invalid area");
/// }
///
/// ```
#[derive(Debug, Default, Eq)]
pub struct Area {
    /// Area `10-19`: The string `a0-a9` derived from `a0-a9 <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Area;
    ///
    /// let area = Area::new("30-39 My Area").expect("Invalid area");
    ///
    /// assert!(area.get_area() == "30-39");
    /// ```
    area: String,

    /// Title `My Title`: The string `<title>` derived from `a0-a9 <title>`
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Area;
    ///
    /// let area = Area::new("30-39 My Area").expect("Invalid area");
    ///
    /// assert!(area.get_name() == "My Area");
    /// ```
    name: String,
}

/// `11 Category`
///
/// A `Category` is derived from a `&str` in the format `ac <title>` where `ac` = `[00..99]`.
///
/// For example, a category `25` has an area `20-29`.
///
/// <https://github.com/johnnydecimal/jdcm.al__index-spec#categories>
///
/// # Example
///
/// ```
/// use johnnydecimal::Category;
///
/// if let Ok(category) = Category::new("42 My Category") {
///     assert_eq!(category.get_area(), "40-49");
///     assert_eq!(category.get_category(), "42");
///     assert_eq!(category.get_name(), "My Category");
/// } else {
///     panic!("Invalid category");
/// }
///
/// ```
#[derive(Debug, Default, Eq)]
pub struct Category {
    /// Area `10-19`: The string `a0-a9` derived from `ac <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Category;
    ///
    /// let category = Category::new("53 My Category").expect("Invalid category");
    ///
    /// assert!(category.get_area() == "50-59");
    /// ```
    area: String,

    /// Category `11`: The string `ac` derived from `ac <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Category;
    ///
    /// let category = Category::new("53 My Category").expect("Invalid category");
    ///
    /// assert!(category.get_category() == "53");
    /// ```
    category: String,

    /// Title `My Title`: The string `<title>` derived from `ac <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Category;
    ///
    /// let category = Category::new("53 My Category").expect("Invalid category");
    ///
    /// assert!(category.get_name() == "My Category");
    /// ```
    name: String,
}

/// `11.01 Id`
///
/// An `Id` is derived from a `&str` in the format `ac.id <title>` where id = `[00..99]`.
///
/// For example, an id `23.05` has a category `23` and an area `20-29`.
///
/// <https://github.com/johnnydecimal/jdcm.al__index-spec#ids>
///
/// # Example
///
/// ```
/// use johnnydecimal::Id;
///
/// if let Ok(id) = Id::new("35.04 My Id") {
///     assert_eq!(id.get_area(), "30-39");
///     assert_eq!(id.get_category(), "35");
///     assert_eq!(id.get_id(), "35.04");
///     assert_eq!(id.get_name(), "My Id");
/// } else {
///     panic!("Invalid id");
/// }
///
/// ```
#[derive(Debug, Default, Eq)]
pub struct Id {
    /// Area `10-19`: The string `a0-a9` derived from `ac.id <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Id;
    ///
    /// let id = Id::new("39.12 My Id").expect("Invalid id");
    ///
    /// assert!(id.get_area() == "30-39");
    /// ```
    area: String,

    /// Category `11`: The string `ac` derived from `ac.id <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Id;
    ///
    /// let id = Id::new("39.12 My Id").expect("Invalid id");
    ///
    /// assert!(id.get_category() == "39");
    /// ```
    category: String,

    /// Id `11.01`: The string `ac.id` derived from `ac.id <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Id;
    ///
    /// let id = Id::new("39.12 My Id").expect("Invalid id");
    ///
    /// assert!(id.get_id() == "39.12");
    /// ```
    id: String,

    /// Title `My Title`: The string `<title>` derived from `ac.id <title>`.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Id;
    ///
    /// let id = Id::new("39.12 My Id").expect("Invalid id");
    ///
    /// assert!(id.get_name() == "My Id");
    /// ```
    name: String,
}

/// `00.00 Index.txt`
///
/// An `Index` is a flat file database for Johnny.Decimal systems.
///
/// - Must be in order
/// - Parents may be childless
/// - Orphans are forbidden
/// - White space is ignored
/// - JavaScript comments are allowed
/// - Key/value metadata pairs are allowed
///
/// <https://github.com/johnnydecimal/jdcm.al__index-spec#order>
///
/// # Example
///
/// ```
/// use johnnydecimal::Index;
///
/// if let Ok(index) = Index::new("10-19 Area\n13 Category\n13.05 Id") {
///     assert!(index.areas.iter().count() == 1);
///     assert!(index.categories.iter().count() == 1);
///     assert!(index.ids.iter().count() == 1);
/// } else {
///     panic!("Invalid index");
/// }
/// ```
#[derive(Debug)]
pub struct Index {
    /// A `Vec` of `Area`s.
    pub areas: Vec<Area>,

    /// A `Vec` of type `Category`
    pub categories: Vec<Category>,

    /// A `Vec` of `Id`s
    pub ids: Vec<Id>,
}

impl Area {
    /// Creates a new `Area` from a given `&str`, returning a `Result`.
    ///
    /// If `Ok`, returns the `Area`.
    /// If `Err`, returns a `&str` of the error message.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Area;
    ///
    /// if let Ok(area) = Area::new("10-19 Example") {
    ///     assert!(area.get_area() == "10-19");
    ///     assert!(area.get_name() == "Example");
    /// } else {
    ///     panic!("Invalid area");
    /// }
    /// ```
    pub fn new(str: &str) -> Result<Self, &'static str> {
        let chars: Vec<char> = str.chars().collect();

        if chars.len() < 5 {
            return Err("Given area is too short to follow a0-a9");
        }

        if !chars[0].is_ascii_digit() {
            return Err("Given area does not have valid starting digit a in a0");
        }

        if chars[1] != '0' {
            return Err("Given area does not have a starting digit that ends in 0 in a0-a9");
        }

        if chars[2] != '-' {
            return Err("Given area does not have dash separator in a0-a9");
        }

        if chars[3] != chars[0] {
            return Err("Given area does not have an ending digit equal to a in a0-a9");
        }

        if chars[4] != '9' {
            return Err("Given area does not end with 9 in a0-a9");
        }

        if chars.len() < 7 {
            return Err("Given area is too short to have a name");
        }

        if chars[5] != ' ' {
            return Err("Given area does not have a space separator");
        }

        Ok(Area {
            area: chars[0..5].iter().collect(),
            name: chars[6..chars.len()].iter().collect(),
        })
    }

    /// Area `10-19`: The string `a0-a9` derived from `a0-a9 <title>`.
    pub fn get_area(&self) -> &str {
        &self.area
    }

    /// Title `My Title`: The string `<title>` derived from `a0-a9 <title>`
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Change the `Title` of an existing `a0-a9 <title>`.
    pub fn set_name(mut self, name: &str) -> Result<Self, &'static str> {
        // TODO: Check if name contains comment

        self.name = name.to_string();

        Ok(self)
    }
}

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area
    }
}

impl Ord for Area {
    fn cmp(&self, other: &Self) -> Ordering {
        self.area.cmp(&other.area)
    }
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Category {
    /// Creates a new `Category` from a given `&str`, returning a `Result`.
    ///
    /// If `Ok`, returns the `Category`.
    /// If `Err`, returns a `&str` of the error message.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Category;
    ///
    /// if let Ok(category) = Category::new("25 Example") {
    ///     assert!(category.get_area() == "20-29");
    ///     assert!(category.get_category() == "25");
    ///     assert!(category.get_name() == "Example");
    /// } else {
    ///     panic!("Invalid category");
    /// }
    /// ```
    pub fn new(str: &str) -> Result<Self, &'static str> {
        let chars: Vec<char> = str.chars().collect();

        if chars.len() < 2 {
            return Err("Given category is too short to follow [00-99]");
        }

        if !chars[0].is_ascii_digit() || !chars[1].is_ascii_digit() {
            return Err("Given category does not have valid starting digits [00-99]");
        }

        if chars.len() < 4 {
            return Err("Given category is too short to have a name");
        }

        if chars[2] != ' ' {
            return Err("Given category does not have a space separator");
        }

        Ok(Category {
            category: chars[0..2].iter().collect(),
            area: [chars[0], '0', '-', chars[0], '9'].iter().collect(),
            name: chars[3..chars.len()].iter().collect(),
        })
    }

    /// Area `10-19`: The string `a0-a9` derived from `ac <title>`.
    pub fn get_area(&self) -> &str {
        &self.area
    }

    /// Category `11`: The string `ac` derived from `ac <title>`.
    pub fn get_category(&self) -> &str {
        &self.category
    }

    /// Title `My Title`: The string `<title>` derived from `ac <title>`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Change the `Title` of an existing `ac <title>`.
    pub fn set_name(mut self, name: &str) -> Result<Self, &'static str> {
        // TODO: Check if name contains comment

        self.name = name.to_string();

        Ok(self)
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Self) -> Ordering {
        self.category.cmp(&other.category)
    }
}

impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Id {
    /// Creates a new `Id` from a given `&str`, returning a `Result`.
    ///
    /// If `Ok`, returns the `Id`.
    /// If `Err`, returns a `&str` of the error message.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Id;
    ///
    /// if let Ok(id) = Id::new("43.21 Example") {
    ///     assert!(id.get_area() == "40-49");
    ///     assert!(id.get_category() == "43");
    ///     assert!(id.get_id() == "43.21");
    ///     assert!(id.get_name() == "Example");
    /// } else {
    ///     panic!("Invalid id");
    /// }
    /// ```
    pub fn new(str: &str) -> Result<Self, &'static str> {
        let chars: Vec<char> = str.chars().collect();

        if chars.len() < 5 {
            return Err("Given id is too short to follow ac.id");
        }

        if !chars[0].is_ascii_digit() {
            return Err("Given id does not have a valid starting area digit a in ac.id");
        }

        if !chars[1].is_ascii_digit() {
            return Err("Given id does not have a valid category ac in ac.id");
        }

        if chars[2] != '.' {
            return Err("Given id does not have a decimal separator in ac.id");
        }

        if !chars[3].is_ascii_digit() || !chars[4].is_ascii_digit() {
            return Err("Given id does not have valid digits id to follow ac.id");
        }

        if chars.len() < 7 {
            return Err("Given id is too short to have a name");
        }

        if chars[5] != ' ' {
            return Err("Given id does not have a space separator");
        }

        Ok(Id {
            id: chars[0..5].iter().collect(),
            category: chars[0..2].iter().collect(),
            area: [chars[0], '0', '-', chars[0], '9'].iter().collect(),
            name: chars[6..chars.len()].iter().collect(),
        })
    }

    /// Area `10-19`: The string `a0-a9` derived from `ac.id <title>`.
    pub fn get_area(&self) -> &str {
        &self.area
    }

    /// Category `11`: The string `ac` derived from `ac.id <title>`.
    pub fn get_category(&self) -> &str {
        &self.category
    }

    /// Id `11.01`: The string `ac.id` derived from `ac.id <title>`.
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Title `My Title`: The string `<title>` derived from `ac.id <title>`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Change the `Title` of an existing `ac.id <title>`.
    pub fn set_name(mut self, name: &str) -> Result<Self, &'static str> {
        // TODO: Check if name contains comment

        self.name = name.to_string();

        Ok(self)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Index {
    /// Creates a new `Index` from a given `&str`, returning a `Result`.
    ///
    /// If `Ok`, returns the `Index`.
    /// If `Err`, returns a `&str` of the error message.
    ///
    /// # Example
    ///
    /// ```
    /// use johnnydecimal::Index;
    ///
    /// if let Ok(index) = Index::new("10-19 Area\n11 Category\n11.01 Id") {
    ///     assert!(index.areas.iter().count() == 1);
    ///     assert!(index.categories.iter().count() == 1);
    ///     assert!(index.ids.iter().count() == 1);
    /// } else {
    ///     panic!("Invalid index");
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Key/value pairs and comments have not been implemented.
    pub fn new(str: &str) -> Result<Index, &str> {
        let mut areas: Vec<Area> = vec![];
        let mut categories: Vec<Category> = vec![];
        let mut ids: Vec<Id> = vec![];

        for line in str.lines() {
            let line = line.trim_start_matches(' ');

            if line.is_empty() {
                continue;
            }

            if line.starts_with('-') {
                todo!("Add support for key/value pairs");
            }

            if line.contains("//") {
                todo!("Add support for comments");
            }

            if line.contains("/*") {
                todo!("Add support for multi-line comments");
            }

            if let Ok(id) = Id::new(line) {
                if ids.contains(&id) {
                    return Err("Duplicate ids are not allowed");
                }

                if !categories.iter().any(|c| c.category == id.category) {
                    return Err("A given id has no associated category");
                }

                ids.push(id);
                continue;
            }

            if let Ok(category) = Category::new(line) {
                if categories.contains(&category) {
                    return Err("Duplicate categories are not allowed");
                }

                if !areas.iter().any(|a| a.area == category.area) {
                    return Err("A given category has no associated area");
                }

                categories.push(category);
                continue;
            }

            if let Ok(area) = Area::new(line) {
                if areas.contains(&area) {
                    return Err("Duplicate areas are not allowed");
                }

                areas.push(area);
                continue;
            }

            return Err("Given value was neither an Area, Category, or Id");
        }

        areas.sort();
        categories.sort();
        ids.sort();

        Ok(Index { areas, categories, ids })
    }
}
