use jd::Index;

#[test]
fn test_index() {
    if let Ok(empty) = Index::from_str("") {
        assert!(empty.areas.is_empty(), "empty index should have no areas");
        assert!(empty.categories.is_empty(), "empty index should have no categories");
        assert!(empty.ids.is_empty(), "empty index should have no ids");
    } else {
        panic!("empty index should pass")
    }

    if let Ok(empty_lines) = Index::from_str("\n\n\n") {
        assert!(empty_lines.areas.is_empty(), "empty_lines index should have no areas");
        assert!(empty_lines.categories.is_empty(), "empty_lines index should have no categories");
        assert!(empty_lines.ids.is_empty(), "empty_lines index should have no ids");
    } else {
        panic!("empty_lines should pass")
    }

    if let Ok(area_only) = Index::from_str("20-29 Test\n30-39 Another") {
        assert_eq!(area_only.areas.iter().count(), 2, "area_only index should have 2 areas");
        assert!(area_only.categories.is_empty(), "area_only index should have no categories");
        assert!(area_only.ids.is_empty(), "area_only index should have no ids");
    } else {
        panic!("area_only index should pass");
    }

    if let Ok(area_with_category) = Index::from_str("20-29 Area\n22 Category\n25 Another") {
        assert_eq!(area_with_category.areas.iter().count(), 1, "area_with_category index should have 1 area");
        assert_eq!(area_with_category.categories.iter().count(), 2, "area_with_category index should have 2 categories");
        assert!(area_with_category.ids.is_empty(), "area_with_category index should have no ids");
    } else {
        panic!("area_with_category index should pass");
    }

    if let Ok(area_with_category_and_id) = Index::from_str("20-29 Area\n22 Category\n22.03 Id\n22.05 Another") {
        assert_eq!(area_with_category_and_id.areas.iter().count(), 1, "area_with_category_and_id index should have 1 area");
        assert_eq!(area_with_category_and_id.categories.iter().count(), 1, "area_with_category_and_id index should have 1 category");
        assert_eq!(area_with_category_and_id.ids.iter().count(), 2, "area_with_category_and_id index should have 2 ids");
    } else {
        panic!("area_with_category_and_id index should pass");
    }

    if let Ok(valid_with_empty_lines) = Index::from_str("\n10-19 Testing\n\n20-29 Another\n\n\n23 Category\n\n23.05 Id\n\n\n") {
        assert_eq!(valid_with_empty_lines.areas.iter().count(), 2, "valid_with_empty_lines index should have 2 areas");
        assert_eq!(valid_with_empty_lines.categories.iter().count(), 1, "valid_with_empty_lines index should have 1 category");
        assert_eq!(valid_with_empty_lines.ids.iter().count(), 1, "valid_with_empty_lines index should have 1 id");
    } else {
        panic!("valid_with_empty_lines index should pass");
    }

    assert!(Index::from_str("20 Test\n21 Another").is_err(), "should fail if category_only index");
    assert!(Index::from_str("21.02 Test\n21.03 Another").is_err(), "should fail if id_only index");
    assert!(Index::from_str("20-29 Area\n21.55 Id").is_err(), "should fail if area and id only");
    assert!(Index::from_str("12 Category\n10-19 Area").is_err(), "should fail if area before category");
    assert!(Index::from_str("10-19 Area\n11.01 Test\n11 Category").is_err(), "should fail if id before category");
    assert!(Index::from_str("20-29 One\n20-29 Two").is_err(), "should fail if duplicate area");
    assert!(Index::from_str("20-29 Area\n21 Category A\n21 Another Category").is_err(), "should fail if duplicate category");
    assert!(Index::from_str("20-29 Area\n21 Category A\n21.01 Id\n21.01 Another Id").is_err(), "should fail if duplicate id");
    assert!(Index::from_str("20-29 Area\nTest").is_err(), "should fail if input contains a non-valid type");
}
