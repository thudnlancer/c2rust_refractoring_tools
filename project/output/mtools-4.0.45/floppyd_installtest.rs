#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    fn atou16(str: *const i8) -> uint16_t;
    fn safePopenOut(command: *const *const i8, output: *mut i8, len: size_t) -> ssize_t;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
    fn setsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> i32;
    fn inet_addr(__cp: *const i8) -> in_addr_t;
    fn connect(__fd: i32, __addr: *const sockaddr, __len: socklen_t) -> i32;
    fn endhostent();
    fn gethostbyname(__name: *const i8) -> *mut hostent;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
pub type ssize_t = __ssize_t;
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
pub type socklen_t = __socklen_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_NONBLOCK = 2048,
    SOCK_CLOEXEC = 524288,
    SOCK_PACKET = 10,
    SOCK_DCCP = 6,
    SOCK_SEQPACKET = 5,
    SOCK_RDM = 4,
    SOCK_RAW = 3,
    SOCK_DGRAM = 2,
    SOCK_STREAM = 1,
}
impl __socket_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __socket_type::SOCK_NONBLOCK => 2048,
            __socket_type::SOCK_CLOEXEC => 524288,
            __socket_type::SOCK_PACKET => 10,
            __socket_type::SOCK_DCCP => 6,
            __socket_type::SOCK_SEQPACKET => 5,
            __socket_type::SOCK_RDM => 4,
            __socket_type::SOCK_RAW => 3,
            __socket_type::SOCK_DGRAM => 2,
            __socket_type::SOCK_STREAM => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> __socket_type {
        match value {
            2048 => __socket_type::SOCK_NONBLOCK,
            524288 => __socket_type::SOCK_CLOEXEC,
            10 => __socket_type::SOCK_PACKET,
            6 => __socket_type::SOCK_DCCP,
            5 => __socket_type::SOCK_SEQPACKET,
            4 => __socket_type::SOCK_RDM,
            3 => __socket_type::SOCK_RAW,
            2 => __socket_type::SOCK_DGRAM,
            1 => __socket_type::SOCK_STREAM,
            _ => panic!("Invalid value for __socket_type: {}", value),
        }
    }
}
impl AddAssign<u32> for __socket_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for __socket_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for __socket_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for __socket_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for __socket_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for __socket_type {
    type Output = __socket_type;
    fn add(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for __socket_type {
    type Output = __socket_type;
    fn sub(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for __socket_type {
    type Output = __socket_type;
    fn mul(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for __socket_type {
    type Output = __socket_type;
    fn div(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for __socket_type {
    type Output = __socket_type;
    fn rem(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut i8,
    pub h_aliases: *mut *mut i8,
    pub h_addrtype: i32,
    pub h_length: i32,
    pub h_addr_list: *mut *mut i8,
}
pub type Byte = uint8_t;
pub type Dword = uint32_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum FloppydOpcodes {
    OP_SEEK64 = 8,
    OP_OPRW = 7,
    OP_OPRO = 6,
    OP_IOCTL = 5,
    OP_CLOSE = 4,
    OP_FLUSH = 3,
    OP_SEEK = 2,
    OP_WRITE = 1,
    OP_READ = 0,
}
impl FloppydOpcodes {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            FloppydOpcodes::OP_SEEK64 => 8,
            FloppydOpcodes::OP_OPRW => 7,
            FloppydOpcodes::OP_OPRO => 6,
            FloppydOpcodes::OP_IOCTL => 5,
            FloppydOpcodes::OP_CLOSE => 4,
            FloppydOpcodes::OP_FLUSH => 3,
            FloppydOpcodes::OP_SEEK => 2,
            FloppydOpcodes::OP_WRITE => 1,
            FloppydOpcodes::OP_READ => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> FloppydOpcodes {
        match value {
            8 => FloppydOpcodes::OP_SEEK64,
            7 => FloppydOpcodes::OP_OPRW,
            6 => FloppydOpcodes::OP_OPRO,
            5 => FloppydOpcodes::OP_IOCTL,
            4 => FloppydOpcodes::OP_CLOSE,
            3 => FloppydOpcodes::OP_FLUSH,
            2 => FloppydOpcodes::OP_SEEK,
            1 => FloppydOpcodes::OP_WRITE,
            0 => FloppydOpcodes::OP_READ,
            _ => panic!("Invalid value for FloppydOpcodes: {}", value),
        }
    }
}
impl AddAssign<u32> for FloppydOpcodes {
    fn add_assign(&mut self, rhs: u32) {
        *self = FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for FloppydOpcodes {
    fn sub_assign(&mut self, rhs: u32) {
        *self = FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for FloppydOpcodes {
    fn mul_assign(&mut self, rhs: u32) {
        *self = FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for FloppydOpcodes {
    fn div_assign(&mut self, rhs: u32) {
        *self = FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for FloppydOpcodes {
    fn rem_assign(&mut self, rhs: u32) {
        *self = FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for FloppydOpcodes {
    type Output = FloppydOpcodes;
    fn add(self, rhs: u32) -> FloppydOpcodes {
        FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for FloppydOpcodes {
    type Output = FloppydOpcodes;
    fn sub(self, rhs: u32) -> FloppydOpcodes {
        FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for FloppydOpcodes {
    type Output = FloppydOpcodes;
    fn mul(self, rhs: u32) -> FloppydOpcodes {
        FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for FloppydOpcodes {
    type Output = FloppydOpcodes;
    fn div(self, rhs: u32) -> FloppydOpcodes {
        FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for FloppydOpcodes {
    type Output = FloppydOpcodes;
    fn rem(self, rhs: u32) -> FloppydOpcodes {
        FloppydOpcodes::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AuthErrorsEnum {
    AUTH_IO_ERROR = 6,
    AUTH_BADPACKET = 5,
    AUTH_DEVLOCKED = 4,
    AUTH_WRONGVERSION = 3,
    AUTH_AUTHFAILED = 2,
    AUTH_PACKETOVERSIZE = 1,
    AUTH_SUCCESS = 0,
}
impl AuthErrorsEnum {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AuthErrorsEnum::AUTH_IO_ERROR => 6,
            AuthErrorsEnum::AUTH_BADPACKET => 5,
            AuthErrorsEnum::AUTH_DEVLOCKED => 4,
            AuthErrorsEnum::AUTH_WRONGVERSION => 3,
            AuthErrorsEnum::AUTH_AUTHFAILED => 2,
            AuthErrorsEnum::AUTH_PACKETOVERSIZE => 1,
            AuthErrorsEnum::AUTH_SUCCESS => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> AuthErrorsEnum {
        match value {
            6 => AuthErrorsEnum::AUTH_IO_ERROR,
            5 => AuthErrorsEnum::AUTH_BADPACKET,
            4 => AuthErrorsEnum::AUTH_DEVLOCKED,
            3 => AuthErrorsEnum::AUTH_WRONGVERSION,
            2 => AuthErrorsEnum::AUTH_AUTHFAILED,
            1 => AuthErrorsEnum::AUTH_PACKETOVERSIZE,
            0 => AuthErrorsEnum::AUTH_SUCCESS,
            _ => panic!("Invalid value for AuthErrorsEnum: {}", value),
        }
    }
}
impl AddAssign<u32> for AuthErrorsEnum {
    fn add_assign(&mut self, rhs: u32) {
        *self = AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for AuthErrorsEnum {
    fn sub_assign(&mut self, rhs: u32) {
        *self = AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for AuthErrorsEnum {
    fn mul_assign(&mut self, rhs: u32) {
        *self = AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for AuthErrorsEnum {
    fn div_assign(&mut self, rhs: u32) {
        *self = AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for AuthErrorsEnum {
    fn rem_assign(&mut self, rhs: u32) {
        *self = AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for AuthErrorsEnum {
    type Output = AuthErrorsEnum;
    fn add(self, rhs: u32) -> AuthErrorsEnum {
        AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for AuthErrorsEnum {
    type Output = AuthErrorsEnum;
    fn sub(self, rhs: u32) -> AuthErrorsEnum {
        AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for AuthErrorsEnum {
    type Output = AuthErrorsEnum;
    fn mul(self, rhs: u32) -> AuthErrorsEnum {
        AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for AuthErrorsEnum {
    type Output = AuthErrorsEnum;
    fn div(self, rhs: u32) -> AuthErrorsEnum {
        AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for AuthErrorsEnum {
    type Output = AuthErrorsEnum;
    fn rem(self, rhs: u32) -> AuthErrorsEnum {
        AuthErrorsEnum::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
static mut AuthErrors: [*const i8; 5] = [
    b"Auth success!\0" as *const u8 as *const i8,
    b"Auth failed: Packet oversized!\0" as *const u8 as *const i8,
    b"Auth failed: X-Cookie doesn't match!\0" as *const u8 as *const i8,
    b"Auth failed: Wrong transmission protocol version!\0" as *const u8 as *const i8,
    b"Auth failed: Device locked!\0" as *const u8 as *const i8,
];
#[inline]
unsafe extern "C" fn byte2dword(mut val: *mut Byte) -> Dword {
    let mut l: Dword = 0;
    l = (((*val.offset(0 as i32 as isize) as i32) << 24 as i32)
        + ((*val.offset(1 as i32 as isize) as i32) << 16 as i32)
        + ((*val.offset(2 as i32 as isize) as i32) << 8 as i32)
        + *val.offset(3 as i32 as isize) as i32) as Dword;
    return l;
}
unsafe extern "C" fn dword2byte(mut parm: Dword, mut rval: *mut Byte) {
    *rval.offset(0 as i32 as isize) = (parm >> 24 as i32 & 0xff as i32 as u32) as Byte;
    *rval.offset(1 as i32 as isize) = (parm >> 16 as i32 & 0xff as i32 as u32) as Byte;
    *rval.offset(2 as i32 as isize) = (parm >> 8 as i32 & 0xff as i32 as u32) as Byte;
    *rval.offset(3 as i32 as isize) = (parm & 0xff as i32 as u32) as Byte;
}
#[inline]
unsafe extern "C" fn read_dword(mut handle: i32) -> Dword {
    let mut val: [Byte; 4] = [0; 4];
    if read(handle, val.as_mut_ptr() as *mut i8 as *mut libc::c_void, 4 as i32 as size_t)
        < 4 as i32 as i64
    {
        return -(1 as i32) as Dword;
    }
    return byte2dword(val.as_mut_ptr());
}
unsafe extern "C" fn write_dword(mut handle: i32, mut parm: Dword) -> i32 {
    let mut val: [Byte; 4] = [0; 4];
    dword2byte(parm, val.as_mut_ptr());
    if write(handle, val.as_mut_ptr() as *const libc::c_void, 4 as i32 as size_t)
        < 4 as i32 as i64
    {
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn authenticate_to_floppyd(
    mut fullauth: i8,
    mut sock: i32,
    mut display: *mut i8,
    mut protoversion: uint32_t,
) -> uint32_t {
    let mut filelen: uint32_t = 0 as i32 as uint32_t;
    let mut buf: [Byte; 16] = [0; 16];
    let mut command: [*const i8; 6] = [
        b"xauth\0" as *const u8 as *const i8,
        b"xauth\0" as *const u8 as *const i8,
        b"extract\0" as *const u8 as *const i8,
        b"-\0" as *const u8 as *const i8,
        0 as *const i8,
        0 as *const i8,
    ];
    let mut xcookie: *mut i8 = 0 as *mut i8;
    let mut errcode: Dword = 0;
    let mut bytesRead: uint32_t = 0;
    let mut cap: uint32_t = 0 as i32 as uint32_t;
    if fullauth != 0 {
        command[4 as i32 as usize] = display;
        filelen = strlen(display) as uint32_t;
        filelen = (filelen as u32).wrapping_add(100 as i32 as u32) as uint32_t
            as uint32_t;
        xcookie = safe_malloc(filelen.wrapping_add(4 as i32 as u32) as size_t)
            as *mut i8;
        filelen = safePopenOut(
            command.as_mut_ptr(),
            xcookie.offset(4 as i32 as isize),
            filelen as size_t,
        ) as uint32_t;
        if filelen < 1 as i32 as u32 {
            return AuthErrorsEnum::AUTH_AUTHFAILED as i32 as uint32_t;
        }
    }
    dword2byte(4 as i32 as Dword, buf.as_mut_ptr());
    dword2byte(protoversion, buf.as_mut_ptr().offset(4 as i32 as isize));
    if write(sock, buf.as_mut_ptr() as *const libc::c_void, 8 as i32 as size_t)
        < 8 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32 as uint32_t;
    }
    bytesRead = read_dword(sock);
    if bytesRead != 4 as i32 as u32 && bytesRead != 12 as i32 as u32 {
        return AuthErrorsEnum::AUTH_WRONGVERSION as i32 as uint32_t;
    }
    errcode = read_dword(sock);
    if errcode != AuthErrorsEnum::AUTH_SUCCESS as i32 as u32 {
        return errcode;
    }
    protoversion = 10 as i32 as uint32_t;
    if bytesRead >= 12 as i32 as u32 {
        protoversion = read_dword(sock);
        cap = read_dword(sock);
    }
    fprintf(stderr, b"Protocol Version=%d\n\0" as *const u8 as *const i8, protoversion);
    if protoversion >= 11 as i32 as u32 {
        fprintf(
            stderr,
            b"Capabilities:%s%s\n\0" as *const u8 as *const i8,
            if cap & 1 as i32 as u32 != 0 {
                b" ExplicitOpen\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if cap & 2 as i32 as u32 != 0 {
                b" LargeFiles\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
    }
    if fullauth != 0 {
        dword2byte(filelen, xcookie as *mut Byte);
        if write(
            sock,
            xcookie as *const libc::c_void,
            filelen.wrapping_add(4 as i32 as u32) as size_t,
        ) < filelen.wrapping_add(4 as i32 as u32) as ssize_t
        {
            return AuthErrorsEnum::AUTH_IO_ERROR as i32 as uint32_t;
        }
        if read_dword(sock) != 4 as i32 as u32 {
            return AuthErrorsEnum::AUTH_PACKETOVERSIZE as i32 as uint32_t;
        }
        errcode = read_dword(sock);
    }
    return errcode;
}
unsafe extern "C" fn get_host_and_port(
    mut name: *const i8,
    mut hostname: *mut *mut i8,
    mut display: *mut *mut i8,
    mut port: *mut uint16_t,
) -> i32 {
    let mut newname: *mut i8 = strdup(name);
    let mut p: *mut i8 = 0 as *mut i8;
    let mut p2: *mut i8 = 0 as *mut i8;
    p = newname;
    while *p as i32 != '/' as i32 && *p as i32 != 0 {
        p = p.offset(1);
        p;
    }
    p2 = p;
    if *p != 0 {
        p = p.offset(1);
        p;
    }
    *p2 = 0 as i32 as i8;
    *port = atou16(p);
    if *port as i32 == 0 as i32 {
        *port = 5703 as i32 as uint16_t;
    }
    *display = strdup(newname);
    p = newname;
    while *p as i32 != ':' as i32 && *p as i32 != 0 {
        p = p.offset(1);
        p;
    }
    p2 = p;
    if *p != 0 {
        p = p.offset(1);
        p;
    }
    *p2 = 0 as i32 as i8;
    *port = (*port as i32 + atoi(p)) as uint16_t;
    if *newname == 0 || strcmp(newname, b"unix\0" as *const u8 as *const i8) == 0 as i32
    {
        free(newname as *mut libc::c_void);
        newname = strdup(b"localhost\0" as *const u8 as *const i8);
    }
    *hostname = newname;
    return 1 as i32;
}
unsafe extern "C" fn getipaddress(mut ipaddr: *mut i8) -> in_addr_t {
    let mut host: *mut hostent = 0 as *mut hostent;
    let mut ip: in_addr_t = 0;
    ip = inet_addr(ipaddr);
    if ip == 0xffffffff as u32
        && strcmp(ipaddr, b"255.255.255.255\0" as *const u8 as *const i8) != 0 as i32
    {
        host = gethostbyname(ipaddr);
        if !host.is_null() {
            memcpy(
                &mut ip as *mut in_addr_t as *mut libc::c_void,
                *((*host).h_addr_list).offset(0 as i32 as isize) as *const libc::c_void,
                ::core::mem::size_of::<in_addr_t>() as u64,
            );
        }
        endhostent();
    }
    return ip;
}
unsafe extern "C" fn connect_to_server(mut ip: in_addr_t, mut port: uint16_t) -> i32 {
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sock: i32 = 0;
    sock = socket(2 as i32, __socket_type::SOCK_STREAM as i32, 0 as i32);
    if sock < 0 as i32 {
        return -(1 as i32);
    }
    addr.sin_family = 2 as i32 as sa_family_t;
    addr.sin_port = ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = port;
        if 0 != 0 {
            __v = (__x as i32 >> 8 as i32 & 0xff as i32
                | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
        } else {
            let fresh0 = &mut __v;
            let fresh1;
            let fresh2 = __x;
            asm!(
                "rorw $8, {0:x}", inlateout(reg)
                c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                options(pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
    });
    addr.sin_addr.s_addr = ip;
    if connect(
        sock,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as u64 as socklen_t,
    ) < 0 as i32
    {
        return -(1 as i32);
    }
    let mut on: i32 = 1 as i32;
    setsockopt(
        0 as i32,
        1 as i32,
        9 as i32,
        &mut on as *mut i32 as *mut i8 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    );
    return sock;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut hostname: *mut i8 = 0 as *mut i8;
    let mut display: *mut i8 = 0 as *mut i8;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut port: uint16_t = 0;
    let mut sock: i32 = 0;
    let mut reply: uint32_t = 0;
    let mut rval: i32 = 0;
    let mut protoversion: uint32_t = 0;
    let mut fullauth: i8 = 0 as i32 as i8;
    let mut opcode: Byte = FloppydOpcodes::OP_CLOSE as i32 as Byte;
    if argc < 2 as i32 {
        puts(
            b"Usage: floppyd_installtest [-f] Connect-String\n-f\tDo full X-Cookie-Authentication\0"
                as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    name = *argv.offset(1 as i32 as isize);
    if strcmp(name, b"-f\0" as *const u8 as *const i8) == 0 as i32 {
        fullauth = 1 as i32 as i8;
        name = *argv.offset(2 as i32 as isize);
    }
    rval = get_host_and_port(name, &mut hostname, &mut display, &mut port);
    if rval == 0 {
        return -(1 as i32);
    }
    sock = connect_to_server(getipaddress(hostname), port);
    if sock == -(1 as i32) {
        fprintf(
            stderr,
            b"Can't connect to floppyd server on %s, port %i!\n\0" as *const u8
                as *const i8,
            hostname,
            port as i32,
        );
        return -(1 as i32);
    }
    protoversion = 11 as i32 as uint32_t;
    loop {
        reply = authenticate_to_floppyd(fullauth, sock, display, protoversion);
        if protoversion == 10 as i32 as u32 {
            break;
        }
        if !(reply == AuthErrorsEnum::AUTH_WRONGVERSION as i32 as u32) {
            break;
        }
        protoversion = 10 as i32 as uint32_t;
    }
    if reply != 0 as i32 as u32 {
        fprintf(
            stderr,
            b"Connection to floppyd failed:\n%s\n\0" as *const u8 as *const i8,
            AuthErrors[reply as usize],
        );
        return -(1 as i32);
    }
    free(hostname as *mut libc::c_void);
    free(display as *mut libc::c_void);
    if write_dword(sock, 1 as i32 as Dword) < 0 as i32 {
        fprintf(
            stderr,
            b"Short write to floppyd:\n%s\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    if write(sock, &mut opcode as *mut Byte as *const libc::c_void, 1 as i32 as size_t)
        < 0 as i32 as i64
    {
        fprintf(
            stderr,
            b"Short write to floppyd:\n%s\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    close(sock);
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}