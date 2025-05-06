#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn at_fatal_signal(function: Option<unsafe extern "C" fn(i32) -> ()>) -> i32;
    fn xalloc_die();
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type size_t = u64;
pub type __pid_t = i32;
pub type __sig_atomic_t = i32;
pub type pid_t = __pid_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slaves_entry_t {
    pub used: sig_atomic_t,
    pub child: pid_t,
}
static mut static_slaves: [slaves_entry_t; 32] = [slaves_entry_t {
    used: 0,
    child: 0,
}; 32];
static mut slaves: *mut slaves_entry_t = unsafe { static_slaves.as_ptr() as *mut _ };
static mut slaves_count: sig_atomic_t = 0 as i32;
static mut slaves_allocated: size_t = 0;
unsafe extern "C" fn cleanup_slaves() {
    loop {
        let mut n: size_t = slaves_count as size_t;
        if n == 0 as i32 as u64 {
            break;
        }
        n = n.wrapping_sub(1);
        n;
        ::core::ptr::write_volatile(
            &mut slaves_count as *mut sig_atomic_t,
            n as sig_atomic_t,
        );
        if (*slaves.offset(n as isize)).used != 0 {
            let mut slave: pid_t = (*slaves.offset(n as isize)).child;
            kill(slave, 1 as i32);
        }
    };
}
unsafe extern "C" fn cleanup_slaves_action(mut sig: i32) {
    cleanup_slaves();
}
#[no_mangle]
pub unsafe extern "C" fn register_slave_subprocess(mut child: pid_t) {
    static mut cleanup_slaves_registered: bool = 0 as i32 != 0;
    if !cleanup_slaves_registered {
        atexit(Some(cleanup_slaves as unsafe extern "C" fn() -> ()));
        if at_fatal_signal(
            Some(cleanup_slaves_action as unsafe extern "C" fn(i32) -> ()),
        ) < 0 as i32
        {
            xalloc_die();
        }
        cleanup_slaves_registered = 1 as i32 != 0;
    }
    let mut s: *mut slaves_entry_t = slaves;
    let mut s_end: *mut slaves_entry_t = s.offset(slaves_count as isize);
    while s < s_end {
        if (*s).used == 0 {
            ::core::ptr::write_volatile(&mut (*s).child as *mut pid_t, child);
            ::core::ptr::write_volatile(&mut (*s).used as *mut sig_atomic_t, 1 as i32);
            return;
        }
        s = s.offset(1);
        s;
    }
    if slaves_count as u64 == slaves_allocated {
        let mut old_slaves: *mut slaves_entry_t = slaves;
        let mut new_slaves_allocated: size_t = (2 as i32 as u64)
            .wrapping_mul(slaves_allocated);
        let mut new_slaves: *mut slaves_entry_t = malloc(
            new_slaves_allocated
                .wrapping_mul(::core::mem::size_of::<slaves_entry_t>() as u64),
        ) as *mut slaves_entry_t;
        if new_slaves.is_null() {
            kill(child, 1 as i32);
            xalloc_die();
        }
        memcpy(
            new_slaves as *mut libc::c_void,
            old_slaves as *const libc::c_void,
            slaves_allocated
                .wrapping_mul(::core::mem::size_of::<slaves_entry_t>() as u64),
        );
        ::core::ptr::write_volatile(&mut slaves as *mut *mut slaves_entry_t, new_slaves);
        slaves_allocated = new_slaves_allocated;
        if old_slaves != static_slaves.as_mut_ptr() {
            rpl_free(old_slaves as *mut libc::c_void);
        }
    }
    ::core::ptr::write_volatile(
        &mut (*slaves.offset(slaves_count as isize)).child as *mut pid_t,
        child,
    );
    ::core::ptr::write_volatile(
        &mut (*slaves.offset(slaves_count as isize)).used as *mut sig_atomic_t,
        1 as i32,
    );
    ::core::ptr::write_volatile(
        &mut slaves_count as *mut sig_atomic_t,
        ::core::ptr::read_volatile::<sig_atomic_t>(&slaves_count as *const sig_atomic_t)
            + 1,
    );
    ::core::ptr::read_volatile::<sig_atomic_t>(&slaves_count as *const sig_atomic_t);
}
unsafe extern "C" fn unregister_slave_subprocess(mut child: pid_t) {
    let mut s: *mut slaves_entry_t = slaves;
    let mut s_end: *mut slaves_entry_t = s.offset(slaves_count as isize);
    while s < s_end {
        if (*s).used != 0 && (*s).child == child {
            ::core::ptr::write_volatile(&mut (*s).used as *mut sig_atomic_t, 0 as i32);
        }
        s = s.offset(1);
        s;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wait_subprocess(
    mut child: pid_t,
    mut progname: *const i8,
    mut ignore_sigpipe: bool,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut termsigp: *mut i32,
) -> i32 {
    let mut status: i32 = 0;
    if !termsigp.is_null() {
        *termsigp = 0 as i32;
    }
    status = 0 as i32;
    loop {
        let mut result: i32 = waitpid(child, &mut status, 0 as i32);
        if result != child {
            if *__errno_location() == 4 as i32 {
                continue;
            }
            if exit_on_error as i32 != 0 || !null_stderr {
                error(
                    if exit_on_error as i32 != 0 { 1 as i32 } else { 0 as i32 },
                    *__errno_location(),
                    dcgettext(
                        b"wget-gnulib\0" as *const u8 as *const i8,
                        b"%s subprocess\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    progname,
                );
            }
            return 127 as i32;
        } else if !(status & 0xff as i32 == 0x7f as i32) {
            break;
        }
    }
    if slave_process {
        unregister_slave_subprocess(child);
    }
    if ((status & 0x7f as i32) + 1 as i32) as libc::c_schar as i32 >> 1 as i32 > 0 as i32
    {
        if !termsigp.is_null() {
            *termsigp = status & 0x7f as i32;
        }
        if status & 0x7f as i32 == 13 as i32 && ignore_sigpipe as i32 != 0 {
            return 0 as i32;
        }
        if exit_on_error as i32 != 0 || !null_stderr && termsigp.is_null() {
            error(
                if exit_on_error as i32 != 0 { 1 as i32 } else { 0 as i32 },
                0 as i32,
                dcgettext(
                    b"wget-gnulib\0" as *const u8 as *const i8,
                    b"%s subprocess got fatal signal %d\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                progname,
                status & 0x7f as i32,
            );
        }
        return 127 as i32;
    }
    if !(status & 0x7f as i32 == 0 as i32) {
        abort();
    }
    if (status & 0xff00 as i32) >> 8 as i32 == 127 as i32 {
        if exit_on_error as i32 != 0 || !null_stderr {
            error(
                if exit_on_error as i32 != 0 { 1 as i32 } else { 0 as i32 },
                0 as i32,
                dcgettext(
                    b"wget-gnulib\0" as *const u8 as *const i8,
                    b"%s subprocess failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                progname,
            );
        }
        return 127 as i32;
    }
    return (status & 0xff00 as i32) >> 8 as i32;
}
unsafe extern "C" fn run_static_initializers() {
    slaves_allocated = (::core::mem::size_of::<[slaves_entry_t; 32]>() as u64)
        .wrapping_div(::core::mem::size_of::<slaves_entry_t>() as u64);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];