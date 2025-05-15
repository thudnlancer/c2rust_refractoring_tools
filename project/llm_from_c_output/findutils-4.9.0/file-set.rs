/* Very specialized set-of-files code.
   Copyright (C) 2007-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Jim Meyering, 2007.  */

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::fs::Metadata;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FileTriple {
    name: String,
    st_ino: u64,
    st_dev: u64,
}

impl FileTriple {
    fn new(name: String, metadata: &Metadata) -> Self {
        Self {
            name,
            st_ino: metadata.ino(),
            st_dev: metadata.dev(),
        }
    }
}

type HashTable = std::collections::HashSet<FileTriple>;

/// Record file, FILE, and dev/ino from metadata, in the hash table, HT.
/// If HT is None, return immediately.
/// If memory allocation fails, exit immediately.
pub fn record_file(ht: &mut Option<HashTable>, file: &Path, metadata: &Metadata) {
    if ht.is_none() {
        return;
    }

    let ht = ht.as_mut().unwrap();
    let ent = FileTriple::new(file.to_string_lossy().into_owned(), metadata);

    if !ht.insert(ent) {
        // There was already a matching entry in the table
    }
}

/// Return true if there is an entry in hash table, HT,
/// for the file described by FILE and metadata.
pub fn seen_file(ht: &Option<HashTable>, file: &Path, metadata: &Metadata) -> bool {
    if ht.is_none() {
        return false;
    }

    let ht = ht.as_ref().unwrap();
    let new_ent = FileTriple::new(file.to_string_lossy().into_owned(), metadata);

    ht.contains(&new_ent)
}