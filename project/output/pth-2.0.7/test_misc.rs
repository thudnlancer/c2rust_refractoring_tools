#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type pth_st;
    pub type pth_attr_st;
    pub type pth_event_st;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pth_init() -> libc::c_int;
    fn pth_kill() -> libc::c_int;
    fn pth_ctrl(_: libc::c_ulong, _: ...) -> libc::c_long;
    fn pth_attr_of(_: pth_t) -> pth_attr_t;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: libc::c_int, _: ...) -> libc::c_int;
    fn pth_attr_destroy(_: pth_attr_t) -> libc::c_int;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_mutex_acquire(
        _: *mut pth_mutex_t,
        _: libc::c_int,
        _: pth_event_t,
    ) -> libc::c_int;
    fn pth_mutex_release(_: *mut pth_mutex_t) -> libc::c_int;
    fn pth_usleep(_: libc::c_uint) -> libc::c_int;
    fn pth_read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type pth_t = *mut pth_st;
pub type pth_attr_t = *mut pth_attr_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    PTH_ATTR_BOUND,
    PTH_ATTR_EVENTS,
    PTH_ATTR_STATE,
    PTH_ATTR_START_ARG,
    PTH_ATTR_START_FUNC,
    PTH_ATTR_TIME_RAN,
    PTH_ATTR_TIME_LAST,
    PTH_ATTR_TIME_SPAWN,
    PTH_ATTR_DISPATCHES,
    PTH_ATTR_STACK_ADDR,
    PTH_ATTR_STACK_SIZE,
    PTH_ATTR_CANCEL_STATE,
    PTH_ATTR_JOINABLE,
    PTH_ATTR_NAME,
    PTH_ATTR_PRIO,
}  // end of enum

