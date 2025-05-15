// help detect directory cycles efficiently

// Copyright (C) 2003-2004, 2006, 2009-2022 Free Software Foundation, Inc.
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

// Written by Jim Meyering

use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::fs;

const CC_MAGIC: i32 = 9827862;

#[derive(Debug)]
pub struct CycleCheckState {
    dev_ino: (u64, u64), // (st_dev, st_ino)
    chdir_counter: u64,
    magic: i32,
}

impl CycleCheckState {
    pub fn new() -> Self {
        CycleCheckState {
            dev_ino: (0, 0),
            chdir_counter: 0,
            magic: CC_MAGIC,
        }
    }

    /// Check for directory cycles
    pub fn check(&mut self, metadata: &fs::Metadata) -> bool {
        assert_eq!(self.magic, CC_MAGIC);

        // If the current directory ever happens to be the same
        // as the one we last recorded for the cycle detection,
        // then it's obviously part of a cycle.
        if self.chdir_counter != 0 && 
           self.dev_ino.0 == metadata.dev() && 
           self.dev_ino.1 == metadata.ino() {
            return true;
        }

        // If the number of "descending" chdir calls is a power of two,
        // record the dev/ino of the current directory.
        self.chdir_counter = self.chdir_counter.saturating_add(1);
        if is_zero_or_power_of_two(self.chdir_counter) {
            // On overflow, assume there's a cycle
            if self.chdir_counter == 0 {
                return true;
            }

            self.dev_ino = (metadata.dev(), metadata.ino());
        }

        false
    }

    /// Reflect chdir up operation
    pub fn reflect_chdir_up(&mut self, dir_metadata: &fs::Metadata, subdir_metadata: &fs::Metadata) {
        // You must call cycle_check at least once before using this method.
        if self.chdir_counter == 0 {
            panic!("cycle_check must be called at least once before reflect_chdir_up");
        }

        if self.dev_ino.0 == subdir_metadata.dev() && 
           self.dev_ino.1 == subdir_metadata.ino() {
            self.dev_ino = (dir_metadata.dev(), dir_metadata.ino());
        }
    }
}

/// Return true if i is a power of 2, or is zero.
fn is_zero_or_power_of_two(i: u64) -> bool {
    (i & (i.wrapping_sub(1))) == 0
}