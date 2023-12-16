use johnnydecimal::Area;

#[test]
fn test_area() {
    assert!(Area::new("").is_err(), "should fail if empty");
    assert!(Area::new("1-9").is_err(), "should fail if too short");
    assert!(Area::new("c0-c9").is_err(), "should fail if a is not a digit");
    assert!(Area::new("24-21.Test Area").is_err(), "should fail if no space between area and name");
    assert!(Area::new("12-19 Test Area").is_err(), "should fail if doesn't start at 10");
    assert!(Area::new("10-15 Test Area").is_err(), "should fail if doesn't end at 19");
    assert!(Area::new("20-39 Test Area").is_err(), "should fail if range isn't a0-a9");
    assert!(Area::new("20>29 Test Area").is_err(), "should fail if no dash separator");
    assert!(Area::new("1-9 Test Area").is_err(), "should fail if 1 digit");
    assert!(Area::new("100-109 Test Area").is_err(), "should fail if 3 digits");
    assert!(Area::new("10-19").is_err(), "should fail if no name");
    assert!(Area::new("10-19 ").is_err(), "should fail if only space at end");
    assert!(Area::new("33 Invalid Area").is_err(), "should fail if a category was given");
    assert!(Area::new("12.49 Invalid Area").is_err(), "should fail if ac.id was given");
    assert!(Area::new("Just a regular folder").is_err(), "should fail if no identifier");
    assert!(Area::new("10-19 Test Area").is_ok(), "should pass if a0-a9 name");
}

#[test]
fn get_area() {
    let area = Area::new("20-29 Finance Test").expect("`20-29 Finance Test` should be a valid area");

    assert_eq!(area.get_area(), "20-29", "`20-29 Finance Test` should have area equal to `20-29`");
    assert_eq!(area.get_name(), "Finance Test", "`20-29 Finance Test` should have name equal to `Finance Test`");
}

#[test]
fn compare_area() {
    let area_1 = Area::new("10-19 First").expect("`10-19 First` should be a valid area");
    let area_2 = Area::new("10-19 Second").expect("`10-19 Second` should be a valid area");
    let area_3 = Area::new("20-29 First").expect("20-29 First` should be a valid area");

    assert!(area_1 == area_2, "`10-19 First` should equal the same area as `10-19 Second`");
    assert!(area_1 != area_3, "`10-19 First` should NOT equal the same area as `20-29 First`");
}

#[test]
fn sort_area() {
    let area_1 = Area::new("10-19 Example 1").expect("`10-19 Example 1` should be a valid area");
    let area_2 = Area::new("20-29 Example 2").expect("`20-29 Example 2` should be a valid area");
    let area_3 = Area::new("30-39 Example 3").expect("`30-39 Example 3` should be a valid area");
    let area_4 = Area::new("20-29 Example 4").expect("`20-29 Example 4` should be a valid area");

    assert!(area_2 > area_1, "`20-29 Example 2` should be greater than `10-19 Example 1`");
    assert!(area_4 < area_3, "`20-29 Example 4` should be less than `30-39 Example 3`");
}
