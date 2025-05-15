// trav.rs
//
// Copyright (C) 2018 Patrick Alken
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

use std::ptr;
use std::mem;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BstError {
    details: String
}

impl BstError {
    fn new(msg: &str) -> BstError {
        BstError{details: msg.to_string()}
    }
}

impl fmt::Display for BstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for BstError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub trait BstType {
    fn trav_init(&self, trav_data: &mut [u8], table: &[u8]) -> Result<(), BstError>;
    fn trav_first(&self, trav_data: &mut [u8], table: &[u8]) -> Option<&[u8]>;
    fn trav_last(&self, trav_data: &mut [u8], table: &[u8]) -> Option<&[u8]>;
    fn trav_find(&self, item: &[u8], trav_data: &mut [u8], table: &[u8]) -> Option<&[u8]>;
    fn trav_insert(&self, item: &mut [u8], trav_data: &mut [u8], table: &mut [u8]) -> Option<&[u8]>;
    fn trav_copy(&self, dest_trav_data: &mut [u8], src_trav_data: &[u8]) -> Option<&[u8]>;
    fn trav_next(&self, trav_data: &mut [u8]) -> Option<&[u8]>;
    fn trav_prev(&self, trav_data: &mut [u8]) -> Option<&[u8]>;
    fn trav_cur(&self, trav_data: &[u8]) -> Option<&[u8]>;
    fn trav_replace(&self, trav_data: &mut [u8], new_item: &mut [u8]) -> Option<&[u8]>;
}

pub struct BstTrav {
    trav_data: Vec<u8>,
    type_: Box<dyn BstType>,
}

pub struct BstWorkspace {
    table: Vec<u8>,
    type_: Box<dyn BstType>,
}

impl BstTrav {
    pub fn init(&mut self, w: &BstWorkspace) -> Result<(), BstError> {
        self.type_ = w.type_.clone();
        self.type_.trav_init(&mut self.trav_data, &w.table)
    }

    pub fn first(&mut self, w: &BstWorkspace) -> Option<&[u8]> {
        self.type_ = w.type_.clone();
        self.type_.trav_first(&mut self.trav_data, &w.table)
    }

    pub fn last(&mut self, w: &BstWorkspace) -> Option<&[u8]> {
        self.type_ = w.type_.clone();
        self.type_.trav_last(&mut self.trav_data, &w.table)
    }

    pub fn find(&mut self, item: &[u8], w: &BstWorkspace) -> Option<&[u8]> {
        self.type_ = w.type_.clone();
        self.type_.trav_find(item, &mut self.trav_data, &w.table)
    }

    pub fn insert(&mut self, item: &mut [u8], w: &mut BstWorkspace) -> Option<&[u8]> {
        self.type_ = w.type_.clone();
        self.type_.trav_insert(item, &mut self.trav_data, &mut w.table)
    }

    pub fn copy(&mut self, src: &BstTrav) -> Option<&[u8]> {
        self.type_ = src.type_.clone();
        self.type_.trav_copy(&mut self.trav_data, &src.trav_data)
    }

    pub fn next(&mut self) -> Option<&[u8]> {
        self.type_.trav_next(&mut self.trav_data)
    }

    pub fn prev(&mut self) -> Option<&[u8]> {
        self.type_.trav_prev(&mut self.trav_data)
    }

    pub fn cur(&self) -> Option<&[u8]> {
        self.type_.trav_cur(&self.trav_data)
    }

    pub fn replace(&mut self, new_item: &mut [u8]) -> Option<&[u8]> {
        self.type_.trav_replace(&mut self.trav_data, new_item)
    }
}