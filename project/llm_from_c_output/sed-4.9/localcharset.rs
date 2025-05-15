use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;

#[cfg(target_os = "windows")]
use winapi::um::winnls::GetACP;

#[cfg(target_os = "macos")]
use core_foundation::locale::CFLocaleCopyCurrent;
#[cfg(target_os = "macos")]
use core_foundation::string::CFStringGetCString;
#[cfg(target_os = "macos")]
use core_foundation::string::CFStringRef;

#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use libc::nl_langinfo;
#[cfg(any(
    target_os = "linux",
    target_os = "android",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use libc::CODESET;

struct TableEntry {
    alias: &'static str,
    canonical: &'static str,
}

static ALIAS_TABLE: &[TableEntry] = &[
    TableEntry {
        alias: "UTF-8",
        canonical: "UTF-8",
    },
    // Add other entries as needed
];

pub fn locale_charset() -> &'static str {
    let codeset = get_codeset();

    // Resolve alias
    if let Some(entry) = ALIAS_TABLE.iter().find(|e| e.alias == codeset) {
        return entry.canonical;
    }

    // Default cases
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        "UTF-8"
    } else if codeset.is_empty() {
        "ASCII"
    } else {
        codeset
    }
}

fn get_codeset() -> &'static str {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    {
        unsafe {
            let codeset = nl_langinfo(CODESET);
            if !codeset.is_null() {
                if let Ok(cstr) = CStr::from_ptr(codeset).to_str() {
                    return Box::leak(cstr.to_string().into_boxed_str());
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let cp = unsafe { GetACP() };
        return Box::leak(format!("CP{}", cp).into_boxed_str());
    }

    #[cfg(target_os = "macos")]
    {
        unsafe {
            let locale = CFLocaleCopyCurrent();
            let codeset = CFLocaleGetValue(locale, kCFLocaleLanguageCode);
            if !codeset.is_null() {
                let mut buf = [0; 50];
                if CFStringGetCString(
                    codeset as CFStringRef,
                    buf.as_mut_ptr(),
                    buf.len() as i32,
                    kCFStringEncodingUTF8,
                ) != 0
                {
                    if let Ok(cstr) = CStr::from_ptr(buf.as_ptr()).to_str() {
                        return Box::leak(cstr.to_string().into_boxed_str());
                    }
                }
            }
        }
    }

    // Fallback to environment variables
    if let Ok(lang) = env::var("LC_ALL") {
        if !lang.is_empty() {
            return parse_locale(&lang);
        }
    }
    if let Ok(lang) = env::var("LC_CTYPE") {
        if !lang.is_empty() {
            return parse_locale(&lang);
        }
    }
    if let Ok(lang) = env::var("LANG") {
        if !lang.is_empty() {
            return parse_locale(&lang);
        }
    }

    ""
}

fn parse_locale(locale: &str) -> &'static str {
    if let Some(dot) = locale.find('.') {
        let encoding = &locale[dot + 1..];
        if let Some(at) = encoding.find('@') {
            return Box::leak(encoding[..at].to_string().into_boxed_str());
        }
        return Box::leak(encoding.to_string().into_boxed_str());
    }
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_charset() {
        let charset = locale_charset();
        assert!(!charset.is_empty());
    }
}