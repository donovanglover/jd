use jd::Area;

#[test]
fn test_area() {
    assert!(Area::from_str("12-19 Test Area").is_err());      // Doesn't start at 10
    assert!(Area::from_str("10-15 Test Area").is_err());      // Doesn't end at 19
    assert!(Area::from_str("1-9 Test Area").is_err());        // Not 2 digits
    assert!(Area::from_str("100-109 Test Area").is_err());    // Not 2 digits
    assert!(Area::from_str("10-19").is_err());                // No name
    assert!(Area::from_str("10-19 ").is_err());               // Only space
    assert!(Area::from_str("10-19 Test Area").is_ok());
}

#[test]
fn get_area() {
    if let Ok(area) = Area::from_str("20-29 Finance") {
        assert_eq!(area.area, "20-29");
        assert_eq!(area.name, "Finance");
    } else {
        panic!("Valid area 20-29 Finance was not returned as valid.")
    }
}
