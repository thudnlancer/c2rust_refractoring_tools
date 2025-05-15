/* b-anchor.rs --- constant data and their lookup funcs

   Copyright (C) 2010-2020 Thien-Thi Nguyen

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

pub const KS_REVNO: &str = "revision number";

pub const PROG_DIFF: &str = DIFF;
pub const PROG_DIFF3: &str = DIFF3;
pub const DIFF_FLAGS: &str = DIFFFLAGS;

pub const EQUAL_LINE: &str = "=============================================================================\n";

#[derive(Debug)]
pub struct TinySym {
    pub len: usize,
    pub bytes: &'static str,
}

macro_rules! tiny_init {
    ($x:expr) => {
        TinySym {
            len: $x.len(),
            bytes: $x,
        }
    };
}

macro_rules! tinyk {
    ($x:ident) => {
        pub const $x: TinySym = tiny_init!(stringify!($x));
    };
}

// For libgnurcs.so, these should be coalesced into a pool.
pub const CIKLOG: TinySym = tiny_init!("checked in with -k by ");
tinyk!(ACCESS);
tinyk!(AUTHOR);
tinyk!(BRANCH);
tinyk!(BRANCHES);
tinyk!(COMMENT);
tinyk!(COMMITID);
tinyk!(DATE);
tinyk!(DESC);
tinyk!(EXPAND);
tinyk!(HEAD);
tinyk!(INTEGRITY);
tinyk!(LOCKS);
tinyk!(LOG);
tinyk!(NEXT);
tinyk!(STATE);
tinyk!(STRICT);
tinyk!(SYMBOLS);
tinyk!(TEXT);

pub fn looking_at(sym: &TinySym, start: &str) -> bool {
    sym.len == start.len() && sym.bytes == start
}

const KWSUB_POOL: &[u8] = &[
    6, // count
    2, b'k', b'v', 0,
    3, b'k', b'v', b'l', 0,
    1, b'k', 0,
    1, b'v', 0,
    1, b'o', 0,
    1, b'b', 0,
];

const KEYWORD_POOL: &[u8] = &[
    11, // count
    6, b'A', b'u', b't', b'h', b'o', b'r', 0,
    4, b'D', b'a', b't', b'e', 0,
    6, b'H', b'e', b'a', b'd', b'e', b'r', 0,
    2, b'I', b'd', 0,
    6, b'L', b'o', b'c', b'k', b'e', b'r', 0,
    3, b'L', b'o', b'g', 0,
    4, b'N', b'a', b'm', b'e', 0,
    7, b'R', b'C', b'S', b'f', b'i', b'l', b'e', 0,
    8, b'R', b'e', b'v', b'i', b's', b'i', b'o', b'n', 0,
    6, b'S', b'o', b'u', b'r', b'c', b'e', 0,
    5, b'S', b't', b'a', b't', b'e', 0,
];

#[derive(Debug)]
pub struct Cbuf<'a> {
    pub string: &'a str,
    pub size: usize,
}

#[derive(Debug)]
pub struct PoolFound<'a> {
    pub i: usize,
    pub sym: TinySym,
}

fn pool_lookup(pool: &[u8], x: &Cbuf) -> Option<PoolFound> {
    let mut p = &pool[1..];
    let count = pool[0] as usize;

    for i in 0..count {
        let symlen = p[0] as usize;
        let sym_bytes = &p[1..1 + symlen];
        let sym_str = std::str::from_utf8(sym_bytes).ok()?;

        if x.size == symlen && x.string == sym_str {
            return Some(PoolFound {
                i,
                sym: TinySym {
                    len: symlen,
                    bytes: sym_str,
                },
            });
        }
        p = &p[1 + symlen + 1..];
    }
    None
}

pub fn recognize_kwsub(x: &Cbuf) -> Option<usize> {
    pool_lookup(KWSUB_POOL, x).map(|found| found.i)
}

pub fn str2expmode(s: &str) -> Option<usize> {
    recognize_kwsub(&Cbuf {
        string: s,
        size: s.len(),
    })
}

pub fn kwsub_string(i: usize) -> Option<&'static str> {
    let mut count = KWSUB_POOL[0] as usize;
    let mut p = &KWSUB_POOL[1..];
    let mut current_i = i;

    while current_i > 0 && count > 0 {
        let symlen = p[0] as usize;
        p = &p[1 + symlen + 1..];
        count -= 1;
        current_i -= 1;
    }

    if current_i == 0 && count > 0 {
        let symlen = p[0] as usize;
        let sym_bytes = &p[1..1 + symlen];
        std::str::from_utf8(sym_bytes).ok()
    } else {
        None
    }
}

pub fn recognize_keyword(string: &str, kdelim: char, vdelim: char) -> Option<PoolFound> {
    let limit = string.find(|c| c == kdelim || c == vdelim)?;
    let delim = string.chars().nth(limit)?;
    
    if delim == kdelim || delim == vdelim {
        pool_lookup(KEYWORD_POOL, &Cbuf {
            string: &string[..limit],
            size: limit,
        })
    } else {
        None
    }
}

/* b-anchor.rs ends here */