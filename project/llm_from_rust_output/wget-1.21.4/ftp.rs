use libc::{c_char, c_int, c_long, c_ulong, c_void, size_t, ssize_t, time_t, FILE};
use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::mem::{size_of, zeroed};
use std::os::raw::c_uint;

// Constants and types from original C code
const FTPOK: uerr_t = 7;
const RETROK: uerr_t = 25;
const NOCONERROR: uerr_t = 0;
const QUOTEXC: uerr_t = 43;
const HOSTERR: uerr_t = 1;
const FWRITEERR: uerr_t = 21;
const WARC_ERR: uerr_t = 52;
const WARC_TMP_FOPENERR: uerr_t = 53;
const WARC_TMP_FWRITEERR: uerr_t = 54;
const FTPNSFOD: uerr_t = 12;
const FTPRESTFAIL: uerr_t = 17;
const FTPRETRINT: uerr_t = 16;
const FTPNOAUTH: uerr_t = 32;
const FTPNOPROT: uerr_t = 31;
const FTPNOPBSZ: uerr_t = 30;
const FTPNOPASV: uerr_t = 29;
const FTPINVPASV: uerr_t = 28;
const FTPSRVERR: uerr_t = 15;
const FTPRERR: uerr_t = 14;
const FTPUNKNOWNTYPE: uerr_t = 13;
const FTPPORTERR: uerr_t = 10;
const FTPLOGREFUSED: uerr_t = 9;
const FTPLOGINC: uerr_t = 8;
const CONERROR: uerr_t = 3;
const CONIMPOSSIBLE: uerr_t = 5;
const CONSSLERR: uerr_t = 4;
const FOPENERR: uerr_t = 19;
const FOPEN_EXCL_ERR: uerr_t = 20;
const UNLINKERR: uerr_t = 47;
const VERIFCERTERR: uerr_t = 46;
const SSLINITFAILED: uerr_t = 45;
const RETRBADPATTERN: uerr_t = 40;
const RECLEVELEXC: uerr_t = 26;
const TRYLIMEXC: uerr_t = 37;
const RETRFINISHED: uerr_t = 35;

const DO_LOGIN: c_int = 1;
const DO_CWD: c_int = 2;
const DO_RETR: c_int = 4;
const DO_LIST: c_int = 8;
const LEAVE_PENDING: c_int = 16;

const ON_YOUR_OWN: c_int = 1;
const DONE_CWD: c_int = 2;
const AVOID_LIST_A: c_int = 4;
const AVOID_LIST: c_int = 8;
const LIST_AFTER_LIST_A_CHECK_DONE: c_int = 16;
const DATA_CHANNEL_SECURITY: c_int = 32;

const ST_UNIX: stype = 0;
const ST_VMS: stype = 1;
const ST_WINNT: stype = 2;
const ST_MACOS: stype = 3;
const ST_OS400: stype = 4;
const ST_OTHER: stype = 5;

const UST_TYPE_L8: ustype = 0;
const UST_MULTINET: ustype = 1;
const UST_OTHER: ustype = 2;

const FT_PLAINFILE: ftype = 0;
const FT_DIRECTORY: ftype = 1;
const FT_SYMLINK: ftype = 2;
const FT_UNKNOWN: ftype = 3;

const GLOB_GLOBALL: c_int = 0;
const GLOB_GETALL: c_int = 1;
const GLOB_GETONE: c_int = 2;

const PROT_CLEAR: prot_level = 67;
const PROT_SAFE: prot_level = 83;
const PROT_CONFIDENTIAL: prot_level = 69;
const PROT_PRIVATE: prot_level = 80;

const LOG_VERBOSE: log_options = 0;
const LOG_NOTQUIET: log_options = 1;
const LOG_NONVERBOSE: log_options = 2;
const LOG_ALWAYS: log_options = 3;
const LOG_PROGRESS: log_options = 4;

const escape_quoting_style: quoting_style = 7;

type uerr_t = c_uint;
type wgint = i64;
type stype = c_uint;
type ustype = c_uint;
type ftype = c_uint;
type prot_level = c_uint;
type log_options = c_uint;
type quoting_style = c_uint;

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: c_int,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: timespec,
    st_mtim: timespec,
    st_ctim: timespec,
    __glibc_reserved: [i64; 3],
}

#[repr(C)]
struct ip_address {
    family: c_int,
    data: C2RustUnnamed_7,
    ipv6_scope: c_int,
}

