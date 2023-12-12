use std::fs;
use jd::Area;

/// List areas
pub fn list_areas() {
    let mut areas: Vec<String> = Vec::new();

    for entry in fs::read_dir("/home/user").unwrap() {
        let path = entry.as_ref().unwrap().path();

        if path.is_dir() {
            let path = path.file_name().unwrap().to_str().unwrap().to_string();

            if Area::from_str(&path).is_ok() {
                areas.push(path);
            }
        }
    }

    areas.sort();

    for area in areas {
        println!("{area}");
    }
}
