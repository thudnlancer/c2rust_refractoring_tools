// Hash functions for file-related triples: name, device, inode.
// Copyright (C) 2007, 2009-2023 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// written by Jim Meyering

use std::hash::{Hash, Hasher};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

#[derive(Debug)]
pub struct FTriple {
    pub name: String,
    pub st_dev: u64,
    pub st_ino: u64,
}

impl FTriple {
    /// Hash an FTriple, considering the file name.
    pub fn hash(&self, table_size: usize) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.name.hash(&mut hasher);
        let tmp = hasher.finish() as usize % table_size;
        
        // Ignoring the device number here should be fine.
        (tmp ^ self.st_ino as usize) % table_size
    }

    /// Compare two FTriple structs.
    pub fn compare_ino_str(&self, other: &Self) -> bool {
        self.st_dev == other.st_dev && 
        self.st_ino == other.st_ino && 
        self.name == other.name
    }
}

impl Drop for FTriple {
    fn drop(&mut self) {
        // Rust's ownership system automatically handles the memory
        // No explicit free needed
    }
}