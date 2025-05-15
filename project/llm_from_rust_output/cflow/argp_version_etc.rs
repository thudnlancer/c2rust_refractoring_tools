use std::ffi::CStr;
use std::ptr;

#[repr(C)]
pub struct FILE {
    // Simplified FILE structure since we only need to pass it through
    _unused: [u8; 0],
}

#[repr(C)]
pub struct argp_state {
    // Simplified argp_state structure since we only need to pass it through
    _unused: [u8; 0],
}

pub type error_t = i32;

static PROGRAM_CANONICAL_NAME: once_cell::sync::OnceCell<&'static CStr> = once_cell::sync::OnceCell::new();
static PROGRAM_AUTHORS: once_cell::sync::OnceCell<&'static [&'static CStr]> = once_cell::sync::OnceCell::new();

extern "C" {
    fn version_etc_ar(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        authors: *const *const libc::c_char,
    );
}

pub fn version_etc_hook(stream: *mut FILE, _state: *mut argp_state) {
    unsafe {
        version_etc_ar(
            stream,
            PROGRAM_CANONICAL_NAME.get().unwrap().as_ptr(),
            b"GNU cflow\0".as_ptr() as *const libc::c_char,
            b"1.4\0".as_ptr() as *const libc::c_char,
            PROGRAM_AUTHORS.get().unwrap().as_ptr() as *const *const libc::c_char,
        );
    }
}

#[no_mangle]
pub extern "C" fn argp_version_setup(name: *const libc::c_char, authors: *const *const libc::c_char) {
    unsafe {
        let name_cstr = CStr::from_ptr(name);
        let name_static = name_cstr.to_str().unwrap();
        PROGRAM_CANONICAL_NAME.set(name_cstr).unwrap();

        let mut authors_vec = Vec::new();
        let mut current = authors;
        while !(*current).is_null() {
            authors_vec.push(CStr::from_ptr(*current));
            current = current.add(1);
        }
        PROGRAM_AUTHORS.set(&authors_vec).unwrap();
    }
}

pub static ARGP_PROGRAM_VERSION_HOOK: Option<extern "C" fn(*mut FILE, *mut argp_state)> = Some(version_etc_hook);