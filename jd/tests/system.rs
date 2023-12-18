use jd::System;
use johnnydecimal::{Area, Category, Id, Index};

#[test]
fn test_adding_and_removing_things() {
    let mut system = System::new("../tests/empty").expect("Test system should be valid");

    let area = Area::new("40-49 Area 4").expect("`40-49 Area 4` should be valid");
    let category = Category::new("43 New Category").expect("`43 New Category` should be valid");
    let id = Id::new("43.05 New Id").expect("`43.05 New Id` should be valid");

    assert_eq!(system.add_area(&area), Ok(&vec![area.clone()]), "should be able to add a new area `40-49 Area 4`");
    assert!(system.add_area(&area).is_err(), "should fail when adding an existing area");

    assert_eq!(system.add_category(&category), Ok(&vec![category.clone()]), "should be able to add a new category `43 New Category`");
    assert!(system.add_category(&category).is_err(), "should fail when adding an existing category");

    assert_eq!(system.add_id(&id), Ok(&vec![id.clone()]), "should be able to add a new id `43.05 New Id`");
    assert!(system.add_id(&id).is_err(), "should fail when adding an existing id");

    assert!(system.remove_id(&id).is_ok(), "should be able to remove the added id `43.05 New Id`");
    assert!(system.remove_category(&category).is_ok(), "should be able to remove the added category `43 New Category`");
    assert!(system.remove_area(&area).is_ok(), "should be able to remove the added area `40-49 Area 4`");
}

#[test]
fn file_index_different_than_filesystem() {
    assert!(System::new("../tests/sync").is_err(), "should fail if index different than filesystem (out of sync)");
}

#[test]
fn read_from_file() {
    let system = System::new("../tests/file").expect("should be valid from file");

    let areas = vec![
        Area::new("10-19 Your first area's title").expect("should be valid area"),
        Area::new("20-29 Your second area").expect("should be valid area"),
    ];

    let categories = vec![
        Category::new("11 Your first category's title").expect("should be valid category"),
        Category::new("12 Category twelve").expect("should be valid category"),
        Category::new("21 Category twenty-one").expect("should be valid category"),
    ];

    let ids = vec![
        Id::new("11.01 Your first ID's title").expect("should be valid id"),
        Id::new("11.02 The second ID in category 11").expect("should be valid id"),
        Id::new("21.01 and so on").expect("should be valid id"),
    ];

    let index = Index::with_vecs(&areas, &categories, &ids).expect("should be valid index");

    assert_eq!(*system.get_index(), index, "should create correct index from verified index file");
}
