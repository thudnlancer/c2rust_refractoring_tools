use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Once;
use std::collections::HashMap;
use std::sync::Mutex;
use libc::{c_char, c_int, c_void};

type NlItem = c_int;
type LocaleT = *mut libc::__locale_struct;

#[repr(C)]
struct LocaleStruct {
    __locales: [*mut libc::__locale_data; 13],
    __ctype_b: *const libc::c_ushort,
    __ctype_tolower: *const libc::c_int,
    __ctype_toupper: *const libc::c_int,
    __names: [*const c_char; 13],
}

lazy_static! {
    static ref STRUNIQ_HASH_TABLE: Mutex<HashMap<String, CString>> = Mutex::new(HashMap::new());
}

fn string_hash(s: &str) -> usize {
    let mut h: usize = 0;
    for c in s.chars() {
        h = (c as usize).wrapping_add(h << 9 | h >> (usize::BITS as usize - 9));
    }
    h
}

fn struniq(string: &str) -> &'static CStr {
    let hashcode = string_hash(string);
    let slot = hashcode % 257;
    let mut table = STRUNIQ_HASH_TABLE.lock().unwrap();

    if let Some(existing) = table.get(string) {
        return existing.as_c_str();
    }

    let new_entry = CString::new(string).unwrap_or_default();
    let ptr = new_entry.as_ptr();
    table.insert(string.to_string(), new_entry);
    
    unsafe { CStr::from_ptr(ptr) }
}

fn gl_locale_name_thread_unsafe(category: c_int, categoryname: &CStr) -> Option<&'static CStr> {
    unsafe {
        let thread_locale = libc::uselocale(ptr::null_mut());
        if thread_locale != (-1_isize) as LocaleT {
            let name_ptr = libc::nl_langinfo((category << 16) | (-1_i32) & 0xffff);
            let name = CStr::from_ptr(name_ptr);
            if name.to_bytes().is_empty() {
                let locale_name = (*thread_locale).__names[category as usize];
                return Some(CStr::from_ptr(locale_name));
            }
            return Some(name);
        }
        None
    }
}

pub fn gl_locale_name_thread(category: c_int, categoryname: &CStr) -> Option<&'static CStr> {
    gl_locale_name_thread_unsafe(category, categoryname)
        .map(|name| struniq(name.to_str().unwrap_or("")))
}

pub fn gl_locale_name_posix(category: c_int, _categoryname: &CStr) -> Option<&'static CStr> {
    unsafe {
        let locname = libc::setlocale_null(category);
        if locname.is_null() {
            None
        } else {
            Some(CStr::from_ptr(locname))
        }
    }
}

pub fn gl_locale_name_environ(category: c_int, categoryname: &CStr) -> Option<&'static CStr> {
    unsafe {
        let check_var = |var: &str| {
            let var_c = CString::new(var).unwrap();
            let val = libc::getenv(var_c.as_ptr());
            if !val.is_null() && !CStr::from_ptr(val).to_bytes().is_empty() {
                Some(CStr::from_ptr(val))
            } else {
                None
            }
        };

        check_var("LC_ALL")
            .or_else(|| check_var(categoryname.to_str().unwrap()))
            .or_else(|| check_var("LANG"))
    }
}

pub fn gl_locale_name_default() -> &'static CStr {
    unsafe { CStr::from_bytes_with_nul_unchecked(b"C\0") }
}

pub fn gl_locale_name(category: c_int, categoryname: &CStr) -> &'static CStr {
    gl_locale_name_thread(category, categoryname)
        .or_else(|| gl_locale_name_posix(category, categoryname))
        .unwrap_or_else(gl_locale_name_default)
}