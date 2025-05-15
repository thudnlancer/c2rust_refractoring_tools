use std::ffi::{CString, CStr};
use std::os::raw::c_void;
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;
use libloading::{Library, Symbol};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOADED_SYMS: Mutex<Vec<LoadEntry>> = Mutex::new(Vec::new());
}

const SYMBOL_EXTENSION: &str = "_gmk_setup";

struct LoadEntry {
    name: String,
    lib: Option<Library>,
}

type LoadFunc = unsafe extern "C" fn(*const Floc) -> i32;

struct Floc {
    // Placeholder for floc structure
}

struct File {
    name: String,
    loaded: bool,
}

fn load_object(flocp: &Floc, noerror: bool, ldname: &str, symname: &str) -> Option<LoadFunc> {
    lazy_static! {
        static ref GLOBAL_DL: Mutex<Option<Library>> = Mutex::new(None);
    }

    let mut global_dl = GLOBAL_DL.lock().unwrap();
    if global_dl.is_none() {
        unsafe {
            *global_dl = Library::default().ok();
        }
        if global_dl.is_none() {
            let err = "Failed to open global symbol table";
            // fatal(flocp, err);
            return None;
        }
    }

    unsafe {
        if let Some(lib) = &*global_dl {
            if let Ok(sym) = lib.get::<LoadFunc>(symname.as_bytes()) {
                return Some(*sym);
            }
        }
    }

    let mut dlp = None;

    // If path has no '/', try current directory first
    if !ldname.contains('/') && !ldname.contains('\\') {
        let path = PathBuf::from(".").join(ldname);
        dlp = Library::new(path).ok();
    }

    // Try default search path
    if dlp.is_none() {
        dlp = Library::new(ldname).ok();
    }

    let dlp = match dlp {
        Some(lib) => lib,
        None => {
            if noerror {
                // debug_print(err);
            } else {
                // error(flocp, err);
            }
            return None;
        }
    };

    // debug_print(format!("Loaded shared object {}", ldname));

    // Check GPL compatibility
    unsafe {
        if dlp.get::<LoadFunc>("plugin_is_GPL_compatible".as_bytes()).is_err() {
            // fatal(flocp, "Object not GPL compatible");
            return None;
        }
    }

    let symp = unsafe {
        match dlp.get::<LoadFunc>(symname.as_bytes()) {
            Ok(sym) => *sym,
            Err(_) => {
                // fatal(flocp, "Failed to load symbol");
                return None;
            }
        }
    };

    // Add to loaded symbols list
    let mut loaded = LOADED_SYMS.lock().unwrap();
    loaded.push(LoadEntry {
        name: ldname.to_string(),
        lib: Some(dlp),
    });

    Some(symp)
}

fn load_file(flocp: &Floc, file: &mut File, noerror: bool) -> i32 {
    let ldname = file.name.clone();
    let mut symname = String::new();

    // Parse symbol name if provided in format name(symbol)
    if let Some(open_paren) = ldname.find('(') {
        if let Some(close_paren) = ldname[open_paren+1..].find(')') {
            let close_paren = open_paren + 1 + close_paren;
            if close_paren == ldname.len() - 1 {
                if open_paren + 1 == close_paren {
                    // fatal(flocp, "Empty symbol name");
                    return 0;
                }

                symname = ldname[open_paren+1..close_paren].to_string();
                file.name = ldname[..open_paren].to_string();
            }
        }
    }

    // If no symbol name provided, construct from filename
    if symname.is_empty() {
        let base = Path::new(&file.name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&file.name);

        let mut cleaned = String::new();
        for c in base.chars() {
            if c.is_alphanumeric() || c == '_' {
                cleaned.push(c);
            }
        }
        symname = format!("{}{}", cleaned, SYMBOL_EXTENSION);
    }

    // debug_print(format!("Loading symbol {} from {}", symname, file.name));

    if let Some(existing) = lookup_file(&file.name) {
        if existing.loaded {
            return -1;
        }
    }

    let symp = match load_object(flocp, noerror, &file.name, &symname) {
        Some(s) => s,
        None => return 0,
    };

    let r = unsafe { symp(flocp) };

    if r != 0 {
        // do_variable_definition(flocp, ".LOADED", &file.name);
    }

    r
}

fn unload_file(name: &str) -> i32 {
    let mut loaded = LOADED_SYMS.lock().unwrap();
    for entry in loaded.iter_mut() {
        if entry.name == name && entry.lib.is_some() {
            // debug_print(format!("Unloading shared object {}", name));
            entry.lib = None;
            return 0;
        }
    }
    1
}

// Placeholder functions
fn lookup_file(_name: &str) -> Option<File> {
    None
}

#[cfg(not(feature = "make_load"))]
fn load_file(flocp: &Floc, _file: &mut File, noerror: bool) -> i32 {
    if !noerror {
        // fatal(flocp, "Load operation not supported");
    }
    0
}

#[cfg(not(feature = "make_load"))]
fn unload_file(_name: &str) -> i32 {
    // fatal(None, "Cannot unload when load is not supported");
    1
}