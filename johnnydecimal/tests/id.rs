use johnnydecimal::Id;

#[test]
fn test_id() {
    assert!(Id::new("").is_err(), "should fail if empty");
    assert!(Id::new("1.9").is_err(), "should fail if too short");
    assert!(Id::new("a0.39").is_err(), "should fail if a in ac.id is not a digit");
    assert!(Id::new("3c.39").is_err(), "should fail if c in ac.id is not a digit");
    assert!(Id::new("15.x8").is_err(), "should fail if i in ac.id is not a digit");
    assert!(Id::new("12.0x").is_err(), "should fail if d in ac.id is not a digit");
    assert!(Id::new("20_29 Test Id").is_err(), "should fail if no decimal separator");
    assert!(Id::new("1.09 Test Id").is_err(), "should fail if 1 digit ac in ac.id");
    assert!(Id::new("12.9 Test Id").is_err(), "should fail if 1 digit id in ac.id");
    assert!(Id::new("10.109 Test Id").is_err(), "should fail if 3 digit id in ac.id");
    assert!(Id::new("201.22 Test Id").is_err(), "should fail if 3 digit ac in ac.id");
    assert!(Id::new("24.21.Test Id").is_err(), "should fail if no space between ac.id and name");
    assert!(Id::new("33.19").is_err(), "should fail if no name");
    assert!(Id::new("52.41 ").is_err(), "should fail if only space at end");
    assert!(Id::new("20-29 Invalid Id").is_err(), "should fail if an area was given");
    assert!(Id::new("25 Invalid Id").is_err(), "should fail if a category was given");
    assert!(Id::new("Just a regular folder").is_err(), "should fail if no identifier");
    assert!(Id::new("24.05 Test Id").is_ok(), "should pass if ac.id name");
}

#[test]
fn get_id() {
    let id = Id::new("48.32 Important Trip").expect("`48.32 Important Trip` should be a valid id");

    assert_eq!(id.id, "48.32", "`48.32 Important Trip` should have id equal to `48.32`");
    assert_eq!(id.category, "48", "`48.32 Important Trip` should have category equal to `48`");
    assert_eq!(id.area, "40-49", "`48.32 Important Trip` should have area equal to `40-49`");
    assert_eq!(id.name, "Important Trip", "`48.32 Important Trip` should have name equal to `Important Trip`");
}

#[test]
fn compare_id() {
    let id_1 = Id::new("11.01 First").expect("`11.01 First` should be a valid id");
    let id_2 = Id::new("11.01 Second").expect("`11.01 Second` should be a valid id");
    let id_3 = Id::new("11.02 First").expect("11.02 First` should be a valid id");

    assert!(id_1 == id_2, "`11.01 First` should equal the same id as `11.01 Second`");
    assert!(id_1 != id_3, "`11.01 First` should NOT equal the same id as `11.02 First`");
}

#[test]
fn sort_id() {
    let id_1 = Id::new("11.01 Example 1").expect("`11.01 Example 1` should be a valid id");
    let id_2 = Id::new("11.02 Example 2").expect("`11.02 Example 2` should be a valid id");
    let id_3 = Id::new("11.03 Example 3").expect("`11.03 Example 3` should be a valid id");
    let id_4 = Id::new("11.02 Example 4").expect("`11.02 Example 4` should be a valid id");
    let id_5 = Id::new("01.02 Example 5").expect("`01.02 Example 5` should be a valid id");
    let id_6 = Id::new("14.02 Example 6").expect("`14.02 Example 6` should be a valid id");
    let id_7 = Id::new("21.02 Example 7").expect("`21.02 Example 7` should be a valid id");

    assert!(id_2 > id_1, "`11.02 Example 2` should be greater than `11.01 Example 1`");
    assert!(id_4 < id_3, "`11.02 Example 4` should be less than `11.03 Example 3`");
    assert!(id_5 < id_1, "`01.02 Example 5` should be less than `11.01 Example 1`");
    assert!(id_5 < id_3, "`01.02 Example 5` should be less than `11.03 Example 3`");
    assert!(id_6 > id_1, "`14.02 Example 6` should be greater than `11.01 Example 1`");
    assert!(id_6 > id_3, "`14.02 Example 6` should be greater than `11.03 Example 3`");
    assert!(id_7 > id_1, "`21.02 Example 7` should be greater than `11.01 Example 1`");
    assert!(id_7 > id_3, "`21.02 Example 7` should be greater than `11.03 Example 3`");
}
