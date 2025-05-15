use std::ffi::CString;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type Ino = u64;
pub type Dev = u64;

#[derive(Debug, Clone)]
pub struct FTriple {
    pub name: CString,
    pub st_ino: Ino,
    pub st_dev: Dev,
}

impl FTriple {
    pub fn new(name: CString, st_ino: Ino, st_dev: Dev) -> Self {
        Self { name, st_ino, st_dev }
    }
}

impl Hash for FTriple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        let name_hash = hasher.finish();
        (name_hash ^ self.st_ino).hash(state);
    }
}

impl PartialEq for FTriple {
    fn eq(&self, other: &Self) -> bool {
        self.st_ino == other.st_ino && 
        self.st_dev == other.st_dev && 
        self.name == other.name
    }
}

impl Eq for FTriple {}