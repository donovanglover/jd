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
        if let Ok(stuff) = get_categories_from_areas(&areas, root) {
            dbg!(areas);
            dbg!(stuff.0);
            dbg!(stuff.1);
        }

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

fn get_categories_from_areas(areas: &Vec<String>, root: &str) -> Result<(Vec<Category>, Vec<Id>), std::io::Error> {
    let mut categories = vec![];
    let mut ids = vec![];

    for area in areas {
        let path = root.to_owned() + "/" + area;
        let dirs = fs::read_dir(path)?;

        for dir in dirs {
            let dir = dir?;

            if let Some(dir_name) = dir.file_name().to_str() {
                if let Ok(category) = Category::new(dir_name) {
                    categories.push(category);
                } else {
                    todo!("Warn if non-Johnny.Decimal stuff in directory?")
                }
            }

            if let Ok(sub_dirs) = fs::read_dir(dir.path()) {
                for sub_dir in sub_dirs {
                    let sub_dir = sub_dir?;

                    if let Some(sub_dir_name) = sub_dir.file_name().to_str() {
                        if let Ok(id) = Id::new(sub_dir_name) {
                            ids.push(id)
                        } else {
                            todo!("Warn if non-Johnny.Decimal stuff in directory?")
                        }
                    }
                }
            }

        }

    }

    Ok((categories, ids))
}
