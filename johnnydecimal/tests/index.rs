use johnnydecimal::Index;

#[test]
fn test_index() {
    if let Ok(empty) = Index::new("") {
        assert!(empty.get_areas().is_empty(), "empty index should have no areas");
        assert!(empty.get_categories().is_empty(), "empty index should have no categories");
        assert!(empty.get_ids().is_empty(), "empty index should have no ids");
    } else {
        panic!("empty index should pass")
    }

    if let Ok(empty_lines) = Index::new("\n\n\n") {
        assert!(empty_lines.get_areas().is_empty(), "empty_lines index should have no areas");
        assert!(empty_lines.get_categories().is_empty(), "empty_lines index should have no categories");
        assert!(empty_lines.get_ids().is_empty(), "empty_lines index should have no ids");
    } else {
        panic!("empty_lines should pass")
    }

    if let Ok(area_only) = Index::new("20-29 Test\n30-39 Another") {
        assert_eq!(area_only.get_areas().iter().count(), 2, "area_only index should have 2 areas");
        assert!(area_only.get_categories().is_empty(), "area_only index should have no categories");
        assert!(area_only.get_ids().is_empty(), "area_only index should have no ids");
    } else {
        panic!("area_only index should pass");
    }

    if let Ok(area_with_category) = Index::new("20-29 Area\n22 Category\n25 Another") {
        assert_eq!(area_with_category.get_areas().iter().count(), 1, "area_with_category index should have 1 area");
        assert_eq!(area_with_category.get_categories().iter().count(), 2, "area_with_category index should have 2 categories");
        assert!(area_with_category.get_ids().is_empty(), "area_with_category index should have no ids");
    } else {
        panic!("area_with_category index should pass");
    }

    if let Ok(area_with_category_and_id) = Index::new("20-29 Area\n22 Category\n22.03 Id\n22.05 Another") {
        assert_eq!(area_with_category_and_id.get_areas().iter().count(), 1, "area_with_category_and_id index should have 1 area");
        assert_eq!(area_with_category_and_id.get_categories().iter().count(), 1, "area_with_category_and_id index should have 1 category");
        assert_eq!(area_with_category_and_id.get_ids().iter().count(), 2, "area_with_category_and_id index should have 2 ids");
    } else {
        panic!("area_with_category_and_id index should pass");
    }

    if let Ok(valid_with_empty_lines) = Index::new("\n10-19 Testing\n\n20-29 Another\n\n\n23 Category\n\n23.05 Id\n\n\n") {
        assert_eq!(valid_with_empty_lines.get_areas().iter().count(), 2, "valid_with_empty_lines index should have 2 areas");
        assert_eq!(valid_with_empty_lines.get_categories().iter().count(), 1, "valid_with_empty_lines index should have 1 category");
        assert_eq!(valid_with_empty_lines.get_ids().iter().count(), 1, "valid_with_empty_lines index should have 1 id");
    } else {
        panic!("valid_with_empty_lines index should pass");
    }

    assert!(Index::new("20 Test\n21 Another").is_err(), "should fail if category_only index");
    assert!(Index::new("21.02 Test\n21.03 Another").is_err(), "should fail if id_only index");
    assert!(Index::new("20-29 Area\n21.55 Id").is_err(), "should fail if area and id only");
    assert!(Index::new("12 Category\n10-19 Area").is_err(), "should fail if category before area");
    assert!(Index::new("10-19 Area\n11.01 Test\n11 Category").is_err(), "should fail if id before category");
    assert!(Index::new("20-29 One\n20-29 Two").is_err(), "should fail if duplicate area");
    assert!(Index::new("20-29 Area\n21 Category A\n21 Another Category").is_err(), "should fail if duplicate category");
    assert!(Index::new("20-29 Area\n21 Category A\n21.01 Id\n21.01 Another Id").is_err(), "should fail if duplicate id");
    assert!(Index::new("20-29 Area\nTest").is_err(), "should fail if input contains a non-valid type");
}

