#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type __spawn_action;
    pub type hsts_store;
    pub type ptimer;
    fn time(__timer: *mut time_t) -> time_t;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn set_uri_encoding(i: *mut iri, charset: *const i8, force: bool);
    fn iri_free(i: *mut iri);
    fn iri_new() -> *mut iri;
    fn check_encoding_name(encoding: *const i8) -> bool;
    fn find_locale() -> *const i8;
    fn xstrdup(str: *const i8) -> *mut i8;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn perror(__s: *const i8);
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn log_init(_: *const i8, _: bool);
    fn redirect_output(_: bool, _: *const i8);
    fn quote(arg: *const i8) -> *const i8;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut i32) -> i32;
    static mut environ: *mut *mut i8;
    fn unlink(__name: *const i8) -> i32;
    static mut optarg: *mut i8;
    fn getpass(__prompt: *const i8) -> *mut i8;
    static mut optind: i32;
    static mut opterr: i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn rpl_posix_spawnp(
        __pid: *mut pid_t,
        __file: *const i8,
        __file_actions: *const rpl_posix_spawn_file_actions_t,
        __attrp: *const rpl_posix_spawnattr_t,
        argv: *const *mut i8,
        envp: *const *mut i8,
    ) -> i32;
    fn rpl_posix_spawn_file_actions_init(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
    ) -> i32;
    fn rpl_posix_spawn_file_actions_adddup2(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
        __fd: i32,
        __newfd: i32,
    ) -> i32;
    fn __errno_location() -> *mut i32;
    fn inform_exit_status(err: uerr_t);
    fn get_exit_status() -> i32;
    fn datetime_str(_: time_t) -> *mut i8;
    fn fork_to_background() -> bool;
    fn file_exists_p(_: *const i8, _: *mut file_stats_t) -> bool;
    fn human_readable(_: wgint, _: i32, _: i32) -> *mut i8;
    fn compile_pcre_regex(_: *const i8) -> *mut libc::c_void;
    fn match_pcre_regex(_: *const libc::c_void, _: *const i8) -> bool;
    fn compile_posix_regex(_: *const i8) -> *mut libc::c_void;
    fn match_posix_regex(_: *const libc::c_void, _: *const i8) -> bool;
    fn print_decimal(_: libc::c_double) -> *const i8;
    fn ajoin_dir_file(_: *const i8, _: *const i8) -> *mut i8;
    fn wgetrc_env_file_name() -> *mut i8;
    fn wgetrc_user_file_name() -> *mut i8;
    fn initialize() -> i32;
    fn run_command(_: *const i8);
    fn setoptval(_: *const i8, _: *const i8, _: *const i8);
    fn home_dir() -> *mut i8;
    fn cleanup();
    fn defaults();
    fn run_wgetrc(file: *const i8, _: *mut file_stats_t) -> bool;
    fn url_parse(
        _: *const i8,
        _: *mut i32,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: i32) -> *const i8;
    fn url_free(_: *mut url);
    fn url_scheme(_: *const i8) -> url_scheme;
    static mut output_stream: *mut FILE;
    static mut output_stream_regular: bool;
    static mut total_download_time: libc::c_double;
    static mut total_downloaded_bytes: wgint;
    fn scheme_leading_string(_: url_scheme) -> *const i8;
    fn rewrite_shorthand_url(_: *const i8) -> *mut i8;
    fn retrieve_url(
        _: *mut url,
        _: *const i8,
        _: *mut *mut i8,
        _: *mut *mut i8,
        _: *const i8,
        _: *mut i32,
        _: bool,
        _: *mut iri,
        _: bool,
    ) -> uerr_t;
    fn retrieve_from_file(_: *const i8, _: bool, _: *mut i32) -> uerr_t;
    fn retr_rate(_: wgint, _: libc::c_double) -> *const i8;
    fn url_uses_proxy(_: *mut url) -> bool;
    fn retrieve_tree(_: *mut url, _: *mut iri) -> uerr_t;
    fn set_progress_implementation(_: *const i8);
    fn progress_schedule_redirect();
    fn progress_handle_sigwinch(_: i32);
    fn convert_all_links();
    fn print_broken_links();
    fn save_cookies();
    fn hsts_store_open(_: *const i8) -> hsts_store_t;
    fn hsts_store_save(_: hsts_store_t, _: *const i8);
    fn hsts_store_close(_: hsts_store_t);
    fn hsts_store_has_changed(_: hsts_store_t) -> bool;
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn warc_init();
    static mut version_string: *const i8;
    static mut compilation_string: *const i8;
    static mut link_string: *const i8;
    static mut compiled_features: [*const i8; 0];
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn base_name(file: *const i8) -> *mut i8;
    fn xmemdup0(p: *const libc::c_void, s: size_t) -> *mut i8;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
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
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iri {
    pub uri_encoding: *mut i8,
    pub content_encoding: *mut i8,
    pub orig_url: *mut i8,
    pub utf8_encode: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum uerr_t {
    NOCONERROR,
    HOSTERR,
    CONSOCKERR,
    CONERROR,
    CONSSLERR,
    CONIMPOSSIBLE,
    NEWLOCATION,
    FTPOK,
    FTPLOGINC,
    FTPLOGREFUSED,
    FTPPORTERR,
    FTPSYSERR,
    FTPNSFOD,
    FTPUNKNOWNTYPE,
    FTPRERR,
    FTPSRVERR,
    FTPRETRINT,
    FTPRESTFAIL,
    URLERROR,
    FOPENERR,
    FOPEN_EXCL_ERR,
    FWRITEERR,
    HEOF,
    GATEWAYTIMEOUT,
    HERR,
    RETROK,
    RECLEVELEXC,
    WRONGCODE,
    FTPINVPASV,
    FTPNOPASV,
    FTPNOPBSZ,
    FTPNOPROT,
    FTPNOAUTH,
    CONTNOTSUPPORTED,
    RETRUNNEEDED,
    RETRFINISHED,
    READERR,
    TRYLIMEXC,
    FILEBADFILE,
    RANGEERR,
    RETRBADPATTERN,
    PROXERR,
    AUTHFAILED,
    QUOTEXC,
    WRITEFAILED,
    SSLINITFAILED,
    VERIFCERTERR,
    UNLINKERR,
    NEWLOCATION_KEEP_POST,
    CLOSEFAILED,
    ATTRMISSING,
    UNKNOWNATTR,
    WARC_ERR,
    WARC_TMP_FOPENERR,
    WARC_TMP_FWRITEERR,
    TIMECONV_ERR,
    METALINK_PARSE_ERROR,
    METALINK_RETR_ERROR,
    METALINK_CHKSUM_ERROR,
    METALINK_SIG_ERROR,
    METALINK_MISSING_RESOURCE,
    RETR_WITH_METALINK,
    METALINK_SIZE_ERROR,
}
impl uerr_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            uerr_t::NOCONERROR => 0,
            uerr_t::HOSTERR => 1,
            uerr_t::CONSOCKERR => 2,
            uerr_t::CONERROR => 3,
            uerr_t::CONSSLERR => 4,
            uerr_t::CONIMPOSSIBLE => 5,
            uerr_t::NEWLOCATION => 6,
            uerr_t::FTPOK => 7,
            uerr_t::FTPLOGINC => 8,
            uerr_t::FTPLOGREFUSED => 9,
            uerr_t::FTPPORTERR => 10,
            uerr_t::FTPSYSERR => 11,
            uerr_t::FTPNSFOD => 12,
            uerr_t::FTPUNKNOWNTYPE => 13,
            uerr_t::FTPRERR => 14,
            uerr_t::FTPSRVERR => 15,
            uerr_t::FTPRETRINT => 16,
            uerr_t::FTPRESTFAIL => 17,
            uerr_t::URLERROR => 18,
            uerr_t::FOPENERR => 19,
            uerr_t::FOPEN_EXCL_ERR => 20,
            uerr_t::FWRITEERR => 21,
            uerr_t::HEOF => 22,
            uerr_t::GATEWAYTIMEOUT => 23,
            uerr_t::HERR => 24,
            uerr_t::RETROK => 25,
            uerr_t::RECLEVELEXC => 26,
            uerr_t::WRONGCODE => 27,
            uerr_t::FTPINVPASV => 28,
            uerr_t::FTPNOPASV => 29,
            uerr_t::FTPNOPBSZ => 30,
            uerr_t::FTPNOPROT => 31,
            uerr_t::FTPNOAUTH => 32,
            uerr_t::CONTNOTSUPPORTED => 33,
            uerr_t::RETRUNNEEDED => 34,
            uerr_t::RETRFINISHED => 35,
            uerr_t::READERR => 36,
            uerr_t::TRYLIMEXC => 37,
            uerr_t::FILEBADFILE => 38,
            uerr_t::RANGEERR => 39,
            uerr_t::RETRBADPATTERN => 40,
            uerr_t::PROXERR => 41,
            uerr_t::AUTHFAILED => 42,
            uerr_t::QUOTEXC => 43,
            uerr_t::WRITEFAILED => 44,
            uerr_t::SSLINITFAILED => 45,
            uerr_t::VERIFCERTERR => 46,
            uerr_t::UNLINKERR => 47,
            uerr_t::NEWLOCATION_KEEP_POST => 48,
            uerr_t::CLOSEFAILED => 49,
            uerr_t::ATTRMISSING => 50,
            uerr_t::UNKNOWNATTR => 51,
            uerr_t::WARC_ERR => 52,
            uerr_t::WARC_TMP_FOPENERR => 53,
            uerr_t::WARC_TMP_FWRITEERR => 54,
            uerr_t::TIMECONV_ERR => 55,
            uerr_t::METALINK_PARSE_ERROR => 56,
            uerr_t::METALINK_RETR_ERROR => 57,
            uerr_t::METALINK_CHKSUM_ERROR => 58,
            uerr_t::METALINK_SIG_ERROR => 59,
            uerr_t::METALINK_MISSING_RESOURCE => 60,
            uerr_t::RETR_WITH_METALINK => 61,
            uerr_t::METALINK_SIZE_ERROR => 62,
        }
    }
    fn from_libc_c_uint(value: u32) -> uerr_t {
        match value {
            0 => uerr_t::NOCONERROR,
            1 => uerr_t::HOSTERR,
            2 => uerr_t::CONSOCKERR,
            3 => uerr_t::CONERROR,
            4 => uerr_t::CONSSLERR,
            5 => uerr_t::CONIMPOSSIBLE,
            6 => uerr_t::NEWLOCATION,
            7 => uerr_t::FTPOK,
            8 => uerr_t::FTPLOGINC,
            9 => uerr_t::FTPLOGREFUSED,
            10 => uerr_t::FTPPORTERR,
            11 => uerr_t::FTPSYSERR,
            12 => uerr_t::FTPNSFOD,
            13 => uerr_t::FTPUNKNOWNTYPE,
            14 => uerr_t::FTPRERR,
            15 => uerr_t::FTPSRVERR,
            16 => uerr_t::FTPRETRINT,
            17 => uerr_t::FTPRESTFAIL,
            18 => uerr_t::URLERROR,
            19 => uerr_t::FOPENERR,
            20 => uerr_t::FOPEN_EXCL_ERR,
            21 => uerr_t::FWRITEERR,
            22 => uerr_t::HEOF,
            23 => uerr_t::GATEWAYTIMEOUT,
            24 => uerr_t::HERR,
            25 => uerr_t::RETROK,
            26 => uerr_t::RECLEVELEXC,
            27 => uerr_t::WRONGCODE,
            28 => uerr_t::FTPINVPASV,
            29 => uerr_t::FTPNOPASV,
            30 => uerr_t::FTPNOPBSZ,
            31 => uerr_t::FTPNOPROT,
            32 => uerr_t::FTPNOAUTH,
            33 => uerr_t::CONTNOTSUPPORTED,
            34 => uerr_t::RETRUNNEEDED,
            35 => uerr_t::RETRFINISHED,
            36 => uerr_t::READERR,
            37 => uerr_t::TRYLIMEXC,
            38 => uerr_t::FILEBADFILE,
            39 => uerr_t::RANGEERR,
            40 => uerr_t::RETRBADPATTERN,
            41 => uerr_t::PROXERR,
            42 => uerr_t::AUTHFAILED,
            43 => uerr_t::QUOTEXC,
            44 => uerr_t::WRITEFAILED,
            45 => uerr_t::SSLINITFAILED,
            46 => uerr_t::VERIFCERTERR,
            47 => uerr_t::UNLINKERR,
            48 => uerr_t::NEWLOCATION_KEEP_POST,
            49 => uerr_t::CLOSEFAILED,
            50 => uerr_t::ATTRMISSING,
            51 => uerr_t::UNKNOWNATTR,
            52 => uerr_t::WARC_ERR,
            53 => uerr_t::WARC_TMP_FOPENERR,
            54 => uerr_t::WARC_TMP_FWRITEERR,
            55 => uerr_t::TIMECONV_ERR,
            56 => uerr_t::METALINK_PARSE_ERROR,
            57 => uerr_t::METALINK_RETR_ERROR,
            58 => uerr_t::METALINK_CHKSUM_ERROR,
            59 => uerr_t::METALINK_SIG_ERROR,
            60 => uerr_t::METALINK_MISSING_RESOURCE,
            61 => uerr_t::RETR_WITH_METALINK,
            62 => uerr_t::METALINK_SIZE_ERROR,
            _ => panic!("Invalid value for uerr_t: {}", value),
        }
    }
}
impl AddAssign<u32> for uerr_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = uerr_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for uerr_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = uerr_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for uerr_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = uerr_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for uerr_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = uerr_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for uerr_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = uerr_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for uerr_t {
    type Output = uerr_t;
    fn add(self, rhs: u32) -> uerr_t {
        uerr_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for uerr_t {
    type Output = uerr_t;
    fn sub(self, rhs: u32) -> uerr_t {
        uerr_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for uerr_t {
    type Output = uerr_t;
    fn mul(self, rhs: u32) -> uerr_t {
        uerr_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for uerr_t {
    type Output = uerr_t;
    fn div(self, rhs: u32) -> uerr_t {
        uerr_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for uerr_t {
    type Output = uerr_t;
    fn rem(self, rhs: u32) -> uerr_t {
        uerr_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawnattr_t {
    pub _flags: libc::c_short,
    pub _pgrp: pid_t,
    pub _sd: sigset_t,
    pub _ss: sigset_t,
    pub _sp: sched_param,
    pub _policy: i32,
    pub __pad: [i32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawn_file_actions_t {
    pub _allocated: i32,
    pub _used: i32,
    pub _actions: *mut __spawn_action,
    pub __pad: [i32; 16],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
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
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::WGET_EXIT_SUCCESS => 0,
            C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR => 1,
            C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR => 2,
            C2RustUnnamed_4::WGET_EXIT_IO_FAIL => 3,
            C2RustUnnamed_4::WGET_EXIT_NETWORK_FAIL => 4,
            C2RustUnnamed_4::WGET_EXIT_SSL_AUTH_FAIL => 5,
            C2RustUnnamed_4::WGET_EXIT_SERVER_AUTH_FAIL => 6,
            C2RustUnnamed_4::WGET_EXIT_PROTOCOL_ERROR => 7,
            C2RustUnnamed_4::WGET_EXIT_SERVER_ERROR => 8,
            C2RustUnnamed_4::WGET_EXIT_UNKNOWN => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            0 => C2RustUnnamed_4::WGET_EXIT_SUCCESS,
            1 => C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR,
            2 => C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR,
            3 => C2RustUnnamed_4::WGET_EXIT_IO_FAIL,
            4 => C2RustUnnamed_4::WGET_EXIT_NETWORK_FAIL,
            5 => C2RustUnnamed_4::WGET_EXIT_SSL_AUTH_FAIL,
            6 => C2RustUnnamed_4::WGET_EXIT_SERVER_AUTH_FAIL,
            7 => C2RustUnnamed_4::WGET_EXIT_PROTOCOL_ERROR,
            8 => C2RustUnnamed_4::WGET_EXIT_SERVER_ERROR,
            9 => C2RustUnnamed_4::WGET_EXIT_UNKNOWN,
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
pub struct file_stat_s {
    pub access_err: i32,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type file_stats_t = file_stat_s;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum url_scheme {
    SCHEME_HTTP,
    SCHEME_HTTPS,
    SCHEME_FTP,
    SCHEME_FTPS,
    SCHEME_INVALID,
}
impl url_scheme {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            url_scheme::SCHEME_HTTP => 0,
            url_scheme::SCHEME_HTTPS => 1,
            url_scheme::SCHEME_FTP => 2,
            url_scheme::SCHEME_FTPS => 3,
            url_scheme::SCHEME_INVALID => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> url_scheme {
        match value {
            0 => url_scheme::SCHEME_HTTP,
            1 => url_scheme::SCHEME_HTTPS,
            2 => url_scheme::SCHEME_FTP,
            3 => url_scheme::SCHEME_FTPS,
            4 => url_scheme::SCHEME_INVALID,
            _ => panic!("Invalid value for url_scheme: {}", value),
        }
    }
}
impl AddAssign<u32> for url_scheme {
    fn add_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for url_scheme {
    fn sub_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for url_scheme {
    fn mul_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for url_scheme {
    fn div_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for url_scheme {
    fn rem_assign(&mut self, rhs: u32) {
        *self = url_scheme::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for url_scheme {
    type Output = url_scheme;
    fn add(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for url_scheme {
    type Output = url_scheme;
    fn sub(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for url_scheme {
    type Output = url_scheme;
    fn mul(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for url_scheme {
    type Output = url_scheme;
    fn div(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for url_scheme {
    type Output = url_scheme;
    fn rem(self, rhs: u32) -> url_scheme {
        url_scheme::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url {
    pub url: *mut i8,
    pub scheme: url_scheme,
    pub host: *mut i8,
    pub port: i32,
    pub path: *mut i8,
    pub params: *mut i8,
    pub query: *mut i8,
    pub fragment: *mut i8,
    pub dir: *mut i8,
    pub file: *mut i8,
    pub user: *mut i8,
    pub passwd: *mut i8,
}
pub type hsts_store_t = *mut hsts_store;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdline_option {
    pub long_name: [i8; 26],
    pub short_name: i8,
    pub type_0: C2RustUnnamed_5,
    pub data: *const libc::c_void,
    pub argtype: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    OPT__PARENT = 8,
    OPT__NO = 7,
    OPT__EXECUTE = 6,
    OPT__DONT_REMOVE_LISTING = 5,
    OPT__CLOBBER = 4,
    OPT__APPEND_OUTPUT = 3,
    OPT_FUNCALL = 2,
    OPT_BOOLEAN = 1,
    OPT_VALUE = 0,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_5::OPT__PARENT => 8,
            C2RustUnnamed_5::OPT__NO => 7,
            C2RustUnnamed_5::OPT__EXECUTE => 6,
            C2RustUnnamed_5::OPT__DONT_REMOVE_LISTING => 5,
            C2RustUnnamed_5::OPT__CLOBBER => 4,
            C2RustUnnamed_5::OPT__APPEND_OUTPUT => 3,
            C2RustUnnamed_5::OPT_FUNCALL => 2,
            C2RustUnnamed_5::OPT_BOOLEAN => 1,
            C2RustUnnamed_5::OPT_VALUE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_5 {
        match value {
            8 => C2RustUnnamed_5::OPT__PARENT,
            7 => C2RustUnnamed_5::OPT__NO,
            6 => C2RustUnnamed_5::OPT__EXECUTE,
            5 => C2RustUnnamed_5::OPT__DONT_REMOVE_LISTING,
            4 => C2RustUnnamed_5::OPT__CLOBBER,
            3 => C2RustUnnamed_5::OPT__APPEND_OUTPUT,
            2 => C2RustUnnamed_5::OPT_FUNCALL,
            1 => C2RustUnnamed_5::OPT_BOOLEAN,
            0 => C2RustUnnamed_5::OPT_VALUE,
            _ => panic!("Invalid value for C2RustUnnamed_5: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_5 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_5 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_5 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_5 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_5 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn add(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn sub(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn mul(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn div(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn rem(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
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
#[no_mangle]
pub static mut ares: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut opt: options = options {
    verbose: 0,
    quiet: false,
    ntry: 0,
    retry_connrefused: false,
    retry_on_host_error: false,
    retry_on_http_error: 0 as *const i8 as *mut i8,
    background: false,
    ignore_length: false,
    recursive: false,
    spanhost: false,
    max_redirect: 0,
    relative_only: false,
    no_parent: false,
    reclevel: 0,
    dirstruct: false,
    no_dirstruct: false,
    cut_dirs: 0,
    add_hostdir: false,
    protocol_directories: false,
    noclobber: false,
    unlink_requested: false,
    dir_prefix: 0 as *const i8 as *mut i8,
    lfilename: 0 as *const i8 as *mut i8,
    input_filename: 0 as *const i8 as *mut i8,
    choose_config: 0 as *const i8 as *mut i8,
    noconfig: false,
    force_html: false,
    default_page: 0 as *const i8 as *mut i8,
    spider: false,
    accepts: 0 as *const *mut i8 as *mut *mut i8,
    rejects: 0 as *const *mut i8 as *mut *mut i8,
    excludes: 0 as *const *const i8 as *mut *const i8,
    includes: 0 as *const *const i8 as *mut *const i8,
    ignore_case: false,
    acceptregex_s: 0 as *const i8 as *mut i8,
    rejectregex_s: 0 as *const i8 as *mut i8,
    acceptregex: 0 as *const libc::c_void as *mut libc::c_void,
    rejectregex: 0 as *const libc::c_void as *mut libc::c_void,
    regex_type: C2RustUnnamed_3::regex_type_pcre,
    regex_compile_fun: None,
    regex_match_fun: None,
    domains: 0 as *const *mut i8 as *mut *mut i8,
    exclude_domains: 0 as *const *mut i8 as *mut *mut i8,
    dns_cache: false,
    follow_tags: 0 as *const *mut i8 as *mut *mut i8,
    ignore_tags: 0 as *const *mut i8 as *mut *mut i8,
    follow_ftp: false,
    retr_symlinks: false,
    output_document: 0 as *const i8 as *mut i8,
    warc_filename: 0 as *const i8 as *mut i8,
    warc_tempdir: 0 as *const i8 as *mut i8,
    warc_cdx_dedup_filename: 0 as *const i8 as *mut i8,
    warc_maxsize: 0,
    warc_compression_enabled: false,
    warc_digests_enabled: false,
    warc_cdx_enabled: false,
    warc_keep_log: false,
    warc_user_headers: 0 as *const *mut i8 as *mut *mut i8,
    enable_xattr: false,
    user: 0 as *const i8 as *mut i8,
    passwd: 0 as *const i8 as *mut i8,
    ask_passwd: false,
    use_askpass: 0 as *const i8 as *mut i8,
    always_rest: false,
    start_pos: 0,
    ftp_user: 0 as *const i8 as *mut i8,
    ftp_passwd: 0 as *const i8 as *mut i8,
    netrc: false,
    ftp_glob: false,
    ftp_pasv: false,
    http_user: 0 as *const i8 as *mut i8,
    http_passwd: 0 as *const i8 as *mut i8,
    user_headers: 0 as *const *mut i8 as *mut *mut i8,
    http_keep_alive: false,
    use_proxy: false,
    allow_cache: false,
    http_proxy: 0 as *const i8 as *mut i8,
    ftp_proxy: 0 as *const i8 as *mut i8,
    https_proxy: 0 as *const i8 as *mut i8,
    no_proxy: 0 as *const *mut i8 as *mut *mut i8,
    base_href: 0 as *const i8 as *mut i8,
    progress_type: 0 as *const i8 as *mut i8,
    show_progress: 0,
    noscroll: false,
    proxy_user: 0 as *const i8 as *mut i8,
    proxy_passwd: 0 as *const i8 as *mut i8,
    read_timeout: 0.,
    dns_timeout: 0.,
    connect_timeout: 0.,
    random_wait: false,
    wait: 0.,
    waitretry: 0.,
    use_robots: false,
    limit_rate: 0,
    quota: 0,
    server_response: false,
    save_headers: false,
    content_on_error: false,
    debug: false,
    timestamping: false,
    if_modified_since: false,
    backup_converted: false,
    backups: 0,
    useragent: 0 as *const i8 as *mut i8,
    referer: 0 as *const i8 as *mut i8,
    convert_links: false,
    convert_file_only: false,
    remove_listing: false,
    htmlify: false,
    dot_style: 0 as *const i8 as *mut i8,
    dot_bytes: 0,
    dots_in_line: 0,
    dot_spacing: 0,
    delete_after: false,
    adjust_extension: false,
    page_requisites: false,
    bind_address: 0 as *const i8 as *mut i8,
    secure_protocol: C2RustUnnamed_2::secure_protocol_auto,
    secure_protocol_name: [0; 8],
    check_cert: 0,
    cert_file: 0 as *const i8 as *mut i8,
    private_key: 0 as *const i8 as *mut i8,
    cert_type: keyfile_type::keyfile_pem,
    private_key_type: keyfile_type::keyfile_pem,
    ca_directory: 0 as *const i8 as *mut i8,
    ca_cert: 0 as *const i8 as *mut i8,
    crl_file: 0 as *const i8 as *mut i8,
    pinnedpubkey: 0 as *const i8 as *mut i8,
    random_file: 0 as *const i8 as *mut i8,
    egd_file: 0 as *const i8 as *mut i8,
    https_only: false,
    ftps_resume_ssl: false,
    ftps_fallback_to_ftp: false,
    ftps_implicit: false,
    ftps_clear_data_connection: false,
    tls_ciphers_string: 0 as *const i8 as *mut i8,
    cookies: false,
    cookies_input: 0 as *const i8 as *mut i8,
    cookies_output: 0 as *const i8 as *mut i8,
    keep_badhash: false,
    keep_session_cookies: false,
    post_data: 0 as *const i8 as *mut i8,
    post_file_name: 0 as *const i8 as *mut i8,
    method: 0 as *const i8 as *mut i8,
    body_data: 0 as *const i8 as *mut i8,
    body_file: 0 as *const i8 as *mut i8,
    restrict_files_os: C2RustUnnamed_1::restrict_unix,
    restrict_files_ctrl: false,
    restrict_files_nonascii: false,
    restrict_files_case: C2RustUnnamed_0::restrict_no_case_restriction,
    strict_comments: false,
    preserve_perm: false,
    ipv4_only: false,
    ipv6_only: false,
    prefer_family: C2RustUnnamed::prefer_ipv4,
    content_disposition: false,
    auth_without_challenge: false,
    enable_iri: false,
    encoding_remote: 0 as *const i8 as *mut i8,
    locale: 0 as *const i8,
    trustservernames: false,
    useservertimestamps: false,
    show_all_dns_entries: false,
    report_bps: false,
    compression: compression_options::compression_auto,
    rejected_log: 0 as *const i8 as *mut i8,
    hsts: false,
    hsts_file: 0 as *const i8 as *mut i8,
    homedir: 0 as *const i8,
    wgetrcfile: 0 as *const i8,
};
#[no_mangle]
pub static mut exec_name: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut numurls: i32 = 0 as i32;
unsafe extern "C" fn redirect_output_signal(mut sig: i32) {
    let mut signal_name: *const i8 = b"WTF?!\0" as *const u8 as *const i8;
    if sig == 1 as i32 {
        signal_name = b"SIGHUP\0" as *const u8 as *const i8;
    }
    if sig == 10 as i32 {
        signal_name = b"SIGUSR1\0" as *const u8 as *const i8;
    }
    redirect_output(1 as i32 != 0, signal_name);
    progress_schedule_redirect();
    signal(sig, Some(redirect_output_signal as unsafe extern "C" fn(i32) -> ()));
}
unsafe extern "C" fn i18n_initialize() {
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"wget\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    bindtextdomain(
        b"wget-gnulib\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"wget\0" as *const u8 as *const i8);
}
#[no_mangle]
pub static mut hsts_store: hsts_store_t = 0 as *const hsts_store as *mut hsts_store;
unsafe extern "C" fn get_hsts_database() -> *mut i8 {
    if !(opt.hsts_file).is_null() {
        return xstrdup(opt.hsts_file);
    }
    if !(opt.homedir).is_null() {
        let mut dir: *mut i8 = ajoin_dir_file(
            opt.homedir,
            b".wget-hsts\0" as *const u8 as *const i8,
        );
        return dir;
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn load_hsts() {
    if hsts_store.is_null() {
        let mut filename: *mut i8 = get_hsts_database();
        if !filename.is_null() {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Reading HSTS entries from %s\n\0" as *const u8 as *const i8,
                    filename,
                );
            }
            hsts_store = hsts_store_open(filename);
            if hsts_store.is_null() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"ERROR: could not open HSTS store at '%s'. HSTS will be disabled.\n\0"
                        as *const u8 as *const i8,
                    filename,
                );
            }
        } else {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"ERROR: could not open HSTS store. HSTS will be disabled.\n\0"
                    as *const u8 as *const i8,
            );
        }
        rpl_free(filename as *mut libc::c_void);
        filename = 0 as *mut i8;
    }
}
unsafe extern "C" fn save_hsts() {
    if !hsts_store.is_null() {
        let mut filename: *mut i8 = get_hsts_database();
        if !filename.is_null() && hsts_store_has_changed(hsts_store) as i32 != 0 {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Saving HSTS entries to %s\n\0" as *const u8 as *const i8,
                    filename,
                );
            }
            hsts_store_save(hsts_store, filename);
        }
        hsts_store_close(hsts_store);
        rpl_free(hsts_store as *mut libc::c_void);
        hsts_store = 0 as hsts_store_t;
        rpl_free(filename as *mut libc::c_void);
        filename = 0 as *mut i8;
    }
}
static mut option_data: [cmdline_option; 164] = unsafe {
    [
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"accept\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'A' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"accept\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"accept-regex\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"acceptregex\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"adjust-extension\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'E' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"adjustextension\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"append-output\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'a' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT__APPEND_OUTPUT,
                data: 0 as *const libc::c_void,
                argtype: 1 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ask-password\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"askpassword\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"auth-no-challenge\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"authnochallenge\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"background\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'b' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"background\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"backup-converted\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'K' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"backupconverted\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"backups\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"backups\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"base\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'B' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"base\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"bind-address\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"bindaddress\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"body-data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"bodydata\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"body-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"bodyfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ca-certificate\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"cacertificate\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ca-directory\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"cadirectory\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"cache\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"cache\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"certificate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"certificate\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"certificate-type\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"certificatetype\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"check-certificate\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"checkcertificate\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"clobber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT__CLOBBER,
                data: 0 as *const libc::c_void,
                argtype: 2 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"compression\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"compression\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"chooseconfig\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"connect-timeout\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"connecttimeout\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"continue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'c' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"continue\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"convert-file-only\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"convertfileonly\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"convert-links\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'k' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"convertlinks\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"content-disposition\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"contentdisposition\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"content-on-error\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"contentonerror\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"cookies\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"crl-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"crlfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"cut-dirs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"cutdirs\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"debug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'd' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"debug\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"default-page\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"defaultpage\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"delete-after\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"deleteafter\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"directories\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"dirstruct\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"directory-prefix\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'P' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"dirprefix\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"dns-cache\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"dnscache\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"dns-timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"dnstimeout\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"domains\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'D' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"domains\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"dont-remove-listing\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT__DONT_REMOVE_LISTING,
                data: 0 as *const libc::c_void,
                argtype: 0 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"dot-style\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"dotstyle\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"egd-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"egdfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"exclude-directories\0\0\0\0\0\0\0"),
                short_name: 'X' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"excludedirectories\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"exclude-domains\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"excludedomains\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"execute\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'e' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT__EXECUTE,
                data: 0 as *const libc::c_void,
                argtype: 1 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"follow-ftp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"followftp\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"follow-tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"followtags\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"force-directories\0\0\0\0\0\0\0\0\0"),
                short_name: 'x' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"dirstruct\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"force-html\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'F' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"forcehtml\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ftp-password\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"ftppassword\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ftp-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"ftpuser\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ftps-clear-data-connection"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"ftpscleardataconnection\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ftps-fallback-to-ftp\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"ftpsfallbacktoftp\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ftps-implicit\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"ftpsimplicit\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ftps-resume-ssl\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"ftpsresumessl\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"glob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"glob\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"header\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"header\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"help\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'h' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_FUNCALL,
                data: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> ()>,
                    *mut libc::c_void,
                >(Some(print_help as unsafe extern "C" fn() -> ())),
                argtype: 0 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"host-directories\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"addhostdir\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"hsts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"hsts\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"hsts-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"hstsfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"html-extension\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'E' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"adjustextension\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"htmlify\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"htmlify\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"http-keep-alive\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"httpkeepalive\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"http-passwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"httppassword\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"http-password\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"httppassword\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"http-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"httpuser\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"https-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"httpsonly\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ignore-case\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"ignorecase\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ignore-length\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"ignorelength\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ignore-tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"ignoretags\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"include-directories\0\0\0\0\0\0\0"),
                short_name: 'I' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"includedirectories\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"inet4-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: '4' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"inet4only\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"inet6-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: '6' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"inet6only\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"input-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'i' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"input\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"iri\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"iri\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"keep-badhash\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"keepbadhash\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"keep-session-cookies\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"keepsessioncookies\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"level\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'l' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"reclevel\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"limit-rate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"limitrate\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"load-cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"loadcookies\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"local-encoding\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"localencoding\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"rejected-log\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"rejectedlog\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"max-redirect\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"maxredirect\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"method\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"method\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"mirror\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'm' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"mirror\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"netrc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"netrc\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"no\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'n' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT__NO,
                data: 0 as *const libc::c_void,
                argtype: 1 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"no-clobber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"noclobber\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"no-config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"noconfig\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"no-parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"noparent\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"output-document\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'O' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"outputdocument\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"output-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'o' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"logfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"page-requisites\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'p' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"pagerequisites\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT__PARENT,
                data: 0 as *const libc::c_void,
                argtype: 2 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"passive-ftp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"passiveftp\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"password\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"password\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"pinnedpubkey\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"pinnedpubkey\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"post-data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"postdata\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"post-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"postfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"prefer-family\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"preferfamily\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"preserve-permissions\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"preservepermissions\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"ciphers\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"ciphers\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"private-key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"privatekey\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"private-key-type\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"privatekeytype\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"progress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"progress\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"show-progress\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"showprogress\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"protocol-directories\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"protocoldirectories\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"useproxy\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"proxy__compat\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'Y' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"useproxy\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"proxy-passwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"proxypassword\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"proxy-password\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"proxypassword\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"proxy-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"proxyuser\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"quiet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'q' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"quiet\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"quota\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'Q' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"quota\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"random-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"randomfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"random-wait\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"randomwait\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"read-timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"readtimeout\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"recursive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'r' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"recursive\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"referer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"referer\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"regex-type\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"regextype\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"reject\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'R' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"reject\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"reject-regex\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"rejectregex\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"relative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'L' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"relativeonly\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"remote-encoding\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"remoteencoding\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"remove-listing\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"removelisting\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"report-speed\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"reportspeed\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"restrict-file-names\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"restrictfilenames\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"retr-symlinks\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"retrsymlinks\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"retry-connrefused\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"retryconnrefused\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"retry-on-host-error\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"retryonhosterror\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"retry-on-http-error\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"retryonhttperror\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"save-cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"savecookies\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"save-headers\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"saveheaders\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"secure-protocol\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"secureprotocol\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"server-response\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'S' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"serverresponse\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"span-hosts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'H' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"spanhosts\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"spider\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"spider\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"start-pos\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"startpos\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"strict-comments\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"strictcomments\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'T' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"timeout\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"timestamping\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'N' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"timestamping\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"if-modified-since\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"ifmodifiedsince\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"tries\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 't' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"tries\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"unlink\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"unlink\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"trust-server-names\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"trustservernames\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"use-askpass\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"useaskpass\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"use-server-timestamps\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"useservertimestamps\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"user\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"user-agent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'U' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"useragent\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"verbose\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'v' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"verbose\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'V' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_FUNCALL,
                data: ::core::mem::transmute::<
                    Option<unsafe extern "C" fn() -> ()>,
                    *mut libc::c_void,
                >(Some(print_version as unsafe extern "C" fn() -> ())),
                argtype: 0 as i32,
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"wait\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 'w' as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"wait\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"waitretry\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"waitretry\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-cdx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"warccdx\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-compression\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"warccompression\0" as *const u8 as *const i8
                    as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-dedup\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"warccdxdedup\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-digests\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"warcdigests\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"warcfile\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-header\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"warcheader\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-keep-log\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"warckeeplog\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-max-size\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"warcmaxsize\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"warc-tempdir\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_VALUE,
                data: b"warctempdir\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
        {
            let mut init = cmdline_option {
                long_name: *::core::mem::transmute::<
                    &[u8; 26],
                    &mut [i8; 26],
                >(b"xattr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                short_name: 0 as i32 as i8,
                type_0: C2RustUnnamed_5::OPT_BOOLEAN,
                data: b"xattr\0" as *const u8 as *const i8 as *const libc::c_void,
                argtype: -(1 as i32),
            };
            init
        },
    ]
};
unsafe extern "C" fn no_prefix(mut s: *const i8) -> *mut i8 {
    static mut buffer: [i8; 2048] = [0; 2048];
    static mut p: *mut i8 = unsafe { buffer.as_ptr() as *mut _ };
    let mut cp: *mut i8 = p;
    let mut size: i32 = (3 as i32 as u64)
        .wrapping_add(strlen(s))
        .wrapping_add(1 as i32 as u64) as i32;
    *cp.offset(0 as i32 as isize) = 'n' as i32 as i8;
    *cp.offset(1 as i32 as isize) = 'o' as i32 as i8;
    *cp.offset(2 as i32 as isize) = '-' as i32 as i8;
    strcpy(cp.offset(3 as i32 as isize), s);
    p = p.offset(size as isize);
    return cp;
}
static mut long_options: [option; 329] = [option {
    name: 0 as *const i8,
    has_arg: 0,
    flag: 0 as *const i32 as *mut i32,
    val: 0,
}; 329];
static mut short_options: [i8; 128] = [0; 128];
static mut optmap: [u8; 96] = [0; 96];
unsafe extern "C" fn init_switches() {
    static mut initialized: bool = false;
    let mut p: *mut i8 = short_options.as_mut_ptr();
    let mut i: size_t = 0;
    let mut o: size_t = 0 as i32 as size_t;
    if initialized {
        return;
    }
    initialized = 1 as i32 != 0;
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[cmdline_option; 164]>() as u64)
            .wrapping_div(::core::mem::size_of::<cmdline_option>() as u64)
    {
        let mut cmdopt: *mut cmdline_option = &mut *option_data
            .as_mut_ptr()
            .offset(i as isize) as *mut cmdline_option;
        let mut longopt: *mut option = 0 as *mut option;
        let fresh0 = o;
        o = o.wrapping_add(1);
        longopt = &mut *long_options.as_mut_ptr().offset(fresh0 as isize) as *mut option;
        (*longopt).name = ((*cmdopt).long_name).as_mut_ptr();
        (*longopt).val = i as i32;
        if (*cmdopt).short_name != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = (*cmdopt).short_name;
            optmap[((*cmdopt).short_name as i32 - 32 as i32) as usize] = longopt
                .offset_from(long_options.as_mut_ptr()) as i64 as u8;
        }
        match (*cmdopt).type_0 as u32 {
            0 => {
                (*longopt).has_arg = 1 as i32;
                if (*cmdopt).short_name != 0 {
                    let fresh2 = p;
                    p = p.offset(1);
                    *fresh2 = ':' as i32 as i8;
                }
            }
            1 => {
                (*longopt).has_arg = 2 as i32;
                let fresh3 = o;
                o = o.wrapping_add(1);
                longopt = &mut *long_options.as_mut_ptr().offset(fresh3 as isize)
                    as *mut option;
                (*longopt).name = no_prefix(((*cmdopt).long_name).as_mut_ptr());
                (*longopt).has_arg = 0 as i32;
                (*longopt).val = (i | 1024 as i32 as u64) as i32;
            }
            _ => {
                (*longopt).has_arg = (*cmdopt).argtype;
                if (*cmdopt).short_name != 0 {
                    if (*longopt).has_arg == 1 as i32 {
                        let fresh4 = p;
                        p = p.offset(1);
                        *fresh4 = ':' as i32 as i8;
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    *p = '\0' as i32 as i8;
}
unsafe extern "C" fn print_usage(mut error: i32) -> i32 {
    return fprintf(
        if error != 0 { stderr } else { stdout },
        dcgettext(
            0 as *const i8,
            b"Usage: %s [OPTION]... [URL]...\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        exec_name,
    );
}
unsafe extern "C" fn print_help() {
    static mut help: [*const i8; 178] = [
        b"\n\0" as *const u8 as *const i8,
        b"Mandatory arguments to long options are mandatory for short options too.\n\n\0"
            as *const u8 as *const i8,
        b"Startup:\n\0" as *const u8 as *const i8,
        b"  -V,  --version                   display the version of Wget and exit\n\0"
            as *const u8 as *const i8,
        b"  -h,  --help                      print this help\n\0" as *const u8
            as *const i8,
        b"  -b,  --background                go to background after startup\n\0"
            as *const u8 as *const i8,
        b"  -e,  --execute=COMMAND           execute a `.wgetrc'-style command\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"Logging and input file:\n\0" as *const u8 as *const i8,
        b"  -o,  --output-file=FILE          log messages to FILE\n\0" as *const u8
            as *const i8,
        b"  -a,  --append-output=FILE        append messages to FILE\n\0" as *const u8
            as *const i8,
        b"  -d,  --debug                     print lots of debugging information\n\0"
            as *const u8 as *const i8,
        b"  -q,  --quiet                     quiet (no output)\n\0" as *const u8
            as *const i8,
        b"  -v,  --verbose                   be verbose (this is the default)\n\0"
            as *const u8 as *const i8,
        b"  -nv, --no-verbose                turn off verboseness, without being quiet\n\0"
            as *const u8 as *const i8,
        b"       --report-speed=TYPE         output bandwidth as TYPE.  TYPE can be bits\n\0"
            as *const u8 as *const i8,
        b"  -i,  --input-file=FILE           download URLs found in local or external FILE\n\0"
            as *const u8 as *const i8,
        b"  -F,  --force-html                treat input file as HTML\n\0" as *const u8
            as *const i8,
        b"  -B,  --base=URL                  resolves HTML input-file links (-i -F)\n                                     relative to URL\n\0"
            as *const u8 as *const i8,
        b"       --config=FILE               specify config file to use\n\0" as *const u8
            as *const i8,
        b"       --no-config                 do not read any config file\n\0"
            as *const u8 as *const i8,
        b"       --rejected-log=FILE         log reasons for URL rejection to FILE\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"Download:\n\0" as *const u8 as *const i8,
        b"  -t,  --tries=NUMBER              set number of retries to NUMBER (0 unlimits)\n\0"
            as *const u8 as *const i8,
        b"       --retry-connrefused         retry even if connection is refused\n\0"
            as *const u8 as *const i8,
        b"       --retry-on-host-error       consider host errors as non-fatal, transient errors\n\0"
            as *const u8 as *const i8,
        b"       --retry-on-http-error=ERRORS    comma-separated list of HTTP errors to retry\n\0"
            as *const u8 as *const i8,
        b"  -O,  --output-document=FILE      write documents to FILE\n\0" as *const u8
            as *const i8,
        b"  -nc, --no-clobber                skip downloads that would download to\n                                     existing files (overwriting them)\n\0"
            as *const u8 as *const i8,
        b"       --no-netrc                  don't try to obtain credentials from .netrc\n\0"
            as *const u8 as *const i8,
        b"  -c,  --continue                  resume getting a partially-downloaded file\n\0"
            as *const u8 as *const i8,
        b"       --start-pos=OFFSET          start downloading from zero-based position OFFSET\n\0"
            as *const u8 as *const i8,
        b"       --progress=TYPE             select progress gauge type\n\0" as *const u8
            as *const i8,
        b"       --show-progress             display the progress bar in any verbosity mode\n\0"
            as *const u8 as *const i8,
        b"  -N,  --timestamping              don't re-retrieve files unless newer than\n                                     local\n\0"
            as *const u8 as *const i8,
        b"       --no-if-modified-since      don't use conditional if-modified-since get\n                                     requests in timestamping mode\n\0"
            as *const u8 as *const i8,
        b"       --no-use-server-timestamps  don't set the local file's timestamp by\n                                     the one on the server\n\0"
            as *const u8 as *const i8,
        b"  -S,  --server-response           print server response\n\0" as *const u8
            as *const i8,
        b"       --spider                    don't download anything\n\0" as *const u8
            as *const i8,
        b"  -T,  --timeout=SECONDS           set all timeout values to SECONDS\n\0"
            as *const u8 as *const i8,
        b"       --dns-timeout=SECS          set the DNS lookup timeout to SECS\n\0"
            as *const u8 as *const i8,
        b"       --connect-timeout=SECS      set the connect timeout to SECS\n\0"
            as *const u8 as *const i8,
        b"       --read-timeout=SECS         set the read timeout to SECS\n\0"
            as *const u8 as *const i8,
        b"  -w,  --wait=SECONDS              wait SECONDS between retrievals\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            as *const u8 as *const i8,
        b"       --waitretry=SECONDS         wait 1..SECONDS between retries of a retrieval\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            as *const u8 as *const i8,
        b"       --random-wait               wait from 0.5*WAIT...1.5*WAIT secs between retrievals\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            as *const u8 as *const i8,
        b"       --no-proxy                  explicitly turn off proxy\n\0" as *const u8
            as *const i8,
        b"  -Q,  --quota=NUMBER              set retrieval quota to NUMBER\n\0"
            as *const u8 as *const i8,
        b"       --bind-address=ADDRESS      bind to ADDRESS (hostname or IP) on local host\n\0"
            as *const u8 as *const i8,
        b"       --limit-rate=RATE           limit download rate to RATE\n\0"
            as *const u8 as *const i8,
        b"       --no-dns-cache              disable caching DNS lookups\n\0"
            as *const u8 as *const i8,
        b"       --restrict-file-names=OS    restrict chars in file names to ones OS allows\n\0"
            as *const u8 as *const i8,
        b"       --ignore-case               ignore case when matching files/directories\n\0"
            as *const u8 as *const i8,
        b"  -4,  --inet4-only                connect only to IPv4 addresses\n\0"
            as *const u8 as *const i8,
        b"  -6,  --inet6-only                connect only to IPv6 addresses\n\0"
            as *const u8 as *const i8,
        b"       --prefer-family=FAMILY      connect first to addresses of specified family,\n                                     one of IPv6, IPv4, or none\n\0"
            as *const u8 as *const i8,
        b"       --user=USER                 set both ftp and http user to USER\n\0"
            as *const u8 as *const i8,
        b"       --password=PASS             set both ftp and http password to PASS\n\0"
            as *const u8 as *const i8,
        b"       --ask-password              prompt for passwords\n\0" as *const u8
            as *const i8,
        b"       --use-askpass=COMMAND       specify credential handler for requesting \n                                     username and password.  If no COMMAND is \n                                     specified the WGET_ASKPASS or the SSH_ASKPASS \n                                     environment variable is used.\n\0"
            as *const u8 as *const i8,
        b"       --no-iri                    turn off IRI support\n\0" as *const u8
            as *const i8,
        b"       --local-encoding=ENC        use ENC as the local encoding for IRIs\n\0"
            as *const u8 as *const i8,
        b"       --remote-encoding=ENC       use ENC as the default remote encoding\n\0"
            as *const u8 as *const i8,
        b"       --unlink                    remove file before clobber\n\0" as *const u8
            as *const i8,
        b"       --xattr                     turn on storage of metadata in extended file attributes\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"Directories:\n\0" as *const u8 as *const i8,
        b"  -nd, --no-directories            don't create directories\n\0" as *const u8
            as *const i8,
        b"  -x,  --force-directories         force creation of directories\n\0"
            as *const u8 as *const i8,
        b"  -nH, --no-host-directories       don't create host directories\n\0"
            as *const u8 as *const i8,
        b"       --protocol-directories      use protocol name in directories\n\0"
            as *const u8 as *const i8,
        b"  -P,  --directory-prefix=PREFIX   save files to PREFIX/..\n\0" as *const u8
            as *const i8,
        b"       --cut-dirs=NUMBER           ignore NUMBER remote directory components\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"HTTP options:\n\0" as *const u8 as *const i8,
        b"       --http-user=USER            set http user to USER\n\0" as *const u8
            as *const i8,
        b"       --http-password=PASS        set http password to PASS\n\0" as *const u8
            as *const i8,
        b"       --no-cache                  disallow server-cached data\n\0"
            as *const u8 as *const i8,
        b"       --default-page=NAME         change the default page name (normally\n                                     this is 'index.html'.)\n\0"
            as *const u8 as *const i8,
        b"  -E,  --adjust-extension          save HTML/CSS documents with proper extensions\n\0"
            as *const u8 as *const i8,
        b"       --ignore-length             ignore 'Content-Length' header field\n\0"
            as *const u8 as *const i8,
        b"       --header=STRING             insert STRING among the headers\n\0"
            as *const u8 as *const i8,
        b"       --compression=TYPE          choose compression, one of auto, gzip and none. (default: none)\n\0"
            as *const u8 as *const i8,
        b"       --max-redirect              maximum redirections allowed per page\n\0"
            as *const u8 as *const i8,
        b"       --proxy-user=USER           set USER as proxy username\n\0" as *const u8
            as *const i8,
        b"       --proxy-password=PASS       set PASS as proxy password\n\0" as *const u8
            as *const i8,
        b"       --referer=URL               include 'Referer: URL' header in HTTP request\n\0"
            as *const u8 as *const i8,
        b"       --save-headers              save the HTTP headers to file\n\0"
            as *const u8 as *const i8,
        b"  -U,  --user-agent=AGENT          identify as AGENT instead of Wget/VERSION\n\0"
            as *const u8 as *const i8,
        b"       --no-http-keep-alive        disable HTTP keep-alive (persistent connections)\n\0"
            as *const u8 as *const i8,
        b"       --no-cookies                don't use cookies\n\0" as *const u8
            as *const i8,
        b"       --load-cookies=FILE         load cookies from FILE before session\n\0"
            as *const u8 as *const i8,
        b"       --save-cookies=FILE         save cookies to FILE after session\n\0"
            as *const u8 as *const i8,
        b"       --keep-session-cookies      load and save session (non-permanent) cookies\n\0"
            as *const u8 as *const i8,
        b"       --post-data=STRING          use the POST method; send STRING as the data\n\0"
            as *const u8 as *const i8,
        b"       --post-file=FILE            use the POST method; send contents of FILE\n\0"
            as *const u8 as *const i8,
        b"       --method=HTTPMethod         use method \"HTTPMethod\" in the request\n\0"
            as *const u8 as *const i8,
        b"       --body-data=STRING          send STRING as data. --method MUST be set\n\0"
            as *const u8 as *const i8,
        b"       --body-file=FILE            send contents of FILE. --method MUST be set\n\0"
            as *const u8 as *const i8,
        b"       --content-disposition       honor the Content-Disposition header when\n                                     choosing local file names (EXPERIMENTAL)\n\0"
            as *const u8 as *const i8,
        b"       --content-on-error          output the received content on server errors\n\0"
            as *const u8 as *const i8,
        b"       --auth-no-challenge         send Basic HTTP authentication information\n                                     without first waiting for the server's\n                                     challenge\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"HTTPS (SSL/TLS) options:\n\0" as *const u8 as *const i8,
        b"       --secure-protocol=PR        choose secure protocol, one of auto, SSLv2,\n                                     SSLv3, TLSv1, TLSv1_1, TLSv1_2, TLSv1_3 and PFS\n\0"
            as *const u8 as *const i8,
        b"       --https-only                only follow secure HTTPS links\n\0"
            as *const u8 as *const i8,
        b"       --no-check-certificate      don't validate the server's certificate\n\0"
            as *const u8 as *const i8,
        b"       --certificate=FILE          client certificate file\n\0" as *const u8
            as *const i8,
        b"       --certificate-type=TYPE     client certificate type, PEM or DER\n\0"
            as *const u8 as *const i8,
        b"       --private-key=FILE          private key file\n\0" as *const u8
            as *const i8,
        b"       --private-key-type=TYPE     private key type, PEM or DER\n\0"
            as *const u8 as *const i8,
        b"       --ca-certificate=FILE       file with the bundle of CAs\n\0"
            as *const u8 as *const i8,
        b"       --ca-directory=DIR          directory where hash list of CAs is stored\n\0"
            as *const u8 as *const i8,
        b"       --crl-file=FILE             file with bundle of CRLs\n\0" as *const u8
            as *const i8,
        b"       --pinnedpubkey=FILE/HASHES  Public key (PEM/DER) file, or any number\n                                   of base64 encoded sha256 hashes preceded by\n                                   'sha256//' and separated by ';', to verify\n                                   peer against\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"       --ciphers=STR           Set the priority string (GnuTLS) or cipher list string (OpenSSL) directly.\n                                   Use with care. This option overrides --secure-protocol.\n                                   The format and syntax of this string depend on the specific SSL/TLS engine.\n\0"
            as *const u8 as *const i8,
        b"HSTS options:\n\0" as *const u8 as *const i8,
        b"       --no-hsts                   disable HSTS\n\0" as *const u8 as *const i8,
        b"       --hsts-file                 path of HSTS database (will override default)\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"FTP options:\n\0" as *const u8 as *const i8,
        b"       --ftp-user=USER             set ftp user to USER\n\0" as *const u8
            as *const i8,
        b"       --ftp-password=PASS         set ftp password to PASS\n\0" as *const u8
            as *const i8,
        b"       --no-remove-listing         don't remove '.listing' files\n\0"
            as *const u8 as *const i8,
        b"       --no-glob                   turn off FTP file name globbing\n\0"
            as *const u8 as *const i8,
        b"       --no-passive-ftp            disable the \"passive\" transfer mode\n\0"
            as *const u8 as *const i8,
        b"       --preserve-permissions      preserve remote file permissions\n\0"
            as *const u8 as *const i8,
        b"       --retr-symlinks             when recursing, get linked-to files (not dir)\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"FTPS options:\n\0" as *const u8 as *const i8,
        b"       --ftps-implicit                 use implicit FTPS (default port is 990)\n\0"
            as *const u8 as *const i8,
        b"       --ftps-resume-ssl               resume the SSL/TLS session started in the control connection when\n                                         opening a data connection\n\0"
            as *const u8 as *const i8,
        b"       --ftps-clear-data-connection    cipher the control channel only; all the data will be in plaintext\n\0"
            as *const u8 as *const i8,
        b"       --ftps-fallback-to-ftp          fall back to FTP if FTPS is not supported in the target server\n\0"
            as *const u8 as *const i8,
        b"WARC options:\n\0" as *const u8 as *const i8,
        b"       --warc-file=FILENAME        save request/response data to a .warc.gz file\n\0"
            as *const u8 as *const i8,
        b"       --warc-header=STRING        insert STRING into the warcinfo record\n\0"
            as *const u8 as *const i8,
        b"       --warc-max-size=NUMBER      set maximum size of WARC files to NUMBER\n\0"
            as *const u8 as *const i8,
        b"       --warc-cdx                  write CDX index files\n\0" as *const u8
            as *const i8,
        b"       --warc-dedup=FILENAME       do not store records listed in this CDX file\n\0"
            as *const u8 as *const i8,
        b"       --no-warc-compression       do not compress WARC files with GZIP\n\0"
            as *const u8 as *const i8,
        b"       --no-warc-digests           do not calculate SHA1 digests\n\0"
            as *const u8 as *const i8,
        b"       --no-warc-keep-log          do not store the log file in a WARC record\n\0"
            as *const u8 as *const i8,
        b"       --warc-tempdir=DIRECTORY    location for temporary files created by the\n                                     WARC writer\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"Recursive download:\n\0" as *const u8 as *const i8,
        b"  -r,  --recursive                 specify recursive download\n\0" as *const u8
            as *const i8,
        b"  -l,  --level=NUMBER              maximum recursion depth (inf or 0 for infinite)\n\0"
            as *const u8 as *const i8,
        b"       --delete-after              delete files locally after downloading them\n\0"
            as *const u8 as *const i8,
        b"  -k,  --convert-links             make links in downloaded HTML or CSS point to\n                                     local files\n\0"
            as *const u8 as *const i8,
        b"       --convert-file-only         convert the file part of the URLs only (usually known as the basename)\n\0"
            as *const u8 as *const i8,
        b"       --backups=N                 before writing file X, rotate up to N backup files\n\0"
            as *const u8 as *const i8,
        b"  -K,  --backup-converted          before converting file X, back up as X.orig\n\0"
            as *const u8 as *const i8,
        b"  -m,  --mirror                    shortcut for -N -r -l inf --no-remove-listing\n\0"
            as *const u8 as *const i8,
        b"  -p,  --page-requisites           get all images, etc. needed to display HTML page\n\0"
            as *const u8 as *const i8,
        b"       --strict-comments           turn on strict (SGML) handling of HTML comments\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"Recursive accept/reject:\n\0" as *const u8 as *const i8,
        b"  -A,  --accept=LIST               comma-separated list of accepted extensions\n\0"
            as *const u8 as *const i8,
        b"  -R,  --reject=LIST               comma-separated list of rejected extensions\n\0"
            as *const u8 as *const i8,
        b"       --accept-regex=REGEX        regex matching accepted URLs\n\0"
            as *const u8 as *const i8,
        b"       --reject-regex=REGEX        regex matching rejected URLs\n\0"
            as *const u8 as *const i8,
        b"       --regex-type=TYPE           regex type (posix|pcre)\n\0" as *const u8
            as *const i8,
        b"  -D,  --domains=LIST              comma-separated list of accepted domains\n\0"
            as *const u8 as *const i8,
        b"       --exclude-domains=LIST      comma-separated list of rejected domains\n\0"
            as *const u8 as *const i8,
        b"       --follow-ftp                follow FTP links from HTML documents\n\0"
            as *const u8 as *const i8,
        b"       --follow-tags=LIST          comma-separated list of followed HTML tags\n\0"
            as *const u8 as *const i8,
        b"       --ignore-tags=LIST          comma-separated list of ignored HTML tags\n\0"
            as *const u8 as *const i8,
        b"  -H,  --span-hosts                go to foreign hosts when recursive\n\0"
            as *const u8 as *const i8,
        b"  -L,  --relative                  follow relative links only\n\0" as *const u8
            as *const i8,
        b"  -I,  --include-directories=LIST  list of allowed directories\n\0"
            as *const u8 as *const i8,
        b"       --trust-server-names        use the name specified by the redirection\n                                     URL's last component\n\0"
            as *const u8 as *const i8,
        b"  -X,  --exclude-directories=LIST  list of excluded directories\n\0"
            as *const u8 as *const i8,
        b"  -np, --no-parent                 don't ascend to the parent directory\n\0"
            as *const u8 as *const i8,
        b"\n\0" as *const u8 as *const i8,
        b"Email bug reports, questions, discussions to <bug-wget@gnu.org>\nand/or open issues at https://savannah.gnu.org/bugs/?func=additem&group=wget.\n\0"
            as *const u8 as *const i8,
    ];
    let mut i: size_t = 0;
    if printf(
        dcgettext(
            0 as *const i8,
            b"GNU Wget %s, a non-interactive network retriever.\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        version_string,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if print_usage(0 as i32) < 0 as i32 {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[*const i8; 178]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
    {
        if fputs(dcgettext(0 as *const i8, help[i as usize], 5 as i32), stdout)
            < 0 as i32
        {
            exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
        }
        i = i.wrapping_add(1);
        i;
    }
    exit(C2RustUnnamed_4::WGET_EXIT_SUCCESS as i32);
}
unsafe extern "C" fn secs_to_human_time(mut interval: libc::c_double) -> *mut i8 {
    static mut buf: [i8; 32] = [0; 32];
    let mut secs: i32 = (interval + 0.5f64) as i32;
    let mut hours: i32 = 0;
    let mut mins: i32 = 0;
    let mut days: i32 = 0;
    days = secs / 86400 as i32;
    secs %= 86400 as i32;
    hours = secs / 3600 as i32;
    secs %= 3600 as i32;
    mins = secs / 60 as i32;
    secs %= 60 as i32;
    if days != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%dd %dh %dm %ds\0" as *const u8 as *const i8,
            days,
            hours,
            mins,
            secs,
        );
    } else if hours != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%dh %dm %ds\0" as *const u8 as *const i8,
            hours,
            mins,
            secs,
        );
    } else if mins != 0 {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%dm %ds\0" as *const u8 as *const i8,
            mins,
            secs,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 32]>() as u64,
            b"%ss\0" as *const u8 as *const i8,
            print_decimal(interval),
        );
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn prompt_for_password() -> *mut i8 {
    if !(opt.user).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Password for user %s: \0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(opt.user),
        );
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Password: \0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    return getpass(b"\0" as *const u8 as *const i8);
}
unsafe extern "C" fn run_use_askpass(mut question: *mut i8, mut answer: *mut *mut i8) {
    let mut tmp: [i8; 1024] = [0; 1024];
    let mut pid: pid_t = 0;
    let mut status: i32 = 0;
    let mut com: [i32; 2] = [0; 2];
    let mut bytes: ssize_t = 0 as i32 as ssize_t;
    let mut argv: [*mut i8; 3] = [0 as *mut i8; 3];
    let mut p: *mut i8 = 0 as *mut i8;
    let mut fa: rpl_posix_spawn_file_actions_t = rpl_posix_spawn_file_actions_t {
        _allocated: 0,
        _used: 0,
        _actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    if pipe(com.as_mut_ptr()) == -(1 as i32) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Cannot create pipe\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    status = rpl_posix_spawn_file_actions_init(&mut fa);
    if status != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Error initializing spawn file actions for use-askpass: %d\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            status,
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    status = rpl_posix_spawn_file_actions_adddup2(
        &mut fa,
        com[1 as i32 as usize],
        1 as i32,
    );
    if status != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Error setting spawn file actions for use-askpass: %d\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            status,
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    argv[0 as i32 as usize] = opt.use_askpass;
    argv[1 as i32 as usize] = question;
    argv[2 as i32 as usize] = 0 as *mut i8;
    status = rpl_posix_spawnp(
        &mut pid,
        opt.use_askpass,
        &mut fa,
        0 as *const rpl_posix_spawnattr_t,
        argv.as_mut_ptr() as *const *mut i8,
        environ as *const *mut i8,
    );
    if status != 0 {
        fprintf(
            stderr,
            b"Error spawning %s: %d\n\0" as *const u8 as *const i8,
            opt.use_askpass,
            status,
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    close(com[1 as i32 as usize]);
    bytes = read(
        com[0 as i32 as usize],
        tmp.as_mut_ptr() as *mut libc::c_void,
        (::core::mem::size_of::<[i8; 1024]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    if bytes <= 0 as i32 as i64 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Error reading response from command \"%s %s\": %s\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            opt.use_askpass,
            question,
            strerror(*__errno_location()),
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    tmp[bytes as usize] = '\0' as i32 as i8;
    p = strpbrk(tmp.as_mut_ptr(), b"\r\n\0" as *const u8 as *const i8);
    if !p.is_null() {
        bytes = p.offset_from(tmp.as_mut_ptr()) as i64;
    }
    *answer = xmemdup0(tmp.as_mut_ptr() as *const libc::c_void, bytes as size_t);
}
unsafe extern "C" fn use_askpass(mut u: *mut url) {
    static mut question: [i8; 1024] = [0; 1024];
    if ((*u).user).is_null()
        || *((*u).user).offset(0 as i32 as isize) as i32 == '\0' as i32
    {
        snprintf(
            question.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 1024]>() as u64,
            dcgettext(
                0 as *const i8,
                b"Username for '%s%s': \0" as *const u8 as *const i8,
                5 as i32,
            ),
            scheme_leading_string((*u).scheme),
            (*u).host,
        );
        run_use_askpass(question.as_mut_ptr(), &mut (*u).user);
        if opt.recursive {
            opt.user = xstrdup((*u).user);
        }
    }
    if ((*u).passwd).is_null()
        || *((*u).passwd).offset(0 as i32 as isize) as i32 == '\0' as i32
    {
        snprintf(
            question.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 1024]>() as u64,
            dcgettext(
                0 as *const i8,
                b"Password for '%s%s@%s': \0" as *const u8 as *const i8,
                5 as i32,
            ),
            scheme_leading_string((*u).scheme),
            (*u).user,
            (*u).host,
        );
        run_use_askpass(question.as_mut_ptr(), &mut (*u).passwd);
        if opt.recursive {
            opt.passwd = xstrdup((*u).passwd);
        }
    }
}
unsafe extern "C" fn format_and_print_line(
    mut prefix: *const i8,
    mut line: *const i8,
    mut line_length: i32,
) -> i32 {
    let mut remaining_chars: i32 = 0;
    let mut line_dup: *mut i8 = 0 as *mut i8;
    let mut token: *mut i8 = 0 as *mut i8;
    line_dup = xstrdup(line);
    if printf(b"%s\0" as *const u8 as *const i8, prefix) < 0 as i32 {
        rpl_free(line_dup as *mut libc::c_void);
        line_dup = 0 as *mut i8;
        return -(1 as i32);
    }
    remaining_chars = 0 as i32;
    token = strtok(line_dup, b" \0" as *const u8 as *const i8);
    while !token.is_null() {
        if remaining_chars <= strlen(token) as i32 {
            if printf(b"\n%*c\0" as *const u8 as *const i8, 4 as i32, ' ' as i32)
                < 0 as i32
            {
                rpl_free(line_dup as *mut libc::c_void);
                line_dup = 0 as *mut i8;
                return -(1 as i32);
            }
            remaining_chars = line_length - 4 as i32;
        }
        if printf(b"%s \0" as *const u8 as *const i8, token) < 0 as i32 {
            rpl_free(line_dup as *mut libc::c_void);
            line_dup = 0 as *mut i8;
            return -(1 as i32);
        }
        remaining_chars = (remaining_chars as u64)
            .wrapping_sub((strlen(token)).wrapping_add(1 as i32 as u64)) as i32 as i32;
        token = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
    }
    if printf(b"\n\0" as *const u8 as *const i8) < 0 as i32 {
        rpl_free(line_dup as *mut libc::c_void);
        line_dup = 0 as *mut i8;
        return -(1 as i32);
    }
    rpl_free(line_dup as *mut libc::c_void);
    line_dup = 0 as *mut i8;
    return 0 as i32;
}
unsafe extern "C" fn print_version() {
    let mut wgetrc_title: *const i8 = dcgettext(
        0 as *const i8,
        b"Wgetrc: \0" as *const u8 as *const i8,
        5 as i32,
    );
    let mut locale_title: *const i8 = dcgettext(
        0 as *const i8,
        b"Locale: \0" as *const u8 as *const i8,
        5 as i32,
    );
    let mut compile_title: *const i8 = dcgettext(
        0 as *const i8,
        b"Compile: \0" as *const u8 as *const i8,
        5 as i32,
    );
    let mut link_title: *const i8 = dcgettext(
        0 as *const i8,
        b"Link: \0" as *const u8 as *const i8,
        5 as i32,
    );
    let mut env_wgetrc: *mut i8 = 0 as *mut i8;
    let mut user_wgetrc: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    if printf(
        dcgettext(
            0 as *const i8,
            b"GNU Wget %s built on %s.\n\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        version_string,
        b"linux-gnu\0" as *const u8 as *const i8,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    i = 0 as i32;
    while !(*compiled_features.as_mut_ptr().offset(i as isize)).is_null() {
        let mut line_length: i32 = 72 as i32;
        while line_length > 0 as i32
            && !(*compiled_features.as_mut_ptr().offset(i as isize)).is_null()
        {
            if printf(
                b"%s \0" as *const u8 as *const i8,
                *compiled_features.as_mut_ptr().offset(i as isize),
            ) < 0 as i32
            {
                exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
            }
            line_length
                -= strlen(*compiled_features.as_mut_ptr().offset(i as isize)) as i32
                    + 2 as i32;
            i += 1;
            i;
        }
        if printf(b"\n\0" as *const u8 as *const i8) < 0 as i32 {
            exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
        }
    }
    if printf(b"\n\0" as *const u8 as *const i8) < 0 as i32 {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if printf(b"%s\n\0" as *const u8 as *const i8, wgetrc_title) < 0 as i32 {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    env_wgetrc = wgetrc_env_file_name();
    if !env_wgetrc.is_null() && *env_wgetrc as i32 != 0 {
        if printf(
            dcgettext(
                0 as *const i8,
                b"    %s (env)\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            env_wgetrc,
        ) < 0 as i32
        {
            exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
        }
        rpl_free(env_wgetrc as *mut libc::c_void);
        env_wgetrc = 0 as *mut i8;
    }
    user_wgetrc = wgetrc_user_file_name();
    if !user_wgetrc.is_null() {
        if printf(
            dcgettext(
                0 as *const i8,
                b"    %s (user)\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            user_wgetrc,
        ) < 0 as i32
        {
            exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
        }
        rpl_free(user_wgetrc as *mut libc::c_void);
        user_wgetrc = 0 as *mut i8;
    }
    if printf(
        dcgettext(
            0 as *const i8,
            b"    %s (system)\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        b"/usr/local/etc/wgetrc\0" as *const u8 as *const i8,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if format_and_print_line(
        locale_title,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
        72 as i32,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if !compilation_string.is_null() {
        if format_and_print_line(compile_title, compilation_string, 72 as i32) < 0 as i32
        {
            exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
        }
    }
    if !link_string.is_null() {
        if format_and_print_line(link_title, link_string, 72 as i32) < 0 as i32 {
            exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
        }
    }
    if printf(b"\n\0" as *const u8 as *const i8) < 0 as i32 {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if printf(
        dcgettext(
            0 as *const i8,
            b"Copyright (C) %s Free Software Foundation, Inc.\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        b"2015\0" as *const u8 as *const i8,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if fputs(
        dcgettext(
            0 as *const i8,
            b"License GPLv3+: GNU GPL version 3 or later\n<http://www.gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if fputs(
        dcgettext(
            0 as *const i8,
            b"\nOriginally written by Hrvoje Niksic <hniksic@xemacs.org>.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    if fputs(
        dcgettext(
            0 as *const i8,
            b"Please send bug reports and questions to <bug-wget@gnu.org>.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        stdout,
    ) < 0 as i32
    {
        exit(C2RustUnnamed_4::WGET_EXIT_IO_FAIL as i32);
    }
    exit(C2RustUnnamed_4::WGET_EXIT_SUCCESS as i32);
}
#[no_mangle]
pub static mut program_name: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut program_argstring: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut timer: *mut ptimer = 0 as *const ptimer as *mut ptimer;
#[no_mangle]
pub static mut cleaned_up: i32 = 0;
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut longindex: i32 = 0;
    let mut nurls: i32 = 0;
    let mut argstring_length: i32 = 0;
    let mut use_userconfig: bool = 0 as i32 != 0;
    let mut noconfig: bool = 0 as i32 != 0;
    let mut append_to_log: bool = 0 as i32 != 0;
    cleaned_up = 0 as i32;
    timer = ptimer_new();
    let mut start_time: libc::c_double = ptimer_measure(timer);
    total_downloaded_bytes = 0 as i32 as wgint;
    program_name = *argv.offset(0 as i32 as isize);
    i18n_initialize();
    exec_name = base_name(*argv.offset(0 as i32 as isize));
    argstring_length = 1 as i32;
    i = 1 as i32;
    while i < argc {
        argstring_length = (argstring_length as u64)
            .wrapping_add(
                (strlen(*argv.offset(i as isize)))
                    .wrapping_add(3 as i32 as u64)
                    .wrapping_add(1 as i32 as u64),
            ) as i32 as i32;
        i += 1;
        i;
    }
    p = malloc(argstring_length as u64) as *mut i8;
    program_argstring = p;
    if p.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Memory allocation problem\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        exit(C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32);
    }
    i = 1 as i32;
    while i < argc {
        let mut arglen: i32 = 0;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '"' as i32 as i8;
        arglen = strlen(*argv.offset(i as isize)) as i32;
        memcpy(
            p as *mut libc::c_void,
            *argv.offset(i as isize) as *const libc::c_void,
            arglen as u64,
        );
        p = p.offset(arglen as isize);
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '"' as i32 as i8;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ' ' as i32 as i8;
        i += 1;
        i;
    }
    *p = '\0' as i32 as i8;
    defaults();
    opt.homedir = home_dir();
    init_switches();
    longindex = -(1 as i32);
    while getopt_long(
        argc,
        argv,
        short_options.as_mut_ptr(),
        long_options.as_mut_ptr(),
        &mut longindex,
    ) != -(1 as i32)
    {
        let mut confval: i32 = 0;
        let mut config_opt: *mut cmdline_option = 0 as *mut cmdline_option;
        if !(longindex >= 0 as i32) {
            continue;
        }
        confval = long_options[longindex as usize].val;
        config_opt = &mut *option_data
            .as_mut_ptr()
            .offset((confval & !(1024 as i32)) as isize) as *mut cmdline_option;
        if strcmp(
            ((*config_opt).long_name).as_mut_ptr(),
            b"no-config\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            noconfig = 1 as i32 != 0;
            break;
        } else {
            if !(strcmp(
                ((*config_opt).long_name).as_mut_ptr(),
                b"config\0" as *const u8 as *const i8,
            ) == 0 as i32)
            {
                continue;
            }
            let mut flstats: file_stats_t = file_stats_t {
                access_err: 0,
                st_ino: 0,
                st_dev: 0,
            };
            use_userconfig = 1 as i32 != 0;
            memset(
                &mut flstats as *mut file_stats_t as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<file_stats_t>() as u64,
            );
            if file_exists_p(optarg, &mut flstats) as i32 != 0
                && run_wgetrc(optarg, &mut flstats) as i32 != 0
            {
                break;
            }
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"Exiting due to error in %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                optarg,
            );
            exit(C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32);
        }
    }
    if noconfig as i32 == 0 as i32 && use_userconfig as i32 == 0 as i32 {
        ret = initialize();
        if ret != 0 {
            return ret;
        }
    }
    opterr = 0 as i32;
    optind = 0 as i32;
    longindex = -(1 as i32);
    loop {
        ret = getopt_long(
            argc,
            argv,
            short_options.as_mut_ptr(),
            long_options.as_mut_ptr(),
            &mut longindex,
        );
        if !(ret != -(1 as i32)) {
            break;
        }
        let mut val: i32 = 0;
        let mut cmdopt: *mut cmdline_option = 0 as *mut cmdline_option;
        if longindex == -(1 as i32) {
            if ret == '?' as i32 {
                print_usage(1 as i32);
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"Try `%s --help' for more options.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    exec_name,
                );
                exit(C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32);
            }
            longindex = optmap[(ret - 32 as i32) as usize] as i32;
        }
        val = long_options[longindex as usize].val;
        cmdopt = &mut *option_data.as_mut_ptr().offset((val & !(1024 as i32)) as isize)
            as *mut cmdline_option;
        match (*cmdopt).type_0 as u32 {
            0 => {
                setoptval(
                    (*cmdopt).data as *const i8,
                    optarg,
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
            }
            1 => {
                if !optarg.is_null() {
                    setoptval(
                        (*cmdopt).data as *const i8,
                        optarg,
                        ((*cmdopt).long_name).as_mut_ptr(),
                    );
                } else {
                    let mut neg: bool = val & 1024 as i32 != 0;
                    setoptval(
                        (*cmdopt).data as *const i8,
                        if neg as i32 != 0 {
                            b"0\0" as *const u8 as *const i8
                        } else {
                            b"1\0" as *const u8 as *const i8
                        },
                        ((*cmdopt).long_name).as_mut_ptr(),
                    );
                }
            }
            2 => {
                let mut func: Option<unsafe extern "C" fn() -> ()> = ::core::mem::transmute::<
                    *const libc::c_void,
                    Option<unsafe extern "C" fn() -> ()>,
                >((*cmdopt).data);
                func.expect("non-null function pointer")();
            }
            3 => {
                setoptval(
                    b"logfile\0" as *const u8 as *const i8,
                    optarg,
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
                append_to_log = 1 as i32 != 0;
            }
            6 => {
                if !optarg.is_null() {
                    run_command(optarg);
                }
            }
            7 => {
                p = optarg;
                while !p.is_null() && *p as i32 != 0 {
                    match *p as i32 {
                        118 => {
                            setoptval(
                                b"verbose\0" as *const u8 as *const i8,
                                b"0\0" as *const u8 as *const i8,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        72 => {
                            setoptval(
                                b"addhostdir\0" as *const u8 as *const i8,
                                b"0\0" as *const u8 as *const i8,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        100 => {
                            setoptval(
                                b"dirstruct\0" as *const u8 as *const i8,
                                b"0\0" as *const u8 as *const i8,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        99 => {
                            setoptval(
                                b"noclobber\0" as *const u8 as *const i8,
                                b"1\0" as *const u8 as *const i8,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        112 => {
                            setoptval(
                                b"noparent\0" as *const u8 as *const i8,
                                b"1\0" as *const u8 as *const i8,
                                ((*cmdopt).long_name).as_mut_ptr(),
                            );
                        }
                        _ => {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"%s: illegal option -- `-n%c'\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                exec_name,
                                *p as i32,
                            );
                            print_usage(1 as i32);
                            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"Try `%s --help' for more options.\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                exec_name,
                            );
                            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
                        }
                    }
                    p = p.offset(1);
                    p;
                }
            }
            8 | 4 => {
                let mut flag: bool = 1 as i32 != 0;
                if !optarg.is_null() {
                    flag = *optarg as i32 == '1' as i32
                        || c_tolower(*optarg as i32) == 'y' as i32
                        || c_tolower(*optarg.offset(0 as i32 as isize) as i32)
                            == 'o' as i32
                            && c_tolower(*optarg.offset(1 as i32 as isize) as i32)
                                == 'n' as i32;
                }
                setoptval(
                    if (*cmdopt).type_0 as u32
                        == C2RustUnnamed_5::OPT__PARENT as i32 as u32
                    {
                        b"noparent\0" as *const u8 as *const i8
                    } else {
                        b"noclobber\0" as *const u8 as *const i8
                    },
                    if flag as i32 != 0 {
                        b"0\0" as *const u8 as *const i8
                    } else {
                        b"1\0" as *const u8 as *const i8
                    },
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
            }
            5 => {
                setoptval(
                    b"removelisting\0" as *const u8 as *const i8,
                    b"0\0" as *const u8 as *const i8,
                    ((*cmdopt).long_name).as_mut_ptr(),
                );
            }
            _ => {}
        }
        longindex = -(1 as i32);
    }
    nurls = argc - optind;
    log_init(opt.lfilename, append_to_log);
    if opt.noclobber as i32 != 0
        && (opt.convert_links as i32 != 0 || opt.convert_file_only as i32 != 0)
    {
        fprintf(
            stderr,
            if opt.convert_links as i32 != 0 {
                dcgettext(
                    0 as *const i8,
                    b"Both --no-clobber and --convert-links were specified, only --convert-links will be used.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                )
            } else {
                dcgettext(
                    0 as *const i8,
                    b"Both --no-clobber and --convert-file-only were specified, only --convert-file-only will be used.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
        opt.noclobber = 0 as i32 != 0;
    }
    if opt.reclevel == 0 as i32 {
        opt.reclevel = -(1 as i32);
    }
    if opt.spider as i32 != 0 || opt.delete_after as i32 != 0 {
        opt.no_dirstruct = 1 as i32 != 0;
    }
    if opt.page_requisites as i32 != 0 && !opt.recursive {
        opt.reclevel = 0 as i32;
        if !opt.no_dirstruct {
            opt.dirstruct = 1 as i32 != 0;
        }
    }
    if opt.verbose == -(1 as i32) {
        opt.verbose = !opt.quiet as i32;
    }
    if opt.verbose == 0 && opt.show_progress == -(1 as i32) {
        opt.show_progress = 0 as i32;
    }
    if opt.quiet as i32 != 0 && opt.show_progress == -(1 as i32) {
        opt.show_progress = 0 as i32;
    }
    if opt.verbose != 0 && opt.quiet as i32 != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Can't be verbose and quiet at the same time.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        print_usage(1 as i32);
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    if opt.timestamping as i32 != 0 && opt.noclobber as i32 != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Can't timestamp and not clobber old files at the same time.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        print_usage(1 as i32);
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    if opt.ipv4_only as i32 != 0 && opt.ipv6_only as i32 != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Cannot specify both --inet4-only and --inet6-only.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        print_usage(1 as i32);
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    if !(opt.output_document).is_null() {
        if (opt.convert_links as i32 != 0 || opt.convert_file_only as i32 != 0)
            && (nurls > 1 as i32 || opt.page_requisites as i32 != 0
                || opt.recursive as i32 != 0)
        {
            fputs(
                dcgettext(
                    0 as *const i8,
                    b"Cannot specify both -k or --convert-file-only and -O if multiple URLs are given, or in combination\nwith -p or -r. See the manual for details.\n\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                stderr,
            );
            print_usage(1 as i32);
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
        if opt.page_requisites as i32 != 0 || opt.recursive as i32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s\0" as *const u8 as *const i8,
                dcgettext(
                    0 as *const i8,
                    b"WARNING: combining -O with -r or -p will mean that all downloaded content\nwill be placed in the single file you specified.\n\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if opt.timestamping {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s\0" as *const u8 as *const i8,
                dcgettext(
                    0 as *const i8,
                    b"WARNING: timestamping does nothing in combination with -O. See the manual\nfor details.\n\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            opt.timestamping = 0 as i32 != 0;
        }
        if opt.noclobber as i32 != 0
            && file_exists_p(opt.output_document, 0 as *mut file_stats_t) as i32 != 0
        {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"File %s already there; not retrieving.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quote(opt.output_document),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if !(opt.warc_filename).is_null() {
        if opt.noclobber {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"WARC output does not work with --no-clobber, --no-clobber will be disabled.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            opt.noclobber = 0 as i32 != 0;
        }
        if opt.timestamping {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"WARC output does not work with timestamping, timestamping will be disabled.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            opt.timestamping = 0 as i32 != 0;
        }
        if opt.spider {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"WARC output does not work with --spider.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
        if opt.always_rest as i32 != 0 || opt.start_pos >= 0 as i32 as i64 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"WARC output does not work with --continue or --start-pos, they will be disabled.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            opt.always_rest = 0 as i32 != 0;
            opt.start_pos = -(1 as i32) as wgint;
        }
        if !(opt.warc_cdx_dedup_filename).is_null() && !opt.warc_digests_enabled {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"Digests are disabled; WARC deduplication will not find duplicate records.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if opt.warc_keep_log {
            opt.progress_type = xstrdup(b"dot\0" as *const u8 as *const i8);
        }
    }
    if opt.always_rest as i32 != 0 || opt.start_pos >= 0 as i32 as i64 {
        if opt.compression as u32 == compression_options::compression_auto as i32 as u32
        {
            opt.compression = compression_options::compression_none;
        } else if opt.compression as u32
            != compression_options::compression_none as i32 as u32
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"Compression does not work with --continue or --start-pos, they will be disabled.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            opt.always_rest = 0 as i32 != 0;
            opt.start_pos = -(1 as i32) as wgint;
        }
    }
    if opt.ask_passwd as i32 != 0 && !(opt.passwd).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Cannot specify both --ask-password and --password.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        print_usage(1 as i32);
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    if opt.ask_passwd as i32 != 0
        && !(!(opt.user).is_null() || !(opt.http_user).is_null()
            || !(opt.ftp_user).is_null())
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"WARNING: No username set with --ask-password. This is usually not what you want.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if opt.start_pos >= 0 as i32 as i64 && opt.always_rest as i32 != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Specifying both --start-pos and --continue is not recommended; --continue will be disabled.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        opt.always_rest = 0 as i32 != 0;
    }
    if nurls == 0 && (opt.input_filename).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: missing URL\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
        );
        print_usage(1 as i32);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Try `%s --help' for more options.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    match opt.regex_type as u32 {
        0 => {
            opt.regex_compile_fun = Some(
                compile_pcre_regex
                    as unsafe extern "C" fn(*const i8) -> *mut libc::c_void,
            );
            opt.regex_match_fun = Some(
                match_pcre_regex
                    as unsafe extern "C" fn(*const libc::c_void, *const i8) -> bool,
            );
        }
        1 | _ => {
            opt.regex_compile_fun = Some(
                compile_posix_regex
                    as unsafe extern "C" fn(*const i8) -> *mut libc::c_void,
            );
            opt.regex_match_fun = Some(
                match_posix_regex
                    as unsafe extern "C" fn(*const libc::c_void, *const i8) -> bool,
            );
        }
    }
    if !(opt.acceptregex_s).is_null() {
        opt.acceptregex = (opt.regex_compile_fun)
            .expect("non-null function pointer")(opt.acceptregex_s);
        if (opt.acceptregex).is_null() {
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if !(opt.rejectregex_s).is_null() {
        opt.rejectregex = (opt.regex_compile_fun)
            .expect("non-null function pointer")(opt.rejectregex_s);
        if (opt.rejectregex).is_null() {
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if !(opt.post_data).is_null() || !(opt.post_file_name).is_null() {
        if !(opt.post_data).is_null() && !(opt.post_file_name).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"You cannot specify both --post-data and --post-file.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        } else if !(opt.method).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"You cannot use --post-data or --post-file along with --method. --method expects data through --body-data and --body-file options\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if !(opt.body_data).is_null() || !(opt.body_file).is_null() {
        if (opt.method).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"You must specify a method through --method=HTTPMethod to use with --body-data or --body-file.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        } else if !(opt.body_data).is_null() && !(opt.body_file).is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"You cannot specify both --body-data and --body-file.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if !(opt.method).is_null()
        && c_strcasecmp(opt.method, b"HEAD\0" as *const u8 as *const i8) == 0 as i32
    {
        setoptval(
            b"spider\0" as *const u8 as *const i8,
            b"1\0" as *const u8 as *const i8,
            b"spider\0" as *const u8 as *const i8,
        );
    }
    if !(opt.post_data).is_null() || !(opt.post_file_name).is_null() {
        setoptval(
            b"method\0" as *const u8 as *const i8,
            b"POST\0" as *const u8 as *const i8,
            b"method\0" as *const u8 as *const i8,
        );
        if !(opt.post_data).is_null() {
            setoptval(
                b"bodydata\0" as *const u8 as *const i8,
                opt.post_data,
                b"body-data\0" as *const u8 as *const i8,
            );
            rpl_free(opt.post_data as *mut libc::c_void);
            opt.post_data = 0 as *mut i8;
        } else {
            setoptval(
                b"bodyfile\0" as *const u8 as *const i8,
                opt.post_file_name,
                b"body-file\0" as *const u8 as *const i8,
            );
            rpl_free(opt.post_file_name as *mut libc::c_void);
            opt.post_file_name = 0 as *mut i8;
        }
    }
    if opt.enable_iri {
        if !(opt.locale).is_null() && !check_encoding_name(opt.locale) {
            rpl_free(opt.locale as *mut libc::c_void);
            opt.locale = 0 as *const i8;
        }
        if (opt.locale).is_null() {
            opt.locale = find_locale();
        }
        if !(opt.encoding_remote).is_null() && !check_encoding_name(opt.encoding_remote)
        {
            rpl_free(opt.encoding_remote as *mut libc::c_void);
            opt.encoding_remote = 0 as *mut i8;
        }
    }
    if opt.ask_passwd {
        opt.passwd = prompt_for_password();
        if (opt.passwd).is_null()
            || *(opt.passwd).offset(0 as i32 as isize) as i32 == '\0' as i32
        {
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if !(opt.use_askpass).is_null() {
        if *(opt.use_askpass).offset(0 as i32 as isize) as i32 == '\0' as i32 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"use-askpass requires a string or either environment variable WGET_ASKPASS or SSH_ASKPASS to be set.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if opt.background {
        let mut logfile_changed: bool = fork_to_background();
        if logfile_changed {
            log_init(opt.lfilename, append_to_log);
        }
    }
    if opt.show_progress != 0 {
        set_progress_implementation(opt.progress_type);
    }
    if !(opt.warc_filename).is_null() {
        warc_init();
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"DEBUG output created by Wget %s on %s.\n\n\0" as *const u8 as *const i8,
            version_string,
            b"linux-gnu\0" as *const u8 as *const i8,
        );
    }
    if !(opt.output_document).is_null() {
        if *opt.output_document as i32 == '-' as i32
            && *(opt.output_document).offset(1 as i32 as isize) == 0
        {
            output_stream = stdout;
        } else {
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
            if opt.unlink_requested {
                unlink(opt.output_document);
            }
            output_stream = rpl_fopen(
                opt.output_document,
                if opt.always_rest as i32 != 0 {
                    b"ab\0" as *const u8 as *const i8
                } else {
                    b"wb\0" as *const u8 as *const i8
                },
            );
            if output_stream.is_null() {
                perror(opt.output_document);
                exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
            }
            if fstat(fileno(output_stream), &mut st) == 0 as i32
                && st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
            {
                output_stream_regular = 1 as i32 != 0;
            }
        }
        if !output_stream_regular
            && (opt.convert_links as i32 != 0 || opt.recursive as i32 != 0)
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"-k or -r can be used together with -O only if outputting to a regular file.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
        if !output_stream_regular
            && (opt.convert_links as i32 != 0 || opt.convert_file_only as i32 != 0)
        {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"--convert-links or --convert-file-only can be used together only if outputting to a regular file.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
    }
    if signal(
        1 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    )
        != ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t)
    {
        signal(
            1 as i32,
            Some(redirect_output_signal as unsafe extern "C" fn(i32) -> ()),
        );
    }
    signal(10 as i32, Some(redirect_output_signal as unsafe extern "C" fn(i32) -> ()));
    signal(
        13 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    signal(28 as i32, Some(progress_handle_sigwinch as unsafe extern "C" fn(i32) -> ()));
    if opt.hsts {
        load_hsts();
    }
    i = 0 as i32;
    while i < nurls {
        let mut t: *mut i8 = 0 as *mut i8;
        let mut filename: *mut i8 = 0 as *mut i8;
        let mut redirected_URL: *mut i8 = 0 as *mut i8;
        let mut dt: i32 = 0 as i32;
        let mut url_err: i32 = 0;
        let mut iri: *mut iri = iri_new();
        let mut url_parsed: *mut url = 0 as *mut url;
        t = rewrite_shorthand_url(*argv.offset(optind as isize));
        if t.is_null() {
            t = *argv.offset(optind as isize);
        }
        set_uri_encoding(iri, opt.locale, 1 as i32 != 0);
        url_parsed = url_parse(t, &mut url_err, iri, 1 as i32 != 0);
        if url_parsed.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s: %s.\n\0" as *const u8 as *const i8,
                t,
                url_error(url_err),
            );
            inform_exit_status(uerr_t::URLERROR);
        } else {
            if !(opt.use_askpass).is_null() {
                use_askpass(url_parsed);
            }
            if (opt.recursive as i32 != 0 || opt.page_requisites as i32 != 0)
                && (url_scheme(t) as u32 != url_scheme::SCHEME_FTP as i32 as u32
                    && url_scheme(t) as u32 != url_scheme::SCHEME_FTPS as i32 as u32
                    || url_uses_proxy(url_parsed) as i32 != 0)
            {
                let mut old_follow_ftp: i32 = opt.follow_ftp as i32;
                if url_scheme(t) as u32 == url_scheme::SCHEME_FTP as i32 as u32
                    || url_scheme(t) as u32 == url_scheme::SCHEME_FTPS as i32 as u32
                {
                    opt.follow_ftp = 1 as i32 != 0;
                }
                retrieve_tree(url_parsed, 0 as *mut iri);
                opt.follow_ftp = old_follow_ftp != 0;
            } else {
                retrieve_url(
                    url_parsed,
                    t,
                    &mut filename,
                    &mut redirected_URL,
                    0 as *const i8,
                    &mut dt,
                    opt.recursive,
                    iri,
                    1 as i32 != 0,
                );
            }
            if opt.delete_after as i32 != 0 && !filename.is_null()
                && file_exists_p(filename, 0 as *mut file_stats_t) as i32 != 0
            {
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Removing file due to --delete-after in main():\n\0"
                            as *const u8 as *const i8,
                    );
                }
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Removing %s.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    filename,
                );
                if unlink(filename) != 0 {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        b"unlink: %s\n\0" as *const u8 as *const i8,
                        strerror(*__errno_location()),
                    );
                }
            }
            rpl_free(redirected_URL as *mut libc::c_void);
            redirected_URL = 0 as *mut i8;
            rpl_free(filename as *mut libc::c_void);
            filename = 0 as *mut i8;
            url_free(url_parsed);
        }
        iri_free(iri);
        if t != *argv.offset(optind as isize) {
            rpl_free(t as *mut libc::c_void);
            t = 0 as *mut i8;
        }
        i += 1;
        i;
        optind += 1;
        optind;
    }
    if !(opt.input_filename).is_null() {
        let mut count: i32 = 0;
        let mut status: i32 = 0;
        status = retrieve_from_file(opt.input_filename, opt.force_html, &mut count)
            as i32;
        inform_exit_status(uerr_t::from_libc_c_uint(status as u32));
        if count == 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"No URLs found in %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                opt.input_filename,
            );
        }
    }
    if opt.recursive as i32 != 0 && opt.spider as i32 != 0 {
        print_broken_links();
    }
    if (opt.recursive as i32 != 0 || opt.page_requisites as i32 != 0 || nurls > 1 as i32
        || !(opt.input_filename).is_null() && total_downloaded_bytes != 0 as i32 as i64)
        && total_downloaded_bytes != 0 as i32 as i64
    {
        let mut end_time: libc::c_double = ptimer_measure(timer);
        let mut wall_time: *mut i8 = xstrdup(secs_to_human_time(end_time - start_time));
        let mut download_time: *mut i8 = xstrdup(
            secs_to_human_time(total_download_time),
        );
        ptimer_destroy(timer);
        timer = 0 as *mut ptimer;
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"FINISHED --%s--\nTotal wall clock time: %s\nDownloaded: %d files, %s in %s (%s)\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            datetime_str(time(0 as *mut time_t)),
            wall_time,
            numurls,
            human_readable(total_downloaded_bytes, 10 as i32, 1 as i32),
            download_time,
            retr_rate(total_downloaded_bytes, total_download_time),
        );
        rpl_free(wall_time as *mut libc::c_void);
        wall_time = 0 as *mut i8;
        rpl_free(download_time as *mut libc::c_void);
        download_time = 0 as *mut i8;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Download quota of %s EXCEEDED!\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                human_readable(opt.quota, 10 as i32, 1 as i32),
            );
        }
    }
    if !(opt.cookies_output).is_null() {
        save_cookies();
    }
    if opt.hsts as i32 != 0 && !hsts_store.is_null() {
        save_hsts();
    }
    if (opt.convert_links as i32 != 0 || opt.convert_file_only as i32 != 0)
        && !opt.delete_after
    {
        convert_all_links();
    }
    cleanup();
    exit(get_exit_status());
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