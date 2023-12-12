use jd::Area;

#[test]
fn test_area() {
    assert!(Area::from_str("").is_err(), "should fail if empty");
    assert!(Area::from_str("1-9").is_err(), "should fail if too short");
    assert!(Area::from_str("c0-c9").is_err(), "should fail if a is not a digit");
    assert!(Area::from_str("24-21.Test Area").is_err(), "should fail if no space between area and name");
    assert!(Area::from_str("12-19 Test Area").is_err(), "should fail if doesn't start at 10");
    assert!(Area::from_str("10-15 Test Area").is_err(), "should fail if doesn't end at 19");
    assert!(Area::from_str("20-39 Test Area").is_err(), "should fail if range isn't a0-a9");
    assert!(Area::from_str("20>29 Test Area").is_err(), "should fail if no dash separator");
    assert!(Area::from_str("1-9 Test Area").is_err(), "should fail if 1 digit");
    assert!(Area::from_str("100-109 Test Area").is_err(), "should fail if 3 digits");
    assert!(Area::from_str("10-19").is_err(), "should fail if no name");
    assert!(Area::from_str("10-19 ").is_err(), "should fail if only space at end");
    assert!(Area::from_str("33 Invalid Area").is_err(), "should fail if a category was given");
    assert!(Area::from_str("12.49 Invalid Area").is_err(), "should fail if ac.id was given");
    assert!(Area::from_str("Just a regular folder").is_err(), "should fail if no identifier");
    assert!(Area::from_str("10-19 Test Area").is_ok(), "should pass if a0-a9 name");
}

#[test]
fn get_area() {
    if let Ok(area) = Area::from_str("20-29 Finance Test") {
        assert_eq!(area.area, "20-29", "`20-29 Finance Test` should have area equal to `20-29`");
        assert_eq!(area.name, "Finance Test", "`20-29 Finance Test` should have name equal to `Finance Test`");
    } else {
        panic!("Valid area `20-29 Finance Test` was not returned as valid.")
    }
}

#[test]
fn compare_area() {
    let area_1 = Area::from_str("10-19 First").expect("`10-19 First` should be a valid area");
    let area_2 = Area::from_str("10-19 Second").expect("`10-19 Second` should be a valid area");
    let area_3 = Area::from_str("20-29 First").expect("20-29 First` should be a valid area");

    assert!(area_1 == area_2, "`10-19 First` should equal the same area as `10-19 Second`");
    assert!(area_1 != area_3, "`10-19 First` should NOT equal the same area as `20-29 First`");
}
