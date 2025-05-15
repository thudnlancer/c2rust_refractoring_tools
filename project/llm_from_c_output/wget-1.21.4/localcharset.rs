use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

#[cfg(target_os = "windows")]
use winapi::um::winnls::GetACP;

#[cfg(target_os = "macos")]
use libc::{uselocale, MB_CUR_MAX};

#[cfg(any(target_os = "linux", target_os = "android"))]
use libc::nl_langinfo;

#[cfg(target_os = "windows")]
const WINDOWS_NATIVE: bool = true;

struct TableEntry {
    alias: &'static str,
    canonical: &'static str,
}

struct LocaleEntry {
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
    target_os = "windows"
))]
const ALIAS_TABLE: &[TableEntry] = &[
    // FreeBSD entries
    #[cfg(target_os = "freebsd")]
    TableEntry {
        alias: "Big5",
        canonical: "BIG5",
    },
    #[cfg(target_os = "freebsd")]
    TableEntry {
        alias: "C",
        canonical: "ASCII",
    },
    // Add other platform-specific entries here...
];

// Platform-specific locale tables
#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
const LOCALE_TABLE: &[LocaleEntry] = &[
    LocaleEntry {
        locale: "cs_CZ.ISO_8859-2",
        canonical: "ISO-8859-2",
    },
    // Add other locale entries here...
];

pub fn locale_charset() -> &'static str {
    let codeset = get_codeset_from_platform();

    // Resolve alias
    let resolved = resolve_alias(codeset);
    if resolved != codeset {
        return resolved;
    }

    // Special case for Darwin
    #[cfg(target_os = "macos")]
    {
        if resolved == "UTF-8" {
            unsafe {
                let cur_locale = uselocale(ptr::null_mut());
                if MB_CUR_MAX(cur_locale) <= 1 {
                    return "ASCII";
                }
            }
        }
    }

    resolved
}

fn get_codeset_from_platform() -> &'static str {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        unsafe {
            let codeset = nl_langinfo(libc::CODESET);
            if !codeset.is_null() {
                let cstr = CStr::from_ptr(codeset);
                return cstr.to_str().unwrap_or("");
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let current_locale = unsafe { libc::setlocale(libc::LC_CTYPE, ptr::null()) };
        if !current_locale.is_null() {
            let cstr = unsafe { CStr::from_ptr(current_locale) };
            if let Ok(locale_str) = cstr.to_str() {
                if let Some(pdot) = locale_str.rfind('.') {
                    let suffix = &locale_str[pdot + 1..];
                    if suffix == "65001" || suffix == "utf8" {
                        return "UTF-8";
                    } else {
                        return suffix;
                    }
                }
            }
        }

        let cp = unsafe { GetACP() };
        return match cp {
            65001 => "UTF-8",
            _ => "CP1252", // Fallback to Windows-1252
        };
    }

    // Fallback for other platforms
    get_codeset_from_env()
}

fn get_codeset_from_env() -> &'static str {
    if let Ok(locale) = env::var("LC_ALL") {
        if !locale.is_empty() {
            return parse_locale(&locale);
        }
    }

    if let Ok(locale) = env::var("LC_CTYPE") {
        if !locale.is_empty() {
            return parse_locale(&locale);
        }
    }

    if let Ok(locale) = env::var("LANG") {
        if !locale.is_empty() {
            return parse_locale(&locale);
        }
    }

    // Default fallback
    "ASCII"
}

fn parse_locale(locale: &str) -> &'static str {
    if let Some(dot) = locale.find('.') {
        let encoding = &locale[dot + 1..];
        if let Some(at) = encoding.find('@') {
            return &encoding[..at];
        }
        return encoding;
    }

    // Check locale table
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    {
        if let Ok(idx) = LOCALE_TABLE.binary_search_by(|entry| entry.locale.cmp(locale)) {
            return LOCALE_TABLE[idx].canonical;
        }
    }

    // Default cases
    if locale == "C" || locale == "POSIX" {
        "ASCII"
    } else {
        #[cfg(any(target_os = "macos", target_os = "haiku"))]
        {
            "UTF-8"
        }
        #[cfg(not(any(target_os = "macos", target_os = "haiku"))]
        {
            "ASCII"
        }
    }
}

fn resolve_alias(codeset: &str) -> &'static str {
    #[cfg(any(
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "macos",
        target_os = "linux",
        target_os = "windows"
    ))]
    {
        if let Ok(idx) = ALIAS_TABLE.binary_search_by(|entry| entry.alias.cmp(codeset)) {
            return ALIAS_TABLE[idx].canonical;
        }
    }

    // Special cases
    if codeset.is_empty() {
        return "ASCII";
    }

    #[cfg(any(target_os = "macos", target_os = "haiku"))]
    {
        if codeset == "UTF-8" {
            return "UTF-8";
        }
    }

    codeset
}