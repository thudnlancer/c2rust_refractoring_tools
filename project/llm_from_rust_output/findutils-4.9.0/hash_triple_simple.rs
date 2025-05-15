use std::ffi::CString;
use std::os::raw::{c_char, c_ulong};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

type DevT = c_ulong;
type InoT = c_ulong;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FTriple {
    pub name: CString,
    pub st_ino: InoT,
    pub st_dev: DevT,
}

impl FTriple {
    pub fn new(name: CString, st_ino: InoT, st_dev: DevT) -> Self {
        Self { name, st_ino, st_dev }
    }
}

impl Hash for FTriple {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.st_ino.hash(state);
    }
}

pub fn triple_hash(triple: &FTriple, table_size: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    triple.hash(&mut hasher);
    (hasher.finish() as usize) % table_size
}

pub fn triple_compare_ino_str(a: &FTriple, b: &FTriple) -> bool {
    a.st_ino == b.st_ino && a.st_dev == b.st_dev && a.name == b.name
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_triple_operations() {
        let name1 = CString::new("test1").unwrap();
        let name2 = CString::new("test2").unwrap();
        
        let triple1 = FTriple::new(name1.clone(), 123, 456);
        let triple2 = FTriple::new(name1.clone(), 123, 456);
        let triple3 = FTriple::new(name2, 123, 456);

        assert!(triple_compare_ino_str(&triple1, &triple2));
        assert!(!triple_compare_ino_str(&triple1, &triple3));

        let hash1 = triple_hash(&triple1, 100);
        let hash2 = triple_hash(&triple2, 100);
        assert_eq!(hash1, hash2);
    }
}