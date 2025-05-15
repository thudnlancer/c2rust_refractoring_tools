use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_long;
use std::ptr;
use std::mem;
use std::fmt;

#[derive(Debug, Clone)]
struct Node {
    flags: u32,
    numbr: f64,
    stptr: Option<CString>,
    stlen: usize,
    vname: Option<CString>,
    parent_array: Option<Box<Node>>,
    xarray: Option<Box<Node>>,
    buckets: Option<Vec<Bucket>>,
    array_size: usize,
    table_size: usize,
}

#[derive(Debug, Clone)]
struct Bucket {
    ainum: Vec<c_long>,
    aivalue: Vec<Node>,
    aicount: usize,
    ainext: Option<Box<Bucket>>,
}

static mut INT_CHAIN_MAX: usize = 2;

const NUMINT: u32 = 0x1;
const NUMCUR: u32 = 0x2;
const STRCUR: u32 = 0x4;
const USER_INPUT: u32 = 0x8;
const STRING: u32 = 0x10;
const NUMBER: u32 = 0x20;
const XARRAY: u32 = 0x40;
const ARRAYMAXED: u32 = 0x80;

const INT32_MAX: f64 = 2147483647.0;
const INT32_MIN: f64 = -2147483648.0;

fn int_array_init(symbol: Option<&mut Node>, subs: Option<&Node>) -> Option<&'static Node> {
    if symbol.is_none() {
        // First time initialization
        let newval = unsafe { libc::getenv(CString::new("INT_CHAIN_MAX").unwrap().as_ptr()) };
        if !newval.is_null() {
            let newval_str = unsafe { std::ffi::CStr::from_ptr(newval).to_string_lossy() };
            if let Ok(val) = newval_str.parse::<usize>() {
                if val > 0 {
                    unsafe { INT_CHAIN_MAX = val };
                }
            }
        }
    } else {
        null_array(symbol.unwrap());
    }
    Some(&success_node())
}

fn standard_integer_string(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    if s == "0" {
        return true;
    }
    
    let mut chars = s.chars();
    let first_char = chars.next().unwrap();
    
    let mut start = 0;
    if first_char == '-' {
        if s.len() == 1 {
            return false;
        }
        start = 1;
    }
    
    let s = &s[start..];
    if s.is_empty() || !s.chars().next().unwrap().is_digit(10) || s.chars().next().unwrap() == '0' {
        return false;
    }
    
    s.chars().all(|c| c.is_digit(10))
}

fn is_integer(symbol: &Node, subs: &Node) -> Option<&'static Node> {
    if (subs.flags & NUMINT) != 0 {
        return Some(&success_node());
    }

    if subs == &Nnull_string() || do_mpfr() {
        return None;
    }

    let d = subs.numbr;
    if d <= INT32_MAX && d >= INT32_MIN && d == d.trunc() {
        if (subs.flags & STRCUR) == 0 || {
            let stptr = subs.stptr.as_ref().unwrap().to_str().unwrap();
            standard_integer_string(stptr)
        } {
            // In Rust we can't modify subs directly, would need &mut
            // For now return success if condition matches
            return Some(&success_node());
        }
    }
    None
}

fn int_lookup(symbol: &mut Node, subs: &Node) -> Option<&mut Node> {
    if let Some(result) = is_integer(symbol, subs) {
        return Some(result);
    }

    let k = subs.numbr as c_long;
    let hash1 = int_hash(k as u32, symbol.array_size as u32) as usize;

    if symbol.buckets.is_none() {
        grow_int_table(symbol);
    }

    if let Some(buckets) = &mut symbol.buckets {
        if let Some(bucket) = &mut buckets[hash1] {
            for i in 0..bucket.aicount {
                if bucket.ainum[i] == k {
                    return Some(&mut bucket.aivalue[i]);
                }
            }
        }
    }

    // Not found, insert it
    symbol.table_size += 1;
    
    let size = if let Some(xn) = &symbol.xarray {
        symbol.table_size - xn.table_size
    } else {
        symbol.table_size
    };

    if (symbol.flags & ARRAYMAXED) == 0 && (size / symbol.array_size) > unsafe { INT_CHAIN_MAX } {
        grow_int_table(symbol);
        let hash1 = int_hash(k as u32, symbol.array_size as u32) as usize;
    }

    int_insert(symbol, k, hash1 as u32)
}

fn int_hash(k: u32, hsize: u32) -> u32 {
    let mut k = k;
    k ^= k << 3;
    k += k >> 5;
    k ^= k << 4;
    k += k >> 17;
    k ^= k << 25;
    k += k >> 6;
    k % hsize
}

fn int_insert(symbol: &mut Node, k: c_long, hash1: u32) -> Option<&mut Node> {
    let hash1 = hash1 as usize;
    if symbol.buckets.is_none() {
        symbol.buckets = Some(vec![None; symbol.array_size]);
    }

    let buckets = symbol.buckets.as_mut().unwrap();
    if buckets[hash1].is_none() || buckets[hash1].as_ref().unwrap().aicount == 2 {
        let mut new_bucket = Bucket {
            ainum: vec![0; 2],
            aivalue: vec![Node::default(), Node::default()],
            aicount: 0,
            ainext: None,
        };
        new_bucket.ainext = buckets[hash1].take().map(Box::new);
        buckets[hash1] = Some(new_bucket);
    }

    let bucket = buckets[hash1].as_mut().unwrap();
    let i = bucket.aicount;
    bucket.ainum[i] = k;
    bucket.aivalue[i] = new_array_element();
    bucket.aicount += 1;
    Some(&mut bucket.aivalue[i])
}

fn grow_int_table(symbol: &mut Node) {
    const SIZES: [usize; 21] = [
        13, 127, 1021, 8191, 16381, 32749, 65497,
        131101, 262147, 524309, 1048583, 2097169,
        4194319, 8388617, 16777259, 33554467,
        67108879, 134217757, 268435459, 536870923,
        1073741827
    ];

    let oldsize = symbol.array_size;
    let mut newsize = oldsize;

    for &size in &SIZES {
        if oldsize < size {
            newsize = size;
            break;
        }
    }

    if newsize == oldsize {
        symbol.flags |= ARRAYMAXED;
        return;
    }

    let mut new_buckets: Vec<Option<Bucket>> = vec![None; newsize];
    if let Some(old_buckets) = symbol.buckets.take() {
        for bucket_opt in old_buckets {
            if let Some(bucket) = bucket_opt {
                for i in 0..bucket.aicount {
                    let num = bucket.ainum[i];
                    let hash1 = int_hash(num as u32, newsize as u32) as usize;
                    // Need to implement proper insertion here
                }
            }
        }
    }

    symbol.array_size = newsize;
    symbol.buckets = Some(new_buckets);
}

// Helper functions
fn success_node() -> Node {
    Node::default()
}

fn Nnull_string() -> Node {
    Node::default()
}

fn do_mpfr() -> bool {
    false
}

fn null_array(symbol: &mut Node) {
    symbol.table_size = 0;
    symbol.buckets = None;
    symbol.xarray = None;
}

fn new_array_element() -> Node {
    Node::default()
}

impl Default for Node {
    fn default() -> Self {
        Node {
            flags: 0,
            numbr: 0.0,
            stptr: None,
            stlen: 0,
            vname: None,
            parent_array: None,
            xarray: None,
            buckets: None,
            array_size: 0,
            table_size: 0,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags &&
        self.numbr == other.numbr &&
        self.stptr == other.stptr &&
        self.stlen == other.stlen &&
        self.vname == other.vname &&
        self.array_size == other.array_size &&
        self.table_size == other.table_size
    }
}