use std::collections::HashMap;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use std::mem;

lazy_static::lazy_static! {
    static ref PREFIX_TABLE: [HashMap<String, ()>; 2] = [HashMap::new(), HashMap::new()];
}

fn hash_string_hasher(name: &CStr, n_buckets: usize) -> usize {
    let mut hash: usize = 0;
    for &byte in name.to_bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
    }
    hash % n_buckets
}

fn hash_string_compare(name1: &CStr, name2: &CStr) -> bool {
    name1 == name2
}

fn hash_string_insert_prefix(
    table: &mut HashMap<String, ()>,
    string: &str,
    len: usize,
    return_prefix: &mut Option<String>,
) -> bool {
    let s = if len > 0 {
        string[..len].to_string()
    } else {
        string.to_string()
    };

    match table.entry(s.clone()) {
        std::collections::hash_map::Entry::Occupied(_) => {
            false
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(());
            *return_prefix = Some(s);
            true
        }
    }
}

pub fn removed_prefixes_p() -> bool {
    PREFIX_TABLE[0].len() != 0 || PREFIX_TABLE[1].len() != 0
}

pub fn safer_name_suffix(
    file_name: &str,
    link_target: bool,
    absolute_names: bool,
) -> String {
    let p = if absolute_names {
        file_name
    } else {
        let mut prefix_len = 0;
        let mut chars = file_name.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == '.' && chars[i + 1] == '.' {
                prefix_len = i + 2;
            }
            while i < chars.len() && chars[i] != '/' {
                i += 1;
            }
            if i < chars.len() {
                i += 1;
            }
        }
        while prefix_len < chars.len() && chars[prefix_len] == '/' {
            prefix_len += 1;
        }
        &file_name[prefix_len..]
    };

    if p.is_empty() {
        if p == file_name {
            // Warning logic would go here
        }
        ".".to_string()
    } else {
        if !absolute_names && !p.is_empty() {
            let mut prefix = None;
            let mut table = PREFIX_TABLE[link_target as usize].clone();
            let inserted = hash_string_insert_prefix(
                &mut table,
                file_name,
                file_name.len() - p.len(),
                &mut prefix,
            );
            if inserted {
                // Warning logic would go here
            }
        }
        p.to_string()
    }
}