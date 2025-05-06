#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn rpl_fprintf(fp: *mut FILE, format: *const i8, _: ...) -> i32;
    fn read_buffer(fd: i32, buf: voidp, cnt: u32) -> i32;
    fn read_error();
    static mut method: i32;
    static mut outbuf: [uch; 0];
    static mut outcnt: u32;
    static mut bytes_in: off_t;
    static mut ifd: i32;
    static mut ofd: i32;
    static mut ifname: [i8; 0];
    static mut program_name: *mut i8;
    static mut time_stamp: timespec;
    static mut ifile_size: off_t;
    static mut level: i32;
    static mut save_orig_name: i32;
    fn flush_outbuf();
    fn getcrc() -> ulg;
    fn deflate(pack_level: i32) -> off_t;
    fn gzip_base_name(fname: *mut i8) -> *mut i8;
    fn ct_init(attr: *mut ush, method_0: *mut i32);
    fn bi_init(zipfile: file_t);
    fn updcrc(s: *const uch, n: u32) -> ulg;
    fn warning(m: *const i8);
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __syscall_slong_t = i64;
pub type voidp = *mut libc::c_void;
pub type size_t = u64;
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
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uch = u8;
pub type ush = libc::c_ushort;
pub type ulg = u64;
pub type file_t = i32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    SLOW = 2,
    FAST = 4,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::SLOW => 2,
            C2RustUnnamed::FAST => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            2 => C2RustUnnamed::SLOW,
            4 => C2RustUnnamed::FAST,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub static mut header_bytes: off_t = 0;
