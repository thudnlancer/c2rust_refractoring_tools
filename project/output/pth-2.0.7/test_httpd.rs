#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(asm, extern_types)]
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type pth_st;
    pub type pth_attr_st;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn getprotobyname(__name: *const libc::c_char) -> *mut protoent;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pth_init() -> libc::c_int;
    fn pth_kill() -> libc::c_int;
    fn pth_ctrl(_: libc::c_ulong, _: ...) -> libc::c_long;
    fn pth_attr_new() -> pth_attr_t;
    fn pth_attr_set(_: pth_attr_t, _: libc::c_int, _: ...) -> libc::c_int;
    fn pth_attr_destroy(_: pth_attr_t) -> libc::c_int;
    fn pth_spawn(
        _: pth_attr_t,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> pth_t;
    fn pth_yield(_: pth_t) -> libc::c_int;
    fn pth_write(_: libc::c_int, _: *const libc::c_void, _: size_t) -> ssize_t;
    fn pth_accept(_: libc::c_int, _: *mut sockaddr, _: *mut socklen_t) -> libc::c_int;
    fn pth_sleep(_: libc::c_uint) -> libc::c_uint;
    fn pth_readline(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
}  // end of enum

pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
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
    pub sin_zero: [libc::c_uchar; 8],
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct protoent {
    pub p_name: *mut libc::c_char,
    pub p_aliases: *mut *mut libc::c_char,
    pub p_proto: libc::c_int,
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
}  // end of enum

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn handler(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut fd: libc::c_int = _arg as libc::c_long as libc::c_int;
    let mut caLine: [libc::c_char; 1024] = [0; 1024];
    let mut str: [libc::c_char; 1024] = [0; 1024];
    let mut n: libc::c_int = 0;
    loop {
        n = pth_readline(
            fd,
            caLine.as_mut_ptr() as *mut libc::c_void,
            1024 as libc::c_int as size_t,
        ) as libc::c_int;
        if n < 0 as libc::c_int {
            fprintf(
                stderr,
                b"read error: errno=%d\n\0" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            close(fd);
            return 0 as *mut libc::c_void;
        }
        if n == 0 as libc::c_int {
            break;
        }
        if n == 1 as libc::c_int
            && caLine[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
        {
            break;
        }
        caLine[(n - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    }
    pth_yield(0 as pth_t);
    sprintf(
        str.as_mut_ptr(),
        b"HTTP/1.0 200 Ok\r\nServer: test_httpd/%x\r\nConnection: close\r\nContent-type: text/plain\r\n\r\nJust a trivial test for GNU Pth\nto show that it's serving data.\r\n\0"
            as *const u8 as *const libc::c_char,
        0x200207 as libc::c_int,
    );
    pth_write(fd, str.as_mut_ptr() as *const libc::c_void, strlen(str.as_mut_ptr()));
    fprintf(
        stderr,
        b"connection shutdown (fd: %d)\n\0" as *const u8 as *const libc::c_char,
        fd,
    );
    close(fd);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn ticker(mut _arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut now: time_t = 0;
    let mut ct: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut avload: libc::c_float = 0.;
    loop {
        pth_sleep(5 as libc::c_int as libc::c_uint);
        now = time(0 as *mut time_t);
        ct = ctime(&mut now);
        *ct
            .offset(
                (strlen(ct)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        pth_ctrl(
            ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong,
            &mut avload as *mut libc::c_float,
        );
        fprintf(
            stderr,
            b"ticker woken up on %s, average load: %.2f\n\0" as *const u8
                as *const libc::c_char,
            ct,
            avload as libc::c_double,
        );
    };
}
static mut s: libc::c_int = 0;
#[no_mangle]
pub static mut attr: pth_attr_t = 0 as *const pth_attr_st as *mut pth_attr_st;
unsafe extern "C" fn myexit(mut sig: libc::c_int) {
    close(s);
    pth_attr_destroy(attr);
    pth_kill();
    fprintf(stderr, b"**Break\n\0" as *const u8 as *const libc::c_char);
    exit(0 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
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
    let mut sr: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    pth_init();
    signal(
        13 as libc::c_int,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(2 as libc::c_int, Some(myexit as unsafe extern "C" fn(libc::c_int) -> ()));
    signal(15 as libc::c_int, Some(myexit as unsafe extern "C" fn(libc::c_int) -> ()));
    if argc != 2 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: %s <port>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    port = atoi(*argv.offset(1 as libc::c_int as isize));
    if port <= 0 as libc::c_int || port >= 65535 as libc::c_int {
        fprintf(
            stderr,
            b"Illegal port: %d\n\0" as *const u8 as *const libc::c_char,
            port,
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        stderr,
        b"This is TEST_HTTPD, a Pth test using socket I/O.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"Multiple connections are accepted on the specified port.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"For each connection a separate thread is spawned which\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"reads a HTTP request the socket and writes back a constant\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"(and useless) HTTP response to the socket.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Additionally a useless ticker thread awakens every 5s.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Watch the average scheduler load the ticker displays.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Hit CTRL-C for stopping this test.\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    attr = pth_attr_new();
    pth_attr_set(
        attr,
        PTH_ATTR_NAME as libc::c_int,
        b"ticker\0" as *const u8 as *const libc::c_char,
    );
    pth_attr_set(attr, PTH_ATTR_JOINABLE as libc::c_int, 0 as libc::c_int);
    pth_attr_set(
        attr,
        PTH_ATTR_STACK_SIZE as libc::c_int,
        64 as libc::c_int * 1024 as libc::c_int,
    );
    pth_spawn(
        attr,
        Some(ticker as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pe = getprotobyname(b"tcp\0" as *const u8 as *const libc::c_char);
    if pe.is_null() {
        perror(b"getprotobyname\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    s = socket(2 as libc::c_int, SOCK_STREAM as libc::c_int, (*pe).p_proto);
    if s == -(1 as libc::c_int) {
        perror(b"socket\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    sar.sin_family = 2 as libc::c_int as sa_family_t;
    sar.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    sar
        .sin_port = ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = port as libc::c_ushort;
        if 0 != 0 {
            __v = (__x as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                | (__x as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
                as libc::c_ushort;
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
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        perror(b"socket\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if listen(s, 1024 as libc::c_int - 100 as libc::c_int) == -(1 as libc::c_int) {
        perror(b"listen\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    pth_attr_set(
        attr,
        PTH_ATTR_NAME as libc::c_int,
        b"handler\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"listening on port %d (max %d simultaneous connections)\n\0" as *const u8
            as *const libc::c_char,
        port,
        1024 as libc::c_int - 100 as libc::c_int,
    );
    loop {
        peer_len = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
        sr = pth_accept(
            s,
            &mut peer_addr as *mut sockaddr_in as *mut sockaddr,
            &mut peer_len,
        );
        if sr == -(1 as libc::c_int) {
            perror(b"accept\0" as *const u8 as *const libc::c_char);
            pth_sleep(1 as libc::c_int as libc::c_uint);
        } else if pth_ctrl(
            ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
        ) >= (1024 as libc::c_int - 100 as libc::c_int) as libc::c_long
        {
            fprintf(
                stderr,
                b"currently no more connections acceptable\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            fprintf(
                stderr,
                b"connection established (fd: %d, ip: %s, port: %d)\n\0" as *const u8
                    as *const libc::c_char,
                sr,
                inet_ntoa(peer_addr.sin_addr),
                ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = peer_addr.sin_port;
                    if 0 != 0 {
                        __v = (__x as libc::c_int >> 8 as libc::c_int
                            & 0xff as libc::c_int
                            | (__x as libc::c_int & 0xff as libc::c_int)
                                << 8 as libc::c_int) as libc::c_ushort;
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
                }) as libc::c_int,
            );
            pth_spawn(
                attr,
                Some(
                    handler
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                sr as libc::c_long as *mut libc::c_void,
            );
        }
    };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
