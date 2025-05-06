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
    pub buffer: [i8; 4096],
    pub readBytes: size_t,
    pub bufPos: size_t,
    pub pendingNl: bool,
    pub eof: bool,
}
unsafe extern "C" fn read_filter(
    mut Stream: *mut Stream_t,
    mut output: *mut i8,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Filter_t = Stream as *mut Filter_t;
    let mut i: size_t = 0;
    if (*This).eof {
        return 0 as i32 as ssize_t;
    }
    i = 0 as i32 as size_t;
    while i < len && !(*This).eof {
        let mut c: i8 = 0;
        if (*This).pendingNl {
            c = '\n' as i32 as i8;
            (*This).pendingNl = 0 as i32 != 0;
        } else {
            if (*This).bufPos == (*This).readBytes {
                let mut ret: ssize_t = ((*(*(*This).head.Next).Class).read)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*This).head.Next,
                    ((*This).buffer).as_mut_ptr(),
                    4096 as i32 as size_t,
                );
                if ret < 0 as i32 as i64 {
                    if !(i == 0 as i32 as u64) {
                        break;
                    }
                    return -(1 as i32) as ssize_t;
                } else {
                    (*This).readBytes = ret as size_t;
                    (*This).bufPos = 0 as i32 as size_t;
                }
            }
            if (*This).bufPos == (*This).readBytes {
                c = '\u{1a}' as i32 as i8;
                (*This).eof = 1 as i32 != 0;
            } else {
                let fresh0 = (*This).bufPos;
                (*This).bufPos = ((*This).bufPos).wrapping_add(1);
                c = (*This).buffer[fresh0 as usize];
                if c as i32 == '\n' as i32 {
                    (*This).pendingNl = 1 as i32 != 0;
                    c = '\r' as i32 as i8;
                }
            }
        }
        *output.offset(i as isize) = c;
        i = i.wrapping_add(1);
        i;
    }
    return i as ssize_t;
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
pub unsafe extern "C" fn open_unix2dos(
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
    (*This).bufPos = 0 as i32 as size_t;
    (*This).readBytes = (*This).bufPos;
    (*This).pendingNl = 0 as i32 != 0;
    (*This).eof = 0 as i32 != 0;
    return &mut (*This).head;
}