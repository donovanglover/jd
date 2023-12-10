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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_area() {
        assert_eq!(is_area("10-19 Area"), true);
        assert_eq!(is_area("10-30 Area"), false);
    }

    #[test]
    fn test_is_category() {
        assert_eq!(is_category("11 Category"), true);
        assert_eq!(is_category("120 Category"), false);
    }

    #[test]
    fn test_is_id() {
        assert_eq!(is_id("12.04 Name"), true);
    }
}
