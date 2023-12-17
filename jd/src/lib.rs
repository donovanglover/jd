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

#[derive(Debug)]
pub struct System {
    root: String,
    index: Index,
}

impl System {
    pub fn new(root: &str) -> Result<Self, &'static str> {
        if let Ok(string) = std::fs::read_to_string(format!("{root}/00.00 Index.txt")) {
            if let Ok(index) = Index::new(&string) {
                // TODO: Validate that index is valid with filesystem?
                // Depends on whether System::new() is used once or multiple times
                return Ok(Self {
                    root: root.to_string(),
                    index,
                })
            }
        }

        match get_stuff(root) {
            Ok(stuff) => {
                dbg!(stuff);
            }

            Err(e) => {
                dbg!(e);
            }
        }

        Ok(Self {
            root: root.to_string(),
            index: Index::new("").unwrap(),
        })
    }

    pub fn add_area(&mut self, area: Area) -> Result<(), &'static str> {
        if self.index.get_areas().contains(&area) {
            return Err("Area already exists in index.");
        }

        if fs::create_dir(format!("{}/{} {}", self.root, area.get_area(), area.get_name())).is_ok() {
            if self.index.add_area(area).is_ok() {
                dbg!("Added area");
            }
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

fn get_stuff(root: &str) -> Result<Index, std::io::Error> {
    let mut areas = vec![];
    let mut categories = vec![];
    let mut ids = vec![];
    let directory = fs::read_dir(root)?;

    for path in directory {
        let path = path?;

        if !path.path().is_dir() {
            continue;
        }

        if let Ok(area) = Area::new(path.file_name().to_str().ok_or(std::io::ErrorKind::Other)?) {
            if areas.contains(&area) {
                return Err(std::io::ErrorKind::Other.into());
            }

            areas.push(area);
        }

        let subdirs = fs::read_dir(path.path())?;

        for dir in subdirs {
            let dir = dir?;

            if !dir.path().is_dir() {
                continue;
            }

            if let Ok(category) = Category::new(dir.file_name().to_str().ok_or(std::io::ErrorKind::Other)?) {
                if categories.contains(&category) {
                    return Err(std::io::ErrorKind::Other.into());
                }

                categories.push(category);
            }

            let sub_dirs = fs::read_dir(dir.path())?;

            for sub_dir in sub_dirs {
                let sub_dir = sub_dir?;

                if !sub_dir.path().is_dir() {
                    continue;
                }

                if let Ok(id) = Id::new(sub_dir.file_name().to_str().ok_or(std::io::ErrorKind::Other)?) {
                    if ids.contains(&id) {
                        return Err(std::io::ErrorKind::Other.into());
                    }

                    ids.push(id)
                }
            }
        }
    }

    if let Ok(index) = Index::with_vecs(&areas, &categories, &ids) {
        return Ok(index);
    }

    Err(std::io::ErrorKind::Other.into())
}
