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

#[test]
fn sort_index() {
    if let Ok(index_str_reverse) = Index::from_str("20-29 1 Test\n10-19 2 Another") {
        assert!(index_str_reverse.areas[0].area == "10-19", "should sort 10-19 before 20-29");
        assert!(index_str_reverse.areas[1].area == "20-29", "should sort 20-29 after 10-19");
        assert!(index_str_reverse.areas[0].name == "2 Another", "should not sort by name");
        assert!(index_str_reverse.areas[1].name == "1 Test", "should not sort by name");
    } else {
        panic!("index_str_reverse should pass");
    }

    let lines = [
        "30-39 Area A",
        "34 Category A",
        "37 Category B",
        "34.05 Id A",
        "37.01 Id B",
        "40-49 Area C",
        "20-29 Area B",
        "25 Category C",
        "10-19 Area D",
        "25.09 Id C",
        "22 Category D",
        "25.05 Id D",
    ];

    if let Ok(index_str_mix) = Index::from_str(&lines.join("\n")) {
        assert!(index_str_mix.areas[0].area == "10-19", "should have 10-19 first");
        assert!(index_str_mix.areas[1].name == "Area B", "should have Area B second");
        assert!(index_str_mix.areas[2].area == "30-39", "should have 30-39 third");
        assert!(index_str_mix.areas[3].name == "Area C", "should have Area C fourth");
        assert!(index_str_mix.categories[0].area == "20-29", "should have a category with area 20-29 first");
        assert!(index_str_mix.categories[1].category == "25", "should have category 25 second");
        assert!(index_str_mix.categories[2].name == "Category A", "should have Category A third");
        assert!(index_str_mix.categories[3].category == "37", "should have category 37 fourth");
        assert!(index_str_mix.ids[0].area == "20-29", "should have an id with area 20-29 first");
        assert!(index_str_mix.ids[1].category == "25", "should have an id with category 25 second");
        assert!(index_str_mix.ids[2].id == "34.05", "should have id 34.05 third");
        assert!(index_str_mix.ids[3].name == "Id B", "should have Id B fourth");
    } else {
        panic!("index_str_mix should pass");
    }
}
