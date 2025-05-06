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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type doscp_t;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn str_to_off_with_end(str: *const i8, endp: *mut *mut i8) -> mt_off_t;
    fn limitSizeToOffT(len: *mut size_t, maxLen: mt_off_t);
    fn set_geom_pass_through(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> i32;
    fn adjust_tot_sectors(dev: *mut device, offset: mt_off_t, errmsg: *mut i8) -> i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn printOom();
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Remap_t {
    pub head: Stream_t,
    pub map: *mut map,
    pub mapSize: i32,
    pub net_offset: mt_off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map {
    pub orig: mt_off_t,
    pub remapped: mt_off_t,
    pub type_0: map_type_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum map_type_t {
    DATA,
    ZERO,
    SKIP,
    POS,
}
impl map_type_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            map_type_t::DATA => 0,
            map_type_t::ZERO => 1,
            map_type_t::SKIP => 2,
            map_type_t::POS => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> map_type_t {
        match value {
            0 => map_type_t::DATA,
            1 => map_type_t::ZERO,
            2 => map_type_t::SKIP,
            3 => map_type_t::POS,
            _ => panic!("Invalid value for map_type_t: {}", value),
        }
    }
}
impl AddAssign<u32> for map_type_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = map_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for map_type_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = map_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for map_type_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = map_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for map_type_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = map_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for map_type_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = map_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for map_type_t {
    type Output = map_type_t;
    fn add(self, rhs: u32) -> map_type_t {
        map_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for map_type_t {
    type Output = map_type_t;
    fn sub(self, rhs: u32) -> map_type_t {
        map_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for map_type_t {
    type Output = map_type_t;
    fn mul(self, rhs: u32) -> map_type_t {
        map_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for map_type_t {
    type Output = map_type_t;
    fn div(self, rhs: u32) -> map_type_t {
        map_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for map_type_t {
    type Output = map_type_t;
    fn rem(self, rhs: u32) -> map_type_t {
        map_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
unsafe extern "C" fn remap(
    mut This: *mut Remap_t,
    mut start: *mut mt_off_t,
    mut len: *mut size_t,
) -> map_type_t {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*This).mapSize - 1 as i32 {
        if *start < (*((*This).map).offset((i + 1 as i32) as isize)).remapped {
            limitSizeToOffT(
                len,
                (*((*This).map).offset((i + 1 as i32) as isize)).remapped - *start,
            );
            break;
        } else {
            i += 1;
            i;
        }
    }
    *start = *start - (*((*This).map).offset(i as isize)).remapped
        + (*((*This).map).offset(i as isize)).orig;
    return (*((*This).map).offset(i as isize)).type_0;
}
unsafe extern "C" fn remap_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Remap_t = Stream as *mut Remap_t;
    if remap(This, &mut start, &mut len) as u32 == map_type_t::DATA as i32 as u32 {
        return ((*(*(*This).head.Next).Class).pread)
            .expect("non-null function pointer")((*This).head.Next, buf, start, len)
    } else {
        memset(buf as *mut libc::c_void, 0 as i32, len);
        return len as ssize_t;
    };
}
unsafe extern "C" fn remap_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Remap_t = Stream as *mut Remap_t;
    if remap(This, &mut start, &mut len) as u32 == map_type_t::DATA as i32 as u32 {
        return ((*(*(*This).head.Next).Class).pwrite)
            .expect("non-null function pointer")((*This).head.Next, buf, start, len)
    } else {
        let mut i: u32 = 0;
        i = 0 as i32 as u32;
        while (i as u64) < len {
            if *buf.offset(i as isize) != 0 {
                fprintf(
                    stderr,
                    b"Bad data written to unmapped sectors\n\0" as *const u8 as *const i8,
                );
                *__errno_location() = 14 as i32;
                return -(1 as i32) as ssize_t;
            }
            i = i.wrapping_add(1);
            i;
        }
        return len as ssize_t;
    };
}
unsafe extern "C" fn remap_free(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut Remap_t = Stream as *mut Remap_t;
    if !((*This).map).is_null() {
        free((*This).map as *mut libc::c_void);
    }
    return 0 as i32;
}
static mut RemapClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                remap_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                remap_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: Some(remap_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
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
unsafe extern "C" fn process_map(
    mut This: *mut Remap_t,
    mut ptr: *const i8,
    mut countOnly: i32,
    mut errmsg: *mut i8,
) -> i32 {
    let mut orig: mt_off_t = 0 as i32 as mt_off_t;
    let mut remapped: mt_off_t = 0 as i32 as mt_off_t;
    let mut count: i32 = 0 as i32;
    let mut atEnd: i32 = 0 as i32;
    let mut eptr: *mut i8 = 0 as *mut i8;
    while atEnd == 0 {
        let mut len: mt_off_t = 0;
        let mut type_0: map_type_t = map_type_t::DATA;
        if *ptr as i32 == '\0' as i32 {
            type_0 = map_type_t::DATA;
            atEnd = 1 as i32;
        } else if strncmp(ptr, b"skip\0" as *const u8 as *const i8, 4 as i32 as u64) == 0
        {
            type_0 = map_type_t::SKIP;
            ptr = ptr.offset(4 as i32 as isize);
        } else if strncmp(ptr, b"zero\0" as *const u8 as *const i8, 4 as i32 as u64) == 0
        {
            type_0 = map_type_t::ZERO;
            ptr = ptr.offset(4 as i32 as isize);
        } else if strncmp(ptr, b"pos\0" as *const u8 as *const i8, 3 as i32 as u64) == 0
        {
            type_0 = map_type_t::POS;
            ptr = ptr.offset(3 as i32 as isize);
        } else {
            type_0 = map_type_t::DATA;
        }
        len = str_to_off_with_end(ptr, &mut eptr);
        ptr = eptr;
        match *ptr as i32 {
            0 => {}
            44 => {
                ptr = ptr.offset(1);
                ptr;
            }
            _ => {
                sprintf(errmsg, b"Bad number %s\n\0" as *const u8 as *const i8, ptr);
                return -(1 as i32);
            }
        }
        if type_0 as u32 == map_type_t::POS as i32 as u32 {
            orig = len;
        } else {
            if type_0 as u32 != map_type_t::SKIP as i32 as u32 {
                if countOnly == 0 {
                    let mut m: *mut map = ((*This).map).offset(count as isize);
                    (*m).orig = orig;
                    (*m).remapped = remapped;
                    (*m).type_0 = type_0;
                }
                remapped += len;
                count += 1;
                count;
            }
            if type_0 as u32 != map_type_t::ZERO as i32 as u32 {
                orig += len;
            }
        }
    }
    (*This).net_offset = orig - remapped;
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn Remap(
    mut Next: *mut Stream_t,
    mut dev: *mut device,
    mut errmsg: *mut i8,
) -> *mut Stream_t {
    let mut This: *mut Remap_t = 0 as *mut Remap_t;
    let mut nrItems: i32 = 0 as i32;
    let mut map: *const i8 = (*dev).data_map;
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Remap_t>() as u64)
        as *mut Remap_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<Remap_t>() as u64,
    );
    init_head(&mut (*This).head, &mut RemapClass, Next);
    nrItems = process_map(This, map, 1 as i32, errmsg);
    if nrItems < 0 as i32 {
        free(This as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    (*This).map = calloc(nrItems as size_t, ::core::mem::size_of::<map>() as u64)
        as *mut map;
    if ((*This).map).is_null() {
        printOom();
    } else {
        process_map(This, map, 0 as i32, errmsg);
        if adjust_tot_sectors(dev, (*This).net_offset, errmsg) < 0 as i32 {
            free((*This).map as *mut libc::c_void);
        } else {
            (*This).mapSize = nrItems;
            return &mut (*This).head;
        }
    }
    free(This as *mut libc::c_void);
    printOom();
    return 0 as *mut Stream_t;
}