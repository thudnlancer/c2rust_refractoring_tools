use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn select(
        __nfds: i32,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn pclose(__stream: *mut FILE) -> i32;
    fn popen(__command: *const i8, __modes: *const i8) -> *mut FILE;
    fn fileno(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stdin: *mut _IO_FILE;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn ptsname(__fd: i32) -> *mut i8;
    fn unlockpt(__fd: i32) -> i32;
    fn grantpt(__fd: i32) -> i32;
    fn posix_openpt(__oflag: i32) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const i8,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const i8, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn access(__name: *const i8, __type: i32) -> i32;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn usleep(__useconds: __useconds_t) -> i32;
    fn dup(__fd: i32) -> i32;
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    fn execl(__path: *const i8, __arg: *const i8, _: ...) -> i32;
    fn _exit(_: i32) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    static mut NR: i64;
    static mut FNR: i64;
    static mut BINMODE: i32;
    static mut IGNORECASE: bool;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut FILENAME_node: *mut NODE;
    static mut FNR_node: *mut NODE;
    static mut NR_node: *mut NODE;
    static mut RS_node: *mut NODE;
    static mut RT_node: *mut NODE;
    static mut SUBSEP_node: *mut NODE;
    static mut PROCINFO_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut fields_arr: *mut *mut NODE;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: i32;
    static mut defpath: *const i8;
    static mut deflibpath: *const i8;
    static envsep: i8;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    static mut stack_ptr: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn array_vname(symbol: *const NODE) -> *const i8;
    fn efflush(fp: *mut FILE, from: *const i8, rp: *mut redirect);
    fn sanitize_exit_status(status: i32) -> i32;
    fn update_ERRNO_int(_: i32);
    fn update_ERRNO_string(string: *const i8);
    fn genflags2str(flagval: i32, tab: *const flagtab) -> *const i8;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn set_record(buf: *const i8, cnt: size_t, _: *const awk_fieldwidth_info_t);
    fn set_FS();
    fn current_field_sep() -> field_sep_type;
    static mut btowc_cache: [wint_t; 0];
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn set_loc(file: *const i8, line: i32);
    fn reisstring(text: *const i8, len: size_t, re: *mut Regexp, buf: *const i8) -> i32;
    fn research(
        rp: *mut Regexp,
        str: *mut i8,
        start: i32,
        len: size_t,
        flags: i32,
    ) -> i32;
    fn make_regexp(
        s: *const i8,
        len: size_t,
        ignorecase: bool,
        dfa: bool,
        canfatal: bool,
    ) -> *mut Regexp;
    fn refree(rp: *mut Regexp);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn os_devopen(name: *const i8, flag: i32) -> i32;
    fn os_close_on_exec(fd: i32, name: *const i8, what: *const i8, dir: *const i8);
    fn os_isatty(fd: i32) -> i32;
    fn os_isreadable(iobuf: *const awk_input_buf_t, isdir: *mut bool) -> i32;
    fn os_setbinmode(fd: i32, mode: i32) -> i32;
    fn os_restore_mode(fd: i32);
    fn os_maybe_set_errno();
    fn optimal_bufsize(fd: i32, sbuf: *mut stat) -> size_t;
    fn ispath(file: *const i8) -> i32;
    fn isdirpunct(c: i32) -> i32;
    fn init_sockets();
    fn getenv_long(name: *const i8) -> i64;
    fn r_fatal(mesg: *const i8, _: ...);
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn r_warning(mesg: *const i8, _: ...);
    fn arg_assign(arg: *mut i8, initing: bool) -> i32;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn r_free_wstr(n: *mut NODE);
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn tcsetattr(__fd: i32, __optional_actions: i32, __termios_p: *const termios) -> i32;
    fn tcgetattr(__fd: i32, __termios_p: *mut termios) -> i32;
    fn shutdown(__fd: i32, __how: i32) -> i32;
    fn connect(__fd: i32, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> i32;
    fn recvfrom(
        __fd: i32,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: i32,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
    fn setsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> i32;
    fn bind(__fd: i32, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> i32;
    fn listen(__fd: i32, __n: i32) -> i32;
    fn accept(__fd: i32, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> i32;
    fn getaddrinfo(
        __name: *const i8,
        __service: *const i8,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> i32;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: i32) -> *const i8;
    static mut ARGC_node: *mut NODE;
    static mut ARGV_node: *mut NODE;
    static mut ARGIND_node: *mut NODE;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __useconds_t = u32;
pub type __suseconds_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type __socklen_t = u32;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
pub type wint_t = u32;
pub type mbstate_t = __mbstate_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type socklen_t = __socklen_t;
pub type __re_size_t = u32;
pub type __re_long_size_t = u64;
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut u8,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut dfa,
    pub has_meta: bool,
    pub maybe_long: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_bool {
    awk_false = 0,
    awk_true,
}
impl awk_bool {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            awk_bool::awk_false => 0,
            awk_bool::awk_true => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> awk_bool {
        match value {
            0 => awk_bool::awk_false,
            1 => awk_bool::awk_true,
            _ => panic!("Invalid value for awk_bool: {}", value),
        }
    }
}
impl AddAssign<u32> for awk_bool {
    fn add_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for awk_bool {
    fn sub_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for awk_bool {
    fn mul_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for awk_bool {
    fn div_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for awk_bool {
    fn rem_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for awk_bool {
    type Output = awk_bool;
    fn add(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for awk_bool {
    type Output = awk_bool;
    fn sub(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for awk_bool {
    type Output = awk_bool;
    fn mul(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for awk_bool {
    type Output = awk_bool;
    fn div(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for awk_bool {
    type Output = awk_bool;
    fn rem(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input {
    pub name: *const i8,
    pub fd: i32,
    pub opaque: *mut libc::c_void,
    pub get_record: Option<
        unsafe extern "C" fn(
            *mut *mut i8,
            *mut awk_input,
            *mut i32,
            *mut *mut i8,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> i32,
    >,
    pub read_func: Option<
        unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input_parser {
    pub name: *const i8,
    pub can_take_file: Option<
        unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option<
        unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_input_parser,
}
pub type awk_input_parser_t = awk_input_parser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const i8,
    pub mode: *const i8,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
    pub gawk_ferror: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
    pub gawk_fclose: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_wrapper {
    pub name: *const i8,
    pub can_take_file: Option<
        unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option<
        unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_output_wrapper,
}
pub type awk_output_wrapper_t = awk_output_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_two_way_processor {
    pub name: *const i8,
    pub can_take_two_way: Option<unsafe extern "C" fn(*const i8) -> awk_bool_t>,
    pub take_control_of: Option<
        unsafe extern "C" fn(
            *const i8,
            *mut awk_input_buf_t,
            *mut awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub next: *mut awk_two_way_processor,
}
pub type awk_two_way_processor_t = awk_two_way_processor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut i8,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE,
    AWK_NUMBER_TYPE_MPFR,
    AWK_NUMBER_TYPE_MPZ,
}
impl AWK_NUMBER_TYPE {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE => 0,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR => 1,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> AWK_NUMBER_TYPE {
        match value {
            0 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE,
            1 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR,
            2 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ,
            _ => panic!("Invalid value for AWK_NUMBER_TYPE: {}", value),
        }
    }
}
impl AddAssign<u32> for AWK_NUMBER_TYPE {
    fn add_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for AWK_NUMBER_TYPE {
    fn sub_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for AWK_NUMBER_TYPE {
    fn mul_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for AWK_NUMBER_TYPE {
    fn div_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for AWK_NUMBER_TYPE {
    fn rem_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn add(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn sub(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn mul(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn div(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn rem(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: libc::c_double,
    pub type_0: AWK_NUMBER_TYPE,
    pub ptr: *mut libc::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut libc::c_void;
pub type awk_scalar_t = *mut libc::c_void;
pub type awk_value_cookie_t = *mut libc::c_void;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_valtype_t {
    AWK_UNDEFINED,
    AWK_NUMBER,
    AWK_STRING,
    AWK_REGEX,
    AWK_STRNUM,
    AWK_ARRAY,
    AWK_SCALAR,
    AWK_VALUE_COOKIE,
    AWK_BOOL,
}
impl awk_valtype_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            awk_valtype_t::AWK_UNDEFINED => 0,
            awk_valtype_t::AWK_NUMBER => 1,
            awk_valtype_t::AWK_STRING => 2,
            awk_valtype_t::AWK_REGEX => 3,
            awk_valtype_t::AWK_STRNUM => 4,
            awk_valtype_t::AWK_ARRAY => 5,
            awk_valtype_t::AWK_SCALAR => 6,
            awk_valtype_t::AWK_VALUE_COOKIE => 7,
            awk_valtype_t::AWK_BOOL => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> awk_valtype_t {
        match value {
            0 => awk_valtype_t::AWK_UNDEFINED,
            1 => awk_valtype_t::AWK_NUMBER,
            2 => awk_valtype_t::AWK_STRING,
            3 => awk_valtype_t::AWK_REGEX,
            4 => awk_valtype_t::AWK_STRNUM,
            5 => awk_valtype_t::AWK_ARRAY,
            6 => awk_valtype_t::AWK_SCALAR,
            7 => awk_valtype_t::AWK_VALUE_COOKIE,
            8 => awk_valtype_t::AWK_BOOL,
            _ => panic!("Invalid value for awk_valtype_t: {}", value),
        }
    }
}
impl AddAssign<u32> for awk_valtype_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for awk_valtype_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for awk_valtype_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for awk_valtype_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for awk_valtype_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn add(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn sub(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn mul(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn div(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn rem(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const i8,
    pub function: Option<
        unsafe extern "C" fn(
            i32,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut libc::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum nodevals {
    Node_illegal,
    Node_val,
    Node_regex,
    Node_dynregex,
    Node_var,
    Node_var_array,
    Node_var_new,
    Node_elem_new,
    Node_param_list,
    Node_func,
    Node_ext_func,
    Node_builtin_func,
    Node_array_ref,
    Node_array_tree,
    Node_array_leaf,
    Node_dump_array,
    Node_arrayfor,
    Node_frame,
    Node_instruction,
    Node_final,
}
impl nodevals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            nodevals::Node_illegal => 0,
            nodevals::Node_val => 1,
            nodevals::Node_regex => 2,
            nodevals::Node_dynregex => 3,
            nodevals::Node_var => 4,
            nodevals::Node_var_array => 5,
            nodevals::Node_var_new => 6,
            nodevals::Node_elem_new => 7,
            nodevals::Node_param_list => 8,
            nodevals::Node_func => 9,
            nodevals::Node_ext_func => 10,
            nodevals::Node_builtin_func => 11,
            nodevals::Node_array_ref => 12,
            nodevals::Node_array_tree => 13,
            nodevals::Node_array_leaf => 14,
            nodevals::Node_dump_array => 15,
            nodevals::Node_arrayfor => 16,
            nodevals::Node_frame => 17,
            nodevals::Node_instruction => 18,
            nodevals::Node_final => 19,
        }
    }
    fn from_libc_c_uint(value: u32) -> nodevals {
        match value {
            0 => nodevals::Node_illegal,
            1 => nodevals::Node_val,
            2 => nodevals::Node_regex,
            3 => nodevals::Node_dynregex,
            4 => nodevals::Node_var,
            5 => nodevals::Node_var_array,
            6 => nodevals::Node_var_new,
            7 => nodevals::Node_elem_new,
            8 => nodevals::Node_param_list,
            9 => nodevals::Node_func,
            10 => nodevals::Node_ext_func,
            11 => nodevals::Node_builtin_func,
            12 => nodevals::Node_array_ref,
            13 => nodevals::Node_array_tree,
            14 => nodevals::Node_array_leaf,
            15 => nodevals::Node_dump_array,
            16 => nodevals::Node_arrayfor,
            17 => nodevals::Node_frame,
            18 => nodevals::Node_instruction,
            19 => nodevals::Node_final,
            _ => panic!("Invalid value for nodevals: {}", value),
        }
    }
}
impl AddAssign<u32> for nodevals {
    fn add_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for nodevals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for nodevals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for nodevals {
    fn div_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for nodevals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for nodevals {
    type Output = nodevals;
    fn add(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for nodevals {
    type Output = nodevals;
    fn sub(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for nodevals {
    type Output = nodevals;
    fn mul(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for nodevals {
    type Output = nodevals;
    fn div(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for nodevals {
    type Output = nodevals;
    fn rem(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2RustUnnamed_1,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: i64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum flagvals {
    REGEX = 524288,
    NUMCONSTSTR = 262144,
    XARRAY = 131072,
    HALFHAT = 65536,
    ARRAYMAXED = 32768,
    NULL_FIELD = 16384,
    NO_EXT_SET = 8192,
    MPZN = 4096,
    MPFN = 2048,
    WSTRCUR = 1024,
    INTIND = 512,
    NUMINT = 256,
    INTLSTR = 128,
    BOOLVAL = 64,
    USER_INPUT = 32,
    NUMBER = 16,
    NUMCUR = 8,
    STRCUR = 4,
    STRING = 2,
    MALLOC = 1,
}
impl flagvals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            flagvals::REGEX => 524288,
            flagvals::NUMCONSTSTR => 262144,
            flagvals::XARRAY => 131072,
            flagvals::HALFHAT => 65536,
            flagvals::ARRAYMAXED => 32768,
            flagvals::NULL_FIELD => 16384,
            flagvals::NO_EXT_SET => 8192,
            flagvals::MPZN => 4096,
            flagvals::MPFN => 2048,
            flagvals::WSTRCUR => 1024,
            flagvals::INTIND => 512,
            flagvals::NUMINT => 256,
            flagvals::INTLSTR => 128,
            flagvals::BOOLVAL => 64,
            flagvals::USER_INPUT => 32,
            flagvals::NUMBER => 16,
            flagvals::NUMCUR => 8,
            flagvals::STRCUR => 4,
            flagvals::STRING => 2,
            flagvals::MALLOC => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> flagvals {
        match value {
            524288 => flagvals::REGEX,
            262144 => flagvals::NUMCONSTSTR,
            131072 => flagvals::XARRAY,
            65536 => flagvals::HALFHAT,
            32768 => flagvals::ARRAYMAXED,
            16384 => flagvals::NULL_FIELD,
            8192 => flagvals::NO_EXT_SET,
            4096 => flagvals::MPZN,
            2048 => flagvals::MPFN,
            1024 => flagvals::WSTRCUR,
            512 => flagvals::INTIND,
            256 => flagvals::NUMINT,
            128 => flagvals::INTLSTR,
            64 => flagvals::BOOLVAL,
            32 => flagvals::USER_INPUT,
            16 => flagvals::NUMBER,
            8 => flagvals::NUMCUR,
            4 => flagvals::STRCUR,
            2 => flagvals::STRING,
            1 => flagvals::MALLOC,
            _ => panic!("Invalid value for flagvals: {}", value),
        }
    }
}
impl AddAssign<u32> for flagvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for flagvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for flagvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for flagvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for flagvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for flagvals {
    type Output = flagvals;
    fn add(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for flagvals {
    type Output = flagvals;
    fn sub(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for flagvals {
    type Output = flagvals;
    fn mul(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for flagvals {
    type Output = flagvals;
    fn div(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for flagvals {
    type Output = flagvals;
    fn rem(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nodep: C2RustUnnamed_3,
    pub val: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fltnum: libc::c_double,
    pub sp: *mut i8,
    pub slen: size_t,
    pub idx: i32,
    pub wsp: *mut wchar_t,
    pub wslen: size_t,
    pub typre: *mut exp_node,
    pub comtype: commenttype,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum commenttype {
    FOR_COMMENT = 3,
    BLOCK_COMMENT = 2,
    EOL_COMMENT = 1,
}
impl commenttype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            commenttype::FOR_COMMENT => 3,
            commenttype::BLOCK_COMMENT => 2,
            commenttype::EOL_COMMENT => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> commenttype {
        match value {
            3 => commenttype::FOR_COMMENT,
            2 => commenttype::BLOCK_COMMENT,
            1 => commenttype::EOL_COMMENT,
            _ => panic!("Invalid value for commenttype: {}", value),
        }
    }
}
impl AddAssign<u32> for commenttype {
    fn add_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for commenttype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for commenttype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for commenttype {
    fn div_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for commenttype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for commenttype {
    type Output = commenttype;
    fn add(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for commenttype {
    type Output = commenttype;
    fn sub(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for commenttype {
    type Output = commenttype;
    fn mul(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for commenttype {
    type Output = commenttype;
    fn div(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for commenttype {
    type Output = commenttype;
    fn rem(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub l: C2RustUnnamed_10,
    pub r: C2RustUnnamed_5,
    pub x: C2RustUnnamed_4,
    pub name: *mut i8,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: u64,
    pub reflags: reflagvals,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum reflagvals {
    CONSTANT = 1,
    FS_DFLT = 2,
}
impl reflagvals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            reflagvals::CONSTANT => 1,
            reflagvals::FS_DFLT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> reflagvals {
        match value {
            1 => reflagvals::CONSTANT,
            2 => reflagvals::FS_DFLT,
            _ => panic!("Invalid value for reflagvals: {}", value),
        }
    }
}
impl AddAssign<u32> for reflagvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for reflagvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for reflagvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for reflagvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for reflagvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for reflagvals {
    type Output = reflagvals;
    fn add(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for reflagvals {
    type Output = reflagvals;
    fn sub(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for reflagvals {
    type Output = reflagvals;
    fn mul(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for reflagvals {
    type Output = reflagvals;
    fn div(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for reflagvals {
    type Output = reflagvals;
    fn rem(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option<unsafe extern "C" fn() -> ()>,
    pub iptr: *mut exp_instruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_instruction {
    pub nexti: *mut exp_instruction,
    pub d: C2RustUnnamed_7,
    pub x: C2RustUnnamed_6,
    pub comment: *mut exp_instruction,
    pub source_line: libc::c_short,
    pub pool_size: libc::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum opcodeval {
    Op_final = 122,
    Op_parens = 121,
    Op_cond_exp = 120,
    Op_K_function = 119,
    Op_K_else = 118,
    Op_K_if = 117,
    Op_K_switch = 116,
    Op_K_while = 115,
    Op_K_arrayfor = 114,
    Op_K_for = 113,
    Op_K_do = 112,
    Op_list = 111,
    Op_symbol = 110,
    Op_token = 109,
    Op_stop = 108,
    Op_atexit = 107,
    Op_lint_plus = 106,
    Op_lint = 105,
    Op_breakpoint = 104,
    Op_exec_count = 103,
    Op_comment = 102,
    Op_func = 101,
    Op_after_endfile = 100,
    Op_after_beginfile = 99,
    Op_subscript_assign = 98,
    Op_field_assign = 97,
    Op_var_assign = 96,
    Op_var_update = 95,
    Op_arrayfor_final = 94,
    Op_arrayfor_incr = 93,
    Op_arrayfor_init = 92,
    Op_newfile = 91,
    Op_get_record = 90,
    Op_jmp_false = 89,
    Op_jmp_true = 88,
    Op_jmp = 87,
    Op_pop = 86,
    Op_no_op = 85,
    Op_field_spec_lhs = 84,
    Op_subscript_lhs = 83,
    Op_push_lhs = 82,
    Op_push_param = 81,
    Op_push_array = 80,
    Op_push_re = 79,
    Op_push_i = 78,
    Op_push_arg_untyped = 77,
    Op_push_arg = 76,
    Op_push = 75,
    Op_indirect_func_call = 74,
    Op_func_call = 73,
    Op_in_array = 72,
    Op_ext_builtin = 71,
    Op_sub_builtin = 70,
    Op_builtin = 69,
    Op_K_namespace = 68,
    Op_K_nextfile = 67,
    Op_K_getline = 66,
    Op_K_getline_redir = 65,
    Op_K_delete_loop = 64,
    Op_K_delete = 63,
    Op_K_return_from_eval = 62,
    Op_K_return = 61,
    Op_K_exit = 60,
    Op_K_next = 59,
    Op_K_printf = 58,
    Op_K_print_rec = 57,
    Op_K_print = 56,
    Op_K_continue = 55,
    Op_K_break = 54,
    Op_K_default = 53,
    Op_K_case = 52,
    Op_rule = 51,
    Op_nomatch = 50,
    Op_match_rec = 49,
    Op_match = 48,
    Op_geq = 47,
    Op_leq = 46,
    Op_greater = 45,
    Op_less = 44,
    Op_notequal = 43,
    Op_equal = 42,
    Op_or_final = 41,
    Op_or = 40,
    Op_and_final = 39,
    Op_and = 38,
    Op_assign_concat = 37,
    Op_assign_exp = 36,
    Op_assign_minus = 35,
    Op_assign_plus = 34,
    Op_assign_mod = 33,
    Op_assign_quotient = 32,
    Op_assign_times = 31,
    Op_store_field_exp = 30,
    Op_store_field = 29,
    Op_store_sub = 28,
    Op_store_var = 27,
    Op_assign = 26,
    Op_not = 25,
    Op_field_spec = 24,
    Op_unary_plus = 23,
    Op_unary_minus = 22,
    Op_postdecrement = 21,
    Op_postincrement = 20,
    Op_predecrement = 19,
    Op_preincrement = 18,
    Op_sub_array = 17,
    Op_subscript = 16,
    Op_cond_pair = 15,
    Op_line_range = 14,
    Op_concat = 13,
    Op_exp_i = 12,
    Op_exp = 11,
    Op_minus_i = 10,
    Op_minus = 9,
    Op_plus_i = 8,
    Op_plus = 7,
    Op_mod_i = 6,
    Op_mod = 5,
    Op_quotient_i = 4,
    Op_quotient = 3,
    Op_times_i = 2,
    Op_times = 1,
    Op_illegal = 0,
}
impl opcodeval {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            opcodeval::Op_final => 122,
            opcodeval::Op_parens => 121,
            opcodeval::Op_cond_exp => 120,
            opcodeval::Op_K_function => 119,
            opcodeval::Op_K_else => 118,
            opcodeval::Op_K_if => 117,
            opcodeval::Op_K_switch => 116,
            opcodeval::Op_K_while => 115,
            opcodeval::Op_K_arrayfor => 114,
            opcodeval::Op_K_for => 113,
            opcodeval::Op_K_do => 112,
            opcodeval::Op_list => 111,
            opcodeval::Op_symbol => 110,
            opcodeval::Op_token => 109,
            opcodeval::Op_stop => 108,
            opcodeval::Op_atexit => 107,
            opcodeval::Op_lint_plus => 106,
            opcodeval::Op_lint => 105,
            opcodeval::Op_breakpoint => 104,
            opcodeval::Op_exec_count => 103,
            opcodeval::Op_comment => 102,
            opcodeval::Op_func => 101,
            opcodeval::Op_after_endfile => 100,
            opcodeval::Op_after_beginfile => 99,
            opcodeval::Op_subscript_assign => 98,
            opcodeval::Op_field_assign => 97,
            opcodeval::Op_var_assign => 96,
            opcodeval::Op_var_update => 95,
            opcodeval::Op_arrayfor_final => 94,
            opcodeval::Op_arrayfor_incr => 93,
            opcodeval::Op_arrayfor_init => 92,
            opcodeval::Op_newfile => 91,
            opcodeval::Op_get_record => 90,
            opcodeval::Op_jmp_false => 89,
            opcodeval::Op_jmp_true => 88,
            opcodeval::Op_jmp => 87,
            opcodeval::Op_pop => 86,
            opcodeval::Op_no_op => 85,
            opcodeval::Op_field_spec_lhs => 84,
            opcodeval::Op_subscript_lhs => 83,
            opcodeval::Op_push_lhs => 82,
            opcodeval::Op_push_param => 81,
            opcodeval::Op_push_array => 80,
            opcodeval::Op_push_re => 79,
            opcodeval::Op_push_i => 78,
            opcodeval::Op_push_arg_untyped => 77,
            opcodeval::Op_push_arg => 76,
            opcodeval::Op_push => 75,
            opcodeval::Op_indirect_func_call => 74,
            opcodeval::Op_func_call => 73,
            opcodeval::Op_in_array => 72,
            opcodeval::Op_ext_builtin => 71,
            opcodeval::Op_sub_builtin => 70,
            opcodeval::Op_builtin => 69,
            opcodeval::Op_K_namespace => 68,
            opcodeval::Op_K_nextfile => 67,
            opcodeval::Op_K_getline => 66,
            opcodeval::Op_K_getline_redir => 65,
            opcodeval::Op_K_delete_loop => 64,
            opcodeval::Op_K_delete => 63,
            opcodeval::Op_K_return_from_eval => 62,
            opcodeval::Op_K_return => 61,
            opcodeval::Op_K_exit => 60,
            opcodeval::Op_K_next => 59,
            opcodeval::Op_K_printf => 58,
            opcodeval::Op_K_print_rec => 57,
            opcodeval::Op_K_print => 56,
            opcodeval::Op_K_continue => 55,
            opcodeval::Op_K_break => 54,
            opcodeval::Op_K_default => 53,
            opcodeval::Op_K_case => 52,
            opcodeval::Op_rule => 51,
            opcodeval::Op_nomatch => 50,
            opcodeval::Op_match_rec => 49,
            opcodeval::Op_match => 48,
            opcodeval::Op_geq => 47,
            opcodeval::Op_leq => 46,
            opcodeval::Op_greater => 45,
            opcodeval::Op_less => 44,
            opcodeval::Op_notequal => 43,
            opcodeval::Op_equal => 42,
            opcodeval::Op_or_final => 41,
            opcodeval::Op_or => 40,
            opcodeval::Op_and_final => 39,
            opcodeval::Op_and => 38,
            opcodeval::Op_assign_concat => 37,
            opcodeval::Op_assign_exp => 36,
            opcodeval::Op_assign_minus => 35,
            opcodeval::Op_assign_plus => 34,
            opcodeval::Op_assign_mod => 33,
            opcodeval::Op_assign_quotient => 32,
            opcodeval::Op_assign_times => 31,
            opcodeval::Op_store_field_exp => 30,
            opcodeval::Op_store_field => 29,
            opcodeval::Op_store_sub => 28,
            opcodeval::Op_store_var => 27,
            opcodeval::Op_assign => 26,
            opcodeval::Op_not => 25,
            opcodeval::Op_field_spec => 24,
            opcodeval::Op_unary_plus => 23,
            opcodeval::Op_unary_minus => 22,
            opcodeval::Op_postdecrement => 21,
            opcodeval::Op_postincrement => 20,
            opcodeval::Op_predecrement => 19,
            opcodeval::Op_preincrement => 18,
            opcodeval::Op_sub_array => 17,
            opcodeval::Op_subscript => 16,
            opcodeval::Op_cond_pair => 15,
            opcodeval::Op_line_range => 14,
            opcodeval::Op_concat => 13,
            opcodeval::Op_exp_i => 12,
            opcodeval::Op_exp => 11,
            opcodeval::Op_minus_i => 10,
            opcodeval::Op_minus => 9,
            opcodeval::Op_plus_i => 8,
            opcodeval::Op_plus => 7,
            opcodeval::Op_mod_i => 6,
            opcodeval::Op_mod => 5,
            opcodeval::Op_quotient_i => 4,
            opcodeval::Op_quotient => 3,
            opcodeval::Op_times_i => 2,
            opcodeval::Op_times => 1,
            opcodeval::Op_illegal => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> opcodeval {
        match value {
            122 => opcodeval::Op_final,
            121 => opcodeval::Op_parens,
            120 => opcodeval::Op_cond_exp,
            119 => opcodeval::Op_K_function,
            118 => opcodeval::Op_K_else,
            117 => opcodeval::Op_K_if,
            116 => opcodeval::Op_K_switch,
            115 => opcodeval::Op_K_while,
            114 => opcodeval::Op_K_arrayfor,
            113 => opcodeval::Op_K_for,
            112 => opcodeval::Op_K_do,
            111 => opcodeval::Op_list,
            110 => opcodeval::Op_symbol,
            109 => opcodeval::Op_token,
            108 => opcodeval::Op_stop,
            107 => opcodeval::Op_atexit,
            106 => opcodeval::Op_lint_plus,
            105 => opcodeval::Op_lint,
            104 => opcodeval::Op_breakpoint,
            103 => opcodeval::Op_exec_count,
            102 => opcodeval::Op_comment,
            101 => opcodeval::Op_func,
            100 => opcodeval::Op_after_endfile,
            99 => opcodeval::Op_after_beginfile,
            98 => opcodeval::Op_subscript_assign,
            97 => opcodeval::Op_field_assign,
            96 => opcodeval::Op_var_assign,
            95 => opcodeval::Op_var_update,
            94 => opcodeval::Op_arrayfor_final,
            93 => opcodeval::Op_arrayfor_incr,
            92 => opcodeval::Op_arrayfor_init,
            91 => opcodeval::Op_newfile,
            90 => opcodeval::Op_get_record,
            89 => opcodeval::Op_jmp_false,
            88 => opcodeval::Op_jmp_true,
            87 => opcodeval::Op_jmp,
            86 => opcodeval::Op_pop,
            85 => opcodeval::Op_no_op,
            84 => opcodeval::Op_field_spec_lhs,
            83 => opcodeval::Op_subscript_lhs,
            82 => opcodeval::Op_push_lhs,
            81 => opcodeval::Op_push_param,
            80 => opcodeval::Op_push_array,
            79 => opcodeval::Op_push_re,
            78 => opcodeval::Op_push_i,
            77 => opcodeval::Op_push_arg_untyped,
            76 => opcodeval::Op_push_arg,
            75 => opcodeval::Op_push,
            74 => opcodeval::Op_indirect_func_call,
            73 => opcodeval::Op_func_call,
            72 => opcodeval::Op_in_array,
            71 => opcodeval::Op_ext_builtin,
            70 => opcodeval::Op_sub_builtin,
            69 => opcodeval::Op_builtin,
            68 => opcodeval::Op_K_namespace,
            67 => opcodeval::Op_K_nextfile,
            66 => opcodeval::Op_K_getline,
            65 => opcodeval::Op_K_getline_redir,
            64 => opcodeval::Op_K_delete_loop,
            63 => opcodeval::Op_K_delete,
            62 => opcodeval::Op_K_return_from_eval,
            61 => opcodeval::Op_K_return,
            60 => opcodeval::Op_K_exit,
            59 => opcodeval::Op_K_next,
            58 => opcodeval::Op_K_printf,
            57 => opcodeval::Op_K_print_rec,
            56 => opcodeval::Op_K_print,
            55 => opcodeval::Op_K_continue,
            54 => opcodeval::Op_K_break,
            53 => opcodeval::Op_K_default,
            52 => opcodeval::Op_K_case,
            51 => opcodeval::Op_rule,
            50 => opcodeval::Op_nomatch,
            49 => opcodeval::Op_match_rec,
            48 => opcodeval::Op_match,
            47 => opcodeval::Op_geq,
            46 => opcodeval::Op_leq,
            45 => opcodeval::Op_greater,
            44 => opcodeval::Op_less,
            43 => opcodeval::Op_notequal,
            42 => opcodeval::Op_equal,
            41 => opcodeval::Op_or_final,
            40 => opcodeval::Op_or,
            39 => opcodeval::Op_and_final,
            38 => opcodeval::Op_and,
            37 => opcodeval::Op_assign_concat,
            36 => opcodeval::Op_assign_exp,
            35 => opcodeval::Op_assign_minus,
            34 => opcodeval::Op_assign_plus,
            33 => opcodeval::Op_assign_mod,
            32 => opcodeval::Op_assign_quotient,
            31 => opcodeval::Op_assign_times,
            30 => opcodeval::Op_store_field_exp,
            29 => opcodeval::Op_store_field,
            28 => opcodeval::Op_store_sub,
            27 => opcodeval::Op_store_var,
            26 => opcodeval::Op_assign,
            25 => opcodeval::Op_not,
            24 => opcodeval::Op_field_spec,
            23 => opcodeval::Op_unary_plus,
            22 => opcodeval::Op_unary_minus,
            21 => opcodeval::Op_postdecrement,
            20 => opcodeval::Op_postincrement,
            19 => opcodeval::Op_predecrement,
            18 => opcodeval::Op_preincrement,
            17 => opcodeval::Op_sub_array,
            16 => opcodeval::Op_subscript,
            15 => opcodeval::Op_cond_pair,
            14 => opcodeval::Op_line_range,
            13 => opcodeval::Op_concat,
            12 => opcodeval::Op_exp_i,
            11 => opcodeval::Op_exp,
            10 => opcodeval::Op_minus_i,
            9 => opcodeval::Op_minus,
            8 => opcodeval::Op_plus_i,
            7 => opcodeval::Op_plus,
            6 => opcodeval::Op_mod_i,
            5 => opcodeval::Op_mod,
            4 => opcodeval::Op_quotient_i,
            3 => opcodeval::Op_quotient,
            2 => opcodeval::Op_times_i,
            1 => opcodeval::Op_times,
            0 => opcodeval::Op_illegal,
            _ => panic!("Invalid value for opcodeval: {}", value),
        }
    }
}
impl AddAssign<u32> for opcodeval {
    fn add_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for opcodeval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for opcodeval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for opcodeval {
    fn div_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for opcodeval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for opcodeval {
    type Output = opcodeval;
    fn add(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for opcodeval {
    type Output = opcodeval;
    fn sub(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for opcodeval {
    type Output = opcodeval;
    fn mul(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for opcodeval {
    type Output = opcodeval;
    fn div(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for opcodeval {
    type Output = opcodeval;
    fn rem(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub xl: i64,
    pub xn: *mut NODE,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
pub type NODE = exp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub dn: *mut NODE,
    pub di: *mut exp_instruction,
    pub fptr: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
    pub efptr: Option<
        unsafe extern "C" fn(
            i32,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: i64,
    pub ldl: exec_count_t,
    pub name: *mut i8,
}
pub type exec_count_t = libc::c_ulonglong;
pub type BUCKET = bucket_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bucket_item {
    pub hs: C2RustUnnamed_9,
    pub hi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub li: [i64; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub str_0: *mut i8,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub lptr: *mut exp_node,
    pub li: *mut exp_instruction,
    pub ll: i64,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const i8,
    pub init: afunc_t,
    pub type_of: afunc_t,
    pub lookup: afunc_t,
    pub exists: afunc_t,
    pub clear: afunc_t,
    pub remove: afunc_t,
    pub list: afunc_t,
    pub copy: afunc_t,
    pub dump: afunc_t,
    pub store: afunc_t,
}
pub type afunc_t = Option<
    unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redirval {
    redirect_twoway = 6,
    redirect_input = 5,
    redirect_pipein = 4,
    redirect_pipe = 3,
    redirect_append = 2,
    redirect_output = 1,
    redirect_none = 0,
}
impl redirval {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            redirval::redirect_twoway => 6,
            redirval::redirect_input => 5,
            redirval::redirect_pipein => 4,
            redirval::redirect_pipe => 3,
            redirval::redirect_append => 2,
            redirval::redirect_output => 1,
            redirval::redirect_none => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> redirval {
        match value {
            6 => redirval::redirect_twoway,
            5 => redirval::redirect_input,
            4 => redirval::redirect_pipein,
            3 => redirval::redirect_pipe,
            2 => redirval::redirect_append,
            1 => redirval::redirect_output,
            0 => redirval::redirect_none,
            _ => panic!("Invalid value for redirval: {}", value),
        }
    }
}
impl AddAssign<u32> for redirval {
    fn add_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for redirval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for redirval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for redirval {
    fn div_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for redirval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for redirval {
    type Output = redirval;
    fn add(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for redirval {
    type Output = redirval;
    fn sub(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for redirval {
    type Output = redirval;
    fn mul(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for redirval {
    type Output = redirval;
    fn div(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for redirval {
    type Output = redirval;
    fn rem(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type INSTRUCTION = exp_instruction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iobuf {
    pub public: awk_input_buf_t,
    pub buf: *mut i8,
    pub off: *mut i8,
    pub dataend: *mut i8,
    pub end: *mut i8,
    pub readsize: size_t,
    pub size: size_t,
    pub count: ssize_t,
    pub scanoff: size_t,
    pub valid: bool,
    pub errcode: i32,
    pub flag: iobuf_flags,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum iobuf_flags {
    IOP_IS_TTY = 1,
    IOP_AT_EOF = 2,
    IOP_CLOSED = 4,
    IOP_AT_START = 8,
}
impl iobuf_flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            iobuf_flags::IOP_IS_TTY => 1,
            iobuf_flags::IOP_AT_EOF => 2,
            iobuf_flags::IOP_CLOSED => 4,
            iobuf_flags::IOP_AT_START => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> iobuf_flags {
        match value {
            1 => iobuf_flags::IOP_IS_TTY,
            2 => iobuf_flags::IOP_AT_EOF,
            4 => iobuf_flags::IOP_CLOSED,
            8 => iobuf_flags::IOP_AT_START,
            _ => panic!("Invalid value for iobuf_flags: {}", value),
        }
    }
}
impl AddAssign<u32> for iobuf_flags {
    fn add_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for iobuf_flags {
    fn sub_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for iobuf_flags {
    fn mul_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for iobuf_flags {
    fn div_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for iobuf_flags {
    fn rem_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn add(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn sub(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn mul(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn div(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn rem(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type IOBUF = iobuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub flag: redirect_flags,
    pub value: *mut i8,
    pub ifp: *mut FILE,
    pub iop: *mut IOBUF,
    pub pid: i32,
    pub status: i32,
    pub prev: *mut redirect,
    pub next: *mut redirect,
    pub mode: *const i8,
    pub output: awk_output_buf_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redirect_flags {
    RED_TCP = 2048,
    RED_SOCKET = 1024,
    RED_PTY = 512,
    RED_TWOWAY = 256,
    RED_EOF = 128,
    RED_USED = 64,
    RED_FLUSH = 32,
    RED_APPEND = 16,
    RED_WRITE = 8,
    RED_READ = 4,
    RED_PIPE = 2,
    RED_FILE = 1,
    RED_NONE = 0,
}
impl redirect_flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            redirect_flags::RED_TCP => 2048,
            redirect_flags::RED_SOCKET => 1024,
            redirect_flags::RED_PTY => 512,
            redirect_flags::RED_TWOWAY => 256,
            redirect_flags::RED_EOF => 128,
            redirect_flags::RED_USED => 64,
            redirect_flags::RED_FLUSH => 32,
            redirect_flags::RED_APPEND => 16,
            redirect_flags::RED_WRITE => 8,
            redirect_flags::RED_READ => 4,
            redirect_flags::RED_PIPE => 2,
            redirect_flags::RED_FILE => 1,
            redirect_flags::RED_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> redirect_flags {
        match value {
            2048 => redirect_flags::RED_TCP,
            1024 => redirect_flags::RED_SOCKET,
            512 => redirect_flags::RED_PTY,
            256 => redirect_flags::RED_TWOWAY,
            128 => redirect_flags::RED_EOF,
            64 => redirect_flags::RED_USED,
            32 => redirect_flags::RED_FLUSH,
            16 => redirect_flags::RED_APPEND,
            8 => redirect_flags::RED_WRITE,
            4 => redirect_flags::RED_READ,
            2 => redirect_flags::RED_PIPE,
            1 => redirect_flags::RED_FILE,
            0 => redirect_flags::RED_NONE,
            _ => panic!("Invalid value for redirect_flags: {}", value),
        }
    }
}
impl AddAssign<u32> for redirect_flags {
    fn add_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for redirect_flags {
    fn sub_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for redirect_flags {
    fn mul_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for redirect_flags {
    fn div_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for redirect_flags {
    fn rem_assign(&mut self, rhs: u32) {
        *self = redirect_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for redirect_flags {
    type Output = redirect_flags;
    fn add(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for redirect_flags {
    type Output = redirect_flags;
    fn sub(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for redirect_flags {
    type Output = redirect_flags;
    fn mul(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for redirect_flags {
    type Output = redirect_flags;
    fn div(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for redirect_flags {
    type Output = redirect_flags;
    fn rem(self, rhs: u32) -> redirect_flags {
        redirect_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type redirect_flags_t = redirect_flags;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum binmode_values {
    TEXT_TRANSLATE = 0,
    BINMODE_INPUT = 1,
    BINMODE_OUTPUT = 2,
    BINMODE_BOTH = 3,
}
impl binmode_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            binmode_values::TEXT_TRANSLATE => 0,
            binmode_values::BINMODE_INPUT => 1,
            binmode_values::BINMODE_OUTPUT => 2,
            binmode_values::BINMODE_BOTH => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> binmode_values {
        match value {
            0 => binmode_values::TEXT_TRANSLATE,
            1 => binmode_values::BINMODE_INPUT,
            2 => binmode_values::BINMODE_OUTPUT,
            3 => binmode_values::BINMODE_BOTH,
            _ => panic!("Invalid value for binmode_values: {}", value),
        }
    }
}
impl AddAssign<u32> for binmode_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for binmode_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for binmode_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for binmode_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for binmode_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for binmode_values {
    type Output = binmode_values;
    fn add(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for binmode_values {
    type Output = binmode_values;
    fn sub(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for binmode_values {
    type Output = binmode_values;
    fn mul(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for binmode_values {
    type Output = binmode_values;
    fn div(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for binmode_values {
    type Output = binmode_values;
    fn rem(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srcfile {
    pub next: *mut srcfile,
    pub prev: *mut srcfile,
    pub stype: srctype,
    pub src: *mut i8,
    pub fullpath: *mut i8,
    pub mtime: time_t,
    pub sbuf: stat,
    pub srclines: i32,
    pub bufsize: size_t,
    pub buf: *mut i8,
    pub line_offset: *mut i32,
    pub fd: i32,
    pub maxlen: i32,
    pub fini_func: Option<unsafe extern "C" fn() -> ()>,
    pub lexptr: *mut i8,
    pub lexend: *mut i8,
    pub lexeme: *mut i8,
    pub lexptr_begin: *mut i8,
    pub lasttok: i32,
    pub comment: *mut INSTRUCTION,
    pub namespace: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum srctype {
    SRC_CMDLINE = 1,
    SRC_STDIN,
    SRC_FILE,
    SRC_INC,
    SRC_EXTLIB,
}
impl srctype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            srctype::SRC_CMDLINE => 1,
            srctype::SRC_STDIN => 2,
            srctype::SRC_FILE => 3,
            srctype::SRC_INC => 4,
            srctype::SRC_EXTLIB => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> srctype {
        match value {
            1 => srctype::SRC_CMDLINE,
            2 => srctype::SRC_STDIN,
            3 => srctype::SRC_FILE,
            4 => srctype::SRC_INC,
            5 => srctype::SRC_EXTLIB,
            _ => panic!("Invalid value for srctype: {}", value),
        }
    }
}
impl AddAssign<u32> for srctype {
    fn add_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for srctype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for srctype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for srctype {
    fn div_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for srctype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for srctype {
    type Output = srctype;
    fn add(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for srctype {
    type Output = srctype;
    fn sub(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for srctype {
    type Output = srctype;
    fn mul(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for srctype {
    type Output = srctype;
    fn div(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for srctype {
    type Output = srctype;
    fn rem(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type SRCFILE = srcfile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagtab {
    pub val: i32,
    pub name: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum do_flag_values {
    DO_MPFR = 32768,
    DO_DEBUG = 16384,
    DO_PROFILE = 8192,
    DO_SANDBOX = 4096,
    DO_TIDY_MEM = 2048,
    DO_DUMP_VARS = 1024,
    DO_PRETTY_PRINT = 512,
    DO_INTERVALS = 256,
    DO_NON_DEC_DATA = 128,
    DO_INTL = 64,
    DO_POSIX = 32,
    DO_TRADITIONAL = 16,
    DO_LINT_OLD = 8,
    DO_LINT_ALL = 4,
    DO_LINT_EXTENSIONS = 2,
    DO_LINT_INVALID = 1,
    DO_FLAG_NONE = 0,
}
impl do_flag_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            do_flag_values::DO_MPFR => 32768,
            do_flag_values::DO_DEBUG => 16384,
            do_flag_values::DO_PROFILE => 8192,
            do_flag_values::DO_SANDBOX => 4096,
            do_flag_values::DO_TIDY_MEM => 2048,
            do_flag_values::DO_DUMP_VARS => 1024,
            do_flag_values::DO_PRETTY_PRINT => 512,
            do_flag_values::DO_INTERVALS => 256,
            do_flag_values::DO_NON_DEC_DATA => 128,
            do_flag_values::DO_INTL => 64,
            do_flag_values::DO_POSIX => 32,
            do_flag_values::DO_TRADITIONAL => 16,
            do_flag_values::DO_LINT_OLD => 8,
            do_flag_values::DO_LINT_ALL => 4,
            do_flag_values::DO_LINT_EXTENSIONS => 2,
            do_flag_values::DO_LINT_INVALID => 1,
            do_flag_values::DO_FLAG_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> do_flag_values {
        match value {
            32768 => do_flag_values::DO_MPFR,
            16384 => do_flag_values::DO_DEBUG,
            8192 => do_flag_values::DO_PROFILE,
            4096 => do_flag_values::DO_SANDBOX,
            2048 => do_flag_values::DO_TIDY_MEM,
            1024 => do_flag_values::DO_DUMP_VARS,
            512 => do_flag_values::DO_PRETTY_PRINT,
            256 => do_flag_values::DO_INTERVALS,
            128 => do_flag_values::DO_NON_DEC_DATA,
            64 => do_flag_values::DO_INTL,
            32 => do_flag_values::DO_POSIX,
            16 => do_flag_values::DO_TRADITIONAL,
            8 => do_flag_values::DO_LINT_OLD,
            4 => do_flag_values::DO_LINT_ALL,
            2 => do_flag_values::DO_LINT_EXTENSIONS,
            1 => do_flag_values::DO_LINT_INVALID,
            0 => do_flag_values::DO_FLAG_NONE,
            _ => panic!("Invalid value for do_flag_values: {}", value),
        }
    }
}
impl AddAssign<u32> for do_flag_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for do_flag_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for do_flag_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for do_flag_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for do_flag_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for do_flag_values {
    type Output = do_flag_values;
    fn add(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for do_flag_values {
    type Output = do_flag_values;
    fn sub(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for do_flag_values {
    type Output = do_flag_values;
    fn mul(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for do_flag_values {
    type Output = do_flag_values;
    fn div(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for do_flag_values {
    type Output = do_flag_values;
    fn rem(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union stack_item {
    pub rptr: *mut NODE,
    pub lptr: *mut *mut NODE,
}
pub type STACK_ITEM = stack_item;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum field_sep_type {
    Using_FS,
    Using_FIELDWIDTHS,
    Using_FPAT,
    Using_API,
}
impl field_sep_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            field_sep_type::Using_FS => 0,
            field_sep_type::Using_FIELDWIDTHS => 1,
            field_sep_type::Using_FPAT => 2,
            field_sep_type::Using_API => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> field_sep_type {
        match value {
            0 => field_sep_type::Using_FS,
            1 => field_sep_type::Using_FIELDWIDTHS,
            2 => field_sep_type::Using_FPAT,
            3 => field_sep_type::Using_API,
            _ => panic!("Invalid value for field_sep_type: {}", value),
        }
    }
}
impl AddAssign<u32> for field_sep_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for field_sep_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for field_sep_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for field_sep_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for field_sep_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = field_sep_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for field_sep_type {
    type Output = field_sep_type;
    fn add(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for field_sep_type {
    type Output = field_sep_type;
    fn sub(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for field_sep_type {
    type Output = field_sep_type;
    fn mul(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for field_sep_type {
    type Output = field_sep_type;
    fn div(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for field_sep_type {
    type Output = field_sep_type;
    fn rem(self, rhs: u32) -> field_sep_type {
        field_sep_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type SCANSTATE = scanstate;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum scanstate {
    INTERM = 3,
    INDATA = 2,
    INLEADER = 1,
    NOSTATE = 0,
}
impl scanstate {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            scanstate::INTERM => 3,
            scanstate::INDATA => 2,
            scanstate::INLEADER => 1,
            scanstate::NOSTATE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> scanstate {
        match value {
            3 => scanstate::INTERM,
            2 => scanstate::INDATA,
            1 => scanstate::INLEADER,
            0 => scanstate::NOSTATE,
            _ => panic!("Invalid value for scanstate: {}", value),
        }
    }
}
impl AddAssign<u32> for scanstate {
    fn add_assign(&mut self, rhs: u32) {
        *self = scanstate::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for scanstate {
    fn sub_assign(&mut self, rhs: u32) {
        *self = scanstate::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for scanstate {
    fn mul_assign(&mut self, rhs: u32) {
        *self = scanstate::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for scanstate {
    fn div_assign(&mut self, rhs: u32) {
        *self = scanstate::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for scanstate {
    fn rem_assign(&mut self, rhs: u32) {
        *self = scanstate::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for scanstate {
    type Output = scanstate;
    fn add(self, rhs: u32) -> scanstate {
        scanstate::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for scanstate {
    type Output = scanstate;
    fn sub(self, rhs: u32) -> scanstate {
        scanstate::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for scanstate {
    type Output = scanstate;
    fn mul(self, rhs: u32) -> scanstate {
        scanstate::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for scanstate {
    type Output = scanstate;
    fn div(self, rhs: u32) -> scanstate {
        scanstate::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for scanstate {
    type Output = scanstate;
    fn rem(self, rhs: u32) -> scanstate {
        scanstate::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recmatch {
    pub start: *mut i8,
    pub len: size_t,
    pub rt_start: *mut i8,
    pub rt_len: size_t,
}
pub type RECVALUE = recvalues;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum recvalues {
    REC_OK,
    NOTERM,
    TERMATEND,
    TERMNEAREND,
}
impl recvalues {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            recvalues::REC_OK => 0,
            recvalues::NOTERM => 1,
            recvalues::TERMATEND => 2,
            recvalues::TERMNEAREND => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> recvalues {
        match value {
            0 => recvalues::REC_OK,
            1 => recvalues::NOTERM,
            2 => recvalues::TERMATEND,
            3 => recvalues::TERMNEAREND,
            _ => panic!("Invalid value for recvalues: {}", value),
        }
    }
}
impl AddAssign<u32> for recvalues {
    fn add_assign(&mut self, rhs: u32) {
        *self = recvalues::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for recvalues {
    fn sub_assign(&mut self, rhs: u32) {
        *self = recvalues::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for recvalues {
    fn mul_assign(&mut self, rhs: u32) {
        *self = recvalues::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for recvalues {
    fn div_assign(&mut self, rhs: u32) {
        *self = recvalues::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for recvalues {
    fn rem_assign(&mut self, rhs: u32) {
        *self = recvalues::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for recvalues {
    type Output = recvalues;
    fn add(self, rhs: u32) -> recvalues {
        recvalues::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for recvalues {
    type Output = recvalues;
    fn sub(self, rhs: u32) -> recvalues {
        recvalues::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for recvalues {
    type Output = recvalues;
    fn mul(self, rhs: u32) -> recvalues {
        recvalues::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for recvalues {
    type Output = recvalues;
    fn div(self, rhs: u32) -> recvalues {
        recvalues::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for recvalues {
    type Output = recvalues;
    fn rem(self, rhs: u32) -> recvalues {
        recvalues::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub offset: i32,
    pub len: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inet_socket_info {
    pub family: i32,
    pub protocol: i32,
    pub localport: C2RustUnnamed_11,
    pub remotehost: C2RustUnnamed_11,
    pub remoteport: C2RustUnnamed_11,
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
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [i8; 118],
    pub __ss_align: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
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
    pub __in6_u: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
pub const MSG_PEEK: C2RustUnnamed_13 = 2;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_DGRAM = 2,
    SOCK_STREAM = 1,
    SOCK_NONBLOCK = 2048,
    SOCK_CLOEXEC = 524288,
    SOCK_PACKET = 10,
    SOCK_DCCP = 6,
    SOCK_SEQPACKET = 5,
    SOCK_RDM = 4,
    SOCK_RAW = 3,
}
impl __socket_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __socket_type::SOCK_DGRAM => 2,
            __socket_type::SOCK_STREAM => 1,
            __socket_type::SOCK_NONBLOCK => 2048,
            __socket_type::SOCK_CLOEXEC => 524288,
            __socket_type::SOCK_PACKET => 10,
            __socket_type::SOCK_DCCP => 6,
            __socket_type::SOCK_SEQPACKET => 5,
            __socket_type::SOCK_RDM => 4,
            __socket_type::SOCK_RAW => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> __socket_type {
        match value {
            2 => __socket_type::SOCK_DGRAM,
            1 => __socket_type::SOCK_STREAM,
            2048 => __socket_type::SOCK_NONBLOCK,
            524288 => __socket_type::SOCK_CLOEXEC,
            10 => __socket_type::SOCK_PACKET,
            6 => __socket_type::SOCK_DCCP,
            5 => __socket_type::SOCK_SEQPACKET,
            4 => __socket_type::SOCK_RDM,
            3 => __socket_type::SOCK_RAW,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linger {
    pub l_onoff: i32,
    pub l_linger: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = u32;
pub type cc_t = u8;
pub type tcflag_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mixture {
    pub common: redirect_flags_t,
    pub mode: redirect_flags_t,
    pub other_mode: redirect_flags_t,
    pub message: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum two_way_close_type {
    CLOSE_ALL,
    CLOSE_TO,
    CLOSE_FROM,
}
impl two_way_close_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            two_way_close_type::CLOSE_ALL => 0,
            two_way_close_type::CLOSE_TO => 1,
            two_way_close_type::CLOSE_FROM => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> two_way_close_type {
        match value {
            0 => two_way_close_type::CLOSE_ALL,
            1 => two_way_close_type::CLOSE_TO,
            2 => two_way_close_type::CLOSE_FROM,
            _ => panic!("Invalid value for two_way_close_type: {}", value),
        }
    }
}
impl AddAssign<u32> for two_way_close_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for two_way_close_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for two_way_close_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for two_way_close_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for two_way_close_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn add(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn sub(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn mul(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn div(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for two_way_close_type {
    type Output = two_way_close_type;
    fn rem(self, rhs: u32) -> two_way_close_type {
        two_way_close_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_14 {
    SHUT_RD = 0,
    SHUT_WR = 1,
    SHUT_RDWR = 2,
}
impl C2RustUnnamed_14 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_14::SHUT_RD => 0,
            C2RustUnnamed_14::SHUT_WR => 1,
            C2RustUnnamed_14::SHUT_RDWR => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_14 {
        match value {
            0 => C2RustUnnamed_14::SHUT_RD,
            1 => C2RustUnnamed_14::SHUT_WR,
            2 => C2RustUnnamed_14::SHUT_RDWR,
            _ => panic!("Invalid value for C2RustUnnamed_14: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_14 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_14 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_14 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_14 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_14 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_14 {
    type Output = C2RustUnnamed_14;
    fn add(self, rhs: u32) -> C2RustUnnamed_14 {
        C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_14 {
    type Output = C2RustUnnamed_14;
    fn sub(self, rhs: u32) -> C2RustUnnamed_14 {
        C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_14 {
    type Output = C2RustUnnamed_14;
    fn mul(self, rhs: u32) -> C2RustUnnamed_14 {
        C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_14 {
    type Output = C2RustUnnamed_14;
    fn div(self, rhs: u32) -> C2RustUnnamed_14 {
        C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_14 {
    type Output = C2RustUnnamed_14;
    fn rem(self, rhs: u32) -> C2RustUnnamed_14 {
        C2RustUnnamed_14::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_info {
    pub envname: *const i8,
    pub dfltp: *mut *const i8,
    pub awkpath: *mut *const i8,
    pub max_pathlen: i32,
}
pub type C2RustUnnamed_13 = u32;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_13 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_13 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_13 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_13 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_13 = 65536;
pub const MSG_MORE: C2RustUnnamed_13 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_13 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_13 = 8192;
pub const MSG_RST: C2RustUnnamed_13 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_13 = 2048;
pub const MSG_SYN: C2RustUnnamed_13 = 1024;
pub const MSG_FIN: C2RustUnnamed_13 = 512;
pub const MSG_WAITALL: C2RustUnnamed_13 = 256;
pub const MSG_EOR: C2RustUnnamed_13 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_13 = 64;
pub const MSG_TRUNC: C2RustUnnamed_13 = 32;
pub const MSG_PROXY: C2RustUnnamed_13 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_13 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_13 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_13 = 4;
pub const MSG_OOB: C2RustUnnamed_13 = 1;
#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const i8,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as i32 as i64 {
        return;
    }
    r_unref(r);
}
#[inline]
unsafe extern "C" fn force_string_fmt(
    mut s: *mut NODE,
    mut fmtstr: *const i8,
    mut fmtidx: i32,
) -> *mut NODE {
    if (*s).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
        (*s).type_0 = nodevals::Node_val;
        (*s).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*s).flags as u32 & !(flagvals::NUMBER as i32) as u32);
        return s;
    }
    if (*s).flags as u32 & flagvals::STRCUR as i32 as u32 != 0 as i32 as u32
        && ((*s).sub.val.idx == -(1 as i32) || (*s).sub.val.idx == fmtidx)
    {
        return s;
    }
    return format_val.expect("non-null function pointer")(fmtstr, fmtidx, s);
}
#[inline]
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32 {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    if !r.is_null()
        && {
            (*r).valref -= 1;
            (*r).valref <= 0 as i32 as i64
        }
    {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn fixtype(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & (flagvals::NUMCUR as i32 | flagvals::USER_INPUT as i32) as u32
        == flagvals::USER_INPUT as i32 as u32
    {
        return force_number(n);
    }
    if (*n).flags as u32 & flagvals::INTIND as i32 as u32 != 0 as i32 as u32 {
        return force_string_fmt(n, CONVFMT, CONVFMTidx);
    }
    return n;
}
#[inline]
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut libc::c_void,
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2088 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: erealloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_realloc(ptr, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2092 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot reallocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2052 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: emalloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_malloc(count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2056 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn ezalloc_real(
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2070 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: ezalloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_calloc(1 as i32 as size_t, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2074 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh0 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh0).rptr;
    if (*t).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 1881 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            array_vname(t),
        );
    } else if (*t).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
        t = elem_new_to_scalar(t);
    }
    return t;
}
#[inline]
unsafe extern "C" fn boolval(mut t: *mut NODE) -> bool {
    if (*t).type_0 as u32 == nodevals::Node_var as i32 as u32 {
        t = (*t).sub.nodep.l.lptr;
    }
    fixtype(t);
    if (*t).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32 {
        return !((*t).sub.val.fltnum == 0.0f64);
    }
    return (*t).sub.val.slen > 0 as i32 as u64;
}
static mut matchrec: Option<
    unsafe extern "C" fn(*mut IOBUF, *mut recmatch, *mut SCANSTATE) -> RECVALUE,
> = unsafe {
    Some(
        rs1scan
            as unsafe extern "C" fn(
                *mut IOBUF,
                *mut recmatch,
                *mut SCANSTATE,
            ) -> RECVALUE,
    )
};
static mut read_can_timeout: bool = 0 as i32 != 0;
static mut read_timeout: i64 = 0;
static mut read_default_timeout: i64 = 0;
static mut red_head: *mut redirect = 0 as *const redirect as *mut redirect;
static mut RS: *mut NODE = 0 as *const NODE as *mut NODE;
static mut RS_re: [*mut Regexp; 2] = [0 as *const Regexp as *mut Regexp; 2];
static mut RS_regexp: *mut Regexp = 0 as *const Regexp as *mut Regexp;
static mut nonfatal: [i8; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &[i8; 9]>(b"NONFATAL\0")
};
#[no_mangle]
pub static mut RS_is_null: bool = false;
#[no_mangle]
pub unsafe extern "C" fn init_io() {
    let mut tmout: i64 = 0;
    init_sockets();
    tmout = getenv_long(b"GAWK_READ_TIMEOUT\0" as *const u8 as *const i8);
    if tmout > 0 as i32 as i64 {
        read_default_timeout = tmout;
        read_can_timeout = 1 as i32 != 0;
    }
    if !PROCINFO_node.is_null() {
        read_can_timeout = 1 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn after_beginfile(mut curfile: *mut *mut IOBUF) {
    let mut iop: *mut IOBUF = 0 as *mut IOBUF;
    iop = *curfile;
    find_input_parser(iop);
    if !(*iop).valid {
        let mut fname: *const i8 = 0 as *const i8;
        let mut errcode: i32 = 0;
        let mut valid: bool = false;
        fname = (*iop).public.name;
        errcode = (*iop).errcode;
        valid = (*iop).valid;
        *__errno_location() = 0 as i32;
        update_ERRNO_int(errcode);
        iop_close(iop);
        *curfile = 0 as *mut IOBUF;
        if !valid && errcode == 21 as i32
            && do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 406 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"command line argument `%s' is a directory: skipped\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                fname,
            );
            return;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 409 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"cannot open file `%s' for reading: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
            strerror(errcode),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn nextfile(
    mut curfile: *mut *mut IOBUF,
    mut skipping: bool,
) -> i32 {
    static mut i: i64 = 1 as i32 as i64;
    static mut files: bool = 0 as i32 != 0;
    let mut arg: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut fname: *const i8 = 0 as *const i8;
    let mut fd: i32 = -(1 as i32);
    let mut errcode: i32 = 0 as i32;
    let mut iop: *mut IOBUF = *curfile;
    if skipping {
        errcode = 0 as i32;
        if !iop.is_null() {
            errcode = (*iop).errcode;
            iop_close(iop);
        }
        *curfile = 0 as *mut IOBUF;
        return (errcode == 0 as i32) as i32;
    }
    if !iop.is_null() {
        if (*iop).flag as u32 & iobuf_flags::IOP_AT_EOF as i32 as u32 != 0 as i32 as u32
        {
            iop_close(iop);
            *curfile = 0 as *mut IOBUF;
            return 1 as i32;
        } else {
            return 0 as i32
        }
    }
    while i < (*(*ARGC_node).sub.nodep.l.lptr).sub.val.fltnum as i64 {
        tmp = make_number.expect("non-null function pointer")(i as libc::c_double);
        force_string_fmt(tmp, CONVFMT, CONVFMTidx);
        arg = in_array(ARGV_node, tmp);
        unref(tmp);
        if !(arg.is_null() || (*arg).sub.val.slen == 0 as i32 as u64) {
            arg = force_string_fmt(arg, CONVFMT, CONVFMTidx);
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
                unref((*ARGIND_node).sub.nodep.l.lptr);
                (*ARGIND_node).sub.nodep.l.lptr = make_number
                    .expect("non-null function pointer")(i as libc::c_double);
            }
            if arg_assign((*arg).sub.val.sp, 0 as i32 != 0) == 0 {
                files = 1 as i32 != 0;
                fname = (*arg).sub.val.sp;
                unref((*FILENAME_node).sub.nodep.l.lptr);
                (*FILENAME_node).sub.nodep.l.lptr = dupnode(arg);
                FNR = 0 as i32 as i64;
                *__errno_location() = 0 as i32;
                fd = devopen(fname, b"r\0" as *const u8 as *const i8);
                if fd == -(1 as i32) && *__errno_location() == 24 as i32 {
                    close_one();
                    close_one();
                    fd = devopen(fname, b"r\0" as *const u8 as *const i8);
                }
                errcode = *__errno_location();
                if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
                    update_ERRNO_int(*__errno_location());
                }
                iop = iop_alloc(fd, fname, errcode);
                *curfile = iop_finish(iop);
                if (*iop).public.fd == -(1 as i32) {
                    (*iop).errcode = errcode;
                } else if (*iop).valid {
                    (*iop).errcode = 0 as i32;
                }
                if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                    && (*iop).errcode != 0 as i32
                {
                    update_ERRNO_int((*iop).errcode);
                }
                i += 1;
                return i as i32;
            }
        }
        i += 1;
        i;
    }
    if files as i32 == 0 as i32 {
        files = 1 as i32 != 0;
        *__errno_location() = 0 as i32;
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
            update_ERRNO_int(*__errno_location());
        }
        unref((*FILENAME_node).sub.nodep.l.lptr);
        (*FILENAME_node).sub.nodep.l.lptr = make_str_node(
            b"-\0" as *const u8 as *const i8,
            1 as i32 as size_t,
            0 as i32,
        );
        (*(*FILENAME_node).sub.nodep.l.lptr).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >(
            (*(*FILENAME_node).sub.nodep.l.lptr).flags as u32
                | flagvals::USER_INPUT as i32 as u32,
        );
        fname = b"-\0" as *const u8 as *const i8;
        iop = iop_alloc(fileno(stdin), fname, 0 as i32);
        *curfile = iop_finish(iop);
        if (*iop).public.fd == -(1 as i32) {
            errcode = *__errno_location();
            *__errno_location() = 0 as i32;
            update_ERRNO_int(*__errno_location());
            iop_close(iop);
            *curfile = 0 as *mut IOBUF;
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 523 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"cannot open file `%s' for reading: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                fname,
                strerror(errcode),
            );
        }
        i += 1;
        return i as i32;
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn set_FNR() {
    let mut n: *mut NODE = (*FNR_node).sub.nodep.l.lptr;
    force_number(n);
    FNR = (*n).sub.val.fltnum as i64;
}
#[no_mangle]
pub unsafe extern "C" fn set_NR() {
    let mut n: *mut NODE = (*NR_node).sub.nodep.l.lptr;
    force_number(n);
    NR = (*n).sub.val.fltnum as i64;
}
#[no_mangle]
pub unsafe extern "C" fn inrec(mut iop: *mut IOBUF, mut errcode: *mut i32) -> bool {
    let mut begin: *mut i8 = 0 as *mut i8;
    let mut cnt: size_t = 0;
    let mut retval: bool = false;
    let mut field_width: *const awk_fieldwidth_info_t = 0
        as *const awk_fieldwidth_info_t;
    if (*iop).flag as u32 & iobuf_flags::IOP_AT_EOF as i32 as u32 != 0 as i32 as u32
        && (*iop).off >= (*iop).dataend
    {
        retval = 0 as i32 != 0;
    } else if (*iop).flag as u32 & iobuf_flags::IOP_CLOSED as i32 as u32
        != 0 as i32 as u32
    {
        retval = 0 as i32 != 0;
    } else {
        retval = get_a_record(&mut begin, &mut cnt, iop, errcode, &mut field_width)
            == 0 as i32;
    }
    if retval {
        NR += 1;
        NR;
        FNR += 1;
        FNR;
        set_record(begin, cnt, field_width);
        if *errcode > 0 as i32 {
            retval = 0 as i32 != 0;
        }
    }
    return retval;
}
unsafe extern "C" fn remap_std_file(mut oldfd: i32) -> i32 {
    let mut newfd: i32 = 0;
    let mut ret: i32 = -(1 as i32);
    newfd = os_devopen(b"/dev/null\0" as *const u8 as *const i8, 0o2 as i32);
    if newfd == -(1 as i32) {
        newfd = open(b"/dev/null\0" as *const u8 as *const i8, 0o2 as i32);
    }
    if newfd >= 0 as i32 {
        ret = dup2(newfd, oldfd);
        close(newfd);
    } else {
        ret = 0 as i32;
    }
    return ret;
}
unsafe extern "C" fn iop_close(mut iop: *mut IOBUF) -> i32 {
    let mut ret: i32 = 0 as i32;
    if iop.is_null() {
        return 0 as i32;
    }
    *__errno_location() = 0 as i32;
    (*iop).flag = ::core::mem::transmute::<
        u32,
        iobuf_flags,
    >((*iop).flag as u32 & !(iobuf_flags::IOP_AT_EOF as i32) as u32);
    (*iop).flag = ::core::mem::transmute::<
        u32,
        iobuf_flags,
    >((*iop).flag as u32 | iobuf_flags::IOP_CLOSED as i32 as u32);
    (*iop).dataend = 0 as *mut i8;
    if ((*iop).public.close_func).is_some() {
        ((*iop).public.close_func)
            .expect("non-null function pointer")(&mut (*iop).public);
    }
    if (*iop).public.fd != -(1 as i32) {
        if (*iop).public.fd == fileno(stdin) || (*iop).public.fd == fileno(stdout)
            || (*iop).public.fd == fileno(stderr)
        {
            ret = remap_std_file((*iop).public.fd);
        } else {
            ret = close((*iop).public.fd);
        }
    }
    if ret == -(1 as i32) {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 650 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"close of fd %d (`%s') failed: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*iop).public.fd,
            (*iop).public.name,
            strerror(*__errno_location()),
        );
    }
    if !((*iop).buf).is_null() {
        if (**fields_arr.offset(0 as i32 as isize)).sub.val.sp >= (*iop).buf
            && (**fields_arr.offset(0 as i32 as isize)).sub.val.sp
                < ((*iop).buf).offset((*iop).size as isize)
        {
            let mut t: *mut NODE = 0 as *mut NODE;
            t = make_str_node(
                (**fields_arr.offset(0 as i32 as isize)).sub.val.sp,
                (**fields_arr.offset(0 as i32 as isize)).sub.val.slen,
                0 as i32,
            );
            unref(*fields_arr.offset(0 as i32 as isize));
            let ref mut fresh1 = *fields_arr.offset(0 as i32 as isize);
            *fresh1 = t;
        }
        pma_free((*iop).buf as *mut libc::c_void);
        (*iop).buf = 0 as *mut i8;
    }
    pma_free(iop as *mut libc::c_void);
    return if ret == -(1 as i32) { 1 as i32 } else { 0 as i32 };
}
#[no_mangle]
pub unsafe extern "C" fn redflags2str(mut flags: i32) -> *const i8 {
    static mut redtab: [flagtab; 12] = [
        {
            let mut init = flagtab {
                val: redirect_flags::RED_FILE as i32,
                name: b"redirect_flags::RED_FILE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_PIPE as i32,
                name: b"redirect_flags::RED_PIPE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_READ as i32,
                name: b"redirect_flags::RED_READ\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_WRITE as i32,
                name: b"redirect_flags::RED_WRITE\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_APPEND as i32,
                name: b"redirect_flags::RED_APPEND\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_FLUSH as i32,
                name: b"redirect_flags::RED_FLUSH\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_EOF as i32,
                name: b"redirect_flags::RED_EOF\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_TWOWAY as i32,
                name: b"redirect_flags::RED_TWOWAY\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_PTY as i32,
                name: b"redirect_flags::RED_PTY\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_SOCKET as i32,
                name: b"redirect_flags::RED_SOCKET\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: redirect_flags::RED_TCP as i32,
                name: b"redirect_flags::RED_TCP\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: 0 as i32,
                name: 0 as *const i8,
            };
            init
        },
    ];
    if flags == redirect_flags::RED_NONE as i32 {
        return b"redirect_flags::RED_NONE\0" as *const u8 as *const i8;
    }
    return genflags2str(flags, redtab.as_ptr());
}
unsafe extern "C" fn check_duplicated_redirections(
    mut name: *const i8,
    mut len: size_t,
    mut oldflags: redirect_flags_t,
    mut newflags: redirect_flags_t,
) {
    static mut mixtures: [mixture; 11] = [
        {
            let mut init = mixture {
                common: redirect_flags::RED_FILE,
                mode: redirect_flags::RED_READ,
                other_mode: redirect_flags::RED_WRITE,
                message: b"`%.*s' used for input file and for output file\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_READ,
                mode: redirect_flags::RED_FILE,
                other_mode: redirect_flags::RED_PIPE,
                message: b"`%.*s' used for input file and input pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_READ,
                mode: redirect_flags::RED_FILE,
                other_mode: redirect_flags::RED_TWOWAY,
                message: b"`%.*s' used for input file and two-way pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_NONE,
                mode: (redirect_flags::RED_FILE as i32 | redirect_flags::RED_READ as i32)
                    as redirect_flags_t,
                other_mode: (redirect_flags::RED_PIPE as i32
                    | redirect_flags::RED_WRITE as i32) as redirect_flags_t,
                message: b"`%.*s' used for input file and output pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: (redirect_flags::RED_FILE as i32
                    | redirect_flags::RED_WRITE as i32) as redirect_flags_t,
                mode: (redirect_flags::RED_FILE as i32
                    | redirect_flags::RED_WRITE as i32) as redirect_flags_t,
                other_mode: redirect_flags::RED_APPEND,
                message: b"unnecessary mixing of `>' and `>>' for file `%.*s'\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_NONE,
                mode: (redirect_flags::RED_FILE as i32
                    | redirect_flags::RED_WRITE as i32) as redirect_flags_t,
                other_mode: (redirect_flags::RED_PIPE as i32
                    | redirect_flags::RED_READ as i32) as redirect_flags_t,
                message: b"`%.*s' used for input pipe and output file\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_WRITE,
                mode: redirect_flags::RED_FILE,
                other_mode: redirect_flags::RED_PIPE,
                message: b"`%.*s' used for output file and output pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_WRITE,
                mode: redirect_flags::RED_FILE,
                other_mode: redirect_flags::RED_TWOWAY,
                message: b"`%.*s' used for output file and two-way pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_PIPE,
                mode: redirect_flags::RED_READ,
                other_mode: redirect_flags::RED_WRITE,
                message: b"`%.*s' used for input pipe and output pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_READ,
                mode: redirect_flags::RED_PIPE,
                other_mode: redirect_flags::RED_TWOWAY,
                message: b"`%.*s' used for input pipe and two-way pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mixture {
                common: redirect_flags::RED_WRITE,
                mode: redirect_flags::RED_PIPE,
                other_mode: redirect_flags::RED_TWOWAY,
                message: b"`%.*s' used for output pipe and two-way pipe\0" as *const u8
                    as *const i8,
            };
            init
        },
    ];
    let mut i: i32 = 0 as i32;
    let mut j: i32 = (::core::mem::size_of::<[mixture; 11]>() as u64)
        .wrapping_div(::core::mem::size_of::<mixture>() as u64) as i32;
    oldflags = ::core::mem::transmute::<
        u32,
        redirect_flags_t,
    >(
        oldflags as u32
            & !(redirect_flags::RED_FLUSH as i32 | redirect_flags::RED_EOF as i32
                | redirect_flags::RED_PTY as i32) as u32,
    );
    newflags = ::core::mem::transmute::<
        u32,
        redirect_flags_t,
    >(
        newflags as u32
            & !(redirect_flags::RED_FLUSH as i32 | redirect_flags::RED_EOF as i32
                | redirect_flags::RED_PTY as i32) as u32,
    );
    i = 0 as i32;
    while i < j {
        let mut both_have_common: bool = oldflags as u32
            & mixtures[i as usize].common as u32 == mixtures[i as usize].common as u32
            && newflags as u32 & mixtures[i as usize].common as u32
                == mixtures[i as usize].common as u32;
        let mut old_has_mode: bool = oldflags as u32 & mixtures[i as usize].mode as u32
            == mixtures[i as usize].mode as u32;
        let mut new_has_mode: bool = newflags as u32 & mixtures[i as usize].mode as u32
            == mixtures[i as usize].mode as u32;
        let mut old_has_other_mode: bool = oldflags as u32
            & mixtures[i as usize].other_mode as u32
            == mixtures[i as usize].other_mode as u32;
        let mut new_has_other_mode: bool = newflags as u32
            & mixtures[i as usize].other_mode as u32
            == mixtures[i as usize].other_mode as u32;
        if both_have_common as i32 != 0 && oldflags as u32 != newflags as u32
            && ((old_has_mode as i32 != 0 || new_has_mode as i32 != 0)
                && (old_has_other_mode as i32 != 0 || new_has_other_mode as i32 != 0))
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 763 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(0 as *const i8, mixtures[i as usize].message, 5 as i32),
                len,
                name,
            );
            return;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn redirect_string(
    mut str: *const i8,
    mut explen: size_t,
    mut not_string: bool,
    mut redirtype: i32,
    mut errflg: *mut i32,
    mut extfd: i32,
    mut failure_fatal: bool,
) -> *mut redirect {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut tflag: redirect_flags_t = redirect_flags::RED_NONE;
    let mut outflag: redirect_flags_t = redirect_flags::RED_NONE;
    let mut direction: *const i8 = b"to\0" as *const u8 as *const i8;
    let mut mode: *const i8 = 0 as *const i8;
    let mut fd: i32 = 0;
    let mut what: *const i8 = 0 as *const i8;
    let mut new_rp: bool = 0 as i32 != 0;
    let mut isi: inet_socket_info = inet_socket_info {
        family: 0,
        protocol: 0,
        localport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remotehost: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remoteport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
    };
    static mut save_rp: *mut redirect = 0 as *const redirect as *mut redirect;
    if do_flags as u32 & do_flag_values::DO_SANDBOX as i32 as u32 != 0 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 791 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"redirection not allowed in sandbox mode\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    let mut current_block_16: u64;
    match redirtype {
        redirval::redirect_append => {}
        1 => {
            current_block_16 = 8415834763729869163;
        }
        3 => {
            tflag = (redirect_flags::RED_PIPE as i32 | redirect_flags::RED_WRITE as i32)
                as redirect_flags_t;
            what = b"|\0" as *const u8 as *const i8;
            current_block_16 = 224731115979188411;
        }
        4 => {
            tflag = (redirect_flags::RED_PIPE as i32 | redirect_flags::RED_READ as i32)
                as redirect_flags_t;
            what = b"|\0" as *const u8 as *const i8;
            current_block_16 = 224731115979188411;
        }
        5 => {
            tflag = (redirect_flags::RED_FILE as i32 | redirect_flags::RED_READ as i32)
                as redirect_flags_t;
            what = b"<\0" as *const u8 as *const i8;
            current_block_16 = 224731115979188411;
        }
        6 => {
            tflag = (redirect_flags::RED_READ as i32 | redirect_flags::RED_WRITE as i32
                | redirect_flags::RED_TWOWAY as i32) as redirect_flags_t;
            what = b"|&\0" as *const u8 as *const i8;
            current_block_16 = 224731115979188411;
        }
        _ => {
            r_fatal(
                b"internal error: file %s, line %d: invalid redirection type %d\0"
                    as *const u8 as *const i8,
                b"io.c\0" as *const u8 as *const i8,
                822 as i32,
                redirtype,
            );
            current_block_16 = 224731115979188411;
        }
    }
    match current_block_16 {
        8415834763729869163 => {
            outflag = (redirect_flags::RED_FILE as i32
                | redirect_flags::RED_WRITE as i32) as redirect_flags_t;
            tflag = ::core::mem::transmute::<
                u32,
                redirect_flags_t,
            >(tflag as u32 | outflag as u32);
            if redirtype == redirval::redirect_output as i32 {
                what = b">\0" as *const u8 as *const i8;
            } else {
                what = b">>\0" as *const u8 as *const i8;
            }
        }
        _ => {}
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && not_string as i32 != 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 825 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"expression in `%s' redirection is a number\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            what,
        );
    }
    if explen < 1 as i32 as u64 || str.is_null() || *str as i32 == '\0' as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 829 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"expression for `%s' redirection has null string value\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            what,
        );
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (strncmp(str, b"0\0" as *const u8 as *const i8, explen) == 0 as i32
            || strncmp(str, b"1\0" as *const u8 as *const i8, explen) == 0 as i32)
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 834 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"filename `%.*s' for `%s' redirection may be result of logical expression\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            explen as i32,
            str,
            what,
        );
    }
    if inetfile(str, explen, &mut isi) {
        tflag = ::core::mem::transmute::<
            u32,
            redirect_flags_t,
        >(tflag as u32 | redirect_flags::RED_SOCKET as i32 as u32);
        if isi.protocol == __socket_type::SOCK_STREAM as i32 {
            tflag = ::core::mem::transmute::<
                u32,
                redirect_flags_t,
            >(tflag as u32 | redirect_flags::RED_TCP as i32 as u32);
        }
    }
    rp = red_head;
    while !rp.is_null() {
        if (*rp).flag as u32 & redirect_flags::RED_EOF as i32 as u32 != 0 as i32 as u32
            && redirtype == redirval::redirect_pipein as i32
        {
            if (*rp).pid != -(1 as i32) {
                wait_any(0 as i32);
            }
        }
        if strlen((*rp).value) == explen
            && memcmp(
                (*rp).value as *const libc::c_void,
                str as *const libc::c_void,
                explen,
            ) == 0 as i32
        {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            {
                check_duplicated_redirections((*rp).value, explen, (*rp).flag, tflag);
            }
            if (*rp).flag as u32
                & !(redirect_flags::RED_FLUSH as i32 | redirect_flags::RED_EOF as i32
                    | redirect_flags::RED_PTY as i32) as u32 == tflag as u32
                || outflag as u32 != 0 as i32 as u32
                    && (*rp).flag as u32
                        & (redirect_flags::RED_FILE as i32
                            | redirect_flags::RED_WRITE as i32) as u32 == outflag as u32
            {
                break;
            }
        }
        rp = (*rp).next;
    }
    if rp.is_null() {
        let mut newstr: *mut i8 = 0 as *mut i8;
        new_rp = 1 as i32 != 0;
        if !save_rp.is_null() {
            rp = save_rp;
            pma_free((*rp).value as *mut libc::c_void);
        } else {
            rp = emalloc_real(
                ::core::mem::size_of::<redirect>() as u64,
                b"redirect\0" as *const u8 as *const i8,
                b"rp\0" as *const u8 as *const i8,
                b"io.c\0" as *const u8 as *const i8,
                893 as i32,
            ) as *mut redirect;
        }
        newstr = emalloc_real(
            explen.wrapping_add(1 as i32 as u64),
            b"redirect\0" as *const u8 as *const i8,
            b"newstr\0" as *const u8 as *const i8,
            b"io.c\0" as *const u8 as *const i8,
            894 as i32,
        ) as *mut i8;
        memcpy(newstr as *mut libc::c_void, str as *const libc::c_void, explen);
        *newstr.offset(explen as isize) = '\0' as i32 as i8;
        str = newstr;
        (*rp).value = newstr;
        (*rp).flag = tflag;
        init_output_wrapper(&mut (*rp).output);
        (*rp).output.name = str;
        (*rp).iop = 0 as *mut IOBUF;
        (*rp).pid = -(1 as i32);
        (*rp).status = 0 as i32;
    } else {
        str = (*rp).value;
    }
    save_rp = rp;
    while ((*rp).output.fp).is_null() && ((*rp).iop).is_null() {
        if !new_rp
            && (*rp).flag as u32 & redirect_flags::RED_EOF as i32 as u32
                != 0 as i32 as u32
        {
            save_rp = 0 as *mut redirect;
            return rp;
        }
        mode = 0 as *const i8;
        *__errno_location() = 0 as i32;
        match redirtype {
            1 => {
                mode = b"w\0" as *const u8 as *const i8;
                if (*rp).flag as u32 & redirect_flags::RED_USED as i32 as u32
                    != 0 as i32 as u32
                {
                    mode = if *((*rp).mode).offset(1 as i32 as isize) as i32
                        == 'b' as i32
                    {
                        b"ab\0" as *const u8 as *const i8
                    } else {
                        b"a\0" as *const u8 as *const i8
                    };
                }
            }
            2 => {
                mode = b"a\0" as *const u8 as *const i8;
            }
            3 => {
                if extfd >= 0 as i32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 931 as i32);
                    (Some(
                        (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"get_file cannot create pipe `%s' with fd %d\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        str,
                        extfd,
                    );
                    return 0 as *mut redirect;
                }
                flush_io();
                os_restore_mode(fileno(stdin));
                signal(13 as i32, None);
                (*rp).output.fp = popen(str, b"w\0" as *const u8 as *const i8);
                if ((*rp).output.fp).is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 946 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot open pipe `%s' for output: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        str,
                        strerror(*__errno_location()),
                    );
                }
                signal(
                    13 as i32,
                    ::core::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as i32 as libc::intptr_t),
                );
                os_close_on_exec(
                    fileno((*rp).output.fp),
                    str,
                    b"pipe\0" as *const u8 as *const i8,
                    b"to\0" as *const u8 as *const i8,
                );
                (*rp).flag = ::core::mem::transmute::<
                    u32,
                    redirect_flags,
                >((*rp).flag as u32 | redirect_flags::RED_FLUSH as i32 as u32);
            }
            4 => {
                if extfd >= 0 as i32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 956 as i32);
                    (Some(
                        (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"get_file cannot create pipe `%s' with fd %d\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        str,
                        extfd,
                    );
                    return 0 as *mut redirect;
                }
                direction = b"from\0" as *const u8 as *const i8;
                if (gawk_popen(str, rp)).is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 961 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot open pipe `%s' for input: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        str,
                        strerror(*__errno_location()),
                    );
                }
            }
            5 => {
                direction = b"from\0" as *const u8 as *const i8;
                fd = if extfd >= 0 as i32 {
                    extfd
                } else {
                    devopen(str, b"r\0" as *const u8 as *const i8)
                };
                if fd == -(1 as i32) && *__errno_location() == 21 as i32 {
                    *errflg = 21 as i32;
                    return 0 as *mut redirect;
                }
                (*rp).iop = iop_alloc(fd, str, *__errno_location());
                find_input_parser((*rp).iop);
                iop_finish((*rp).iop);
                if !(*(*rp).iop).valid {
                    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                        == 0 && (*(*rp).iop).errcode != 0 as i32
                    {
                        update_ERRNO_int((*(*rp).iop).errcode);
                    }
                    iop_close((*rp).iop);
                    (*rp).iop = 0 as *mut IOBUF;
                }
            }
            6 => {
                direction = b"to/from\0" as *const u8 as *const i8;
                if two_way_open(str, rp, extfd) == 0 {
                    if !failure_fatal || is_non_fatal_redirect(str, explen) as i32 != 0 {
                        *errflg = *__errno_location();
                        return 0 as *mut redirect;
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(b"io.c\0" as *const u8 as *const i8, 996 as i32);
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"cannot open two way pipe `%s' for input/output: %s\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            str,
                            strerror(*__errno_location()),
                        );
                    }
                }
            }
            _ => {
                r_fatal(
                    b"internal error: file %s, line %d: invalid redirection type %d\0"
                        as *const u8 as *const i8,
                    b"io.c\0" as *const u8 as *const i8,
                    1001 as i32,
                    redirtype,
                );
            }
        }
        if !mode.is_null() {
            *__errno_location() = 0 as i32;
            (*rp).output.mode = mode;
            fd = if extfd >= 0 as i32 { extfd } else { devopen(str, mode) };
            if fd > -(1 as i32) {
                if fd == fileno(stdin) {
                    (*rp).output.fp = stdin;
                } else if fd == fileno(stdout) {
                    (*rp).output.fp = stdout;
                } else if fd == fileno(stderr) {
                    (*rp).output.fp = stderr;
                } else {
                    let mut omode: *const i8 = mode;
                    let mut fd_flags: i32 = 0;
                    fd_flags = fcntl(fd, 3 as i32);
                    if fd_flags != -(1 as i32)
                        && fd_flags & 0o2000 as i32 == 0o2000 as i32
                    {
                        omode = b"a\0" as *const u8 as *const i8;
                    }
                    os_close_on_exec(
                        fd,
                        str,
                        b"file\0" as *const u8 as *const i8,
                        b"\0" as *const u8 as *const i8,
                    );
                    (*rp).output.fp = fdopen(fd, omode);
                    (*rp).mode = mode;
                    if ((*rp).output.fp).is_null() {
                        close(fd);
                    }
                }
                if !((*rp).output.fp).is_null() && os_isatty(fd) != 0 {
                    (*rp).flag = ::core::mem::transmute::<
                        u32,
                        redirect_flags,
                    >((*rp).flag as u32 | redirect_flags::RED_FLUSH as i32 as u32);
                }
                if !new_rp && red_head != rp {
                    (*(*rp).prev).next = (*rp).next;
                    if !((*(*rp).prev).next).is_null() {
                        (*(*rp).next).prev = (*rp).prev;
                    }
                    (*red_head).prev = rp;
                    (*rp).prev = 0 as *mut redirect;
                    (*rp).next = red_head;
                    red_head = rp;
                }
            }
            find_output_wrapper(&mut (*rp).output);
        }
        if ((*rp).output.fp).is_null() && ((*rp).iop).is_null() {
            if *__errno_location() == 24 as i32 || *__errno_location() == 23 as i32 {
                close_one();
            } else {
                if !errflg.is_null() {
                    *errflg = *__errno_location();
                }
                if failure_fatal as i32 != 0 && !is_non_fatal_redirect(str, explen)
                    && (redirtype == redirval::redirect_output as i32
                        || redirtype == redirval::redirect_append as i32)
                {
                    if *direction as i32 == 'f' as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(b"io.c\0" as *const u8 as *const i8, 1083 as i32);
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"cannot redirect from `%s': %s\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            str,
                            strerror(*__errno_location()),
                        );
                    } else {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(b"io.c\0" as *const u8 as *const i8, 1086 as i32);
                        (Some(
                            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"cannot redirect to `%s': %s\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            str,
                            strerror(*__errno_location()),
                        );
                    }
                } else {
                    return 0 as *mut redirect
                }
            }
        }
    }
    if new_rp {
        if !red_head.is_null() {
            (*red_head).prev = rp;
        }
        (*rp).prev = 0 as *mut redirect;
        (*rp).next = red_head;
        red_head = rp;
    }
    save_rp = 0 as *mut redirect;
    return rp;
}
#[no_mangle]
pub unsafe extern "C" fn redirect(
    mut redir_exp: *mut NODE,
    mut redirtype: i32,
    mut errflg: *mut i32,
    mut failure_fatal: bool,
) -> *mut redirect {
    let mut not_string: bool = (*fixtype(redir_exp)).flags as u32
        & flagvals::STRING as i32 as u32 == 0 as i32 as u32;
    redir_exp = force_string_fmt(redir_exp, CONVFMT, CONVFMTidx);
    return redirect_string(
        (*redir_exp).sub.val.sp,
        (*redir_exp).sub.val.slen,
        not_string,
        redirtype,
        errflg,
        -(1 as i32),
        failure_fatal,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getredirect(mut str: *const i8, mut len: i32) -> *mut redirect {
    let mut rp: *mut redirect = 0 as *mut redirect;
    rp = red_head;
    while !rp.is_null() {
        if strlen((*rp).value) == len as u64
            && memcmp(
                (*rp).value as *const libc::c_void,
                str as *const libc::c_void,
                len as u64,
            ) == 0 as i32
        {
            return rp;
        }
        rp = (*rp).next;
    }
    return 0 as *mut redirect;
}
#[no_mangle]
pub unsafe extern "C" fn is_non_fatal_std(mut fp: *mut FILE) -> bool {
    if !(in_PROCINFO(nonfatal.as_ptr(), 0 as *const i8, 0 as *mut *mut NODE)).is_null() {
        return 1 as i32 != 0;
    }
    if fp == stdout {
        return !(in_PROCINFO(
            b"-\0" as *const u8 as *const i8,
            nonfatal.as_ptr(),
            0 as *mut *mut NODE,
        ))
            .is_null()
            || !(in_PROCINFO(
                b"/dev/stdout\0" as *const u8 as *const i8,
                nonfatal.as_ptr(),
                0 as *mut *mut NODE,
            ))
                .is_null()
    } else if fp == stderr {
        return !(in_PROCINFO(
            b"/dev/stderr\0" as *const u8 as *const i8,
            nonfatal.as_ptr(),
            0 as *mut *mut NODE,
        ))
            .is_null()
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_non_fatal_redirect(
    mut str: *const i8,
    mut len: size_t,
) -> bool {
    let mut ret: bool = false;
    let mut save: i8 = 0;
    let mut s: *mut i8 = str as *mut i8;
    save = *s.offset(len as isize);
    *s.offset(len as isize) = '\0' as i32 as i8;
    ret = !(in_PROCINFO(nonfatal.as_ptr(), 0 as *const i8, 0 as *mut *mut NODE))
        .is_null()
        || !(in_PROCINFO(s, nonfatal.as_ptr(), 0 as *mut *mut NODE)).is_null();
    *s.offset(len as isize) = save;
    return ret;
}
unsafe extern "C" fn close_one() {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut rplast: *mut redirect = 0 as *mut redirect;
    static mut warned: bool = 0 as i32 != 0;
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && !warned
    {
        warned = 1 as i32 != 0;
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 1188 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"reached system limit for open files: starting to multiplex file descriptors\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    rp = red_head;
    while !rp.is_null() {
        rplast = rp;
        rp = (*rp).next;
    }
    rp = rplast;
    while !rp.is_null() {
        if !(((*rp).output.fp).is_null() || (*rp).output.fp == stderr
            || (*rp).output.fp == stdout)
        {
            if (*rp).flag as u32
                & (redirect_flags::RED_FILE as i32 | redirect_flags::RED_WRITE as i32)
                    as u32
                == (redirect_flags::RED_FILE as i32 | redirect_flags::RED_WRITE as i32)
                    as u32
            {
                (*rp).flag = ::core::mem::transmute::<
                    u32,
                    redirect_flags,
                >((*rp).flag as u32 | redirect_flags::RED_USED as i32 as u32);
                *__errno_location() = 0 as i32;
                if ((*rp).output.gawk_fclose)
                    .expect(
                        "non-null function pointer",
                    )((*rp).output.fp, (*rp).output.opaque) != 0 as i32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 1204 as i32);
                    (Some(
                        (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"close of `%s' failed: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                }
                (*rp).output.fp = 0 as *mut FILE;
                break;
            }
        }
        rp = (*rp).prev;
    }
    if rp.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 1212 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"too many pipes or input files open\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_close(mut nargs: i32) -> *mut NODE {
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut tmp2: *mut NODE = 0 as *mut NODE;
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut how: two_way_close_type = two_way_close_type::CLOSE_ALL;
    if nargs == 2 as i32 {
        let mut save: i8 = 0;
        tmp2 = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        save = *((*tmp2).sub.val.sp).offset((*tmp2).sub.val.slen as isize);
        *((*tmp2).sub.val.sp).offset((*tmp2).sub.val.slen as isize) = '\0' as i32 as i8;
        if strcasecmp((*tmp2).sub.val.sp, b"to\0" as *const u8 as *const i8) == 0 as i32
        {
            how = two_way_close_type::CLOSE_TO;
        } else if strcasecmp((*tmp2).sub.val.sp, b"from\0" as *const u8 as *const i8)
            == 0 as i32
        {
            how = two_way_close_type::CLOSE_FROM;
        } else {
            DEREF(tmp2);
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1238 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close: second argument must be `to' or `from'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        *((*tmp2).sub.val.sp).offset((*tmp2).sub.val.slen as isize) = save;
        DEREF(tmp2);
    }
    tmp = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    rp = red_head;
    while !rp.is_null() {
        if strlen((*rp).value) == (*tmp).sub.val.slen
            && memcmp(
                (*rp).value as *const libc::c_void,
                (*tmp).sub.val.sp as *const libc::c_void,
                (*tmp).sub.val.slen,
            ) == 0 as i32
        {
            break;
        }
        rp = (*rp).next;
    }
    if rp.is_null() {
        let mut cp: *mut i8 = 0 as *mut i8;
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1256 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close: `%.*s' is not an open file, pipe or co-process\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*tmp).sub.val.slen as i32,
                (*tmp).sub.val.sp,
            );
        }
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
            cp = dcgettext(
                0 as *const i8,
                b"close of redirection that was never opened\0" as *const u8
                    as *const i8,
                5 as i32,
            );
            update_ERRNO_string(cp);
        }
        DEREF(tmp);
        return make_number.expect("non-null function pointer")(-1.0f64);
    }
    DEREF(tmp);
    fflush(stdout);
    tmp = make_number
        .expect(
            "non-null function pointer",
        )(close_redir(rp, 0 as i32 != 0, how) as libc::c_double);
    rp = 0 as *mut redirect;
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
        unref(tmp);
        tmp = make_number
            .expect("non-null function pointer")(0 as i32 as libc::c_double);
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn close_rp(
    mut rp: *mut redirect,
    mut how: two_way_close_type,
) -> i32 {
    let mut status: i32 = 0 as i32;
    *__errno_location() = 0 as i32;
    if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32 != 0 as i32 as u32 {
        if (how as u32 == two_way_close_type::CLOSE_ALL as i32 as u32
            || how as u32 == two_way_close_type::CLOSE_TO as i32 as u32)
            && !((*rp).output.fp).is_null()
        {
            if (*rp).flag as u32 & redirect_flags::RED_TCP as i32 as u32
                != 0 as i32 as u32
            {
                shutdown(fileno((*rp).output.fp), C2RustUnnamed_14::SHUT_WR as i32);
            }
            if (*rp).flag as u32 & redirect_flags::RED_PTY as i32 as u32
                != 0 as i32 as u32
            {
                ((*rp).output.gawk_fwrite)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"\x04\n\0" as *const u8 as *const i8 as *const libc::c_void,
                    (::core::mem::size_of::<[i8; 3]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                    1 as i32 as size_t,
                    (*rp).output.fp,
                    (*rp).output.opaque,
                );
                ((*rp).output.gawk_fflush)
                    .expect(
                        "non-null function pointer",
                    )((*rp).output.fp, (*rp).output.opaque);
            }
            status = ((*rp).output.gawk_fclose)
                .expect(
                    "non-null function pointer",
                )((*rp).output.fp, (*rp).output.opaque);
            (*rp).output.fp = 0 as *mut FILE;
        }
        if how as u32 == two_way_close_type::CLOSE_ALL as i32 as u32
            || how as u32 == two_way_close_type::CLOSE_FROM as i32 as u32
        {
            if (*rp).flag as u32 & redirect_flags::RED_SOCKET as i32 as u32
                != 0 as i32 as u32 && !((*rp).iop).is_null()
            {
                if (*rp).flag as u32 & redirect_flags::RED_TCP as i32 as u32
                    != 0 as i32 as u32
                {
                    shutdown((*(*rp).iop).public.fd, C2RustUnnamed_14::SHUT_RD as i32);
                }
                iop_close((*rp).iop);
            } else {
                status = gawk_pclose(rp);
            }
            (*rp).iop = 0 as *mut IOBUF;
        }
    } else if (*rp).flag as u32
        & (redirect_flags::RED_PIPE as i32 | redirect_flags::RED_WRITE as i32) as u32
        == (redirect_flags::RED_PIPE as i32 | redirect_flags::RED_WRITE as i32) as u32
    {
        status = sanitize_exit_status(pclose((*rp).output.fp));
        if BINMODE & binmode_values::BINMODE_INPUT as i32 != 0 as i32 {
            os_setbinmode(fileno(stdin), 0 as i32);
        }
        (*rp).output.fp = 0 as *mut FILE;
    } else if !((*rp).output.fp).is_null() {
        status = ((*rp).output.gawk_fclose)
            .expect("non-null function pointer")((*rp).output.fp, (*rp).output.opaque);
        (*rp).output.fp = 0 as *mut FILE;
    } else if !((*rp).iop).is_null() {
        if (*rp).flag as u32 & redirect_flags::RED_PIPE as i32 as u32 != 0 as i32 as u32
        {
            status = gawk_pclose(rp);
        } else {
            status = iop_close((*rp).iop);
            (*rp).iop = 0 as *mut IOBUF;
        }
    }
    return status;
}
unsafe extern "C" fn close_redir(
    mut rp: *mut redirect,
    mut exitwarn: bool,
    mut how: two_way_close_type,
) -> i32 {
    let mut status: i32 = 0 as i32;
    if rp.is_null() {
        return 0 as i32;
    }
    if (*rp).flag as u32 & redirect_flags::RED_WRITE as i32 as u32 != 0
        && !((*rp).output.fp).is_null()
    {
        efflush((*rp).output.fp, b"flush\0" as *const u8 as *const i8, rp);
    }
    if !((*rp).output.fp == stdout || (*rp).output.fp == stderr) {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            && (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
                == 0 as i32 as u32
            && how as u32 != two_way_close_type::CLOSE_ALL as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1363 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close: redirection `%s' not opened with `|&', second argument ignored\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*rp).value,
            );
        }
        status = close_rp(rp, how);
        if status != 0 as i32 {
            let mut save_errno: i32 = *__errno_location();
            let mut s: *mut i8 = strerror(save_errno);
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            {
                if (*rp).flag as u32 & redirect_flags::RED_PIPE as i32 as u32
                    != 0 as i32 as u32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 1380 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"failure status (%d) on pipe close of `%s': %s\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        status,
                        (*rp).value,
                        s,
                    );
                } else if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
                    != 0 as i32 as u32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 1383 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"failure status (%d) on two-way pipe close of `%s': %s\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        status,
                        (*rp).value,
                        s,
                    );
                } else {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 1386 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"failure status (%d) on file close of `%s': %s\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        status,
                        (*rp).value,
                        s,
                    );
                }
            }
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
                update_ERRNO_int(save_errno);
            }
        }
    }
    if exitwarn {
        if (*rp).flag as u32 & redirect_flags::RED_SOCKET as i32 as u32
            != 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1406 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"no explicit close of socket `%s' provided\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*rp).value,
            );
        } else if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
            != 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1409 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"no explicit close of co-process `%s' provided\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*rp).value,
            );
        } else if (*rp).flag as u32 & redirect_flags::RED_PIPE as i32 as u32
            != 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1412 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"no explicit close of pipe `%s' provided\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*rp).value,
            );
        } else {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1415 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"no explicit close of file `%s' provided\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*rp).value,
            );
        }
    }
    if how as u32 == two_way_close_type::CLOSE_ALL as i32 as u32
        || ((*rp).iop).is_null() && ((*rp).output.fp).is_null()
    {
        if !((*rp).next).is_null() {
            (*(*rp).next).prev = (*rp).prev;
        }
        if !((*rp).prev).is_null() {
            (*(*rp).prev).next = (*rp).next;
        } else {
            red_head = (*rp).next;
        }
        free_rp(rp);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn non_fatal_flush_std_file(mut fp: *mut FILE) -> bool {
    let mut status: i32 = fflush(fp);
    if status != 0 as i32 {
        let mut is_fatal: bool = !is_non_fatal_std(fp);
        if is_fatal {
            os_maybe_set_errno();
            if *__errno_location() == 32 as i32 {
                signal(13 as i32, None);
                kill(getpid(), 13 as i32);
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 1449 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    if fp == stdout {
                        dcgettext(
                            0 as *const i8,
                            b"fflush: cannot flush standard output: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        )
                    } else {
                        dcgettext(
                            0 as *const i8,
                            b"fflush: cannot flush standard error: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        )
                    },
                    strerror(*__errno_location()),
                );
            }
        } else {
            update_ERRNO_int(*__errno_location());
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1455 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                if fp == stdout {
                    dcgettext(
                        0 as *const i8,
                        b"error writing standard output: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    )
                } else {
                    dcgettext(
                        0 as *const i8,
                        b"error writing standard error: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    )
                },
                strerror(*__errno_location()),
            );
        }
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn flush_io() -> i32 {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut status: i32 = 0 as i32;
    *__errno_location() = 0 as i32;
    if !non_fatal_flush_std_file(stdout) {
        status += 1;
        status;
    }
    *__errno_location() = 0 as i32;
    if !non_fatal_flush_std_file(stderr) {
        status += 1;
        status;
    }
    rp = red_head;
    while !rp.is_null() {
        let mut messagefunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()> = Some(
            r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
        );
        if (*rp).flag as u32 & redirect_flags::RED_WRITE as i32 as u32 != 0 as i32 as u32
            && !((*rp).output.fp).is_null()
        {
            if ((*rp).output.gawk_fflush)
                .expect(
                    "non-null function pointer",
                )((*rp).output.fp, (*rp).output.opaque) != 0 as i32
            {
                update_ERRNO_int(*__errno_location());
                if is_non_fatal_redirect((*rp).value, strlen((*rp).value)) {
                    messagefunc = Some(
                        r_warning as unsafe extern "C" fn(*const i8, ...) -> (),
                    );
                }
                if (*rp).flag as u32 & redirect_flags::RED_PIPE as i32 as u32
                    != 0 as i32 as u32
                {
                    messagefunc
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"pipe flush of `%s' failed: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                } else if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
                    != 0 as i32 as u32
                {
                    messagefunc
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"co-process flush of pipe to `%s' failed: %s\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                } else {
                    messagefunc
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"file flush of `%s' failed: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*rp).value,
                        strerror(*__errno_location()),
                    );
                }
                status += 1;
                status;
            }
        }
        rp = (*rp).next;
    }
    if status != 0 as i32 {
        status = -(1 as i32);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn close_io(
    mut stdio_problem: *mut bool,
    mut got_EPIPE: *mut bool,
) -> i32 {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut next: *mut redirect = 0 as *mut redirect;
    let mut status: i32 = 0 as i32;
    *got_EPIPE = 0 as i32 != 0;
    *stdio_problem = *got_EPIPE;
    *__errno_location() = 0 as i32;
    rp = red_head;
    while !rp.is_null() {
        next = (*rp).next;
        if close_redir(
            rp,
            do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0,
            two_way_close_type::CLOSE_ALL,
        ) != 0
        {
            status += 1;
            status;
        }
        rp = 0 as *mut redirect;
        rp = next;
    }
    *stdio_problem = 0 as i32 != 0;
    if fflush(stdout) != 0 as i32 {
        os_maybe_set_errno();
        if *__errno_location() != 32 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1545 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"error writing standard output: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        } else {
            *got_EPIPE = 1 as i32 != 0;
        }
        status += 1;
        status;
        *stdio_problem = 1 as i32 != 0;
    }
    if fflush(stderr) != 0 as i32 {
        os_maybe_set_errno();
        if *__errno_location() != 32 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1556 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"error writing standard error: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        } else {
            *got_EPIPE = 1 as i32 != 0;
        }
        status += 1;
        status;
        *stdio_problem = 1 as i32 != 0;
    }
    return status;
}
unsafe extern "C" fn str2mode(mut mode: *const i8) -> i32 {
    let mut ret: i32 = 0;
    let mut second: *const i8 = &*mode.offset(1 as i32 as isize) as *const i8;
    if *second as i32 == 'b' as i32 {
        second = second.offset(1);
        second;
    }
    match *mode.offset(0 as i32 as isize) as i32 {
        114 => {
            ret = 0 as i32;
            if *second as i32 == '+' as i32 || *second as i32 == 'w' as i32 {
                ret = 0o2 as i32;
            }
        }
        119 => {
            ret = 0o1 as i32 | 0o100 as i32 | 0o1000 as i32;
            if *second as i32 == '+' as i32 || *second as i32 == 'r' as i32 {
                ret = 0o2 as i32 | 0o100 as i32 | 0o1000 as i32;
            }
        }
        97 => {
            ret = 0o1 as i32 | 0o2000 as i32 | 0o100 as i32;
            if *second as i32 == '+' as i32 {
                ret = 0o2 as i32 | 0o2000 as i32 | 0o100 as i32;
            }
        }
        _ => {
            ret = 0 as i32;
            r_fatal(
                b"internal error: file %s, line %d: invalid open mode \"%s\"\0"
                    as *const u8 as *const i8,
                b"io.c\0" as *const u8 as *const i8,
                1598 as i32,
                mode,
            );
        }
    }
    if !(strchr(mode, 'b' as i32)).is_null() {
        ret |= 0 as i32;
    }
    return ret;
}
unsafe extern "C" fn socketopen(
    mut family: i32,
    mut type_0: i32,
    mut localpname: *const i8,
    mut remotepname: *const i8,
    mut remotehostname: *const i8,
    mut hard_error: *mut bool,
) -> i32 {
    let mut lres: *mut addrinfo = 0 as *mut addrinfo;
    let mut lres0: *mut addrinfo = 0 as *mut addrinfo;
    let mut lhints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut i8,
        ai_next: 0 as *mut addrinfo,
    };
    let mut rres: *mut addrinfo = 0 as *mut addrinfo;
    let mut rres0: *mut addrinfo = 0 as *mut addrinfo;
    let mut rhints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut i8,
        ai_next: 0 as *mut addrinfo,
    };
    let mut lerror: i32 = 0;
    let mut rerror: i32 = 0;
    let mut socket_fd: i32 = -(1 as i32);
    let mut any_remote_host: i32 = (strcmp(
        remotehostname,
        b"0\0" as *const u8 as *const i8,
    ) == 0 as i32) as i32;
    memset(
        &mut lhints as *mut addrinfo as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<addrinfo>() as u64,
    );
    lhints.ai_socktype = type_0;
    lhints.ai_family = family;
    lhints.ai_flags = 0x1 as i32;
    if lhints.ai_family == 0 as i32 {
        lhints.ai_flags |= 0x20 as i32;
    }
    lerror = getaddrinfo(0 as *const i8, localpname, &mut lhints, &mut lres);
    if lerror != 0 {
        if strcmp(localpname, b"0\0" as *const u8 as *const i8) != 0 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1645 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"local port %s invalid in `/inet': %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                localpname,
                gai_strerror(lerror),
            );
            *hard_error = 1 as i32 != 0;
            return -(1 as i32);
        }
        lres0 = 0 as *mut addrinfo;
        lres = &mut lhints;
    } else {
        lres0 = lres;
    }
    while !lres.is_null() {
        memset(
            &mut rhints as *mut addrinfo as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<addrinfo>() as u64,
        );
        rhints.ai_flags = lhints.ai_flags;
        rhints.ai_socktype = lhints.ai_socktype;
        rhints.ai_family = lhints.ai_family;
        rhints.ai_protocol = lhints.ai_protocol;
        rerror = getaddrinfo(
            if any_remote_host != 0 { 0 as *const i8 } else { remotehostname },
            remotepname,
            &mut rhints,
            &mut rres,
        );
        if rerror != 0 {
            if !lres0.is_null() {
                freeaddrinfo(lres0);
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 1671 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"remote host and port information (%s, %s) invalid: %s\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                remotehostname,
                remotepname,
                gai_strerror(rerror),
            );
            *hard_error = 1 as i32 != 0;
            return -(1 as i32);
        }
        rres0 = rres;
        socket_fd = -(1 as i32);
        while !rres.is_null() {
            socket_fd = socket(
                (*rres).ai_family,
                (*rres).ai_socktype,
                (*rres).ai_protocol,
            );
            if !(socket_fd < 0 as i32 || socket_fd == -(1 as i32)) {
                if type_0 == __socket_type::SOCK_STREAM as i32 {
                    let mut on: i32 = 1 as i32;
                    let mut linger: linger = linger { l_onoff: 0, l_linger: 0 };
                    memset(
                        &mut linger as *mut linger as *mut libc::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<linger>() as u64,
                    );
                    setsockopt(
                        socket_fd,
                        1 as i32,
                        2 as i32,
                        &mut on as *mut i32 as *mut i8 as *const libc::c_void,
                        ::core::mem::size_of::<i32>() as u64 as socklen_t,
                    );
                    linger.l_onoff = 1 as i32;
                    linger.l_linger = 30 as i32;
                    setsockopt(
                        socket_fd,
                        1 as i32,
                        13 as i32,
                        &mut linger as *mut linger as *mut i8 as *const libc::c_void,
                        ::core::mem::size_of::<linger>() as u64 as socklen_t,
                    );
                }
                if !(bind(
                    socket_fd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: (*lres).ai_addr,
                    },
                    (*lres).ai_addrlen,
                ) != 0 as i32)
                {
                    if any_remote_host == 0 {
                        if connect(
                            socket_fd,
                            __CONST_SOCKADDR_ARG {
                                __sockaddr__: (*rres).ai_addr,
                            },
                            (*rres).ai_addrlen,
                        ) == 0 as i32
                        {
                            break;
                        }
                    } else if type_0 == __socket_type::SOCK_STREAM as i32 {
                        let mut clientsocket_fd: i32 = -(1 as i32);
                        let mut remote_addr: sockaddr_storage = sockaddr_storage {
                            ss_family: 0,
                            __ss_padding: [0; 118],
                            __ss_align: 0,
                        };
                        let mut namelen: socklen_t = ::core::mem::size_of::<
                            sockaddr_storage,
                        >() as u64 as socklen_t;
                        if listen(socket_fd, 1 as i32) >= 0 as i32
                            && {
                                clientsocket_fd = accept(
                                    socket_fd,
                                    __SOCKADDR_ARG {
                                        __sockaddr__: &mut remote_addr as *mut sockaddr_storage
                                            as *mut sockaddr,
                                    },
                                    &mut namelen,
                                );
                                clientsocket_fd >= 0 as i32
                            }
                        {
                            close(socket_fd);
                            socket_fd = clientsocket_fd;
                            break;
                        }
                    } else if type_0 == __socket_type::SOCK_DGRAM as i32 {
                        let mut buf: [i8; 10] = [0; 10];
                        let mut remote_addr_0: sockaddr_storage = sockaddr_storage {
                            ss_family: 0,
                            __ss_padding: [0; 118],
                            __ss_align: 0,
                        };
                        let mut read_len: socklen_t = ::core::mem::size_of::<
                            sockaddr_storage,
                        >() as u64 as socklen_t;
                        if recvfrom(
                            socket_fd,
                            buf.as_mut_ptr() as *mut libc::c_void,
                            1 as i32 as size_t,
                            MSG_PEEK as i32,
                            __SOCKADDR_ARG {
                                __sockaddr__: &mut remote_addr_0 as *mut sockaddr_storage
                                    as *mut sockaddr,
                            },
                            &mut read_len,
                        ) >= 0 as i32 as i64 && read_len != 0
                            && connect(
                                socket_fd,
                                __CONST_SOCKADDR_ARG {
                                    __sockaddr__: &mut remote_addr_0 as *mut sockaddr_storage
                                        as *mut sockaddr,
                                },
                                read_len,
                            ) == 0 as i32
                        {
                            break;
                        }
                    }
                }
            }
            if socket_fd != -(1 as i32) {
                close(socket_fd);
            }
            socket_fd = -(1 as i32);
            rres = (*rres).ai_next;
        }
        freeaddrinfo(rres0);
        if socket_fd != -(1 as i32) {
            break;
        }
        lres = (*lres).ai_next;
    }
    if !lres0.is_null() {
        freeaddrinfo(lres0);
    }
    return socket_fd;
}
#[no_mangle]
pub unsafe extern "C" fn devopen_simple(
    mut name: *const i8,
    mut mode: *const i8,
    mut try_real_open: bool,
) -> i32 {
    let mut openfd: i32 = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut ptr: *mut i8 = 0 as *mut i8;
    let mut flag: i32 = 0 as i32;
    if strcmp(name, b"-\0" as *const u8 as *const i8) == 0 as i32 {
        if *mode.offset(0 as i32 as isize) as i32 == 'r' as i32 {
            return fileno(stdin)
        } else {
            return fileno(stdout)
        }
    }
    flag = str2mode(mode);
    openfd = -(1 as i32);
    if !(do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0) {
        openfd = os_devopen(name, flag);
        if openfd != -(1 as i32) {
            os_close_on_exec(
                openfd,
                name,
                b"file\0" as *const u8 as *const i8,
                b"\0" as *const u8 as *const i8,
            );
            return openfd;
        }
        if strncmp(name, b"/dev/\0" as *const u8 as *const i8, 5 as i32 as u64)
            == 0 as i32
        {
            cp = (name as *mut i8).offset(5 as i32 as isize);
            if strcmp(cp, b"stdin\0" as *const u8 as *const i8) == 0 as i32
                && flag & 0o3 as i32 == 0 as i32
            {
                openfd = fileno(stdin);
            } else if strcmp(cp, b"stdout\0" as *const u8 as *const i8) == 0 as i32
                && flag & 0o3 as i32 == 0o1 as i32
            {
                openfd = fileno(stdout);
            } else if strcmp(cp, b"stderr\0" as *const u8 as *const i8) == 0 as i32
                && flag & 0o3 as i32 == 0o1 as i32
            {
                openfd = fileno(stderr);
            } else if !(do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                != 0)
            {
                if strncmp(cp, b"fd/\0" as *const u8 as *const i8, 3 as i32 as u64)
                    == 0 as i32
                {
                    let mut sbuf: stat = stat {
                        st_dev: 0,
                        st_ino: 0,
                        st_nlink: 0,
                        st_mode: 0,
                        st_uid: 0,
                        st_gid: 0,
                        __pad0: 0,
                        st_rdev: 0,
                        st_size: 0,
                        st_blksize: 0,
                        st_blocks: 0,
                        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
                        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
                        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
                        __glibc_reserved: [0; 3],
                    };
                    cp = cp.offset(3 as i32 as isize);
                    openfd = strtoul(cp, &mut ptr, 10 as i32) as i32;
                    if openfd <= -(1 as i32) || ptr == cp
                        || fstat(openfd, &mut sbuf) < 0 as i32
                    {
                        openfd = -(1 as i32);
                    }
                }
            }
        }
    }
    if try_real_open {
        openfd = open(name, flag, 0o666 as i32);
    }
    return openfd;
}
#[no_mangle]
pub unsafe extern "C" fn devopen(mut name: *const i8, mut mode: *const i8) -> i32 {
    let mut openfd: i32 = 0;
    let mut flag: i32 = 0;
    let mut isi: inet_socket_info = inet_socket_info {
        family: 0,
        protocol: 0,
        localport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remotehost: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remoteport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
    };
    let mut save_errno: i32 = 0 as i32;
    openfd = devopen_simple(name, mode, 0 as i32 != 0);
    if openfd != -(1 as i32) {
        return openfd;
    }
    flag = str2mode(mode);
    if !(do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0) {
        if inetfile(name, strlen(name), &mut isi) {
            static mut def_retries: u64 = 20 as i32 as u64;
            static mut first_time: bool = 1 as i32 != 0;
            let mut retries: u64 = 0 as i32 as u64;
            static mut msleep: i64 = 1000 as i32 as i64;
            let mut hard_error: bool = 0 as i32 != 0;
            let mut non_fatal: bool = is_non_fatal_redirect(name, strlen(name));
            let mut save: i8 = 0;
            let mut cp: *mut i8 = name as *mut i8;
            *cp.offset((isi.localport.offset + isi.localport.len) as isize) = '\0' as i32
                as i8;
            *cp.offset((isi.remotehost.offset + isi.remotehost.len) as isize) = '\0'
                as i32 as i8;
            save = *cp.offset((isi.remoteport.offset + isi.remoteport.len) as isize);
            *cp.offset((isi.remoteport.offset + isi.remoteport.len) as isize) = '\0'
                as i32 as i8;
            if first_time {
                let mut cp_0: *mut i8 = 0 as *mut i8;
                let mut end: *mut i8 = 0 as *mut i8;
                let mut count: u64 = 0 as i32 as u64;
                let mut ms2: *mut i8 = 0 as *mut i8;
                first_time = 0 as i32 != 0;
                cp_0 = getenv(b"GAWK_SOCK_RETRIES\0" as *const u8 as *const i8);
                if !cp_0.is_null() {
                    count = strtoul(cp_0, &mut end, 10 as i32);
                    if end != cp_0 && count > 0 as i32 as u64 {
                        def_retries = count;
                    }
                }
                ms2 = getenv(b"GAWK_MSEC_SLEEP\0" as *const u8 as *const i8);
                if !ms2.is_null() {
                    msleep = strtol(ms2, &mut end, 10 as i32);
                    if end == ms2 || msleep < 0 as i32 as i64 {
                        msleep = 1000 as i32 as i64;
                    } else {
                        msleep *= 1000 as i32 as i64;
                    }
                }
            }
            retries = if non_fatal as i32 != 0 { 1 as i32 as u64 } else { def_retries };
            *__errno_location() = 0 as i32;
            loop {
                openfd = socketopen(
                    isi.family,
                    isi.protocol,
                    name.offset(isi.localport.offset as isize),
                    name.offset(isi.remoteport.offset as isize),
                    name.offset(isi.remotehost.offset as isize),
                    &mut hard_error,
                );
                retries = retries.wrapping_sub(1);
                retries;
                if !(openfd == -(1 as i32) && !hard_error && retries > 0 as i32 as u64
                    && usleep(msleep as __useconds_t) == 0 as i32)
                {
                    break;
                }
            }
            save_errno = *__errno_location();
            *cp.offset((isi.localport.offset + isi.localport.len) as isize) = '/' as i32
                as i8;
            *cp.offset((isi.remotehost.offset + isi.remotehost.len) as isize) = '/'
                as i32 as i8;
            *cp.offset((isi.remoteport.offset + isi.remoteport.len) as isize) = save;
        }
    }
    if openfd == -(1 as i32) {
        openfd = open(name, flag, 0o666 as i32);
        if openfd == -(1 as i32) && *__errno_location() == 2 as i32 && save_errno != 0 {
            *__errno_location() = save_errno;
        }
    }
    if openfd != -(1 as i32) {
        if openfd > fileno(stderr) {
            os_close_on_exec(
                openfd,
                name,
                b"file\0" as *const u8 as *const i8,
                b"\0" as *const u8 as *const i8,
            );
        }
    }
    return openfd;
}
unsafe extern "C" fn push_pty_line_disciplines(mut slave: i32) {
    if ioctl(
        slave,
        (('S' as i32) << 8 as i32 | 11 as i32) as u64,
        b"ptem\0" as *const u8 as *const i8,
    ) == 0 as i32
    {
        ioctl(
            slave,
            (('S' as i32) << 8 as i32 | 2 as i32) as u64,
            b"ptem\0" as *const u8 as *const i8,
        );
    }
    if ioctl(
        slave,
        (('S' as i32) << 8 as i32 | 11 as i32) as u64,
        b"ldterm\0" as *const u8 as *const i8,
    ) == 0 as i32
    {
        ioctl(
            slave,
            (('S' as i32) << 8 as i32 | 2 as i32) as u64,
            b"ldterm\0" as *const u8 as *const i8,
        );
    }
}
unsafe extern "C" fn set_slave_pty_attributes(mut slave: i32) {
    let mut st: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    tcgetattr(slave, &mut st);
    st.c_iflag &= !(0o40 as i32 | 0o200 as i32 | 0o100 as i32 | 0o10000 as i32) as u32;
    st.c_iflag |= (0o400 as i32 | 0o4 as i32 | 0o2 as i32 | 0o2000 as i32) as u32;
    st.c_oflag &= !(0o1 as i32) as u32;
    st.c_cflag &= !(0o60 as i32) as u32;
    st.c_cflag |= (0o200 as i32 | 0o60 as i32 | 0o4000 as i32) as u32;
    st.c_lflag
        &= !(0o10 as i32 | 0o20 as i32 | 0o40 as i32 | 0o200 as i32 | 0o400 as i32)
            as u32;
    st.c_lflag |= 0o1 as i32 as u32;
    st.c_cc[0 as i32 as usize] = '\u{3}' as i32 as cc_t;
    st.c_cc[1 as i32 as usize] = '\u{1c}' as i32 as cc_t;
    st.c_cc[2 as i32 as usize] = '\u{7f}' as i32 as cc_t;
    st.c_cc[3 as i32 as usize] = '\u{15}' as i32 as cc_t;
    st.c_cc[4 as i32 as usize] = '\u{4}' as i32 as cc_t;
    tcsetattr(slave, 0 as i32, &mut st);
}
unsafe extern "C" fn fork_and_open_slave_pty(
    mut slavenam: *const i8,
    mut master: i32,
    mut command: *const i8,
    mut pid: *mut pid_t,
) -> bool {
    let mut slave: i32 = 0;
    let mut save_errno: i32 = 0;
    slave = open(slavenam, 0o2 as i32);
    if slave < 0 as i32 {
        close(master);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 2087 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"could not open `%s', mode `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            slavenam,
            b"r+\0" as *const u8 as *const i8,
        );
    }
    push_pty_line_disciplines(slave);
    set_slave_pty_attributes(slave);
    *pid = fork();
    match *pid {
        0 => {
            setsid();
            ioctl(slave, 0x540e as i32 as u64, 0 as i32);
            if close(master) == -(1 as i32) {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 2104 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"close of master pty failed: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if close(1 as i32) == -(1 as i32) {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 2106 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"close of stdout in child failed: %s\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if dup(slave) != 1 as i32 {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 2109 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"moving slave pty to stdout in child failed (dup: %s)\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if close(0 as i32) == -(1 as i32) {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 2111 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"close of stdin in child failed: %s\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if dup(slave) != 0 as i32 {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 2114 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"moving slave pty to stdin in child failed (dup: %s)\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    strerror(*__errno_location()),
                );
            }
            if close(slave) != 0 {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 2116 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"close of slave pty failed: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    strerror(*__errno_location()),
                );
            }
            signal(13 as i32, None);
            execl(
                b"/bin/sh\0" as *const u8 as *const i8,
                b"sh\0" as *const u8 as *const i8,
                b"-c\0" as *const u8 as *const i8,
                command,
                0 as *mut libc::c_void,
            );
            _exit(if *__errno_location() == 2 as i32 { 127 as i32 } else { 126 as i32 });
        }
        -1 => {
            save_errno = *__errno_location();
            close(master);
            close(slave);
            *__errno_location() = save_errno;
            return 0 as i32 != 0;
        }
        _ => {}
    }
    if close(slave) != 0 as i32 {
        close(master);
        kill(*pid, 9 as i32);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 2138 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"close of slave pty failed: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn two_way_open(
    mut str: *const i8,
    mut rp: *mut redirect,
    mut extfd: i32,
) -> i32 {
    let mut current_block: u64;
    static mut no_ptys: bool = 0 as i32 != 0;
    if extfd >= 0 as i32
        || inetfile(str, strlen(str), 0 as *mut inet_socket_info) as i32 != 0
    {
        let mut fd: i32 = 0;
        let mut newfd: i32 = 0;
        fd = if extfd >= 0 as i32 {
            extfd
        } else {
            devopen(str, b"rw\0" as *const u8 as *const i8)
        };
        if fd == -(1 as i32) {
            return 0 as i32;
        }
        if BINMODE & binmode_values::BINMODE_OUTPUT as i32 != 0 as i32 {
            os_setbinmode(fd, 0 as i32);
        }
        (*rp).output.fp = fdopen(fd, b"wb\0" as *const u8 as *const i8);
        if ((*rp).output.fp).is_null() {
            close(fd);
            return 0 as i32;
        }
        newfd = dup(fd);
        if newfd < 0 as i32 {
            ((*rp).output.gawk_fclose)
                .expect(
                    "non-null function pointer",
                )((*rp).output.fp, (*rp).output.opaque);
            return 0 as i32;
        }
        if BINMODE & binmode_values::BINMODE_INPUT as i32 != 0 as i32 {
            os_setbinmode(newfd, 0 as i32);
        }
        os_close_on_exec(
            fd,
            str,
            b"socket\0" as *const u8 as *const i8,
            b"to/from\0" as *const u8 as *const i8,
        );
        os_close_on_exec(
            newfd,
            str,
            b"socket\0" as *const u8 as *const i8,
            b"to/from\0" as *const u8 as *const i8,
        );
        (*rp).iop = iop_alloc(newfd, str, 0 as i32);
        (*rp).output.name = str;
        find_input_parser((*rp).iop);
        iop_finish((*rp).iop);
        if !(*(*rp).iop).valid {
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                && (*(*rp).iop).errcode != 0 as i32
            {
                update_ERRNO_int((*(*rp).iop).errcode);
            }
            iop_close((*rp).iop);
            (*rp).iop = 0 as *mut IOBUF;
            ((*rp).output.gawk_fclose)
                .expect(
                    "non-null function pointer",
                )((*rp).output.fp, (*rp).output.opaque);
            return 0 as i32;
        }
        (*rp).flag = ::core::mem::transmute::<
            u32,
            redirect_flags,
        >((*rp).flag as u32 | redirect_flags::RED_SOCKET as i32 as u32);
        return 1 as i32;
    }
    if find_two_way_processor(str, rp) {
        return 1 as i32;
    }
    if !no_ptys && pty_vs_pipe(str) as i32 != 0 {
        static mut initialized: bool = 0 as i32 != 0;
        static mut first_pty_letter: i8 = 0;
        let mut slavenam: [i8; 32] = [0; 32];
        let mut c: i8 = 0;
        let mut master: i32 = 0;
        let mut dup_master: i32 = 0;
        let mut pid: pid_t = 0;
        let mut statb: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        static mut pty_chars: [i8; 27] = unsafe {
            *::core::mem::transmute::<
                &[u8; 27],
                &mut [i8; 27],
            >(b"pqrstuvwxyzabcdefghijklmno\0")
        };
        let mut i: i32 = 0;
        if !initialized {
            initialized = 1 as i32 != 0;
            i = 0 as i32;
            loop {
                let fresh2 = i;
                i = i + 1;
                c = pty_chars[fresh2 as usize];
                sprintf(
                    slavenam.as_mut_ptr(),
                    b"/dev/pty%c0\0" as *const u8 as *const i8,
                    c as i32,
                );
                if stat(slavenam.as_mut_ptr(), &mut statb) >= 0 as i32 {
                    first_pty_letter = c;
                    break;
                } else if !(pty_chars[i as usize] as i32 != '\0' as i32) {
                    break;
                }
            }
        }
        master = posix_openpt(0o2 as i32 | 0o400 as i32);
        if master >= 0 as i32 {
            let mut tem: *mut i8 = 0 as *mut i8;
            grantpt(master);
            unlockpt(master);
            tem = ptsname(master);
            if !tem.is_null() {
                strcpy(slavenam.as_mut_ptr(), tem);
                current_block = 158932413621273621;
            } else {
                close(master);
                current_block = 15004371738079956865;
            }
        } else {
            current_block = 15004371738079956865;
        }
        match current_block {
            15004371738079956865 => {
                if first_pty_letter != 0 {
                    c = first_pty_letter;
                    's_236: loop {
                        let mut i_0: i32 = 0;
                        let mut cp: *mut i8 = 0 as *mut i8;
                        i_0 = 0 as i32;
                        while i_0 < 16 as i32 {
                            sprintf(
                                slavenam.as_mut_ptr(),
                                b"/dev/pty%c%x\0" as *const u8 as *const i8,
                                c as i32,
                                i_0,
                            );
                            if stat(slavenam.as_mut_ptr(), &mut statb) < 0 as i32 {
                                no_ptys = 1 as i32 != 0;
                                current_block = 11076367681174882236;
                                break 's_236;
                            } else {
                                master = open(slavenam.as_mut_ptr(), 0o2 as i32);
                                if master >= 0 as i32 {
                                    slavenam[(::core::mem::size_of::<[i8; 6]>() as u64)
                                        .wrapping_sub(1 as i32 as u64) as usize] = 't' as i32 as i8;
                                    if access(slavenam.as_mut_ptr(), 4 as i32 | 2 as i32)
                                        == 0 as i32
                                    {
                                        current_block = 158932413621273621;
                                        break 's_236;
                                    }
                                    close(master);
                                }
                                i_0 += 1;
                                i_0;
                            }
                        }
                        cp = strchr(pty_chars.as_mut_ptr(), c as i32);
                        if *cp.offset(1 as i32 as isize) as i32 != '\0' as i32 {
                            cp = cp.offset(1);
                            cp;
                        } else {
                            cp = pty_chars.as_mut_ptr();
                        }
                        c = *cp;
                        if !(c as i32 != first_pty_letter as i32) {
                            current_block = 17075014677070940716;
                            break;
                        }
                    }
                } else {
                    no_ptys = 1 as i32 != 0;
                    current_block = 17075014677070940716;
                }
                match current_block {
                    11076367681174882236 => {}
                    158932413621273621 => {}
                    _ => {
                        current_block = 11076367681174882236;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            11076367681174882236 => {}
            _ => {
                if !fork_and_open_slave_pty(
                    slavenam.as_mut_ptr(),
                    master,
                    str,
                    &mut pid,
                ) {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"io.c\0" as *const u8 as *const i8, 2300 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"could not create child process or open pty\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                (*rp).pid = pid;
                (*rp).iop = iop_alloc(master, str, 0 as i32);
                find_input_parser((*rp).iop);
                iop_finish((*rp).iop);
                if !(*(*rp).iop).valid {
                    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                        == 0 && (*(*rp).iop).errcode != 0 as i32
                    {
                        update_ERRNO_int((*(*rp).iop).errcode);
                    }
                    iop_close((*rp).iop);
                    (*rp).iop = 0 as *mut IOBUF;
                    kill(pid, 9 as i32);
                    return 0 as i32;
                }
                (*rp).output.name = str;
                (*rp).output.mode = b"w\0" as *const u8 as *const i8;
                dup_master = dup(master);
                if dup_master < 0 as i32
                    || {
                        (*rp).output.fp = fdopen(
                            dup_master,
                            b"w\0" as *const u8 as *const i8,
                        );
                        ((*rp).output.fp).is_null()
                    }
                {
                    iop_close((*rp).iop);
                    (*rp).iop = 0 as *mut IOBUF;
                    close(master);
                    kill(pid, 9 as i32);
                    if dup_master > 0 as i32 {
                        close(dup_master);
                    }
                    return 0 as i32;
                } else {
                    find_output_wrapper(&mut (*rp).output);
                }
                (*rp).flag = ::core::mem::transmute::<
                    u32,
                    redirect_flags,
                >((*rp).flag as u32 | redirect_flags::RED_PTY as i32 as u32);
                os_close_on_exec(
                    master,
                    str,
                    b"pipe\0" as *const u8 as *const i8,
                    b"from\0" as *const u8 as *const i8,
                );
                os_close_on_exec(
                    dup_master,
                    str,
                    b"pipe\0" as *const u8 as *const i8,
                    b"to\0" as *const u8 as *const i8,
                );
                first_pty_letter = '\0' as i32 as i8;
                return 1 as i32;
            }
        }
    }
    let mut ptoc: [i32; 2] = [0; 2];
    let mut ctop: [i32; 2] = [0; 2];
    let mut pid_0: i32 = 0;
    let mut save_errno: i32 = 0;
    if pipe(ptoc.as_mut_ptr()) < 0 as i32 {
        return 0 as i32;
    }
    if pipe(ctop.as_mut_ptr()) < 0 as i32 {
        save_errno = *__errno_location();
        close(ptoc[0 as i32 as usize]);
        close(ptoc[1 as i32 as usize]);
        *__errno_location() = save_errno;
        return 0 as i32;
    }
    pid_0 = fork();
    if pid_0 < 0 as i32 {
        save_errno = *__errno_location();
        close(ptoc[0 as i32 as usize]);
        close(ptoc[1 as i32 as usize]);
        close(ctop[0 as i32 as usize]);
        close(ctop[1 as i32 as usize]);
        *__errno_location() = save_errno;
        return 0 as i32;
    }
    if pid_0 == 0 as i32 {
        if close(1 as i32) == -(1 as i32) {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2447 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close of stdout in child failed: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        if dup(ctop[1 as i32 as usize]) != 1 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2450 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"moving pipe to stdout in child failed (dup: %s)\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        if close(0 as i32) == -(1 as i32) {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2452 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close of stdin in child failed: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        if dup(ptoc[0 as i32 as usize]) != 0 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2455 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"moving pipe to stdin in child failed (dup: %s)\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        if close(ptoc[0 as i32 as usize]) == -(1 as i32)
            || close(ptoc[1 as i32 as usize]) == -(1 as i32)
            || close(ctop[0 as i32 as usize]) == -(1 as i32)
            || close(ctop[1 as i32 as usize]) == -(1 as i32)
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2458 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close of pipe failed: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        signal(13 as i32, None);
        execl(
            b"/bin/sh\0" as *const u8 as *const i8,
            b"sh\0" as *const u8 as *const i8,
            b"-c\0" as *const u8 as *const i8,
            str,
            0 as *mut libc::c_void,
        );
        _exit(if *__errno_location() == 2 as i32 { 127 as i32 } else { 126 as i32 });
    }
    if BINMODE & binmode_values::BINMODE_INPUT as i32 != 0 as i32 {
        os_setbinmode(ctop[0 as i32 as usize], 0 as i32);
    }
    if BINMODE & binmode_values::BINMODE_OUTPUT as i32 != 0 as i32 {
        os_setbinmode(ptoc[1 as i32 as usize], 0 as i32);
    }
    (*rp).pid = pid_0;
    (*rp).iop = iop_alloc(ctop[0 as i32 as usize], str, 0 as i32);
    find_input_parser((*rp).iop);
    iop_finish((*rp).iop);
    if !(*(*rp).iop).valid {
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && (*(*rp).iop).errcode != 0 as i32
        {
            update_ERRNO_int((*(*rp).iop).errcode);
        }
        iop_close((*rp).iop);
        (*rp).iop = 0 as *mut IOBUF;
        close(ctop[1 as i32 as usize]);
        close(ptoc[0 as i32 as usize]);
        close(ptoc[1 as i32 as usize]);
        kill(pid_0, 9 as i32);
        return 0 as i32;
    }
    (*rp).output.fp = fdopen(ptoc[1 as i32 as usize], b"w\0" as *const u8 as *const i8);
    (*rp).output.mode = b"w\0" as *const u8 as *const i8;
    (*rp).output.name = str;
    if ((*rp).output.fp).is_null() {
        iop_close((*rp).iop);
        (*rp).iop = 0 as *mut IOBUF;
        close(ctop[0 as i32 as usize]);
        close(ctop[1 as i32 as usize]);
        close(ptoc[0 as i32 as usize]);
        close(ptoc[1 as i32 as usize]);
        kill(pid_0, 9 as i32);
        return 0 as i32;
    } else {
        find_output_wrapper(&mut (*rp).output);
    }
    os_close_on_exec(
        ctop[0 as i32 as usize],
        str,
        b"pipe\0" as *const u8 as *const i8,
        b"from\0" as *const u8 as *const i8,
    );
    os_close_on_exec(
        ptoc[1 as i32 as usize],
        str,
        b"pipe\0" as *const u8 as *const i8,
        b"from\0" as *const u8 as *const i8,
    );
    close(ptoc[0 as i32 as usize]);
    close(ctop[1 as i32 as usize]);
    return 1 as i32;
}
unsafe extern "C" fn wait_any(mut interesting: i32) -> i32 {
    let mut pid: i32 = 0;
    let mut status: i32 = 0 as i32;
    let mut redp: *mut redirect = 0 as *mut redirect;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut set);
    sigaddset(&mut set, 2 as i32);
    sigaddset(&mut set, 1 as i32);
    sigaddset(&mut set, 3 as i32);
    sigprocmask(0 as i32, &mut set, &mut oldset);
    loop {
        pid = waitpid(
            -(1 as i32),
            &mut status,
            (if interesting != 0 { 0 as i32 } else { 1 as i32 }),
        );
        if pid == 0 as i32 {
            break;
        }
        if interesting != 0 && pid == interesting {
            break;
        }
        if pid != -(1 as i32) {
            redp = red_head;
            while !redp.is_null() {
                if pid == (*redp).pid {
                    (*redp).pid = -(1 as i32);
                    (*redp).status = sanitize_exit_status(status);
                    break;
                } else {
                    redp = (*redp).next;
                }
            }
        }
        if pid == -(1 as i32) && *__errno_location() == 10 as i32 {
            break;
        }
    }
    sigprocmask(2 as i32, &mut oldset, 0 as *mut sigset_t);
    return status;
}
unsafe extern "C" fn gawk_popen(
    mut cmd: *const i8,
    mut rp: *mut redirect,
) -> *mut IOBUF {
    let mut p: [i32; 2] = [0; 2];
    let mut pid: i32 = 0;
    if pipe(p.as_mut_ptr()) < 0 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 2645 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"cannot open pipe `%s': %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            cmd,
            strerror(*__errno_location()),
        );
    }
    pid = fork();
    if pid == 0 as i32 {
        if close(1 as i32) == -(1 as i32) {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2685 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close of stdout in child failed: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        if dup(p[1 as i32 as usize]) != 1 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2688 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"moving pipe to stdout in child failed (dup: %s)\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        if close(p[0 as i32 as usize]) == -(1 as i32)
            || close(p[1 as i32 as usize]) == -(1 as i32)
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 2690 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"close of pipe failed: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        signal(13 as i32, None);
        execl(
            b"/bin/sh\0" as *const u8 as *const i8,
            b"sh\0" as *const u8 as *const i8,
            b"-c\0" as *const u8 as *const i8,
            cmd,
            0 as *mut libc::c_void,
        );
        _exit(if *__errno_location() == 2 as i32 { 127 as i32 } else { 126 as i32 });
    }
    if pid == -(1 as i32) {
        close(p[0 as i32 as usize]);
        close(p[1 as i32 as usize]);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 2699 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"cannot create child process for `%s' (fork: %s)\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            cmd,
            strerror(*__errno_location()),
        );
    }
    (*rp).pid = pid;
    if close(p[1 as i32 as usize]) == -(1 as i32) {
        close(p[0 as i32 as usize]);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 2705 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"close of pipe failed: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
    }
    os_close_on_exec(
        p[0 as i32 as usize],
        cmd,
        b"pipe\0" as *const u8 as *const i8,
        b"from\0" as *const u8 as *const i8,
    );
    if BINMODE & binmode_values::BINMODE_INPUT as i32 != 0 as i32 {
        os_setbinmode(p[0 as i32 as usize], 0 as i32);
    }
    (*rp).iop = iop_alloc(p[0 as i32 as usize], cmd, 0 as i32);
    find_input_parser((*rp).iop);
    iop_finish((*rp).iop);
    if !(*(*rp).iop).valid {
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && (*(*rp).iop).errcode != 0 as i32
        {
            update_ERRNO_int((*(*rp).iop).errcode);
        }
        iop_close((*rp).iop);
        (*rp).iop = 0 as *mut IOBUF;
    }
    return (*rp).iop;
}
unsafe extern "C" fn gawk_pclose(mut rp: *mut redirect) -> i32 {
    if !((*rp).iop).is_null() {
        iop_close((*rp).iop);
    }
    (*rp).iop = 0 as *mut IOBUF;
    if (*rp).pid == -(1 as i32) {
        return (*rp).status;
    }
    (*rp).status = sanitize_exit_status(wait_any((*rp).pid));
    (*rp).pid = -(1 as i32);
    return (*rp).status;
}
#[no_mangle]
pub unsafe extern "C" fn do_getline_redir(
    mut into_variable: i32,
    mut redirtype: redirval,
) -> *mut NODE {
    let mut rp: *mut redirect = 0 as *mut redirect;
    let mut iop: *mut IOBUF = 0 as *mut IOBUF;
    let mut cnt: size_t = 0;
    let mut retval: i32 = -(1 as i32);
    let mut s: *mut i8 = 0 as *mut i8;
    let mut errcode: i32 = 0;
    let mut redir_exp: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut redir_error: i32 = 0 as i32;
    let mut field_width: *const awk_fieldwidth_info_t = 0
        as *const awk_fieldwidth_info_t;
    if into_variable != 0 {
        let fresh3 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        lhs = (*fresh3).lptr;
    }
    redir_exp = (*stack_ptr).rptr;
    rp = redirect(redir_exp, redirtype as i32, &mut redir_error, 0 as i32 != 0);
    DEREF(redir_exp);
    stack_ptr = stack_ptr.offset(-1);
    stack_ptr;
    if rp.is_null() {
        if redir_error != 0 {
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
                update_ERRNO_int(redir_error);
            }
        }
        return make_number.expect("non-null function pointer")(-1.0f64);
    } else if (*rp).flag as u32 & redirect_flags::RED_TWOWAY as i32 as u32
        != 0 as i32 as u32 && ((*rp).iop).is_null()
    {
        if is_non_fatal_redirect((*redir_exp).sub.val.sp, (*redir_exp).sub.val.slen) {
            update_ERRNO_int(9 as i32);
            return make_number.expect("non-null function pointer")(-1.0f64);
        }
        close_rp(rp, two_way_close_type::CLOSE_ALL);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 2838 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"getline: attempt to read from closed read end of two-way pipe\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    iop = (*rp).iop;
    if iop.is_null() {
        return make_number.expect("non-null function pointer")(0.0f64);
    }
    errcode = 0 as i32;
    retval = get_a_record(
        &mut s,
        &mut cnt,
        iop,
        &mut errcode,
        if !lhs.is_null() {
            0 as *mut *const awk_fieldwidth_info_t
        } else {
            &mut field_width
        },
    );
    if errcode != 0 as i32 {
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && errcode != -(1 as i32)
        {
            update_ERRNO_int(errcode);
        }
        return make_number.expect("non-null function pointer")(retval as libc::c_double);
    }
    if retval == -(1 as i32) {
        if (*rp).flag as u32
            & (redirect_flags::RED_PIPE as i32 | redirect_flags::RED_TWOWAY as i32)
                as u32 == 0 as i32 as u32
        {
            iop_close(iop);
            (*rp).iop = 0 as *mut IOBUF;
        }
        (*rp).flag = ::core::mem::transmute::<
            u32,
            redirect_flags,
        >((*rp).flag as u32 | redirect_flags::RED_EOF as i32 as u32);
        return make_number.expect("non-null function pointer")(0.0f64);
    }
    if lhs.is_null() {
        set_record(s, cnt, field_width);
    } else {
        unref(*lhs);
        *lhs = make_str_node(
            if !s.is_null() { s } else { b"\0" as *const u8 as *const i8 },
            cnt,
            0 as i32,
        );
        (**lhs).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((**lhs).flags as u32 | flagvals::USER_INPUT as i32 as u32);
    }
    return make_number.expect("non-null function pointer")(1.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn do_getline(
    mut into_variable: i32,
    mut iop: *mut IOBUF,
) -> *mut NODE {
    let mut cnt: size_t = 0;
    let mut retval: i32 = -(1 as i32);
    let mut s: *mut i8 = 0 as *mut i8;
    let mut errcode: i32 = 0;
    let mut field_width: *const awk_fieldwidth_info_t = 0
        as *const awk_fieldwidth_info_t;
    if iop.is_null() {
        if into_variable != 0 {
            stack_ptr = stack_ptr.offset(-1);
            stack_ptr;
        }
        return make_number.expect("non-null function pointer")(0.0f64);
    }
    errcode = 0 as i32;
    retval = get_a_record(
        &mut s,
        &mut cnt,
        iop,
        &mut errcode,
        if into_variable != 0 {
            0 as *mut *const awk_fieldwidth_info_t
        } else {
            &mut field_width
        },
    );
    if errcode != 0 as i32 {
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && errcode != -(1 as i32)
        {
            update_ERRNO_int(errcode);
        }
        if into_variable != 0 {
            stack_ptr = stack_ptr.offset(-1);
            stack_ptr;
        }
        return make_number.expect("non-null function pointer")(retval as libc::c_double);
    }
    if retval == -(1 as i32) {
        return 0 as *mut NODE;
    }
    NR += 1;
    NR;
    FNR += 1;
    FNR;
    if into_variable == 0 {
        set_record(s, cnt, field_width);
    } else {
        let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
        let fresh4 = stack_ptr;
        stack_ptr = stack_ptr.offset(-1);
        lhs = (*fresh4).lptr;
        unref(*lhs);
        *lhs = make_str_node(
            if !s.is_null() { s } else { b"\0" as *const u8 as *const i8 },
            cnt,
            0 as i32,
        );
        (**lhs).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((**lhs).flags as u32 | flagvals::USER_INPUT as i32 as u32);
    }
    return make_number.expect("non-null function pointer")(1.0f64);
}
static mut pi_awkpath: path_info = unsafe {
    {
        let mut init = path_info {
            envname: b"AWKPATH\0" as *const u8 as *const i8,
            dfltp: &defpath as *const *const i8 as *mut *const i8,
            awkpath: 0 as *const *const i8 as *mut *const i8,
            max_pathlen: 0,
        };
        init
    }
};
static mut pi_awklibpath: path_info = unsafe {
    {
        let mut init = path_info {
            envname: b"AWKLIBPATH\0" as *const u8 as *const i8,
            dfltp: &deflibpath as *const *const i8 as *mut *const i8,
            awkpath: 0 as *const *const i8 as *mut *const i8,
            max_pathlen: 0,
        };
        init
    }
};
unsafe extern "C" fn init_awkpath(mut pi: *mut path_info) {
    let mut path: *const i8 = 0 as *const i8;
    let mut start: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut max_path: i32 = 0;
    (*pi).max_pathlen = 0 as i32;
    path = getenv((*pi).envname);
    if path.is_null() || *path as i32 == '\0' as i32 {
        path = *((*pi).dfltp).offset(0 as i32 as isize);
    }
    max_path = 0 as i32;
    p = path as *mut i8;
    while *p != 0 {
        if *p as i32 == envsep as i32 {
            max_path += 1;
            max_path;
        }
        p = p.offset(1);
        p;
    }
    (*pi).awkpath = ezalloc_real(
        ((max_path + 3 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
        b"init_awkpath\0" as *const u8 as *const i8,
        b"pi->awkpath\0" as *const u8 as *const i8,
        b"io.c\0" as *const u8 as *const i8,
        2963 as i32,
    ) as *mut *const i8;
    start = path;
    i = 0 as i32;
    if *path as i32 == envsep as i32 {
        let fresh5 = i;
        i = i + 1;
        let ref mut fresh6 = *((*pi).awkpath).offset(fresh5 as isize);
        *fresh6 = b".\0" as *const u8 as *const i8;
        (*pi).max_pathlen = 1 as i32;
    }
    while *start != 0 {
        if *start as i32 == envsep as i32 {
            if *start.offset(1 as i32 as isize) as i32 == envsep as i32 {
                let fresh7 = i;
                i = i + 1;
                let ref mut fresh8 = *((*pi).awkpath).offset(fresh7 as isize);
                *fresh8 = b".\0" as *const u8 as *const i8;
                if (*pi).max_pathlen == 0 as i32 {
                    (*pi).max_pathlen = 1 as i32;
                }
                start = start.offset(1);
                start;
            } else if *start.offset(1 as i32 as isize) as i32 == '\0' as i32 {
                let fresh9 = i;
                i = i + 1;
                let ref mut fresh10 = *((*pi).awkpath).offset(fresh9 as isize);
                *fresh10 = b".\0" as *const u8 as *const i8;
                if (*pi).max_pathlen == 0 as i32 {
                    (*pi).max_pathlen = 1 as i32;
                }
                break;
            } else {
                start = start.offset(1);
                start;
            }
        } else {
            end = start;
            while *end as i32 != 0 && *end as i32 != envsep as i32 {
                end = end.offset(1);
                end;
            }
            len = end.offset_from(start) as i64 as i32;
            if len > 0 as i32 {
                p = emalloc_real(
                    (len + 2 as i32) as size_t,
                    b"init_awkpath\0" as *const u8 as *const i8,
                    b"p\0" as *const u8 as *const i8,
                    b"io.c\0" as *const u8 as *const i8,
                    2993 as i32,
                ) as *mut i8;
                memcpy(p as *mut libc::c_void, start as *const libc::c_void, len as u64);
                if isdirpunct(*end.offset(-(1 as i32) as isize) as i32) == 0 {
                    let fresh11 = len;
                    len = len + 1;
                    *p.offset(fresh11 as isize) = '/' as i32 as i8;
                }
                *p.offset(len as isize) = '\0' as i32 as i8;
                let fresh12 = i;
                i = i + 1;
                let ref mut fresh13 = *((*pi).awkpath).offset(fresh12 as isize);
                *fresh13 = p;
                if len > (*pi).max_pathlen {
                    (*pi).max_pathlen = len;
                }
                start = end;
            } else {
                start = start.offset(1);
                start;
            }
        }
    }
    let ref mut fresh14 = *((*pi).awkpath).offset(i as isize);
    *fresh14 = 0 as *const i8;
}
unsafe extern "C" fn do_find_source(
    mut src: *const i8,
    mut stb: *mut stat,
    mut errcode: *mut i32,
    mut pi: *mut path_info,
) -> *mut i8 {
    let mut path: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    if ispath(src) != 0 {
        path = emalloc_real(
            (strlen(src)).wrapping_add(1 as i32 as u64),
            b"do_find_source\0" as *const u8 as *const i8,
            b"path\0" as *const u8 as *const i8,
            b"io.c\0" as *const u8 as *const i8,
            3025 as i32,
        ) as *mut i8;
        strcpy(path, src);
        if stat(path, stb) == 0 as i32 {
            return path;
        }
        *errcode = *__errno_location();
        pma_free(path as *mut libc::c_void);
        return 0 as *mut i8;
    }
    if ((*pi).awkpath).is_null() {
        init_awkpath(pi);
    }
    path = emalloc_real(
        ((*pi).max_pathlen as u64)
            .wrapping_add(strlen(src))
            .wrapping_add(1 as i32 as u64),
        b"do_find_source\0" as *const u8 as *const i8,
        b"path\0" as *const u8 as *const i8,
        b"io.c\0" as *const u8 as *const i8,
        3037 as i32,
    ) as *mut i8;
    i = 0 as i32;
    while !(*((*pi).awkpath).offset(i as isize)).is_null() {
        if strcmp(*((*pi).awkpath).offset(i as isize), b"./\0" as *const u8 as *const i8)
            == 0 as i32
            || strcmp(
                *((*pi).awkpath).offset(i as isize),
                b".\0" as *const u8 as *const i8,
            ) == 0 as i32
        {
            *path = '\0' as i32 as i8;
        } else {
            strcpy(path, *((*pi).awkpath).offset(i as isize));
        }
        strcat(path, src);
        if stat(path, stb) == 0 as i32 {
            return path;
        }
        i += 1;
        i;
    }
    *errcode = *__errno_location();
    pma_free(path as *mut libc::c_void);
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn find_source(
    mut src: *const i8,
    mut stb: *mut stat,
    mut errcode: *mut i32,
    mut is_extlib: i32,
) -> *mut i8 {
    let mut path: *mut i8 = 0 as *mut i8;
    let mut pi: *mut path_info = if is_extlib != 0 {
        &mut pi_awklibpath
    } else {
        &mut pi_awkpath
    };
    *errcode = 0 as i32;
    if src.is_null() || *src as i32 == '\0' as i32 {
        return 0 as *mut i8;
    }
    path = do_find_source(src, stb, errcode, pi);
    if path.is_null() && is_extlib != 0 {
        let mut file_ext: *mut i8 = 0 as *mut i8;
        let mut save_errno: i32 = 0;
        let mut src_len: size_t = 0;
        let mut suffix_len: size_t = 0;
        src_len = strlen(src);
        suffix_len = strlen(b".so\0" as *const u8 as *const i8);
        if src_len >= suffix_len
            && strcmp(
                &*src.offset(src_len.wrapping_sub(suffix_len) as isize),
                b".so\0" as *const u8 as *const i8,
            ) == 0 as i32
        {
            return 0 as *mut i8;
        }
        save_errno = *__errno_location();
        file_ext = emalloc_real(
            src_len.wrapping_add(suffix_len).wrapping_add(1 as i32 as u64),
            b"find_source\0" as *const u8 as *const i8,
            b"file_ext\0" as *const u8 as *const i8,
            b"io.c\0" as *const u8 as *const i8,
            3083 as i32,
        ) as *mut i8;
        sprintf(
            file_ext,
            b"%s%s\0" as *const u8 as *const i8,
            src,
            b".so\0" as *const u8 as *const i8,
        );
        path = do_find_source(file_ext, stb, errcode, pi);
        pma_free(file_ext as *mut libc::c_void);
        if path.is_null() {
            *__errno_location() = save_errno;
        }
        return path;
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
        && path.is_null()
    {
        let mut file_awk: *mut i8 = 0 as *mut i8;
        let mut save_errno_0: i32 = *__errno_location();
        file_awk = emalloc_real(
            (strlen(src))
                .wrapping_add(::core::mem::size_of::<[i8; 5]>() as u64)
                .wrapping_add(1 as i32 as u64),
            b"find_source\0" as *const u8 as *const i8,
            b"file_awk\0" as *const u8 as *const i8,
            b"io.c\0" as *const u8 as *const i8,
            3111 as i32,
        ) as *mut i8;
        sprintf(
            file_awk,
            b"%s%s\0" as *const u8 as *const i8,
            src,
            b".awk\0" as *const u8 as *const i8,
        );
        path = do_find_source(file_awk, stb, errcode, pi);
        pma_free(file_awk as *mut libc::c_void);
        if path.is_null() {
            *__errno_location() = save_errno_0;
        }
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn srcopen(mut s: *mut SRCFILE) -> i32 {
    let mut fd: i32 = -(1 as i32);
    if (*s).stype as u32 == srctype::SRC_STDIN as i32 as u32 {
        fd = fileno(stdin);
    } else if (*s).stype as u32 == srctype::SRC_FILE as i32 as u32
        || (*s).stype as u32 == srctype::SRC_INC as i32 as u32
    {
        fd = devopen((*s).fullpath, b"r\0" as *const u8 as *const i8);
    }
    if fd != -(1 as i32) {
        os_setbinmode(fd, 0 as i32);
    }
    return fd;
}
static mut ip_head: *mut awk_input_parser_t = 0 as *const awk_input_parser_t
    as *mut awk_input_parser_t;
static mut ip_tail: *mut awk_input_parser_t = 0 as *const awk_input_parser_t
    as *mut awk_input_parser_t;
#[no_mangle]
pub unsafe extern "C" fn register_input_parser(
    mut input_parser: *mut awk_input_parser_t,
) {
    if input_parser.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 3162 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"register_input_parser: received NULL pointer\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    (*input_parser).next = 0 as *mut awk_input_parser;
    if ip_head.is_null() {
        ip_tail = input_parser;
        ip_head = ip_tail;
    } else {
        (*ip_tail).next = input_parser;
        ip_tail = (*ip_tail).next;
    };
}
unsafe extern "C" fn find_input_parser(mut iop: *mut IOBUF) {
    let mut ip: *mut awk_input_parser_t = 0 as *mut awk_input_parser_t;
    let mut ip2: *mut awk_input_parser_t = 0 as *mut awk_input_parser_t;
    if ((*iop).public.get_record).is_some()
        || (*iop).public.read_func
            != Some(
                read as unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t,
            )
    {
        return;
    }
    ip2 = 0 as *mut awk_input_parser_t;
    ip = ip2;
    ip2 = ip_head;
    while !ip2.is_null() {
        if ((*ip2).can_take_file).expect("non-null function pointer")(&mut (*iop).public)
            as u64 != 0
        {
            if ip.is_null() {
                ip = ip2;
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 3190 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"input parser `%s' conflicts with previously installed input parser `%s'\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*ip2).name,
                    (*ip).name,
                );
            }
        }
        ip2 = (*ip2).next;
    }
    if !ip.is_null() {
        if ((*ip).take_control_of)
            .expect("non-null function pointer")(&mut (*iop).public) as u64 == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 3197 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"input parser `%s' failed to open `%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*ip).name,
                (*iop).public.name,
            );
        } else {
            (*iop).valid = 1 as i32 != 0;
        }
    }
}
static mut op_head: *mut awk_output_wrapper_t = 0 as *const awk_output_wrapper_t
    as *mut awk_output_wrapper_t;
static mut op_tail: *mut awk_output_wrapper_t = 0 as *const awk_output_wrapper_t
    as *mut awk_output_wrapper_t;
#[no_mangle]
pub unsafe extern "C" fn register_output_wrapper(
    mut wrapper: *mut awk_output_wrapper_t,
) {
    if wrapper.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 3217 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"register_output_wrapper: received NULL pointer\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    (*wrapper).next = 0 as *mut awk_output_wrapper;
    if op_head.is_null() {
        op_tail = wrapper;
        op_head = op_tail;
    } else {
        (*op_tail).next = wrapper;
        op_tail = (*op_tail).next;
    };
}
unsafe extern "C" fn find_output_wrapper(mut outbuf: *mut awk_output_buf_t) -> bool {
    let mut op: *mut awk_output_wrapper_t = 0 as *mut awk_output_wrapper_t;
    let mut op2: *mut awk_output_wrapper_t = 0 as *mut awk_output_wrapper_t;
    if (*outbuf).redirected as u64 != 0 {
        return 0 as i32 != 0;
    }
    op2 = 0 as *mut awk_output_wrapper_t;
    op = op2;
    op2 = op_head;
    while !op2.is_null() {
        if ((*op2).can_take_file).expect("non-null function pointer")(outbuf) as u64 != 0
        {
            if op.is_null() {
                op = op2;
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 3245 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"output wrapper `%s' conflicts with previously installed output wrapper `%s'\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*op2).name,
                    (*op).name,
                );
            }
        }
        op2 = (*op2).next;
    }
    if !op.is_null() {
        if ((*op).take_control_of).expect("non-null function pointer")(outbuf) as u64
            == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 3252 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"output wrapper `%s' failed to open `%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*op).name,
                (*outbuf).name,
            );
            return 0 as i32 != 0;
        }
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
static mut tw_head: *mut awk_two_way_processor_t = 0 as *const awk_two_way_processor_t
    as *mut awk_two_way_processor_t;
static mut tw_tail: *mut awk_two_way_processor_t = 0 as *const awk_two_way_processor_t
    as *mut awk_two_way_processor_t;
#[no_mangle]
pub unsafe extern "C" fn register_two_way_processor(
    mut processor: *mut awk_two_way_processor_t,
) {
    if processor.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 3273 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"register_output_processor: received NULL pointer\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    (*processor).next = 0 as *mut awk_two_way_processor;
    if tw_head.is_null() {
        tw_tail = processor;
        tw_head = tw_tail;
    } else {
        (*tw_tail).next = processor;
        tw_tail = (*tw_tail).next;
    };
}
unsafe extern "C" fn find_two_way_processor(
    mut name: *const i8,
    mut rp: *mut redirect,
) -> bool {
    let mut tw: *mut awk_two_way_processor_t = 0 as *mut awk_two_way_processor_t;
    let mut tw2: *mut awk_two_way_processor_t = 0 as *mut awk_two_way_processor_t;
    if !((*rp).iop).is_null() && (*(*rp).iop).public.fd != -(1 as i32)
        || !((*rp).output.fp).is_null()
    {
        return 0 as i32 != 0;
    }
    tw2 = 0 as *mut awk_two_way_processor_t;
    tw = tw2;
    tw2 = tw_head;
    while !tw2.is_null() {
        if ((*tw2).can_take_two_way).expect("non-null function pointer")(name) as u64
            != 0
        {
            if tw.is_null() {
                tw = tw2;
            } else {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 3302 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"two-way processor `%s' conflicts with previously installed two-way processor `%s'\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*tw2).name,
                    (*tw).name,
                );
            }
        }
        tw2 = (*tw2).next;
    }
    if !tw.is_null() {
        if ((*rp).iop).is_null() {
            (*rp).iop = iop_alloc(-(1 as i32), name, 0 as i32);
        }
        if ((*tw).take_control_of)
            .expect(
                "non-null function pointer",
            )(name, &mut (*(*rp).iop).public, &mut (*rp).output) as u64 == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"io.c\0" as *const u8 as *const i8, 3311 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"two way processor `%s' failed to open `%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*tw).name,
                name,
            );
            return 0 as i32 != 0;
        }
        iop_finish((*rp).iop);
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn iop_alloc(
    mut fd: i32,
    mut name: *const i8,
    mut errno_val: i32,
) -> *mut IOBUF {
    let mut iop: *mut IOBUF = 0 as *mut IOBUF;
    iop = ezalloc_real(
        ::core::mem::size_of::<IOBUF>() as u64,
        b"iop_alloc\0" as *const u8 as *const i8,
        b"iop\0" as *const u8 as *const i8,
        b"io.c\0" as *const u8 as *const i8,
        3370 as i32,
    ) as *mut IOBUF;
    (*iop).public.fd = fd;
    (*iop).public.name = name;
    (*iop).public.read_func = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ssize_t>,
        Option<unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t>,
    >(
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t>,
            Option<unsafe extern "C" fn() -> ssize_t>,
        >(Some(read as unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t)),
    );
    (*iop).valid = 0 as i32 != 0;
    (*iop).errcode = errno_val;
    if fd != -(1 as i32) {
        fstat(fd, &mut (*iop).public.sbuf);
    } else {
        let mut statf: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32> = Some(
            lstat as unsafe extern "C" fn(*const i8, *mut stat) -> i32,
        );
        if statf.expect("non-null function pointer")(name, &mut (*iop).public.sbuf)
            < 0 as i32
        {
            memset(
                &mut (*iop).public.sbuf as *mut stat as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<stat>() as u64,
            );
        }
    }
    return iop;
}
unsafe extern "C" fn iop_finish(mut iop: *mut IOBUF) -> *mut IOBUF {
    let mut isdir: bool = 0 as i32 != 0;
    if (*iop).public.fd != -(1 as i32) {
        if os_isreadable(&mut (*iop).public, &mut isdir) != 0 {
            (*iop).valid = 1 as i32 != 0;
        } else if isdir {
            (*iop).errcode = 21 as i32;
        } else {
            (*iop).errcode = 5 as i32;
            if fcntl((*iop).public.fd, 3 as i32) >= 0 as i32 {
                close((*iop).public.fd);
            }
            (*iop).public.fd = -(1 as i32);
        }
    }
    if !(*iop).valid || (*iop).public.fd == -(1 as i32) {
        return iop;
    }
    if os_isatty((*iop).public.fd) != 0 {
        (*iop).flag = ::core::mem::transmute::<
            u32,
            iobuf_flags,
        >((*iop).flag as u32 | iobuf_flags::IOP_IS_TTY as i32 as u32);
    }
    (*iop).size = optimal_bufsize((*iop).public.fd, &mut (*iop).public.sbuf);
    (*iop).readsize = (*iop).size;
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*iop).public.sbuf.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
        && (*iop).public.sbuf.st_size == 0 as i32 as i64
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 3442 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"data file `%s' is empty\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*iop).public.name,
        );
    }
    let ref mut fresh15 = *__errno_location();
    *fresh15 = 0 as i32;
    (*iop).errcode = *fresh15;
    (*iop).scanoff = 0 as i32 as size_t;
    (*iop).count = (*iop).scanoff as ssize_t;
    (*iop).size = ((*iop).size as u64).wrapping_add(1 as i32 as u64) as size_t as size_t;
    (*iop).buf = emalloc_real(
        (*iop).size,
        b"iop_finish\0" as *const u8 as *const i8,
        b"iop->buf\0" as *const u8 as *const i8,
        b"io.c\0" as *const u8 as *const i8,
        3445 as i32,
    ) as *mut i8;
    (*iop).off = (*iop).buf;
    (*iop).dataend = 0 as *mut i8;
    (*iop).end = ((*iop).buf).offset((*iop).size as isize);
    (*iop).flag = ::core::mem::transmute::<
        u32,
        iobuf_flags,
    >((*iop).flag as u32 | iobuf_flags::IOP_AT_START as i32 as u32);
    return iop;
}
unsafe extern "C" fn grow_iop_buffer(mut iop: *mut IOBUF) {
    let mut valid: size_t = ((*iop).dataend).offset_from((*iop).off) as i64 as size_t;
    let mut off: size_t = ((*iop).off).offset_from((*iop).buf) as i64 as size_t;
    let mut newsize: size_t = 0;
    newsize = ((*iop).size)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_mul(2 as i32 as u64)
        .wrapping_add(1 as i32 as u64);
    if newsize <= (*iop).size {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 3484 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"could not allocate more input memory\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if newsize.wrapping_sub(valid) < (*iop).readsize {
        newsize = (newsize as u64)
            .wrapping_add(((*iop).readsize).wrapping_add(1 as i32 as u64)) as size_t
            as size_t;
    }
    if newsize <= (*iop).size {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"io.c\0" as *const u8 as *const i8, 3492 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"could not allocate more input memory\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    (*iop).size = newsize;
    (*iop).buf = erealloc_real(
        (*iop).buf as *mut libc::c_void,
        (*iop).size,
        b"grow_iop_buffer\0" as *const u8 as *const i8,
        b"iop->buf\0" as *const u8 as *const i8,
        b"io.c\0" as *const u8 as *const i8,
        3495 as i32,
    ) as *mut i8;
    (*iop).off = ((*iop).buf).offset(off as isize);
    (*iop).dataend = ((*iop).off).offset(valid as isize);
    (*iop).end = ((*iop).buf).offset((*iop).size as isize);
}
unsafe extern "C" fn rs1scan(
    mut iop: *mut IOBUF,
    mut recm: *mut recmatch,
    mut state: *mut SCANSTATE,
) -> RECVALUE {
    let mut bp: *mut i8 = 0 as *mut i8;
    let mut rs: i8 = 0;
    let mut mbclen: size_t = 0 as i32 as size_t;
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    memset(
        recm as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<recmatch>() as u64,
    );
    rs = *((*RS).sub.val.sp).offset(0 as i32 as isize);
    *(*iop).dataend = rs;
    (*recm).start = (*iop).off;
    bp = (*iop).off;
    if *state as u32 == scanstate::INDATA as i32 as u32 {
        bp = bp.offset((*iop).scanoff as isize);
    }
    if rs as i32 != '\n' as i32 && gawk_mb_cur_max > 1 as i32 {
        let mut len: i32 = ((*iop).dataend).offset_from(bp) as i64 as i32;
        let mut found: bool = 0 as i32 != 0;
        memset(
            &mut mbs as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        loop {
            if *bp as i32 == rs as i32 {
                found = 1 as i32 != 0;
            }
            if *btowc_cache.as_mut_ptr().offset((*bp as i32 & 0xff as i32) as isize)
                != 0xffffffff as u32
            {
                mbclen = 1 as i32 as size_t;
            } else {
                mbclen = mbrlen(bp, len as size_t, &mut mbs);
            }
            if mbclen == 1 as i32 as u64 || mbclen == -(1 as i32) as size_t
                || mbclen == -(2 as i32) as size_t || mbclen == 0 as i32 as u64
            {
                mbclen = 1 as i32 as size_t;
            }
            len = (len as u64).wrapping_sub(mbclen) as i32 as i32;
            bp = bp.offset(mbclen as isize);
            if !(len > 0 as i32 && !found) {
                break;
            }
        }
        if found as i32 != 0 && bp.offset(-(mbclen as isize)) < (*iop).dataend {
            (*recm).len = (bp.offset_from((*recm).start) as i64 as u64)
                .wrapping_sub(mbclen);
            (*recm).rt_start = bp.offset(-(mbclen as isize));
            (*recm).rt_len = mbclen;
            *state = scanstate::NOSTATE;
            return recvalues::REC_OK;
        } else {
            (*recm).len = bp.offset_from((*recm).start) as i64 as size_t;
            *state = scanstate::INDATA;
            (*iop).scanoff = bp.offset_from((*iop).off) as i64 as size_t;
            return recvalues::NOTERM;
        }
    }
    while *bp as i32 != rs as i32 {
        bp = bp.offset(1);
        bp;
    }
    (*recm).len = bp.offset_from((*recm).start) as i64 as size_t;
    if bp < (*iop).dataend {
        (*recm).rt_start = bp;
        (*recm).rt_len = 1 as i32 as size_t;
        *state = scanstate::NOSTATE;
        return recvalues::REC_OK;
    } else {
        *state = scanstate::INDATA;
        (*iop).scanoff = bp.offset_from((*iop).off) as i64 as size_t;
        return recvalues::NOTERM;
    };
}
unsafe extern "C" fn rsrescan(
    mut iop: *mut IOBUF,
    mut recm: *mut recmatch,
    mut state: *mut SCANSTATE,
) -> RECVALUE {
    let mut bp: *mut i8 = 0 as *mut i8;
    let mut restart: size_t = 0 as i32 as size_t;
    let mut reend: size_t = 0 as i32 as size_t;
    let mut RSre: *mut Regexp = RS_regexp;
    let mut regex_flags: i32 = 1 as i32;
    memset(
        recm as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<recmatch>() as u64,
    );
    (*recm).start = (*iop).off;
    bp = (*iop).off;
    if *state as u32 == scanstate::INDATA as i32 as u32 {
        bp = bp.offset((*iop).scanoff as isize);
    }
    if (*iop).flag as u32 & iobuf_flags::IOP_AT_START as i32 as u32 == 0 as i32 as u32 {
        regex_flags |= 2 as i32;
    }
    loop {
        if research(
            RSre,
            bp,
            0 as i32,
            ((*iop).dataend).offset_from(bp) as i64 as size_t,
            regex_flags,
        ) == -(1 as i32)
        {
            (*recm).len = ((*iop).dataend).offset_from((*iop).off) as i64 as size_t;
            return recvalues::NOTERM;
        }
        restart = *((*RSre).regs.start).offset(0 as i32 as isize) as size_t;
        reend = *((*RSre).regs.end).offset(0 as i32 as isize) as size_t;
        if restart == reend {
            *state = scanstate::INDATA;
            (*iop).scanoff = reend.wrapping_add(1 as i32 as u64);
            if bp.offset((*iop).scanoff as isize) <= (*iop).dataend {
                bp = bp.offset((*iop).scanoff as isize);
            } else {
                (*recm).len = (bp.offset_from((*iop).off) as i64 as u64)
                    .wrapping_add(restart);
                return recvalues::NOTERM;
            }
        } else {
            (*recm).len = restart;
            (*recm).rt_start = bp.offset(restart as isize);
            (*recm).rt_len = reend.wrapping_sub(restart);
            *state = scanstate::NOSTATE;
            if ((*iop).off).offset(reend as isize) >= (*iop).dataend {
                if reisstring((*RS).sub.val.sp, (*RS).sub.val.slen, RSre, (*iop).off)
                    != 0
                {
                    return recvalues::REC_OK
                } else {
                    return recvalues::TERMATEND
                }
            }
            if (*RSre).maybe_long {
                let mut matchend: *mut i8 = ((*iop).off).offset(reend as isize);
                if (((*iop).dataend).offset_from(matchend) as i64 as u64)
                    < (*RS).sub.val.slen
                {
                    return recvalues::TERMNEAREND;
                }
            }
            return recvalues::REC_OK;
        }
    };
}
unsafe extern "C" fn rsnullscan(
    mut iop: *mut IOBUF,
    mut recm: *mut recmatch,
    mut state: *mut SCANSTATE,
) -> RECVALUE {
    let mut bp: *mut i8 = 0 as *mut i8;
    if *state as u32 == scanstate::NOSTATE as i32 as u32
        || *state as u32 == scanstate::INLEADER as i32 as u32
    {
        memset(
            recm as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<recmatch>() as u64,
        );
    }
    (*recm).start = (*iop).off;
    bp = (*iop).off;
    if *state as u32 != scanstate::NOSTATE as i32 as u32 {
        bp = bp.offset((*iop).scanoff as isize);
    }
    *(*iop).dataend = '\n' as i32 as i8;
    if !(*state as u32 == scanstate::INTERM as i32 as u32) {
        if !(*state as u32 == scanstate::INDATA as i32 as u32) {
            while *bp as i32 == '\n' as i32 && bp < (*iop).dataend {
                bp = bp.offset(1);
                bp;
            }
            if bp >= (*iop).dataend {
                *state = scanstate::INLEADER;
                (*iop).scanoff = bp.offset_from((*iop).off) as i64 as size_t;
                return recvalues::NOTERM;
            }
            (*recm).start = bp;
            (*iop).off = (*recm).start;
        }
        loop {
            loop {
                let fresh16 = bp;
                bp = bp.offset(1);
                if !(*fresh16 as i32 != '\n' as i32) {
                    break;
                }
            }
            if bp >= (*iop).dataend {
                (*recm).len = (bp.offset_from((*iop).off) as i64 - 1 as i32 as i64)
                    as size_t;
                (*iop).scanoff = (*recm).len;
                if bp == (*iop).dataend {
                    (*recm).rt_start = bp.offset(-(1 as i32 as isize));
                    (*recm).rt_len = 1 as i32 as size_t;
                }
                *state = scanstate::INDATA;
                return recvalues::NOTERM;
            }
            if !(*bp as i32 != '\n' as i32) {
                break;
            }
        }
        *state = scanstate::INTERM;
        (*recm).len = (bp.offset_from((*iop).off) as i64 - 1 as i32 as i64) as size_t;
        (*recm).rt_start = bp.offset(-(1 as i32 as isize));
    }
    while *bp as i32 == '\n' as i32 && bp < (*iop).dataend {
        bp = bp.offset(1);
        bp;
    }
    (*recm).rt_len = bp.offset_from((*recm).rt_start) as i64 as size_t;
    (*iop).scanoff = bp.offset_from((*iop).off) as i64 as size_t;
    if bp >= (*iop).dataend {
        return recvalues::TERMATEND;
    }
    return recvalues::REC_OK;
}
#[inline]
unsafe extern "C" fn retryable(mut iop: *mut IOBUF) -> i32 {
    return (!PROCINFO_node.is_null()
        && !(in_PROCINFO(
            (*iop).public.name,
            b"RETRY\0" as *const u8 as *const i8,
            0 as *mut *mut NODE,
        ))
            .is_null()) as i32;
}
#[inline]
unsafe extern "C" fn errno_io_retry() -> i32 {
    match *__errno_location() {
        11 | 4 | 110 => return 1 as i32,
        _ => return 0 as i32,
    };
}
unsafe extern "C" fn get_a_record(
    mut out: *mut *mut i8,
    mut len: *mut size_t,
    mut iop: *mut IOBUF,
    mut errcode: *mut i32,
    mut field_width: *mut *const awk_fieldwidth_info_t,
) -> i32 {
    let mut recm: recmatch = recmatch {
        start: 0 as *mut i8,
        len: 0,
        rt_start: 0 as *mut i8,
        rt_len: 0,
    };
    let mut state: SCANSTATE = scanstate::NOSTATE;
    let mut ret: RECVALUE = recvalues::REC_OK;
    let mut rtval: *mut NODE = 0 as *mut NODE;
    static mut lastmatchrec: Option<
        unsafe extern "C" fn(*mut IOBUF, *mut recmatch, *mut SCANSTATE) -> RECVALUE,
    > = None;
    if (*iop).flag as u32 & iobuf_flags::IOP_AT_EOF as i32 as u32 != 0 as i32 as u32
        && (*iop).off >= (*iop).dataend
    {
        return -(1 as i32);
    }
    if read_can_timeout {
        read_timeout = get_read_timeout(iop);
    }
    if ((*iop).public.get_record).is_some() {
        let mut rt_start: *mut i8 = 0 as *mut i8;
        let mut rt_len: size_t = 0;
        let mut rc: i32 = ((*iop).public.get_record)
            .expect(
                "non-null function pointer",
            )(out, &mut (*iop).public, errcode, &mut rt_start, &mut rt_len, field_width);
        if rc == -(1 as i32) {
            (*iop).flag = ::core::mem::transmute::<
                u32,
                iobuf_flags,
            >((*iop).flag as u32 | iobuf_flags::IOP_AT_EOF as i32 as u32);
        } else {
            *len = rc as size_t;
            rc = 0 as i32;
            if rt_len != 0 as i32 as u64 {
                (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                    && {
                        unref((*RT_node).sub.nodep.l.lptr);
                        (*RT_node).sub.nodep.l.lptr = make_str_node(
                            rt_start,
                            rt_len,
                            0 as i32,
                        );
                        !((*RT_node).sub.nodep.l.lptr).is_null()
                    }) as i32;
            } else {
                (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                    && {
                        unref((*RT_node).sub.nodep.l.lptr);
                        (*RT_node).sub.nodep.l.lptr = dupnode(Nnull_string);
                        !((*RT_node).sub.nodep.l.lptr).is_null()
                    }) as i32;
            }
        }
        return rc;
    }
    if ((*iop).dataend).is_null() || (*iop).off >= (*iop).dataend {
        (*iop).count = ((*iop).public.read_func)
            .expect(
                "non-null function pointer",
            )((*iop).public.fd, (*iop).buf as *mut libc::c_void, (*iop).readsize);
        if (*iop).count == 0 as i32 as i64 {
            (*iop).flag = ::core::mem::transmute::<
                u32,
                iobuf_flags,
            >((*iop).flag as u32 | iobuf_flags::IOP_AT_EOF as i32 as u32);
            return -(1 as i32);
        } else if (*iop).count == -(1 as i32) as i64 {
            *errcode = *__errno_location();
            if errno_io_retry() != 0 && retryable(iop) != 0 {
                return -(2 as i32);
            }
            (*iop).flag = ::core::mem::transmute::<
                u32,
                iobuf_flags,
            >((*iop).flag as u32 | iobuf_flags::IOP_AT_EOF as i32 as u32);
            return -(1 as i32);
        } else {
            (*iop).dataend = ((*iop).buf).offset((*iop).count as isize);
            (*iop).off = (*iop).buf;
        }
    }
    state = scanstate::NOSTATE;
    loop {
        let mut dataend_off: size_t = 0;
        let mut room_left: size_t = 0;
        let mut amt_to_read: size_t = 0;
        ret = (Some(matchrec.expect("non-null function pointer")))
            .expect("non-null function pointer")(iop, &mut recm, &mut state);
        (*iop).flag = ::core::mem::transmute::<
            u32,
            iobuf_flags,
        >((*iop).flag as u32 & !(iobuf_flags::IOP_AT_START as i32) as u32);
        if ret as u32 == recvalues::REC_OK as i32 as u32 {
            break;
        }
        if ret as u32 == recvalues::TERMNEAREND as i32 as u32
            && ((*iop).dataend).offset_from((*iop).off) as i64
                == (*iop).public.sbuf.st_size
        {
            break;
        }
        dataend_off = ((*iop).dataend).offset_from((*iop).off) as i64 as size_t;
        memmove(
            (*iop).buf as *mut libc::c_void,
            (*iop).off as *const libc::c_void,
            dataend_off,
        );
        (*iop).off = (*iop).buf;
        (*iop).dataend = ((*iop).buf).offset(dataend_off as isize);
        recm.start = (*iop).off;
        if !(recm.rt_start).is_null() {
            recm.rt_start = ((*iop).off).offset(recm.len as isize);
        }
        room_left = (((*iop).end).offset_from((*iop).dataend) as i64 - 1 as i32 as i64)
            as size_t;
        amt_to_read = if (*iop).readsize < room_left {
            (*iop).readsize
        } else {
            room_left
        };
        if amt_to_read < (*iop).readsize {
            grow_iop_buffer(iop);
            recm.start = (*iop).off;
            if !(recm.rt_start).is_null() {
                recm.rt_start = ((*iop).off).offset(recm.len as isize);
            }
            room_left = (((*iop).end).offset_from((*iop).dataend) as i64
                - 1 as i32 as i64) as size_t;
            amt_to_read = if (*iop).readsize < room_left {
                (*iop).readsize
            } else {
                room_left
            };
        }
        while amt_to_read.wrapping_add((*iop).readsize) < room_left {
            amt_to_read = (amt_to_read as u64).wrapping_add((*iop).readsize) as size_t
                as size_t;
        }
        amt_to_read = if amt_to_read < 9223372036854775807 as i64 as u64 {
            amt_to_read
        } else {
            9223372036854775807 as i64 as u64
        };
        (*iop).count = ((*iop).public.read_func)
            .expect(
                "non-null function pointer",
            )((*iop).public.fd, (*iop).dataend as *mut libc::c_void, amt_to_read);
        if (*iop).count == -(1 as i32) as i64 {
            *errcode = *__errno_location();
            if errno_io_retry() != 0 && retryable(iop) != 0 {
                return -(2 as i32);
            }
            (*iop).flag = ::core::mem::transmute::<
                u32,
                iobuf_flags,
            >((*iop).flag as u32 | iobuf_flags::IOP_AT_EOF as i32 as u32);
            break;
        } else if (*iop).count == 0 as i32 as i64 {
            if ret as u32 != recvalues::TERMNEAREND as i32 as u32 {
                (*iop).flag = ::core::mem::transmute::<
                    u32,
                    iobuf_flags,
                >((*iop).flag as u32 | iobuf_flags::IOP_AT_EOF as i32 as u32);
            }
            break;
        } else {
            (*iop).dataend = ((*iop).dataend).offset((*iop).count as isize);
        }
    }
    rtval = (*RT_node).sub.nodep.l.lptr;
    if recm.rt_len == 0 as i32 as u64 {
        (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && {
                unref((*RT_node).sub.nodep.l.lptr);
                (*RT_node).sub.nodep.l.lptr = dupnode(Nnull_string);
                !((*RT_node).sub.nodep.l.lptr).is_null()
            }) as i32;
        lastmatchrec = None;
    } else if lastmatchrec.is_none() || lastmatchrec != matchrec {
        lastmatchrec = matchrec;
        (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && {
                unref((*RT_node).sub.nodep.l.lptr);
                (*RT_node).sub.nodep.l.lptr = make_str_node(
                    recm.rt_start,
                    recm.rt_len,
                    0 as i32,
                );
                !((*RT_node).sub.nodep.l.lptr).is_null()
            }) as i32;
    } else if matchrec
        == Some(
            rs1scan
                as unsafe extern "C" fn(
                    *mut IOBUF,
                    *mut recmatch,
                    *mut SCANSTATE,
                ) -> RECVALUE,
        )
    {
        if (*rtval).sub.val.slen != 1 as i32 as u64
            || *((*rtval).sub.val.sp).offset(0 as i32 as isize) as i32
                != *(recm.rt_start).offset(0 as i32 as isize) as i32
        {
            (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                && {
                    unref((*RT_node).sub.nodep.l.lptr);
                    (*RT_node).sub.nodep.l.lptr = make_str_node(
                        recm.rt_start,
                        recm.rt_len,
                        0 as i32,
                    );
                    !((*RT_node).sub.nodep.l.lptr).is_null()
                }) as i32;
        }
    } else if matchrec
        == Some(
            rsnullscan
                as unsafe extern "C" fn(
                    *mut IOBUF,
                    *mut recmatch,
                    *mut SCANSTATE,
                ) -> RECVALUE,
        )
    {
        if (*rtval).sub.val.slen >= recm.rt_len {
            (*rtval).sub.val.slen = recm.rt_len;
            if (*rtval).flags as u32 & flagvals::WSTRCUR as i32 as u32 != 0 {
                r_free_wstr(rtval);
            }
        } else {
            (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                && {
                    unref((*RT_node).sub.nodep.l.lptr);
                    (*RT_node).sub.nodep.l.lptr = make_str_node(
                        recm.rt_start,
                        recm.rt_len,
                        0 as i32,
                    );
                    !((*RT_node).sub.nodep.l.lptr).is_null()
                }) as i32;
        }
    } else {
        (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && {
                unref((*RT_node).sub.nodep.l.lptr);
                (*RT_node).sub.nodep.l.lptr = make_str_node(
                    recm.rt_start,
                    recm.rt_len,
                    0 as i32,
                );
                !((*RT_node).sub.nodep.l.lptr).is_null()
            }) as i32;
    }
    if recm.len == 0 as i32 as u64 {
        *out = 0 as *mut i8;
        *len = 0 as i32 as size_t;
    } else {
        *out = recm.start;
        *len = recm.len;
    }
    (*iop).off = ((*iop).off).offset((recm.len).wrapping_add(recm.rt_len) as isize);
    if recm.len == 0 as i32 as u64 && recm.rt_len == 0 as i32 as u64
        && (*iop).flag as u32 & iobuf_flags::IOP_AT_EOF as i32 as u32 != 0 as i32 as u32
    {
        return -(1 as i32)
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_RS() {
    static mut save_rs: *mut NODE = 0 as *const NODE as *mut NODE;
    if !save_rs.is_null()
        && (*(*RS_node).sub.nodep.l.lptr).sub.val.slen == (*save_rs).sub.val.slen
        && memcmp(
            (*(*RS_node).sub.nodep.l.lptr).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.sp as *const libc::c_void,
            (*save_rs).sub.val.slen,
        ) == 0 as i32
    {
        RS_regexp = RS_re[IGNORECASE as usize];
    } else {
        unref(save_rs);
        save_rs = dupnode((*RS_node).sub.nodep.l.lptr);
        RS_is_null = 0 as i32 != 0;
        RS = force_string_fmt((*RS_node).sub.nodep.l.lptr, CONVFMT, CONVFMTidx);
        refree(RS_re[0 as i32 as usize]);
        refree(RS_re[1 as i32 as usize]);
        RS_regexp = 0 as *mut Regexp;
        RS_re[1 as i32 as usize] = RS_regexp;
        RS_re[0 as i32 as usize] = RS_re[1 as i32 as usize];
        if (*RS).sub.val.slen == 0 as i32 as u64 {
            RS_is_null = 1 as i32 != 0;
            matchrec = Some(
                rsnullscan
                    as unsafe extern "C" fn(
                        *mut IOBUF,
                        *mut recmatch,
                        *mut SCANSTATE,
                    ) -> RECVALUE,
            );
        } else if ((*RS).sub.val.slen > 1 as i32 as u64
            || (*RS).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32)
            && do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
        {
            static mut warned: bool = 0 as i32 != 0;
            RS_re[0 as i32 as usize] = make_regexp(
                (*RS).sub.val.sp,
                (*RS).sub.val.slen,
                0 as i32 != 0,
                1 as i32 != 0,
                1 as i32 != 0,
            );
            RS_re[1 as i32 as usize] = make_regexp(
                (*RS).sub.val.sp,
                (*RS).sub.val.slen,
                1 as i32 != 0,
                1 as i32 != 0,
                1 as i32 != 0,
            );
            RS_regexp = RS_re[IGNORECASE as usize];
            matchrec = Some(
                rsrescan
                    as unsafe extern "C" fn(
                        *mut IOBUF,
                        *mut recmatch,
                        *mut SCANSTATE,
                    ) -> RECVALUE,
            );
            if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
                && !warned
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"io.c\0" as *const u8 as *const i8, 4113 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"multicharacter value of `RS' is a gawk extension\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                warned = 1 as i32 != 0;
            }
        } else {
            matchrec = Some(
                rs1scan
                    as unsafe extern "C" fn(
                        *mut IOBUF,
                        *mut recmatch,
                        *mut SCANSTATE,
                    ) -> RECVALUE,
            );
        }
    }
    if current_field_sep() as u32 == field_sep_type::Using_FS as i32 as u32 {
        set_FS();
    }
}
unsafe extern "C" fn pty_vs_pipe(mut command: *const i8) -> bool {
    let mut val: *mut NODE = 0 as *mut NODE;
    val = in_PROCINFO(command, b"pty\0" as *const u8 as *const i8, 0 as *mut *mut NODE);
    if !val.is_null() {
        return boolval(val);
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn iopflags2str(mut flag: i32) -> *const i8 {
    static mut values: [flagtab; 5] = [
        {
            let mut init = flagtab {
                val: iobuf_flags::IOP_IS_TTY as i32,
                name: b"iobuf_flags::IOP_IS_TTY\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: iobuf_flags::IOP_AT_EOF as i32,
                name: b"iobuf_flags::IOP_AT_EOF\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: iobuf_flags::IOP_CLOSED as i32,
                name: b"iobuf_flags::IOP_CLOSED\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: iobuf_flags::IOP_AT_START as i32,
                name: b"iobuf_flags::IOP_AT_START\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = flagtab {
                val: 0 as i32,
                name: 0 as *const i8,
            };
            init
        },
    ];
    return genflags2str(flag, values.as_ptr());
}
unsafe extern "C" fn free_rp(mut rp: *mut redirect) {
    pma_free((*rp).value as *mut libc::c_void);
    pma_free(rp as *mut libc::c_void);
}
unsafe extern "C" fn inetfile(
    mut str: *const i8,
    mut len: size_t,
    mut isi: *mut inet_socket_info,
) -> bool {
    let mut cp: *const i8 = str;
    let mut cpend: *const i8 = str.offset(len as isize);
    let mut buf: inet_socket_info = inet_socket_info {
        family: 0,
        protocol: 0,
        localport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remotehost: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
        remoteport: C2RustUnnamed_11 {
            offset: 0,
            len: 0,
        },
    };
    if len < 5 as i32 as u64
        || memcmp(
            cp as *const libc::c_void,
            b"/inet\0" as *const u8 as *const i8 as *const libc::c_void,
            5 as i32 as u64,
        ) != 0 as i32
    {
        return 0 as i32 != 0;
    }
    if isi.is_null() {
        isi = &mut buf;
    }
    cp = cp.offset(5 as i32 as isize);
    if (cpend.offset_from(cp) as i64) < 2 as i32 as i64 {
        return 0 as i32 != 0;
    }
    match *cp as i32 {
        47 => {
            (*isi).family = 0 as i32;
        }
        52 => {
            cp = cp.offset(1);
            if *cp as i32 != '/' as i32 {
                return 0 as i32 != 0;
            }
            (*isi).family = 2 as i32;
        }
        54 => {
            cp = cp.offset(1);
            if *cp as i32 != '/' as i32 {
                return 0 as i32 != 0;
            }
            (*isi).family = 10 as i32;
        }
        _ => return 0 as i32 != 0,
    }
    cp = cp.offset(1);
    cp;
    if (cpend.offset_from(cp) as i64) < 5 as i32 as i64 {
        return 0 as i32 != 0;
    }
    if memcmp(
        cp as *const libc::c_void,
        b"tcp/\0" as *const u8 as *const i8 as *const libc::c_void,
        4 as i32 as u64,
    ) == 0 as i32
    {
        (*isi).protocol = __socket_type::SOCK_STREAM as i32;
    } else if memcmp(
        cp as *const libc::c_void,
        b"udp/\0" as *const u8 as *const i8 as *const libc::c_void,
        4 as i32 as u64,
    ) == 0 as i32
    {
        (*isi).protocol = __socket_type::SOCK_DGRAM as i32;
    } else {
        return 0 as i32 != 0
    }
    cp = cp.offset(4 as i32 as isize);
    (*isi).localport.offset = cp.offset_from(str) as i64 as i32;
    while *cp as i32 != '/' as i32 {
        cp = cp.offset(1);
        if cp >= cpend {
            return 0 as i32 != 0;
        }
    }
    (*isi).localport.len = (cp.offset_from(str) as i64 - (*isi).localport.offset as i64)
        as i32;
    if (*isi).localport.len == 0 as i32 {
        return 0 as i32 != 0;
    }
    if (cpend.offset_from(cp) as i64) < 2 as i32 as i64 {
        return 0 as i32 != 0;
    }
    cp = cp.offset(1);
    cp;
    (*isi).remotehost.offset = cp.offset_from(str) as i64 as i32;
    while *cp as i32 != '/' as i32 {
        cp = cp.offset(1);
        if cp >= cpend {
            return 0 as i32 != 0;
        }
    }
    (*isi).remotehost.len = (cp.offset_from(str) as i64
        - (*isi).remotehost.offset as i64) as i32;
    if (*isi).remotehost.len == 0 as i32 {
        return 0 as i32 != 0;
    }
    if (cpend.offset_from(cp) as i64) < 2 as i32 as i64 {
        return 0 as i32 != 0;
    }
    cp = cp.offset(1);
    cp;
    (*isi).remoteport.offset = cp.offset_from(str) as i64 as i32;
    while *cp as i32 != '/' as i32 && cp < cpend {
        cp = cp.offset(1);
        cp;
    }
    if cp != cpend
        || {
            (*isi).remoteport.len = (cp.offset_from(str) as i64
                - (*isi).remoteport.offset as i64) as i32;
            (*isi).remoteport.len == 0 as i32
        }
    {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn in_PROCINFO(
    mut pidx1: *const i8,
    mut pidx2: *const i8,
    mut full_idx: *mut *mut NODE,
) -> *mut NODE {
    let mut str: *mut i8 = 0 as *mut i8;
    let mut str_len: size_t = 0;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut subsep: *mut NODE = (*SUBSEP_node).sub.nodep.l.lptr;
    if PROCINFO_node.is_null() || pidx1.is_null() && pidx2.is_null() {
        return 0 as *mut NODE;
    }
    if !full_idx.is_null() {
        sub = *full_idx;
    }
    if !pidx1.is_null() && pidx2.is_null() {
        str_len = strlen(pidx1);
    } else if pidx1.is_null() && !pidx2.is_null() {
        str_len = strlen(pidx2);
    } else {
        str_len = (strlen(pidx1))
            .wrapping_add((*subsep).sub.val.slen)
            .wrapping_add(strlen(pidx2));
    }
    if sub.is_null() {
        str = emalloc_real(
            str_len.wrapping_add(1 as i32 as u64),
            b"in_PROCINFO\0" as *const u8 as *const i8,
            b"str\0" as *const u8 as *const i8,
            b"io.c\0" as *const u8 as *const i8,
            4302 as i32,
        ) as *mut i8;
        sub = make_str_node(str, str_len, 2 as i32);
        if !full_idx.is_null() {
            *full_idx = sub;
        }
    } else if str_len != (*sub).sub.val.slen {
        (*sub).sub.val.sp = erealloc_real(
            (*sub).sub.val.sp as *mut libc::c_void,
            str_len.wrapping_add(1 as i32 as u64),
            b"in_PROCINFO\0" as *const u8 as *const i8,
            b"sub->stptr\0" as *const u8 as *const i8,
            b"io.c\0" as *const u8 as *const i8,
            4310 as i32,
        ) as *mut i8;
        (*sub).sub.val.slen = str_len;
    }
    if !pidx1.is_null() && pidx2.is_null() {
        strcpy((*sub).sub.val.sp, pidx1);
    } else if pidx1.is_null() && !pidx2.is_null() {
        strcpy((*sub).sub.val.sp, pidx2);
    } else {
        sprintf(
            (*sub).sub.val.sp,
            b"%s%.*s%s\0" as *const u8 as *const i8,
            pidx1,
            (*subsep).sub.val.slen as i32,
            (*subsep).sub.val.sp,
            pidx2,
        );
    }
    r = in_array(PROCINFO_node, sub);
    if full_idx.is_null() {
        unref(sub);
    }
    return r;
}
unsafe extern "C" fn get_read_timeout(mut iop: *mut IOBUF) -> i64 {
    let mut tmout: i64 = 0 as i32 as i64;
    if !PROCINFO_node.is_null() {
        let mut name: *const i8 = (*iop).public.name;
        let mut val: *mut NODE = 0 as *mut NODE;
        static mut full_idx: *mut NODE = 0 as *const NODE as *mut NODE;
        static mut last_name: *const i8 = 0 as *const i8;
        if full_idx.is_null() || strcmp(name, last_name) != 0 as i32 {
            val = in_PROCINFO(
                name,
                b"READ_TIMEOUT\0" as *const u8 as *const i8,
                &mut full_idx,
            );
            if !last_name.is_null() {
                pma_free(last_name as *mut libc::c_void);
            }
            last_name = estrdup(name, strlen(name));
        } else {
            val = in_array(PROCINFO_node, full_idx);
        }
        if !val.is_null() {
            force_number(val);
            tmout = (*val).sub.val.fltnum as i64;
        }
    } else {
        tmout = read_default_timeout;
    }
    if (*iop).public.read_func
        == ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ssize_t>,
            Option<unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t>,
        >(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t>,
                Option<unsafe extern "C" fn() -> ssize_t>,
            >(
                Some(
                    read
                        as unsafe extern "C" fn(
                            i32,
                            *mut libc::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            ),
        ) && tmout > 0 as i32 as i64
    {
        (*iop).public.read_func = Some(
            read_with_timeout
                as unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t,
        );
    }
    return tmout;
}
unsafe extern "C" fn read_with_timeout(
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut s: i32 = fd;
    tv.tv_sec = read_timeout / 1000 as i32 as i64;
    tv.tv_usec = 1000 as i32 as i64 * (read_timeout - 1000 as i32 as i64 * tv.tv_sec);
    let mut __d0: i32 = 0;
    let mut __d1: i32 = 0;
    let fresh17 = &mut __d0;
    let fresh18;
    let fresh19 = (::core::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
    let fresh20 = &mut __d1;
    let fresh21;
    let fresh22 = &mut *(readfds.fds_bits).as_mut_ptr().offset(0 as i32 as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh17,
        fresh19) => fresh18, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh20,
        fresh22) => fresh21, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
    c2rust_asm_casts::AsmCast::cast_out(fresh20, fresh22, fresh21);
    readfds
        .fds_bits[(s / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
        as usize]
        |= ((1 as u64)
            << s % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
            as __fd_mask;
    *__errno_location() = 0 as i32;
    if select(fd + 1 as i32, &mut readfds, 0 as *mut fd_set, 0 as *mut fd_set, &mut tv)
        < 0 as i32
    {
        return -(1 as i32) as ssize_t;
    }
    if readfds
        .fds_bits[(s / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
        as usize]
        & ((1 as u64)
            << s % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
            as __fd_mask != 0 as i32 as i64
    {
        return read(fd, buf, size);
    }
    *__errno_location() = 110 as i32;
    return -(1 as i32) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn gawk_fwrite(
    mut buf: *const libc::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> size_t {
    return if 0 != 0 && 0 != 0 && size.wrapping_mul(count) <= 8 as i32 as u64
        && size != 0 as i32 as u64
    {
        ({
            let mut __ptr: *const i8 = buf as *const i8;
            let mut __stream: *mut FILE = fp;
            let mut __cnt: size_t = 0;
            __cnt = size.wrapping_mul(count);
            while __cnt > 0 as i32 as u64 {
                if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32
                    as i64 != 0
                {
                    let fresh23 = __ptr;
                    __ptr = __ptr.offset(1);
                    __overflow(__stream, *fresh23 as u8 as i32)
                } else {
                    let fresh24 = __ptr;
                    __ptr = __ptr.offset(1);
                    let fresh25 = (*__stream)._IO_write_ptr;
                    (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
                    *fresh25 = *fresh24;
                    *fresh25 as u8 as i32
                }) == -(1 as i32)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            size.wrapping_mul(count).wrapping_sub(__cnt).wrapping_div(size)
        })
    } else if 0 != 0 && size == 0 as i32 as u64 || 0 != 0 && count == 0 as i32 as u64 {
        0 as i32 as size_t
    } else {
        fwrite_unlocked(buf, size, count, fp)
    };
}
unsafe extern "C" fn gawk_fflush(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> i32 {
    return fflush(fp);
}
unsafe extern "C" fn gawk_ferror(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> i32 {
    return ferror(fp);
}
unsafe extern "C" fn gawk_fclose(
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> i32 {
    let mut result: i32 = 0;
    result = fclose(fp);
    return result;
}
unsafe extern "C" fn init_output_wrapper(mut outbuf: *mut awk_output_buf_t) {
    (*outbuf).name = 0 as *const i8;
    (*outbuf).mode = 0 as *const i8;
    (*outbuf).fp = 0 as *mut FILE;
    (*outbuf).opaque = 0 as *mut libc::c_void;
    (*outbuf).redirected = awk_bool::awk_false;
    (*outbuf).gawk_fwrite = Some(
        gawk_fwrite
            as unsafe extern "C" fn(
                *const libc::c_void,
                size_t,
                size_t,
                *mut FILE,
                *mut libc::c_void,
            ) -> size_t,
    );
    (*outbuf).gawk_fflush = Some(
        gawk_fflush as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32,
    );
    (*outbuf).gawk_ferror = Some(
        gawk_ferror as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32,
    );
    (*outbuf).gawk_fclose = Some(
        gawk_fclose as unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32,
    );
}