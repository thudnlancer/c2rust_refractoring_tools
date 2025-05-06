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
    pub type doscp_t;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn fs_init(drive: i8, mode: i32, isRop: *mut i32) -> *mut Stream_t;
    fn OpenRoot(Dir: *mut Stream_t) -> *mut Stream_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: i32,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub pread: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub pwrite: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub set_geom: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> i32,
    >,
    pub get_data: Option<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut i32,
            *mut uint32_t,
        ) -> i32,
    >,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
}
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut is_initialized: i32 = 0 as i32;
static mut fss: [*mut Stream_t; 256] = [0 as *const Stream_t as *mut Stream_t; 256];
unsafe extern "C" fn finish_sc() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 256 as i32 {
        if !(fss[i as usize]).is_null() && (*fss[i as usize]).refs != 1 as i32 {
            fprintf(
                stderr,
                b"Streamcache allocation problem:%c %d\n\0" as *const u8 as *const i8,
                i,
                (*fss[i as usize]).refs,
            );
        }
        free_stream(&mut *fss.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
}
unsafe extern "C" fn init_streamcache() {
    let mut i: i32 = 0;
    if is_initialized != 0 {
        return;
    }
    is_initialized = 1 as i32;
    i = 0 as i32;
    while i < 256 as i32 {
        fss[i as usize] = 0 as *mut Stream_t;
        i += 1;
        i;
    }
    atexit(Some(finish_sc as unsafe extern "C" fn() -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn open_root_dir(
    mut drive: i8,
    mut flags: i32,
    mut isRop: *mut i32,
) -> *mut Stream_t {
    let mut Fs: *mut Stream_t = 0 as *mut Stream_t;
    init_streamcache();
    drive = ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = drive as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(drive as i32);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(drive as i32 as isize);
        }
        __res
    }) as i8;
    if !(fss[drive as u8 as usize]).is_null() {
        Fs = fss[drive as u8 as usize];
    } else {
        Fs = fs_init(drive, flags, isRop);
        if Fs.is_null() {
            fprintf(
                stderr,
                b"Cannot initialize '%c:'\n\0" as *const u8 as *const i8,
                drive as i32,
            );
            return 0 as *mut Stream_t;
        }
        fss[drive as u8 as usize] = Fs;
    }
    return OpenRoot(Fs);
}