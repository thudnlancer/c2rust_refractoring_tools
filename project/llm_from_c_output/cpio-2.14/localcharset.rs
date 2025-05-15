use std::env;
use std::ffi::CString;
use std::ptr;

#[cfg(target_os = "windows")]
use winapi::um::winnls::GetACP;

#[cfg(target_os = "macos")]
use libc::{uselocale, MB_CUR_MAX};

#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd", target_os = "dragonfly"))]
use libc::nl_langinfo;
use libc::CODESET;

struct TableEntry {
    alias: &'static str,
    canonical: &'static str,
}

struct LocaleEntry {
    locale: &'static str,
    canonical: &'static str,
}

static ALIAS_TABLE: &[TableEntry] = &[
    // Include all the alias mappings from the original C code
    // Example:
    TableEntry { alias: "ISO8859-1", canonical: "ISO-8859-1" },
    // ... rest of the entries
];

static LOCALE_TABLE: &[LocaleEntry] = &[
    // Include all the locale mappings from the original C code
    // Example:
    LocaleEntry { locale: "cs_CZ.ISO_8859-2", canonical: "ISO-8859-2" },
    // ... rest of the entries
];

pub fn locale_charset() -> &'static str {
    let codeset = get_codeset();

    // Resolve alias
    if let Some(canonical) = resolve_alias(codeset) {
        return canonical;
    }

    // Default cases
    if codeset.is_empty() {
        return "ASCII";
    }

    #[cfg(target_os = "macos")]
    {
        if codeset == "UTF-8" {
            unsafe {
                let loc = uselocale(ptr::null_mut());
                if libc::MB_CUR_MAX(loc) <= 1 {
                    return "ASCII";
                }
            }
        }
    }

    codeset
}

fn get_codeset() -> &'static str {
    #[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd", target_os = "dragonfly"))]
    {
        unsafe {
            let codeset_cstr = nl_langinfo(CODESET);
            if !codeset_cstr.is_null() {
                let codeset = std::ffi::CStr::from_ptr(codeset_cstr).to_str().unwrap_or("");
                
                #[cfg(target_os = "cygwin")]
                {
                    if codeset == "US-ASCII" {
                        if let Some(enc) = get_cygwin_encoding() {
                            return enc;
                        }
                    }
                }
                
                return codeset;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(cp) = get_windows_codepage() {
            return cp;
        }
    }

    // Fallback for other platforms
    get_locale_from_env()
}

#[cfg(target_os = "cygwin")]
fn get_cygwin_encoding() -> Option<&'static str> {
    let locale = env::var("LC_ALL")
        .or_else(|_| env::var("LC_CTYPE"))
        .or_else(|_| env::var("LANG"))
        .ok()?;

    if let Some(dot) = locale.find('.') {
        let encoding = &locale[dot + 1..];
        if let Some(at) = encoding.find('@') {
            return Some(&encoding[..at]);
        }
        return Some(encoding);
    }

    unsafe {
        let cp = GetACP();
        Some(match cp {
            65001 => "UTF-8",
            _ => {
                static mut BUF: [u8; 12] = [0; 12];
                let s = format!("CP{}", cp);
                BUF[..s.len()].copy_from_slice(s.as_bytes());
                std::str::from_utf8_unchecked(&BUF[..s.len()])
            }
        })
    }
}

#[cfg(target_os = "windows")]
fn get_windows_codepage() -> Option<&'static str> {
    unsafe {
        let cp = GetACP();
        Some(match cp {
            65001 => "UTF-8",
            _ => {
                static mut BUF: [u8; 12] = [0; 12];
                let s = format!("CP{}", cp);
                BUF[..s.len()].copy_from_slice(s.as_bytes());
                std::str::from_utf8_unchecked(&BUF[..s.len()])
            }
        })
    }
}

fn get_locale_from_env() -> &'static str {
    let locale = env::var("LC_ALL")
        .or_else(|_| env::var("LC_CTYPE"))
        .or_else(|_| env::var("LANG"))
        .unwrap_or_default();

    if locale.is_empty() {
        return "ASCII";
    }

    if let Some(entry) = LOCALE_TABLE.iter().find(|e| e.locale == locale) {
        return entry.canonical;
    }

    #[cfg(any(target_os = "macos", target_os = "haiku"))]
    {
        return "UTF-8";
    }

    "ASCII"
}

fn resolve_alias(alias: &str) -> Option<&'static str> {
    ALIAS_TABLE.iter()
        .find(|e| e.alias == alias)
        .map(|e| e.canonical)
}