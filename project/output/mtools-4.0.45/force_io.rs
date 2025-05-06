#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type doscp_t;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
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
pub type mt_off_t = off_t;
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
unsafe extern "C" fn force_pio(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
    mut io: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    let mut done: i32 = 0 as i32;
    while len != 0 {
        ret = io.expect("non-null function pointer")(Stream, buf, start, len);
        if ret <= 0 as i32 as i64 {
            if done != 0 { return done as ssize_t } else { return ret }
        }
        if ret as size_t <= len {} else {
            __assert_fail(
                b"(size_t)ret <= len\0" as *const u8 as *const i8,
                b"force_io.c\0" as *const u8 as *const i8,
                45 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 107],
                    &[i8; 107],
                >(
                    b"ssize_t force_pio(Stream_t *, char *, mt_off_t, size_t, ssize_t (*)(Stream_t *, char *, mt_off_t, size_t))\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7022: {
            if ret as size_t <= len {} else {
                __assert_fail(
                    b"(size_t)ret <= len\0" as *const u8 as *const i8,
                    b"force_io.c\0" as *const u8 as *const i8,
                    45 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 107],
                        &[i8; 107],
                    >(
                        b"ssize_t force_pio(Stream_t *, char *, mt_off_t, size_t, ssize_t (*)(Stream_t *, char *, mt_off_t, size_t))\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        start = (start as u64).wrapping_add(ret as size_t) as mt_off_t as mt_off_t;
        done = (done as i64 + ret) as i32;
        len = (len as u64).wrapping_sub(ret as size_t) as size_t as size_t;
        buf = buf.offset(ret as isize);
    }
    return done as ssize_t;
}
unsafe extern "C" fn write_wrapper(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return ((*(*Stream).Class).write)
        .expect("non-null function pointer")(Stream, buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn force_write(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut len: size_t,
) -> ssize_t {
    return force_pio(
        Stream,
        buf,
        0 as i32 as mt_off_t,
        len,
        Some(
            write_wrapper
                as unsafe extern "C" fn(
                    *mut Stream_t,
                    *mut i8,
                    mt_off_t,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn force_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return force_pio(Stream, buf, start, len, (*(*Stream).Class).pwrite);
}
#[no_mangle]
pub unsafe extern "C" fn force_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return force_pio(Stream, buf, start, len, (*(*Stream).Class).pread);
}