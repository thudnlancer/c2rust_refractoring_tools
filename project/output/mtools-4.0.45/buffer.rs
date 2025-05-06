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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type doscp_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
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
pub struct Buffer_t {
    pub head: Stream_t,
    pub size: size_t,
    pub dirty: i32,
    pub sectorSize: size_t,
    pub cylinderSize: size_t,
    pub ever_dirty: i32,
    pub dirty_pos: size_t,
    pub dirty_end: size_t,
    pub current: mt_off_t,
    pub cur_size: size_t,
    pub buf: *mut i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum position_t {
    OUTSIDE,
    APPEND,
    INSIDE,
    ERROR,
}
impl position_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            position_t::OUTSIDE => 0,
            position_t::APPEND => 1,
            position_t::INSIDE => 2,
            position_t::ERROR => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> position_t {
        match value {
            0 => position_t::OUTSIDE,
            1 => position_t::APPEND,
            2 => position_t::INSIDE,
            3 => position_t::ERROR,
            _ => panic!("Invalid value for position_t: {}", value),
        }
    }
}
impl AddAssign<u32> for position_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = position_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for position_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = position_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for position_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = position_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for position_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = position_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for position_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = position_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for position_t {
    type Output = position_t;
    fn add(self, rhs: u32) -> position_t {
        position_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for position_t {
    type Output = position_t;
    fn sub(self, rhs: u32) -> position_t {
        position_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for position_t {
    type Output = position_t;
    fn mul(self, rhs: u32) -> position_t {
        position_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for position_t {
    type Output = position_t;
    fn div(self, rhs: u32) -> position_t {
        position_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for position_t {
    type Output = position_t;
    fn rem(self, rhs: u32) -> position_t {
        position_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
unsafe extern "C" fn abs_pos(mut Buffer: *mut Buffer_t, mut rel: size_t) -> mt_off_t {
    return (*Buffer).current + rel as mt_off_t;
}
unsafe extern "C" fn cur_end(mut Buffer: *mut Buffer_t) -> mt_off_t {
    return abs_pos(Buffer, (*Buffer).cur_size);
}
unsafe extern "C" fn pos_to_next_full_cyl(
    mut Buffer: *mut Buffer_t,
    mut pos: mt_off_t,
) -> size_t {
    return ((*Buffer).cylinderSize)
        .wrapping_sub((pos % (*Buffer).cylinderSize as mt_off_t) as size_t);
}
unsafe extern "C" fn mt_buf_flush(mut Buffer: *mut Buffer_t) -> i32 {
    let mut ret: ssize_t = 0;
    if !((*Buffer).head.Next).is_null() {} else {
        __assert_fail(
            b"Buffer->head.Next != NULL\0" as *const u8 as *const i8,
            b"buffer.c\0" as *const u8 as *const i8,
            69 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[i8; 29],
            >(b"int mt_buf_flush(Buffer_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7360: {
        if !((*Buffer).head.Next).is_null() {} else {
            __assert_fail(
                b"Buffer->head.Next != NULL\0" as *const u8 as *const i8,
                b"buffer.c\0" as *const u8 as *const i8,
                69 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[i8; 29],
                >(b"int mt_buf_flush(Buffer_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*Buffer).dirty == 0 {
        return 0 as i32;
    }
    ret = force_pwrite(
        (*Buffer).head.Next,
        ((*Buffer).buf).offset((*Buffer).dirty_pos as isize),
        (*Buffer).current + (*Buffer).dirty_pos as mt_off_t,
        ((*Buffer).dirty_end).wrapping_sub((*Buffer).dirty_pos),
    );
    if ret < 0 as i32 as i64 {
        perror(b"buffer_flush: write\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    if ret as size_t != ((*Buffer).dirty_end).wrapping_sub((*Buffer).dirty_pos) {
        fprintf(stderr, b"buffer_flush: short write\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    (*Buffer).dirty = 0 as i32;
    (*Buffer).dirty_end = 0 as i32 as size_t;
    (*Buffer).dirty_pos = 0 as i32 as size_t;
    return 0 as i32;
}
unsafe extern "C" fn invalidate_buffer(
    mut Buffer: *mut Buffer_t,
    mut start: mt_off_t,
) -> i32 {
    if mt_buf_flush(Buffer) < 0 as i32 {
        return -(1 as i32);
    }
    (*Buffer).current = start - start % (*Buffer).sectorSize as mt_off_t;
    (*Buffer).cur_size = 0 as i32 as size_t;
    return 0 as i32;
}
unsafe extern "C" fn isInBuffer(
    mut This: *mut Buffer_t,
    mut start: mt_off_t,
    mut len: *mut size_t,
) -> position_t {
    if start >= (*This).current && start < cur_end(This) {
        if *len > ((*This).cur_size).wrapping_sub((start - (*This).current) as size_t) {
            *len = ((*This).cur_size).wrapping_sub((start - (*This).current) as size_t);
        }
        return position_t::INSIDE;
    } else if start == cur_end(This) && (*This).cur_size < (*This).size
        && *len >= (*This).sectorSize
    {
        if *len > ((*This).size).wrapping_sub((*This).cur_size) {
            *len = ((*This).size).wrapping_sub((*This).cur_size);
        }
        *len = (*len).wrapping_sub((*len).wrapping_rem((*This).sectorSize));
        return position_t::APPEND;
    } else {
        if invalidate_buffer(This, start) < 0 as i32 {
            return position_t::ERROR;
        }
        if *len
            > ((*This).cylinderSize).wrapping_sub((start - (*This).current) as size_t)
        {
            *len = ((*This).cylinderSize)
                .wrapping_sub((start - (*This).current) as size_t);
        }
        if *len > pos_to_next_full_cyl(This, (*This).current) {
            *len = pos_to_next_full_cyl(This, (*This).current);
        }
        return position_t::OUTSIDE;
    };
}
unsafe extern "C" fn buf_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut length: size_t = 0;
    let mut offset: size_t = 0;
    let mut disk_ptr: *mut i8 = 0 as *mut i8;
    let mut ret: ssize_t = 0;
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    if len == 0 {
        return 0 as i32 as ssize_t;
    }
    match isInBuffer(This, start, &mut len) as u32 {
        0 | 1 => {
            length = pos_to_next_full_cyl(This, cur_end(This));
            if length > ((*This).size).wrapping_sub((*This).cur_size) {
                length = ((*This).size).wrapping_sub((*This).cur_size);
            }
            ret = ((*(*(*This).head.Next).Class).pread)
                .expect(
                    "non-null function pointer",
                )(
                (*This).head.Next,
                ((*This).buf).offset((*This).cur_size as isize),
                (*This).current + (*This).cur_size as mt_off_t,
                length,
            );
            if ret < 0 as i32 as i64 {
                return ret;
            }
            (*This).cur_size = ((*This).cur_size as u64).wrapping_add(ret as size_t)
                as size_t as size_t;
            if ((*This).current + (*This).cur_size as mt_off_t) < start {
                fprintf(stderr, b"Short buffer fill\n\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
        }
        3 => return -(1 as i32) as ssize_t,
        2 | _ => {}
    }
    offset = (start - (*This).current) as size_t;
    disk_ptr = ((*This).buf).offset(offset as isize);
    if len > ((*This).cur_size).wrapping_sub(offset) {
        len = ((*This).cur_size).wrapping_sub(offset);
    }
    memcpy(buf as *mut libc::c_void, disk_ptr as *const libc::c_void, len);
    return len as ssize_t;
}
unsafe extern "C" fn buf_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut disk_ptr: *mut i8 = 0 as *mut i8;
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    let mut offset: size_t = 0 as i32 as size_t;
    if len == 0 {
        return 0 as i32 as ssize_t;
    }
    (*This).ever_dirty = 1 as i32;
    let mut current_block_38: u64;
    match isInBuffer(This, start, &mut len) as u32 {
        0 => {
            if start % (*This).cylinderSize as mt_off_t != 0 || len < (*This).sectorSize
            {
                let mut readSize: size_t = 0;
                let mut ret: ssize_t = 0;
                let mut bytes_read: size_t = 0;
                readSize = ((*This).cylinderSize)
                    .wrapping_sub(
                        ((*This).current % (*This).cylinderSize as mt_off_t) as size_t,
                    );
                ret = ((*(*(*This).head.Next).Class).pread)
                    .expect(
                        "non-null function pointer",
                    )((*This).head.Next, (*This).buf, (*This).current, readSize);
                if ret < 0 as i32 as i64 {
                    return ret;
                }
                bytes_read = ret as size_t;
                if bytes_read.wrapping_rem((*This).sectorSize) != 0 {
                    fprintf(
                        stderr,
                        b"Weird: read size (%ld) not a multiple of sector size (%d)\n\0"
                            as *const u8 as *const i8,
                        bytes_read,
                        (*This).sectorSize as i32,
                    );
                    bytes_read = (bytes_read as u64)
                        .wrapping_sub(bytes_read.wrapping_rem((*This).sectorSize))
                        as size_t as size_t;
                    if bytes_read == 0 as i32 as u64 {
                        fprintf(stderr, b"Nothing left\n\0" as *const u8 as *const i8);
                        exit(1 as i32);
                    }
                }
                (*This).cur_size = bytes_read;
                if (*This).cur_size == 0 {
                    memset((*This).buf as *mut libc::c_void, 0 as i32, readSize);
                    (*This).cur_size = readSize;
                }
                offset = (start - (*This).current) as size_t;
                current_block_38 = 7333393191927787629;
            } else {
                current_block_38 = 9666422233699910082;
            }
        }
        1 => {
            current_block_38 = 9666422233699910082;
        }
        2 => {
            offset = (start - (*This).current) as size_t;
            if len > ((*This).cur_size).wrapping_sub(offset) {
                len = ((*This).cur_size).wrapping_sub(offset);
            }
            current_block_38 = 7333393191927787629;
        }
        3 => return -(1 as i32) as ssize_t,
        _ => {
            current_block_38 = 7333393191927787629;
        }
    }
    match current_block_38 {
        9666422233699910082 => {
            len = len.wrapping_sub(len.wrapping_rem((*This).sectorSize));
            offset = (start - (*This).current) as size_t;
            if len > ((*This).size).wrapping_sub(offset) {
                len = ((*This).size).wrapping_sub(offset);
            }
            (*This).cur_size = ((*This).cur_size as u64).wrapping_add(len) as size_t
                as size_t;
            if ((*(*(*This).head.Next).Class).pre_allocate).is_some() {
                ((*(*(*This).head.Next).Class).pre_allocate)
                    .expect(
                        "non-null function pointer",
                    )((*This).head.Next, cur_end(This));
            }
        }
        _ => {}
    }
    disk_ptr = ((*This).buf).offset(offset as isize);
    if offset.wrapping_add(len) > (*This).cur_size {
        len = (len as u64)
            .wrapping_sub(offset.wrapping_add(len).wrapping_rem((*This).sectorSize))
            as size_t as size_t;
        (*This).cur_size = len.wrapping_add(offset);
    }
    memcpy(disk_ptr as *mut libc::c_void, buf as *const libc::c_void, len);
    if (*This).dirty == 0 || offset < (*This).dirty_pos {
        (*This).dirty_pos = offset.wrapping_sub(offset.wrapping_rem((*This).sectorSize));
    }
    if (*This).dirty == 0 || offset.wrapping_add(len) > (*This).dirty_end {
        (*This).dirty_end = offset
            .wrapping_add(len)
            .wrapping_add((*This).sectorSize)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_sub(
                offset
                    .wrapping_add(len)
                    .wrapping_add((*This).sectorSize)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_rem((*This).sectorSize),
            );
    }
    if (*This).dirty_end > (*This).cur_size {
        fprintf(
            stderr,
            b"Internal error, dirty end too big dirty_end=%x cur_size=%x len=%x offset=%d sectorSize=%x\n\0"
                as *const u8 as *const i8,
            (*This).dirty_end as u32,
            (*This).cur_size as u32,
            len as u32,
            offset as i32,
            (*This).sectorSize as i32,
        );
        fprintf(
            stderr,
            b"offset + len + grain - 1 = %x\n\0" as *const u8 as *const i8,
            offset
                .wrapping_add(len)
                .wrapping_add((*This).sectorSize)
                .wrapping_sub(1 as i32 as u64) as i32,
        );
        fprintf(
            stderr,
            b"ROUNDOWN(offset + len + grain - 1) = %x\n\0" as *const u8 as *const i8,
            offset
                .wrapping_add(len)
                .wrapping_add((*This).sectorSize)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub(
                    offset
                        .wrapping_add(len)
                        .wrapping_add((*This).sectorSize)
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_rem((*This).sectorSize),
                ) as i32,
        );
        fprintf(
            stderr,
            b"This->dirty = %d\n\0" as *const u8 as *const i8,
            (*This).dirty,
        );
        exit(1 as i32);
    }
    (*This).dirty = 1 as i32;
    return len as ssize_t;
}
unsafe extern "C" fn buf_flush(mut Stream: *mut Stream_t) -> i32 {
    let mut ret: i32 = 0;
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    if (*This).ever_dirty == 0 {
        return 0 as i32;
    }
    ret = mt_buf_flush(This);
    if ret == 0 as i32 {
        (*This).ever_dirty = 0 as i32;
    }
    return ret;
}
unsafe extern "C" fn buf_free(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    if !((*This).buf).is_null() {
        free((*This).buf as *mut libc::c_void);
    }
    (*This).buf = 0 as *mut i8;
    return 0 as i32;
}
static mut BufferClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                buf_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                buf_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(buf_flush as unsafe extern "C" fn(*mut Stream_t) -> i32),
            freeFunc: Some(buf_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
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
pub unsafe extern "C" fn buf_init(
    mut Next: *mut Stream_t,
    mut size: size_t,
    mut cylinderSize: size_t,
    mut sectorSize: size_t,
) -> *mut Stream_t {
    let mut Buffer: *mut Buffer_t = 0 as *mut Buffer_t;
    if size != 0 as i32 as u64 {} else {
        __assert_fail(
            b"size != 0\0" as *const u8 as *const i8,
            b"buffer.c\0" as *const u8 as *const i8,
            364 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8925: {
        if size != 0 as i32 as u64 {} else {
            __assert_fail(
                b"size != 0\0" as *const u8 as *const i8,
                b"buffer.c\0" as *const u8 as *const i8,
                364 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if cylinderSize != 0 as i32 as u64 {} else {
        __assert_fail(
            b"cylinderSize != 0\0" as *const u8 as *const i8,
            b"buffer.c\0" as *const u8 as *const i8,
            365 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8887: {
        if cylinderSize != 0 as i32 as u64 {} else {
            __assert_fail(
                b"cylinderSize != 0\0" as *const u8 as *const i8,
                b"buffer.c\0" as *const u8 as *const i8,
                365 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if sectorSize != 0 as i32 as u64 {} else {
        __assert_fail(
            b"sectorSize != 0\0" as *const u8 as *const i8,
            b"buffer.c\0" as *const u8 as *const i8,
            366 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8848: {
        if sectorSize != 0 as i32 as u64 {} else {
            __assert_fail(
                b"sectorSize != 0\0" as *const u8 as *const i8,
                b"buffer.c\0" as *const u8 as *const i8,
                366 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if !Next.is_null() {} else {
        __assert_fail(
            b"Next != NULL\0" as *const u8 as *const i8,
            b"buffer.c\0" as *const u8 as *const i8,
            367 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8804: {
        if !Next.is_null() {} else {
            __assert_fail(
                b"Next != NULL\0" as *const u8 as *const i8,
                b"buffer.c\0" as *const u8 as *const i8,
                367 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if size.wrapping_rem(cylinderSize) != 0 as i32 as u64 {
        fprintf(
            stderr,
            b"size not multiple of cylinder size\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    if cylinderSize.wrapping_rem(sectorSize) != 0 as i32 as u64 {
        fprintf(
            stderr,
            b"cylinder size not multiple of sector size\n\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    Buffer = calloc(1 as i32 as u64, ::core::mem::size_of::<Buffer_t>() as u64)
        as *mut Buffer_t;
    if Buffer.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*Buffer).head, &mut BufferClass, Next);
    (*Buffer).buf = malloc(size) as *mut i8;
    if ((*Buffer).buf).is_null() {
        free(Buffer as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    (*Buffer).size = size;
    (*Buffer).dirty = 0 as i32;
    (*Buffer).cylinderSize = cylinderSize;
    (*Buffer).sectorSize = sectorSize;
    (*Buffer).ever_dirty = 0 as i32;
    (*Buffer).dirty_pos = 0 as i32 as size_t;
    (*Buffer).dirty_end = 0 as i32 as size_t;
    (*Buffer).current = 0 as i64;
    (*Buffer).cur_size = 0 as i32 as size_t;
    return &mut (*Buffer).head;
}