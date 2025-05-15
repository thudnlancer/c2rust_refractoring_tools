use std::collections::HashMap;
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::Path;

#[derive(Debug)]
struct SharefileEntry {
    device: u64,
    inode: u64,
    name: String,
    file: File,
}

impl SharefileEntry {
    fn new(name: &str, file: File) -> io::Result<Self> {
        let metadata = file.metadata()?;
        Ok(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            name: name.to_string(),
            file,
        })
    }
}

#[derive(Debug)]
pub struct Sharefile {
    mode: String,
    table: HashMap<(u64, u64), SharefileEntry>,
}

impl Sharefile {
    pub fn new(mode: &str) -> Option<Self> {
        Some(Self {
            mode: mode.to_string(),
            table: HashMap::new(),
        })
    }

    pub fn open(&mut self, filename: &str) -> io::Result<&File> {
        let file = OpenOptions::new()
            .read(self.mode.contains('r'))
            .write(self.mode.contains('w'))
            .append(self.mode.contains('a'))
            .create(self.mode.contains('+'))
            .open(filename)?;

        let entry = SharefileEntry::new(filename, file)?;
        let key = (entry.device, entry.inode);

        if !self.table.contains_key(&key) {
            self.table.insert(key, entry);
        }

        Ok(&self.table[&key].file)
    }
}

impl Drop for Sharefile {
    fn drop(&mut self) {
        for (_, entry) in self.table.drain() {
            if let Err(e) = std::fs::File::sync_all(&entry.file) {
                eprintln!("Error closing file {}: {}", entry.name, e);
            }
        }
    }
}