pub type pth_event_t = *mut pth_event_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_ringnode_t = pth_ringnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mutex_st {
    pub mx_node: pth_ringnode_t,
    pub mx_state: libc::c_int,
    pub mx_owner: pth_t,
    pub mx_count: libc::c_ulong,
}
pub type pth_mutex_t = pth_mutex_st;
#[no_mangle]
pub static mut mutex: pth_mutex_t = {
    let mut init = pth_mutex_st {
        mx_node: {
            let mut init = pth_ringnode_st {
                rn_next: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                rn_prev: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
            };
            init
        },
        mx_state: (1 as libc::c_int) << 0 as libc::c_int,
        mx_owner: 0 as *const pth_st as pth_t,
        mx_count: 0 as libc::c_int as libc::c_ulong,
    };
    init
};
unsafe extern "C" fn my_reader(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut buf: [libc::c_char; 3] = [0; 3];
    let mut n: libc::c_int = 0;
    loop {
        n = pth_read(
            0 as libc::c_int,
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if n < 0 as libc::c_int {
            fprintf(stderr, b"reader: error\n\0" as *const u8 as *const libc::c_char);
            break;
        } else if n == 0 as libc::c_int {
            fprintf(stderr, b"reader: EOF\n\0" as *const u8 as *const libc::c_char);
            break;
        } else {
            if n == 1 as libc::c_int
                && buf[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
            {
                buf[0 as libc::c_int as usize] = '\\' as i32 as libc::c_char;
                buf[1 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
                n = 2 as libc::c_int;
            }
            buf[n as usize] = '\0' as i32 as libc::c_char;
            fprintf(
                stderr,
                b"reader: bytes=%d, char='%s'\n\0" as *const u8 as *const libc::c_char,
                n,
                buf.as_mut_ptr(),
            );
            if buf[0 as libc::c_int as usize] as libc::c_int == 'Q' as i32
                || buf[0 as libc::c_int as usize] as libc::c_int == 'q' as i32
            {
                break;
            }
            if buf[0 as libc::c_int as usize] as libc::c_int == 'L' as i32
                || buf[0 as libc::c_int as usize] as libc::c_int == 'l' as i32
            {
                fprintf(
                    stderr,
                    b"reader: ACQUIRE MUTEX\n\0" as *const u8 as *const libc::c_char,
                );
                pth_mutex_acquire(&mut mutex, 0 as libc::c_int, 0 as pth_event_t);
            }
            if buf[0 as libc::c_int as usize] as libc::c_int == 'U' as i32
                || buf[0 as libc::c_int as usize] as libc::c_int == 'u' as i32
            {
                fprintf(
                    stderr,
                    b"reader: RELEASE MUTEX\n\0" as *const u8 as *const libc::c_char,
                );
                pth_mutex_release(&mut mutex);
            }
            fflush(stderr);
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn my_child(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut name: *mut libc::c_char = _arg as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        pth_mutex_acquire(&mut mutex, 0 as libc::c_int, 0 as pth_event_t);
        fprintf(stderr, b"%s: %d\n\0" as *const u8 as *const libc::c_char, name, i);
        pth_mutex_release(&mut mutex);
        pth_usleep(500000 as libc::c_int as libc::c_uint);
        i += 1;
        i;
    }
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut child: [pth_t; 10] = [0 as *mut pth_st; 10];
    let mut t_attr: pth_attr_t = 0 as *mut pth_attr_st;
    let mut t_attr2: pth_attr_t = 0 as *mut pth_attr_st;
    let mut n: libc::c_long = 0;
    pth_init();
    fprintf(
        stderr,
        b"This is TEST_MISC, a Pth test using various stuff.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"A stdin reader child and various looping childs are\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"spawned. When you enter 'l' you can lock a mutex which\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"blocks the looping childs. 'u' unlocks this mutex.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"Enter 'q' to quit.\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Main Startup (%ld total threads running)\n\0" as *const u8
            as *const libc::c_char,
        pth_ctrl(
            ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
        ),
    );
    t_attr = pth_attr_new();
    pth_attr_set(t_attr, PTH_ATTR_JOINABLE as libc::c_int, 0 as libc::c_int);
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"foo\0" as *const u8 as *const libc::c_char,
    );
    child[0 as libc::c_int
        as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"foo\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"bar\0" as *const u8 as *const libc::c_char,
    );
    child[1 as libc::c_int
        as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"bar\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"baz\0" as *const u8 as *const libc::c_char,
    );
    child[2 as libc::c_int
        as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"baz\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"quux\0" as *const u8 as *const libc::c_char,
    );
    child[3 as libc::c_int
        as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"quux\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"killer\0" as *const u8 as *const libc::c_char,
    );
    pth_attr_set(t_attr, PTH_ATTR_PRIO as libc::c_int, 4 as libc::c_int);
    child[4 as libc::c_int
        as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"killer\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"killer II\0" as *const u8 as *const libc::c_char,
    );
    pth_attr_set(t_attr, PTH_ATTR_PRIO as libc::c_int, 5 as libc::c_int);
    child[5 as libc::c_int
        as usize] = pth_spawn(
        t_attr,
        Some(my_child as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"killer II\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_set(
        t_attr,
        PTH_ATTR_NAME as libc::c_int,
        b"reader\0" as *const u8 as *const libc::c_char,
    );
    pth_attr_set(t_attr, PTH_ATTR_PRIO as libc::c_int, 0 as libc::c_int);
    child[6 as libc::c_int
        as usize] = pth_spawn(
        t_attr,
        Some(my_reader as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        b"reader\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
    );
    pth_attr_destroy(t_attr);
    t_attr2 = pth_attr_of(child[0 as libc::c_int as usize]);
    pth_attr_set(t_attr2, PTH_ATTR_PRIO as libc::c_int, -(1 as libc::c_int));
    pth_attr_destroy(t_attr2);
    t_attr2 = pth_attr_of(child[3 as libc::c_int as usize]);
    pth_attr_set(t_attr2, PTH_ATTR_PRIO as libc::c_int, 1 as libc::c_int);
    pth_attr_destroy(t_attr2);
    fprintf(
        stderr,
        b"Main Loop (%ld total threads running)\n\0" as *const u8 as *const libc::c_char,
        pth_ctrl(
            ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
        ),
    );
    loop {
        n = pth_ctrl(
            ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
        );
        if !(n > 1 as libc::c_int as libc::c_long) {
            break;
        }
        fprintf(
            stderr,
            b"Main Loop (%ld total threads still running)\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
        pth_usleep(500000 as libc::c_int as libc::c_uint);
    }
    fprintf(
        stderr,
        b"Main Exit (%ld total threads running)\n\0" as *const u8 as *const libc::c_char,
        pth_ctrl(
            ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
        ),
    );
    pth_kill();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
