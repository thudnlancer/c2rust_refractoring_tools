// defer.rs
// Copyright (C) 1993-2023 Free Software Foundation, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public
// License along with this program; if not, write to the Free
// Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
// Boston, MA 02110-1301 USA.

use std::ffi::CString;
use std::ptr;

#[derive(Debug)]
pub struct CpioFileStat {
    pub c_name: CString,
    // Include other fields from the original struct as needed
}

#[derive(Debug)]
pub struct Deferment {
    pub next: Option<Box<Deferment>>,
    pub header: CpioFileStat,
}

impl Deferment {
    pub fn create(file_hdr: &CpioFileStat) -> Result<Box<Self>, std::ffi::NulError> {
        let c_name = CString::new(file_hdr.c_name.as_bytes())?;
        Ok(Box::new(Deferment {
            next: None,
            header: CpioFileStat {
                c_name,
                // Copy other fields from file_hdr as needed
            },
        }))
    }
}

impl Drop for Deferment {
    fn drop(&mut self) {
        // Rust's ownership system will automatically handle the memory deallocation
        // The CStrings and Boxes will be dropped properly
    }
}