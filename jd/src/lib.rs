#![doc = include_str!("../README.md")]

use johnnydecimal::{Area, Category, Id, Index};
use std::fs;

/// A `System` consists of a `root` and an `index`.
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

        let path = self.index.derive_path_for_area(area)?;

        if fs::create_dir(self.root.clone() + &path).is_ok() {
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

        let path = self.index.derive_path_for_area(area)?;

        if trash::delete(self.root.clone() + &path).is_ok() {
            self.index.remove_area(area)
        } else {
            Err("The given area *was* in the index, but *wasn't* able to be moved to trash.")
        }
    }

    /// Adds a new `Category` to the `System`'s `Index`.
    ///
    /// If the category already exists in the cached index, the file won't be created.
    pub fn add_category(&mut self, category: &Category) -> Result<&Vec<Category>, &'static str> {
        if self.index.get_categories().contains(category) {
            return Err("Category already exists in index.");
        }

        let path = self.index.derive_path_for_category(category)?;

        if fs::create_dir(self.root.clone() + &path).is_ok() {
            self.index.add_category(category)
        } else {
            Err("A directory for the given category already exists, but wasn't in index.")
        }
    }

    /// Removes an existing `Area` from the `System`'s `Index`.
    ///
    /// TODO: Note that categories and their contents get sent to the user's trash,
    /// so affected ids should be removed from the `Index` as well.
    pub fn remove_category(&mut self, category: &Category) -> Result<&Vec<Category>, &'static str> {
        if !self.index.get_categories().contains(category) {
            todo!("Handle possibility that filesystem could have category but index doesn't")
        }

        let path = self.index.derive_path_for_category(category)?;

        if trash::delete(self.root.clone() + &path).is_ok() {
            self.index.remove_category(category)
        } else {
            Err("The given category *was* in the index, but *wasn't* able to be moved to trash.")
        }
    }

    /// Adds a new `Id` to the `System`'s `Index`.
    ///
    /// If the id already exists in the cached index, the file won't be created.
    pub fn add_id(&mut self, id: &Id) -> Result<&Vec<Id>, &'static str> {
        if self.index.get_ids().contains(id) {
            return Err("Id already exists in index.");
        }

        let path = self.index.derive_path_for_id(id)?;

        if fs::create_dir(self.root.clone() + &path).is_ok() {
            self.index.add_id(id)
        } else {
            Err("A directory for the given id already exists, but wasn't in index.")
        }
    }

    /// Removes an existing `Id` from the `System`'s `Index`.
    pub fn remove_id(&mut self, id: &Id) -> Result<&Vec<Id>, &'static str> {
        if !self.index.get_ids().contains(id) {
            todo!("Handle possibility that filesystem could have category but index doesn't")
        }

        let path = self.index.derive_path_for_id(id)?;

        if trash::delete(self.root.clone() + &path).is_ok() {
            self.index.remove_id(id)
        } else {
            Err("The given id *was* in the index, but *wasn't* able to be moved to trash.")
        }
    }

    /// Returns the current `Index` of the `System`.
    pub fn get_index(&self) -> &Index {
        &self.index
    }
}

fn get_index_from_fs(root: &str) -> Result<Index, &'static str> {
    let mut areas = vec![];
    let mut categories = vec![];
    let mut ids = vec![];

    let Ok(directory) = fs::read_dir(root) else {
        return Err("Couldn't read root directory");
    };

    for path in directory {
        let Ok(path) = path else {
            return Err("Couldn't path in path");
        };

        if !path.path().is_dir() {
            continue;
        }

        let maybe_area = path.file_name();

        let Some(maybe_area) = maybe_area.to_str() else {
            return Err("Couldn't convert path to str");
        };

        if let Ok(area) = Area::new(maybe_area) {
            if areas.contains(&area) {
                return Err("Given area is already in index");
            }

            areas.push(area);
        }

        let Ok(subdirs) = fs::read_dir(path.path()) else {
            return Err("Couldn't read subdirs");
        };

        for dir in subdirs {
            let Ok(dir) = dir else {
                return Err("Couldn't dir subdir");
            };

            if !dir.path().is_dir() {
                continue;
            }

            let maybe_category = dir.file_name();

            let Some(maybe_category) = maybe_category.to_str() else {
                return Err("Couldn't convert path to str");
            };

            if let Ok(category) = Category::new(maybe_category) {
                if categories.contains(&category) {
                    return Err("Given category is already in index");
                }

                categories.push(category);
            }

            let Ok(sub_dirs) = fs::read_dir(dir.path()) else {
                return Err("Couldn't read_dir of child");
            };

            for sub_dir in sub_dirs {
                let Ok(sub_dir) = sub_dir else {
                    return Err("Couldn't dir sub_dir");
                };

                if !sub_dir.path().is_dir() {
                    continue;
                }

                let maybe_id = sub_dir.file_name();

                let Some(maybe_id) = maybe_id.to_str() else {
                    return Err("Couldn't convert path to str");
                };

                if let Ok(id) = Id::new(maybe_id) {
                    if ids.contains(&id) {
                        return Err("Given id is already in index");
                    }

                    ids.push(id)
                }
            }
        }
    }

    if let Ok(index) = Index::with_vecs(&areas, &categories, &ids) {
        return Ok(index);
    }

    Err("An index couldn't be created")
}
