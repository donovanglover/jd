use jd::System;
use johnnydecimal::Area;

#[test]
fn area_add() {
    let mut system = System::new("../test_systems/empty").expect("Test system should be valid");
    let area = Area::new("40-49 Area 4").expect("`40-49 Area 4` should be valid");

    assert!(system.add_area(&area).is_ok(), "should be able to add a new area `40-49 Area 4`");
    assert!(system.remove_area(&area).is_ok(), "should be able to remove the added area `40-49 Area 4`");
}
