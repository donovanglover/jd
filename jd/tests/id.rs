use jd::Id;

#[test]
fn test_id() {
    assert!(Id::from_str("12.03 Johnny Decimal ID").is_ok());
    assert!(Id::from_str("Just a regular folder").is_err());
    assert!(Id::from_str("1.03 Invalid AC.ID format").is_err());
}

#[test]
fn get_id() {
    if let Ok(area) = Id::from_str("48.32 Important Trip") {
        assert_eq!(area.id, "48.32");
        assert_eq!(area.category, "48");
        assert_eq!(area.area, "40-49");
        assert_eq!(area.name, "Important Trip");
    } else {
        panic!("Valid category 32 Sales was not returned as valid.")
    }
}
