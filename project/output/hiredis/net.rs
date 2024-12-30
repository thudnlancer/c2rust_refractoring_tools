#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type redisAsyncContext;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: size_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    static mut hiredisAllocFns: hiredisAllocFuncs;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn __redisSetError(
        c: *mut redisContext,
        type_0: libc::c_int,
        str: *const libc::c_char,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReadTask {
    pub type_0: libc::c_int,
    pub elements: libc::c_longlong,
    pub idx: libc::c_int,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createArray: Option::<
        unsafe extern "C" fn(*const redisReadTask, size_t) -> *mut libc::c_void,
    >,
    pub createInteger: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_longlong) -> *mut libc::c_void,
    >,
    pub createDouble: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            libc::c_double,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createNil: Option::<
        unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
    >,
    pub createBool: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_int) -> *mut libc::c_void,
    >,
    pub freeObject: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReader {
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub buf: *mut libc::c_char,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,
    pub maxelements: libc::c_longlong,
    pub task: *mut *mut redisReadTask,
    pub tasks: libc::c_int,
    pub ridx: libc::c_int,
    pub reply: *mut libc::c_void,
    pub fn_0: *mut redisReplyObjectFunctions,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hiredisAllocFuncs {
    pub mallocFn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub callocFn: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub reallocFn: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub strdupFn: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
    >,
    pub freeFn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContext {
    pub funcs: *const redisContextFuncs,
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub fd: redisFD,
    pub flags: libc::c_int,
    pub obuf: *mut libc::c_char,
    pub reader: *mut redisReader,
    pub connection_type: redisConnectionType,
    pub connect_timeout: *mut timeval,
    pub command_timeout: *mut timeval,
    pub tcp: C2RustUnnamed_0,
    pub unix_sock: C2RustUnnamed,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub privctx: *mut libc::c_void,
    pub push_cb: Option::<redisPushFn>,
}
pub type redisPushFn = unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub host: *mut libc::c_char,
    pub source_addr: *mut libc::c_char,
    pub port: libc::c_int,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redisConnectionType {
    REDIS_CONN_TCP,
    REDIS_CONN_UNIX,
    REDIS_CONN_USERFD,
}  // end of enum

pub type redisFD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContextFuncs {
    pub close: Option::<unsafe extern "C" fn(*mut redisContext) -> ()>,
    pub free_privctx: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub async_read: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub async_write: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub read: Option::<
        unsafe extern "C" fn(*mut redisContext, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<unsafe extern "C" fn(*mut redisContext) -> ssize_t>,
}
pub type socklen_t = __socklen_t;
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
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub const SOCK_STREAM: __socket_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_STREAM,
    SOCK_NONBLOCK,
    SOCK_CLOEXEC,
    SOCK_PACKET,
    SOCK_DCCP,
    SOCK_SEQPACKET,
    SOCK_RDM,
    SOCK_RAW,
    SOCK_DGRAM,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    IPPROTO_TCP,
    IPPROTO_MAX,
    IPPROTO_RAW,
    IPPROTO_MPLS,
    IPPROTO_UDPLITE,
    IPPROTO_SCTP,
    IPPROTO_COMP,
    IPPROTO_PIM,
    IPPROTO_ENCAP,
    IPPROTO_BEETPH,
    IPPROTO_MTP,
    IPPROTO_AH,
    IPPROTO_ESP,
    IPPROTO_GRE,
    IPPROTO_RSVP,
    IPPROTO_IPV6,
    IPPROTO_DCCP,
    IPPROTO_TP,
    IPPROTO_IDP,
    IPPROTO_UDP,
    IPPROTO_PUP,
    IPPROTO_EGP,
    IPPROTO_IPIP,
    IPPROTO_IGMP,
    IPPROTO_ICMP,
    IPPROTO_IP,
}  // end of enum

#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(::core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s
                .offset(-(::core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s
                .offset(-(::core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s
                .offset(-(::core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn hi_malloc(mut size: size_t) -> *mut libc::c_void {
    return (hiredisAllocFns.mallocFn).expect("non-null function pointer")(size);
}
#[inline]
unsafe extern "C" fn hi_strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    return (hiredisAllocFns.strdupFn).expect("non-null function pointer")(str);
}
#[inline]
unsafe extern "C" fn hi_free(mut ptr: *mut libc::c_void) {
    (hiredisAllocFns.freeFn).expect("non-null function pointer")(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn redisNetClose(mut c: *mut redisContext) {
    if !c.is_null() && (*c).fd != -(1 as libc::c_int) {
        close((*c).fd);
        (*c).fd = -(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn redisNetRead(
    mut c: *mut redisContext,
    mut buf: *mut libc::c_char,
    mut bufcap: size_t,
) -> ssize_t {
    let mut nread: ssize_t = recv(
        (*c).fd,
        buf as *mut libc::c_void,
        bufcap,
        0 as libc::c_int,
    );
    if nread == -(1 as libc::c_int) as libc::c_long {
        if *__errno_location() == 11 as libc::c_int
            && (*c).flags & 0x1 as libc::c_int == 0
            || *__errno_location() == 4 as libc::c_int
        {
            return 0 as libc::c_int as ssize_t
        } else if *__errno_location() == 110 as libc::c_int
            && (*c).flags & 0x1 as libc::c_int != 0
        {
            __redisSetError(
                c,
                6 as libc::c_int,
                b"recv timeout\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int) as ssize_t;
        } else {
            __redisSetError(c, 1 as libc::c_int, strerror(*__errno_location()));
            return -(1 as libc::c_int) as ssize_t;
        }
    } else if nread == 0 as libc::c_int as libc::c_long {
        __redisSetError(
            c,
            3 as libc::c_int,
            b"Server closed the connection\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int) as ssize_t;
    } else {
        return nread
    };
}
#[no_mangle]
pub unsafe extern "C" fn redisNetWrite(mut c: *mut redisContext) -> ssize_t {
    let mut nwritten: ssize_t = 0;
    nwritten = send(
        (*c).fd,
        (*c).obuf as *const libc::c_void,
        sdslen((*c).obuf),
        0 as libc::c_int,
    );
    if nwritten < 0 as libc::c_int as libc::c_long {
        if *__errno_location() == 11 as libc::c_int
            && (*c).flags & 0x1 as libc::c_int == 0
            || *__errno_location() == 4 as libc::c_int
        {
            return 0 as libc::c_int as ssize_t
        } else {
            __redisSetError(c, 1 as libc::c_int, strerror(*__errno_location()));
            return -(1 as libc::c_int) as ssize_t;
        }
    }
    return nwritten;
}
unsafe extern "C" fn __redisSetErrorFromErrno(
    mut c: *mut redisContext,
    mut type_0: libc::c_int,
    mut prefix: *const libc::c_char,
) {
    let mut errorno: libc::c_int = *__errno_location();
    let mut buf: [libc::c_char; 128] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut len: size_t = 0 as libc::c_int as size_t;
    if !prefix.is_null() {
        len = snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%s: \0" as *const u8 as *const libc::c_char,
            prefix,
        ) as size_t;
    }
    strerror_r(
        errorno,
        buf.as_mut_ptr().offset(len as isize),
        (::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(len),
    );
    __redisSetError(c, type_0, buf.as_mut_ptr());
}
unsafe extern "C" fn redisSetReuseAddr(mut c: *mut redisContext) -> libc::c_int {
    let mut on: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        (*c).fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut on as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        __redisSetErrorFromErrno(c, 1 as libc::c_int, 0 as *const libc::c_char);
        redisNetClose(c);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisCreateSocket(
    mut c: *mut redisContext,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut s: redisFD = 0;
    s = socket(type_0, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if s == -(1 as libc::c_int) {
        __redisSetErrorFromErrno(c, 1 as libc::c_int, 0 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    (*c).fd = s;
    if type_0 == 2 as libc::c_int {
        if redisSetReuseAddr(c) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisSetBlocking(
    mut c: *mut redisContext,
    mut blocking: libc::c_int,
) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl((*c).fd, 3 as libc::c_int);
    if flags == -(1 as libc::c_int) {
        __redisSetErrorFromErrno(
            c,
            1 as libc::c_int,
            b"fcntl(F_GETFL)\0" as *const u8 as *const libc::c_char,
        );
        redisNetClose(c);
        return -(1 as libc::c_int);
    }
    if blocking != 0 {
        flags &= !(0o4000 as libc::c_int);
    } else {
        flags |= 0o4000 as libc::c_int;
    }
    if fcntl((*c).fd, 4 as libc::c_int, flags) == -(1 as libc::c_int) {
        __redisSetErrorFromErrno(
            c,
            1 as libc::c_int,
            b"fcntl(F_SETFL)\0" as *const u8 as *const libc::c_char,
        );
        redisNetClose(c);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisKeepAlive(
    mut c: *mut redisContext,
    mut interval: libc::c_int,
) -> libc::c_int {
    let mut val: libc::c_int = 1 as libc::c_int;
    let mut fd: redisFD = (*c).fd;
    if (*c).connection_type as libc::c_uint
        == REDIS_CONN_UNIX as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if setsockopt(
        fd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        __redisSetError(c, 2 as libc::c_int, strerror(*__errno_location()));
        return -(1 as libc::c_int);
    }
    val = interval;
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        4 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        __redisSetError(c, 2 as libc::c_int, strerror(*__errno_location()));
        return -(1 as libc::c_int);
    }
    val = interval / 3 as libc::c_int;
    if val == 0 as libc::c_int {
        val = 1 as libc::c_int;
    }
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        5 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        __redisSetError(c, 2 as libc::c_int, strerror(*__errno_location()));
        return -(1 as libc::c_int);
    }
    val = 3 as libc::c_int;
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        6 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        __redisSetError(c, 2 as libc::c_int, strerror(*__errno_location()));
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisSetTcpNoDelay(mut c: *mut redisContext) -> libc::c_int {
    let mut yes: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        (*c).fd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        __redisSetErrorFromErrno(
            c,
            1 as libc::c_int,
            b"setsockopt(TCP_NODELAY)\0" as *const u8 as *const libc::c_char,
        );
        redisNetClose(c);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisContextSetTcpUserTimeout(
    mut c: *mut redisContext,
    mut timeout: libc::c_uint,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = setsockopt(
        (*c).fd,
        IPPROTO_TCP as libc::c_int,
        18 as libc::c_int,
        &mut timeout as *mut libc::c_uint as *const libc::c_void,
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as socklen_t,
    );
    if res == -(1 as libc::c_int) {
        __redisSetErrorFromErrno(
            c,
            1 as libc::c_int,
            b"setsockopt(TCP_USER_TIMEOUT)\0" as *const u8 as *const libc::c_char,
        );
        redisNetClose(c);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisContextTimeoutMsec(
    mut c: *mut redisContext,
    mut result: *mut libc::c_long,
) -> libc::c_int {
    let mut timeout: *const timeval = (*c).connect_timeout;
    let mut msec: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if !timeout.is_null() {
        if (*timeout).tv_usec > 1000000 as libc::c_int as libc::c_long
            || (*timeout).tv_sec
                > (9223372036854775807 as libc::c_long
                    - 999 as libc::c_int as libc::c_long)
                    / 1000 as libc::c_int as libc::c_long
        {
            __redisSetError(
                c,
                1 as libc::c_int,
                b"Invalid timeout specified\0" as *const u8 as *const libc::c_char,
            );
            *result = msec;
            return -(1 as libc::c_int);
        }
        msec = (*timeout).tv_sec * 1000 as libc::c_int as libc::c_long
            + ((*timeout).tv_usec + 999 as libc::c_int as libc::c_long)
                / 1000 as libc::c_int as libc::c_long;
        if msec < 0 as libc::c_int as libc::c_long
            || msec > 2147483647 as libc::c_int as libc::c_long
        {
            msec = 2147483647 as libc::c_int as libc::c_long;
        }
    }
    *result = msec;
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisPollMillis() -> libc::c_long {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut now);
    return now.tv_sec * 1000 as libc::c_int as libc::c_long
        + now.tv_nsec / 1000000 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn redisContextWaitReady(
    mut c: *mut redisContext,
    mut msec: libc::c_long,
) -> libc::c_int {
    let mut wfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut end: libc::c_long = 0;
    let mut res: libc::c_int = 0;
    if *__errno_location() != 115 as libc::c_int {
        __redisSetErrorFromErrno(c, 1 as libc::c_int, 0 as *const libc::c_char);
        redisNetClose(c);
        return -(1 as libc::c_int);
    }
    wfd.fd = (*c).fd;
    wfd.events = 0x4 as libc::c_int as libc::c_short;
    end = if msec >= 0 as libc::c_int as libc::c_long {
        redisPollMillis() + msec
    } else {
        0 as libc::c_int as libc::c_long
    };
    loop {
        res = poll(&mut wfd, 1 as libc::c_int as nfds_t, msec as libc::c_int);
        if !(res <= 0 as libc::c_int) {
            break;
        }
        if res < 0 as libc::c_int && *__errno_location() != 4 as libc::c_int {
            __redisSetErrorFromErrno(
                c,
                1 as libc::c_int,
                b"poll(2)\0" as *const u8 as *const libc::c_char,
            );
            redisNetClose(c);
            return -(1 as libc::c_int);
        } else if res == 0 as libc::c_int
            || msec >= 0 as libc::c_int as libc::c_long && redisPollMillis() >= end
        {
            *__errno_location() = 110 as libc::c_int;
            __redisSetErrorFromErrno(c, 1 as libc::c_int, 0 as *const libc::c_char);
            redisNetClose(c);
            return -(1 as libc::c_int);
        }
    }
    if redisCheckConnectDone(c, &mut res) != 0 as libc::c_int || res == 0 as libc::c_int
    {
        redisCheckSocketError(c);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisCheckConnectDone(
    mut c: *mut redisContext,
    mut completed: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = connect(
        (*c).fd,
        (*c).saddr as *const sockaddr,
        (*c).addrlen as socklen_t,
    );
    if rc == 0 as libc::c_int {
        *completed = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut error: libc::c_int = *__errno_location();
    if error == 115 as libc::c_int {
        let mut fail: libc::c_int = 0;
        let mut so_error: libc::c_int = 0;
        let mut optlen: socklen_t = ::core::mem::size_of::<libc::c_int>()
            as libc::c_ulong as socklen_t;
        fail = getsockopt(
            (*c).fd,
            1 as libc::c_int,
            4 as libc::c_int,
            &mut so_error as *mut libc::c_int as *mut libc::c_void,
            &mut optlen,
        );
        if fail == 0 as libc::c_int {
            if so_error == 0 as libc::c_int {
                *completed = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            *__errno_location() = so_error;
            error = so_error;
        }
    }
    match error {
        106 => {
            *completed = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        114 | 11 => {
            *completed = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
        _ => return -(1 as libc::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn redisCheckSocketError(mut c: *mut redisContext) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut errno_saved: libc::c_int = *__errno_location();
    let mut errlen: socklen_t = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        as socklen_t;
    if getsockopt(
        (*c).fd,
        1 as libc::c_int,
        4 as libc::c_int,
        &mut err as *mut libc::c_int as *mut libc::c_void,
        &mut errlen,
    ) == -(1 as libc::c_int)
    {
        __redisSetErrorFromErrno(
            c,
            1 as libc::c_int,
            b"getsockopt(SO_ERROR)\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if err == 0 as libc::c_int {
        err = errno_saved;
    }
    if err != 0 {
        *__errno_location() = err;
        __redisSetErrorFromErrno(c, 1 as libc::c_int, 0 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisContextSetTimeout(
    mut c: *mut redisContext,
    tv: timeval,
) -> libc::c_int {
    let mut to_ptr: *const libc::c_void = &tv as *const timeval as *const libc::c_void;
    let mut to_sz: size_t = ::core::mem::size_of::<timeval>() as libc::c_ulong;
    if redisContextUpdateCommandTimeout(c, &tv) != 0 as libc::c_int {
        __redisSetError(
            c,
            5 as libc::c_int,
            b"Out of memory\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if setsockopt(
        (*c).fd,
        1 as libc::c_int,
        20 as libc::c_int,
        to_ptr,
        to_sz as socklen_t,
    ) == -(1 as libc::c_int)
    {
        __redisSetErrorFromErrno(
            c,
            1 as libc::c_int,
            b"setsockopt(SO_RCVTIMEO)\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if setsockopt(
        (*c).fd,
        1 as libc::c_int,
        21 as libc::c_int,
        to_ptr,
        to_sz as socklen_t,
    ) == -(1 as libc::c_int)
    {
        __redisSetErrorFromErrno(
            c,
            1 as libc::c_int,
            b"setsockopt(SO_SNDTIMEO)\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisContextUpdateConnectTimeout(
    mut c: *mut redisContext,
    mut timeout: *const timeval,
) -> libc::c_int {
    if (*c).connect_timeout == timeout as *mut timeval {
        return 0 as libc::c_int;
    }
    if ((*c).connect_timeout).is_null() {
        (*c)
            .connect_timeout = hi_malloc(
            ::core::mem::size_of::<timeval>() as libc::c_ulong,
        ) as *mut timeval;
        if ((*c).connect_timeout).is_null() {
            return -(1 as libc::c_int);
        }
    }
    memcpy(
        (*c).connect_timeout as *mut libc::c_void,
        timeout as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisContextUpdateCommandTimeout(
    mut c: *mut redisContext,
    mut timeout: *const timeval,
) -> libc::c_int {
    if (*c).command_timeout == timeout as *mut timeval {
        return 0 as libc::c_int;
    }
    if ((*c).command_timeout).is_null() {
        (*c)
            .command_timeout = hi_malloc(
            ::core::mem::size_of::<timeval>() as libc::c_ulong,
        ) as *mut timeval;
        if ((*c).command_timeout).is_null() {
            return -(1 as libc::c_int);
        }
    }
    memcpy(
        (*c).command_timeout as *mut libc::c_void,
        timeout as *const libc::c_void,
        ::core::mem::size_of::<timeval>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn _redisContextConnectTcp(
    mut c: *mut redisContext,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
    mut timeout: *const timeval,
    mut source_addr: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut s: redisFD = 0;
    let mut rv: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut _port: [libc::c_char; 6] = [0; 6];
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
    let mut servinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut bservinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut b: *mut addrinfo = 0 as *mut addrinfo;
    let mut blocking: libc::c_int = (*c).flags & 0x1 as libc::c_int;
    let mut reuseaddr: libc::c_int = (*c).flags & 0x80 as libc::c_int;
    let mut reuses: libc::c_int = 0 as libc::c_int;
    let mut timeout_msec: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    servinfo = 0 as *mut addrinfo;
    (*c).connection_type = REDIS_CONN_TCP;
    (*c).tcp.port = port;
    if (*c).tcp.host != addr as *mut libc::c_char {
        hi_free((*c).tcp.host as *mut libc::c_void);
        (*c).tcp.host = hi_strdup(addr);
        if ((*c).tcp.host).is_null() {
            current_block = 17447817038493667125;
        } else {
            current_block = 17216689946888361452;
        }
    } else {
        current_block = 17216689946888361452;
    }
    match current_block {
        17216689946888361452 => {
            if !timeout.is_null() {
                if redisContextUpdateConnectTimeout(c, timeout) == -(1 as libc::c_int) {
                    current_block = 17447817038493667125;
                } else {
                    current_block = 11050875288958768710;
                }
            } else {
                hi_free((*c).connect_timeout as *mut libc::c_void);
                (*c).connect_timeout = 0 as *mut timeval;
                current_block = 11050875288958768710;
            }
            match current_block {
                17447817038493667125 => {}
                _ => {
                    if redisContextTimeoutMsec(c, &mut timeout_msec) != 0 as libc::c_int
                    {
                        current_block = 4576699668870366210;
                    } else {
                        if source_addr.is_null() {
                            hi_free((*c).tcp.source_addr as *mut libc::c_void);
                            (*c).tcp.source_addr = 0 as *mut libc::c_char;
                        } else if (*c).tcp.source_addr
                            != source_addr as *mut libc::c_char
                        {
                            hi_free((*c).tcp.source_addr as *mut libc::c_void);
                            (*c).tcp.source_addr = hi_strdup(source_addr);
                        }
                        snprintf(
                            _port.as_mut_ptr(),
                            6 as libc::c_int as libc::c_ulong,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            port,
                        );
                        memset(
                            &mut hints as *mut addrinfo as *mut libc::c_void,
                            0 as libc::c_int,
                            ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
                        );
                        hints.ai_family = 2 as libc::c_int;
                        hints.ai_socktype = SOCK_STREAM as libc::c_int;
                        if (*c).flags & 0x1000 as libc::c_int != 0
                            && (*c).flags & 0x800 as libc::c_int != 0
                        {
                            hints.ai_family = 0 as libc::c_int;
                        } else if (*c).flags & 0x1000 as libc::c_int != 0 {
                            hints.ai_family = 10 as libc::c_int;
                        } else {
                            hints.ai_family = 2 as libc::c_int;
                        }
                        rv = getaddrinfo(
                            (*c).tcp.host,
                            _port.as_mut_ptr(),
                            &mut hints,
                            &mut servinfo,
                        );
                        if rv != 0 as libc::c_int && hints.ai_family != 0 as libc::c_int
                        {
                            hints
                                .ai_family = if hints.ai_family == 2 as libc::c_int {
                                10 as libc::c_int
                            } else {
                                2 as libc::c_int
                            };
                            rv = getaddrinfo(
                                (*c).tcp.host,
                                _port.as_mut_ptr(),
                                &mut hints,
                                &mut servinfo,
                            );
                        }
                        if rv != 0 as libc::c_int {
                            __redisSetError(c, 2 as libc::c_int, gai_strerror(rv));
                            return -(1 as libc::c_int);
                        }
                        p = servinfo;
                        's_170: loop {
                            if p.is_null() {
                                current_block = 919954187481050311;
                                break;
                            }
                            loop {
                                s = socket(
                                    (*p).ai_family,
                                    (*p).ai_socktype,
                                    (*p).ai_protocol,
                                );
                                if s == -(1 as libc::c_int) {
                                    current_block = 18153031941552419006;
                                    break;
                                }
                                (*c).fd = s;
                                if redisSetBlocking(c, 0 as libc::c_int) != 0 as libc::c_int
                                {
                                    current_block = 4576699668870366210;
                                    break 's_170;
                                }
                                if !((*c).tcp.source_addr).is_null() {
                                    let mut bound: libc::c_int = 0 as libc::c_int;
                                    rv = getaddrinfo(
                                        (*c).tcp.source_addr,
                                        0 as *const libc::c_char,
                                        &mut hints,
                                        &mut bservinfo,
                                    );
                                    if rv != 0 as libc::c_int {
                                        let mut buf: [libc::c_char; 128] = [0; 128];
                                        snprintf(
                                            buf.as_mut_ptr(),
                                            ::core::mem::size_of::<[libc::c_char; 128]>()
                                                as libc::c_ulong,
                                            b"Can't get addr: %s\0" as *const u8 as *const libc::c_char,
                                            gai_strerror(rv),
                                        );
                                        __redisSetError(c, 2 as libc::c_int, buf.as_mut_ptr());
                                        current_block = 4576699668870366210;
                                        break 's_170;
                                    } else {
                                        if reuseaddr != 0 {
                                            n = 1 as libc::c_int;
                                            if setsockopt(
                                                s,
                                                1 as libc::c_int,
                                                2 as libc::c_int,
                                                &mut n as *mut libc::c_int as *mut libc::c_char
                                                    as *const libc::c_void,
                                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                                    as socklen_t,
                                            ) < 0 as libc::c_int
                                            {
                                                freeaddrinfo(bservinfo);
                                                current_block = 4576699668870366210;
                                                break 's_170;
                                            }
                                        }
                                        b = bservinfo;
                                        while !b.is_null() {
                                            if bind(s, (*b).ai_addr, (*b).ai_addrlen)
                                                != -(1 as libc::c_int)
                                            {
                                                bound = 1 as libc::c_int;
                                                break;
                                            } else {
                                                b = (*b).ai_next;
                                            }
                                        }
                                        freeaddrinfo(bservinfo);
                                        if bound == 0 {
                                            let mut buf_0: [libc::c_char; 128] = [0; 128];
                                            snprintf(
                                                buf_0.as_mut_ptr(),
                                                ::core::mem::size_of::<[libc::c_char; 128]>()
                                                    as libc::c_ulong,
                                                b"Can't bind socket: %s\0" as *const u8
                                                    as *const libc::c_char,
                                                strerror(*__errno_location()),
                                            );
                                            __redisSetError(c, 2 as libc::c_int, buf_0.as_mut_ptr());
                                            current_block = 4576699668870366210;
                                            break 's_170;
                                        }
                                    }
                                }
                                hi_free((*c).saddr as *mut libc::c_void);
                                (*c)
                                    .saddr = hi_malloc((*p).ai_addrlen as size_t)
                                    as *mut sockaddr;
                                if ((*c).saddr).is_null() {
                                    current_block = 17447817038493667125;
                                    break 's_170;
                                }
                                memcpy(
                                    (*c).saddr as *mut libc::c_void,
                                    (*p).ai_addr as *const libc::c_void,
                                    (*p).ai_addrlen as libc::c_ulong,
                                );
                                (*c).addrlen = (*p).ai_addrlen as size_t;
                                if !(connect(s, (*p).ai_addr, (*p).ai_addrlen)
                                    == -(1 as libc::c_int))
                                {
                                    current_block = 16029476503615101993;
                                    break;
                                }
                                if *__errno_location() == 113 as libc::c_int {
                                    redisNetClose(c);
                                    current_block = 18153031941552419006;
                                    break;
                                } else if *__errno_location() == 115 as libc::c_int {
                                    if blocking != 0 {
                                        current_block = 18413039352935389787;
                                        break;
                                    } else {
                                        current_block = 16029476503615101993;
                                        break;
                                    }
                                } else {
                                    if !(*__errno_location() == 99 as libc::c_int
                                        && reuseaddr != 0)
                                    {
                                        current_block = 18413039352935389787;
                                        break;
                                    }
                                    reuses += 1;
                                    if reuses >= 10 as libc::c_int {
                                        current_block = 4576699668870366210;
                                        break 's_170;
                                    }
                                    redisNetClose(c);
                                }
                            }
                            match current_block {
                                18413039352935389787 => {
                                    if redisContextWaitReady(c, timeout_msec)
                                        != 0 as libc::c_int
                                    {
                                        current_block = 4576699668870366210;
                                        break;
                                    }
                                    if redisSetTcpNoDelay(c) != 0 as libc::c_int {
                                        current_block = 4576699668870366210;
                                        break;
                                    }
                                }
                                18153031941552419006 => {
                                    p = (*p).ai_next;
                                    continue;
                                }
                                _ => {}
                            }
                            if blocking != 0
                                && redisSetBlocking(c, 1 as libc::c_int) != 0 as libc::c_int
                            {
                                current_block = 4576699668870366210;
                                break;
                            }
                            (*c).flags |= 0x2 as libc::c_int;
                            rv = 0 as libc::c_int;
                            current_block = 13320834858111541275;
                            break;
                        }
                        match current_block {
                            4576699668870366210 => {}
                            17447817038493667125 => {}
                            13320834858111541275 => {}
                            _ => {
                                if p.is_null() {
                                    let mut buf_1: [libc::c_char; 128] = [0; 128];
                                    snprintf(
                                        buf_1.as_mut_ptr(),
                                        ::core::mem::size_of::<[libc::c_char; 128]>()
                                            as libc::c_ulong,
                                        b"Can't create socket: %s\0" as *const u8
                                            as *const libc::c_char,
                                        strerror(*__errno_location()),
                                    );
                                    __redisSetError(c, 2 as libc::c_int, buf_1.as_mut_ptr());
                                    current_block = 4576699668870366210;
                                } else {
                                    current_block = 17447817038493667125;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        17447817038493667125 => {
            __redisSetError(
                c,
                5 as libc::c_int,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
            current_block = 4576699668870366210;
        }
        _ => {}
    }
    match current_block {
        4576699668870366210 => {
            rv = -(1 as libc::c_int);
        }
        _ => {}
    }
    if !servinfo.is_null() {
        freeaddrinfo(servinfo);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn redisContextConnectTcp(
    mut c: *mut redisContext,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
    mut timeout: *const timeval,
) -> libc::c_int {
    return _redisContextConnectTcp(c, addr, port, timeout, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn redisContextConnectBindTcp(
    mut c: *mut redisContext,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
    mut timeout: *const timeval,
    mut source_addr: *const libc::c_char,
) -> libc::c_int {
    return _redisContextConnectTcp(c, addr, port, timeout, source_addr);
}
#[no_mangle]
pub unsafe extern "C" fn redisContextConnectUnix(
    mut c: *mut redisContext,
    mut path: *const libc::c_char,
    mut timeout: *const timeval,
) -> libc::c_int {
    let mut current_block: u64;
    let mut blocking: libc::c_int = (*c).flags & 0x1 as libc::c_int;
    let mut sa: *mut sockaddr_un = 0 as *mut sockaddr_un;
    let mut timeout_msec: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if redisCreateSocket(c, 1 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if redisSetBlocking(c, 0 as libc::c_int) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*c).connection_type = REDIS_CONN_UNIX;
    if (*c).unix_sock.path != path as *mut libc::c_char {
        hi_free((*c).unix_sock.path as *mut libc::c_void);
        (*c).unix_sock.path = hi_strdup(path);
        if ((*c).unix_sock.path).is_null() {
            current_block = 6069098414066754246;
        } else {
            current_block = 10886091980245723256;
        }
    } else {
        current_block = 10886091980245723256;
    }
    match current_block {
        10886091980245723256 => {
            if !timeout.is_null() {
                if redisContextUpdateConnectTimeout(c, timeout) == -(1 as libc::c_int) {
                    current_block = 6069098414066754246;
                } else {
                    current_block = 1856101646708284338;
                }
            } else {
                hi_free((*c).connect_timeout as *mut libc::c_void);
                (*c).connect_timeout = 0 as *mut timeval;
                current_block = 1856101646708284338;
            }
            match current_block {
                6069098414066754246 => {}
                _ => {
                    if redisContextTimeoutMsec(c, &mut timeout_msec) != 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    if !((*c).saddr).is_null() {
                        hi_free((*c).saddr as *mut libc::c_void);
                    }
                    (*c)
                        .saddr = hi_malloc(
                        ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong,
                    ) as *mut sockaddr;
                    sa = (*c).saddr as *mut sockaddr_un;
                    if !sa.is_null() {
                        (*c)
                            .addrlen = ::core::mem::size_of::<sockaddr_un>()
                            as libc::c_ulong;
                        (*sa).sun_family = 1 as libc::c_int as sa_family_t;
                        strncpy(
                            ((*sa).sun_path).as_mut_ptr(),
                            path,
                            (::core::mem::size_of::<[libc::c_char; 108]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                        if connect(
                            (*c).fd,
                            sa as *mut sockaddr,
                            ::core::mem::size_of::<sockaddr_un>() as libc::c_ulong
                                as socklen_t,
                        ) == -(1 as libc::c_int)
                        {
                            if !((*__errno_location() == 11 as libc::c_int
                                || *__errno_location() == 115 as libc::c_int)
                                && blocking == 0)
                            {
                                if redisContextWaitReady(c, timeout_msec)
                                    != 0 as libc::c_int
                                {
                                    return -(1 as libc::c_int);
                                }
                            }
                        }
                        if blocking != 0
                            && redisSetBlocking(c, 1 as libc::c_int) != 0 as libc::c_int
                        {
                            return -(1 as libc::c_int);
                        }
                        (*c).flags |= 0x2 as libc::c_int;
                        return 0 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    __redisSetError(
        c,
        5 as libc::c_int,
        b"Out of memory\0" as *const u8 as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
