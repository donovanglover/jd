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
