use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;

type SizeT = usize;
type IconvT = *mut c_void;

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn mem_cd_iconv(
        src: *const c_char,
        srclen: SizeT,
        cd: IconvT,
        resultp: *mut *mut c_char,
        lengthp: *mut SizeT,
    ) -> c_int;
    fn str_cd_iconv(src: *const c_char, cd: IconvT) -> *mut c_char;
    fn str_iconv(
        src: *const c_char,
        from_codeset: *const c_char,
        to_codeset: *const c_char,
    ) -> *mut c_char;
    fn xalloc_die();
}

fn check_errno() -> bool {
    unsafe { *__errno_location() == 12 }
}

pub fn xmem_cd_iconv(
    src: &CStr,
    srclen: SizeT,
    cd: IconvT,
) -> Result<(CString, SizeT), ()> {
    let mut result_ptr: *mut c_char = ptr::null_mut();
    let mut length: SizeT = 0;

    let retval = unsafe {
        mem_cd_iconv(
            src.as_ptr(),
            srclen,
            cd,
            &mut result_ptr,
            &mut length,
        )
    };

    if retval < 0 && check_errno() {
        unsafe { xalloc_die() };
    }

    if retval < 0 {
        Err(())
    } else {
        unsafe {
            Ok((
                CString::from_raw(result_ptr),
                length,
            ))
        }
    }
}

pub fn xstr_cd_iconv(
    src: &CStr,
    cd: IconvT,
) -> Result<CString, ()> {
    let result = unsafe { str_cd_iconv(src.as_ptr(), cd) };

    if result.is_null() && check_errno() {
        unsafe { xalloc_die() };
    }

    if result.is_null() {
        Err(())
    } else {
        unsafe { Ok(CString::from_raw(result)) }
    }
}

pub fn xstr_iconv(
    src: &CStr,
    from_codeset: &CStr,
    to_codeset: &CStr,
) -> Result<CString, ()> {
    let result = unsafe {
        str_iconv(
            src.as_ptr(),
            from_codeset.as_ptr(),
            to_codeset.as_ptr(),
        )
    };

    if result.is_null() && check_errno() {
        unsafe { xalloc_die() };
    }

    if result.is_null() {
        Err(())
    } else {
        unsafe { Ok(CString::from_raw(result)) }
    }
}