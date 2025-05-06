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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn set_geom_pass_through(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> i32;
    fn adjust_tot_sectors(dev: *mut device, offset: mt_off_t, errmsg: *mut i8) -> i32;
    fn printOom();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Offset_t {
    pub head: Stream_t,
    pub offset: mt_off_t,
}
unsafe extern "C" fn offset_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Offset_t = Stream as *mut Offset_t;
    return ((*(*(*This).head.Next).Class).pread)
        .expect(
            "non-null function pointer",
        )((*This).head.Next, buf, start + (*This).offset, len);
}
unsafe extern "C" fn offset_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Offset_t = Stream as *mut Offset_t;
    return ((*(*(*This).head.Next).Class).pwrite)
        .expect(
            "non-null function pointer",
        )((*This).head.Next, buf, start + (*This).offset, len);
}
static mut OffsetClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                offset_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                offset_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: None,
            set_geom: Some(
                set_geom_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device_t,
                        *mut device_t,
                    ) -> i32,
            ),
            get_data: None,
            pre_allocate: None,
            get_dosConvert: Some(
                get_dosConvert_pass_through
                    as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn OpenOffset(
    mut Next: *mut Stream_t,
    mut dev: *mut device,
    mut offset: off_t,
    mut errmsg: *mut i8,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut This: *mut Offset_t = 0 as *mut Offset_t;
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Offset_t>() as u64)
        as *mut Offset_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<Offset_t>() as u64,
    );
    init_head(&mut (*This).head, &mut OffsetClass, Next);
    (*This).offset = offset;
    if !maxSize.is_null() {
        if (*This).offset > *maxSize {
            if !errmsg.is_null() {
                sprintf(
                    errmsg,
                    b"init: Big disks not supported\0" as *const u8 as *const i8,
                );
            }
            current_block = 3366000581002294609;
        } else {
            *maxSize -= (*This).offset;
            current_block = 1917311967535052937;
        }
    } else {
        current_block = 1917311967535052937;
    }
    match current_block {
        1917311967535052937 => {
            if !(adjust_tot_sectors(dev, (*This).offset, errmsg) < 0 as i32) {
                return &mut (*This).head;
            }
        }
        _ => {}
    }
    free(This as *mut i8 as *mut libc::c_void);
    return 0 as *mut Stream_t;
}