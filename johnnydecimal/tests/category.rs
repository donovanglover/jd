use johnnydecimal::Category;

#[test]
fn test_category() {
    assert!(Category::new("").is_err(), "should fail if empty");
    assert!(Category::new("5").is_err(), "should fail if too short");
    assert!(Category::new("x3 Test").is_err(), "should fail if first character is not a digit");
    assert!(Category::new("4x Test").is_err(), "should fail if second character is not a digit");
    assert!(Category::new("24.Test Category").is_err(), "should fail if no space between category and name");
    assert!(Category::new("7 Test").is_err(), "should fail if 1 digit");
    assert!(Category::new("381 Test Category").is_err(), "should fail if 3 digits");
    assert!(Category::new("32").is_err(), "should fail if no name");
    assert!(Category::new("18 ").is_err(), "should fail if only space at end");
    assert!(Category::new("20-29 Invalid Category").is_err(), "should fail if an area was given");
    assert!(Category::new("31.09 Invalid Category").is_err(), "should fail if ac.id was given");
    assert!(Category::new("Just a regular folder").is_err(), "should fail if no identifier");
    assert!(Category::new("14 Test Category").is_ok(), "should pass if [00-99] name");
}

#[test]
fn test_category_name() {
    assert!(Category::new("15 Test Test").is_ok(), "Name `Test Test` should pass");
    assert!(Category::new("15 テスト").is_ok(), "Name `テスト` should pass");
    assert!(Category::new("15 ../Directory traversal").is_err(), "Name `../Directory traversal` should fail");
    assert!(Category::new("15 /* Comment */").is_err(), "Name `/* Comment */` should fail");
    assert!(Category::new("15 Testing // Comment").is_err(), "Name `Testing // Comment` should fail");
}

#[test]
fn get_category() {
    let category = Category::new("32 Sales Test").expect("`32 Sales Test` should be a valid category");

    assert_eq!(category.get_category(), "32", "`32 Sales Test` should have category equal to `32`");
    assert_eq!(category.get_area(), "30-39", "`32 Sales Test` should have area equal to `30-39`");
    assert_eq!(category.get_name(), "Sales Test", "`32 Sales Test` should have name equal to `Sales Test`");
}

#[test]
fn compare_category() {
    let category_1 = Category::new("11 First").expect("`11 First` should be a valid category");
    let category_2 = Category::new("11 Second").expect("`11 Second` should be a valid category");
    let category_3 = Category::new("12 First").expect("12 First` should be a valid category");

    assert_eq!(category_1, category_2, "`11 First` should equal the same category as `11 Second`");
    assert_ne!(category_1, category_3, "`11 First` should NOT equal the same category as `12 First`");
}

#[test]
fn sort_category() {
    let category_1 = Category::new("11 Example 1").expect("`11 Example 1` should be a valid category");
    let category_2 = Category::new("12 Example 2").expect("`12 Example 2` should be a valid category");
    let category_3 = Category::new("13 Example 3").expect("`13 Example 3` should be a valid category");
    let category_4 = Category::new("12 Example 4").expect("`12 Example 4` should be a valid category");

    assert!(category_2 > category_1, "`12 Example 2` should be greater than `11 Example 1`");
    assert!(category_4 < category_3, "`12 Example 4` should be less than `13 Example 3`");
}

#[test]
fn set_category_name() {
    let mut category = Category::new("14 Test Category").expect("`14 Test Category` should be a valid category");

    assert!(category.set_name("Test 2").is_ok(), "Name `Test 2` should pass");
    assert!(category.set_name("テスト").is_ok(), "Name `テスト` should pass");
    assert!(category.set_name("../Directory traversal").is_err(), "Name `../Directory traversal` should fail");
    assert!(category.set_name("/* Comment */").is_err(), "Name `/* Comment */` should fail");
    assert!(category.set_name("Testing // Comment").is_err(), "Name `Testing // Comment` should fail");
}
