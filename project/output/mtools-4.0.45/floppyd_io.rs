use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type doscp_t;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
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
    fn strtosi(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> i32;
    fn strtou16(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint16_t;
    fn printOom();
    fn safePopenOut(command: *const *const i8, output: *mut i8, len: size_t) -> ssize_t;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn set_geom_noop(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> i32;
    static max_off_t_31: mt_off_t;
    static max_off_t_seek: mt_off_t;
    fn connect(__fd: i32, __addr: *const sockaddr, __len: socklen_t) -> i32;
    fn setsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> i32;
    fn shutdown(__fd: i32, __how: i32) -> i32;
    fn inet_addr(__cp: *const i8) -> in_addr_t;
    fn endhostent();
    fn gethostbyname(__name: *const i8) -> *mut hostent;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    IPPROTO_MAX = 256,
    IPPROTO_RAW = 255,
    IPPROTO_MPLS = 137,
    IPPROTO_UDPLITE = 136,
    IPPROTO_SCTP = 132,
    IPPROTO_COMP = 108,
    IPPROTO_PIM = 103,
    IPPROTO_ENCAP = 98,
    IPPROTO_BEETPH = 94,
    IPPROTO_MTP = 92,
    IPPROTO_AH = 51,
    IPPROTO_ESP = 50,
    IPPROTO_GRE = 47,
    IPPROTO_RSVP = 46,
    IPPROTO_IPV6 = 41,
    IPPROTO_DCCP = 33,
    IPPROTO_TP = 29,
    IPPROTO_IDP = 22,
    IPPROTO_UDP = 17,
    IPPROTO_PUP = 12,
    IPPROTO_EGP = 8,
    IPPROTO_TCP = 6,
    IPPROTO_IPIP = 4,
    IPPROTO_IGMP = 2,
    IPPROTO_ICMP = 1,
    IPPROTO_IP = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::IPPROTO_MAX => 256,
            C2RustUnnamed::IPPROTO_RAW => 255,
            C2RustUnnamed::IPPROTO_MPLS => 137,
            C2RustUnnamed::IPPROTO_UDPLITE => 136,
            C2RustUnnamed::IPPROTO_SCTP => 132,
            C2RustUnnamed::IPPROTO_COMP => 108,
            C2RustUnnamed::IPPROTO_PIM => 103,
            C2RustUnnamed::IPPROTO_ENCAP => 98,
            C2RustUnnamed::IPPROTO_BEETPH => 94,
            C2RustUnnamed::IPPROTO_MTP => 92,
            C2RustUnnamed::IPPROTO_AH => 51,
            C2RustUnnamed::IPPROTO_ESP => 50,
            C2RustUnnamed::IPPROTO_GRE => 47,
            C2RustUnnamed::IPPROTO_RSVP => 46,
            C2RustUnnamed::IPPROTO_IPV6 => 41,
            C2RustUnnamed::IPPROTO_DCCP => 33,
            C2RustUnnamed::IPPROTO_TP => 29,
            C2RustUnnamed::IPPROTO_IDP => 22,
            C2RustUnnamed::IPPROTO_UDP => 17,
            C2RustUnnamed::IPPROTO_PUP => 12,
            C2RustUnnamed::IPPROTO_EGP => 8,
            C2RustUnnamed::IPPROTO_TCP => 6,
            C2RustUnnamed::IPPROTO_IPIP => 4,
            C2RustUnnamed::IPPROTO_IGMP => 2,
            C2RustUnnamed::IPPROTO_ICMP => 1,
            C2RustUnnamed::IPPROTO_IP => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            256 => C2RustUnnamed::IPPROTO_MAX,
            255 => C2RustUnnamed::IPPROTO_RAW,
            137 => C2RustUnnamed::IPPROTO_MPLS,
            136 => C2RustUnnamed::IPPROTO_UDPLITE,
            132 => C2RustUnnamed::IPPROTO_SCTP,
            108 => C2RustUnnamed::IPPROTO_COMP,
            103 => C2RustUnnamed::IPPROTO_PIM,
            98 => C2RustUnnamed::IPPROTO_ENCAP,
            94 => C2RustUnnamed::IPPROTO_BEETPH,
            92 => C2RustUnnamed::IPPROTO_MTP,
            51 => C2RustUnnamed::IPPROTO_AH,
            50 => C2RustUnnamed::IPPROTO_ESP,
            47 => C2RustUnnamed::IPPROTO_GRE,
            46 => C2RustUnnamed::IPPROTO_RSVP,
            41 => C2RustUnnamed::IPPROTO_IPV6,
            33 => C2RustUnnamed::IPPROTO_DCCP,
            29 => C2RustUnnamed::IPPROTO_TP,
            22 => C2RustUnnamed::IPPROTO_IDP,
            17 => C2RustUnnamed::IPPROTO_UDP,
            12 => C2RustUnnamed::IPPROTO_PUP,
            8 => C2RustUnnamed::IPPROTO_EGP,
            6 => C2RustUnnamed::IPPROTO_TCP,
            4 => C2RustUnnamed::IPPROTO_IPIP,
            2 => C2RustUnnamed::IPPROTO_IGMP,
            1 => C2RustUnnamed::IPPROTO_ICMP,
            0 => C2RustUnnamed::IPPROTO_IP,
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
pub type Qword = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RemoteFile_t {
    pub head: Stream_t,
    pub fd: i32,
    pub offset: mt_off_t,
    pub lastwhere: mt_off_t,
    pub size: mt_off_t,
    pub version: u32,
    pub capabilities: u32,
    pub drive: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AuthErrorsEnum {
    AUTH_IO_ERROR = 6,
    AUTH_WRONGVERSION = 3,
    AUTH_PACKETOVERSIZE = 1,
    AUTH_SUCCESS = 0,
    AUTH_AUTHFAILED = 2,
    AUTH_BADPACKET = 5,
    AUTH_DEVLOCKED = 4,
}
impl AuthErrorsEnum {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AuthErrorsEnum::AUTH_IO_ERROR => 6,
            AuthErrorsEnum::AUTH_WRONGVERSION => 3,
            AuthErrorsEnum::AUTH_PACKETOVERSIZE => 1,
            AuthErrorsEnum::AUTH_SUCCESS => 0,
            AuthErrorsEnum::AUTH_AUTHFAILED => 2,
            AuthErrorsEnum::AUTH_BADPACKET => 5,
            AuthErrorsEnum::AUTH_DEVLOCKED => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> AuthErrorsEnum {
        match value {
            6 => AuthErrorsEnum::AUTH_IO_ERROR,
            3 => AuthErrorsEnum::AUTH_WRONGVERSION,
            1 => AuthErrorsEnum::AUTH_PACKETOVERSIZE,
            0 => AuthErrorsEnum::AUTH_SUCCESS,
            2 => AuthErrorsEnum::AUTH_AUTHFAILED,
            5 => AuthErrorsEnum::AUTH_BADPACKET,
            4 => AuthErrorsEnum::AUTH_DEVLOCKED,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum FloppydOpcodes {
    OP_OPRW = 7,
    OP_OPRO = 6,
    OP_CLOSE = 4,
    OP_FLUSH = 3,
    OP_WRITE = 1,
    OP_SEEK = 2,
    OP_SEEK64 = 8,
    OP_READ = 0,
    OP_IOCTL = 5,
}
impl FloppydOpcodes {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            FloppydOpcodes::OP_OPRW => 7,
            FloppydOpcodes::OP_OPRO => 6,
            FloppydOpcodes::OP_CLOSE => 4,
            FloppydOpcodes::OP_FLUSH => 3,
            FloppydOpcodes::OP_WRITE => 1,
            FloppydOpcodes::OP_SEEK => 2,
            FloppydOpcodes::OP_SEEK64 => 8,
            FloppydOpcodes::OP_READ => 0,
            FloppydOpcodes::OP_IOCTL => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> FloppydOpcodes {
        match value {
            7 => FloppydOpcodes::OP_OPRW,
            6 => FloppydOpcodes::OP_OPRO,
            4 => FloppydOpcodes::OP_CLOSE,
            3 => FloppydOpcodes::OP_FLUSH,
            1 => FloppydOpcodes::OP_WRITE,
            2 => FloppydOpcodes::OP_SEEK,
            8 => FloppydOpcodes::OP_SEEK64,
            0 => FloppydOpcodes::OP_READ,
            5 => FloppydOpcodes::OP_IOCTL,
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
pub type iofn = Option<unsafe extern "C" fn(i32, *mut i8, uint32_t) -> ssize_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SQwordRet {
    pub v: int64_t,
    pub err: i32,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn cork(mut sockhandle: i32, mut on: i32) {
    if setsockopt(
        sockhandle,
        C2RustUnnamed::IPPROTO_TCP as i32,
        3 as i32,
        &mut on as *mut i32 as *mut i8 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    ) < 0 as i32
    {
        perror(b"setsockopt cork\0" as *const u8 as *const i8);
    }
}
static mut AuthErrors: [*const i8; 7] = [
    b"Auth success\0" as *const u8 as *const i8,
    b"Auth failed: Packet oversized\0" as *const u8 as *const i8,
    b"Auth failed: X-Cookie doesn't match\0" as *const u8 as *const i8,
    b"Auth failed: Wrong transmission protocol version\0" as *const u8 as *const i8,
    b"Auth failed: Device locked\0" as *const u8 as *const i8,
    b"Auth failed: Bad packet\0" as *const u8 as *const i8,
    b"Auth failed: I/O Error\0" as *const u8 as *const i8,
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
#[inline]
unsafe extern "C" fn byte2sdword(mut val: *mut Byte) -> int32_t {
    let mut l: int32_t = 0;
    l = ((*val.offset(0 as i32 as isize) as i32) << 24 as i32)
        + ((*val.offset(1 as i32 as isize) as i32) << 16 as i32)
        + ((*val.offset(2 as i32 as isize) as i32) << 8 as i32)
        + *val.offset(3 as i32 as isize) as i32;
    return l;
}
#[inline]
unsafe extern "C" fn byte2qword(mut val: *mut Byte) -> Qword {
    let mut l: Qword = 0;
    l = *val.offset(0 as i32 as isize) as Qword;
    l = l << 8 as i32 | *val.offset(1 as i32 as isize) as u64;
    l = l << 8 as i32 | *val.offset(2 as i32 as isize) as u64;
    l = l << 8 as i32 | *val.offset(3 as i32 as isize) as u64;
    l = l << 8 as i32 | *val.offset(4 as i32 as isize) as u64;
    l = l << 8 as i32 | *val.offset(5 as i32 as isize) as u64;
    l = l << 8 as i32 | *val.offset(6 as i32 as isize) as u64;
    l = l << 8 as i32 | *val.offset(7 as i32 as isize) as u64;
    return l;
}
unsafe extern "C" fn dword2byte(mut parm: Dword, mut rval: *mut Byte) {
    *rval.offset(0 as i32 as isize) = (parm >> 24 as i32 & 0xff as i32 as u32) as Byte;
    *rval.offset(1 as i32 as isize) = (parm >> 16 as i32 & 0xff as i32 as u32) as Byte;
    *rval.offset(2 as i32 as isize) = (parm >> 8 as i32 & 0xff as i32 as u32) as Byte;
    *rval.offset(3 as i32 as isize) = (parm & 0xff as i32 as u32) as Byte;
}
#[inline]
unsafe extern "C" fn sdword2byte(mut parm: int32_t, mut rval: *mut Byte) {
    *rval.offset(0 as i32 as isize) = (parm >> 24 as i32 & 0xff as i32) as Byte;
    *rval.offset(1 as i32 as isize) = (parm >> 16 as i32 & 0xff as i32) as Byte;
    *rval.offset(2 as i32 as isize) = (parm >> 8 as i32 & 0xff as i32) as Byte;
    *rval.offset(3 as i32 as isize) = (parm & 0xff as i32) as Byte;
}
#[inline]
unsafe extern "C" fn qword2byte(mut parm: Qword, mut rval: *mut Byte) {
    *rval.offset(0 as i32 as isize) = (parm >> 56 as i32 & 0xff as i32 as u64) as Byte;
    *rval.offset(1 as i32 as isize) = (parm >> 48 as i32 & 0xff as i32 as u64) as Byte;
    *rval.offset(2 as i32 as isize) = (parm >> 40 as i32 & 0xff as i32 as u64) as Byte;
    *rval.offset(3 as i32 as isize) = (parm >> 32 as i32 & 0xff as i32 as u64) as Byte;
    *rval.offset(4 as i32 as isize) = (parm >> 24 as i32 & 0xff as i32 as u64) as Byte;
    *rval.offset(5 as i32 as isize) = (parm >> 16 as i32 & 0xff as i32 as u64) as Byte;
    *rval.offset(6 as i32 as isize) = (parm >> 8 as i32 & 0xff as i32 as u64) as Byte;
    *rval.offset(7 as i32 as isize) = (parm & 0xff as i32 as u64) as Byte;
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
#[inline]
unsafe extern "C" fn read_sdword(mut handle: i32) -> int32_t {
    let mut val: [Byte; 4] = [0; 4];
    if read(handle, val.as_mut_ptr() as *mut i8 as *mut libc::c_void, 4 as i32 as size_t)
        < 4 as i32 as i64
    {
        return -(1 as i32);
    }
    return byte2sdword(val.as_mut_ptr());
}
#[inline]
unsafe extern "C" fn read_sqword(mut handle: i32) -> SQwordRet {
    let mut val: [Byte; 8] = [0; 8];
    let mut ret: SQwordRet = SQwordRet { v: 0, err: 0 };
    if read(handle, val.as_mut_ptr() as *mut i8 as *mut libc::c_void, 8 as i32 as size_t)
        < 8 as i32 as i64
    {
        ret.err = -(1 as i32);
    } else {
        ret.v = byte2qword(val.as_mut_ptr()) as int64_t;
        ret.err = 0 as i32;
    }
    return ret;
}
unsafe extern "C" fn authenticate_to_floppyd(
    mut floppyd: *mut RemoteFile_t,
    mut sock: i32,
    mut display: *mut i8,
) -> u32 {
    let mut cookielen: size_t = 0;
    let mut filelen: uint16_t = 0;
    let mut newlen: ssize_t = 0;
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
    let mut l: i32 = 0;
    command[4 as i32 as usize] = display;
    cookielen = strlen(display);
    cookielen = (cookielen as u64).wrapping_add(100 as i32 as u64) as size_t as size_t;
    xcookie = safe_malloc(cookielen.wrapping_add(4 as i32 as u64)) as *mut i8;
    newlen = safePopenOut(
        command.as_mut_ptr(),
        xcookie.offset(4 as i32 as isize),
        cookielen,
    );
    if newlen < 1 as i32 as i64 || newlen > 65535 as i32 as i64 {
        return AuthErrorsEnum::AUTH_AUTHFAILED as i32 as u32;
    }
    filelen = newlen as uint16_t;
    dword2byte(4 as i32 as Dword, buf.as_mut_ptr());
    dword2byte((*floppyd).version, buf.as_mut_ptr().offset(4 as i32 as isize));
    if write(sock, buf.as_mut_ptr() as *const libc::c_void, 8 as i32 as size_t)
        < 8 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32 as u32;
    }
    l = read_dword(sock) as i32;
    if l < 4 as i32 {
        return AuthErrorsEnum::AUTH_WRONGVERSION as i32 as u32;
    }
    errcode = read_dword(sock);
    if errcode != AuthErrorsEnum::AUTH_SUCCESS as i32 as u32 {
        return errcode;
    }
    if l >= 8 as i32 {
        (*floppyd).version = read_dword(sock);
    }
    if l >= 12 as i32 {
        (*floppyd).capabilities = read_dword(sock);
    }
    dword2byte(filelen as Dword, xcookie as *mut Byte);
    if write(sock, xcookie as *const libc::c_void, (filelen as i32 + 4 as i32) as size_t)
        < (filelen as i32 + 4 as i32) as ssize_t
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32 as u32;
    }
    if read_dword(sock) != 4 as i32 as u32 {
        return AuthErrorsEnum::AUTH_PACKETOVERSIZE as i32 as u32;
    }
    errcode = read_dword(sock);
    return errcode;
}
unsafe extern "C" fn floppyd_reader(
    mut fd: i32,
    mut buffer: *mut i8,
    mut len: uint32_t,
) -> ssize_t {
    let mut errcode: Dword = 0;
    let mut gotlen: Dword = 0;
    let mut buf: [Byte; 16] = [0; 16];
    dword2byte(1 as i32 as Dword, buf.as_mut_ptr());
    buf[4 as i32 as usize] = FloppydOpcodes::OP_READ as i32 as Byte;
    dword2byte(4 as i32 as Dword, buf.as_mut_ptr().offset(5 as i32 as isize));
    dword2byte(len, buf.as_mut_ptr().offset(9 as i32 as isize));
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 13 as i32 as size_t)
        < 13 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32 as ssize_t;
    }
    if read_dword(fd) != 8 as i32 as u32 {
        *__errno_location() = 5 as i32;
        return -(1 as i32) as ssize_t;
    }
    gotlen = read_dword(fd);
    errcode = read_dword(fd);
    if gotlen != -(1 as i32) as Dword {
        let mut l: size_t = 0;
        let mut start: u32 = 0;
        if read_dword(fd) != gotlen {
            *__errno_location() = 5 as i32;
            return -(1 as i32) as ssize_t;
        }
        start = 0 as i32 as u32;
        l = 0 as i32 as size_t;
        while start < gotlen {
            let mut ret: ssize_t = read(
                fd,
                buffer.offset(start as isize) as *mut libc::c_void,
                gotlen.wrapping_sub(start) as size_t,
            );
            if ret < 0 as i32 as i64 {
                return -(1 as i32) as ssize_t;
            }
            if ret == 0 as i32 as i64 {
                *__errno_location() = 5 as i32;
                return -(1 as i32) as ssize_t;
            }
            l = ret as size_t;
            start = (start as u64).wrapping_add(l) as u32 as u32;
        }
    } else {
        *__errno_location() = errcode as i32;
    }
    return gotlen as ssize_t;
}
unsafe extern "C" fn floppyd_writer(
    mut fd: i32,
    mut buffer: *mut i8,
    mut len: uint32_t,
) -> ssize_t {
    let mut errcode: i32 = 0;
    let mut gotlen: int32_t = 0;
    let mut buf: [Byte; 16] = [0; 16];
    let mut ret: ssize_t = 0;
    dword2byte(1 as i32 as Dword, buf.as_mut_ptr());
    buf[4 as i32 as usize] = FloppydOpcodes::OP_WRITE as i32 as Byte;
    dword2byte(len, buf.as_mut_ptr().offset(5 as i32 as isize));
    cork(fd, 1 as i32);
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 9 as i32 as size_t)
        < 9 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32 as ssize_t;
    }
    ret = write(fd, buffer as *const libc::c_void, len as size_t);
    if ret == -(1 as i32) as i64 || (ret as size_t) < len as u64 {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32 as ssize_t;
    }
    cork(fd, 0 as i32);
    if read_dword(fd) != 8 as i32 as u32 {
        *__errno_location() = 5 as i32;
        return -(1 as i32) as ssize_t;
    }
    gotlen = read_sdword(fd);
    errcode = read_sdword(fd);
    *__errno_location() = errcode;
    if *__errno_location() != 0 as i32 && gotlen == 0 as i32 {
        if *__errno_location() == 9 as i32 {
            *__errno_location() = 30 as i32;
        }
        gotlen = -(1 as i32);
    }
    return gotlen as ssize_t;
}
unsafe extern "C" fn floppyd_lseek(
    mut fd: i32,
    mut offset: int32_t,
    mut whence: i32,
) -> i32 {
    let mut errcode: i32 = 0;
    let mut gotlen: i32 = 0;
    let mut buf: [Byte; 32] = [0; 32];
    dword2byte(1 as i32 as Dword, buf.as_mut_ptr());
    buf[4 as i32 as usize] = FloppydOpcodes::OP_SEEK as i32 as Byte;
    dword2byte(8 as i32 as Dword, buf.as_mut_ptr().offset(5 as i32 as isize));
    sdword2byte(offset, buf.as_mut_ptr().offset(9 as i32 as isize));
    sdword2byte(whence, buf.as_mut_ptr().offset(13 as i32 as isize));
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 17 as i32 as size_t)
        < 17 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32;
    }
    if read_dword(fd) != 8 as i32 as u32 {
        *__errno_location() = 5 as i32;
        return -(1 as i32);
    }
    gotlen = read_sdword(fd);
    errcode = read_sdword(fd);
    *__errno_location() = errcode;
    return gotlen;
}
unsafe extern "C" fn floppyd_lseek64(
    mut fd: i32,
    mut offset: mt_off_t,
    mut whence: i32,
) -> mt_off_t {
    let mut errcode: i32 = 0;
    let mut gotlen: SQwordRet = SQwordRet { v: 0, err: 0 };
    let mut buf: [Byte; 32] = [0; 32];
    dword2byte(1 as i32 as Dword, buf.as_mut_ptr());
    buf[4 as i32 as usize] = FloppydOpcodes::OP_SEEK64 as i32 as Byte;
    dword2byte(12 as i32 as Dword, buf.as_mut_ptr().offset(5 as i32 as isize));
    qword2byte(offset as uint32_t as Qword, buf.as_mut_ptr().offset(9 as i32 as isize));
    sdword2byte(whence, buf.as_mut_ptr().offset(17 as i32 as isize));
    if write(fd, buf.as_mut_ptr() as *const libc::c_void, 21 as i32 as size_t)
        < 21 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32 as mt_off_t;
    }
    if read_dword(fd) != 12 as i32 as u32 {
        *__errno_location() = 5 as i32;
        return -(1 as i32) as mt_off_t;
    }
    gotlen = read_sqword(fd);
    errcode = read_sdword(fd);
    *__errno_location() = errcode;
    return gotlen.v;
}
unsafe extern "C" fn floppyd_open(mut This: *mut RemoteFile_t, mut mode: i32) -> i32 {
    let mut errcode: i32 = 0;
    let mut gotlen: i32 = 0;
    let mut buf: [Byte; 16] = [0; 16];
    if (*This).capabilities & 1 as i32 as u32 == 0 {
        return 0 as i32;
    }
    dword2byte(1 as i32 as Dword, buf.as_mut_ptr());
    if mode & 0o3 as i32 == 0 as i32 {
        buf[4 as i32 as usize] = FloppydOpcodes::OP_OPRO as i32 as Byte;
    } else {
        buf[4 as i32 as usize] = FloppydOpcodes::OP_OPRW as i32 as Byte;
    }
    dword2byte(4 as i32 as Dword, buf.as_mut_ptr().offset(5 as i32 as isize));
    sdword2byte((*This).drive, buf.as_mut_ptr().offset(9 as i32 as isize));
    if write((*This).fd, buf.as_mut_ptr() as *const libc::c_void, 13 as i32 as size_t)
        < 13 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32;
    }
    if read_dword((*This).fd) != 8 as i32 as u32 {
        *__errno_location() = 5 as i32;
        return -(1 as i32);
    }
    gotlen = read_sdword((*This).fd);
    errcode = read_sdword((*This).fd);
    *__errno_location() = errcode;
    return gotlen;
}
unsafe extern "C" fn floppyd_io(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
    mut io: iofn,
) -> ssize_t {
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    let mut ret: ssize_t = 0;
    where_0 += (*This).offset;
    if where_0 != (*This).lastwhere {
        if (*This).capabilities & 2 as i32 as u32 != 0 {
            if floppyd_lseek64((*This).fd, where_0, 0 as i32) < 0 as i32 as i64 {
                perror(b"floppyd_lseek64\0" as *const u8 as *const i8);
                (*This).lastwhere = -(1 as i32) as mt_off_t;
                return -(1 as i32) as ssize_t;
            }
        } else {
            if where_0 > 2147483647 as i32 as i64
                || where_0 < (-(2147483647 as i32) - 1 as i32) as i64
            {
                fprintf(
                    stderr,
                    b"Seek position out of range\n\0" as *const u8 as *const i8,
                );
                return -(1 as i32) as ssize_t;
            }
            if floppyd_lseek((*This).fd, where_0 as int32_t, 0 as i32) < 0 as i32 {
                perror(b"floppyd_lseek\0" as *const u8 as *const i8);
                (*This).lastwhere = -(1 as i32) as mt_off_t;
                return -(1 as i32) as ssize_t;
            }
        }
    }
    ret = io
        .expect(
            "non-null function pointer",
        )(
        (*This).fd,
        buf,
        if len > 2147483647 as i32 as u64 {
            (2147483647 as i32 as uint32_t).wrapping_add(1 as i32 as u32)
        } else {
            len as uint32_t
        },
    );
    if ret == -(1 as i32) as i64 {
        perror(b"floppyd_io\0" as *const u8 as *const i8);
        (*This).lastwhere = -(1 as i32) as mt_off_t;
        return -(1 as i32) as ssize_t;
    }
    (*This).lastwhere = where_0 + ret;
    return ret;
}
unsafe extern "C" fn floppyd_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return floppyd_io(
        Stream,
        buf,
        where_0,
        len,
        Some(floppyd_reader as unsafe extern "C" fn(i32, *mut i8, uint32_t) -> ssize_t),
    );
}
unsafe extern "C" fn floppyd_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return floppyd_io(
        Stream,
        buf,
        where_0,
        len,
        Some(floppyd_writer as unsafe extern "C" fn(i32, *mut i8, uint32_t) -> ssize_t),
    );
}
unsafe extern "C" fn floppyd_flush(mut Stream: *mut Stream_t) -> i32 {
    let mut buf: [Byte; 16] = [0; 16];
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    dword2byte(1 as i32 as Dword, buf.as_mut_ptr());
    buf[4 as i32 as usize] = FloppydOpcodes::OP_FLUSH as i32 as Byte;
    dword2byte(1 as i32 as Dword, buf.as_mut_ptr().offset(5 as i32 as isize));
    buf[9 as i32 as usize] = '\0' as i32 as Byte;
    if write((*This).fd, buf.as_mut_ptr() as *const libc::c_void, 10 as i32 as size_t)
        < 10 as i32 as i64
    {
        return AuthErrorsEnum::AUTH_IO_ERROR as i32;
    }
    if read_dword((*This).fd) != 8 as i32 as u32 {
        *__errno_location() = 5 as i32;
        return -(1 as i32);
    }
    read_dword((*This).fd);
    read_dword((*This).fd);
    return 0 as i32;
}
unsafe extern "C" fn floppyd_free(mut Stream: *mut Stream_t) -> i32 {
    let mut buf: [Byte; 16] = [0; 16];
    let mut gotlen: i32 = 0;
    let mut errcode: i32 = 0;
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    if (*This).fd > 2 as i32 {
        dword2byte(1 as i32 as Dword, buf.as_mut_ptr());
        buf[4 as i32 as usize] = FloppydOpcodes::OP_CLOSE as i32 as Byte;
        if write((*This).fd, buf.as_mut_ptr() as *const libc::c_void, 5 as i32 as size_t)
            < 5 as i32 as i64
        {
            return AuthErrorsEnum::AUTH_IO_ERROR as i32;
        }
        shutdown((*This).fd, 1 as i32);
        if read_dword((*This).fd) != 8 as i32 as u32 {
            *__errno_location() = 5 as i32;
            return -(1 as i32);
        }
        gotlen = read_sdword((*This).fd);
        errcode = read_sdword((*This).fd);
        *__errno_location() = errcode;
        close((*This).fd);
        return gotlen;
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn floppyd_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut i32,
    mut address: *mut uint32_t,
) -> i32 {
    let mut This: *mut RemoteFile_t = Stream as *mut RemoteFile_t;
    if !date.is_null() {
        *date = 0 as i32 as time_t;
    }
    if !size.is_null() {
        *size = (*This).size;
    }
    if !type_0.is_null() {
        *type_0 = 0 as i32;
    }
    if !address.is_null() {
        *address = 0 as i32 as uint32_t;
    }
    return 0 as i32;
}
static mut FloppydFileClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                floppyd_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                floppyd_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(floppyd_flush as unsafe extern "C" fn(*mut Stream_t) -> i32),
            freeFunc: Some(floppyd_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
            set_geom: Some(
                set_geom_noop
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device_t,
                        *mut device_t,
                    ) -> i32,
            ),
            get_data: Some(
                floppyd_data
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
unsafe extern "C" fn get_host_and_port_and_drive(
    mut name: *const i8,
    mut hostname: *mut *mut i8,
    mut display: *mut *mut i8,
    mut port: *mut uint16_t,
    mut drive: *mut i32,
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
    *port = 5703 as i32 as uint16_t;
    if *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
        *port = strtou16(p, &mut p, 0 as i32);
    }
    if *p as i32 == '/' as i32 {
        p = p.offset(1);
        p;
    }
    *drive = 0 as i32;
    if *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
        *drive = strtosi(p, &mut p, 0 as i32);
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
        sock,
        1 as i32,
        9 as i32,
        &mut on as *mut i32 as *mut i8 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    );
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn FloppydOpen(
    mut dev: *mut device,
    mut name: *const i8,
    mut mode: i32,
    mut errmsg: *mut i8,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    let mut This: *mut RemoteFile_t = 0 as *mut RemoteFile_t;
    if dev.is_null() || (*dev).misc_flags & 0x40 as u32 == 0 {
        return 0 as *mut Stream_t;
    }
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<RemoteFile_t>() as u64)
        as *mut RemoteFile_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut FloppydFileClass, 0 as *mut Stream_t);
    (*This).offset = 0 as i32 as mt_off_t;
    (*This).lastwhere = 0 as i32 as mt_off_t;
    (*This).fd = ConnectToFloppyd(This, name, errmsg);
    if (*This).fd == -(1 as i32) {
        free(This as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    if floppyd_open(This, mode) < 0 as i32 {
        sprintf(
            errmsg,
            b"Can't open remote drive: %s\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
        close((*This).fd);
        free(This as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    if !maxSize.is_null() {
        *maxSize = if (*This).capabilities & 2 as i32 as u32 != 0 {
            max_off_t_seek
        } else {
            max_off_t_31
        };
    }
    return &mut (*This).head;
}
unsafe extern "C" fn ConnectToFloppyd(
    mut floppyd: *mut RemoteFile_t,
    mut name: *const i8,
    mut errmsg: *mut i8,
) -> i32 {
    let mut hostname: *mut i8 = 0 as *mut i8;
    let mut display: *mut i8 = 0 as *mut i8;
    let mut port: uint16_t = 0;
    let mut rval: i32 = get_host_and_port_and_drive(
        name,
        &mut hostname,
        &mut display,
        &mut port,
        &mut (*floppyd).drive,
    );
    let mut sock: i32 = 0;
    let mut reply: u32 = 0;
    if rval == 0 {
        return -(1 as i32);
    }
    (*floppyd).version = 11 as i32 as u32;
    (*floppyd).capabilities = 0 as i32 as u32;
    loop {
        sock = connect_to_server(getipaddress(hostname), port);
        if sock == -(1 as i32) {
            snprintf(
                errmsg,
                200 as i32 as u64,
                b"Can't connect to floppyd server on %s, port %i (%s)!\0" as *const u8
                    as *const i8,
                hostname,
                port as i32,
                strerror(*__errno_location()),
            );
            return -(1 as i32);
        }
        reply = authenticate_to_floppyd(floppyd, sock, display);
        if (*floppyd).version == 10 as i32 as u32 {
            break;
        }
        if !(reply == AuthErrorsEnum::AUTH_WRONGVERSION as i32 as u32) {
            break;
        }
        (*floppyd).version = 10 as i32 as u32;
    }
    if reply != 0 as i32 as u32 {
        fprintf(
            stderr,
            b"Permission denied, authentication failed!\n%s\n\0" as *const u8
                as *const i8,
            AuthErrors[reply as usize],
        );
        return -(1 as i32);
    }
    free(hostname as *mut libc::c_void);
    free(display as *mut libc::c_void);
    return sock;
}