use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use std::mem;

pub fn xvasprintf(format: &CStr, args: &mut dyn std::fmt::Arguments) -> Option<CString> {
    let mut argcount = 0;
    let mut f = format.to_bytes();

    // Check for simple %s format strings
    loop {
        if f.is_empty() {
            return xstrcat(argcount, args);
        }
        if f[0] != b'%' {
            break;
        }
        if f.len() < 2 {
            break;
        }
        if f[1] != b's' {
            break;
        }
        f = &f[2..];
        argcount += 1;
    }

    // Fall back to vasprintf for complex formats
    let mut result = ptr::null_mut();
    let c_str = unsafe {
        let ap = args.as_va_list();
        if libc::vasprintf(&mut result, format.as_ptr(), ap) < 0 {
            if libc::__errno_location().read() == libc::ENOMEM {
                xalloc_die();
            }
            return None;
        }
        CStr::from_ptr(result)
    };

    let ret = c_str.to_owned();
    unsafe { libc::free(result as *mut _) };
    Some(ret)
}

fn xstrcat(argcount: usize, args: &mut dyn std::fmt::Arguments) -> Option<CString> {
    let mut totalsize = 0;
    let mut ap = args.as_va_list();

    // Calculate total size
    for _ in 0..argcount {
        let next = unsafe { ap.arg::<*const c_char>() };
        let next_str = unsafe { CStr::from_ptr(next) };
        totalsize = match totalsize.checked_add(next_str.to_bytes().len()) {
            Some(sum) => sum,
            None => {
                unsafe { *libc::__errno_location() = libc::EOVERFLOW };
                return None;
            }
        };
    }

    // Check for overflow
    if totalsize > libc::c_int::MAX as usize {
        unsafe { *libc::__errno_location() = libc::EOVERFLOW };
        return None;
    }

    // Allocate buffer
    let mut buffer = Vec::with_capacity(totalsize + 1);
    let mut args = args.as_va_list();

    // Concatenate strings
    for _ in 0..argcount {
        let next = unsafe { args.arg::<*const c_char>() };
        let next_str = unsafe { CStr::from_ptr(next) };
        buffer.extend_from_slice(next_str.to_bytes());
    }

    buffer.push(0); // Null terminator

    // Convert to CString
    unsafe { CString::from_vec_unchecked(buffer) }.into()
}

fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}