#[repr(C)]
union C2RustUnnamed_7 {
    d4: in_addr,
    d6: in6_addr,
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct in6_addr {
    __in6_u: C2RustUnnamed_6,
}

#[repr(C)]
union C2RustUnnamed_6 {
    __u6_addr8: [u8; 16],
    __u6_addr16: [u16; 8],
    __u6_addr32: [u32; 4],
}

#[repr(C)]
struct url {
    url: *mut c_char,
    scheme: url_scheme,
    host: *mut c_char,
    port: c_int,
    path: *mut c_char,
    params: *mut c_char,
    query: *mut c_char,
    fragment: *mut c_char,
    dir: *mut c_char,
    file: *mut c_char,
    user: *mut c_char,
    passwd: *mut c_char,
}

type url_scheme = c_uint;

#[repr(C)]
struct fileinfo {
    type_: ftype,
    name: *mut c_char,
    size: wgint,
    tstamp: c_long,
    ptype: parsetype,
    perms: c_int,
    linkto: *mut c_char,
    prev: *mut fileinfo,
    next: *mut fileinfo,
}

type parsetype = c_uint;

#[repr(C)]
struct ccon {
    st: c_int,
    cmd: c_int,
    csock: c_int,
    dltime: f64,
    rs: stype,
    rsu: ustype,
    id: *mut c_char,
    target: *mut c_char,
    proxy: *mut url,
}

// Global variables
static mut opt: options = unsafe { zeroed() };
static mut numurls: c_int = 0;
static mut total_downloaded_bytes: wgint = 0;
static mut total_download_time: f64 = 0.0;
static mut output_stream: *mut FILE = null_mut();

// Function declarations
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(__ver: c_int, __filename: *const c_char, __stat_buf: *mut stat) -> c_int;
    fn __lxstat(__ver: c_int, __filename: *const c_char, __stat_buf: *mut stat) -> c_int;
    fn chmod(__file: *const c_char, __mode: u32) -> c_int;
    fn fnmatch(__pattern: *const c_char, __name: *const c_char, __flags: c_int) -> c_int;
    fn dcgettext(__domainname: *const c_char, __msgid: *const c_char, __category: c_int) -> *mut c_char;
    fn memcpy(_: *mut c_void, _: *const c_void, _: c_ulong) -> *mut c_void;
    fn memset(_: *mut c_void, _: c_int, _: c_ulong) -> *mut c_void;
    fn memcmp(_: *const c_void, _: *const c_void, _: c_ulong) -> c_int;
    fn strcpy(_: *mut c_char, _: *const c_char) -> *mut c_char;
    fn strcmp(_: *const c_char, _: *const c_char) -> c_int;
    fn strchr(_: *const c_char, _: c_int) -> *mut c_char;
    fn strrchr(_: *const c_char, _: c_int) -> *mut c_char;
    fn strstr(_: *const c_char, _: *const c_char) -> *mut c_char;
    fn strlen(_: *const c_char) -> c_ulong;
    fn strerror(_: c_int) -> *mut c_char;
    fn strcasecmp(_: *const c_char, _: *const c_char) -> c_int;
    fn rpl_free(_: *mut c_void);
    fn abort() -> !;
    fn rpl_strtoll(string: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn xmalloc(s: size_t) -> *mut c_void;
    fn xrealloc(p: *mut c_void, s: size_t) -> *mut c_void;
    fn xstrdup(str: *const c_char) -> *mut c_char;
    fn fclose(__stream: *mut FILE) -> c_int;
    fn sprintf(_: *mut c_char, _: *const c_char, _: ...) -> c_int;
    fn snprintf(_: *mut c_char, _: c_ulong, _: *const c_char, _: ...) -> c_int;
    fn __getdelim(__lineptr: *mut *mut c_char, __n: *mut size_t, __delimiter: c_int, __stream: *mut FILE) -> ssize_t;
    fn rpl_fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn logprintf(_: log_options, _: *const c_char, _: ...);
    fn debug_logprintf(_: *const c_char, _: ...);
    fn logputs(_: log_options, _: *const c_char);
    fn quote_n(n: c_int, arg: *const c_char) -> *const c_char;
    fn quote(arg: *const c_char) -> *const c_char;
    fn quotearg_style(s: quoting_style, arg: *const c_char) -> *mut c_char;
    fn symlink(__from: *const c_char, __to: *const c_char) -> c_int;
    fn readlink(__path: *const c_char, __buf: *mut c_char, __len: size_t) -> ssize_t;
    fn unlink(__name: *const c_char) -> c_int;
    fn __errno_location() -> *mut c_int;
    fn datetime_str(_: time_t) -> *mut c_char;
    fn aprintf(_: *const c_char, _: ...) -> *mut c_char;
    fn concat_strings(_: *const c_char, _: ...) -> *mut c_char;
    fn touch(_: *const c_char, _: time_t);
    fn remove_link(_: *const c_char) -> c_int;
    fn file_exists_p(_: *const c_char, _: *mut file_stats_t) -> bool;
    fn fopen_excl(_: *const c_char, _: c_int) -> *mut FILE;
    fn file_merge(_: *const c_char, _: *const c_char) -> *mut c_char;
    fn fnmatch_nocase(_: *const c_char, _: *const c_char, _: c_int) -> c_int;
    fn acceptable(_: *const c_char) -> bool;
    fn accept_url(_: *const c_char) -> bool;
    fn accdir(s: *const c_char) -> bool;
    fn has_wildcards_p(_: *const c_char) -> bool;
    fn human_readable(_: wgint, _: c_int, _: c_int) -> *mut c_char;
    fn number_to_static_string(_: wgint) -> *mut c_char;
    fn url_set_dir(_: *mut url, _: *const c_char);
    fn url_set_file(_: *mut url, _: *const c_char);
    fn scheme_disable(_: url_scheme);
    fn url_string(_: *const url, _: url_auth_mode) -> *mut c_char;
    fn url_file_name(_: *const url, _: *mut c_char) -> *mut c_char;
    fn mkalldirs(_: *const c_char) -> c_int;
    fn fd_read_body(
        _: *const c_char,
        _: c_int,
        _: *mut FILE,
        _: wgint,
        _: wgint,
        _: *mut wgint,
        _: *mut wgint,
        _: *mut f64,
        _: c_int,
        _: *mut FILE,
    ) -> c_int;
    fn retr_rate(_: wgint, _: f64) -> *const c_char;
    fn printwhat(_: c_int, _: c_int);
    fn sleep_between_retrievals(_: c_int);
    fn rotate_backups(_: *const c_char);
    fn set_local_file(_: *mut *const c_char, _: *const c_char);
    fn input_file_url(_: *const c_char) -> bool;
    fn ftp_prot(_: c_int, _: prot_level) -> uerr_t;
    fn print_address(_: *const ip_address) -> *const c_char;
    fn ftp_response(_: c_int, _: *mut *mut c_char) -> uerr_t;
    fn ftp_greeting(_: c_int) -> uerr_t;
    fn ftp_login(_: c_int, _: *const c_char, _: *const c_char) -> uerr_t;
    fn ftp_port(_: c_int, _: *mut c_int) -> uerr_t;
    fn ftp_pasv(_: c_int, _: *mut ip_address, _: *mut c_int) -> uerr_t;
    fn ftp_auth(_: c_int, _: url_scheme) -> uerr_t;
    fn ftp_pbsz(_: c_int, _: c_int) -> uerr_t;
    fn ftp_pwd(_: c_int, _: *mut *mut c_char) -> uerr_t;
    fn ftp_list(
        _: c_int,
        _: *const c_char,
        _: bool,
        _: bool,
        _: *mut bool,
    ) -> uerr_t;
    fn ftp_size(_: c_int, _: *const c_char, _: *mut wgint) -> uerr_t;
    fn ftp_rest(_: c_int, _: wgint) -> uerr_t;
    fn ftp_retr(_: c_int, _: *const c_char) -> uerr_t;
    fn ftp_syst(_: c_int, _: *mut stype, _: *mut ustype) -> uerr_t;
    fn ftp_cwd(_: c_int, _: *const c_char) -> uerr_t;
    fn ftp_type(_: c_int, _: c_int) -> uerr_t;
    fn ftp_epsv(_: c_int, _: *mut ip_address, _: *mut c_int) -> uerr_t;
    fn ftp_eprt(_: c_int, _: *mut c_int) -> uerr_t;
    fn ftp_lprt(_: c_int, _: *mut c_int) -> uerr_t;
    fn ftp_lpsv(_: c_int, _: *mut ip_address, _: *mut c_int) -> uerr_t;
    fn ftp_parse_ls(_: *const c_char, _: stype) -> *mut fileinfo;
    fn ftp_index(_: *const c_char, _: *mut url, _: *mut fileinfo) -> uerr_t;
    fn ftp_process_type(_: *const c_char) -> c_char;
    fn ssl_init() -> bool;
    fn ssl_connect_wget(_: c_int, _: *const c_char, _: *mut c_int) -> bool;
    fn ssl_check_certificate(_: c_int, _: *const c_char) -> bool;
    fn connect_to_host(_: *const c_char, _: c_int) -> c_int;
    fn connect_to_ip(_: *const ip_address, _: c_int, _: *const c_char) -> c_int;
    fn accept_connection(_: c_int) -> c_int;
    fn socket_ip_address(_: c_int, _: *mut ip_address, _: c_int) -> bool;
    fn retryable_socket_connect_error(_: c_int) -> bool;
    fn fd_errstr(_: c_int) -> *const c_char;
    fn fd_close(_: c_int);
    fn search_netrc(
        _: *const c_char,
        _: *mut *const c_char,
        _: *mut *const c_char,
        _: c_int,
        _: *mut FILE,
    );
    fn downloaded_file(_: downloaded_file_t, _: *const c_char) -> downloaded