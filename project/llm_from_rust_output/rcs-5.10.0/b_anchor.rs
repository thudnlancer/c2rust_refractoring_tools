use std::cmp::Ordering;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KwSub {
    Kv,
    Kvl,
    K,
    V,
    O,
    B,
}

#[derive(Debug, Clone, Copy)]
pub struct Cbuf {
    pub string: *const c_char,
    pub size: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Tinysym {
    pub len: u8,
    pub bytes: &'static [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct PoolFound {
    pub i: i32,
    pub sym: &'static Tinysym,
}

pub static KS_REVNO: &[u8] = b"revision number\0";
pub static PROG_DIFF: &[u8] = b"/usr/bin/diff\0";
pub static PROG_DIFF3: &[u8] = b"/usr/bin/diff3\0";
pub static DIFF_FLAGS: &[u8] = b"-an\0";
pub static EQUAL_LINE: &[u8] = b"=============================================================================\n\0";

pub static TINY_CIKLOG: Tinysym = Tinysym {
    len: 22,
    bytes: b"checked in with -k by \0",
};
pub static TINY_ACCESS: Tinysym = Tinysym {
    len: 6,
    bytes: b"access\0",
};
pub static TINY_AUTHOR: Tinysym = Tinysym {
    len: 6,
    bytes: b"author\0",
};
pub static TINY_BRANCH: Tinysym = Tinysym {
    len: 6,
    bytes: b"branch\0",
};
pub static TINY_BRANCHES: Tinysym = Tinysym {
    len: 8,
    bytes: b"branches\0",
};
pub static TINY_COMMENT: Tinysym = Tinysym {
    len: 7,
    bytes: b"comment\0",
};
pub static TINY_COMMITID: Tinysym = Tinysym {
    len: 8,
    bytes: b"commitid\0",
};
pub static TINY_DATE: Tinysym = Tinysym {
    len: 4,
    bytes: b"date\0",
};
pub static TINY_DESC: Tinysym = Tinysym {
    len: 4,
    bytes: b"desc\0",
};
pub static TINY_EXPAND: Tinysym = Tinysym {
    len: 6,
    bytes: b"expand\0",
};
pub static TINY_HEAD: Tinysym = Tinysym {
    len: 4,
    bytes: b"head\0",
};
pub static TINY_INTEGRITY: Tinysym = Tinysym {
    len: 9,
    bytes: b"integrity\0",
};
pub static TINY_LOCKS: Tinysym = Tinysym {
    len: 5,
    bytes: b"locks\0",
};
pub static TINY_LOG: Tinysym = Tinysym {
    len: 3,
    bytes: b"log\0",
};
pub static TINY_NEXT: Tinysym = Tinysym {
    len: 4,
    bytes: b"next\0",
};
pub static TINY_STATE: Tinysym = Tinysym {
    len: 5,
    bytes: b"state\0",
};
pub static TINY_STRICT: Tinysym = Tinysym {
    len: 6,
    bytes: b"strict\0",
};
pub static TINY_SYMBOLS: Tinysym = Tinysym {
    len: 7,
    bytes: b"symbols\0",
};
pub static TINY_TEXT: Tinysym = Tinysym {
    len: 4,
    bytes: b"text\0",
};

const KWSUB_POOL: &[(&str, KwSub)] = &[
    ("kv", KwSub::Kv),
    ("kvl", KwSub::Kvl),
    ("k", KwSub::K),
    ("v", KwSub::V),
    ("o", KwSub::O),
    ("b", KwSub::B),
];

const KEYWORD_POOL: &[&str] = &[
    "Author", "Date", "Header", "Id", "Locker", "Log", "Name", "RCSfile", "Revision", "Source", "State",
];

pub fn looking_at(sym: &Tinysym, start: &[u8]) -> bool {
    start.len() >= sym.len as usize && &start[..sym.len as usize] == &sym.bytes[..sym.len as usize]
}

pub fn recognize_kwsub(x: &Cbuf) -> Option<KwSub> {
    let s = unsafe { CStr::from_ptr(x.string) };
    let bytes = s.to_bytes();
    if bytes.len() != x.size {
        return None;
    }

    KWSUB_POOL.iter()
        .find(|(s, _)| s.as_bytes() == bytes)
        .map(|(_, kw)| *kw)
}

pub fn str2expmode(s: &[u8]) -> Option<KwSub> {
    recognize_kwsub(&Cbuf {
        string: s.as_ptr() as *const c_char,
        size: s.len(),
    })
}

pub fn kwsub_string(kw: KwSub) -> Option<&'static str> {
    KWSUB_POOL.iter()
        .find(|(_, k)| *k == kw)
        .map(|(s, _)| *s)
}

pub fn recognize_keyword(string: &[u8]) -> Option<(PoolFound, usize)> {
    let delim_pos = string.iter()
        .position(|&c| c == b'$' || c == b':')?;

    let keyword = &string[..delim_pos];
    let found = KEYWORD_POOL.iter()
        .enumerate()
        .find(|(_, s)| s.as_bytes() == keyword)?;

    Some((
        PoolFound {
            i: found.0 as i32,
            sym: &Tinysym {
                len: found.1.len() as u8,
                bytes: found.1.as_bytes(),
            },
        },
        delim_pos,
    ))
}