use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::sync::Arc;
use std::collections::hash_map::{HashMap, Entry};

type Ino = u64;
type Dev = u64;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct FileTriple {
    name: Arc<CString>,
    st_ino: Ino,
    st_dev: Dev,
}

#[derive(Debug, Default)]
pub struct FileTable {
    table: HashMap<FileTriple, ()>,
}

impl FileTable {
    pub fn new() -> Self {
        FileTable {
            table: HashMap::new(),
        }
    }

    pub fn record_file(&mut self, file: &CStr, stats: &libc::stat) {
        let ent = FileTriple {
            name: Arc::new(CString::new(file.to_bytes()).unwrap()),
            st_ino: stats.st_ino as Ino,
            st_dev: stats.st_dev as Dev,
        };

        match self.table.entry(ent) {
            Entry::Occupied(_) => (),
            Entry::Vacant(v) => {
                v.insert(());
            }
        }
    }

    pub fn seen_file(&self, file: &CStr, stats: &libc::stat) -> bool {
        let new_ent = FileTriple {
            name: Arc::new(CString::new(file.to_bytes()).unwrap()),
            st_ino: stats.st_ino as Ino,
            st_dev: stats.st_dev as Dev,
        };

        self.table.contains_key(&new_ent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_file_table() {
        let mut ft = FileTable::new();
        let file = CString::new("test.txt").unwrap();
        let mut stats = unsafe { std::mem::zeroed::<libc::stat>() };
        stats.st_ino = 123;
        stats.st_dev = 456;

        assert!(!ft.seen_file(&file, &stats));
        ft.record_file(&file, &stats);
        assert!(ft.seen_file(&file, &stats));
    }
}