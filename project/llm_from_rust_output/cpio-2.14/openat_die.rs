use std::ffi::CString;
use std::process;

fn gettext(msgid: &str) -> String {
    unsafe {
        let msgid_cstr = CString::new(msgid).unwrap();
        let translated = dcgettext(
            std::ptr::null(),
            msgid_cstr.as_ptr(),
            libc::LC_MESSAGES,
        );
        CString::from_raw(translated).into_string().unwrap()
    }
}

extern "C" {
    fn dcgettext(
        domainname: *const libc::c_char,
        msgid: *const libc::c_char,
        category: libc::c_int,
    ) -> *mut libc::c_char;
}

pub fn openat_save_fail(errnum: libc::c_int) -> ! {
    let msg = gettext("unable to record current working directory");
    eprintln!("{}", msg);
    process::exit(1);
}

pub fn openat_restore_fail(errnum: libc::c_int) -> ! {
    let msg = gettext("failed to return to initial working directory");
    eprintln!("{}", msg);
    process::exit(1);
}