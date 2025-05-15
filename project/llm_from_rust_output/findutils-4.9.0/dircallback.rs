use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::ptr;
use libc::{self, c_int, c_char, c_void};

#[derive(Debug)]
struct SavedCwd {
    desc: RawFd,
    name: Option<CString>,
}

impl SavedCwd {
    fn new() -> Self {
        SavedCwd {
            desc: 0,
            name: None,
        }
    }

    fn save(&mut self) -> Result<(), i32> {
        unsafe {
            let mut c_saved = saved_cwd {
                desc: 0,
                name: ptr::null_mut(),
            };
            if save_cwd(&mut c_saved) == 0 {
                self.desc = c_saved.desc;
                if !c_saved.name.is_null() {
                    self.name = Some(CString::from_raw(c_saved.name));
                }
                Ok(())
            } else {
                Err(*__errno_location())
            }
        }
    }

    fn restore(&self) -> Result<(), i32> {
        unsafe {
            let c_saved = saved_cwd {
                desc: self.desc,
                name: self.name.as_ref().map(|s| s.as_ptr() as *mut c_char).unwrap_or(ptr::null_mut()),
            };
            if restore_cwd(&c_saved) == 0 {
                Ok(())
            } else {
                Err(*__errno_location())
            }
        }
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        unsafe {
            let mut c_saved = saved_cwd {
                desc: self.desc,
                name: self.name.take().map(|s| s.into_raw()).unwrap_or(ptr::null_mut()),
            };
            free_cwd(&mut c_saved);
        }
    }
}

pub fn run_in_dir<F>(there: &SavedCwd, callback: F, usercontext: *mut c_void) -> Result<i32, i32>
where
    F: FnOnce(*mut c_void) -> i32,
{
    let mut here = SavedCwd::new();
    let saved_errno = match here.save() {
        Ok(_) => {
            match there.restore() {
                Ok(_) => {
                    let err = callback(usercontext);
                    if err < 0 {
                        unsafe { *__errno_location() }
                    } else {
                        0
                    }
                }
                Err(e) => {
                    unsafe { openat_restore_fail(e) };
                    return Err(e);
                }
            }
        }
        Err(e) => {
            unsafe { openat_save_fail(e) };
            return Err(e);
        }
    };

    if let Err(e) = here.restore() {
        unsafe { openat_restore_fail(e) };
        return Err(e);
    }

    if saved_errno != 0 {
        unsafe { *__errno_location() = saved_errno };
    }
    Ok(saved_errno)
}

pub fn run_in_dirfd<F>(dir_fd: RawFd, callback: F, usercontext: *mut c_void) -> Result<i32, i32>
where
    F: FnOnce(*mut c_void) -> i32,
{
    if dir_fd == -100 {
        Ok(callback(usercontext))
    } else {
        let mut saved_cwd = SavedCwd::new();
        let saved_errno = match saved_cwd.save() {
            Ok(_) => {
                if unsafe { fchdir(dir_fd) } != 0 {
                    let e = unsafe { *__errno_location() };
                    return Err(e);
                }
                let err = callback(usercontext);
                if err < 0 {
                    unsafe { *__errno_location() }
                } else {
                    0
                }
            }
            Err(e) => {
                unsafe { openat_save_fail(e) };
                return Err(e);
            }
        };

        if let Err(e) = saved_cwd.restore() {
            unsafe { openat_restore_fail(e) };
            return Err(e);
        }

        if saved_errno != 0 {
            unsafe { *__errno_location() = saved_errno };
        }
        Ok(saved_errno)
    }
}

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn openat_restore_fail(errno: c_int);
    fn openat_save_fail(errno: c_int);
    fn save_cwd(cwd: *mut saved_cwd) -> c_int;
    fn restore_cwd(cwd: *const saved_cwd) -> c_int;
    fn free_cwd(cwd: *mut saved_cwd);
}

#[repr(C)]
struct saved_cwd {
    desc: c_int,
    name: *mut c_char,
}