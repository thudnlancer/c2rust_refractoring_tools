#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn getsockname(__fd: i32, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> i32;
    fn getpeername(__fd: i32, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> i32;
    fn send(__fd: i32, __buf: *const libc::c_void, __n: size_t, __flags: i32) -> ssize_t;
    fn recv(__fd: i32, __buf: *mut libc::c_void, __n: size_t, __flags: i32) -> ssize_t;
    fn getsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> i32;
    fn setsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> i32;
    fn getaddrinfo(
        __name: *const i8,
        __service: *const i8,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> i32;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: i32) -> *const i8;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut i8,
        __hostlen: socklen_t,
        __serv: *mut i8,
        __servlen: socklen_t,
        __flags: i32,
    ) -> i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn _log(file: *const i8, line: i32, panic: i32, fmt: *const i8, _: ...);
    fn log_loggable(level: i32) -> i32;
    fn backtrace(__array: *mut *mut libc::c_void, __size: i32) -> i32;
    fn backtrace_symbols(__array: *const *mut libc::c_void, __size: i32) -> *mut *mut i8;
    fn backtrace_symbols_fd(__array: *const *mut libc::c_void, __size: i32, __fd: i32);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type socklen_t = __socklen_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linger {
    pub l_onoff: i32,
    pub l_linger: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [i8; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
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
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut i8,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub family: i32,
    pub addrlen: socklen_t,
    pub addr: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_blocking(mut sd: i32) -> i32 {
    let mut flags: i32 = 0;
    flags = fcntl(sd, 3 as i32, 0 as i32);
    if flags < 0 as i32 {
        return flags;
    }
    return fcntl(sd, 4 as i32, flags & !(0o4000 as i32));
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_nonblocking(mut sd: i32) -> i32 {
    let mut flags: i32 = 0;
    flags = fcntl(sd, 3 as i32, 0 as i32);
    if flags < 0 as i32 {
        return flags;
    }
    return fcntl(sd, 4 as i32, flags | 0o4000 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_reuseaddr(mut sd: i32) -> i32 {
    let mut reuse: i32 = 0;
    let mut len: socklen_t = 0;
    reuse = 1 as i32;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    return setsockopt(
        sd,
        1 as i32,
        2 as i32,
        &mut reuse as *mut i32 as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_reuseport(mut sd: i32) -> i32 {
    let mut reuse: i32 = 0;
    let mut len: socklen_t = 0;
    reuse = 1 as i32;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    return setsockopt(
        sd,
        1 as i32,
        15 as i32,
        &mut reuse as *mut i32 as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_tcpnodelay(mut sd: i32) -> i32 {
    let mut nodelay: i32 = 0;
    let mut len: socklen_t = 0;
    nodelay = 1 as i32;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    return setsockopt(
        sd,
        C2RustUnnamed_0::IPPROTO_TCP as i32,
        1 as i32,
        &mut nodelay as *mut i32 as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_linger(mut sd: i32, mut timeout: i32) -> i32 {
    let mut linger: linger = linger { l_onoff: 0, l_linger: 0 };
    let mut len: socklen_t = 0;
    linger.l_onoff = 1 as i32;
    linger.l_linger = timeout;
    len = ::core::mem::size_of::<linger>() as u64 as socklen_t;
    return setsockopt(
        sd,
        1 as i32,
        13 as i32,
        &mut linger as *mut linger as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_tcpkeepalive(mut sd: i32) -> i32 {
    let mut val: i32 = 1 as i32;
    return setsockopt(
        sd,
        1 as i32,
        9 as i32,
        &mut val as *mut i32 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_sndbuf(mut sd: i32, mut size: i32) -> i32 {
    let mut len: socklen_t = 0;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    return setsockopt(
        sd,
        1 as i32,
        7 as i32,
        &mut size as *mut i32 as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_rcvbuf(mut sd: i32, mut size: i32) -> i32 {
    let mut len: socklen_t = 0;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    return setsockopt(
        sd,
        1 as i32,
        8 as i32,
        &mut size as *mut i32 as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_get_soerror(mut sd: i32) -> i32 {
    let mut status: i32 = 0;
    let mut err: i32 = 0;
    let mut len: socklen_t = 0;
    err = 0 as i32;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    status = getsockopt(
        sd,
        1 as i32,
        4 as i32,
        &mut err as *mut i32 as *mut libc::c_void,
        &mut len,
    );
    if status == 0 as i32 {
        *__errno_location() = err;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn nc_get_sndbuf(mut sd: i32) -> i32 {
    let mut status: i32 = 0;
    let mut size: i32 = 0;
    let mut len: socklen_t = 0;
    size = 0 as i32;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    status = getsockopt(
        sd,
        1 as i32,
        7 as i32,
        &mut size as *mut i32 as *mut libc::c_void,
        &mut len,
    );
    if status < 0 as i32 {
        return status;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn nc_get_rcvbuf(mut sd: i32) -> i32 {
    let mut status: i32 = 0;
    let mut size: i32 = 0;
    let mut len: socklen_t = 0;
    size = 0 as i32;
    len = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    status = getsockopt(
        sd,
        1 as i32,
        8 as i32,
        &mut size as *mut i32 as *mut libc::c_void,
        &mut len,
    );
    if status < 0 as i32 {
        return status;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_atoi(mut line: *const uint8_t, mut n: size_t) -> i32 {
    let mut value: i32 = 0;
    if n == 0 as i32 as u64 {
        return -(1 as i32);
    }
    value = 0 as i32;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        if (*line as i32) < '0' as i32 || *line as i32 > '9' as i32 {
            return -(1 as i32);
        }
        value = value * 10 as i32 + (*line as i32 - '0' as i32);
        line = line.offset(1);
        line;
    }
    if value < 0 as i32 {
        return -(1 as i32);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn nc_valid_port(mut n: i32) -> bool {
    if n < 1 as i32 || n > 65535 as i32 {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_alloc(
    mut size: size_t,
    mut name: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = malloc(size);
    if p.is_null() {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_util.c\0" as *const u8 as *const i8,
                247 as i32,
                0 as i32,
                b"malloc(%zu) failed @ %s:%d\0" as *const u8 as *const i8,
                size,
                name,
                line,
            );
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_zalloc(
    mut size: size_t,
    mut name: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = _nc_alloc(size, name, line);
    if !p.is_null() {
        memset(p, 0 as i32, size);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_calloc(
    mut nmemb: size_t,
    mut size: size_t,
    mut name: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    return _nc_zalloc(nmemb.wrapping_mul(size), name, line);
}
#[no_mangle]
pub unsafe extern "C" fn _nc_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut name: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = realloc(ptr, size);
    if p.is_null() {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_util.c\0" as *const u8 as *const i8,
                283 as i32,
                0 as i32,
                b"realloc(%zu) failed @ %s:%d\0" as *const u8 as *const i8,
                size,
                name,
                line,
            );
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_free(
    mut ptr: *mut libc::c_void,
    mut name: *const i8,
    mut line: i32,
) {
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn nc_stacktrace(mut skip_count: i32) {
    let mut stack: [*mut libc::c_void; 64] = [0 as *mut libc::c_void; 64];
    let mut symbols: *mut *mut i8 = 0 as *mut *mut i8;
    let mut size: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    size = backtrace(stack.as_mut_ptr(), 64 as i32);
    symbols = backtrace_symbols(stack.as_mut_ptr(), size);
    if symbols.is_null() {
        return;
    }
    skip_count += 1;
    skip_count;
    i = skip_count;
    j = 0 as i32;
    while i < size {
        _log(
            b"nc_util.c\0" as *const u8 as *const i8,
            316 as i32,
            0 as i32,
            b"[%d] %s\0" as *const u8 as *const i8,
            j,
            *symbols.offset(i as isize),
        );
        i += 1;
        i;
        j += 1;
        j;
    }
    free(symbols as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nc_stacktrace_fd(mut fd: i32) {
    let mut stack: [*mut libc::c_void; 64] = [0 as *mut libc::c_void; 64];
    let mut size: i32 = 0;
    size = backtrace(stack.as_mut_ptr(), 64 as i32);
    backtrace_symbols_fd(stack.as_mut_ptr(), size, fd);
}
#[no_mangle]
pub unsafe extern "C" fn nc_assert(
    mut cond: *const i8,
    mut file: *const i8,
    mut line: i32,
    mut panic: i32,
) {
    if log_loggable(1 as i32) != 0 as i32 {
        _log(
            b"nc_util.c\0" as *const u8 as *const i8,
            338 as i32,
            0 as i32,
            b"assert '%s' failed @ (%s, %d)\0" as *const u8 as *const i8,
            cond,
            file,
            line,
        );
    }
    if panic != 0 {
        nc_stacktrace(1 as i32);
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn _vscnprintf(
    mut buf: *mut i8,
    mut size: size_t,
    mut fmt: *const i8,
    mut args: ::core::ffi::VaList,
) -> i32 {
    let mut n: i32 = 0;
    n = vsnprintf(buf, size, fmt, args.as_va_list());
    if n <= 0 as i32 {
        return 0 as i32;
    }
    if n < size as i32 {
        return n;
    }
    return size.wrapping_sub(1 as i32 as u64) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _scnprintf(
    mut buf: *mut i8,
    mut size: size_t,
    mut fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: i32 = 0;
    args_0 = args.clone();
    n = _vscnprintf(buf, size, fmt, args_0.as_va_list());
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_sendn(
    mut sd: i32,
    mut vptr: *const libc::c_void,
    mut n: size_t,
) -> ssize_t {
    let mut nleft: size_t = 0;
    let mut nsend: ssize_t = 0;
    let mut ptr: *const i8 = 0 as *const i8;
    ptr = vptr as *const i8;
    nleft = n;
    while nleft > 0 as i32 as u64 {
        nsend = send(sd, ptr as *const libc::c_void, nleft, 0 as i32);
        if nsend < 0 as i32 as i64 {
            if *__errno_location() == 4 as i32 {
                continue;
            }
            return nsend;
        } else {
            if nsend == 0 as i32 as i64 {
                return -(1 as i32) as ssize_t;
            }
            nleft = (nleft as u64).wrapping_sub(nsend as size_t) as size_t as size_t;
            ptr = ptr.offset(nsend as isize);
        }
    }
    return n as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_recvn(
    mut sd: i32,
    mut vptr: *mut libc::c_void,
    mut n: size_t,
) -> ssize_t {
    let mut nleft: size_t = 0;
    let mut nrecv: ssize_t = 0;
    let mut ptr: *mut i8 = 0 as *mut i8;
    ptr = vptr as *mut i8;
    nleft = n;
    while nleft > 0 as i32 as u64 {
        nrecv = recv(sd, ptr as *mut libc::c_void, nleft, 0 as i32);
        if nrecv < 0 as i32 as i64 {
            if *__errno_location() == 4 as i32 {
                continue;
            }
            return nrecv;
        } else {
            if nrecv == 0 as i32 as i64 {
                break;
            }
            nleft = (nleft as u64).wrapping_sub(nrecv as size_t) as size_t as size_t;
            ptr = ptr.offset(nrecv as isize);
        }
    }
    return n.wrapping_sub(nleft) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn nc_usec_now() -> int64_t {
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut usec: int64_t = 0;
    let mut status: i32 = 0;
    status = gettimeofday(&mut now, 0 as *mut timezone);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_util.c\0" as *const u8 as *const i8,
                460 as i32,
                0 as i32,
                b"gettimeofday failed: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32) as int64_t;
    }
    usec = (now.tv_sec as libc::c_longlong * 1000000 as libc::c_longlong
        + now.tv_usec as libc::c_longlong) as int64_t;
    return usec;
}
#[no_mangle]
pub unsafe extern "C" fn nc_msec_now() -> int64_t {
    return (nc_usec_now() as libc::c_longlong / 1000 as libc::c_longlong) as int64_t;
}
unsafe extern "C" fn nc_resolve_inet(
    mut name: *const string,
    mut port: i32,
    mut si: *mut sockinfo,
) -> i32 {
    let mut status: i32 = 0;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut cai: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut i8,
        ai_next: 0 as *mut addrinfo,
    };
    let mut node: *mut i8 = 0 as *mut i8;
    let mut service: [i8; 21] = [0; 21];
    let mut found: bool = false;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<addrinfo>() as u64,
    );
    hints.ai_flags = 0x400 as i32;
    hints.ai_family = 0 as i32;
    hints.ai_socktype = __socket_type::SOCK_STREAM as i32;
    hints.ai_protocol = 0 as i32;
    hints.ai_addrlen = 0 as i32 as socklen_t;
    hints.ai_addr = 0 as *mut sockaddr;
    hints.ai_canonname = 0 as *mut i8;
    if !name.is_null() {
        node = (*name).data as *mut i8;
    } else {
        node = 0 as *mut i8;
        hints.ai_flags |= 0x1 as i32;
    }
    snprintf(
        service.as_mut_ptr(),
        (20 as i32 + 1 as i32) as size_t,
        b"%d\0" as *const u8 as *const i8,
        port,
    );
    status = getaddrinfo(node, service.as_mut_ptr(), &mut hints, &mut ai);
    if status != 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_util.c\0" as *const u8 as *const i8,
                520 as i32,
                0 as i32,
                b"address resolution of node '%s' service '%s' failed: %s\0" as *const u8
                    as *const i8,
                node,
                service.as_mut_ptr(),
                gai_strerror(status),
            );
        }
        return -(1 as i32);
    }
    cai = ai;
    found = 0 as i32 != 0;
    if !cai.is_null() {
        (*si).family = (*cai).ai_family;
        (*si).addrlen = (*cai).ai_addrlen;
        memcpy(
            &mut (*si).addr as *mut C2RustUnnamed_1 as *mut libc::c_void,
            (*cai).ai_addr as *const libc::c_void,
            (*si).addrlen as size_t,
        );
        found = 1 as i32 != 0;
    }
    freeaddrinfo(ai);
    return if !found { -(1 as i32) } else { 0 as i32 };
}
unsafe extern "C" fn nc_resolve_unix(
    mut name: *const string,
    mut si: *mut sockinfo,
) -> i32 {
    let mut un: *mut sockaddr_un = 0 as *mut sockaddr_un;
    if (*name).len as u64
        >= (::core::mem::size_of::<sockaddr_un>() as u64).wrapping_sub(2 as u64)
    {
        return -(1 as i32);
    }
    un = &mut (*si).addr.un;
    (*un).sun_family = 1 as i32 as sa_family_t;
    memcpy(
        ((*un).sun_path).as_mut_ptr() as *mut libc::c_void,
        (*name).data as *const libc::c_void,
        (*name).len as size_t,
    );
    (*un).sun_path[(*name).len as usize] = '\0' as i32 as i8;
    (*si).family = 1 as i32;
    (*si).addrlen = ::core::mem::size_of::<sockaddr_un>() as u64 as socklen_t;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn nc_resolve(
    mut name: *const string,
    mut port: i32,
    mut si: *mut sockinfo,
) -> i32 {
    if !name.is_null() && *((*name).data).offset(0 as i32 as isize) as i32 == '/' as i32
    {
        return nc_resolve_unix(name, si);
    }
    return nc_resolve_inet(name, port, si);
}
#[no_mangle]
pub unsafe extern "C" fn nc_unresolve_addr(
    mut addr: *mut sockaddr,
    mut addrlen: socklen_t,
) -> *const i8 {
    static mut unresolve: [i8; 1057] = [0; 1057];
    static mut host: [i8; 1025] = [0; 1025];
    static mut service: [i8; 32] = [0; 32];
    let mut status: i32 = 0;
    status = getnameinfo(
        addr,
        addrlen,
        host.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 1025]>() as u64 as socklen_t,
        service.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 32]>() as u64 as socklen_t,
        1 as i32 | 2 as i32,
    );
    if status < 0 as i32 {
        return b"unknown\0" as *const u8 as *const i8;
    }
    snprintf(
        unresolve.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 1057]>() as u64,
        b"%s:%s\0" as *const u8 as *const i8,
        host.as_mut_ptr(),
        service.as_mut_ptr(),
    );
    return unresolve.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn nc_unresolve_peer_desc(mut sd: i32) -> *const i8 {
    static mut si: sockinfo = sockinfo {
        family: 0,
        addrlen: 0,
        addr: C2RustUnnamed_1 {
            in_0: sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            },
        },
    };
    let mut addr: *mut sockaddr = 0 as *mut sockaddr;
    let mut addrlen: socklen_t = 0;
    let mut status: i32 = 0;
    memset(
        &mut si as *mut sockinfo as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<sockinfo>() as u64,
    );
    addr = &mut si.addr as *mut C2RustUnnamed_1 as *mut sockaddr;
    addrlen = ::core::mem::size_of::<C2RustUnnamed_1>() as u64 as socklen_t;
    status = getpeername(
        sd,
        __SOCKADDR_ARG {
            __sockaddr__: addr,
        },
        &mut addrlen,
    );
    if status < 0 as i32 {
        return b"unknown\0" as *const u8 as *const i8;
    }
    return nc_unresolve_addr(addr, addrlen);
}
#[no_mangle]
pub unsafe extern "C" fn nc_unresolve_desc(mut sd: i32) -> *const i8 {
    static mut si: sockinfo = sockinfo {
        family: 0,
        addrlen: 0,
        addr: C2RustUnnamed_1 {
            in_0: sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            },
        },
    };
    let mut addr: *mut sockaddr = 0 as *mut sockaddr;
    let mut addrlen: socklen_t = 0;
    let mut status: i32 = 0;
    memset(
        &mut si as *mut sockinfo as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<sockinfo>() as u64,
    );
    addr = &mut si.addr as *mut C2RustUnnamed_1 as *mut sockaddr;
    addrlen = ::core::mem::size_of::<C2RustUnnamed_1>() as u64 as socklen_t;
    status = getsockname(
        sd,
        __SOCKADDR_ARG {
            __sockaddr__: addr,
        },
        &mut addrlen,
    );
    if status < 0 as i32 {
        return b"unknown\0" as *const u8 as *const i8;
    }
    return nc_unresolve_addr(addr, addrlen);
}