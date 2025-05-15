// Copyright 1997,2000-2003,2007-2010 Alain Knaff.  This file is
// part of mtools.
//
// Mtools is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mtools is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mtools.  If not, see <http://www.gnu.org/licenses/>.

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::fmt;

#[derive(Debug)]
pub struct Direntry {
    entry: i32,
    dir: Box<Stream>,
    begin_slot: u32,
    end_slot: u32,
    name: OsString,
}

#[derive(Debug)]
pub struct Stream {
    // Stream implementation details
    drive: char,
    direntry: Option<Box<Direntry>>,
}

impl Direntry {
    pub fn initialize(&mut self, dir: Stream) {
        self.entry = -1;
        self.dir = Box::new(dir);
        self.begin_slot = 0;
        self.end_slot = 0;
    }

    pub fn is_not_found(&self) -> bool {
        self.entry == -2
    }

    pub fn is_root_entry(&self) -> bool {
        self.entry == -3
    }

    pub fn set_entry_for_iteration(&mut self, pos: u32) {
        assert!(pos <= i32::MAX as u32);
        self.entry = pos as i32 - 1;
    }

    pub fn set_entry_to_pos(&mut self, pos: u32) {
        assert!(pos <= i32::MAX as u32);
        self.entry = pos as i32;
    }

    pub fn get_entry_as_pos(&self) -> u32 {
        assert!(self.entry >= 0);
        self.entry as u32
    }

    pub fn get_next_entry_as_pos(&self) -> u32 {
        assert!(self.entry >= -1);
        (self.entry + 1) as u32
    }

    pub fn get_parent(&self) -> Option<&Direntry> {
        self.dir.direntry.as_deref()
    }

    fn get_path_len(&self) -> usize {
        let mut length = 0;
        let mut current = self;

        loop {
            if current.is_root_entry() {
                return length + 3;
            }

            length += 1 + current.name.len();
            current = match current.get_parent() {
                Some(parent) => parent,
                None => break length,
            };
        }
    }

    fn sprint_pwd(&self, buffer: &mut String, len_available: &mut usize) {
        if self.is_root_entry() {
            buffer.push(self.dir.drive);
            buffer.push(':');
            buffer.push('/');
            *len_available -= 3;
        } else {
            if let Some(parent) = self.get_parent() {
                parent.sprint_pwd(buffer, len_available);
            }
            
            if !buffer.ends_with('/') {
                buffer.push('/');
                *len_available -= 1;
            }
            
            let name = self.name.to_string_lossy();
            buffer.push_str(&name);
            *len_available -= name.len();
        }
    }

    pub fn fprint_pwd<W: Write>(&self, f: &mut W, escape: bool) -> io::Result<()> {
        if escape {
            write!(f, "\"")?;
        }
        self.mt_fprint_pwd(f, false, escape)?;
        if escape {
            write!(f, "\"")?;
        }
        Ok(())
    }

    fn mt_fprint_pwd<W: Write>(&self, f: &mut W, recurs: bool, escape: bool) -> io::Result<()> {
        if self.is_root_entry() {
            write!(f, "{}:", self.dir.drive)?;
            if !recurs {
                write!(f, "/")?;
            }
        } else {
            if let Some(parent) = self.get_parent() {
                parent.mt_fprint_pwd(f, true, escape)?;
            }
            
            if escape && self.name.to_string_lossy().contains(&['"', '$', '\\'][..]) {
                write!(f, "/")?;
                for c in self.name.to_string_lossy().chars() {
                    if c == '"' || c == '$' || c == '\\' {
                        write!(f, "\\")?;
                    }
                    write!(f, "{}", c)?;
                }
            } else {
                write!(f, "/{}", self.name.to_string_lossy())?;
            }
        }
        Ok(())
    }

    pub fn fprint_short_pwd<W: Write>(&self, f: &mut W) -> io::Result<()> {
        self.mt_fprint_short_pwd(f, false)
    }

    fn mt_fprint_short_pwd<W: Write>(&self, f: &mut W, recurs: bool) -> io::Result<()> {
        if self.is_root_entry() {
            write!(f, "{}:", self.dir.drive)?;
            if !recurs {
                write!(f, "/")?;
            }
        } else {
            if let Some(parent) = self.get_parent() {
                parent.mt_fprint_short_pwd(f, true)?;
            }
            
            // Simplified short name representation
            // Note: Original C code uses dir.name and dir.ext fields which aren't
            // represented in this Rust version. You'd need to add those fields
            // to the Direntry struct for full functionality.
            write!(f, "/{}", self.name.to_string_lossy())?;
        }
        Ok(())
    }

    pub fn get_pwd(&self) -> Option<String> {
        let size = self.get_path_len();
        let mut buffer = String::with_capacity(size * 4 + 1);
        let mut len_available = buffer.capacity();
        self.sprint_pwd(&mut buffer, &mut len_available);
        Some(buffer)
    }
}

pub fn is_subdir_of(inside: &Stream, outside: &Stream) -> bool {
    let mut current = inside;
    
    loop {
        if std::ptr::eq(current, outside) {
            return true;
        }
        
        if let Some(direntry) = &current.direntry {
            if direntry.is_root_entry() {
                return false;
            }
            current = &direntry.dir;
        } else {
            return false;
        }
    }
}