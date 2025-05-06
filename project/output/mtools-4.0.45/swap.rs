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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn set_geom_pass_through(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
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
pub struct Swap_t {
    pub head: Stream_t,
}
unsafe extern "C" fn swap_buffer(mut buf: *mut i8, mut len: size_t) {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64) < len {
        let mut temp: i8 = *buf.offset(i as isize);
        *buf.offset(i as isize) = *buf.offset(i.wrapping_add(1 as i32 as u32) as isize);
        *buf.offset(i.wrapping_add(1 as i32 as u32) as isize) = temp;
        i = i.wrapping_add(2 as i32 as u32);
    }
}
unsafe extern "C" fn swap_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Swap_t = Stream as *mut Swap_t;
    let mut result: ssize_t = ((*(*(*This).head.Next).Class).pread)
        .expect("non-null function pointer")((*This).head.Next, buf, where_0, len);
    if result < 0 as i32 as i64 {
        return result;
    }
    swap_buffer(buf, result as size_t);
    return result;
}
unsafe extern "C" fn swap_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Swap_t = Stream as *mut Swap_t;
    let mut result: ssize_t = 0;
    let mut swapping: *mut i8 = malloc(len) as *mut i8;
    memcpy(swapping as *mut libc::c_void, buf as *const libc::c_void, len);
    swap_buffer(swapping, len);
    result = ((*(*(*This).head.Next).Class).pwrite)
        .expect("non-null function pointer")((*This).head.Next, swapping, where_0, len);
    free(swapping as *mut libc::c_void);
    return result;
}
static mut SwapClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                swap_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                swap_pwrite
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
pub unsafe extern "C" fn OpenSwap(mut Next: *mut Stream_t) -> *mut Stream_t {
    let mut This: *mut Swap_t = 0 as *mut Swap_t;
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Swap_t>() as u64)
        as *mut Swap_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(This as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<Swap_t>() as u64);
    init_head(&mut (*This).head, &mut SwapClass, Next);
    return &mut (*This).head;
}