#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn getpeername(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn backtrace(__array: *mut *mut libc::c_void, __size: libc::c_int) -> libc::c_int;
    fn backtrace_symbols(
        __array: *const *mut libc::c_void,
        __size: libc::c_int,
    ) -> *mut *mut libc::c_char;
    fn backtrace_symbols_fd(
        __array: *const *mut libc::c_void,
        __size: libc::c_int,
        __fd: libc::c_int,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
}  // end of enum

pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linger {
    pub l_onoff: libc::c_int,
    pub l_linger: libc::c_int,
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
    pub sun_path: [libc::c_char; 108],
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
    pub sin_zero: [libc::c_uchar; 8],
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
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
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
    pub family: libc::c_int,
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
pub unsafe extern "C" fn nc_set_blocking(mut sd: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(sd, 3 as libc::c_int, 0 as libc::c_int);
    if flags < 0 as libc::c_int {
        return flags;
    }
    return fcntl(sd, 4 as libc::c_int, flags & !(0o4000 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_nonblocking(mut sd: libc::c_int) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(sd, 3 as libc::c_int, 0 as libc::c_int);
    if flags < 0 as libc::c_int {
        return flags;
    }
    return fcntl(sd, 4 as libc::c_int, flags | 0o4000 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_reuseaddr(mut sd: libc::c_int) -> libc::c_int {
    let mut reuse: libc::c_int = 0;
    let mut len: socklen_t = 0;
    reuse = 1 as libc::c_int;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    return setsockopt(
        sd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut reuse as *mut libc::c_int as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_reuseport(mut sd: libc::c_int) -> libc::c_int {
    let mut reuse: libc::c_int = 0;
    let mut len: socklen_t = 0;
    reuse = 1 as libc::c_int;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    return setsockopt(
        sd,
        1 as libc::c_int,
        15 as libc::c_int,
        &mut reuse as *mut libc::c_int as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_tcpnodelay(mut sd: libc::c_int) -> libc::c_int {
    let mut nodelay: libc::c_int = 0;
    let mut len: socklen_t = 0;
    nodelay = 1 as libc::c_int;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    return setsockopt(
        sd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut nodelay as *mut libc::c_int as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_linger(
    mut sd: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut linger: linger = linger { l_onoff: 0, l_linger: 0 };
    let mut len: socklen_t = 0;
    linger.l_onoff = 1 as libc::c_int;
    linger.l_linger = timeout;
    len = ::core::mem::size_of::<linger>() as libc::c_ulong as socklen_t;
    return setsockopt(
        sd,
        1 as libc::c_int,
        13 as libc::c_int,
        &mut linger as *mut linger as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_tcpkeepalive(mut sd: libc::c_int) -> libc::c_int {
    let mut val: libc::c_int = 1 as libc::c_int;
    return setsockopt(
        sd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_sndbuf(
    mut sd: libc::c_int,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut len: socklen_t = 0;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    return setsockopt(
        sd,
        1 as libc::c_int,
        7 as libc::c_int,
        &mut size as *mut libc::c_int as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_set_rcvbuf(
    mut sd: libc::c_int,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut len: socklen_t = 0;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    return setsockopt(
        sd,
        1 as libc::c_int,
        8 as libc::c_int,
        &mut size as *mut libc::c_int as *const libc::c_void,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nc_get_soerror(mut sd: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut len: socklen_t = 0;
    err = 0 as libc::c_int;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    status = getsockopt(
        sd,
        1 as libc::c_int,
        4 as libc::c_int,
        &mut err as *mut libc::c_int as *mut libc::c_void,
        &mut len,
    );
    if status == 0 as libc::c_int {
        *__errno_location() = err;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn nc_get_sndbuf(mut sd: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut len: socklen_t = 0;
    size = 0 as libc::c_int;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    status = getsockopt(
        sd,
        1 as libc::c_int,
        7 as libc::c_int,
        &mut size as *mut libc::c_int as *mut libc::c_void,
        &mut len,
    );
    if status < 0 as libc::c_int {
        return status;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn nc_get_rcvbuf(mut sd: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut len: socklen_t = 0;
    size = 0 as libc::c_int;
    len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    status = getsockopt(
        sd,
        1 as libc::c_int,
        8 as libc::c_int,
        &mut size as *mut libc::c_int as *mut libc::c_void,
        &mut len,
    );
    if status < 0 as libc::c_int {
        return status;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_atoi(
    mut line: *const uint8_t,
    mut n: size_t,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    value = 0 as libc::c_int;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        if (*line as libc::c_int) < '0' as i32 || *line as libc::c_int > '9' as i32 {
            return -(1 as libc::c_int);
        }
        value = value * 10 as libc::c_int + (*line as libc::c_int - '0' as i32);
        line = line.offset(1);
        line;
    }
    if value < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn nc_valid_port(mut n: libc::c_int) -> bool {
    if n < 1 as libc::c_int || n > 65535 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_alloc(
    mut size: size_t,
    mut name: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = malloc(size);
    if p.is_null() {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_util.c\0" as *const u8 as *const libc::c_char,
                247 as libc::c_int,
                0 as libc::c_int,
                b"malloc(%zu) failed @ %s:%d\0" as *const u8 as *const libc::c_char,
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
    mut name: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = _nc_alloc(size, name, line);
    if !p.is_null() {
        memset(p, 0 as libc::c_int, size);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_calloc(
    mut nmemb: size_t,
    mut size: size_t,
    mut name: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    return _nc_zalloc(nmemb.wrapping_mul(size), name, line);
}
#[no_mangle]
pub unsafe extern "C" fn _nc_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut name: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = realloc(ptr, size);
    if p.is_null() {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_util.c\0" as *const u8 as *const libc::c_char,
                283 as libc::c_int,
                0 as libc::c_int,
                b"realloc(%zu) failed @ %s:%d\0" as *const u8 as *const libc::c_char,
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
    mut name: *const libc::c_char,
    mut line: libc::c_int,
) {
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn nc_stacktrace(mut skip_count: libc::c_int) {
    let mut stack: [*mut libc::c_void; 64] = [0 as *mut libc::c_void; 64];
    let mut symbols: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    size = backtrace(stack.as_mut_ptr(), 64 as libc::c_int);
    symbols = backtrace_symbols(stack.as_mut_ptr(), size);
    if symbols.is_null() {
        return;
    }
    skip_count += 1;
    skip_count;
    i = skip_count;
    j = 0 as libc::c_int;
    while i < size {
        _log(
            b"nc_util.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            0 as libc::c_int,
            b"[%d] %s\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn nc_stacktrace_fd(mut fd: libc::c_int) {
    let mut stack: [*mut libc::c_void; 64] = [0 as *mut libc::c_void; 64];
    let mut size: libc::c_int = 0;
    size = backtrace(stack.as_mut_ptr(), 64 as libc::c_int);
    backtrace_symbols_fd(stack.as_mut_ptr(), size, fd);
}
#[no_mangle]
pub unsafe extern "C" fn nc_assert(
    mut cond: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut panic: libc::c_int,
) {
    if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
        _log(
            b"nc_util.c\0" as *const u8 as *const libc::c_char,
            338 as libc::c_int,
            0 as libc::c_int,
            b"assert '%s' failed @ (%s, %d)\0" as *const u8 as *const libc::c_char,
            cond,
            file,
            line,
        );
    }
    if panic != 0 {
        nc_stacktrace(1 as libc::c_int);
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn _vscnprintf(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = vsnprintf(buf, size, fmt, args.as_va_list());
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if n < size as libc::c_int {
        return n;
    }
    return size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _scnprintf(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut n: libc::c_int = 0;
    args_0 = args.clone();
    n = _vscnprintf(buf, size, fmt, args_0.as_va_list());
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_sendn(
    mut sd: libc::c_int,
    mut vptr: *const libc::c_void,
    mut n: size_t,
) -> ssize_t {
    let mut nleft: size_t = 0;
    let mut nsend: ssize_t = 0;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    ptr = vptr as *const libc::c_char;
    nleft = n;
    while nleft > 0 as libc::c_int as libc::c_ulong {
        nsend = send(sd, ptr as *const libc::c_void, nleft, 0 as libc::c_int);
        if nsend < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            return nsend;
        } else {
            if nsend == 0 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int) as ssize_t;
            }
            nleft = (nleft as libc::c_ulong).wrapping_sub(nsend as size_t) as size_t
                as size_t;
            ptr = ptr.offset(nsend as isize);
        }
    }
    return n as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn _nc_recvn(
    mut sd: libc::c_int,
    mut vptr: *mut libc::c_void,
    mut n: size_t,
) -> ssize_t {
    let mut nleft: size_t = 0;
    let mut nrecv: ssize_t = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = vptr as *mut libc::c_char;
    nleft = n;
    while nleft > 0 as libc::c_int as libc::c_ulong {
        nrecv = recv(sd, ptr as *mut libc::c_void, nleft, 0 as libc::c_int);
        if nrecv < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            return nrecv;
        } else {
            if nrecv == 0 as libc::c_int as libc::c_long {
                break;
            }
            nleft = (nleft as libc::c_ulong).wrapping_sub(nrecv as size_t) as size_t
                as size_t;
            ptr = ptr.offset(nrecv as isize);
        }
    }
    return n.wrapping_sub(nleft) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn nc_usec_now() -> int64_t {
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut usec: int64_t = 0;
    let mut status: libc::c_int = 0;
    status = gettimeofday(&mut now, 0 as *mut timezone);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_util.c\0" as *const u8 as *const libc::c_char,
                460 as libc::c_int,
                0 as libc::c_int,
                b"gettimeofday failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int) as int64_t;
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
    mut port: libc::c_int,
    mut si: *mut sockinfo,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut cai: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut node: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut service: [libc::c_char; 21] = [0; 21];
    let mut found: bool = false;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0x400 as libc::c_int;
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_protocol = 0 as libc::c_int;
    hints.ai_addrlen = 0 as libc::c_int as socklen_t;
    hints.ai_addr = 0 as *mut sockaddr;
    hints.ai_canonname = 0 as *mut libc::c_char;
    if !name.is_null() {
        node = (*name).data as *mut libc::c_char;
    } else {
        node = 0 as *mut libc::c_char;
        hints.ai_flags |= 0x1 as libc::c_int;
    }
    snprintf(
        service.as_mut_ptr(),
        (20 as libc::c_int + 1 as libc::c_int) as size_t,
        b"%d\0" as *const u8 as *const libc::c_char,
        port,
    );
    status = getaddrinfo(node, service.as_mut_ptr(), &mut hints, &mut ai);
    if status != 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_util.c\0" as *const u8 as *const libc::c_char,
                520 as libc::c_int,
                0 as libc::c_int,
                b"address resolution of node '%s' service '%s' failed: %s\0" as *const u8
                    as *const libc::c_char,
                node,
                service.as_mut_ptr(),
                gai_strerror(status),
            );
        }
        return -(1 as libc::c_int);
    }
    cai = ai;
    found = 0 as libc::c_int != 0;
    if !cai.is_null() {
        (*si).family = (*cai).ai_family;
        (*si).addrlen = (*cai).ai_addrlen;
        memcpy(
            &mut (*si).addr as *mut C2RustUnnamed_1 as *mut libc::c_void,
            (*cai).ai_addr as *const libc::c_void,
            (*si).addrlen as size_t,
        );
        found = 1 as libc::c_int != 0;
    }
    freeaddrinfo(ai);
    return if !found { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
unsafe extern "C" fn nc_resolve_unix(
    mut name: *const string,
    mut si: *mut sockinfo,
) -> libc::c_int {
    let mut un: *mut sockaddr_un = 0 as *mut sockaddr_un;
    if (*name).len as libc::c_ulong
        >= (::core::mem::size_of::<sockaddr_un>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    un = &mut (*si).addr.un;
    (*un).sun_family = 1 as libc::c_int as sa_family_t;
    memcpy(
        ((*un).sun_path).as_mut_ptr() as *mut libc::c_void,
        (*name).data as *const libc::c_void,
        (*name).len as size_t,
    );
    (*un).sun_path[(*name).len as usize] = '\0' as i32 as libc::c_char;
    (*si).family = 1 as libc::c_int;
    (*si).addrlen = ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nc_resolve(
    mut name: *const string,
    mut port: libc::c_int,
    mut si: *mut sockinfo,
) -> libc::c_int {
    if !name.is_null()
        && *((*name).data).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return nc_resolve_unix(name, si);
    }
    return nc_resolve_inet(name, port, si);
}
#[no_mangle]
pub unsafe extern "C" fn nc_unresolve_addr(
    mut addr: *mut sockaddr,
    mut addrlen: socklen_t,
) -> *const libc::c_char {
    static mut unresolve: [libc::c_char; 1057] = [0; 1057];
    static mut host: [libc::c_char; 1025] = [0; 1025];
    static mut service: [libc::c_char; 32] = [0; 32];
    let mut status: libc::c_int = 0;
    status = getnameinfo(
        addr,
        addrlen,
        host.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong as socklen_t,
        service.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as socklen_t,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if status < 0 as libc::c_int {
        return b"unknown\0" as *const u8 as *const libc::c_char;
    }
    snprintf(
        unresolve.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1057]>() as libc::c_ulong,
        b"%s:%s\0" as *const u8 as *const libc::c_char,
        host.as_mut_ptr(),
        service.as_mut_ptr(),
    );
    return unresolve.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn nc_unresolve_peer_desc(
    mut sd: libc::c_int,
) -> *const libc::c_char {
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
    let mut status: libc::c_int = 0;
    memset(
        &mut si as *mut sockinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockinfo>() as libc::c_ulong,
    );
    addr = &mut si.addr as *mut C2RustUnnamed_1 as *mut sockaddr;
    addrlen = ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as socklen_t;
    status = getpeername(
        sd,
        __SOCKADDR_ARG {
            __sockaddr__: addr,
        },
        &mut addrlen,
    );
    if status < 0 as libc::c_int {
        return b"unknown\0" as *const u8 as *const libc::c_char;
    }
    return nc_unresolve_addr(addr, addrlen);
}
#[no_mangle]
pub unsafe extern "C" fn nc_unresolve_desc(mut sd: libc::c_int) -> *const libc::c_char {
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
    let mut status: libc::c_int = 0;
    memset(
        &mut si as *mut sockinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockinfo>() as libc::c_ulong,
    );
    addr = &mut si.addr as *mut C2RustUnnamed_1 as *mut sockaddr;
    addrlen = ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong as socklen_t;
    status = getsockname(
        sd,
        __SOCKADDR_ARG {
            __sockaddr__: addr,
        },
        &mut addrlen,
    );
    if status < 0 as libc::c_int {
        return b"unknown\0" as *const u8 as *const libc::c_char;
    }
    return nc_unresolve_addr(addr, addrlen);
}
