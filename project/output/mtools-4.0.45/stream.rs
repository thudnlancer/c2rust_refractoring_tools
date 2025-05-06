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
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
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
pub type smt_off_t = mt_off_t;
#[no_mangle]
pub static mut batchmode: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn limitSizeToOffT(mut len: *mut size_t, mut maxLen: mt_off_t) {
    if *len > maxLen as size_t {
        *len = maxLen as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_head(
    mut Stream: *mut Stream_t,
    mut Class: *mut Class_t,
    mut Next: *mut Stream_t,
) {
    (*Stream).Class = Class;
    (*Stream).refs = 1 as i32;
    (*Stream).Next = Next;
}
#[no_mangle]
pub unsafe extern "C" fn flush_stream(mut Stream: *mut Stream_t) -> i32 {
    let mut ret: i32 = 0 as i32;
    if batchmode == 0 {
        if ((*(*Stream).Class).flush).is_some() {
            ret
                |= ((*(*Stream).Class).flush)
                    .expect("non-null function pointer")(Stream);
        }
        if !((*Stream).Next).is_null() {
            ret |= flush_stream((*Stream).Next);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn copy_stream(mut Stream: *mut Stream_t) -> *mut Stream_t {
    if !Stream.is_null() {
        (*Stream).refs += 1;
        (*Stream).refs;
    }
    return Stream;
}
#[no_mangle]
pub unsafe extern "C" fn free_stream(mut Stream: *mut *mut Stream_t) -> i32 {
    let mut ret: i32 = 0 as i32;
    if (*Stream).is_null() {
        return -(1 as i32);
    }
    (**Stream).refs -= 1;
    if (**Stream).refs == 0 {
        if ((*(**Stream).Class).flush).is_some() {
            ret
                |= ((*(**Stream).Class).flush)
                    .expect("non-null function pointer")(*Stream);
        }
        if ((*(**Stream).Class).freeFunc).is_some() {
            ret
                |= ((*(**Stream).Class).freeFunc)
                    .expect("non-null function pointer")(*Stream);
        }
        if !((**Stream).Next).is_null() {
            ret |= free_stream(&mut (**Stream).Next);
        }
        free(*Stream as *mut i8 as *mut libc::c_void);
    }
    *Stream = 0 as *mut Stream_t;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn set_geom_pass_through(
    mut Stream: *mut Stream_t,
    mut dev: *mut device_t,
    mut orig_dev: *mut device_t,
) -> i32 {
    return ((*(*(*Stream).Next).Class).set_geom)
        .expect("non-null function pointer")((*Stream).Next, dev, orig_dev);
}
#[no_mangle]
pub unsafe extern "C" fn set_geom_noop(
    mut Stream: *mut Stream_t,
    mut dev: *mut device_t,
    mut orig_dev: *mut device_t,
) -> i32 {
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn get_data_pass_through(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut i32,
    mut address: *mut uint32_t,
) -> i32 {
    return ((*(*(*Stream).Next).Class).get_data)
        .expect(
            "non-null function pointer",
        )((*Stream).Next, date, size, type_0, address);
}
#[no_mangle]
pub unsafe extern "C" fn pread_pass_through(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return ((*(*(*Stream).Next).Class).pread)
        .expect("non-null function pointer")((*Stream).Next, buf, start, len);
}
#[no_mangle]
pub unsafe extern "C" fn pwrite_pass_through(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return ((*(*(*Stream).Next).Class).pwrite)
        .expect("non-null function pointer")((*Stream).Next, buf, start, len);
}
#[no_mangle]
pub unsafe extern "C" fn get_dosConvert_pass_through(
    mut Stream: *mut Stream_t,
) -> *mut doscp_t {
    return ((*(*(*Stream).Next).Class).get_dosConvert)
        .expect("non-null function pointer")((*Stream).Next);
}
#[no_mangle]
pub unsafe extern "C" fn adjust_tot_sectors(
    mut dev: *mut device,
    mut offset: mt_off_t,
    mut errmsg: *mut i8,
) -> i32 {
    let mut offs_sectors: mt_off_t = 0;
    if (*dev).tot_sectors == 0 {
        return 0 as i32;
    }
    offs_sectors = offset
        / (if (*dev).sector_size as i32 != 0 {
            (*dev).sector_size as i32
        } else {
            512 as i32
        }) as i64;
    if offs_sectors > 0 as i32 as i64 && ((*dev).tot_sectors as i64) < offs_sectors {
        if !errmsg.is_null() {
            sprintf(
                errmsg,
                b"init: Offset bigger than base image\0" as *const u8 as *const i8,
            );
        }
        return -(1 as i32);
    }
    (*dev).tot_sectors = ((*dev).tot_sectors as u32)
        .wrapping_sub(offs_sectors as uint32_t) as uint32_t as uint32_t;
    return 0 as i32;
}