#[no_mangle]
pub unsafe extern "C" fn zip(mut in_0: i32, mut out: i32) -> i32 {
    let mut flags: uch = 0 as i32 as uch;
    let mut attr: ush = 0 as i32 as ush;
    let mut deflate_flags: ush = 0 as i32 as ush;
    let mut stamp: ulg = 0;
    ifd = in_0;
    ofd = out;
    outcnt = 0 as i32 as u32;
    method = 8 as i32;
    let fresh0 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh0 as isize) = (*::core::mem::transmute::<
        &[u8; 3],
        &[i8; 3],
    >(b"\x1F\x8B\0"))[0 as i32 as usize] as uch;
    if outcnt == 0x40000 as i32 as u32 {
        flush_outbuf();
    }
    let fresh1 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh1 as isize) = (*::core::mem::transmute::<
        &[u8; 3],
        &[i8; 3],
    >(b"\x1F\x8B\0"))[1 as i32 as usize] as uch;
    if outcnt == 0x40000 as i32 as u32 {
        flush_outbuf();
    }
    let fresh2 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh2 as isize) = 8 as i32 as uch;
    if outcnt == 0x40000 as i32 as u32 {
        flush_outbuf();
    }
    if save_orig_name != 0 {
        flags = (flags as i32 | 0x8 as i32) as uch;
    }
    let fresh3 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh3 as isize) = flags;
    if outcnt == 0x40000 as i32 as u32 {
        flush_outbuf();
    }
    if time_stamp.tv_nsec < 0 as i32 as i64 {
        stamp = 0 as i32 as ulg;
    } else if (0 as i32 as i64) < time_stamp.tv_sec
        && time_stamp.tv_sec <= 0xffffffff as u32 as i64
    {
        stamp = time_stamp.tv_sec as ulg;
    } else {
        warning(
            b"file timestamp out of range for gzip format\0" as *const u8 as *const i8,
        );
        stamp = 0 as i32 as ulg;
    }
    if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
        let fresh4 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh4 as isize) = (stamp & 0xffff as i32 as u64
            & 0xff as i32 as u64) as uch;
        let fresh5 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh5 as isize) = ((stamp & 0xffff as i32 as u64)
            as ush as i32 >> 8 as i32) as uch;
    } else {
        let fresh6 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh6 as isize) = (stamp & 0xffff as i32 as u64
            & 0xff as i32 as u64) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
        let fresh7 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh7 as isize) = ((stamp & 0xffff as i32 as u64)
            as ush as i32 >> 8 as i32) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
        let fresh8 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh8 as isize) = (stamp >> 16 as i32
            & 0xff as i32 as u64) as uch;
        let fresh9 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh9 as isize) = ((stamp >> 16 as i32) as ush
            as i32 >> 8 as i32) as uch;
    } else {
        let fresh10 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh10 as isize) = (stamp >> 16 as i32
            & 0xff as i32 as u64) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
        let fresh11 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh11 as isize) = ((stamp >> 16 as i32) as ush
            as i32 >> 8 as i32) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    }
    updcrc(0 as *const uch, 0 as i32 as u32);
    bi_init(out);
    ct_init(&mut attr, &mut method);
    if level == 1 as i32 {
        deflate_flags = (deflate_flags as i32 | C2RustUnnamed::FAST as i32) as ush;
    } else if level == 9 as i32 {
        deflate_flags = (deflate_flags as i32 | C2RustUnnamed::SLOW as i32) as ush;
    }
    let fresh12 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh12 as isize) = deflate_flags as uch;
    if outcnt == 0x40000 as i32 as u32 {
        flush_outbuf();
    }
    let fresh13 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh13 as isize) = 0x3 as i32 as uch;
    if outcnt == 0x40000 as i32 as u32 {
        flush_outbuf();
    }
    if save_orig_name != 0 {
        let mut p: *mut i8 = gzip_base_name(ifname.as_mut_ptr());
        loop {
            let fresh14 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh14 as isize) = *p as uch;
            if outcnt == 0x40000 as i32 as u32 {
                flush_outbuf();
            }
            let fresh15 = p;
            p = p.offset(1);
            if !(*fresh15 != 0) {
                break;
            }
        }
    }
    header_bytes = outcnt as off_t;
    deflate(level);
    if ifile_size != -(1 as i64) && bytes_in != ifile_size {
        rpl_fprintf(
            stderr,
            b"%s: %s: file size changed while zipping\n\0" as *const u8 as *const i8,
            program_name,
            ifname.as_mut_ptr(),
        );
    }
    if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
        let fresh16 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh16 as isize) = (getcrc() & 0xffff as i32 as u64
            & 0xff as i32 as u64) as uch;
        let fresh17 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh17 as isize) = ((getcrc()
            & 0xffff as i32 as u64) as ush as i32 >> 8 as i32) as uch;
    } else {
        let fresh18 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh18 as isize) = (getcrc() & 0xffff as i32 as u64
            & 0xff as i32 as u64) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
        let fresh19 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh19 as isize) = ((getcrc()
            & 0xffff as i32 as u64) as ush as i32 >> 8 as i32) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
        let fresh20 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh20 as isize) = (getcrc() >> 16 as i32
            & 0xff as i32 as u64) as uch;
        let fresh21 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh21 as isize) = ((getcrc() >> 16 as i32) as ush
            as i32 >> 8 as i32) as uch;
    } else {
        let fresh22 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh22 as isize) = (getcrc() >> 16 as i32
            & 0xff as i32 as u64) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
        let fresh23 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh23 as isize) = ((getcrc() >> 16 as i32) as ush
            as i32 >> 8 as i32) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
        let fresh24 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh24 as isize) = (bytes_in as ulg
            & 0xffff as i32 as u64 & 0xff as i32 as u64) as uch;
        let fresh25 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh25 as isize) = ((bytes_in as ulg
            & 0xffff as i32 as u64) as ush as i32 >> 8 as i32) as uch;
    } else {
        let fresh26 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh26 as isize) = (bytes_in as ulg
            & 0xffff as i32 as u64 & 0xff as i32 as u64) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
        let fresh27 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh27 as isize) = ((bytes_in as ulg
            & 0xffff as i32 as u64) as ush as i32 >> 8 as i32) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as i32 - 2 as i32) as u32 {
        let fresh28 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh28 as isize) = (bytes_in as ulg >> 16 as i32
            & 0xff as i32 as u64) as uch;
        let fresh29 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh29 as isize) = ((bytes_in as ulg >> 16 as i32)
            as ush as i32 >> 8 as i32) as uch;
    } else {
        let fresh30 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh30 as isize) = (bytes_in as ulg >> 16 as i32
            & 0xff as i32 as u64) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
        let fresh31 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh31 as isize) = ((bytes_in as ulg >> 16 as i32)
            as ush as i32 >> 8 as i32) as uch;
        if outcnt == 0x40000 as i32 as u32 {
            flush_outbuf();
        }
    }
    header_bytes += (2 as i32 * 4 as i32) as i64;
    flush_outbuf();
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn file_read(mut buf: *mut i8, mut size: u32) -> i32 {
    let mut len: u32 = 0;
    len = read_buffer(ifd, buf as voidp, size) as u32;
    if len == 0 as i32 as u32 {
        return len as i32;
    }
    if len == -(1 as i32) as u32 {
        read_error();
    }
    updcrc(buf as *mut uch, len);
    bytes_in += len as off_t;
    return len as i32;
}