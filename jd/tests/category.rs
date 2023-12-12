use jd::Category;

#[test]
fn test_category() {
    assert!(Category::from_str("").is_err(), "should fail if empty");
    assert!(Category::from_str("5").is_err(), "should fail if too short");
    assert!(Category::from_str("x3 Test").is_err(), "should fail if first character is not a digit");
    assert!(Category::from_str("4x Test").is_err(), "should fail if second character is not a digit");
    assert!(Category::from_str("24.Test Category").is_err(), "should fail if no space between category and name");
    assert!(Category::from_str("7 Test").is_err(), "should fail if 1 digit");
    assert!(Category::from_str("381 Test Category").is_err(), "should fail if 3 digits");
    assert!(Category::from_str("32").is_err(), "should fail if no name");
    assert!(Category::from_str("18 ").is_err(), "should fail if only space at end");
    assert!(Category::from_str("20-29 Invalid Category").is_err(), "should fail if an area was given");
    assert!(Category::from_str("31.09 Invalid Category").is_err(), "should fail if ac.id was given");
    assert!(Category::from_str("Just a regular folder").is_err(), "should fail if no identifier");
    assert!(Category::from_str("14 Test Category").is_ok(), "should pass if [00-99] name");
}

#[test]
fn get_category() {
    if let Ok(category) = Category::from_str("32 Sales Test") {
        assert_eq!(category.category, "32", "`32 Sales Test` should have category equal to `32`");
        assert_eq!(category.area, "30-39", "`32 Sales Test` should have area equal to `30-39`");
        assert_eq!(category.name, "Sales Test", "`32 Sales Test` should have name equal to `Sales Test`");
    } else {
        panic!("Valid category `32 Sales Test` was not returned as valid.")
    }
}

#[test]
fn compare_category() {
    let category_1 = Category::from_str("11 First").expect("`11 First` should be a valid category");
    let category_2 = Category::from_str("11 Second").expect("`11 Second` should be a valid category");
    let category_3 = Category::from_str("12 First").expect("12 First` should be a valid category");

    assert!(category_1 == category_2, "`11 First` should equal the same category as `11 Second`");
    assert!(category_1 != category_3, "`11 First` should NOT equal the same category as `12 First`");
}
