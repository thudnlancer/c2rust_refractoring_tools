#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type pth_uctx_st;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn pth_uctx_create(_: *mut pth_uctx_t) -> i32;
    fn pth_uctx_make(
        _: pth_uctx_t,
        _: *mut i8,
        _: size_t,
        _: *const sigset_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
        _: pth_uctx_t,
    ) -> i32;
    fn pth_uctx_switch(_: pth_uctx_t, _: pth_uctx_t) -> i32;
    fn pth_uctx_destroy(_: pth_uctx_t) -> i32;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
pub type pth_uctx_t = *mut pth_uctx_st;
#[no_mangle]
pub static mut uctx: [pth_uctx_t; 10] = [0 as *const pth_uctx_st
    as *mut pth_uctx_st; 10];
#[no_mangle]
pub static mut worker_done: [i32; 10] = [0; 10];
unsafe extern "C" fn worker(mut ctx: *mut libc::c_void) {
    let mut n: i32 = ctx as i32;
    let mut i: i32 = 0 as i32;
    fprintf(stderr, b"worker #%d: enter\n\0" as *const u8 as *const i8, n);
    ::core::ptr::write_volatile(&mut i as *mut i32, 0 as i32);
    while i < 100 as i32 {
        fprintf(
            stderr,
            b"worker #%d: working (step %d)\n\0" as *const u8 as *const i8,
            n,
            i,
        );
        pth_uctx_switch(uctx[n as usize], uctx[0 as i32 as usize]);
        ::core::ptr::write_volatile(
            &mut i as *mut i32,
            ::core::ptr::read_volatile::<i32>(&i as *const i32) + 1,
        );
        ::core::ptr::read_volatile::<i32>(&i as *const i32);
    }
    ::core::ptr::write_volatile(
        &mut worker_done[n as usize] as *mut i32,
        (0 as i32 == 0) as i32,
    );
    fprintf(stderr, b"worker #%d: exit\n\0" as *const u8 as *const i8, n);
}
unsafe extern "C" fn test_working() {
    let mut i: i32 = 0;
    let mut todo: i32 = 0;
    fprintf(stderr, b"master: startup\n\0" as *const u8 as *const i8);
    fprintf(stderr, b"master: create contexts\n\0" as *const u8 as *const i8);
    pth_uctx_create(
        &mut *uctx.as_mut_ptr().offset(0 as i32 as isize) as *mut pth_uctx_t
            as *mut pth_uctx_t,
    );
    ::core::ptr::write_volatile(
        &mut worker_done[0 as i32 as usize] as *mut i32,
        0 as i32,
    );
    ::core::ptr::write_volatile(&mut i as *mut i32, 1 as i32);
    while i < 10 as i32 {
        ::core::ptr::write_volatile(&mut worker_done[i as usize] as *mut i32, 0 as i32);
        pth_uctx_create(
            &mut *uctx.as_mut_ptr().offset(i as isize) as *mut pth_uctx_t
                as *mut pth_uctx_t,
        );
        pth_uctx_make(
            uctx[i as usize],
            0 as *mut i8,
            (32 as i32 * 1024 as i32) as size_t,
            0 as *const sigset_t,
            Some(worker as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            i as *mut libc::c_void,
            uctx[0 as i32 as usize],
        );
        ::core::ptr::write_volatile(
            &mut i as *mut i32,
            ::core::ptr::read_volatile::<i32>(&i as *const i32) + 1,
        );
        ::core::ptr::read_volatile::<i32>(&i as *const i32);
    }
    loop {
        ::core::ptr::write_volatile(&mut todo as *mut i32, 0 as i32);
        ::core::ptr::write_volatile(&mut i as *mut i32, 1 as i32);
        while i < 10 as i32 {
            if worker_done[i as usize] == 0 {
                fprintf(
                    stderr,
                    b"master: switching to worker #%d\n\0" as *const u8 as *const i8,
                    i,
                );
                pth_uctx_switch(uctx[0 as i32 as usize], uctx[i as usize]);
                fprintf(
                    stderr,
                    b"master: came back from worker #%d\n\0" as *const u8 as *const i8,
                    i,
                );
                ::core::ptr::write_volatile(&mut todo as *mut i32, 1 as i32);
            }
            ::core::ptr::write_volatile(
                &mut i as *mut i32,
                ::core::ptr::read_volatile::<i32>(&i as *const i32) + 1,
            );
            ::core::ptr::read_volatile::<i32>(&i as *const i32);
        }
        if !(todo != 0) {
            break;
        }
    }
    fprintf(stderr, b"master: destroy contexts\n\0" as *const u8 as *const i8);
    ::core::ptr::write_volatile(&mut i as *mut i32, 1 as i32);
    while i < 10 as i32 {
        pth_uctx_destroy(uctx[i as usize]);
        ::core::ptr::write_volatile(
            &mut i as *mut i32,
            ::core::ptr::read_volatile::<i32>(&i as *const i32) + 1,
        );
        ::core::ptr::read_volatile::<i32>(&i as *const i32);
    }
    pth_uctx_destroy(uctx[0 as i32 as usize]);
    fprintf(stderr, b"master: exit\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub static mut stat_start: time_t = 0;
#[no_mangle]
pub static mut stat_end: time_t = 0;
#[no_mangle]
pub static mut stat_switched: i32 = 0;
unsafe extern "C" fn dummy(mut ctx: *mut libc::c_void) {
    loop {
        ::core::ptr::write_volatile(
            &mut stat_switched as *mut i32,
            ::core::ptr::read_volatile::<i32>(&stat_switched as *const i32) + 1,
        );
        ::core::ptr::read_volatile::<i32>(&stat_switched as *const i32);
        pth_uctx_switch(uctx[1 as i32 as usize], uctx[0 as i32 as usize]);
    };
}
unsafe extern "C" fn test_performance() {
    let mut i: i32 = 0;
    pth_uctx_create(
        &mut *uctx.as_mut_ptr().offset(0 as i32 as isize) as *mut pth_uctx_t
            as *mut pth_uctx_t,
    );
    pth_uctx_create(
        &mut *uctx.as_mut_ptr().offset(1 as i32 as isize) as *mut pth_uctx_t
            as *mut pth_uctx_t,
    );
    pth_uctx_make(
        uctx[1 as i32 as usize],
        0 as *mut i8,
        (32 as i32 * 1024 as i32) as size_t,
        0 as *const sigset_t,
        Some(dummy as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
        uctx[0 as i32 as usize],
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"Performing %d user-space context switches... be patient!\n\0" as *const u8
            as *const i8,
        10000000 as i32,
    );
    stat_start = time(0 as *mut time_t);
    ::core::ptr::write_volatile(&mut stat_switched as *mut i32, 0 as i32);
    ::core::ptr::write_volatile(&mut i as *mut i32, 0 as i32);
    while i < 10000000 as i32 {
        ::core::ptr::write_volatile(
            &mut stat_switched as *mut i32,
            ::core::ptr::read_volatile::<i32>(&stat_switched as *const i32) + 1,
        );
        ::core::ptr::read_volatile::<i32>(&stat_switched as *const i32);
        pth_uctx_switch(uctx[0 as i32 as usize], uctx[1 as i32 as usize]);
        ::core::ptr::write_volatile(
            &mut i as *mut i32,
            ::core::ptr::read_volatile::<i32>(&i as *const i32) + 1,
        );
        ::core::ptr::read_volatile::<i32>(&i as *const i32);
    }
    stat_end = time(0 as *mut time_t);
    pth_uctx_destroy(uctx[0 as i32 as usize]);
    pth_uctx_destroy(uctx[1 as i32 as usize]);
    fprintf(
        stderr,
        b"We required %d seconds for performing the test, so this means we can\n\0"
            as *const u8 as *const i8,
        (stat_end - stat_start) as i32,
    );
    fprintf(
        stderr,
        b"perform %d user-space context switches per second on this platform.\n\0"
            as *const u8 as *const i8,
        10000000 as i32 / (stat_end - stat_start) as i32,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    test_working();
    test_performance();
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
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
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}