#[test]
fn sort_index() {
    if let Ok(index_str_reverse) = Index::new("20-29 1 Test\n10-19 2 Another") {
        assert!(index_str_reverse.get_areas()[0].get_area() == "10-19", "should sort 10-19 before 20-29");
        assert!(index_str_reverse.get_areas()[1].get_area() == "20-29", "should sort 20-29 after 10-19");
        assert!(index_str_reverse.get_areas()[0].get_name() == "2 Another", "should not sort by name");
        assert!(index_str_reverse.get_areas()[1].get_name() == "1 Test", "should not sort by name");
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

    if let Ok(index_str_mix) = Index::new(&lines.join("\n")) {
        assert!(index_str_mix.get_areas()[0].get_area() == "10-19", "should have 10-19 first");
        assert!(index_str_mix.get_areas()[1].get_name() == "Area B", "should have Area B second");
        assert!(index_str_mix.get_areas()[2].get_area() == "30-39", "should have 30-39 third");
        assert!(index_str_mix.get_areas()[3].get_name() == "Area C", "should have Area C fourth");
        assert!(index_str_mix.get_categories()[0].get_area() == "20-29", "should have a category with area 20-29 first");
        assert!(index_str_mix.get_categories()[1].get_category() == "25", "should have category 25 second");
        assert!(index_str_mix.get_categories()[2].get_name() == "Category A", "should have Category A third");
        assert!(index_str_mix.get_categories()[3].get_category() == "37", "should have category 37 fourth");
        assert!(index_str_mix.get_ids()[0].get_area() == "20-29", "should have an id with area 20-29 first");
        assert!(index_str_mix.get_ids()[1].get_category() == "25", "should have an id with category 25 second");
        assert!(index_str_mix.get_ids()[2].get_id() == "34.05", "should have id 34.05 third");
        assert!(index_str_mix.get_ids()[3].get_name() == "Id B", "should have Id B fourth");
    } else {
        panic!("index_str_mix should pass");
    }
}

#[test]
fn test_with_vecs() {
    use johnnydecimal::{Area, Category, Id};

    let areas = vec![];
    let categories = vec![];
    let ids = vec![];

    assert!(Index::with_vecs(&areas, &categories, &ids).is_ok(), "empty vecs should pass");

    let areas = vec![Area::new("10-19 My Area").expect("`10-19 My Area` should be valid")];

    assert!(Index::with_vecs(&areas, &categories, &ids).is_ok(), "area_only vecs should pass");

    let categories = vec![Category::new("14 Category").expect("`14 Category` should be valid")];

    assert!(Index::with_vecs(&areas, &categories, &ids).is_ok(), "area_with_category vecs should pass");

    let ids = vec![Id::new("14.01 Id").expect("`14.01 Id` should be valid")];

    assert!(Index::with_vecs(&areas, &categories, &ids).is_ok(), "area_with_category_and_id vecs should pass");

    let duplicate_areas = vec![
        Area::new("10-19 Area").expect("`10-19 Area` should be valid"),
        Area::new("10-19 Duplicate Area").expect("`10-19 Duplicate Area` should be valid")
    ];

    assert!(Index::with_vecs(&duplicate_areas, &categories, &ids).is_err(), "duplicate areas should fail");

    let duplicate_categories = vec![
        Category::new("14 Category").expect("`14 Category` should be valid"),
        Category::new("14 Another Category").expect("`14 Another Category` should be valid")
    ];

    assert!(Index::with_vecs(&areas, &duplicate_categories, &ids).is_err(), "duplicate categories should fail");

    let duplicate_ids = vec![
        Id::new("14.01 Id").expect("`14.01 Id` should be valid"),
        Id::new("14.01 Another Id").expect("`14.01 Another Id` should be valid")
    ];

    assert!(Index::with_vecs(&areas, &categories, &duplicate_ids).is_err(), "duplicate ids should fail");

    assert!(Index::with_vecs(&vec![], &categories, &vec![]).is_err(), "should fail if category_only index");
    assert!(Index::with_vecs(&vec![], &vec![], &ids).is_err(), "should fail if id_only index");
    assert!(Index::with_vecs(&areas, &vec![], &ids).is_err(), "should fail if area and id only");
}

#[test]
fn test_add_area() {
    use johnnydecimal::Area;

    if let Ok(mut index) = Index::new("20-29 Area\n22 Category\n22.03 Id\n22.05 Another") {
        let result = index.add_area(Area::new("30-39 Test").expect("`30-39 Test` should be a valid area"));

        assert!(result.is_ok(), "should pass when adding unique area");

        let result = index.add_area(Area::new("20-29 Another").expect("`20-29 Another` should be a valid area"));

        assert!(result.is_err(), "should fail when adding duplicate area");
    } else {
        panic!("area_with_category_and_id index should pass");
    }
}

#[test]
fn test_add_category() {
    use johnnydecimal::Category;

    if let Ok(mut index) = Index::new("20-29 Area\n22 Category\n22.03 Id\n22.05 Another") {
        let result = index.add_category(Category::new("24 Test").expect("`24 Test` should be a valid category"));

        assert!(result.is_ok(), "should pass when adding unique category");

        let result = index.add_category(Category::new("24 Another").expect("`24 Another` should be a valid category"));

        assert!(result.is_err(), "should fail when adding duplicate category");

        let result = index.add_category(Category::new("53 Orphan").expect("`53 Orphan` should be a valid category"));

        assert!(result.is_err(), "should fail when adding category without associated area");
    } else {
        panic!("area_with_category_and_id index should pass");
    }
}

#[test]
fn test_add_id() {
    use johnnydecimal::Id;

    if let Ok(mut index) = Index::new("20-29 Area\n22 Category\n22.03 Id\n22.05 Another") {
        let result = index.add_id(Id::new("22.01 My Id").expect("`22.01 My Id` should be a valid id"));

        assert!(result.is_ok(), "should pass when adding unique id");

        let result = index.add_id(Id::new("22.01 Another Id").expect("`22.01 Another Id` should be a valid id"));

        assert!(result.is_err(), "should fail when adding duplicate id");

        let result = index.add_id(Id::new("23.09 Orphan Id").expect("`23.09 Orphan Id` should be a valid id"));

        assert!(result.is_err(), "should fail when adding id without associated category");
    } else {
        panic!("area_with_category_and_id index should pass");
    }
}

#[test]
fn test_remove_area() {
    use johnnydecimal::Area;

    if let Ok(mut index) = Index::new("20-29 Area\n22 Category\n22.03 Id\n22.05 Another\n40-49 Area 2\n43 Category 2") {
        let area_1 = Area::new("10-19 Test").expect("`10-19 Test` should be a valid area");
        let area_2 = Area::new("20-29 Test").expect("`20-29 Test` should be a valid area");

        assert!(index.get_areas().contains(&area_2), "`20-29 Test` should be in areas");

        assert!(index.remove_area(&area_1).is_err(), "`10-19 Test` should NOT be a valid area to remove");
        assert!(index.remove_area(&area_2).is_ok(), "`20-29 Test` should be a valid area to remove");

        assert!(!index.get_areas().contains(&area_2), "`20-29 Test` should NOT be in areas");
        assert!(index.get_categories().iter().count() == 1, "categories should have 1 remaining");
        assert!(index.get_ids().is_empty(), "ids should be empty");
    } else {
        panic!("area_with_category_and_id index should pass");
    }
}

#[test]
fn test_remove_category() {
    use johnnydecimal::Category;

    if let Ok(mut index) = Index::new("20-29 Area\n22 Category\n22.03 Id\n22.05 Another\n40-49 Area 2\n43 Category 2") {
        let category_1 = Category::new("21 Test").expect("`21 Test` should be a valid category");
        let category_2 = Category::new("22 Test").expect("`22 Test` should be a valid category");

        assert!(index.get_categories().contains(&category_2), "`22 Test` should be in categories");

        assert!(index.remove_category(&category_1).is_err(), "`21 Test` should NOT be a valid category to remove");
        assert!(index.remove_category(&category_2).is_ok(), "`22 Test` should be a valid category to remove");

        assert!(!index.get_categories().contains(&category_2), "`22 Test` should NOT be in categories");
        assert!(index.get_ids().is_empty(), "ids should be empty");
    } else {
        panic!("area_with_category_and_id index should pass");
    }
}
