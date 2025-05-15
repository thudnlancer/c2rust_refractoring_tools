use std::ffi::CStr;
use std::os::unix::io::RawFd;
use libc::{mode_t, c_int, c_char};
use std::ptr;
use std::io;

extern "C" {
    fn qcopy_acl(
        src_name: *const c_char,
        source_desc: c_int,
        dst_name: *const c_char,
        dest_desc: c_int,
        mode: mode_t,
    ) -> c_int;
    fn dcgettext(
        domainname: *const c_char,
        msgid: *const c_char,
        category: c_int,
    ) -> *mut c_char;
}

#[derive(Debug)]
enum CopyAclError {
    SourceError(io::Error),
    DestError(io::Error),
}

fn copy_acl(
    src_name: &CStr,
    source_desc: RawFd,
    dst_name: &CStr,
    dest_desc: RawFd,
    mode: mode_t,
) -> Result<(), CopyAclError> {
    let ret = unsafe {
        qcopy_acl(
            src_name.as_ptr(),
            source_desc,
            dst_name.as_ptr(),
            dest_desc,
            mode,
        )
    };

    match ret {
        -2 => {
            let err = io::Error::last_os_error();
            Err(CopyAclError::SourceError(err))
        }
        -1 => {
            let err = io::Error::last_os_error();
            Err(CopyAclError::DestError(err))
        }
        _ => Ok(()),
    }
}

fn handle_copy_acl_error(
    src_name: &CStr,
    dst_name: &CStr,
    result: Result<(), CopyAclError>,
) {
    match result {
        Ok(_) => (),
        Err(CopyAclError::SourceError(e)) => {
            eprintln!("Error copying ACL from {:?}: {}", src_name, e);
        }
        Err(CopyAclError::DestError(e)) => {
            let msg = unsafe {
                CStr::from_ptr(dcgettext(
                    ptr::null(),
                    b"preserving permissions for %s\0".as_ptr() as *const c_char,
                    5,
                ))
            };
            eprintln!("{} {:?}: {}", msg.to_string_lossy(), dst_name, e);
        }
    }
}