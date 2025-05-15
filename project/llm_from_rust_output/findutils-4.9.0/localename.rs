use std::ffi::{CStr, CString};
use std::sync::Mutex;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref STRUNIQ_HASH_TABLE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn string_hash(s: &str) -> usize {
    let mut h: usize = 0;
    for c in s.chars() {
        h = (c as usize).wrapping_add(
            h << 9 | h >> (std::mem::size_of::<usize>() * 8 - 9)
        );
    }
    h
}

fn struniq(string: &str) -> &'static str {
    let hashcode = string_hash(string);
    let slot = hashcode % 257;
    
    let mut table = STRUNIQ_HASH_TABLE.lock().unwrap();
    
    if let Some(existing) = table.get(string) {
        return existing;
    }
    
    let new_string = string.to_string();
    let boxed = Box::leak(new_string.into_boxed_str());
    table.insert(boxed.to_string(), boxed);
    
    boxed
}

fn gl_locale_name_thread_unsafe(category: i32, categoryname: &str) -> Option<&'static str> {
    // Note: Actual thread locale implementation would require platform-specific unsafe code
    // This is a simplified safe version
    None
}

pub fn gl_locale_name_thread(category: i32, categoryname: &str) -> Option<&'static str> {
    gl_locale_name_thread_unsafe(category, categoryname).map(|name| struniq(name))
}

pub fn gl_locale_name_posix(category: i32, _categoryname: &str) -> Option<&'static str> {
    // Note: Actual POSIX locale implementation would require unsafe code
    // This is a simplified safe version
    None
}

pub fn gl_locale_name_environ(category: i32, categoryname: &str) -> Option<&'static str> {
    if let Ok(lc_all) = std::env::var("LC_ALL") {
        if !lc_all.is_empty() {
            return Some(struniq(&lc_all));
        }
    }
    
    if let Ok(category_val) = std::env::var(categoryname) {
        if !category_val.is_empty() {
            return Some(struniq(&category_val));
        }
    }
    
    if let Ok(lang) = std::env::var("LANG") {
        if !lang.is_empty() {
            return Some(struniq(&lang));
        }
    }
    
    None
}

pub fn gl_locale_name_default() -> &'static str {
    "C"
}

pub fn gl_locale_name(category: i32, categoryname: &str) -> &'static str {
    gl_locale_name_thread(category, categoryname)
        .or_else(|| gl_locale_name_posix(category, categoryname))
        .unwrap_or_else(gl_locale_name_default)
}