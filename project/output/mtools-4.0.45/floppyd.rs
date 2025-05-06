#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type _XDisplay;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn alarm(__seconds: u32) -> u32;
    fn setpgrp() -> i32;
    fn setsid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> i32;
    fn setgid(__gid: __gid_t) -> i32;
    fn fork() -> __pid_t;
    fn unlink(__name: *const i8) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn gethostname(__name: *mut i8, __len: size_t) -> i32;
    fn fsync(__fd: i32) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn __errno_location() -> *mut i32;
    fn endpwent();
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn perror(__s: *const i8);
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn mkstemp(__template: *mut i8) -> i32;
    fn setenv(__name: *const i8, __value: *const i8, __replace: i32) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn mt_lseek(fd: i32, where_0: mt_off_t, whence: i32) -> i32;
    fn initgroups(__user: *const i8, __group: __gid_t) -> i32;
    fn lock_dev(fd: i32, mode: i32, dev: *mut device) -> i32;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
    fn bind(__fd: i32, __addr: *const sockaddr, __len: socklen_t) -> i32;
    fn getsockname(__fd: i32, __addr: *mut sockaddr, __len: *mut socklen_t) -> i32;
    fn setsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> i32;
    fn listen(__fd: i32, __n: i32) -> i32;
    fn accept(__fd: i32, __addr: *mut sockaddr, __addr_len: *mut socklen_t) -> i32;
    fn inet_addr(__cp: *const i8) -> in_addr_t;
    fn endhostent();
    fn gethostbyname(__name: *const i8) -> *mut hostent;
    fn endservent();
    fn getservbyname(__name: *const i8, __proto: *const i8) -> *mut servent;
    fn XauFileName() -> *mut i8;
    fn XOpenDisplay(_: *const i8) -> *mut Display;
    fn XCloseDisplay(_: *mut Display) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __mode_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
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
pub type socklen_t = __socklen_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
pub type mt_off_t = off_t;
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
pub enum C2RustUnnamed_0 {
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
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::IPPROTO_MAX => 256,
            C2RustUnnamed_0::IPPROTO_RAW => 255,
            C2RustUnnamed_0::IPPROTO_MPLS => 137,
            C2RustUnnamed_0::IPPROTO_UDPLITE => 136,
            C2RustUnnamed_0::IPPROTO_SCTP => 132,
            C2RustUnnamed_0::IPPROTO_COMP => 108,
            C2RustUnnamed_0::IPPROTO_PIM => 103,
            C2RustUnnamed_0::IPPROTO_ENCAP => 98,
            C2RustUnnamed_0::IPPROTO_BEETPH => 94,
            C2RustUnnamed_0::IPPROTO_MTP => 92,
            C2RustUnnamed_0::IPPROTO_AH => 51,
            C2RustUnnamed_0::IPPROTO_ESP => 50,
            C2RustUnnamed_0::IPPROTO_GRE => 47,
            C2RustUnnamed_0::IPPROTO_RSVP => 46,
            C2RustUnnamed_0::IPPROTO_IPV6 => 41,
            C2RustUnnamed_0::IPPROTO_DCCP => 33,
            C2RustUnnamed_0::IPPROTO_TP => 29,
            C2RustUnnamed_0::IPPROTO_IDP => 22,
            C2RustUnnamed_0::IPPROTO_UDP => 17,
            C2RustUnnamed_0::IPPROTO_PUP => 12,
            C2RustUnnamed_0::IPPROTO_EGP => 8,
            C2RustUnnamed_0::IPPROTO_TCP => 6,
            C2RustUnnamed_0::IPPROTO_IPIP => 4,
            C2RustUnnamed_0::IPPROTO_IGMP => 2,
            C2RustUnnamed_0::IPPROTO_ICMP => 1,
            C2RustUnnamed_0::IPPROTO_IP => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            256 => C2RustUnnamed_0::IPPROTO_MAX,
            255 => C2RustUnnamed_0::IPPROTO_RAW,
            137 => C2RustUnnamed_0::IPPROTO_MPLS,
            136 => C2RustUnnamed_0::IPPROTO_UDPLITE,
            132 => C2RustUnnamed_0::IPPROTO_SCTP,
            108 => C2RustUnnamed_0::IPPROTO_COMP,
            103 => C2RustUnnamed_0::IPPROTO_PIM,
            98 => C2RustUnnamed_0::IPPROTO_ENCAP,
            94 => C2RustUnnamed_0::IPPROTO_BEETPH,
            92 => C2RustUnnamed_0::IPPROTO_MTP,
            51 => C2RustUnnamed_0::IPPROTO_AH,
            50 => C2RustUnnamed_0::IPPROTO_ESP,
            47 => C2RustUnnamed_0::IPPROTO_GRE,
            46 => C2RustUnnamed_0::IPPROTO_RSVP,
            41 => C2RustUnnamed_0::IPPROTO_IPV6,
            33 => C2RustUnnamed_0::IPPROTO_DCCP,
            29 => C2RustUnnamed_0::IPPROTO_TP,
            22 => C2RustUnnamed_0::IPPROTO_IDP,
            17 => C2RustUnnamed_0::IPPROTO_UDP,
            12 => C2RustUnnamed_0::IPPROTO_PUP,
            8 => C2RustUnnamed_0::IPPROTO_EGP,
            6 => C2RustUnnamed_0::IPPROTO_TCP,
            4 => C2RustUnnamed_0::IPPROTO_IPIP,
            2 => C2RustUnnamed_0::IPPROTO_IGMP,
            1 => C2RustUnnamed_0::IPPROTO_ICMP,
            0 => C2RustUnnamed_0::IPPROTO_IP,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servent {
    pub s_name: *mut i8,
    pub s_aliases: *mut *mut i8,
    pub s_port: i32,
    pub s_proto: *mut i8,
}
pub type Byte = uint8_t;
pub type Dword = uint32_t;
pub type Qword = uint64_t;
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
pub type Display = _XDisplay;
pub type Packet = *mut Packet_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Packet_0 {
    pub data: *mut Byte,
    pub len: Dword,
    pub alloc_size: Dword,
}
pub type io_buffer = *mut io_buffer_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct io_buffer_0 {
    pub out_buffer: [Byte; 16348],
    pub in_buffer: [Byte; 16348],
    pub in_valid: size_t,
    pub in_start: size_t,
    pub out_valid: size_t,
    pub handle: i32,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn cork(mut sockhandle: i32, mut on: i32) {
    if setsockopt(
        sockhandle,
        C2RustUnnamed_0::IPPROTO_TCP as i32,
        3 as i32,
        &mut on as *mut i32 as *mut i8 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    ) < 0 as i32
    {
        perror(b"setsockopt cork\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub static mut mtools_lock_timeout: u32 = 30 as i32 as u32;
unsafe extern "C" fn new_io_buffer(mut _handle: i32) -> io_buffer {
    let mut buffer: io_buffer = 0 as *mut io_buffer_0;
    buffer = calloc(1 as i32 as u64, ::core::mem::size_of::<io_buffer_0>() as u64)
        as *mut io_buffer_0;
    (*buffer).handle = _handle;
    (*buffer).in_start = 0 as i32 as size_t;
    (*buffer).in_valid = (*buffer).in_start;
    (*buffer).out_valid = 0 as i32 as size_t;
    return buffer;
}
unsafe extern "C" fn flush(mut buffer: io_buffer) {
    if (*buffer).out_valid != 0 {
        if write(
            (*buffer).handle,
            ((*buffer).out_buffer).as_mut_ptr() as *const libc::c_void,
            (*buffer).out_valid,
        ) < 0 as i32 as i64
        {
            perror(b"floppyd flush\0" as *const u8 as *const i8);
        }
        (*buffer).out_valid = 0 as i32 as size_t;
    }
}
unsafe extern "C" fn free_io_buffer(mut buffer: io_buffer) {
    flush(buffer);
    free(buffer as *mut libc::c_void);
}
unsafe extern "C" fn buf_read(
    mut buf: io_buffer,
    mut buffer: *mut Byte,
    mut nbytes: size_t,
) -> size_t {
    let mut ret: size_t = 0;
    if nbytes <= (*buf).in_valid {
        memcpy(
            buffer as *mut libc::c_void,
            ((*buf).in_buffer).as_mut_ptr().offset((*buf).in_start as isize)
                as *const libc::c_void,
            nbytes,
        );
        (*buf).in_valid = ((*buf).in_valid as u64).wrapping_sub(nbytes) as size_t
            as size_t;
        (*buf).in_start = ((*buf).in_start as u64).wrapping_add(nbytes) as size_t
            as size_t;
        ret = nbytes;
    } else {
        if (*buf).in_valid != 0 {
            memcpy(
                buffer as *mut libc::c_void,
                ((*buf).in_buffer).as_mut_ptr().offset((*buf).in_start as isize)
                    as *const libc::c_void,
                (*buf).in_valid,
            );
        }
        nbytes = (nbytes as u64).wrapping_sub((*buf).in_valid) as size_t as size_t;
        buffer = buffer.offset((*buf).in_valid as isize);
        if nbytes > 16348 as i32 as u64 {
            let mut rval: ssize_t = read(
                (*buf).handle,
                buffer as *mut libc::c_void,
                nbytes,
            );
            if rval >= 0 as i32 as i64 {
                ret = (rval as size_t).wrapping_add((*buf).in_valid);
            } else {
                perror(b"read error\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
            (*buf).in_start = 0 as i32 as size_t;
            (*buf).in_valid = (*buf).in_start;
        } else {
            let mut rval_0: ssize_t = read(
                (*buf).handle,
                ((*buf).in_buffer).as_mut_ptr() as *mut libc::c_void,
                16348 as i32 as size_t,
            );
            if rval_0 >= 0 as i32 as i64 {
                if rval_0 < nbytes as ssize_t {
                    memcpy(
                        buffer as *mut libc::c_void,
                        ((*buf).in_buffer).as_mut_ptr() as *const libc::c_void,
                        rval_0 as size_t,
                    );
                    ret = (rval_0 as size_t).wrapping_add((*buf).in_valid);
                    (*buf).in_start = 0 as i32 as size_t;
                    (*buf).in_valid = (*buf).in_start;
                } else {
                    let mut a: size_t = 0;
                    memcpy(
                        buffer as *mut libc::c_void,
                        ((*buf).in_buffer).as_mut_ptr() as *const libc::c_void,
                        nbytes,
                    );
                    (*buf).in_start = nbytes;
                    a = (*buf).in_valid;
                    (*buf).in_valid = (rval_0 as size_t).wrapping_sub(nbytes);
                    ret = a.wrapping_add(nbytes);
                }
            } else {
                perror(b"read error\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
        }
    }
    return ret;
}
unsafe extern "C" fn buf_write(
    mut buf: io_buffer,
    mut buffer: *mut libc::c_void,
    mut nbytes: size_t,
) -> ssize_t {
    if ((*buf).out_valid).wrapping_add(nbytes) > 16348 as i32 as u64 {
        flush(buf);
        return write((*buf).handle, buffer, nbytes);
    }
    memcpy(
        ((*buf).out_buffer).as_mut_ptr().offset((*buf).out_valid as isize)
            as *mut libc::c_void,
        buffer,
        nbytes,
    );
    (*buf).out_valid = ((*buf).out_valid as u64).wrapping_add(nbytes) as size_t
        as size_t;
    return nbytes as ssize_t;
}
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
unsafe extern "C" fn read_dword(mut fp: io_buffer) -> Dword {
    let mut val: [Byte; 4] = [0; 4];
    if buf_read(fp, val.as_mut_ptr(), 4 as i32 as size_t) < 4 as i32 as u64 {
        return 0xffffffff as u32;
    }
    return byte2dword(val.as_mut_ptr());
}
unsafe extern "C" fn write_dword(mut fp: io_buffer, mut parm: Dword) {
    let mut val: [Byte; 4] = [0; 4];
    dword2byte(parm, val.as_mut_ptr());
    buf_write(fp, val.as_mut_ptr() as *mut libc::c_void, 4 as i32 as size_t);
}
unsafe extern "C" fn newPacket() -> Packet {
    let mut packet: Packet = 0 as *mut Packet_0;
    packet = calloc(1 as i32 as u64, ::core::mem::size_of::<Packet_0>() as u64)
        as *mut Packet_0;
    (*packet).data = 0 as *mut Byte;
    (*packet).alloc_size = 0 as i32 as Dword;
    (*packet).len = (*packet).alloc_size;
    return packet;
}
unsafe extern "C" fn destroyPacket(mut packet: Packet) {
    if !((*packet).data).is_null() {
        free((*packet).data as *mut libc::c_void);
    }
    free(packet as *mut libc::c_void);
}
unsafe extern "C" fn kill_packet(mut packet: Packet) {
    if !((*packet).data).is_null() {
        free((*packet).data as *mut libc::c_void);
    }
    (*packet).data = 0 as *mut Byte;
    (*packet).len = 0 as i32 as Dword;
    (*packet).alloc_size = 0 as i32 as Dword;
}
unsafe extern "C" fn make_new(mut packet: Packet, mut l: Dword) {
    if l < (*packet).alloc_size {
        (*packet).len = l;
        return;
    }
    kill_packet(packet);
    (*packet).alloc_size = l;
    (*packet).len = (*packet).alloc_size;
    (*packet).data = malloc(l as u64) as *mut Byte;
    memset((*packet).data as *mut libc::c_void, 0 as i32, l as u64);
}
unsafe extern "C" fn send_packet(mut packet: Packet, mut fp: io_buffer) -> i8 {
    if !((*packet).data).is_null() {
        write_dword(fp, (*packet).len);
        buf_write(fp, (*packet).data as *mut libc::c_void, (*packet).len as size_t);
        flush(fp);
    }
    return ((*packet).data != 0 as *mut libc::c_void as *mut Byte) as i32 as i8;
}
unsafe extern "C" fn recv_packet(
    mut packet: Packet,
    mut fp: io_buffer,
    mut maxlength: Dword,
) -> i8 {
    let mut start: Dword = 0;
    let mut l: size_t = 0;
    let mut length: Dword = read_dword(fp);
    if length > maxlength || length == 0xffffffff as u32 {
        return 0 as i32 as i8;
    }
    make_new(packet, length);
    l = 0 as i32 as size_t;
    start = 0 as i32 as Dword;
    while start < length {
        l = buf_read(
            fp,
            ((*packet).data).offset(start as isize),
            length.wrapping_sub(start) as size_t,
        );
        if l == 0 as i32 as u64 {
            return 0 as i32 as i8;
        }
        start = (start as u64).wrapping_add(l) as Dword as Dword;
    }
    if (*packet).len == 0 as i32 as u32 {
        return 0 as i32 as i8;
    }
    return 1 as i32 as i8;
}
unsafe extern "C" fn read_packet(
    mut packet: Packet,
    mut fd: i32,
    mut length: Dword,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    make_new(packet, length);
    ret = read(fd, (*packet).data as *mut libc::c_void, (*packet).len as size_t);
    if ret < 0 as i32 as i64 {
        return ret;
    }
    (*packet).len = ret as Dword;
    return 0 as i32 as ssize_t;
}
unsafe extern "C" fn write_packet(mut packet: Packet, mut fd: i32) -> i32 {
    return write(fd, (*packet).data as *const libc::c_void, (*packet).len as size_t)
        as i32;
}
unsafe extern "C" fn put_dword(mut packet: Packet, mut my_index: i32, mut val: Dword) {
    dword2byte(val, ((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn put_qword(mut packet: Packet, mut my_index: i32, mut val: Qword) {
    qword2byte(val, ((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn get_dword(mut packet: Packet, mut my_index: i32) -> Dword {
    return byte2dword(((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn get_qword(mut packet: Packet, mut my_index: i32) -> Qword {
    return byte2qword(((*packet).data).offset(my_index as isize));
}
unsafe extern "C" fn get_length(mut packet: Packet) -> Dword {
    return (*packet).len;
}
unsafe extern "C" fn eat(mut ptr: *mut *mut u8, mut len: *mut size_t, mut c: u8) -> i32 {
    if *len < (c as u32).wrapping_add(3 as u32) as u64 {
        return -(1 as i32);
    }
    *ptr = (*ptr).offset((c as i32 + 2 as i32) as isize);
    *len = (*len as u64).wrapping_sub((c as i32 + 2 as i32) as u64) as size_t as size_t;
    return 0 as i32;
}
static mut dispName: *const i8 = 0 as *const i8;
static mut XAUTHORITY: [i8; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &mut [i8; 11]>(b"XAUTHORITY\0")
};
unsafe extern "C" fn do_auth(mut sock: io_buffer, mut version: *mut u32) -> i8 {
    let mut fd: i32 = 0;
    let mut displ: *mut Display = 0 as *mut Display;
    let mut proto_version: Packet = newPacket();
    let mut mit_cookie: Packet = 0 as *mut Packet_0;
    let mut ptr: *mut u8 = 0 as *mut u8;
    let mut len: size_t = 0;
    let mut authFile: [i8; 41] = *::core::mem::transmute::<
        &[u8; 41],
        &mut [i8; 41],
    >(b"/tmp/floppyd.XXXXXX\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut template: [u8; 4096] = [0; 4096];
    let mut reply: Packet = newPacket();
    make_new(reply, 4 as i32 as Dword);
    if recv_packet(proto_version, sock, 4 as i32 as Dword) == 0 {
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_PACKETOVERSIZE as i32 as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        destroyPacket(proto_version);
        return 0 as i32 as i8;
    }
    *version = get_dword(proto_version, 0 as i32);
    if *version > 11 as i32 as u32 || *version < 10 as i32 as u32 {
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_WRONGVERSION as i32 as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        destroyPacket(proto_version);
        return 0 as i32 as i8;
    }
    if *version == 10 as i32 as u32 {
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_SUCCESS as i32 as Dword);
    } else {
        let mut cap: Dword = 1 as i32 as Dword;
        if ::core::mem::size_of::<mt_off_t>() as u64 >= 8 as i32 as u64 {
            cap |= 2 as i32 as u32;
        }
        make_new(reply, 12 as i32 as Dword);
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_SUCCESS as i32 as Dword);
        put_dword(reply, 4 as i32, 11 as i32 as Dword);
        put_dword(reply, 8 as i32, cap);
    }
    send_packet(reply, sock);
    destroyPacket(proto_version);
    make_new(reply, 4 as i32 as Dword);
    mit_cookie = newPacket();
    if recv_packet(mit_cookie, sock, 3000 as i32 as Dword) == 0 {
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_PACKETOVERSIZE as i32 as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        destroyPacket(mit_cookie);
        return 0 as i32 as i8;
    }
    umask(0o77 as i32 as __mode_t);
    fd = mkstemp(authFile.as_mut_ptr());
    if fd == -(1 as i32) {
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_DEVLOCKED as i32 as Dword);
        send_packet(reply, sock);
        close(fd);
        destroyPacket(reply);
        destroyPacket(mit_cookie);
        return 0 as i32 as i8;
    }
    setenv(XAUTHORITY.as_mut_ptr(), authFile.as_mut_ptr(), 1 as i32);
    ptr = template.as_mut_ptr();
    *ptr.offset(4095 as i32 as isize) = 0 as i32 as u8;
    let fresh0 = ptr;
    ptr = ptr.offset(1);
    *fresh0 = 1 as i32 as u8;
    let fresh1 = ptr;
    ptr = ptr.offset(1);
    *fresh1 = 0 as i32 as u8;
    let fresh2 = ptr;
    ptr = ptr.offset(1);
    *fresh2 = 0 as i32 as u8;
    gethostname((ptr as *mut i8).offset(1 as i32 as isize), 4088 as i32 as size_t);
    len = strlen((ptr as *mut i8).offset(1 as i32 as isize));
    let fresh3 = ptr;
    ptr = ptr.offset(1);
    *fresh3 = len as u8;
    ptr = ptr.offset(len as isize);
    let fresh4 = ptr;
    ptr = ptr.offset(1);
    *fresh4 = 0 as i32 as u8;
    let fresh5 = ptr;
    ptr = ptr.offset(1);
    *fresh5 = 1 as i32 as u8;
    let fresh6 = ptr;
    ptr = ptr.offset(1);
    *fresh6 = '0' as i32 as u8;
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = '\0' as i32 as u8;
    if write(
        fd,
        template.as_mut_ptr() as *const libc::c_void,
        len.wrapping_add(8 as i32 as u64),
    ) < len.wrapping_add(8 as i32 as u64) as ssize_t
    {
        close(fd);
        return 0 as i32 as i8;
    }
    ptr = (*mit_cookie).data;
    len = (*mit_cookie).len as size_t;
    if eat(&mut ptr, &mut len, 1 as i32 as u8) != 0 || eat(&mut ptr, &mut len, *ptr) != 0
        || eat(&mut ptr, &mut len, *ptr) != 0
    {
        destroyPacket(mit_cookie);
        unlink(XauFileName());
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_BADPACKET as i32 as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        return 0 as i32 as i8;
    }
    if write(fd, ptr as *const libc::c_void, len) < len as ssize_t {
        close(fd);
        return 0 as i32 as i8;
    }
    close(fd);
    destroyPacket(mit_cookie);
    displ = XOpenDisplay(dispName);
    if displ.is_null() {
        unlink(XauFileName());
        put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_AUTHFAILED as i32 as Dword);
        send_packet(reply, sock);
        destroyPacket(reply);
        return 0 as i32 as i8;
    }
    XCloseDisplay(displ);
    put_dword(reply, 0 as i32, AuthErrorsEnum::AUTH_SUCCESS as i32 as Dword);
    send_packet(reply, sock);
    destroyPacket(reply);
    unlink(XauFileName());
    return 1 as i32 as i8;
}
unsafe extern "C" fn getportnum(mut portnum: *mut i8) -> uint16_t {
    let mut digits: *mut i8 = portnum;
    let mut serv: *mut servent = 0 as *mut servent;
    let mut port: uint16_t = 0;
    port = 0 as i32 as uint16_t;
    while *(*__ctype_b_loc()).offset(*digits as i32 as isize) as i32
        & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        port = (port as i32 * 10 as i32
            + (*digits as i32 - '0' as i32) as uint8_t as i32) as uint16_t;
        digits = digits.offset(1);
        digits;
    }
    if *digits as i32 != '\0' as i32 || port as i32 <= 0 as i32 {
        serv = getservbyname(portnum, b"tcp\0" as *const u8 as *const i8);
        if !serv.is_null() {
            port = ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = (*serv).s_port as uint16_t;
                if 0 != 0 {
                    __v = (__x as i32 >> 8 as i32 & 0xff as i32
                        | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
                } else {
                    let fresh8 = &mut __v;
                    let fresh9;
                    let fresh10 = __x;
                    asm!(
                        "rorw $8, {0:x}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10) => fresh9,
                        options(pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
                }
                __v
            });
        } else {
            port = 0 as i32 as uint16_t;
        }
        endservent();
    }
    return port;
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
unsafe extern "C" fn getuserid(mut user: *mut i8) -> uid_t {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut uid: uid_t = 0;
    pw = getpwnam(user);
    if !pw.is_null() {
        uid = (*pw).pw_uid;
    } else if *user as i32 == '#' as i32 {
        uid = atoi(&mut *user.offset(1 as i32 as isize)) as uid_t;
    } else {
        uid = 65535 as i32 as uid_t;
    }
    endpwent();
    return uid;
}
unsafe extern "C" fn getgroupid(mut uid: uid_t) -> uid_t {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut gid: gid_t = 0;
    pw = getpwuid(uid);
    if !pw.is_null() {
        gid = (*pw).pw_gid;
    } else {
        gid = 65535 as i32 as gid_t;
    }
    endpwent();
    return gid;
}
unsafe extern "C" fn bind_to_port(
    mut bind_ip: in_addr_t,
    mut bind_port: uint16_t,
) -> i32 {
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sock: i32 = 0;
    sock = socket(2 as i32, __socket_type::SOCK_STREAM as i32, 0 as i32);
    if sock < 0 as i32 {
        perror(b"socket()\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    let mut on: i32 = 1 as i32;
    if setsockopt(
        sock,
        1 as i32,
        2 as i32,
        &mut on as *mut i32 as *mut i8 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    ) < 0 as i32
    {
        perror(b"setsockopt\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    addr.sin_family = 2 as i32 as sa_family_t;
    addr.sin_port = ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = bind_port;
        if 0 != 0 {
            __v = (__x as i32 >> 8 as i32 & 0xff as i32
                | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
        } else {
            let fresh11 = &mut __v;
            let fresh12;
            let fresh13 = __x;
            asm!(
                "rorw $8, {0:x}", inlateout(reg)
                c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13) => fresh12,
                options(pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
        }
        __v
    });
    addr.sin_addr.s_addr = bind_ip;
    if bind(
        sock,
        &mut addr as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as u64 as socklen_t,
    ) < 0 as i32
    {
        perror(b"bind()\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if listen(sock, 128 as i32) < 0 as i32 {
        perror(b"listen()\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    return sock;
}
static mut sockethandle_now: i32 = -(1 as i32);
unsafe extern "C" fn alarm_signal(mut a: i32) -> ! {
    if sockethandle_now != -(1 as i32) {
        close(sockethandle_now);
        sockethandle_now = -(1 as i32);
        unlink(XauFileName());
    }
    exit(1 as i32);
}
unsafe extern "C" fn server_main_loop(
    mut sock: i32,
    mut device_name: *const *const i8,
    mut n_dev: u32,
) -> ! {
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut len: u32 = 0;
    signal(
        17 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    loop {
        let mut new_sock: i32 = 0;
        len = ::core::mem::size_of::<sockaddr_in>() as u64 as u32;
        loop {
            new_sock = accept(
                sock,
                &mut addr as *mut sockaddr_in as *mut sockaddr,
                &mut len,
            );
            if !(new_sock < 0 as i32) {
                break;
            }
        }
        match fork() {
            0 => {
                serve_client(new_sock, device_name, n_dev, 0 as i32);
                exit(0 as i32);
            }
            -1 | _ => {}
        }
        close(new_sock);
        new_sock = -(1 as i32);
    };
}
unsafe extern "C" fn usage(mut prog: *mut i8, mut opt: *const i8, mut ret: i32) -> ! {
    if !opt.is_null() {
        fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const i8, prog, opt);
    }
    fprintf(
        stderr,
        b"usage: %s [-s port [-r user] [-b ipaddr]] devicename [Names of local host]\n\0"
            as *const u8 as *const i8,
        prog,
    );
    fprintf(
        stderr,
        b"    -d          Run as a server (default port 5703 + DISPLAY)\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"    -s port     Run as a server bound to the specified port.\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"    -r user     Run as the specified user in server mode.\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"    -b ipaddr   Bind to the specified ipaddr in server mode.\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"    -l          Do not attempt to connect to localhost:0 to validate connection\n\0"
            as *const u8 as *const i8,
    );
    exit(ret);
}
unsafe extern "C" fn makeDisplayName(mut dispNr: i32) -> *mut i8 {
    let mut result: [i8; 80] = [0; 80];
    sprintf(result.as_mut_ptr(), b":%d.0\0" as *const u8 as *const i8, dispNr);
    return strdup(result.as_mut_ptr());
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut sockfd: i32 = 0 as i32;
    let mut arg: i32 = 0;
    let mut run_as_server: i32 = 0 as i32;
    let mut bind_ip: in_addr_t = 0 as i32 as in_addr_t;
    let mut bind_port: uint16_t = 0 as i32 as uint16_t;
    let mut run_uid: uid_t = 65535 as i32 as uid_t;
    let mut run_gid: gid_t = 65535 as i32 as gid_t;
    let mut username: *mut i8 = strdup(b"nobody\0" as *const u8 as *const i8);
    let mut sock: i32 = 0;
    let mut device_name: *const *const i8 = 0 as *const *const i8;
    let mut floppy0: *const i8 = b"/dev/fd0\0" as *const u8 as *const i8;
    let mut n_dev: u32 = 0;
    if argc > 1 as i32
        && strcmp(*argv.offset(0 as i32 as isize), b"--help\0" as *const u8 as *const i8)
            == 0
    {
        usage(*argv.offset(0 as i32 as isize), 0 as *const i8, 0 as i32);
    }
    loop {
        arg = getopt(argc, argv, b"ds:r:b:x:h\0" as *const u8 as *const i8);
        if !(arg != -(1 as i32)) {
            break;
        }
        match arg {
            100 => {
                run_as_server = 1 as i32;
            }
            115 => {
                run_as_server = 1 as i32;
                bind_port = getportnum(optarg);
            }
            114 => {
                free(username as *mut libc::c_void);
                username = strdup(optarg);
                run_uid = getuserid(optarg);
                run_gid = getgroupid(run_uid);
            }
            98 => {
                run_as_server = 1 as i32;
                bind_ip = getipaddress(optarg);
            }
            120 => {
                dispName = strdup(optarg);
            }
            104 => {
                usage(*argv.offset(0 as i32 as isize), 0 as *const i8, 0 as i32);
            }
            63 => {
                usage(*argv.offset(0 as i32 as isize), 0 as *const i8, 1 as i32);
            }
            _ => {}
        }
    }
    if optind < argc {
        device_name = (argv as *const *const i8).offset(optind as isize);
        n_dev = (argc - optind) as u32;
    } else {
        device_name = &mut floppy0;
        n_dev = 1 as i32 as u32;
    }
    if dispName.is_null() {
        dispName = getenv(b"DISPLAY\0" as *const u8 as *const i8);
    }
    if dispName.is_null() && bind_port as i32 != 0 as i32 {
        dispName = makeDisplayName(
            (bind_port as i32 - 5703 as i32) as libc::c_ushort as i32,
        );
    }
    if dispName.is_null() {
        dispName = b":0\0" as *const u8 as *const i8;
    }
    if bind_port as i32 == 0 as i32 {
        let mut p: *mut i8 = strchr(dispName, ':' as i32);
        bind_port = 5703 as i32 as uint16_t;
        if !p.is_null() {
            bind_port = (bind_port as i32 + atoi(p.offset(1 as i32 as isize)))
                as uint16_t;
        }
    }
    if run_as_server == 0 {
        let mut addr: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut len: u32 = ::core::mem::size_of::<sockaddr_in>() as u64 as u32;
        if getsockname(
            0 as i32,
            &mut addr as *mut sockaddr_in as *mut sockaddr,
            &mut len,
        ) >= 0 as i32 && len as u64 == ::core::mem::size_of::<sockaddr_in>() as u64
        {
            bind_port = ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = addr.sin_port;
                if 0 != 0 {
                    __v = (__x as i32 >> 8 as i32 & 0xff as i32
                        | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
                } else {
                    let fresh14 = &mut __v;
                    let fresh15;
                    let fresh16 = __x;
                    asm!(
                        "rorw $8, {0:x}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16) => fresh15,
                        options(pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
                }
                __v
            });
        }
    }
    umask(0o77 as i32 as __mode_t);
    if run_as_server != 0 && bind_ip == 0xffffffff as u32 {
        usage(
            *argv.offset(0 as i32 as isize),
            b"The server ipaddr is invalid.\0" as *const u8 as *const i8,
            1 as i32,
        );
    }
    if run_as_server != 0 && bind_port as i32 == 0 as i32 {
        usage(
            *argv.offset(0 as i32 as isize),
            b"No server port was specified (or it was invalid).\0" as *const u8
                as *const i8,
            1 as i32,
        );
    }
    if run_as_server != 0 {
        sock = bind_to_port(bind_ip, bind_port);
        match fork() {
            -1 => {
                perror(b"fork()\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
            0 => {
                signal(
                    1 as i32,
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as i32 as libc::intptr_t),
                );
                signal(
                    3 as i32,
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as i32 as libc::intptr_t),
                );
                signal(
                    20 as i32,
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as i32 as libc::intptr_t),
                );
                signal(
                    18 as i32,
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as i32 as libc::intptr_t),
                );
                signal(
                    13 as i32,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(i32) -> !>,
                        __sighandler_t,
                    >(Some(alarm_signal as unsafe extern "C" fn(i32) -> !)),
                );
                setgid(run_gid);
                initgroups(username, run_gid);
                setuid(run_uid);
                setsid();
                setpgrp();
                server_main_loop(sock, device_name, n_dev);
            }
            _ => {}
        }
        exit(0 as i32);
    }
    signal(
        1 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(i32) -> !)),
    );
    signal(
        2 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(i32) -> !)),
    );
    signal(
        3 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(i32) -> !)),
    );
    signal(
        15 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(i32) -> !)),
    );
    signal(
        20 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    signal(
        18 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    signal(
        13 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(alarm_signal as unsafe extern "C" fn(i32) -> !)),
    );
    serve_client(sockfd, device_name, n_dev, 1 as i32);
    return 0 as i32;
}
unsafe extern "C" fn send_reply(mut rval: i32, mut sock: io_buffer, mut len: Dword) {
    let mut reply: Packet = newPacket();
    make_new(reply, 8 as i32 as Dword);
    put_dword(reply, 0 as i32, len);
    if rval == -(1 as i32) {
        put_dword(reply, 4 as i32, 0 as i32 as Dword);
    } else {
        put_dword(reply, 4 as i32, *__errno_location() as Dword);
    }
    send_packet(reply, sock);
    destroyPacket(reply);
}
unsafe extern "C" fn send_reply64(
    mut rval: i32,
    mut sock: io_buffer,
    mut len: mt_off_t,
) {
    let mut reply: Packet = newPacket();
    make_new(reply, 12 as i32 as Dword);
    put_qword(reply, 0 as i32, len as Qword);
    if rval == -(1 as i32) {
        put_dword(reply, 8 as i32, 0 as i32 as Dword);
    } else {
        put_dword(reply, 8 as i32, *__errno_location() as Dword);
    }
    send_packet(reply, sock);
    destroyPacket(reply);
}
unsafe extern "C" fn cleanup(mut x: i32) -> ! {
    unlink(XauFileName());
    exit(-(1 as i32));
}
#[no_mangle]
pub unsafe extern "C" fn serve_client(
    mut sockhandle: i32,
    mut device_name: *const *const i8,
    mut n_dev: u32,
    mut close_stderr: i32,
) {
    let mut opcode: Packet = 0 as *mut Packet_0;
    let mut parm: Packet = 0 as *mut Packet_0;
    let mut readOnly: i32 = 0;
    let mut devFd: i32 = 0;
    let mut sock: io_buffer = 0 as *mut io_buffer_0;
    let mut stopLoop: i32 = 0;
    let mut version: u32 = 0;
    let mut needSendReply: i32 = 0 as i32;
    let mut rval: i32 = 0 as i32;
    let mut on: i32 = 1 as i32;
    if setsockopt(
        sockhandle,
        1 as i32,
        9 as i32,
        &mut on as *mut i32 as *mut i8 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    ) < 0 as i32
    {
        perror(b"setsockopt\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if close_stderr != 0 {
        close(2 as i32);
        open(b"/dev/null\0" as *const u8 as *const i8, 0o1 as i32);
    }
    sock = new_io_buffer(sockhandle);
    alarm(60 as i32 as u32);
    version = 0 as i32 as u32;
    if do_auth(sock, &mut version) == 0 {
        free_io_buffer(sock);
        return;
    }
    alarm(0 as i32 as u32);
    signal(
        15 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(cleanup as unsafe extern "C" fn(i32) -> !)),
    );
    signal(
        14 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32) -> !>,
            __sighandler_t,
        >(Some(cleanup as unsafe extern "C" fn(i32) -> !)),
    );
    sockethandle_now = sockhandle;
    opcode = newPacket();
    parm = newPacket();
    devFd = -(1 as i32);
    readOnly = 1 as i32;
    stopLoop = 0 as i32;
    if version == 10 as i32 as u32 {
        readOnly = 0 as i32;
        devFd = open(*device_name.offset(0 as i32 as isize), 0o2 as i32 | 0 as i32);
        if devFd < 0 as i32 {
            readOnly = 1 as i32;
            devFd = open(*device_name.offset(0 as i32 as isize), 0 as i32 | 0 as i32);
        }
        if devFd < 0 as i32 {
            send_reply(
                0 as i32,
                sock,
                if devFd >= 0 as i32 { 0 as i32 as u32 } else { -(1 as i32) as Dword },
            );
            stopLoop = 1 as i32;
        }
        lock_dev(devFd, (readOnly == 0) as i32, 0 as *mut device);
    }
    while stopLoop == 0 {
        let mut dev_nr: uint32_t = 0 as i32 as uint32_t;
        if recv_packet(opcode, sock, 1 as i32 as Dword) == 0 {
            break;
        }
        recv_packet(parm, sock, 3000000 as i32 as Dword);
        cork((*sock).handle, 1 as i32);
        match *((*opcode).data).offset(0 as i32 as isize) as i32 {
            6 => {
                if get_length(parm) >= 4 as i32 as u32 {
                    dev_nr = get_dword(parm, 0 as i32);
                } else {
                    dev_nr = 0 as i32 as uint32_t;
                }
                if dev_nr >= n_dev {
                    send_reply(0 as i32, sock, -(1 as i32) as Dword);
                } else {
                    devFd = open(
                        *device_name.offset(dev_nr as isize),
                        0 as i32 | 0 as i32,
                    );
                    if devFd >= 0 as i32
                        && lock_dev(devFd, 0 as i32, 0 as *mut device) != 0
                    {
                        send_reply(0 as i32, sock, -(1 as i32) as Dword);
                    } else {
                        send_reply(
                            0 as i32,
                            sock,
                            if devFd >= 0 as i32 {
                                0 as i32 as u32
                            } else {
                                -(1 as i32) as Dword
                            },
                        );
                        readOnly = 1 as i32;
                    }
                }
            }
            7 => {
                if get_length(parm) >= 4 as i32 as u32 {
                    dev_nr = get_dword(parm, 0 as i32);
                } else {
                    dev_nr = 0 as i32 as uint32_t;
                }
                if dev_nr >= n_dev {
                    send_reply(0 as i32, sock, -(1 as i32) as Dword);
                } else {
                    devFd = open(*device_name.offset(dev_nr as isize), 0o2 as i32);
                    if devFd >= 0 as i32
                        && lock_dev(devFd, 1 as i32, 0 as *mut device) != 0
                    {
                        send_reply(0 as i32, sock, -(1 as i32) as Dword);
                    } else {
                        send_reply(
                            0 as i32,
                            sock,
                            if devFd >= 0 as i32 {
                                0 as i32 as u32
                            } else {
                                -(1 as i32) as Dword
                            },
                        );
                        readOnly = 0 as i32;
                    }
                }
            }
            0 => {
                if read_packet(parm, devFd, get_dword(parm, 0 as i32)) < 0 as i32 as i64
                {
                    send_reply(devFd, sock, -(1 as i32) as Dword);
                } else {
                    send_reply(devFd, sock, get_length(parm));
                    send_packet(parm, sock);
                }
            }
            1 => {
                if readOnly != 0 {
                    *__errno_location() = -(30 as i32);
                    rval = -(1 as i32);
                } else {
                    rval = write_packet(parm, devFd);
                }
                send_reply(devFd, sock, rval as Dword);
            }
            2 => {
                lseek(
                    devFd,
                    get_dword(parm, 0 as i32) as off_t,
                    get_dword(parm, 4 as i32) as i32,
                );
                send_reply(
                    devFd,
                    sock,
                    lseek(devFd, 0 as i32 as __off_t, 1 as i32) as Dword,
                );
            }
            8 => {
                if (::core::mem::size_of::<mt_off_t>() as u64) < 8 as i32 as u64 {
                    *__errno_location() = 22 as i32;
                    send_reply(devFd, sock, -(1 as i32) as Dword);
                } else {
                    mt_lseek(
                        devFd,
                        get_qword(parm, 0 as i32) as mt_off_t,
                        get_dword(parm, 8 as i32) as i32,
                    );
                    send_reply64(
                        devFd,
                        sock,
                        mt_lseek(devFd, 0 as i32 as mt_off_t, 1 as i32) as mt_off_t,
                    );
                }
            }
            3 => {
                fsync(devFd);
                send_reply(devFd, sock, 0 as i32 as Dword);
            }
            4 => {
                close(devFd);
                needSendReply = 1 as i32;
                rval = devFd;
                devFd = -(1 as i32);
                stopLoop = 1 as i32;
            }
            5 => {}
            _ => {
                *__errno_location() = 22 as i32;
                send_reply(devFd, sock, -(1 as i32) as Dword);
            }
        }
        cork((*sock).handle, 0 as i32);
        kill_packet(parm);
        alarm(0 as i32 as u32);
    }
    if devFd >= 0 as i32 {
        close(devFd);
        devFd = -(1 as i32);
    }
    free_io_buffer(sock);
    unlink(XauFileName());
    if needSendReply != 0 {
        send_reply(rval, sock, 0 as i32 as Dword);
    }
    destroyPacket(opcode);
    destroyPacket(parm);
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