use std::ffi::CString;
use std::process;

static FATAL_EXIT_HOOK: std::sync::Once = std::sync::Once::new();

pub fn set_fatal_exit_hook<F>(hook: F)
where
    F: Fn() + Send + Sync + 'static,
{
    FATAL_EXIT_HOOK.call_once(|| {
        let hook = Box::new(hook);
        let hook_ptr = Box::into_raw(hook);
        unsafe {
            libc::atexit(std::mem::transmute::<_, unsafe extern "C" fn()>(hook_ptr));
        }
    });
}

pub fn fatal_exit() -> ! {
    FATAL_EXIT_HOOK.call_once(|| {});

    let msg = CString::new("Error is not recoverable: exiting now").unwrap();
    unsafe {
        libc::error(
            2,
            0,
            libc::dcgettext(
                std::ptr::null(),
                msg.as_ptr(),
                libc::LC_MESSAGES,
            ),
        );
    }
    process::abort();
}

pub fn xalloc_die() {
    let msg = CString::new("memory exhausted").unwrap();
    unsafe {
        libc::error(
            0,
            0,
            b"%s\0".as_ptr() as *const libc::c_char,
            libc::dcgettext(
                std::ptr::null(),
                msg.as_ptr(),
                libc::LC_MESSAGES,
            ),
        );
    }
    fatal_exit();
}