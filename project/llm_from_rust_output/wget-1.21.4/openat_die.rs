use std::ffi::CString;
use std::process;

fn openat_save_fail(errnum: i32) {
    let domain = CString::new("wget-gnulib").unwrap();
    let msg = CString::new("unable to record current working directory").unwrap();
    unsafe {
        error(
            exit_failure,
            errnum,
            dcgettext(domain.as_ptr(), msg.as_ptr(), 5),
        );
    }
    process::abort();
}

fn openat_restore_fail(errnum: i32) {
    let domain = CString::new("wget-gnulib").unwrap();
    let msg = CString::new("failed to return to initial working directory").unwrap();
    unsafe {
        error(
            exit_failure,
            errnum,
            dcgettext(domain.as_ptr(), msg.as_ptr(), 5),
        );
    }
    process::abort();
}

extern "C" {
    static exit_failure: i32;
    fn error(status: i32, errnum: i32, format: *const libc::c_char, ...);
    fn dcgettext(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
        category: libc::c_int,
    ) -> *mut libc::c_char;
}