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
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn get_data_pass_through(
        Stream: *mut Stream_t,
        date: *mut time_t,
        size: *mut mt_off_t,
        type_0: *mut i32,
        address: *mut uint32_t,
    ) -> i32;
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
pub struct Filter_t {
    pub head: Stream_t,
    pub mode: i32,
}
unsafe extern "C" fn read_filter(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Filter_t = Stream as *mut Filter_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut ret: ssize_t = 0;
    let mut newchar: i8 = 0;
    ret = ((*(*(*This).head.Next).Class).read)
        .expect("non-null function pointer")((*This).head.Next, buf, len);
    if ret < 0 as i32 as i64 {
        return ret;
    }
    j = 0 as i32 as size_t;
    i = 0 as i32 as size_t;
    while i < ret as size_t {
        if !(*buf.offset(i as isize) as i32 == '\r' as i32) {
            if *buf.offset(i as isize) as i32 == 0x1a as i32 {
                break;
            }
            newchar = *buf.offset(i as isize);
            let fresh0 = j;
            j = j.wrapping_add(1);
            *buf.offset(fresh0 as isize) = newchar;
        }
        i = i.wrapping_add(1);
        i;
    }
    return j as ssize_t;
}
static mut FilterClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: Some(
                read_filter
                    as unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t,
            ),
            write: None,
            pread: None,
            pwrite: None,
            flush: None,
            freeFunc: None,
            set_geom: None,
            get_data: Some(
                get_data_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut i32,
                        *mut uint32_t,
                    ) -> i32,
            ),
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn open_dos2unix(
    mut Next: *mut Stream_t,
    mut convertCharset: i32,
) -> *mut Stream_t {
    let mut This: *mut Filter_t = 0 as *mut Filter_t;
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Filter_t>() as u64)
        as *mut Filter_t;
    if This.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut FilterClass, Next);
    return &mut (*This).head;
}