// misc.rs

// Original copyright notice:
// Copyright (C) 2002, 2003, 2008, 2011 Niels Möller
//
// This file is part of GNU Nettle.
//
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
//
//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.
//
// or
//
//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.
//
// or both in parallel, as here.
//
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

use std::fmt;
use std::process;
use std::alloc::{self, Layout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SexpMode {
    Canonical = 0,
    Advanced = 1,
    Transport = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SexpToken {
    String,
    Display,
    Comment,
    ListStart,
    ListEnd,
    Eof,
    DisplayStart,
    DisplayEnd,
    TransportStart,
    CodingEnd,
}

pub const SEXP_TOKEN_CHARS: [u8; 0x80] = [
    /* 0, ... 0x1f */
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
    /* SPC ! " # $ % & ' ( ) * + , - . / */
    0,0,0,0,0,0,0,0, 0,0,1,1,0,1,1,1,
    /* 0 1 2 3 4 5 6 7  8 9 : ; < = > ? */
    1,1,1,1,1,1,1,1, 1,1,1,0,0,1,0,0,
    /* @ A ... O */
    0,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,    
    /* P ...             Z [ \ ] ^ _ */
    1,1,1,1,1,1,1,1, 1,1,1,0,0,0,0,1,
    /* ` a, ... o */
    0,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,    
    /* p ...             z { | } ~ DEL */
    1,1,1,1,1,1,1,1, 1,1,1,0,0,0,0,0,
];

pub fn token_char(c: u8) -> bool {
    (c as usize) < 0x80 && SEXP_TOKEN_CHARS[c as usize] != 0
}

#[macro_export]
macro_rules! die {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        process::exit(1);
    }};
}

pub fn werror<T: fmt::Display>(message: T) {
    eprintln!("{}", message);
}

pub fn xalloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 1).unwrap_or_else(|_| {
        eprintln!("Invalid allocation size");
        process::abort();
    });

    unsafe {
        let ptr = alloc::alloc(layout);
        if ptr.is_null() {
            eprintln!("Virtual memory exhausted");
            process::abort();
        }
        ptr
    }
}