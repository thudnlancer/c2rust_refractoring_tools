use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type real_pcre;
    pub type hash_table;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn freopen(
        __filename: *const i8,
        __modes: *const i8,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn vasprintf(__ptr: *mut *mut i8, __f: *const i8, __arg: ::core::ffi::VaList) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    static mut exec_name: *const i8;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn mkdir(__path: *const i8, __mode: __mode_t) -> i32;
    fn fnmatch(__pattern: *const i8, __name: *const i8, __flags: i32) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn localeconv() -> *mut lconv;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn random() -> i64;
    fn srandom(__seed: u32);
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn perror(__s: *const i8);
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: i32) -> i32;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn log_set_save_context(_: bool) -> bool;
    fn quote(arg: *const i8) -> *const i8;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn sha256_buffer(
        buffer: *const i8,
        len: size_t,
        resblock: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sleep(__seconds: u32) -> u32;
    fn usleep(__useconds: __useconds_t) -> i32;
    fn pathconf(__path: *const i8, __name: i32) -> i64;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn group_member(__gid: __gid_t) -> i32;
    fn fork() -> __pid_t;
    fn unlink(__name: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn utime(__file: *const i8, __file_times: *const utimbuf) -> i32;
    fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> i32;
    fn rpl_ioctl(fd: i32, request: i32, _: ...) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: i32) -> i32;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn rpl_regcomp(__preg: *mut regex_t, __pattern: *const i8, __cflags: i32) -> i32;
    fn rpl_regexec(
        __preg: *const regex_t,
        __String: *const i8,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: i32,
    ) -> i32;
    fn rpl_regerror(
        __errcode: i32,
        __preg: *const regex_t,
        __errbuf: *mut i8,
        __errbuf_size: size_t,
    ) -> size_t;
    fn pcre_compile(
        _: *const i8,
        _: i32,
        _: *mut *const i8,
        _: *mut i32,
        _: *const u8,
    ) -> *mut pcre;
    fn pcre_exec(
        _: *const pcre,
        _: *const pcre_extra,
        _: *const i8,
        _: i32,
        _: i32,
        _: i32,
        _: *mut i32,
        _: i32,
    ) -> i32;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> i32;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> i32;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
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
pub type __int64_t = i64;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut i8,
    pub thousands_sep: *mut i8,
    pub grouping: *mut i8,
    pub int_curr_symbol: *mut i8,
    pub currency_symbol: *mut i8,
    pub mon_decimal_point: *mut i8,
    pub mon_thousands_sep: *mut i8,
    pub mon_grouping: *mut i8,
    pub positive_sign: *mut i8,
    pub negative_sign: *mut i8,
    pub int_frac_digits: i8,
    pub frac_digits: i8,
    pub p_cs_precedes: i8,
    pub p_sep_by_space: i8,
    pub n_cs_precedes: i8,
    pub n_sep_by_space: i8,
    pub p_sign_posn: i8,
    pub n_sign_posn: i8,
    pub int_p_cs_precedes: i8,
    pub int_p_sep_by_space: i8,
    pub int_n_cs_precedes: i8,
    pub int_n_sep_by_space: i8,
    pub int_p_sign_posn: i8,
    pub int_n_sign_posn: i8,
}
pub type wgint = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub verbose: i32,
    pub quiet: bool,
    pub ntry: i32,
    pub retry_connrefused: bool,
    pub retry_on_host_error: bool,
    pub retry_on_http_error: *mut i8,
    pub background: bool,
    pub ignore_length: bool,
    pub recursive: bool,
    pub spanhost: bool,
    pub max_redirect: i32,
    pub relative_only: bool,
    pub no_parent: bool,
    pub reclevel: i32,
    pub dirstruct: bool,
    pub no_dirstruct: bool,
    pub cut_dirs: i32,
    pub add_hostdir: bool,
    pub protocol_directories: bool,
    pub noclobber: bool,
    pub unlink_requested: bool,
    pub dir_prefix: *mut i8,
    pub lfilename: *mut i8,
    pub input_filename: *mut i8,
    pub choose_config: *mut i8,
    pub noconfig: bool,
    pub force_html: bool,
    pub default_page: *mut i8,
    pub spider: bool,
    pub accepts: *mut *mut i8,
    pub rejects: *mut *mut i8,
    pub excludes: *mut *const i8,
    pub includes: *mut *const i8,
    pub ignore_case: bool,
    pub acceptregex_s: *mut i8,
    pub rejectregex_s: *mut i8,
    pub acceptregex: *mut libc::c_void,
    pub rejectregex: *mut libc::c_void,
    pub regex_type: C2RustUnnamed_3,
    pub regex_compile_fun: Option<unsafe extern "C" fn(*const i8) -> *mut libc::c_void>,
    pub regex_match_fun: Option<
        unsafe extern "C" fn(*const libc::c_void, *const i8) -> bool,
    >,
    pub domains: *mut *mut i8,
    pub exclude_domains: *mut *mut i8,
    pub dns_cache: bool,
    pub follow_tags: *mut *mut i8,
    pub ignore_tags: *mut *mut i8,
    pub follow_ftp: bool,
    pub retr_symlinks: bool,
    pub output_document: *mut i8,
    pub warc_filename: *mut i8,
    pub warc_tempdir: *mut i8,
    pub warc_cdx_dedup_filename: *mut i8,
    pub warc_maxsize: wgint,
    pub warc_compression_enabled: bool,
    pub warc_digests_enabled: bool,
    pub warc_cdx_enabled: bool,
    pub warc_keep_log: bool,
    pub warc_user_headers: *mut *mut i8,
    pub enable_xattr: bool,
    pub user: *mut i8,
    pub passwd: *mut i8,
    pub ask_passwd: bool,
    pub use_askpass: *mut i8,
    pub always_rest: bool,
    pub start_pos: wgint,
    pub ftp_user: *mut i8,
    pub ftp_passwd: *mut i8,
    pub netrc: bool,
    pub ftp_glob: bool,
    pub ftp_pasv: bool,
    pub http_user: *mut i8,
    pub http_passwd: *mut i8,
    pub user_headers: *mut *mut i8,
    pub http_keep_alive: bool,
    pub use_proxy: bool,
    pub allow_cache: bool,
    pub http_proxy: *mut i8,
    pub ftp_proxy: *mut i8,
    pub https_proxy: *mut i8,
    pub no_proxy: *mut *mut i8,
    pub base_href: *mut i8,
    pub progress_type: *mut i8,
    pub show_progress: i32,
    pub noscroll: bool,
    pub proxy_user: *mut i8,
    pub proxy_passwd: *mut i8,
    pub read_timeout: libc::c_double,
    pub dns_timeout: libc::c_double,
    pub connect_timeout: libc::c_double,
    pub random_wait: bool,
    pub wait: libc::c_double,
    pub waitretry: libc::c_double,
    pub use_robots: bool,
    pub limit_rate: wgint,
    pub quota: wgint,
    pub server_response: bool,
    pub save_headers: bool,
    pub content_on_error: bool,
    pub debug: bool,
    pub timestamping: bool,
    pub if_modified_since: bool,
    pub backup_converted: bool,
    pub backups: i32,
    pub useragent: *mut i8,
    pub referer: *mut i8,
    pub convert_links: bool,
    pub convert_file_only: bool,
    pub remove_listing: bool,
    pub htmlify: bool,
    pub dot_style: *mut i8,
    pub dot_bytes: wgint,
    pub dots_in_line: i32,
    pub dot_spacing: i32,
    pub delete_after: bool,
    pub adjust_extension: bool,
    pub page_requisites: bool,
    pub bind_address: *mut i8,
    pub secure_protocol: C2RustUnnamed_2,
    pub secure_protocol_name: [i8; 8],
    pub check_cert: i32,
    pub cert_file: *mut i8,
    pub private_key: *mut i8,
    pub cert_type: keyfile_type,
    pub private_key_type: keyfile_type,
    pub ca_directory: *mut i8,
    pub ca_cert: *mut i8,
    pub crl_file: *mut i8,
    pub pinnedpubkey: *mut i8,
    pub random_file: *mut i8,
    pub egd_file: *mut i8,
    pub https_only: bool,
    pub ftps_resume_ssl: bool,
    pub ftps_fallback_to_ftp: bool,
    pub ftps_implicit: bool,
    pub ftps_clear_data_connection: bool,
    pub tls_ciphers_string: *mut i8,
    pub cookies: bool,
    pub cookies_input: *mut i8,
    pub cookies_output: *mut i8,
    pub keep_badhash: bool,
    pub keep_session_cookies: bool,
    pub post_data: *mut i8,
    pub post_file_name: *mut i8,
    pub method: *mut i8,
    pub body_data: *mut i8,
    pub body_file: *mut i8,
    pub restrict_files_os: C2RustUnnamed_1,
    pub restrict_files_ctrl: bool,
    pub restrict_files_nonascii: bool,
    pub restrict_files_case: C2RustUnnamed_0,
    pub strict_comments: bool,
    pub preserve_perm: bool,
    pub ipv4_only: bool,
    pub ipv6_only: bool,
    pub prefer_family: C2RustUnnamed,
    pub content_disposition: bool,
    pub auth_without_challenge: bool,
    pub enable_iri: bool,
    pub encoding_remote: *mut i8,
    pub locale: *const i8,
    pub trustservernames: bool,
    pub useservertimestamps: bool,
    pub show_all_dns_entries: bool,
    pub report_bps: bool,
    pub compression: compression_options,
    pub rejected_log: *mut i8,
    pub hsts: bool,
    pub hsts_file: *mut i8,
    pub homedir: *const i8,
    pub wgetrcfile: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum compression_options {
    compression_none = 2,
    compression_gzip = 1,
    compression_auto = 0,
}
impl compression_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            compression_options::compression_none => 2,
            compression_options::compression_gzip => 1,
            compression_options::compression_auto => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> compression_options {
        match value {
            2 => compression_options::compression_none,
            1 => compression_options::compression_gzip,
            0 => compression_options::compression_auto,
            _ => panic!("Invalid value for compression_options: {}", value),
        }
    }
}
impl AddAssign<u32> for compression_options {
    fn add_assign(&mut self, rhs: u32) {
        *self = compression_options::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for compression_options {
    fn sub_assign(&mut self, rhs: u32) {
        *self = compression_options::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for compression_options {
    fn mul_assign(&mut self, rhs: u32) {
        *self = compression_options::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for compression_options {
    fn div_assign(&mut self, rhs: u32) {
        *self = compression_options::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for compression_options {
    fn rem_assign(&mut self, rhs: u32) {
        *self = compression_options::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for compression_options {
    type Output = compression_options;
    fn add(self, rhs: u32) -> compression_options {
        compression_options::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for compression_options {
    type Output = compression_options;
    fn sub(self, rhs: u32) -> compression_options {
        compression_options::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for compression_options {
    type Output = compression_options;
    fn mul(self, rhs: u32) -> compression_options {
        compression_options::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for compression_options {
    type Output = compression_options;
    fn div(self, rhs: u32) -> compression_options {
        compression_options::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for compression_options {
    type Output = compression_options;
    fn rem(self, rhs: u32) -> compression_options {
        compression_options::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    prefer_none = 2,
    prefer_ipv6 = 1,
    prefer_ipv4 = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::prefer_none => 2,
            C2RustUnnamed::prefer_ipv6 => 1,
            C2RustUnnamed::prefer_ipv4 => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            2 => C2RustUnnamed::prefer_none,
            1 => C2RustUnnamed::prefer_ipv6,
            0 => C2RustUnnamed::prefer_ipv4,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    restrict_uppercase = 2,
    restrict_lowercase = 1,
    restrict_no_case_restriction = 0,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::restrict_uppercase => 2,
            C2RustUnnamed_0::restrict_lowercase => 1,
            C2RustUnnamed_0::restrict_no_case_restriction => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            2 => C2RustUnnamed_0::restrict_uppercase,
            1 => C2RustUnnamed_0::restrict_lowercase,
            0 => C2RustUnnamed_0::restrict_no_case_restriction,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    restrict_windows = 2,
    restrict_vms = 1,
    restrict_unix = 0,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::restrict_windows => 2,
            C2RustUnnamed_1::restrict_vms => 1,
            C2RustUnnamed_1::restrict_unix => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            2 => C2RustUnnamed_1::restrict_windows,
            1 => C2RustUnnamed_1::restrict_vms,
            0 => C2RustUnnamed_1::restrict_unix,
            _ => panic!("Invalid value for C2RustUnnamed_1: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_1 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_1 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_1 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_1 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_1 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn add(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn sub(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn mul(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn div(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn rem(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum keyfile_type {
    keyfile_asn1 = 1,
    keyfile_pem = 0,
}
impl keyfile_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            keyfile_type::keyfile_asn1 => 1,
            keyfile_type::keyfile_pem => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> keyfile_type {
        match value {
            1 => keyfile_type::keyfile_asn1,
            0 => keyfile_type::keyfile_pem,
            _ => panic!("Invalid value for keyfile_type: {}", value),
        }
    }
}
impl AddAssign<u32> for keyfile_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = keyfile_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for keyfile_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = keyfile_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for keyfile_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = keyfile_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for keyfile_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = keyfile_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for keyfile_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = keyfile_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for keyfile_type {
    type Output = keyfile_type;
    fn add(self, rhs: u32) -> keyfile_type {
        keyfile_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for keyfile_type {
    type Output = keyfile_type;
    fn sub(self, rhs: u32) -> keyfile_type {
        keyfile_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for keyfile_type {
    type Output = keyfile_type;
    fn mul(self, rhs: u32) -> keyfile_type {
        keyfile_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for keyfile_type {
    type Output = keyfile_type;
    fn div(self, rhs: u32) -> keyfile_type {
        keyfile_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for keyfile_type {
    type Output = keyfile_type;
    fn rem(self, rhs: u32) -> keyfile_type {
        keyfile_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    secure_protocol_pfs = 7,
    secure_protocol_tlsv1_3 = 6,
    secure_protocol_tlsv1_2 = 5,
    secure_protocol_tlsv1_1 = 4,
    secure_protocol_tlsv1 = 3,
    secure_protocol_sslv3 = 2,
    secure_protocol_sslv2 = 1,
    secure_protocol_auto = 0,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::secure_protocol_pfs => 7,
            C2RustUnnamed_2::secure_protocol_tlsv1_3 => 6,
            C2RustUnnamed_2::secure_protocol_tlsv1_2 => 5,
            C2RustUnnamed_2::secure_protocol_tlsv1_1 => 4,
            C2RustUnnamed_2::secure_protocol_tlsv1 => 3,
            C2RustUnnamed_2::secure_protocol_sslv3 => 2,
            C2RustUnnamed_2::secure_protocol_sslv2 => 1,
            C2RustUnnamed_2::secure_protocol_auto => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            7 => C2RustUnnamed_2::secure_protocol_pfs,
            6 => C2RustUnnamed_2::secure_protocol_tlsv1_3,
            5 => C2RustUnnamed_2::secure_protocol_tlsv1_2,
            4 => C2RustUnnamed_2::secure_protocol_tlsv1_1,
            3 => C2RustUnnamed_2::secure_protocol_tlsv1,
            2 => C2RustUnnamed_2::secure_protocol_sslv3,
            1 => C2RustUnnamed_2::secure_protocol_sslv2,
            0 => C2RustUnnamed_2::secure_protocol_auto,
            _ => panic!("Invalid value for C2RustUnnamed_2: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_2 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_2 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_2 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_2 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_2 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn add(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn sub(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn mul(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn div(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn rem(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    regex_type_posix = 1,
    regex_type_pcre = 0,
}
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_3::regex_type_posix => 1,
            C2RustUnnamed_3::regex_type_pcre => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_3 {
        match value {
            1 => C2RustUnnamed_3::regex_type_posix,
            0 => C2RustUnnamed_3::regex_type_pcre,
            _ => panic!("Invalid value for C2RustUnnamed_3: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_3 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_3 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_3 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_3 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_3 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn add(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn sub(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn mul(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn div(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn rem(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
pub type va_list = __builtin_va_list;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_options {
    LOG_VERBOSE,
    LOG_NOTQUIET,
    LOG_NONVERBOSE,
    LOG_ALWAYS,
    LOG_PROGRESS,
}
impl log_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_options::LOG_VERBOSE => 0,
            log_options::LOG_NOTQUIET => 1,
            log_options::LOG_NONVERBOSE => 2,
            log_options::LOG_ALWAYS => 3,
            log_options::LOG_PROGRESS => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_options {
        match value {
            0 => log_options::LOG_VERBOSE,
            1 => log_options::LOG_NOTQUIET,
            2 => log_options::LOG_NONVERBOSE,
            3 => log_options::LOG_ALWAYS,
            4 => log_options::LOG_PROGRESS,
            _ => panic!("Invalid value for log_options: {}", value),
        }
    }
}
impl AddAssign<u32> for log_options {
    fn add_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for log_options {
    fn sub_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for log_options {
    fn mul_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for log_options {
    fn div_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for log_options {
    fn rem_assign(&mut self, rhs: u32) {
        *self = log_options::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for log_options {
    type Output = log_options;
    fn add(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for log_options {
    type Output = log_options;
    fn sub(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for log_options {
    type Output = log_options;
    fn mul(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for log_options {
    type Output = log_options;
    fn div(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for log_options {
    type Output = log_options;
    fn rem(self, rhs: u32) -> log_options {
        log_options::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    SHA256_DIGEST_SIZE = 32,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::SHA256_DIGEST_SIZE => 32,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            32 => C2RustUnnamed_4::SHA256_DIGEST_SIZE,
            _ => panic!("Invalid value for C2RustUnnamed_4: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_4 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_4 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_4 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_4 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_4 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn add(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn sub(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn mul(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn div(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn rem(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __itimer_which {
    ITIMER_PROF = 2,
    ITIMER_VIRTUAL = 1,
    ITIMER_REAL = 0,
}
impl __itimer_which {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __itimer_which::ITIMER_PROF => 2,
            __itimer_which::ITIMER_VIRTUAL => 1,
            __itimer_which::ITIMER_REAL => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> __itimer_which {
        match value {
            2 => __itimer_which::ITIMER_PROF,
            1 => __itimer_which::ITIMER_VIRTUAL,
            0 => __itimer_which::ITIMER_REAL,
            _ => panic!("Invalid value for __itimer_which: {}", value),
        }
    }
}
impl AddAssign<u32> for __itimer_which {
    fn add_assign(&mut self, rhs: u32) {
        *self = __itimer_which::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for __itimer_which {
    fn sub_assign(&mut self, rhs: u32) {
        *self = __itimer_which::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for __itimer_which {
    fn mul_assign(&mut self, rhs: u32) {
        *self = __itimer_which::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for __itimer_which {
    fn div_assign(&mut self, rhs: u32) {
        *self = __itimer_which::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for __itimer_which {
    fn rem_assign(&mut self, rhs: u32) {
        *self = __itimer_which::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for __itimer_which {
    type Output = __itimer_which;
    fn add(self, rhs: u32) -> __itimer_which {
        __itimer_which::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for __itimer_which {
    type Output = __itimer_which;
    fn sub(self, rhs: u32) -> __itimer_which {
        __itimer_which::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for __itimer_which {
    type Output = __itimer_which;
    fn mul(self, rhs: u32) -> __itimer_which {
        __itimer_which::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for __itimer_which {
    type Output = __itimer_which;
    fn div(self, rhs: u32) -> __itimer_which {
        __itimer_which::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for __itimer_which {
    type Output = __itimer_which;
    fn rem(self, rhs: u32) -> __itimer_which {
        __itimer_which::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
pub type __itimer_which_t = __itimer_which;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type sigjmp_buf = [__jmp_buf_tag; 1];
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = u64;
pub type C2RustUnnamed_5 = i32;
pub const _REG_ERPAREN: C2RustUnnamed_5 = 16;
pub const _REG_ESIZE: C2RustUnnamed_5 = 15;
pub const _REG_EEND: C2RustUnnamed_5 = 14;
pub const _REG_BADRPT: C2RustUnnamed_5 = 13;
pub const _REG_ESPACE: C2RustUnnamed_5 = 12;
pub const _REG_ERANGE: C2RustUnnamed_5 = 11;
pub const _REG_BADBR: C2RustUnnamed_5 = 10;
pub const _REG_EBRACE: C2RustUnnamed_5 = 9;
pub const _REG_EPAREN: C2RustUnnamed_5 = 8;
pub const _REG_EBRACK: C2RustUnnamed_5 = 7;
pub const _REG_ESUBREG: C2RustUnnamed_5 = 6;
pub const _REG_EESCAPE: C2RustUnnamed_5 = 5;
pub const _REG_ECTYPE: C2RustUnnamed_5 = 4;
pub const _REG_ECOLLATE: C2RustUnnamed_5 = 3;
pub const _REG_BADPAT: C2RustUnnamed_5 = 2;
pub const _REG_NOMATCH: C2RustUnnamed_5 = 1;
pub const _REG_NOERROR: C2RustUnnamed_5 = 0;
pub const _REG_ENOSYS: C2RustUnnamed_5 = -1;
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
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type pcre = real_pcre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: u64,
    pub study_data: *mut libc::c_void,
    pub match_limit: u64,
    pub callout_data: *mut libc::c_void,
    pub tables: *const u8,
    pub match_limit_recursion: u64,
    pub mark: *mut *mut u8,
    pub executable_jit: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_memory {
    pub content: *mut i8,
    pub length: i64,
    pub mmap_p: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_6 {
    WGET_EXIT_SUCCESS = 0,
    WGET_EXIT_GENERIC_ERROR = 1,
    WGET_EXIT_PARSE_ERROR = 2,
    WGET_EXIT_IO_FAIL = 3,
    WGET_EXIT_NETWORK_FAIL = 4,
    WGET_EXIT_SSL_AUTH_FAIL = 5,
    WGET_EXIT_SERVER_AUTH_FAIL = 6,
    WGET_EXIT_PROTOCOL_ERROR = 7,
    WGET_EXIT_SERVER_ERROR = 8,
    WGET_EXIT_UNKNOWN,
}
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_6::WGET_EXIT_SUCCESS => 0,
            C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR => 1,
            C2RustUnnamed_6::WGET_EXIT_PARSE_ERROR => 2,
            C2RustUnnamed_6::WGET_EXIT_IO_FAIL => 3,
            C2RustUnnamed_6::WGET_EXIT_NETWORK_FAIL => 4,
            C2RustUnnamed_6::WGET_EXIT_SSL_AUTH_FAIL => 5,
            C2RustUnnamed_6::WGET_EXIT_SERVER_AUTH_FAIL => 6,
            C2RustUnnamed_6::WGET_EXIT_PROTOCOL_ERROR => 7,
            C2RustUnnamed_6::WGET_EXIT_SERVER_ERROR => 8,
            C2RustUnnamed_6::WGET_EXIT_UNKNOWN => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_6 {
        match value {
            0 => C2RustUnnamed_6::WGET_EXIT_SUCCESS,
            1 => C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR,
            2 => C2RustUnnamed_6::WGET_EXIT_PARSE_ERROR,
            3 => C2RustUnnamed_6::WGET_EXIT_IO_FAIL,
            4 => C2RustUnnamed_6::WGET_EXIT_NETWORK_FAIL,
            5 => C2RustUnnamed_6::WGET_EXIT_SSL_AUTH_FAIL,
            6 => C2RustUnnamed_6::WGET_EXIT_SERVER_AUTH_FAIL,
            7 => C2RustUnnamed_6::WGET_EXIT_PROTOCOL_ERROR,
            8 => C2RustUnnamed_6::WGET_EXIT_SERVER_ERROR,
            9 => C2RustUnnamed_6::WGET_EXIT_UNKNOWN,
            _ => panic!("Invalid value for C2RustUnnamed_6: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_6 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_6 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_6 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_6 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_6 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn add(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn sub(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn mul(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn div(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn rem(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type file_stats_t = file_stat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: i32,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
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
unsafe extern "C" fn c_isspace(mut c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: i32) -> i32 {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn memfatal(mut context: *const i8, mut attempted_size: i64) {
    log_set_save_context(0 as i32 != 0);
    if attempted_size == -(3 as i32) as i64 {
        logprintf(
            log_options::LOG_ALWAYS,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Failed to allocate enough memory; memory exhausted.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            context,
        );
    } else {
        logprintf(
            log_options::LOG_ALWAYS,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Failed to allocate %ld bytes; memory exhausted.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            context,
            attempted_size,
        );
    }
    exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
}
#[no_mangle]
pub static mut char_prop: [u8; 256] = [
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    2 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    16 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    0 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    16 as i32 as u8,
    4 as i32 as u8,
    0 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    80 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    16 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    16 as i32 as u8,
    1 as i32 as u8,
    96 as i32 as u8,
    96 as i32 as u8,
    96 as i32 as u8,
    96 as i32 as u8,
    96 as i32 as u8,
    96 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    32 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    1 as i32 as u8,
    17 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    8 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    8 as i32 as u8,
];
#[no_mangle]
pub unsafe extern "C" fn xstrdup_lower(mut s: *const i8) -> *mut i8 {
    let mut copy: *mut i8 = xstrdup(s);
    let mut p: *mut i8 = copy;
    while *p != 0 {
        *p = c_tolower(*p as i32) as i8;
        p = p.offset(1);
        p;
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn strdupdelim(mut beg: *const i8, mut end: *const i8) -> *mut i8 {
    if !beg.is_null() && beg <= end {
        let mut res: *mut i8 = xmalloc(
            (end.offset_from(beg) as i64 + 1 as i32 as i64) as size_t,
        ) as *mut i8;
        memcpy(
            res as *mut libc::c_void,
            beg as *const libc::c_void,
            end.offset_from(beg) as i64 as u64,
        );
        *res.offset(end.offset_from(beg) as i64 as isize) = '\0' as i32 as i8;
        return res;
    }
    return xstrdup(b"\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn sepstring(mut s: *const i8) -> *mut *mut i8 {
    let mut res: *mut *mut i8 = 0 as *mut *mut i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut i: i32 = 0 as i32;
    if s.is_null() || *s == 0 {
        return 0 as *mut *mut i8;
    }
    res = 0 as *mut *mut i8;
    p = s;
    while *s != 0 {
        if *s as i32 == ',' as i32 {
            res = xrealloc(
                res as *mut libc::c_void,
                ((i + 2 as i32) as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
            ) as *mut *mut i8;
            let ref mut fresh0 = *res.offset(i as isize);
            *fresh0 = strdupdelim(p, s);
            i += 1;
            let ref mut fresh1 = *res.offset(i as isize);
            *fresh1 = 0 as *mut i8;
            s = s.offset(1);
            s;
            while c_isspace(*s as i32) {
                s = s.offset(1);
                s;
            }
            p = s;
        } else {
            s = s.offset(1);
            s;
        }
    }
    res = xrealloc(
        res as *mut libc::c_void,
        ((i + 2 as i32) as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    let ref mut fresh2 = *res.offset(i as isize);
    *fresh2 = strdupdelim(p, s);
    let ref mut fresh3 = *res.offset((i + 1 as i32) as isize);
    *fresh3 = 0 as *mut i8;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn aprintf(mut fmt: *const i8, mut args: ...) -> *mut i8 {
    let mut ret: i32 = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut str: *mut i8 = 0 as *mut i8;
    args_0 = args.clone();
    ret = vasprintf(&mut str, fmt, args_0.as_va_list());
    if ret < 0 as i32 && *__errno_location() == 12 as i32 {
        memfatal(b"aprintf\0" as *const u8 as *const i8, -(3 as i32) as i64);
    } else if ret < 0 as i32 {
        return 0 as *mut i8
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut i8,
    mut src: *const i8,
    mut size: size_t,
) -> size_t {
    let mut old: *const i8 = src;
    if size != 0 {
        loop {
            size = size.wrapping_sub(1);
            if !(size != 0) {
                break;
            }
            let fresh4 = src;
            src = src.offset(1);
            let fresh5 = dst;
            dst = dst.offset(1);
            *fresh5 = *fresh4;
            if *fresh5 == 0 {
                return (src.offset_from(old) as i64 - 1 as i32 as i64) as size_t;
            }
        }
        *dst = 0 as i32 as i8;
    }
    loop {
        let fresh6 = src;
        src = src.offset(1);
        if !(*fresh6 != 0) {
            break;
        }
    }
    return (src.offset_from(old) as i64 - 1 as i32 as i64) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn concat_strings(mut str0: *const i8, mut args: ...) -> *mut i8 {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut arg: *const i8 = 0 as *const i8;
    let mut length: size_t = 0 as i32 as size_t;
    let mut pos: size_t = 0 as i32 as size_t;
    let mut s: *mut i8 = 0 as *mut i8;
    if str0.is_null() {
        return 0 as *mut i8;
    }
    args_0 = args.clone();
    arg = str0;
    while !arg.is_null() {
        length = (length as u64).wrapping_add(strlen(arg)) as size_t as size_t;
        arg = args_0.arg::<*const i8>();
    }
    s = xmalloc(length.wrapping_add(1 as i32 as u64)) as *mut i8;
    args_0 = args.clone();
    arg = str0;
    while !arg.is_null() {
        pos = (pos as u64)
            .wrapping_add(
                strlcpy(
                    s.offset(pos as isize),
                    arg,
                    length.wrapping_sub(pos).wrapping_add(1 as i32 as u64),
                ),
            ) as size_t as size_t;
        arg = args_0.arg::<*const i8>();
    }
    return s;
}
unsafe extern "C" fn fmttime(mut t: time_t, mut fmt: *const i8) -> *mut i8 {
    static mut output: [i8; 32] = [0; 32];
    let mut tm: *mut tm = localtime(&mut t);
    if tm.is_null() {
        abort();
    }
    if strftime(output.as_mut_ptr(), ::core::mem::size_of::<[i8; 32]>() as u64, fmt, tm)
        == 0
    {
        abort();
    }
    return output.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn time_str(mut t: time_t) -> *mut i8 {
    return fmttime(t, b"%H:%M:%S\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn datetime_str(mut t: time_t) -> *mut i8 {
    return fmttime(t, b"%Y-%m-%d %H:%M:%S\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn fork_to_background() -> bool {
    let mut pid: pid_t = 0;
    let mut logfile_changed: bool = 0 as i32 != 0;
    if (opt.lfilename).is_null() && (!opt.quiet || opt.server_response as i32 != 0) {
        let mut new_log_fp: *mut FILE = unique_create(
            b"wget-log\0" as *const u8 as *const i8,
            0 as i32 != 0,
            &mut opt.lfilename,
        );
        if !new_log_fp.is_null() {
            logfile_changed = 1 as i32 != 0;
            fclose(new_log_fp);
        }
    }
    pid = fork();
    if pid < 0 as i32 {
        perror(b"fork\0" as *const u8 as *const i8);
        exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
    } else if pid != 0 as i32 {
        printf(
            dcgettext(
                0 as *const i8,
                b"Continuing in background, pid %d.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            pid,
        );
        if logfile_changed {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Output will be written to %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote(opt.lfilename),
            );
        }
        exit(C2RustUnnamed_6::WGET_EXIT_SUCCESS as i32);
    }
    setsid();
    if (freopen(
        b"/dev/null\0" as *const u8 as *const i8,
        b"r\0" as *const u8 as *const i8,
        stdin,
    ))
        .is_null()
    {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Failed to redirect stdin to /dev/null.\n\0" as *const u8 as *const i8,
            );
        }
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const i8,
        b"w\0" as *const u8 as *const i8,
        stdout,
    ))
        .is_null()
    {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Failed to redirect stdout to /dev/null.\n\0" as *const u8 as *const i8,
            );
        }
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const i8,
        b"w\0" as *const u8 as *const i8,
        stderr,
    ))
        .is_null()
    {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Failed to redirect stderr to /dev/null.\n\0" as *const u8 as *const i8,
            );
        }
    }
    return logfile_changed;
}
#[no_mangle]
pub unsafe extern "C" fn touch(mut file: *const i8, mut tm: time_t) {
    let mut times: utimbuf = utimbuf { actime: 0, modtime: 0 };
    times.modtime = tm;
    times.actime = time(0 as *mut time_t);
    if utime(file, &mut times) == -(1 as i32) {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"utime(%s): %s\n\0" as *const u8 as *const i8,
            file,
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn remove_link(mut file: *const i8) -> i32 {
    let mut err: i32 = 0 as i32;
    let mut st: stat = stat {
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
    if lstat(file, &mut st) == 0 as i32
        && st.st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32
    {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Unlinking %s (symlink).\n\0" as *const u8 as *const i8,
                file,
            );
        }
        err = unlink(file);
        if err != 0 as i32 {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Failed to unlink symlink %s: %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote(file),
                strerror(*__errno_location()),
            );
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn file_exists_p(
    mut filename: *const i8,
    mut fstats: *mut file_stats_t,
) -> bool {
    let mut buf: stat = stat {
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
    if filename.is_null() {
        return 0 as i32 != 0;
    }
    *__errno_location() = 0 as i32;
    if stat(filename, &mut buf) == 0 as i32
        && buf.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
        && (0o400 as i32 as u32 & buf.st_mode != 0 && getuid() == buf.st_uid
            || (0o400 as i32 >> 3 as i32) as u32 & buf.st_mode != 0
                && group_member(buf.st_gid) != 0
            || (0o400 as i32 >> 3 as i32 >> 3 as i32) as u32 & buf.st_mode != 0)
    {
        if !fstats.is_null() {
            (*fstats).access_err = 0 as i32;
            (*fstats).st_ino = buf.st_ino;
            (*fstats).st_dev = buf.st_dev;
        }
        return 1 as i32 != 0;
    } else {
        if !fstats.is_null() {
            (*fstats).access_err = if *__errno_location() == 0 as i32 {
                13 as i32
            } else {
                *__errno_location()
            };
        }
        *__errno_location() = 0 as i32;
        return 0 as i32 != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn file_non_directory_p(mut path: *const i8) -> bool {
    let mut buf: stat = stat {
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
    if lstat(path, &mut buf) != 0 as i32 {
        return 0 as i32 != 0;
    }
    return if buf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
        0 as i32
    } else {
        1 as i32
    } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn file_size(mut filename: *const i8) -> wgint {
    let mut size: wgint = 0;
    let mut fp: *mut FILE = rpl_fopen(filename, b"rb\0" as *const u8 as *const i8);
    if fp.is_null() {
        return -(1 as i32) as wgint;
    }
    rpl_fseeko(fp, 0 as i32 as off_t, 2 as i32);
    size = ftello(fp);
    fclose(fp);
    return size;
}
unsafe extern "C" fn unique_name_1(mut prefix: *const i8) -> *mut i8 {
    let mut count: i32 = 1 as i32;
    let mut plen: i32 = strlen(prefix) as i32;
    let mut template: *mut i8 = xmalloc((plen + 1 as i32 + 24 as i32) as size_t)
        as *mut i8;
    let mut template_tail: *mut i8 = template.offset(plen as isize);
    memcpy(template as *mut libc::c_void, prefix as *const libc::c_void, plen as u64);
    let fresh7 = template_tail;
    template_tail = template_tail.offset(1);
    *fresh7 = '.' as i32 as i8;
    loop {
        let fresh8 = count;
        count = count + 1;
        number_to_string(template_tail, fresh8 as wgint);
        if !(file_exists_p(template, 0 as *mut file_stats_t) as i32 != 0
            && count < 999999 as i32)
        {
            break;
        }
    }
    return template;
}
#[no_mangle]
pub unsafe extern "C" fn unique_name_passthrough(mut file: *const i8) -> *mut i8 {
    return if file_exists_p(file, 0 as *mut file_stats_t) as i32 != 0 {
        unique_name_1(file)
    } else {
        file as *mut i8
    };
}
#[no_mangle]
pub unsafe extern "C" fn unique_name(mut file: *const i8) -> *mut i8 {
    return if file_exists_p(file, 0 as *mut file_stats_t) as i32 != 0 {
        unique_name_1(file)
    } else {
        xstrdup(file)
    };
}
#[no_mangle]
pub unsafe extern "C" fn unique_create(
    mut name: *const i8,
    mut binary: bool,
    mut opened_name: *mut *mut i8,
) -> *mut FILE {
    let mut uname: *mut i8 = unique_name(name);
    let mut fp: *mut FILE = 0 as *mut FILE;
    loop {
        fp = fopen_excl(uname, binary as i32);
        if !(fp.is_null() && *__errno_location() == 17 as i32) {
            break;
        }
        rpl_free(uname as *mut libc::c_void);
        uname = 0 as *mut i8;
        uname = unique_name(name);
    }
    if !opened_name.is_null() {
        if !fp.is_null() {
            *opened_name = uname;
        } else {
            *opened_name = 0 as *mut i8;
            rpl_free(uname as *mut libc::c_void);
            uname = 0 as *mut i8;
        }
    } else {
        rpl_free(uname as *mut libc::c_void);
        uname = 0 as *mut i8;
    }
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn fopen_excl(mut fname: *const i8, mut binary: i32) -> *mut FILE {
    let mut fd: i32 = 0;
    let mut flags: i32 = 0o1 as i32 | 0o100 as i32 | 0o200 as i32;
    if binary != 0 {
        flags |= 0 as i32;
    }
    fd = open(fname, flags, 0o666 as i32);
    if fd < 0 as i32 {
        return 0 as *mut FILE;
    }
    return fdopen(
        fd,
        if binary != 0 {
            b"wb\0" as *const u8 as *const i8
        } else {
            b"w\0" as *const u8 as *const i8
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn fopen_stat(
    mut fname: *const i8,
    mut mode: *const i8,
    mut fstats: *mut file_stats_t,
) -> *mut FILE {
    let mut fd: i32 = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fdstats: stat = stat {
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
    fp = rpl_fopen(fname, mode);
    if fp.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Failed to Fopen file %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
        );
        return 0 as *mut FILE;
    }
    fd = fileno(fp);
    if fd < 0 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Failed to get FD for file %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
        );
        fclose(fp);
        return 0 as *mut FILE;
    }
    memset(
        &mut fdstats as *mut stat as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<stat>() as u64,
    );
    if fstat(fd, &mut fdstats) == -(1 as i32) {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Failed to stat file %s, (check permissions)\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            fname,
        );
        fclose(fp);
        return 0 as *mut FILE;
    }
    if !fstats.is_null()
        && (fdstats.st_dev != (*fstats).st_dev || fdstats.st_ino != (*fstats).st_ino)
    {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"File %s changed since the last check. Security check failed.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
        );
        fclose(fp);
        return 0 as *mut FILE;
    }
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn open_stat(
    mut fname: *const i8,
    mut flags: i32,
    mut mode: mode_t,
    mut fstats: *mut file_stats_t,
) -> i32 {
    let mut fd: i32 = 0;
    let mut fdstats: stat = stat {
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
    fd = open(fname, flags, mode);
    if fd < 0 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Failed to open file %s, reason :%s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
            strerror(*__errno_location()),
        );
        return -(1 as i32);
    }
    memset(
        &mut fdstats as *mut stat as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<stat>() as u64,
    );
    if fstat(fd, &mut fdstats) == -(1 as i32) {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Failed to stat file %s, error: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
            strerror(*__errno_location()),
        );
        close(fd);
        return -(1 as i32);
    }
    if !fstats.is_null()
        && (fdstats.st_dev != (*fstats).st_dev || fdstats.st_ino != (*fstats).st_ino)
    {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Trying to open file %s but it changed since last check. Security check failed.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
        );
        close(fd);
        return -(1 as i32);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn make_directory(mut directory: *const i8) -> i32 {
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut quit: i32 = 0 as i32;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut dir: *mut i8 = 0 as *mut i8;
    let mut len: size_t = strlen(directory);
    if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            directory as *const libc::c_void,
            len.wrapping_add(1 as i32 as u64),
        );
        dir = buf.as_mut_ptr();
    } else {
        dir = xstrdup(directory);
    }
    i = (*dir as i32 == '/' as i32) as i32;
    loop {
        while *dir.offset(i as isize) as i32 != 0
            && *dir.offset(i as isize) as i32 != '/' as i32
        {
            i += 1;
            i;
        }
        if *dir.offset(i as isize) == 0 {
            quit = 1 as i32;
        }
        *dir.offset(i as isize) = '\0' as i32 as i8;
        if !file_exists_p(dir, 0 as *mut file_stats_t) {
            ret = mkdir(dir, 0o777 as i32 as __mode_t);
        } else {
            ret = 0 as i32;
        }
        if quit != 0 {
            break;
        }
        *dir.offset(i as isize) = '/' as i32 as i8;
        i += 1;
        i;
    }
    if dir != buf.as_mut_ptr() {
        rpl_free(dir as *mut libc::c_void);
        dir = 0 as *mut i8;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn file_merge(
    mut base: *const i8,
    mut file: *const i8,
) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut cut: *const i8 = strrchr(base, '/' as i32) as *const i8;
    if cut.is_null() {
        return xstrdup(file);
    }
    result = xmalloc(
        ((cut.offset_from(base) as i64 + 1 as i32 as i64) as u64)
            .wrapping_add(strlen(file))
            .wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    memcpy(
        result as *mut libc::c_void,
        base as *const libc::c_void,
        cut.offset_from(base) as i64 as u64,
    );
    *result.offset(cut.offset_from(base) as i64 as isize) = '/' as i32 as i8;
    strcpy(
        result.offset(cut.offset_from(base) as i64 as isize).offset(1 as i32 as isize),
        file,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn fnmatch_nocase(
    mut pattern: *const i8,
    mut string: *const i8,
    mut flags: i32,
) -> i32 {
    return fnmatch(pattern, string, flags | (1 as i32) << 4 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn acceptable(mut s: *const i8) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    if !(opt.output_document).is_null() && strcmp(s, opt.output_document) == 0 as i32 {
        return 1 as i32 != 0;
    }
    p = strrchr(s, '/' as i32);
    if !p.is_null() {
        s = p.offset(1 as i32 as isize);
    }
    if !(opt.accepts).is_null() {
        if !(opt.rejects).is_null() {
            return in_acclist(opt.accepts as *const *const i8, s, 1 as i32 != 0) as i32
                != 0 && !in_acclist(opt.rejects as *const *const i8, s, 1 as i32 != 0)
        } else {
            return in_acclist(opt.accepts as *const *const i8, s, 1 as i32 != 0)
        }
    } else if !(opt.rejects).is_null() {
        return !in_acclist(opt.rejects as *const *const i8, s, 1 as i32 != 0)
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn accept_url(mut s: *const i8) -> bool {
    if !(opt.acceptregex).is_null()
        && !(opt.regex_match_fun).expect("non-null function pointer")(opt.acceptregex, s)
    {
        return 0 as i32 != 0;
    }
    if !(opt.rejectregex).is_null()
        && (opt.regex_match_fun).expect("non-null function pointer")(opt.rejectregex, s)
            as i32 != 0
    {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn subdir_p(mut d1: *const i8, mut d2: *const i8) -> bool {
    if *d1 as i32 == '\0' as i32 {
        return 1 as i32 != 0;
    }
    if !opt.ignore_case {
        while *d1 as i32 != 0 && *d2 as i32 != 0 && *d1 as i32 == *d2 as i32 {
            d1 = d1.offset(1);
            d1;
            d2 = d2.offset(1);
            d2;
        }
    } else {
        while *d1 as i32 != 0 && *d2 as i32 != 0
            && c_tolower(*d1 as i32) == c_tolower(*d2 as i32)
        {
            d1 = d1.offset(1);
            d1;
            d2 = d2.offset(1);
            d2;
        }
    }
    return *d1 as i32 == '\0' as i32
        && (*d2 as i32 == '\0' as i32 || *d2 as i32 == '/' as i32);
}
unsafe extern "C" fn dir_matches_p(
    mut dirlist: *mut *const i8,
    mut dir: *const i8,
) -> bool {
    let mut x: *mut *const i8 = 0 as *mut *const i8;
    let mut matcher: Option<unsafe extern "C" fn(*const i8, *const i8, i32) -> i32> = if opt
        .ignore_case as i32 != 0
    {
        Some(fnmatch_nocase as unsafe extern "C" fn(*const i8, *const i8, i32) -> i32)
    } else {
        Some(fnmatch as unsafe extern "C" fn(*const i8, *const i8, i32) -> i32)
    };
    x = dirlist;
    while !(*x).is_null() {
        let mut p: *const i8 = (*x).offset((**x as i32 == '/' as i32) as i32 as isize);
        if has_wildcards_p(p) {
            if matcher
                .expect("non-null function pointer")(p, dir, (1 as i32) << 0 as i32)
                == 0 as i32
            {
                break;
            }
        } else if subdir_p(p, dir) {
            break;
        }
        x = x.offset(1);
        x;
    }
    return if !(*x).is_null() { 1 as i32 } else { 0 as i32 } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn accdir(mut directory: *const i8) -> bool {
    if *directory as i32 == '/' as i32 {
        directory = directory.offset(1);
        directory;
    }
    if !(opt.includes).is_null() {
        if !dir_matches_p(opt.includes, directory) {
            return 0 as i32 != 0;
        }
    }
    if !(opt.excludes).is_null() {
        if dir_matches_p(opt.excludes, directory) {
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn match_tail(
    mut string: *const i8,
    mut tail: *const i8,
    mut fold_case: bool,
) -> bool {
    let mut pos: i32 = strlen(string) as i32 - strlen(tail) as i32;
    if pos < 0 as i32 {
        return 0 as i32 != 0;
    }
    if !fold_case {
        return strcmp(string.offset(pos as isize), tail) == 0
    } else {
        return strcasecmp(string.offset(pos as isize), tail) == 0
    };
}
unsafe extern "C" fn in_acclist(
    mut accepts: *const *const i8,
    mut s: *const i8,
    mut backward: bool,
) -> bool {
    while !(*accepts).is_null() {
        if has_wildcards_p(*accepts) {
            let mut res: i32 = if opt.ignore_case as i32 != 0 {
                fnmatch_nocase(*accepts, s, 0 as i32)
            } else {
                fnmatch(*accepts, s, 0 as i32)
            };
            if res == 0 as i32 {
                return 1 as i32 != 0;
            }
        } else if backward {
            if match_tail(s, *accepts, opt.ignore_case) {
                return 1 as i32 != 0;
            }
        } else {
            let mut cmp: i32 = if opt.ignore_case as i32 != 0 {
                strcasecmp(s, *accepts)
            } else {
                strcmp(s, *accepts)
            };
            if cmp == 0 as i32 {
                return 1 as i32 != 0;
            }
        }
        accepts = accepts.offset(1);
        accepts;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn suffix(mut str: *const i8) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    p = strrchr(str, '.' as i32);
    if !p.is_null() && (strchr(p.offset(1 as i32 as isize), '/' as i32)).is_null() {
        return p.offset(1 as i32 as isize);
    }
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn has_wildcards_p(mut s: *const i8) -> bool {
    return !(strpbrk(s, b"*?[]\0" as *const u8 as *const i8)).is_null();
}
#[no_mangle]
pub unsafe extern "C" fn has_html_suffix_p(mut fname: *const i8) -> bool {
    let mut suf: *mut i8 = 0 as *mut i8;
    suf = suffix(fname);
    if suf.is_null() {
        return 0 as i32 != 0;
    }
    if c_strcasecmp(suf, b"html\0" as *const u8 as *const i8) == 0 {
        return 1 as i32 != 0;
    }
    if c_strcasecmp(suf, b"htm\0" as *const u8 as *const i8) == 0 {
        return 1 as i32 != 0;
    }
    if *suf.offset(0 as i32 as isize) as i32 != 0
        && c_strcasecmp(
            suf.offset(1 as i32 as isize),
            b"html\0" as *const u8 as *const i8,
        ) == 0
    {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_read_file(mut file: *const i8) -> *mut file_memory {
    let mut current_block: u64;
    let mut fd: i32 = 0;
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut size: i64 = 0;
    let mut inhibit_close: bool = 0 as i32 != 0;
    if *file as i32 == '-' as i32 && *file.offset(1 as i32 as isize) == 0 {
        fd = fileno(stdin);
        inhibit_close = 1 as i32 != 0;
    } else {
        fd = open(file, 0 as i32);
    }
    if fd < 0 as i32 {
        return 0 as *mut file_memory;
    }
    fm = xmalloc(::core::mem::size_of::<file_memory>() as u64) as *mut file_memory;
    let mut buf: stat = stat {
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
    if !(fstat(fd, &mut buf) < 0 as i32) {
        (*fm).length = buf.st_size;
        (*fm).content = mmap(
            0 as *mut libc::c_void,
            (*fm).length as size_t,
            0x1 as i32 | 0x2 as i32,
            0x2 as i32,
            fd,
            0 as i32 as __off_t,
        ) as *mut i8;
        if !((*fm).content == -(1 as i32) as *mut libc::c_void as *mut i8) {
            if !inhibit_close {
                close(fd);
            }
            (*fm).mmap_p = 1 as i32;
            return fm;
        }
    }
    (*fm).length = 0 as i32 as i64;
    size = 512 as i32 as i64;
    (*fm).content = xmalloc(size as size_t) as *mut i8;
    loop {
        let mut nread: wgint = 0;
        if (*fm).length > size / 2 as i32 as i64 {
            size <<= 1 as i32;
            (*fm).content = xrealloc((*fm).content as *mut libc::c_void, size as size_t)
                as *mut i8;
        }
        nread = read(
            fd,
            ((*fm).content).offset((*fm).length as isize) as *mut libc::c_void,
            (size - (*fm).length) as size_t,
        );
        if nread > 0 as i32 as i64 {
            (*fm).length += nread;
        } else if nread < 0 as i32 as i64 {
            current_block = 11102004969301533262;
            break;
        } else {
            current_block = 2719512138335094285;
            break;
        }
    }
    match current_block {
        11102004969301533262 => {
            if !inhibit_close {
                close(fd);
            }
            rpl_free((*fm).content as *mut libc::c_void);
            (*fm).content = 0 as *mut i8;
            rpl_free(fm as *mut libc::c_void);
            fm = 0 as *mut file_memory;
            return 0 as *mut file_memory;
        }
        _ => {
            if !inhibit_close {
                close(fd);
            }
            if size > (*fm).length && (*fm).length != 0 as i32 as i64 {
                (*fm).content = xrealloc(
                    (*fm).content as *mut libc::c_void,
                    (*fm).length as size_t,
                ) as *mut i8;
            }
            (*fm).mmap_p = 0 as i32;
            return fm;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_read_file_free(mut fm: *mut file_memory) {
    if (*fm).mmap_p != 0 {
        munmap((*fm).content as *mut libc::c_void, (*fm).length as size_t);
    } else {
        rpl_free((*fm).content as *mut libc::c_void);
        (*fm).content = 0 as *mut i8;
    }
    rpl_free(fm as *mut libc::c_void);
    fm = 0 as *mut file_memory;
}
#[no_mangle]
pub unsafe extern "C" fn free_vec(mut vec: *mut *mut i8) {
    if !vec.is_null() {
        let mut p: *mut *mut i8 = vec;
        while !(*p).is_null() {
            rpl_free(*p as *mut libc::c_void);
            *p = 0 as *mut i8;
            p = p.offset(1);
            p;
        }
        rpl_free(vec as *mut libc::c_void);
        vec = 0 as *mut *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn merge_vecs(
    mut v1: *mut *mut i8,
    mut v2: *mut *mut i8,
) -> *mut *mut i8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if v1.is_null() {
        return v2;
    }
    if v2.is_null() {
        return v1;
    }
    if (*v2).is_null() {
        rpl_free(v2 as *mut libc::c_void);
        v2 = 0 as *mut *mut i8;
        return v1;
    }
    i = 0 as i32;
    while !(*v1.offset(i as isize)).is_null() {
        i += 1;
        i;
    }
    j = 0 as i32;
    while !(*v2.offset(j as isize)).is_null() {
        j += 1;
        j;
    }
    v1 = xrealloc(
        v1 as *mut libc::c_void,
        ((i + j + 1 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    memcpy(
        v1.offset(i as isize) as *mut libc::c_void,
        v2 as *const libc::c_void,
        ((j + 1 as i32) as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    );
    rpl_free(v2 as *mut libc::c_void);
    v2 = 0 as *mut *mut i8;
    return v1;
}
#[no_mangle]
pub unsafe extern "C" fn vec_append(
    mut vec: *mut *mut i8,
    mut str: *const i8,
) -> *mut *mut i8 {
    let mut cnt: i32 = 0;
    if !vec.is_null() {
        cnt = 0 as i32;
        while !(*vec.offset(cnt as isize)).is_null() {
            cnt += 1;
            cnt;
        }
        cnt += 1;
        cnt;
    } else {
        cnt = 1 as i32;
    }
    vec = xrealloc(
        vec as *mut libc::c_void,
        ((cnt + 1 as i32) as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    let ref mut fresh9 = *vec.offset((cnt - 1 as i32) as isize);
    *fresh9 = xstrdup(str);
    let ref mut fresh10 = *vec.offset(cnt as isize);
    *fresh10 = 0 as *mut i8;
    return vec;
}
#[no_mangle]
pub unsafe extern "C" fn string_set_add(mut ht: *mut hash_table, mut s: *const i8) {
    if hash_table_contains(ht, s as *const libc::c_void) != 0 {
        return;
    }
    hash_table_put(
        ht,
        xstrdup(s) as *const libc::c_void,
        b"1\0" as *const u8 as *const i8 as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn string_set_contains(
    mut ht: *mut hash_table,
    mut s: *const i8,
) -> i32 {
    return hash_table_contains(ht, s as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn string_set_to_array(
    mut ht: *mut hash_table,
    mut array: *mut *mut i8,
) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate(ht, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        let fresh11 = array;
        array = array.offset(1);
        *fresh11 = iter.key as *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn string_set_free(mut ht: *mut hash_table) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate(ht, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        rpl_free(iter.key);
        iter.key = 0 as *mut libc::c_void;
    }
    hash_table_destroy(ht);
}
#[no_mangle]
pub unsafe extern "C" fn free_keys_and_values(mut ht: *mut hash_table) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate(ht, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        rpl_free(iter.key);
        iter.key = 0 as *mut libc::c_void;
        rpl_free(iter.value);
        iter.value = 0 as *mut libc::c_void;
    }
}
unsafe extern "C" fn get_grouping_data(
    mut sep: *mut *const i8,
    mut grouping: *mut *const i8,
) {
    static mut cached_sep: *const i8 = 0 as *const i8;
    static mut cached_grouping: *const i8 = 0 as *const i8;
    static mut initialized: bool = false;
    if !initialized {
        let mut lconv: *mut lconv = localeconv();
        cached_sep = (*lconv).thousands_sep;
        cached_grouping = (*lconv).grouping;
        if *cached_sep == 0 {
            if *(*lconv).decimal_point as i32 != ',' as i32 {
                cached_sep = b",\0" as *const u8 as *const i8;
            } else {
                cached_sep = b".\0" as *const u8 as *const i8;
            }
            cached_grouping = b"\x03\0" as *const u8 as *const i8;
        }
        initialized = 1 as i32 != 0;
    }
    *sep = cached_sep;
    *grouping = cached_grouping;
}
#[no_mangle]
pub unsafe extern "C" fn with_thousand_seps(mut n: wgint) -> *const i8 {
    static mut outbuf: [i8; 48] = [0; 48];
    let mut p: *mut i8 = outbuf
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[i8; 48]>() as u64 as isize);
    let mut grouping: *const i8 = 0 as *const i8;
    let mut sep: *const i8 = 0 as *const i8;
    let mut seplen: i32 = 0;
    let mut i: i32 = 0 as i32;
    let mut groupsize: i32 = 0;
    let mut atgroup: *const i8 = 0 as *const i8;
    let mut negative: bool = n < 0 as i32 as i64;
    get_grouping_data(&mut sep, &mut grouping);
    seplen = strlen(sep) as i32;
    atgroup = grouping;
    let fresh12 = atgroup;
    atgroup = atgroup.offset(1);
    groupsize = *fresh12 as i32;
    if negative {
        n = -n;
    }
    p = p.offset(-1);
    *p = '\0' as i32 as i8;
    loop {
        p = p.offset(-1);
        *p = (n % 10 as i32 as i64 + '0' as i32 as i64) as i8;
        n /= 10 as i32 as i64;
        if n == 0 as i32 as i64 {
            break;
        }
        i += 1;
        if i == groupsize {
            if seplen == 1 as i32 {
                p = p.offset(-1);
                *p = *sep;
            } else {
                p = p.offset(-(seplen as isize));
                memcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    seplen as u64,
                );
            }
            i = 0 as i32;
            if *atgroup != 0 {
                let fresh13 = atgroup;
                atgroup = atgroup.offset(1);
                groupsize = *fresh13 as i32;
            }
        }
    }
    if negative {
        p = p.offset(-1);
        *p = '-' as i32 as i8;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn human_readable(
    mut n: wgint,
    acc: i32,
    decimals: i32,
) -> *mut i8 {
    static mut powers: [i8; 6] = [
        'K' as i32 as i8,
        'M' as i32 as i8,
        'G' as i32 as i8,
        'T' as i32 as i8,
        'P' as i32 as i8,
        'E' as i32 as i8,
    ];
    static mut buf: [i8; 8] = [0; 8];
    let mut i: size_t = 0;
    if n < 1024 as i32 as i64 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 8]>() as u64,
            b"%d\0" as *const u8 as *const i8,
            n as i32,
        );
        return buf.as_mut_ptr();
    }
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[i8; 6]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64)
    {
        if (n / 1024 as i32 as i64) < 1024 as i32 as i64
            || i
                == (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    .wrapping_sub(1 as i32 as u64)
        {
            let mut val: libc::c_double = n as libc::c_double / 1024.0f64;
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 8]>() as u64,
                b"%.*f%c\0" as *const u8 as *const i8,
                if val < acc as libc::c_double { decimals } else { 0 as i32 },
                val,
                powers[i as usize] as i32,
            );
            return buf.as_mut_ptr();
        }
        n /= 1024 as i32 as i64;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn numdigit(mut number: wgint) -> i32 {
    let mut cnt: i32 = 1 as i32;
    if number < 0 as i32 as i64 {
        cnt += 1;
        cnt;
    }
    loop {
        number /= 10 as i32 as i64;
        if !(number != 0 as i32 as i64) {
            break;
        }
        cnt += 1;
        cnt;
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn number_to_string(
    mut buffer: *mut i8,
    mut number: wgint,
) -> *mut i8 {
    let mut p: *mut i8 = buffer;
    let mut n: wgint = number;
    let mut last_digit_char: i32 = 0 as i32;
    if n < 0 as i32 as i64 {
        if n < -(9223372036854775807 as i64) {
            let mut last_digit: i32 = (n % 10 as i32 as i64) as i32;
            if last_digit < 0 as i32 {
                last_digit_char = '0' as i32 - last_digit;
            } else {
                last_digit_char = '0' as i32 + last_digit;
            }
            n /= 10 as i32 as i64;
        }
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = '-' as i32 as i8;
        n = -n;
    }
    if n < 10 as i32 as i64 {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = (n / 1 as i32 as i64 + '0' as i32 as i64) as i8;
    } else if n < 100 as i32 as i64 {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = (n / 10 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 10 as i32 as i64;
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = (n / (10 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
    } else if n < 1000 as i32 as i64 {
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = (n / 100 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 100 as i32 as i64;
        let fresh19 = p;
        p = p.offset(1);
        *fresh19 = (n / (100 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n %= (100 as i32 / 10 as i32) as i64;
        let fresh20 = p;
        p = p.offset(1);
        *fresh20 = (n / (100 as i32 / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64)
            as i8;
    } else if n < 10000 as i32 as i64 {
        let fresh21 = p;
        p = p.offset(1);
        *fresh21 = (n / 1000 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 1000 as i32 as i64;
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = (n / (1000 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n %= (1000 as i32 / 10 as i32) as i64;
        let fresh23 = p;
        p = p.offset(1);
        *fresh23 = (n / (1000 as i32 / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64)
            as i8;
        n %= (1000 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh24 = p;
        p = p.offset(1);
        *fresh24 = (n / (1000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
    } else if n < 100000 as i32 as i64 {
        let fresh25 = p;
        p = p.offset(1);
        *fresh25 = (n / 10000 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 10000 as i32 as i64;
        let fresh26 = p;
        p = p.offset(1);
        *fresh26 = (n / (10000 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n %= (10000 as i32 / 10 as i32) as i64;
        let fresh27 = p;
        p = p.offset(1);
        *fresh27 = (n / (10000 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (10000 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh28 = p;
        p = p.offset(1);
        *fresh28 = (n / (10000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (10000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = (n
            / (10000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
    } else if n < 1000000 as i32 as i64 {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = (n / 100000 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 100000 as i32 as i64;
        let fresh31 = p;
        p = p.offset(1);
        *fresh31 = (n / (100000 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n %= (100000 as i32 / 10 as i32) as i64;
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = (n / (100000 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (100000 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh33 = p;
        p = p.offset(1);
        *fresh33 = (n / (100000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (100000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh34 = p;
        p = p.offset(1);
        *fresh34 = (n
            / (100000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (100000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh35 = p;
        p = p.offset(1);
        *fresh35 = (n
            / (100000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32)
                as i64 + '0' as i32 as i64) as i8;
    } else if n < 10000000 as i32 as i64 {
        let fresh36 = p;
        p = p.offset(1);
        *fresh36 = (n / 1000000 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 1000000 as i32 as i64;
        let fresh37 = p;
        p = p.offset(1);
        *fresh37 = (n / (1000000 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n %= (1000000 as i32 / 10 as i32) as i64;
        let fresh38 = p;
        p = p.offset(1);
        *fresh38 = (n / (1000000 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (1000000 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh39 = p;
        p = p.offset(1);
        *fresh39 = (n / (1000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (1000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh40 = p;
        p = p.offset(1);
        *fresh40 = (n
            / (1000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (1000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh41 = p;
        p = p.offset(1);
        *fresh41 = (n
            / (1000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (1000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64;
        let fresh42 = p;
        p = p.offset(1);
        *fresh42 = (n
            / (1000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64 + '0' as i32 as i64) as i8;
    } else if n < 100000000 as i32 as i64 {
        let fresh43 = p;
        p = p.offset(1);
        *fresh43 = (n / 10000000 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 10000000 as i32 as i64;
        let fresh44 = p;
        p = p.offset(1);
        *fresh44 = (n / (10000000 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n %= (10000000 as i32 / 10 as i32) as i64;
        let fresh45 = p;
        p = p.offset(1);
        *fresh45 = (n / (10000000 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (10000000 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh46 = p;
        p = p.offset(1);
        *fresh46 = (n / (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh47 = p;
        p = p.offset(1);
        *fresh47 = (n
            / (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh48 = p;
        p = p.offset(1);
        *fresh48 = (n
            / (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64;
        let fresh49 = p;
        p = p.offset(1);
        *fresh49 = (n
            / (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32) as i64;
        let fresh50 = p;
        p = p.offset(1);
        *fresh50 = (n
            / (10000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
    } else if n < 1000000000 as i32 as i64 {
        let fresh51 = p;
        p = p.offset(1);
        *fresh51 = (n / 100000000 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 100000000 as i32 as i64;
        let fresh52 = p;
        p = p.offset(1);
        *fresh52 = (n / (100000000 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n %= (100000000 as i32 / 10 as i32) as i64;
        let fresh53 = p;
        p = p.offset(1);
        *fresh53 = (n / (100000000 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (100000000 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh54 = p;
        p = p.offset(1);
        *fresh54 = (n / (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh55 = p;
        p = p.offset(1);
        *fresh55 = (n
            / (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh56 = p;
        p = p.offset(1);
        *fresh56 = (n
            / (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64;
        let fresh57 = p;
        p = p.offset(1);
        *fresh57 = (n
            / (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32) as i64;
        let fresh58 = p;
        p = p.offset(1);
        *fresh58 = (n
            / (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh59 = p;
        p = p.offset(1);
        *fresh59 = (n
            / (100000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
    } else if n < 10 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh60 = p;
        p = p.offset(1);
        *fresh60 = (n / 1000000000 as i32 as i64 + '0' as i32 as i64) as i8;
        n %= 1000000000 as i32 as i64;
        let fresh61 = p;
        p = p.offset(1);
        *fresh61 = (n / (1000000000 as i32 / 10 as i32) as i64 + '0' as i32 as i64)
            as i8;
        n %= (1000000000 as i32 / 10 as i32) as i64;
        let fresh62 = p;
        p = p.offset(1);
        *fresh62 = (n / (1000000000 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (1000000000 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh63 = p;
        p = p.offset(1);
        *fresh63 = (n / (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh64 = p;
        p = p.offset(1);
        *fresh64 = (n
            / (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n %= (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh65 = p;
        p = p.offset(1);
        *fresh65 = (n
            / (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32) as i64;
        let fresh66 = p;
        p = p.offset(1);
        *fresh66 = (n
            / (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32) as i64;
        let fresh67 = p;
        p = p.offset(1);
        *fresh67 = (n
            / (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32) as i64 + '0' as i32 as i64) as i8;
        n
            %= (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh68 = p;
        p = p.offset(1);
        *fresh68 = (n
            / (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
        n
            %= (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64;
        let fresh69 = p;
        p = p.offset(1);
        *fresh69 = (n
            / (1000000000 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32
                / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32 / 10 as i32) as i64
            + '0' as i32 as i64) as i8;
    } else if n < 100 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh70 = p;
        p = p.offset(1);
        *fresh70 = (n / (10 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 10 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh71 = p;
        p = p.offset(1);
        *fresh71 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh72 = p;
        p = p.offset(1);
        *fresh72 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh73 = p;
        p = p.offset(1);
        *fresh73 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh74 = p;
        p = p.offset(1);
        *fresh74 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh75 = p;
        p = p.offset(1);
        *fresh75 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh76 = p;
        p = p.offset(1);
        *fresh76 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh77 = p;
        p = p.offset(1);
        *fresh77 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh78 = p;
        p = p.offset(1);
        *fresh78 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh79 = p;
        p = p.offset(1);
        *fresh79 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh80 = p;
        p = p.offset(1);
        *fresh80 = (n
            / (10 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
    } else if n < 1000 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh81 = p;
        p = p.offset(1);
        *fresh81 = (n / (100 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 100 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh82 = p;
        p = p.offset(1);
        *fresh82 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh83 = p;
        p = p.offset(1);
        *fresh83 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh84 = p;
        p = p.offset(1);
        *fresh84 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh85 = p;
        p = p.offset(1);
        *fresh85 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh86 = p;
        p = p.offset(1);
        *fresh86 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh87 = p;
        p = p.offset(1);
        *fresh87 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh88 = p;
        p = p.offset(1);
        *fresh88 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh89 = p;
        p = p.offset(1);
        *fresh89 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh90 = p;
        p = p.offset(1);
        *fresh90 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh91 = p;
        p = p.offset(1);
        *fresh91 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh92 = p;
        p = p.offset(1);
        *fresh92 = (n
            / (100 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
    } else if n < 10000 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh93 = p;
        p = p.offset(1);
        *fresh93 = (n / (1000 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 1000 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh94 = p;
        p = p.offset(1);
        *fresh94 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh95 = p;
        p = p.offset(1);
        *fresh95 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh96 = p;
        p = p.offset(1);
        *fresh96 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh97 = p;
        p = p.offset(1);
        *fresh97 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh98 = p;
        p = p.offset(1);
        *fresh98 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh99 = p;
        p = p.offset(1);
        *fresh99 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh100 = p;
        p = p.offset(1);
        *fresh100 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh101 = p;
        p = p.offset(1);
        *fresh101 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh102 = p;
        p = p.offset(1);
        *fresh102 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh103 = p;
        p = p.offset(1);
        *fresh103 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh104 = p;
        p = p.offset(1);
        *fresh104 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh105 = p;
        p = p.offset(1);
        *fresh105 = (n
            / (1000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
    } else if n < 100000 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh106 = p;
        p = p.offset(1);
        *fresh106 = (n / (10000 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 10000 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh107 = p;
        p = p.offset(1);
        *fresh107 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh108 = p;
        p = p.offset(1);
        *fresh108 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh109 = p;
        p = p.offset(1);
        *fresh109 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh110 = p;
        p = p.offset(1);
        *fresh110 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh111 = p;
        p = p.offset(1);
        *fresh111 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh112 = p;
        p = p.offset(1);
        *fresh112 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh113 = p;
        p = p.offset(1);
        *fresh113 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh114 = p;
        p = p.offset(1);
        *fresh114 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh115 = p;
        p = p.offset(1);
        *fresh115 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh116 = p;
        p = p.offset(1);
        *fresh116 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh117 = p;
        p = p.offset(1);
        *fresh117 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh118 = p;
        p = p.offset(1);
        *fresh118 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh119 = p;
        p = p.offset(1);
        *fresh119 = (n
            / (10000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
    } else if n < 1000000 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh120 = p;
        p = p.offset(1);
        *fresh120 = (n / (100000 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 100000 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh121 = p;
        p = p.offset(1);
        *fresh121 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh122 = p;
        p = p.offset(1);
        *fresh122 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh123 = p;
        p = p.offset(1);
        *fresh123 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh124 = p;
        p = p.offset(1);
        *fresh124 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh125 = p;
        p = p.offset(1);
        *fresh125 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh126 = p;
        p = p.offset(1);
        *fresh126 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh127 = p;
        p = p.offset(1);
        *fresh127 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh128 = p;
        p = p.offset(1);
        *fresh128 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh129 = p;
        p = p.offset(1);
        *fresh129 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh130 = p;
        p = p.offset(1);
        *fresh130 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh131 = p;
        p = p.offset(1);
        *fresh131 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh132 = p;
        p = p.offset(1);
        *fresh132 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh133 = p;
        p = p.offset(1);
        *fresh133 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh134 = p;
        p = p.offset(1);
        *fresh134 = (n
            / (100000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
    } else if n < 10000000 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh135 = p;
        p = p.offset(1);
        *fresh135 = (n / (1000000 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh136 = p;
        p = p.offset(1);
        *fresh136 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh137 = p;
        p = p.offset(1);
        *fresh137 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh138 = p;
        p = p.offset(1);
        *fresh138 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh139 = p;
        p = p.offset(1);
        *fresh139 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh140 = p;
        p = p.offset(1);
        *fresh140 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh141 = p;
        p = p.offset(1);
        *fresh141 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh142 = p;
        p = p.offset(1);
        *fresh142 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh143 = p;
        p = p.offset(1);
        *fresh143 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh144 = p;
        p = p.offset(1);
        *fresh144 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh145 = p;
        p = p.offset(1);
        *fresh145 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh146 = p;
        p = p.offset(1);
        *fresh146 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh147 = p;
        p = p.offset(1);
        *fresh147 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh148 = p;
        p = p.offset(1);
        *fresh148 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh149 = p;
        p = p.offset(1);
        *fresh149 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh150 = p;
        p = p.offset(1);
        *fresh150 = (n
            / (1000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
    } else if n < 100000000 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh151 = p;
        p = p.offset(1);
        *fresh151 = (n / (10000000 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh152 = p;
        p = p.offset(1);
        *fresh152 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh153 = p;
        p = p.offset(1);
        *fresh153 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh154 = p;
        p = p.offset(1);
        *fresh154 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh155 = p;
        p = p.offset(1);
        *fresh155 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh156 = p;
        p = p.offset(1);
        *fresh156 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh157 = p;
        p = p.offset(1);
        *fresh157 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh158 = p;
        p = p.offset(1);
        *fresh158 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh159 = p;
        p = p.offset(1);
        *fresh159 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh160 = p;
        p = p.offset(1);
        *fresh160 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh161 = p;
        p = p.offset(1);
        *fresh161 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh162 = p;
        p = p.offset(1);
        *fresh162 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh163 = p;
        p = p.offset(1);
        *fresh163 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh164 = p;
        p = p.offset(1);
        *fresh164 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh165 = p;
        p = p.offset(1);
        *fresh165 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh166 = p;
        p = p.offset(1);
        *fresh166 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh167 = p;
        p = p.offset(1);
        *fresh167 = (n
            / (10000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
    } else if n < 1000000000 as i32 as i64 * 1000000000 as i32 as wgint {
        let fresh168 = p;
        p = p.offset(1);
        *fresh168 = (n / (100000000 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh169 = p;
        p = p.offset(1);
        *fresh169 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh170 = p;
        p = p.offset(1);
        *fresh170 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh171 = p;
        p = p.offset(1);
        *fresh171 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh172 = p;
        p = p.offset(1);
        *fresh172 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh173 = p;
        p = p.offset(1);
        *fresh173 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh174 = p;
        p = p.offset(1);
        *fresh174 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh175 = p;
        p = p.offset(1);
        *fresh175 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh176 = p;
        p = p.offset(1);
        *fresh176 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh177 = p;
        p = p.offset(1);
        *fresh177 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh178 = p;
        p = p.offset(1);
        *fresh178 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh179 = p;
        p = p.offset(1);
        *fresh179 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh180 = p;
        p = p.offset(1);
        *fresh180 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh181 = p;
        p = p.offset(1);
        *fresh181 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh182 = p;
        p = p.offset(1);
        *fresh182 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh183 = p;
        p = p.offset(1);
        *fresh183 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh184 = p;
        p = p.offset(1);
        *fresh184 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh185 = p;
        p = p.offset(1);
        *fresh185 = (n
            / (100000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
    } else {
        let fresh186 = p;
        p = p.offset(1);
        *fresh186 = (n / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint)
            + '0' as i32 as i64) as i8;
        n %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint;
        let fresh187 = p;
        p = p.offset(1);
        *fresh187 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64;
        let fresh188 = p;
        p = p.offset(1);
        *fresh188 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh189 = p;
        p = p.offset(1);
        *fresh189 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh190 = p;
        p = p.offset(1);
        *fresh190 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh191 = p;
        p = p.offset(1);
        *fresh191 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh192 = p;
        p = p.offset(1);
        *fresh192 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh193 = p;
        p = p.offset(1);
        *fresh193 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh194 = p;
        p = p.offset(1);
        *fresh194 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh195 = p;
        p = p.offset(1);
        *fresh195 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh196 = p;
        p = p.offset(1);
        *fresh196 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh197 = p;
        p = p.offset(1);
        *fresh197 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh198 = p;
        p = p.offset(1);
        *fresh198 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh199 = p;
        p = p.offset(1);
        *fresh199 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh200 = p;
        p = p.offset(1);
        *fresh200 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh201 = p;
        p = p.offset(1);
        *fresh201 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh202 = p;
        p = p.offset(1);
        *fresh202 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64)
            + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64;
        let fresh203 = p;
        p = p.offset(1);
        *fresh203 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64) + '0' as i32 as i64) as i8;
        n
            %= 1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64;
        let fresh204 = p;
        p = p.offset(1);
        *fresh204 = (n
            / (1000000000 as i32 as i64 * 1000000000 as i32 as wgint / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64 / 10 as i32 as i64
                / 10 as i32 as i64 / 10 as i32 as i64) + '0' as i32 as i64) as i8;
    }
    if last_digit_char != 0 {
        let fresh205 = p;
        p = p.offset(1);
        *fresh205 = last_digit_char as i8;
    }
    *p = '\0' as i32 as i8;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn number_to_static_string(mut number: wgint) -> *mut i8 {
    static mut ring: [[i8; 24]; 3] = [[0; 24]; 3];
    static mut ringpos: i32 = 0;
    let mut buf: *mut i8 = (ring[ringpos as usize]).as_mut_ptr();
    number_to_string(buf, number);
    ringpos = (ringpos + 1 as i32) % 3 as i32;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn convert_to_bits(mut num: wgint) -> wgint {
    if opt.report_bps {
        return num * 8 as i32 as i64;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn determine_screen_width() -> i32 {
    let mut fd: i32 = 0;
    let mut wsz: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if !(opt.lfilename).is_null() && opt.show_progress != 1 as i32 {
        return 0 as i32;
    }
    fd = fileno(stderr);
    if rpl_ioctl(fd, 0x5413 as i32, &mut wsz as *mut winsize) < 0 as i32 {
        return 0 as i32;
    }
    return wsz.ws_col as i32;
}
static mut rnd_seeded: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn random_number(mut max: i32) -> i32 {
    if rnd_seeded == 0 {
        srandom((time(0 as *mut time_t) ^ getpid() as i64) as u32);
        rnd_seeded = 1 as i32;
    }
    return (random() % max as i64) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn random_float() -> libc::c_double {
    return random_number(2147483647 as i32) as libc::c_double
        / 2147483647 as i32 as libc::c_double;
}
static mut run_with_timeout_env: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn abort_run_with_timeout(mut sig: i32) {
    siglongjmp(run_with_timeout_env.as_mut_ptr(), -(1 as i32));
}
unsafe extern "C" fn alarm_set(mut timeout: libc::c_double) {
    let mut itv: itimerval = itimerval {
        it_interval: timeval { tv_sec: 0, tv_usec: 0 },
        it_value: timeval { tv_sec: 0, tv_usec: 0 },
    };
    memset(
        &mut itv as *mut itimerval as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<itimerval>() as u64,
    );
    itv.it_value.tv_sec = timeout as i64;
    itv.it_value.tv_usec = (1000000 as i32 as libc::c_double
        * (timeout - timeout as i64 as libc::c_double)) as __suseconds_t;
    if itv.it_value.tv_sec == 0 as i32 as i64 && itv.it_value.tv_usec == 0 as i32 as i64
    {
        itv.it_value.tv_usec = 1 as i32 as __suseconds_t;
    }
    setitimer(__itimer_which::ITIMER_REAL, &mut itv, 0 as *mut itimerval);
}
unsafe extern "C" fn alarm_cancel() {
    let mut disable: itimerval = itimerval {
        it_interval: timeval { tv_sec: 0, tv_usec: 0 },
        it_value: timeval { tv_sec: 0, tv_usec: 0 },
    };
    memset(
        &mut disable as *mut itimerval as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<itimerval>() as u64,
    );
    setitimer(__itimer_which::ITIMER_REAL, &mut disable, 0 as *mut itimerval);
}
#[no_mangle]
pub unsafe extern "C" fn run_with_timeout(
    mut timeout: libc::c_double,
    mut fun: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> bool {
    let mut saved_errno: i32 = 0;
    if timeout == 0 as i32 as libc::c_double {
        fun.expect("non-null function pointer")(arg);
        return 0 as i32 != 0;
    }
    if __sigsetjmp(run_with_timeout_env.as_mut_ptr(), 1 as i32) != 0 as i32 {
        signal(14 as i32, None);
        return 1 as i32 != 0;
    } else {
        signal(
            14 as i32,
            Some(abort_run_with_timeout as unsafe extern "C" fn(i32) -> ()),
        );
    }
    alarm_set(timeout);
    fun.expect("non-null function pointer")(arg);
    saved_errno = *__errno_location();
    alarm_cancel();
    signal(14 as i32, None);
    *__errno_location() = saved_errno;
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn xsleep(mut seconds: libc::c_double) {
    if seconds >= 1 as i32 as libc::c_double {
        sleep(seconds as u32);
        seconds -= seconds as i64 as libc::c_double;
    }
    usleep((seconds * 1000000 as i32 as libc::c_double) as __useconds_t);
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_encode(
    mut data: *const libc::c_void,
    mut length: size_t,
    mut dest: *mut i8,
) -> size_t {
    static mut tbl: [i8; 64] = [
        'A' as i32 as i8,
        'B' as i32 as i8,
        'C' as i32 as i8,
        'D' as i32 as i8,
        'E' as i32 as i8,
        'F' as i32 as i8,
        'G' as i32 as i8,
        'H' as i32 as i8,
        'I' as i32 as i8,
        'J' as i32 as i8,
        'K' as i32 as i8,
        'L' as i32 as i8,
        'M' as i32 as i8,
        'N' as i32 as i8,
        'O' as i32 as i8,
        'P' as i32 as i8,
        'Q' as i32 as i8,
        'R' as i32 as i8,
        'S' as i32 as i8,
        'T' as i32 as i8,
        'U' as i32 as i8,
        'V' as i32 as i8,
        'W' as i32 as i8,
        'X' as i32 as i8,
        'Y' as i32 as i8,
        'Z' as i32 as i8,
        'a' as i32 as i8,
        'b' as i32 as i8,
        'c' as i32 as i8,
        'd' as i32 as i8,
        'e' as i32 as i8,
        'f' as i32 as i8,
        'g' as i32 as i8,
        'h' as i32 as i8,
        'i' as i32 as i8,
        'j' as i32 as i8,
        'k' as i32 as i8,
        'l' as i32 as i8,
        'm' as i32 as i8,
        'n' as i32 as i8,
        'o' as i32 as i8,
        'p' as i32 as i8,
        'q' as i32 as i8,
        'r' as i32 as i8,
        's' as i32 as i8,
        't' as i32 as i8,
        'u' as i32 as i8,
        'v' as i32 as i8,
        'w' as i32 as i8,
        'x' as i32 as i8,
        'y' as i32 as i8,
        'z' as i32 as i8,
        '0' as i32 as i8,
        '1' as i32 as i8,
        '2' as i32 as i8,
        '3' as i32 as i8,
        '4' as i32 as i8,
        '5' as i32 as i8,
        '6' as i32 as i8,
        '7' as i32 as i8,
        '8' as i32 as i8,
        '9' as i32 as i8,
        '+' as i32 as i8,
        '/' as i32 as i8,
    ];
    let mut s: *const u8 = data as *const u8;
    let mut end: *const u8 = (data as *const u8)
        .offset(length as isize)
        .offset(-(2 as i32 as isize));
    let mut p: *mut i8 = dest;
    while s < end {
        let fresh206 = p;
        p = p.offset(1);
        *fresh206 = tbl[(*s.offset(0 as i32 as isize) as i32 >> 2 as i32) as usize];
        let fresh207 = p;
        p = p.offset(1);
        *fresh207 = tbl[(((*s.offset(0 as i32 as isize) as i32 & 3 as i32) << 4 as i32)
            + (*s.offset(1 as i32 as isize) as i32 >> 4 as i32)) as usize];
        let fresh208 = p;
        p = p.offset(1);
        *fresh208 = tbl[(((*s.offset(1 as i32 as isize) as i32 & 0xf as i32) << 2 as i32)
            + (*s.offset(2 as i32 as isize) as i32 >> 6 as i32)) as usize];
        let fresh209 = p;
        p = p.offset(1);
        *fresh209 = tbl[(*s.offset(2 as i32 as isize) as i32 & 0x3f as i32) as usize];
        s = s.offset(3 as i32 as isize);
    }
    match length.wrapping_rem(3 as i32 as u64) {
        1 => {
            let fresh210 = p;
            p = p.offset(1);
            *fresh210 = tbl[(*s.offset(0 as i32 as isize) as i32 >> 2 as i32) as usize];
            let fresh211 = p;
            p = p.offset(1);
            *fresh211 = tbl[((*s.offset(0 as i32 as isize) as i32 & 3 as i32)
                << 4 as i32) as usize];
            let fresh212 = p;
            p = p.offset(1);
            *fresh212 = '=' as i32 as i8;
            let fresh213 = p;
            p = p.offset(1);
            *fresh213 = '=' as i32 as i8;
        }
        2 => {
            let fresh214 = p;
            p = p.offset(1);
            *fresh214 = tbl[(*s.offset(0 as i32 as isize) as i32 >> 2 as i32) as usize];
            let fresh215 = p;
            p = p.offset(1);
            *fresh215 = tbl[(((*s.offset(0 as i32 as isize) as i32 & 3 as i32)
                << 4 as i32) + (*s.offset(1 as i32 as isize) as i32 >> 4 as i32))
                as usize];
            let fresh216 = p;
            p = p.offset(1);
            *fresh216 = tbl[((*s.offset(1 as i32 as isize) as i32 & 0xf as i32)
                << 2 as i32) as usize];
            let fresh217 = p;
            p = p.offset(1);
            *fresh217 = '=' as i32 as i8;
        }
        _ => {}
    }
    *p = '\0' as i32 as i8;
    return p.offset_from(dest) as i64 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_decode(
    mut base64: *const i8,
    mut dest: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    static mut base64_char_to_value: [libc::c_schar; 128] = [
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        62 as i32 as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        63 as i32 as libc::c_schar,
        52 as i32 as libc::c_schar,
        53 as i32 as libc::c_schar,
        54 as i32 as libc::c_schar,
        55 as i32 as libc::c_schar,
        56 as i32 as libc::c_schar,
        57 as i32 as libc::c_schar,
        58 as i32 as libc::c_schar,
        59 as i32 as libc::c_schar,
        60 as i32 as libc::c_schar,
        61 as i32 as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        0 as i32 as libc::c_schar,
        1 as i32 as libc::c_schar,
        2 as i32 as libc::c_schar,
        3 as i32 as libc::c_schar,
        4 as i32 as libc::c_schar,
        5 as i32 as libc::c_schar,
        6 as i32 as libc::c_schar,
        7 as i32 as libc::c_schar,
        8 as i32 as libc::c_schar,
        9 as i32 as libc::c_schar,
        10 as i32 as libc::c_schar,
        11 as i32 as libc::c_schar,
        12 as i32 as libc::c_schar,
        13 as i32 as libc::c_schar,
        14 as i32 as libc::c_schar,
        15 as i32 as libc::c_schar,
        16 as i32 as libc::c_schar,
        17 as i32 as libc::c_schar,
        18 as i32 as libc::c_schar,
        19 as i32 as libc::c_schar,
        20 as i32 as libc::c_schar,
        21 as i32 as libc::c_schar,
        22 as i32 as libc::c_schar,
        23 as i32 as libc::c_schar,
        24 as i32 as libc::c_schar,
        25 as i32 as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        26 as i32 as libc::c_schar,
        27 as i32 as libc::c_schar,
        28 as i32 as libc::c_schar,
        29 as i32 as libc::c_schar,
        30 as i32 as libc::c_schar,
        31 as i32 as libc::c_schar,
        32 as i32 as libc::c_schar,
        33 as i32 as libc::c_schar,
        34 as i32 as libc::c_schar,
        35 as i32 as libc::c_schar,
        36 as i32 as libc::c_schar,
        37 as i32 as libc::c_schar,
        38 as i32 as libc::c_schar,
        39 as i32 as libc::c_schar,
        40 as i32 as libc::c_schar,
        41 as i32 as libc::c_schar,
        42 as i32 as libc::c_schar,
        43 as i32 as libc::c_schar,
        44 as i32 as libc::c_schar,
        45 as i32 as libc::c_schar,
        46 as i32 as libc::c_schar,
        47 as i32 as libc::c_schar,
        48 as i32 as libc::c_schar,
        49 as i32 as libc::c_schar,
        50 as i32 as libc::c_schar,
        51 as i32 as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
        -(1 as i32) as libc::c_schar,
    ];
    let mut p: *const i8 = base64;
    let mut q: *mut u8 = dest as *mut u8;
    let mut n: ssize_t = 0 as i32 as ssize_t;
    loop {
        let mut c: u8 = 0;
        let mut value: u64 = 0;
        loop {
            let fresh218 = p;
            p = p.offset(1);
            c = *fresh218 as u8;
            if !c_isspace(c as i32) {
                break;
            }
        }
        if c == 0 {
            break;
        }
        if c as i32 == '=' as i32
            || !(c as i32 & 0x80 as i32 == 0 as i32
                && base64_char_to_value[c as usize] as i32 >= 0 as i32
                || c as i32 == '=' as i32)
        {
            return -(1 as i32) as ssize_t;
        }
        value = ((base64_char_to_value[c as usize] as i32) << 18 as i32) as u64;
        loop {
            let fresh219 = p;
            p = p.offset(1);
            c = *fresh219 as u8;
            if !c_isspace(c as i32) {
                break;
            }
        }
        if c == 0 {
            return -(1 as i32) as ssize_t;
        }
        if c as i32 == '=' as i32
            || !(c as i32 & 0x80 as i32 == 0 as i32
                && base64_char_to_value[c as usize] as i32 >= 0 as i32
                || c as i32 == '=' as i32)
        {
            return -(1 as i32) as ssize_t;
        }
        value |= ((base64_char_to_value[c as usize] as i32) << 12 as i32) as u64;
        if size != 0 {
            let fresh220 = q;
            q = q.offset(1);
            *fresh220 = (value >> 16 as i32) as u8;
            size = size.wrapping_sub(1);
            size;
        }
        n += 1;
        n;
        loop {
            let fresh221 = p;
            p = p.offset(1);
            c = *fresh221 as u8;
            if !c_isspace(c as i32) {
                break;
            }
        }
        if c == 0 {
            return -(1 as i32) as ssize_t;
        }
        if !(c as i32 & 0x80 as i32 == 0 as i32
            && base64_char_to_value[c as usize] as i32 >= 0 as i32
            || c as i32 == '=' as i32)
        {
            return -(1 as i32) as ssize_t;
        }
        if c as i32 == '=' as i32 {
            loop {
                let fresh222 = p;
                p = p.offset(1);
                c = *fresh222 as u8;
                if !c_isspace(c as i32) {
                    break;
                }
            }
            if c == 0 {
                return -(1 as i32) as ssize_t;
            }
            if c as i32 != '=' as i32 {
                return -(1 as i32) as ssize_t;
            }
        } else {
            value |= ((base64_char_to_value[c as usize] as i32) << 6 as i32) as u64;
            if size != 0 {
                let fresh223 = q;
                q = q.offset(1);
                *fresh223 = (0xff as i32 as u64 & value >> 8 as i32) as u8;
                size = size.wrapping_sub(1);
                size;
            }
            n += 1;
            n;
            loop {
                let fresh224 = p;
                p = p.offset(1);
                c = *fresh224 as u8;
                if !c_isspace(c as i32) {
                    break;
                }
            }
            if c == 0 {
                return -(1 as i32) as ssize_t;
            }
            if c as i32 == '=' as i32 {
                continue;
            }
            if !(c as i32 & 0x80 as i32 == 0 as i32
                && base64_char_to_value[c as usize] as i32 >= 0 as i32
                || c as i32 == '=' as i32)
            {
                return -(1 as i32) as ssize_t;
            }
            value |= base64_char_to_value[c as usize] as i32 as u64;
            if size != 0 {
                let fresh225 = q;
                q = q.offset(1);
                *fresh225 = (0xff as i32 as u64 & value) as u8;
                size = size.wrapping_sub(1);
                size;
            }
            n += 1;
            n;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn compile_pcre_regex(mut str: *const i8) -> *mut libc::c_void {
    let mut errbuf: *const i8 = 0 as *const i8;
    let mut erroffset: i32 = 0;
    let mut regex: *mut pcre = pcre_compile(
        str,
        0 as i32,
        &mut errbuf,
        &mut erroffset,
        0 as *const u8,
    );
    if regex.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Invalid regular expression %s, %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(str),
            errbuf,
        );
    }
    return regex as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn compile_posix_regex(mut str: *const i8) -> *mut libc::c_void {
    let mut regex: *mut regex_t = xmalloc(::core::mem::size_of::<regex_t>() as u64)
        as *mut regex_t;
    let mut errcode: i32 = rpl_regcomp(regex, str, 1 as i32 | (1 as i32) << 3 as i32);
    if errcode != 0 as i32 {
        let mut errbuf_size: size_t = rpl_regerror(
            errcode,
            regex,
            0 as *mut i8,
            0 as i32 as size_t,
        );
        let mut errbuf: *mut i8 = xmalloc(errbuf_size) as *mut i8;
        rpl_regerror(errcode, regex, errbuf, errbuf_size);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Invalid regular expression %s, %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(str),
            errbuf,
        );
        rpl_free(errbuf as *mut libc::c_void);
        errbuf = 0 as *mut i8;
        rpl_free(regex as *mut libc::c_void);
        regex = 0 as *mut regex_t;
        return 0 as *mut libc::c_void;
    }
    return regex as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn match_pcre_regex(
    mut regex: *const libc::c_void,
    mut str: *const i8,
) -> bool {
    let mut l: size_t = strlen(str);
    let mut ovector: [i32; 30] = [0; 30];
    let mut rc: i32 = pcre_exec(
        regex as *mut pcre,
        0 as *const pcre_extra,
        str,
        l as i32,
        0 as i32,
        0 as i32,
        ovector.as_mut_ptr(),
        30 as i32,
    );
    if rc == -(1 as i32) {
        return 0 as i32 != 0
    } else if rc < 0 as i32 {
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Error while matching %s: %d\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(str),
            rc,
        );
        return 0 as i32 != 0;
    } else {
        return 1 as i32 != 0
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_posix_regex(
    mut regex: *const libc::c_void,
    mut str: *const i8,
) -> bool {
    let mut rc: i32 = rpl_regexec(
        regex as *mut regex_t,
        str,
        0 as i32 as size_t,
        0 as *mut regmatch_t,
        0 as i32,
    );
    if rc == _REG_NOMATCH as i32 {
        return 0 as i32 != 0
    } else if rc == 0 as i32 {
        return 1 as i32 != 0
    } else {
        let mut errbuf_size: size_t = rpl_regerror(
            rc,
            opt.acceptregex as *const regex_t,
            0 as *mut i8,
            0 as i32 as size_t,
        );
        let mut errbuf: *mut i8 = xmalloc(errbuf_size) as *mut i8;
        rpl_regerror(rc, opt.acceptregex as *const regex_t, errbuf, errbuf_size);
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Error while matching %s: %d\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(str),
            rc,
        );
        rpl_free(errbuf as *mut libc::c_void);
        errbuf = 0 as *mut i8;
        return 0 as i32 != 0;
    };
}
unsafe extern "C" fn mergesort_internal(
    mut base: *mut libc::c_void,
    mut temp: *mut libc::c_void,
    mut size: size_t,
    mut from: size_t,
    mut to: size_t,
    mut cmpfun: Option<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
    >,
) {
    if from < to {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut k: size_t = 0;
        let mut mid: size_t = to.wrapping_add(from).wrapping_div(2 as i32 as u64);
        mergesort_internal(base, temp, size, from, mid, cmpfun);
        mergesort_internal(
            base,
            temp,
            size,
            mid.wrapping_add(1 as i32 as u64),
            to,
            cmpfun,
        );
        i = from;
        j = mid.wrapping_add(1 as i32 as u64);
        k = from;
        while i <= mid && j <= to {
            if cmpfun
                .expect(
                    "non-null function pointer",
                )(
                (base as *mut i8).offset(i.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                (base as *mut i8).offset(j.wrapping_mul(size) as isize)
                    as *const libc::c_void,
            ) <= 0 as i32
            {
                let fresh226 = i;
                i = i.wrapping_add(1);
                memcpy(
                    (temp as *mut i8).offset(k.wrapping_mul(size) as isize)
                        as *mut libc::c_void,
                    (base as *mut i8).offset(fresh226.wrapping_mul(size) as isize)
                        as *const libc::c_void,
                    size,
                );
            } else {
                let fresh227 = j;
                j = j.wrapping_add(1);
                memcpy(
                    (temp as *mut i8).offset(k.wrapping_mul(size) as isize)
                        as *mut libc::c_void,
                    (base as *mut i8).offset(fresh227.wrapping_mul(size) as isize)
                        as *const libc::c_void,
                    size,
                );
            }
            k = k.wrapping_add(1);
            k;
        }
        while i <= mid {
            let fresh228 = k;
            k = k.wrapping_add(1);
            let fresh229 = i;
            i = i.wrapping_add(1);
            memcpy(
                (temp as *mut i8).offset(fresh228.wrapping_mul(size) as isize)
                    as *mut libc::c_void,
                (base as *mut i8).offset(fresh229.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                size,
            );
        }
        while j <= to {
            let fresh230 = k;
            k = k.wrapping_add(1);
            let fresh231 = j;
            j = j.wrapping_add(1);
            memcpy(
                (temp as *mut i8).offset(fresh230.wrapping_mul(size) as isize)
                    as *mut libc::c_void,
                (base as *mut i8).offset(fresh231.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                size,
            );
        }
        k = from;
        while k <= to {
            memcpy(
                (base as *mut i8).offset(k.wrapping_mul(size) as isize)
                    as *mut libc::c_void,
                (temp as *mut i8).offset(k.wrapping_mul(size) as isize)
                    as *const libc::c_void,
                size,
            );
            k = k.wrapping_add(1);
            k;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn stable_sort(
    mut base: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmpfun: Option<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
    >,
) {
    if nmemb > 1 as i32 as u64 && size > 1 as i32 as u64 {
        let mut temp: *mut libc::c_void = xmalloc(nmemb.wrapping_mul(size));
        mergesort_internal(
            base,
            temp,
            size,
            0 as i32 as size_t,
            nmemb.wrapping_sub(1 as i32 as u64),
            cmpfun,
        );
        rpl_free(temp);
        temp = 0 as *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_decimal(mut number: libc::c_double) -> *const i8 {
    static mut buf: [i8; 32] = [0; 32];
    let mut n: libc::c_double = if number >= 0 as i32 as libc::c_double {
        number
    } else {
        -number
    };
    if n >= 9.95f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%.0f\0" as *const u8 as *const i8,
            number,
        );
    } else if n >= 0.95f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%.1f\0" as *const u8 as *const i8,
            number,
        );
    } else if n >= 0.001f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%.1g\0" as *const u8 as *const i8,
            number,
        );
    } else if n >= 0.0005f64 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%.3f\0" as *const u8 as *const i8,
            number,
        );
    } else {
        strcpy(buf.as_mut_ptr(), b"0\0" as *const u8 as *const i8);
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn get_max_length(
    mut path: *const i8,
    mut length: i32,
    mut name: i32,
) -> i64 {
    let mut ret: i64 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut d: *mut i8 = 0 as *mut i8;
    p = if !path.is_null() {
        strdupdelim(path, path.offset(length as isize))
    } else {
        strdup(b"\0" as *const u8 as *const i8)
    };
    loop {
        *__errno_location() = 0 as i32;
        ret = pathconf(
            if *p as i32 != 0 { p } else { b".\0" as *const u8 as *const i8 },
            name,
        );
        if !(ret < 0 as i32 as i64 && *__errno_location() == 2 as i32) {
            break;
        }
        if *p == 0 || strcmp(p, b"/\0" as *const u8 as *const i8) == 0 as i32 {
            break;
        }
        d = strrchr(p, '/' as i32);
        if d == p {
            *p.offset(1 as i32 as isize) = '\0' as i32 as i8;
        } else if !d.is_null() {
            *d = '\0' as i32 as i8;
        } else {
            *p = '\0' as i32 as i8;
        }
    }
    rpl_free(p as *mut libc::c_void);
    p = 0 as *mut i8;
    if ret < 0 as i32 as i64 {
        if *__errno_location() != 0 as i32 {
            perror(b"pathconf\0" as *const u8 as *const i8);
        }
        return 0 as i32 as i64;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wg_hex_to_string(
    mut str_buffer: *mut i8,
    mut hex_buffer: *const i8,
    mut hex_len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < hex_len {
        sprintf(
            str_buffer.offset((2 as i32 as u64).wrapping_mul(i) as isize),
            b"%02x\0" as *const u8 as *const i8,
            (*hex_buffer.offset(i as isize) as i32 & 0xff as i32) as u32,
        );
        i = i.wrapping_add(1);
        i;
    }
    *str_buffer.offset((2 as i32 as u64).wrapping_mul(i) as isize) = '\0' as i32 as i8;
}
unsafe extern "C" fn wg_pubkey_pem_to_der(
    mut pem: *const i8,
    mut der: *mut *mut u8,
    mut der_len: *mut size_t,
) -> bool {
    let mut stripped_pem: *mut i8 = 0 as *mut i8;
    let mut begin_pos: *mut i8 = 0 as *mut i8;
    let mut end_pos: *mut i8 = 0 as *mut i8;
    let mut pem_count: size_t = 0;
    let mut stripped_pem_count: size_t = 0 as i32 as size_t;
    let mut pem_len: size_t = 0;
    let mut size: ssize_t = 0;
    let mut base64data: *mut u8 = 0 as *mut u8;
    *der = 0 as *mut u8;
    *der_len = 0 as i32 as size_t;
    if pem.is_null() {
        return 0 as i32 != 0;
    }
    begin_pos = strstr(pem, b"-----BEGIN PUBLIC KEY-----\0" as *const u8 as *const i8);
    if begin_pos.is_null() {
        return 0 as i32 != 0;
    }
    pem_count = begin_pos.offset_from(pem) as i64 as size_t;
    if 0 as i32 as u64 != pem_count
        && '\n' as i32
            != *pem.offset(pem_count.wrapping_sub(1 as i32 as u64) as isize) as i32
    {
        return 0 as i32 != 0;
    }
    pem_count = (pem_count as u64).wrapping_add(26 as i32 as u64) as size_t as size_t;
    end_pos = strstr(
        pem.offset(pem_count as isize),
        b"\n-----END PUBLIC KEY-----\0" as *const u8 as *const i8,
    );
    if end_pos.is_null() {
        return 0 as i32 != 0;
    }
    pem_len = end_pos.offset_from(pem) as i64 as size_t;
    stripped_pem = xmalloc(pem_len.wrapping_sub(pem_count).wrapping_add(1 as i32 as u64))
        as *mut i8;
    while pem_count < pem_len {
        if '\n' as i32 != *pem.offset(pem_count as isize) as i32
            && '\r' as i32 != *pem.offset(pem_count as isize) as i32
        {
            let fresh232 = stripped_pem_count;
            stripped_pem_count = stripped_pem_count.wrapping_add(1);
            *stripped_pem.offset(fresh232 as isize) = *pem.offset(pem_count as isize);
        }
        pem_count = pem_count.wrapping_add(1);
        pem_count;
    }
    *stripped_pem.offset(stripped_pem_count as isize) = '\0' as i32 as i8;
    base64data = xmalloc(
        (4 as i32 as u64)
            .wrapping_mul(
                stripped_pem_count
                    .wrapping_add(2 as i32 as u64)
                    .wrapping_div(3 as i32 as u64),
            ),
    ) as *mut u8;
    size = wget_base64_decode(
        stripped_pem,
        base64data as *mut libc::c_void,
        (4 as i32 as u64)
            .wrapping_mul(
                stripped_pem_count
                    .wrapping_add(2 as i32 as u64)
                    .wrapping_div(3 as i32 as u64),
            ),
    );
    if size < 0 as i32 as i64 {
        rpl_free(base64data as *mut libc::c_void);
        base64data = 0 as *mut u8;
    } else {
        *der = base64data;
        *der_len = size as size_t;
    }
    rpl_free(stripped_pem as *mut libc::c_void);
    stripped_pem = 0 as *mut i8;
    return *der_len > 0 as i32 as u64;
}
#[no_mangle]
pub unsafe extern "C" fn wg_pin_peer_pubkey(
    mut pinnedpubkey: *const i8,
    mut pubkey: *const i8,
    mut pubkeylen: size_t,
) -> bool {
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut buf: *mut u8 = 0 as *mut u8;
    let mut pem_ptr: *mut u8 = 0 as *mut u8;
    let mut size: size_t = 0;
    let mut pem_len: size_t = 0;
    let mut pem_read: bool = false;
    let mut result: bool = 0 as i32 != 0;
    let mut pinkeylen: size_t = 0;
    let mut decoded_hash_length: ssize_t = 0;
    let mut pinkeycopy: *mut i8 = 0 as *mut i8;
    let mut begin_pos: *mut i8 = 0 as *mut i8;
    let mut end_pos: *mut i8 = 0 as *mut i8;
    let mut sha256sumdigest: *mut u8 = 0 as *mut u8;
    let mut expectedsha256sumdigest: *mut u8 = 0 as *mut u8;
    if pinnedpubkey.is_null() {
        return 1 as i32 != 0;
    }
    if pubkey.is_null() || pubkeylen == 0 {
        return result;
    }
    if strncmp(pinnedpubkey, b"sha256//\0" as *const u8 as *const i8, 8 as i32 as u64)
        == 0 as i32
    {
        sha256sumdigest = xmalloc(C2RustUnnamed_4::SHA256_DIGEST_SIZE as i32 as size_t)
            as *mut u8;
        sha256_buffer(pubkey, pubkeylen, sha256sumdigest as *mut libc::c_void);
        expectedsha256sumdigest = xmalloc(
            C2RustUnnamed_4::SHA256_DIGEST_SIZE as i32 as size_t,
        ) as *mut u8;
        pinkeylen = (strlen(pinnedpubkey)).wrapping_add(1 as i32 as u64);
        pinkeycopy = xmalloc(pinkeylen) as *mut i8;
        memcpy(
            pinkeycopy as *mut libc::c_void,
            pinnedpubkey as *const libc::c_void,
            pinkeylen,
        );
        begin_pos = pinkeycopy;
        loop {
            end_pos = strstr(begin_pos, b";sha256//\0" as *const u8 as *const i8);
            if !end_pos.is_null() {
                *end_pos.offset(0 as i32 as isize) = '\0' as i32 as i8;
            }
            decoded_hash_length = wget_base64_decode(
                begin_pos.offset(8 as i32 as isize),
                expectedsha256sumdigest as *mut libc::c_void,
                C2RustUnnamed_4::SHA256_DIGEST_SIZE as i32 as size_t,
            );
            if C2RustUnnamed_4::SHA256_DIGEST_SIZE as i32 as i64 == decoded_hash_length {
                if memcmp(
                    sha256sumdigest as *const libc::c_void,
                    expectedsha256sumdigest as *const libc::c_void,
                    C2RustUnnamed_4::SHA256_DIGEST_SIZE as i32 as u64,
                ) == 0
                {
                    result = 1 as i32 != 0;
                    break;
                }
            } else {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Skipping key with wrong size (%d/%d): %s\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (strlen(begin_pos.offset(8 as i32 as isize)))
                        .wrapping_mul(3 as i32 as u64) as i32 / 4 as i32,
                    C2RustUnnamed_4::SHA256_DIGEST_SIZE as i32,
                    quote(begin_pos.offset(8 as i32 as isize)),
                );
            }
            if !end_pos.is_null() {
                *end_pos.offset(0 as i32 as isize) = ';' as i32 as i8;
                begin_pos = strstr(end_pos, b"sha256//\0" as *const u8 as *const i8);
            }
            if !(!end_pos.is_null() && !begin_pos.is_null()) {
                break;
            }
        }
        rpl_free(sha256sumdigest as *mut libc::c_void);
        sha256sumdigest = 0 as *mut u8;
        rpl_free(expectedsha256sumdigest as *mut libc::c_void);
        expectedsha256sumdigest = 0 as *mut u8;
        rpl_free(pinkeycopy as *mut libc::c_void);
        pinkeycopy = 0 as *mut i8;
        return result;
    }
    fm = wget_read_file(pinnedpubkey);
    if fm.is_null() {
        return result;
    }
    if !((*fm).length < 0 as i32 as i64 || (*fm).length > 1048576 as i32 as i64) {
        size = (*fm).length as size_t;
        if !(pubkeylen > size) {
            if pubkeylen == size {
                if memcmp(
                    pubkey as *const libc::c_void,
                    (*fm).content as *const libc::c_void,
                    pubkeylen,
                ) == 0
                {
                    result = 1 as i32 != 0;
                }
            } else {
                buf = xmalloc(size.wrapping_add(1 as i32 as u64)) as *mut u8;
                memcpy(
                    buf as *mut libc::c_void,
                    (*fm).content as *const libc::c_void,
                    size,
                );
                *buf.offset(size as isize) = '\0' as i32 as u8;
                pem_read = wg_pubkey_pem_to_der(
                    buf as *const i8,
                    &mut pem_ptr,
                    &mut pem_len,
                );
                if pem_read {
                    if pubkeylen == pem_len
                        && memcmp(
                            pubkey as *const libc::c_void,
                            pem_ptr as *const libc::c_void,
                            pubkeylen,
                        ) == 0
                    {
                        result = 1 as i32 != 0;
                    }
                }
            }
        }
    }
    rpl_free(buf as *mut libc::c_void);
    buf = 0 as *mut u8;
    rpl_free(pem_ptr as *mut libc::c_void);
    pem_ptr = 0 as *mut u8;
    wget_read_file_free(fm);
    return result;
}