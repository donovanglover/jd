use jd::System;
use johnnydecimal::{Area, Category, Id};

#[test]
fn area_add() {
    let mut system = System::new("../test_systems/empty").expect("Test system should be valid");

    let area = Area::new("40-49 Area 4").expect("`40-49 Area 4` should be valid");
    let category = Category::new("43 New Category").expect("`43 New Category` should be valid");
    let id = Id::new("43.05 New Id").expect("`43.05 New Id` should be valid");

    assert_eq!(system.add_area(&area), Ok(&vec![area.clone()]), "should be able to add a new area `40-49 Area 4`");
    assert_eq!(system.add_category(&category), Ok(&vec![category.clone()]), "should be able to add a new category `43 New Category`");
    assert_eq!(system.add_id(&id), Ok(&vec![id.clone()]), "should be able to add a new id `43.05 New Id`");

    assert!(system.remove_id(&id).is_ok(), "should be able to remove the added id `43.05 New Id`");
    assert!(system.remove_category(&category).is_ok(), "should be able to remove the added category `43 New Category`");
    assert!(system.remove_area(&area).is_ok(), "should be able to remove the added area `40-49 Area 4`");
}
