use jd::Category;

#[test]
fn test_category() {
    assert!(Category::from_str("11 Category").is_ok());
    assert!(Category::from_str("120 Category").is_err());
    assert!(Category::from_str("20-29 Category").is_err());
    assert!(Category::from_str("9 Category").is_err());
    assert!(Category::from_str("54").is_err());
    assert!(Category::from_str("82 ").is_err());
}

#[test]
fn get_category() {
    if let Ok(area) = Category::from_str("32 Sales") {
        assert_eq!(area.category, "32");
        assert_eq!(area.area, "30-39");
        assert_eq!(area.name, "Sales");
    } else {
        panic!("Valid category 32 Sales was not returned as valid.")
    }
}
