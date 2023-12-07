use std::fs;
use regex::Regex;

/// Matches: 10-19 Area
pub fn is_area(maybe_area: &str) -> bool {
    return Regex::new(r"^\d\d-\d\d\s").unwrap().is_match(maybe_area)
}

/// Matches: 11 Category
pub fn is_category(maybe_category: &str) -> bool {
    return Regex::new(r"^\d\d\s").unwrap().is_match(maybe_category)
}

/// Matches: 11.01 Id
pub fn is_id(maybe_id: &str) -> bool {
    return Regex::new(r"^\d\d.\d\d\s").unwrap().is_match(maybe_id)
}

/// List areas
pub fn list_areas() {
    let mut areas: Vec<String> = Vec::new();

    for entry in fs::read_dir("/home/user").unwrap() {
        let path = entry.as_ref().unwrap().path();

        if path.is_dir() {
            let path = path.file_name().unwrap().to_str().unwrap().to_string();

            if is_area(&path) {
                areas.push(path);
            }
        }
    }

    areas.sort();

    for area in areas {
        println!("{area}");
    }
}
