use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[cfg(target_os = "windows")]
use winapi::um::winnls::GetACP;

#[cfg(target_os = "macos")]
use core_foundation::locale::CFLocaleCopyCurrent;
#[cfg(target_os = "macos")]
use core_foundation::string::CFStringGetCString;
#[cfg(target_os = "macos")]
use core_foundation::string::CFStringRef;

struct TableEntry {
    alias: &'static str,
    canonical: &'static str,
}

struct LocaleTableEntry {
    locale: &'static str,
    canonical: &'static str,
}

// Platform-specific alias tables
#[cfg(any(
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "macos",
    target_os = "linux",
    target_os = "windows",
    target_os = "dragonfly"
))]
const ALIAS_TABLE: &[TableEntry] = &[
    // FreeBSD entries
    TableEntry {
        alias: "Big5",
        canonical: "BIG5",
    },
    TableEntry {
        alias: "C",
        canonical: "ASCII",
    },
    TableEntry {
        alias: "ISO8859-1",
        canonical: "ISO-8859-1",
    },
    // Add other platform-specific entries here
];

// Platform-specific locale tables
#[cfg(any(
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "macos",
    target_os = "linux",
    target_os = "windows",
    target_os = "dragonfly"
))]
const LOCALE_TABLE: &[LocaleTableEntry] = &[
    // FreeBSD entries
    LocaleTableEntry {
        locale: "cs_CZ.ISO_8859-2",
        canonical: "ISO-8859-2",
    },
    // Add other platform-specific entries here
];

fn locale_charset() -> &'static str {
    let codeset = if cfg!(unix) {
        unsafe {
            let codeset = libc::nl_langinfo(libc::CODESET);
            if !codeset.is_null() {
                CStr::from_ptr(codeset).to_str().unwrap_or("")
            } else {
                ""
            }
        }
    } else if cfg!(windows) {
        let cp = unsafe { GetACP() };
        format!("CP{}", cp).leak()
    } else {
        ""
    };

    // Resolve alias
    if !codeset.is_empty() {
        if let Some(entry) = ALIAS_TABLE.iter().find(|e| e.alias == codeset) {
            return entry.canonical;
        }

        // Special cases
        if cfg!(target_os = "macos") || cfg!(target_os = "haiku") {
            return "UTF-8";
        }

        if codeset == "UTF-8" {
            return "UTF-8";
        }
    }

    // Fallback to locale table
    let locale = env::var("LC_ALL")
        .or_else(|_| env::var("LC_CTYPE"))
        .or_else(|_| env::var("LANG"))
        .unwrap_or_default();

    if let Some(entry) = LOCALE_TABLE.iter().find(|e| e.locale == locale) {
        return entry.canonical;
    }

    // Final fallbacks
    if cfg!(target_os = "macos") || cfg!(target_os = "haiku") {
        "UTF-8"
    } else if codeset.is_empty() {
        "ASCII"
    } else {
        codeset
    }
}

#[no_mangle]
pub extern "C" fn locale_charset_c() -> *const c_char {
    static mut RESULT: *const c_char = ptr::null();
    
    let charset = locale_charset();
    let c_str = CString::new(charset).unwrap();
    
    unsafe {
        RESULT = c_str.into_raw();
        RESULT
    }
}