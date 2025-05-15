use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type pth_st;
    pub type pth_attr_st;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn exit(_: i32) -> !;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut i8;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
    fn bind(__fd: i32, __addr: *const sockaddr, __len: socklen_t) -> i32;
    fn listen(__fd: i32, __n: i32) -> i32;
    fn inet_ntoa(__in: in_addr) -> *mut i8;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn getprotobyname(__name: *const i8) -> *mut protoent;
    fn close(__fd: i32) -> i32;
    fn pth_init() -> i32;
    fn pth_kill() -> i32;
    fn pth_ctrl(_: u64, _: ...) -> i64;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: i32, _: ...) -> i32;
    fn pth_attr_destroy(_: pth_attr_t) -> i32;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_yield(_: pth_t) -> i32;
    fn pth_write(_: i32, _: *const libc::c_void, _: size_t) -> ssize_t;
    fn pth_accept(_: i32, _: *mut sockaddr, _: *mut socklen_t) -> i32;
    fn pth_sleep(_: u32) -> u32;
    fn pth_readline(_: i32, _: *mut libc::c_void, _: size_t) -> ssize_t;
}
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct protoent {
    pub p_name: *mut i8,
    pub p_aliases: *mut *mut i8,
    pub p_proto: i32,
}
pub type pth_t = *mut pth_st;
pub type pth_attr_t = *mut pth_attr_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    PTH_ATTR_BOUND = 14,
    PTH_ATTR_EVENTS = 13,
    PTH_ATTR_STATE = 12,
    PTH_ATTR_START_ARG = 11,
    PTH_ATTR_START_FUNC = 10,
    PTH_ATTR_TIME_RAN = 9,
    PTH_ATTR_TIME_LAST = 8,
    PTH_ATTR_TIME_SPAWN = 7,
    PTH_ATTR_DISPATCHES = 6,
    PTH_ATTR_STACK_ADDR = 5,
    PTH_ATTR_STACK_SIZE = 4,
    PTH_ATTR_CANCEL_STATE = 3,
    PTH_ATTR_JOINABLE = 2,
    PTH_ATTR_NAME = 1,
    PTH_ATTR_PRIO = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::PTH_ATTR_BOUND => 14,
            C2RustUnnamed::PTH_ATTR_EVENTS => 13,
            C2RustUnnamed::PTH_ATTR_STATE => 12,
            C2RustUnnamed::PTH_ATTR_START_ARG => 11,
            C2RustUnnamed::PTH_ATTR_START_FUNC => 10,
            C2RustUnnamed::PTH_ATTR_TIME_RAN => 9,
            C2RustUnnamed::PTH_ATTR_TIME_LAST => 8,
            C2RustUnnamed::PTH_ATTR_TIME_SPAWN => 7,
            C2RustUnnamed::PTH_ATTR_DISPATCHES => 6,
            C2RustUnnamed::PTH_ATTR_STACK_ADDR => 5,
            C2RustUnnamed::PTH_ATTR_STACK_SIZE => 4,
            C2RustUnnamed::PTH_ATTR_CANCEL_STATE => 3,
            C2RustUnnamed::PTH_ATTR_JOINABLE => 2,
            C2RustUnnamed::PTH_ATTR_NAME => 1,
            C2RustUnnamed::PTH_ATTR_PRIO => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            14 => C2RustUnnamed::PTH_ATTR_BOUND,
            13 => C2RustUnnamed::PTH_ATTR_EVENTS,
            12 => C2RustUnnamed::PTH_ATTR_STATE,
            11 => C2RustUnnamed::PTH_ATTR_START_ARG,
            10 => C2RustUnnamed::PTH_ATTR_START_FUNC,
            9 => C2RustUnnamed::PTH_ATTR_TIME_RAN,
            8 => C2RustUnnamed::PTH_ATTR_TIME_LAST,
            7 => C2RustUnnamed::PTH_ATTR_TIME_SPAWN,
            6 => C2RustUnnamed::PTH_ATTR_DISPATCHES,
            5 => C2RustUnnamed::PTH_ATTR_STACK_ADDR,
            4 => C2RustUnnamed::PTH_ATTR_STACK_SIZE,
            3 => C2RustUnnamed::PTH_ATTR_CANCEL_STATE,
            2 => C2RustUnnamed::PTH_ATTR_JOINABLE,
            1 => C2RustUnnamed::PTH_ATTR_NAME,
            0 => C2RustUnnamed::PTH_ATTR_PRIO,
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
unsafe extern "C" fn handler(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut fd: i32 = _arg as i64 as i32;
    let mut caLine: [i8; 1024] = [0; 1024];
    let mut str: [i8; 1024] = [0; 1024];
    let mut n: i32 = 0;
    loop {
        n = pth_readline(
            fd,
            caLine.as_mut_ptr() as *mut libc::c_void,
            1024 as i32 as size_t,
        ) as i32;
        if n < 0 as i32 {
            fprintf(
                stderr,
                b"read error: errno=%d\n\0" as *const u8 as *const i8,
                *__errno_location(),
            );
            close(fd);
            return 0 as *mut libc::c_void;
        }
        if n == 0 as i32 {
            break;
        }
        if n == 1 as i32 && caLine[0 as i32 as usize] as i32 == '\n' as i32 {
            break;
        }
        caLine[(n - 1 as i32) as usize] = '\0' as i32 as i8;
    }
    pth_yield(0 as pth_t);
    sprintf(
        str.as_mut_ptr(),
        b"HTTP/1.0 200 Ok\r\nServer: test_httpd/%x\r\nConnection: close\r\nContent-type: text/plain\r\n\r\nJust a trivial test for GNU Pth\nto show that it's serving data.\r\n\0"
            as *const u8 as *const i8,
        0x200207 as i32,
    );
    pth_write(fd, str.as_mut_ptr() as *const libc::c_void, strlen(str.as_mut_ptr()));
    fprintf(stderr, b"connection shutdown (fd: %d)\n\0" as *const u8 as *const i8, fd);
    close(fd);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn ticker(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut now: time_t = 0;
    let mut ct: *mut i8 = 0 as *mut i8;
    let mut avload: libc::c_float = 0.;
    loop {
        pth_sleep(5 as i32 as u32);
        now = time(0 as *mut time_t);
        ct = ctime(&mut now);
        *ct.offset((strlen(ct)).wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32
            as i8;
        pth_ctrl(((1 as i32) << 1 as i32) as u64, &mut avload as *mut libc::c_float);
        fprintf(
            stderr,
            b"ticker woken up on %s, average load: %.2f\n\0" as *const u8 as *const i8,
            ct,
            avload as libc::c_double,
        );
    };
}
static mut s: i32 = 0;
#[no_mangle]
pub static mut attr: pth_attr_t = 0 as *const pth_attr_st as *mut pth_attr_st;
unsafe extern "C" fn myexit(mut sig: i32) {
    close(s);
    pth_attr_destroy(attr);
    pth_kill();
    fprintf(stderr, b"**Break\n\0" as *const u8 as *const i8);
    exit(0 as i32);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut sar: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut pe: *mut protoent = 0 as *mut protoent;
    let mut peer_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut peer_len: socklen_t = 0;
    let mut sr: i32 = 0;
    let mut port: i32 = 0;
    pth_init();
    signal(
        13 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    signal(2 as i32, Some(myexit as unsafe extern "C" fn(i32) -> ()));
    signal(15 as i32, Some(myexit as unsafe extern "C" fn(i32) -> ()));
    if argc != 2 as i32 {
        fprintf(
            stderr,
            b"Usage: %s <port>\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
        );
        exit(1 as i32);
    }
    port = atoi(*argv.offset(1 as i32 as isize));
    if port <= 0 as i32 || port >= 65535 as i32 {
        fprintf(stderr, b"Illegal port: %d\n\0" as *const u8 as *const i8, port);
        exit(1 as i32);
    }
    fprintf(
        stderr,
        b"This is TEST_HTTPD, a Pth test using socket I/O.\n\0" as *const u8 as *const i8,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"Multiple connections are accepted on the specified port.\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"For each connection a separate thread is spawned which\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"reads a HTTP request the socket and writes back a constant\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"(and useless) HTTP response to the socket.\n\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr,
        b"Additionally a useless ticker thread awakens every 5s.\n\0" as *const u8
            as *const i8,
    );
    fprintf(
        stderr,
        b"Watch the average scheduler load the ticker displays.\n\0" as *const u8
            as *const i8,
    );
    fprintf(stderr, b"Hit CTRL-C for stopping this test.\n\0" as *const u8 as *const i8);
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    attr = pth_attr_new();
    pth_attr_set(
        attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"ticker\0" as *const u8 as *const i8,
    );
    pth_attr_set(attr, C2RustUnnamed::PTH_ATTR_JOINABLE as i32, 0 as i32);
    pth_attr_set(
        attr,
        C2RustUnnamed::PTH_ATTR_STACK_SIZE as i32,
        64 as i32 * 1024 as i32,
    );
    pth_spawn(
        attr,
        Some(ticker as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pe = getprotobyname(b"tcp\0" as *const u8 as *const i8);
    if pe.is_null() {
        perror(b"getprotobyname\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    s = socket(2 as i32, __socket_type::SOCK_STREAM as i32, (*pe).p_proto);
    if s == -(1 as i32) {
        perror(b"socket\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    sar.sin_family = 2 as i32 as sa_family_t;
    sar.sin_addr.s_addr = 0 as i32 as in_addr_t;
    sar.sin_port = ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = port as libc::c_ushort;
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
    if bind(
        s,
        &mut sar as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as u64 as socklen_t,
    ) == -(1 as i32)
    {
        perror(b"socket\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if listen(s, 1024 as i32 - 100 as i32) == -(1 as i32) {
        perror(b"listen\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    pth_attr_set(
        attr,
        C2RustUnnamed::PTH_ATTR_NAME as i32,
        b"handler\0" as *const u8 as *const i8,
    );
    fprintf(
        stderr,
        b"listening on port %d (max %d simultaneous connections)\n\0" as *const u8
            as *const i8,
        port,
        1024 as i32 - 100 as i32,
    );
    loop {
        peer_len = ::core::mem::size_of::<sockaddr_in>() as u64 as socklen_t;
        sr = pth_accept(
            s,
            &mut peer_addr as *mut sockaddr_in as *mut sockaddr,
            &mut peer_len,
        );
        if sr == -(1 as i32) {
            perror(b"accept\0" as *const u8 as *const i8);
            pth_sleep(1 as i32 as u32);
        } else if pth_ctrl(
            ((1 as i32) << 4 as i32 | (1 as i32) << 5 as i32 | (1 as i32) << 6 as i32
                | (1 as i32) << 7 as i32 | (1 as i32) << 8 as i32
                | (1 as i32) << 9 as i32) as u64,
        ) >= (1024 as i32 - 100 as i32) as i64
        {
            fprintf(
                stderr,
                b"currently no more connections acceptable\n\0" as *const u8 as *const i8,
            );
        } else {
            fprintf(
                stderr,
                b"connection established (fd: %d, ip: %s, port: %d)\n\0" as *const u8
                    as *const i8,
                sr,
                inet_ntoa(peer_addr.sin_addr),
                ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = peer_addr.sin_port;
                    if 0 != 0 {
                        __v = (__x as i32 >> 8 as i32 & 0xff as i32
                            | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
                    } else {
                        let fresh3 = &mut __v;
                        let fresh4;
                        let fresh5 = __x;
                        asm!(
                            "rorw $8, {0:x}", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                            options(pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                    }
                    __v
                }) as i32,
            );
            pth_spawn(
                attr,
                Some(
                    handler
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                sr as i64 as *mut libc::c_void,
            );
        }
    };
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