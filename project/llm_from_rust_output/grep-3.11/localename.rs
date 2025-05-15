use std::ffi::{CStr, CString};
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;
use std::ptr;

type LocaleT = *mut libc::__locale_struct;
type NlItem = libc::c_int;

static STRUNIQ_HASH_TABLE: OnceLock<Mutex<HashMap<u64, CString>>> = OnceLock::new();
static STRUNIQ_LOCK: Mutex<()> = Mutex::new(());

fn string_hash(s: &CStr) -> u64 {
    let mut h: u64 = 0;
    for &c in s.to_bytes() {
        h = c as u64 + ((h << 9) | (h >> (u64::BITS - 9)));
    }
    h
}

fn struniq(string: &CStr) -> &'static CStr {
    let hashcode = string_hash(string);
    let slot = hashcode % 257;

    let mut table = STRUNIQ_HASH_TABLE.get_or_init(|| Mutex::new(HashMap::new()));
    let _guard = STRUNIQ_LOCK.lock().unwrap();

    if let Some(existing) = table.get(&hashcode) {
        return existing.as_c_str();
    }

    let new_entry = CString::new(string.to_bytes()).unwrap();
    let ptr = new_entry.as_ptr();
    table.insert(hashcode, new_entry);

    unsafe { CStr::from_ptr(ptr) }
}

fn gl_locale_name_thread_unsafe(category: libc::c_int, categoryname: &CStr) -> Option<&'static CStr> {
    let thread_locale = unsafe { libc::uselocale(ptr::null_mut()) };
    if thread_locale != (-1_isize) as LocaleT {
        let item = (category << 16) | (-1_i32 & 0xffff);
        let name_ptr = unsafe { libc::nl_langinfo(item) };
        let name = unsafe { CStr::from_ptr(name_ptr) };

        if name.to_bytes().is_empty() {
            let thread_name_ptr = unsafe { (*thread_locale).__names[category as usize] };
            Some(unsafe { CStr::from_ptr(thread_name_ptr) })
        } else {
            Some(name)
        }
    } else {
        None
    }
}

pub fn gl_locale_name_thread(category: libc::c_int, categoryname: &CStr) -> Option<&'static CStr> {
    gl_locale_name_thread_unsafe(category, categoryname).map(|name| struniq(name))
}

pub fn gl_locale_name_posix(category: libc::c_int, _categoryname: &CStr) -> Option<&'static CStr> {
    let locname_ptr = unsafe { libc::setlocale_null(category) };
    if locname_ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(locname_ptr) })
    }
}

pub fn gl_locale_name_environ(category: libc::c_int, categoryname: &CStr) -> Option<&'static CStr> {
    let getenv = |name: &CStr| {
        let ptr = unsafe { libc::getenv(name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            let s = unsafe { CStr::from_ptr(ptr) };
            if s.to_bytes().is_empty() {
                None
            } else {
                Some(s)
            }
        }
    };

    if let Some(retval) = getenv(c_str!("LC_ALL")) {
        return Some(retval);
    }
    if let Some(retval) = getenv(categoryname) {
        return Some(retval);
    }
    if let Some(retval) = getenv(c_str!("LANG")) {
        return Some(retval);
    }
    None
}

pub fn gl_locale_name_default() -> &'static CStr {
    c_str!("C")
}

pub fn gl_locale_name(category: libc::c_int, categoryname: &CStr) -> &'static CStr {
    gl_locale_name_thread(category, categoryname)
        .or_else(|| gl_locale_name_posix(category, categoryname))
        .unwrap_or_else(gl_locale_name_default)
}

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        unsafe { CStr::from_ptr(concat!($s, "\0").as_ptr() as *const libc::c_char) }
    };
}