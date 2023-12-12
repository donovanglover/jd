use jd::Id;

#[test]
fn test_id() {
    assert!(Id::from_str("").is_err(), "should fail if empty");
    assert!(Id::from_str("1.9").is_err(), "should fail if too short");
    assert!(Id::from_str("a0.39").is_err(), "should fail if a in ac.id is not a digit");
    assert!(Id::from_str("3c.39").is_err(), "should fail if c in ac.id is not a digit");
    assert!(Id::from_str("15.x8").is_err(), "should fail if i in ac.id is not a digit");
    assert!(Id::from_str("12.0x").is_err(), "should fail if d in ac.id is not a digit");
    assert!(Id::from_str("20_29 Test Id").is_err(), "should fail if no decimal separator");
    assert!(Id::from_str("1.09 Test Id").is_err(), "should fail if 1 digit ac in ac.id");
    assert!(Id::from_str("12.9 Test Id").is_err(), "should fail if 1 digit id in ac.id");
    assert!(Id::from_str("10.109 Test Id").is_err(), "should fail if 3 digit id in ac.id");
    assert!(Id::from_str("201.22 Test Id").is_err(), "should fail if 3 digit ac in ac.id");
    assert!(Id::from_str("24.21.Test Id").is_err(), "should fail if no space between ac.id and name");
    assert!(Id::from_str("33.19").is_err(), "should fail if no name");
    assert!(Id::from_str("52.41 ").is_err(), "should fail if only space at end");
    assert!(Id::from_str("20-29 Invalid Id").is_err(), "should fail if an area was given");
    assert!(Id::from_str("25 Invalid Id").is_err(), "should fail if a category was given");
    assert!(Id::from_str("Just a regular folder").is_err(), "should fail if no identifier");
    assert!(Id::from_str("24.05 Test Id").is_ok(), "should pass if ac.id name");
}

#[test]
fn get_id() {
    if let Ok(id) = Id::from_str("48.32 Important Trip") {
        assert_eq!(id.id, "48.32", "`48.32 Important Trip` should have id equal to `48.32`");
        assert_eq!(id.category, "48", "`48.32 Important Trip` should have category equal to `48`");
        assert_eq!(id.area, "40-49", "`48.32 Important Trip` should have area equal to `40-49`");
        assert_eq!(id.name, "Important Trip", "`48.32 Important Trip` should have name equal to `Important Trip`");
    } else {
        panic!("Valid id `48.32 Important Trip` was not returned as valid.");
    }
}

#[test]
fn compare_id() {
    let id_1 = Id::from_str("11.01 First").expect("`11.01 First` should be a valid id");
    let id_2 = Id::from_str("11.01 Second").expect("`11.01 Second` should be a valid id");
    let id_3 = Id::from_str("11.02 First").expect("11.02 First` should be a valid id");

    assert!(id_1 == id_2, "`11.01 First` should equal the same id as `11.01 Second`");
    assert!(id_1 != id_3, "`11.01 First` should NOT equal the same id as `11.02 First`");
}
