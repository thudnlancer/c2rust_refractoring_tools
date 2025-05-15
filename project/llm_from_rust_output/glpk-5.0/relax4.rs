use std::ffi::CStr;
use std::process::abort;

type GlpErrFunc = Option<unsafe extern "C" fn(*const libc::c_char, ...)>;

#[derive(Clone)]
pub struct Relax4Csa {
    pub n: libc::c_int,
    pub na: libc::c_int,
    pub large: libc::c_int,
    pub repeat: libc::c_int,
    pub crash: libc::c_int,
    pub startn: Vec<libc::c_int>,
    pub endn: Vec<libc::c_int>,
    pub fou: Vec<libc::c_int>,
    pub nxtou: Vec<libc::c_int>,
    pub fin: Vec<libc::c_int>,
    pub nxtin: Vec<libc::c_int>,
    pub rc: Vec<libc::c_int>,
    pub u: Vec<libc::c_int>,
    pub dfct: Vec<libc::c_int>,
    pub x: Vec<libc::c_int>,
    pub nmultinode: libc::c_int,
    pub iter: libc::c_int,
    pub num_augm: libc::c_int,
    pub num_ascnt: libc::c_int,
    pub nsp: libc::c_int,
    pub label: Vec<libc::c_int>,
    pub prdcsr: Vec<libc::c_int>,
    pub save: Vec<libc::c_int>,
    pub tfstou: Vec<libc::c_int>,
    pub tnxtou: Vec<libc::c_int>,
    pub tfstin: Vec<libc::c_int>,
    pub tnxtin: Vec<libc::c_int>,
    pub nxtqueue: Vec<libc::c_int>,
    pub scan: Vec<libc::c_char>,
    pub mark: Vec<libc::c_char>,
    pub extend_arc: Vec<libc::c_int>,
    pub sb_level: Vec<libc::c_int>,
    pub sb_arc: Vec<libc::c_int>,
}

#[no_mangle]
pub extern "C" fn _glp_relax4(csa: *mut Relax4Csa) -> libc::c_int {
    let func = CStr::from_bytes_with_nul(b"relax4\0").unwrap();
    let error_msg = CStr::from_bytes_with_nul(b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0").unwrap();
    let file = CStr::from_bytes_with_nul(b"misc/relax4.c\0").unwrap();

    unsafe {
        if let Some(err_func) = glp_error_(file.as_ptr(), 9) {
            err_func(error_msg.as_ptr(), func.as_ptr());
        }
        abort();
    }
}

#[no_mangle]
pub extern "C" fn _glp_relax4_inidat(csa: *mut Relax4Csa) {
    let func = CStr::from_bytes_with_nul(b"relax4_inidat\0").unwrap();
    let error_msg = CStr::from_bytes_with_nul(b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0").unwrap();
    let file = CStr::from_bytes_with_nul(b"misc/relax4.c\0").unwrap();

    unsafe {
        if let Some(err_func) = glp_error_(file.as_ptr(), 18) {
            err_func(error_msg.as_ptr(), func.as_ptr());
        }
        abort();
    }
}

extern "C" {
    fn glp_assert_(expr: *const libc::c_char, file: *const libc::c_char, line: libc::c_int);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> GlpErrFunc;
}