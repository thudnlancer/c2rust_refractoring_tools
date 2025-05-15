use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[derive(Clone)]
pub struct CompressionSuffix {
    pub suffix: &'static str,
    pub program: Option<&'static str>,
}

static COMPRESSION_SUFFIXES: &[CompressionSuffix] = &[
    CompressionSuffix {
        suffix: "tar",
        program: None,
    },
    CompressionSuffix {
        suffix: "gz",
        program: Some("gzip"),
    },
    CompressionSuffix {
        suffix: "tgz",
        program: Some("gzip"),
    },
    CompressionSuffix {
        suffix: "taz",
        program: Some("gzip"),
    },
    CompressionSuffix {
        suffix: "Z",
        program: Some("compress"),
    },
    CompressionSuffix {
        suffix: "taZ",
        program: Some("compress"),
    },
    CompressionSuffix {
        suffix: "bz2",
        program: Some("bzip2"),
    },
    CompressionSuffix {
        suffix: "tbz",
        program: Some("bzip2"),
    },
    CompressionSuffix {
        suffix: "tbz2",
        program: Some("bzip2"),
    },
    CompressionSuffix {
        suffix: "tz2",
        program: Some("bzip2"),
    },
    CompressionSuffix {
        suffix: "lz",
        program: Some("lzip"),
    },
    CompressionSuffix {
        suffix: "lzma",
        program: Some("lzma"),
    },
    CompressionSuffix {
        suffix: "tlz",
        program: Some("lzma"),
    },
    CompressionSuffix {
        suffix: "lzo",
        program: Some("lzop"),
    },
    CompressionSuffix {
        suffix: "xz",
        program: Some("xz"),
    },
    CompressionSuffix {
        suffix: "txz",
        program: Some("xz"),
    },
    CompressionSuffix {
        suffix: "zst",
        program: Some("zstd"),
    },
    CompressionSuffix {
        suffix: "tzst",
        program: Some("zstd"),
    },
];

pub fn find_compression_suffix(name: &str) -> Option<(&CompressionSuffix, usize)> {
    if let Some((base, suffix)) = name.rsplit_once('.') {
        for comp in COMPRESSION_SUFFIXES {
            if comp.suffix == suffix {
                return Some((comp, base.len()));
            }
        }
    }
    None
}

pub fn find_compression_program(name: &str, default: Option<&str>) -> Option<&'static str> {
    find_compression_suffix(name)
        .and_then(|(comp, _)| comp.program)
        .or(default)
}

pub fn set_compression_program_by_suffix(name: &str, default: Option<&str>) -> Option<&'static str> {
    find_compression_program(name, default)
}

pub fn strip_compression_suffix(name: &str) -> Option<String> {
    if let Some((comp, len)) = find_compression_suffix(name) {
        if len > 4 && name[len - 4..] == *".tar" && !comp.suffix.starts_with('t') {
            let new_len = len - 4;
            if new_len == 0 {
                return None;
            }
            Some(name[..new_len].to_string())
        } else {
            if len == 0 {
                return None;
            }
            Some(name[..len].to_string())
        }
    } else {
        None
    }
}

// FFI-compatible wrappers
#[no_mangle]
pub extern "C" fn ffi_find_compression_program(
    name: *const c_char,
    defprog: *const c_char,
) -> *const c_char {
    let name_str = unsafe { CStr::from_ptr(name).to_str().unwrap_or("") };
    let defprog_str = if defprog.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(defprog).to_str().ok() }
    };

    if let Some(prog) = find_compression_program(name_str, defprog_str) {
        CString::new(prog).unwrap().into_raw()
    } else {
        ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn ffi_set_compression_program_by_suffix(
    name: *const c_char,
    defprog: *const c_char,
) -> *const c_char {
    let name_str = unsafe { CStr::from_ptr(name).to_str().unwrap_or("") };
    let defprog_str = if defprog.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(defprog).to_str().ok() }
    };

    if let Some(prog) = set_compression_program_by_suffix(name_str, defprog_str) {
        CString::new(prog).unwrap().into_raw()
    } else {
        ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn ffi_strip_compression_suffix(name: *const c_char) -> *mut c_char {
    let name_str = unsafe { CStr::from_ptr(name).to_str().unwrap_or("") };
    if let Some(stripped) = strip_compression_suffix(name_str) {
        CString::new(stripped).unwrap().into_raw()
    } else {
        ptr::null_mut()
    }
}