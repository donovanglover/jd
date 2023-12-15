use johnnydecimal::{Area, Category, Id, Index};
use std::fs;

/// List areas
pub fn list_areas() {
    let mut areas: Vec<String> = Vec::new();

    for entry in fs::read_dir("/home/user").unwrap() {
        let path = entry.as_ref().unwrap().path();

        if path.is_dir() {
            let path = path.file_name().unwrap().to_str().unwrap().to_string();

            if Area::new(&path).is_ok() {
                areas.push(path);
            }
        }
    }

    areas.sort();

    for area in areas {
        println!("{area}");
    }
}

/// Builds a Johnny Decimal `Index` from a given `Path`.
// pub fn build_index() -> johnnydecimal::Index {
//
// }

#[derive(Debug)]
pub struct System {
    root: String,
    index: Index,
}

impl System {
    pub fn new(root: &str) -> Result<Self, &'static str> {
        let areas = get_dirs_from_path(root);
        let (categories, ids) = get_categories_from_areas(&areas, root);

        dbg!(areas);
        dbg!(categories);
        dbg!(ids);

        Ok(Self {
            root: root.to_string(),
            index: Index::new("").unwrap(),
        })
    }

    pub fn add_area(&mut self, area: Area) -> Result<(), &'static str> {
        if self.index.areas.contains(&area) {
            return Err("Area already exists in index.");
        }

        if fs::create_dir(format!("{}/{} {}", self.root, area.area, area.name)).is_ok() {
            self.index.areas.push(area);
            self.index.areas.sort_unstable();
        } else {
            return Err("Directory for Area already exists.");
        }

        Ok(())
    }

    // pub fn remove_area(area: Area) {
    //
    // }
    //
    // pub fn add_category(category: Category) {
    //
    // }
    //
    // pub fn remove_category(category: Category) {
    //
    // }
    //
    // pub fn add_id(id: Id) {
    //
    // }
    //
    // pub fn remove_id(id: Id) {
    //
    // }
}

fn get_dirs_from_path(path: &str) -> Vec<String> {
    let mut directories = vec![];

    if let Ok(directory) = fs::read_dir(path) {
        for path in directory {
            if let Ok(path) = path {
                if let Ok(dir_name) = path.file_name().into_string() {
                    directories.push(dir_name);
                }
            }
        }
    }

    directories
}

fn get_categories_from_areas(areas: &Vec<String>, root: &str) -> (Vec<String>, Vec<String>) {
    let mut categories = vec![];
    let mut ids = vec![];

    for area in areas {
        let path = root.to_owned() + "/" + area;

        if let Ok(dirs) = fs::read_dir(path) {
            for dir in dirs {
                if let Ok(dir) = dir {
                    // categories.push(Category::new(dir.file_name().to_str().unwrap()).unwrap());
                    if let Ok(dir_name) = dir.file_name().into_string() {
                        categories.push(dir_name);
                    }

                    if let Ok(sub_dirs) = fs::read_dir(dir.path()) {
                        for sub_dir in sub_dirs {
                            if let Ok(sub_dir) = sub_dir {
                                if let Ok(sub_dir_name) = sub_dir.file_name().into_string() {
                                    ids.push(sub_dir_name)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (categories, ids)
}
