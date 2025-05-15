use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

#[cfg(target_os = "windows")]
use winapi::um::winnls::GetACP;

#[cfg(target_os = "macos")]
use libc::{uselocale, MB_CUR_MAX};

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
use libc::nl_langinfo;
#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
use libc::CODESET;

struct TableEntry {
    alias: &'static str,
    canonical: &'static str,
}

struct LocaleEntry {
    locale: &'static str,
    canonical: &'static str,
}

// Platform-specific alias tables
#[cfg(target_os = "freebsd")]
const ALIAS_TABLE: &[TableEntry] = &[
    TableEntry { alias: "Big5", canonical: "BIG5" },
    TableEntry { alias: "C", canonical: "ASCII" },
    TableEntry { alias: "ISO8859-1", canonical: "ISO-8859-1" },
    // ... other FreeBSD entries
];

// ... other platform-specific tables

// Platform-specific locale tables
#[cfg(target_os = "freebsd")]
const LOCALE_TABLE: &[LocaleEntry] = &[
    LocaleEntry { locale: "cs_CZ.ISO_8859-2", canonical: "ISO-8859-2" },
    // ... other FreeBSD entries
];

// ... other platform-specific tables

pub fn locale_charset() -> &'static str {
    let codeset = if cfg!(any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    )) {
        unsafe {
            let codeset_ptr = nl_langinfo(CODESET);
            if codeset_ptr.is_null() {
                ""
            } else {
                CStr::from_ptr(codeset_ptr).to_str().unwrap_or("")
            }
        }
    } else if cfg!(target_os = "windows") {
        let mut buf = [0u8; 13];
        let cp = unsafe { GetACP() };
        let s = format!("CP{}", cp);
        buf[..s.len()].copy_from_slice(s.as_bytes());
        unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) }
            .to_str()
            .unwrap_or("")
    } else {
        ""
    };

    // Resolve alias
    let resolved = if !codeset.is_empty() {
        if let Some(entry) = ALIAS_TABLE.iter().find(|e| e.alias == codeset) {
            entry.canonical
        } else {
            codeset
        }
    } else {
        // Try to get from environment
        let locale = env::var("LC_ALL")
            .or_else(|_| env::var("LC_CTYPE"))
            .or_else(|_| env::var("LANG"))
            .unwrap_or_default();

        if let Some(entry) = LOCALE_TABLE.iter().find(|e| e.locale == locale) {
            entry.canonical
        } else {
            // Default fallbacks
            if cfg!(any(target_os = "macos", target_os = "ios")) {
                "UTF-8"
            } else {
                "ASCII"
            }
        }
    };

    // Special case for Darwin
    #[cfg(target_os = "macos")]
    {
        if resolved == "UTF-8" {
            unsafe {
                let max = MB_CUR_MAX(uselocale(ptr::null()));
                if max <= 1 {
                    return "ASCII";
                }
            }
        }
    }

    resolved
}