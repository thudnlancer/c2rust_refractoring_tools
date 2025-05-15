use std::ffi::CStr;
use std::ptr;

#[repr(C)]
pub struct FILE {
    // Omitted fields as they're not directly used in the safe interface
}

#[repr(C)]
pub struct argp_state {
    // Omitted fields as they're not directly used in the safe interface
}

#[repr(C)]
pub struct argp_option {
    pub name: *const libc::c_char,
    pub key: libc::c_int,
    pub arg: *const libc::c_char,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
    pub group: libc::c_int,
}

#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    // Omitted other fields as they're not used in the safe interface
}

static PROGRAM_CANONICAL_NAME: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };
static PROGRAM_AUTHORS: &'static [&'static CStr] = &[];

fn version_etc_hook(stream: *mut FILE, state: *mut argp_state) {
    unsafe {
        version_etc_ar(
            stream,
            PROGRAM_CANONICAL_NAME.as_ptr(),
            CStr::from_bytes_with_nul_unchecked(b"GNU cpio\0").as_ptr(),
            CStr::from_bytes_with_nul_unchecked(b"2.14\0").as_ptr(),
            PROGRAM_AUTHORS.as_ptr() as *const *const libc::c_char,
        );
    }
}

pub fn argp_version_setup(name: &'static CStr, authors: &'static [&'static CStr]) {
    unsafe {
        argp_program_version_hook = Some(version_etc_hook);
        PROGRAM_CANONICAL_NAME = name;
        PROGRAM_AUTHORS = authors;
    }
}

extern "C" {
    fn version_etc_ar(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        authors: *const *const libc::c_char,
    );
    static mut argp_program_version_hook: Option<
        unsafe extern "C" fn(*mut FILE, *mut argp_state),
    >;
}