/* Iterate over arguments from argv or --files0-from=FILE
   Copyright (C) 2008-2022 Free Software Foundation, Inc.

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

use std::fs::File;
use std::io::{self, Read, BufReader};
use std::path::Path;
use std::ffi::CString;
use std::slice;

#[derive(Debug, PartialEq)]
enum ArgvIterError {
    Ok,
    Eof,
    Memory,
    Read,
}

struct ArgvIterator {
    // File mode: reader and state
    reader: Option<BufReader<File>>,
    item_idx: usize,
    buffer: Vec<u8>,
    
    // Argv mode: arguments and current position
    arg_list: Vec<CString>,
    current_pos: usize,
}

impl ArgvIterator {
    fn init_argv(args: Vec<CString>) -> Self {
        ArgvIterator {
            reader: None,
            item_idx: 0,
            buffer: Vec::new(),
            arg_list: args,
            current_pos: 0,
        }
    }

    fn init_stream(file: File) -> Self {
        ArgvIterator {
            reader: Some(BufReader::new(file)),
            item_idx: 0,
            buffer: Vec::new(),
            arg_list: Vec::new(),
            current_pos: 0,
        }
    }

    fn next(&mut self) -> Result<Option<CString>, ArgvIterError> {
        if let Some(reader) = &mut self.reader {
            self.buffer.clear();
            let mut byte = [0u8; 1];
            
            loop {
                match reader.read_exact(&mut byte) {
                    Ok(_) => {
                        if byte[0] == 0 {
                            if self.buffer.is_empty() {
                                continue;
                            }
                            self.item_idx += 1;
                            return match CString::new(self.buffer.clone()) {
                                Ok(s) => Ok(Some(s)),
                                Err(_) => Err(ArgvIterError::Memory),
                            };
                        }
                        self.buffer.push(byte[0]);
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::UnexpectedEof {
                            if self.buffer.is_empty() {
                                return Ok(None);
                            } else {
                                self.item_idx += 1;
                                return match CString::new(self.buffer.clone()) {
                                    Ok(s) => Ok(Some(s)),
                                    Err(_) => Err(ArgvIterError::Memory),
                                };
                            }
                        }
                        return Err(ArgvIterError::Read);
                    }
                }
            }
        } else {
            if self.current_pos >= self.arg_list.len() {
                return Ok(None);
            }
            let item = self.arg_list[self.current_pos].clone();
            self.current_pos += 1;
            Ok(Some(item))
        }
    }

    fn n_args(&self) -> usize {
        if self.reader.is_some() {
            self.item_idx
        } else {
            self.current_pos
        }
    }
}

impl Drop for ArgvIterator {
    fn drop(&mut self) {
        // Rust's ownership system handles cleanup automatically
    }
}

// Helper functions to create iterators
fn argv_iter_init_argv(args: Vec<CString>) -> ArgvIterator {
    ArgvIterator::init_argv(args)
}

fn argv_iter_init_stream(file: File) -> ArgvIterator {
    ArgvIterator::init_stream(file)
}