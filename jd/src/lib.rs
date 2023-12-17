use johnnydecimal::{Area, Category, Id, Index};
use std::fs;

#[derive(Debug)]
pub struct System {
    root: String,
    index: Index,
}

impl System {
    /// Creates a new `System` from a given `root`.
    ///
    /// A `System` is an `Index` instantiated from a given `root` on a filesystem. Put simply, it's
    /// the glue between the theoretical concept of an `Index` and its implementation on the
    /// filesystem.
    pub fn new(root: &str) -> Result<Self, &'static str> {
        if let Ok(string) = fs::read_to_string(format!("{root}/00.00 Index.txt")) {
            if let Ok(index) = Index::new(&string) {
                if let Ok(index_fs) = get_index_from_fs(root) {
                    if index != index_fs {
                        todo!("Handle filesystem and index file being different");
                    }
                }

                return Ok(Self { root: root.to_string(), index });
            }
        }

        if let Ok(index) = get_index_from_fs(root) {
            return Ok(Self { root: root.to_string(), index });
        }

        Err("Couldn't get index from file or directory contents")
    }

    /// Adds a new `Area` to the `System`'s `Index`.
    ///
    /// If the area already exists in the cached index, the file won't be created.
    pub fn add_area(&mut self, area: &Area) -> Result<&Vec<Area>, &'static str> {
        if self.index.get_areas().contains(area) {
            return Err("Area already exists in index.");
        }

        if fs::create_dir(format!("{}/{} {}", self.root, area.get_area(), area.get_name())).is_ok() {
            self.index.add_area(area)
        } else {
            Err("A directory for the given area already exists, but wasn't in index.")
        }
    }

    /// Removes an existing `Area` from the `System`'s `Index`.
    ///
    /// Note that areas get sent to the user's trash, although it may be useful to provide a
    /// warning beforehand or an option to quickly undo in the UI.
    pub fn remove_area(&mut self, area: &Area) -> Result<&Vec<Area>, &'static str> {
        if !self.index.get_areas().contains(area) {
            todo!("Handle possibility that filesystem could have area but index doesn't")
        }

        let path = self.index.get_path_from_area(area)?;

        if trash::delete(self.root.clone() + &path).is_ok() {
            self.index.remove_area(area)
        } else {
            Err("The given area *was* in the index, but *wasn't* able to be moved to trash.")
        }
    }
}

fn get_index_from_fs(root: &str) -> Result<Index, std::io::Error> {
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
