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
    pub type internal_state;
    pub type ptimer;
    pub type hsts_store;
    fn rename(__old: *const i8, __new: *const i8) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn set_content_encoding(i: *mut iri, charset: *const i8);
    fn set_uri_encoding(i: *mut iri, charset: *const i8, force: bool);
    fn iri_free(i: *mut iri);
    fn iri_dup(_: *const iri) -> *mut iri;
    fn iri_new() -> *mut iri;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_strtol(string: *const i8, endptr: *mut *mut i8, base: i32) -> i64;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn escnonprint_uri(_: *const i8) -> *const i8;
    fn quote(arg: *const i8) -> *const i8;
    fn unlink(__name: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn inflate(strm: z_streamp, flush: i32) -> i32;
    fn inflateEnd(strm: z_streamp) -> i32;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: i32,
        version: *const i8,
        stream_size: i32,
    ) -> i32;
    fn inform_exit_status(err: uerr_t);
    fn file_exists_p(_: *const i8, _: *mut file_stats_t) -> bool;
    fn has_html_suffix_p(_: *const i8) -> bool;
    fn number_to_static_string(_: wgint) -> *mut i8;
    fn convert_to_bits(_: wgint) -> wgint;
    fn random_float() -> libc::c_double;
    fn xsleep(_: libc::c_double);
    fn url_parse(
        _: *const i8,
        _: *mut i32,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: i32) -> *const i8;
    fn url_free(_: *mut url);
    fn url_has_scheme(_: *const i8) -> bool;
    fn url_valid_scheme(_: *const i8) -> bool;
    fn uri_merge(_: *const i8, _: *const i8) -> *mut i8;
    fn rewrite_shorthand_url(_: *const i8) -> *mut i8;
    fn progress_create(_: *const i8, _: wgint, _: wgint) -> *mut libc::c_void;
    fn progress_interactive_p(_: *mut libc::c_void) -> bool;
    fn progress_update(_: *mut libc::c_void, _: wgint, _: libc::c_double);
    fn progress_finish(_: *mut libc::c_void, _: libc::c_double);
    fn retrieve_tree(_: *mut url, _: *mut iri) -> uerr_t;
    fn sufmatch(_: *mut *const i8, _: *const i8) -> bool;
    fn ftp_loop(
        _: *mut url,
        _: *mut url,
        _: *mut *mut i8,
        _: *mut i32,
        _: *mut url,
        _: bool,
        _: bool,
    ) -> uerr_t;
    fn http_loop(
        _: *const url,
        _: *mut url,
        _: *mut *mut i8,
        _: *mut *mut i8,
        _: *const i8,
        _: *mut i32,
        _: *mut url,
        _: *mut iri,
    ) -> uerr_t;
    fn hsts_match(_: hsts_store_t, _: *mut url) -> bool;
    fn fd_read(_: i32, _: *mut i8, _: i32, _: libc::c_double) -> i32;
    fn fd_peek(_: i32, _: *mut i8, _: i32, _: libc::c_double) -> i32;
    fn register_download(_: *const i8, _: *const i8);
    fn register_redirection(_: *const i8, _: *const i8);
    fn register_html(_: *const i8);
    fn register_css(_: *const i8);
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn ptimer_read(_: *const ptimer) -> libc::c_double;
    fn ptimer_resolution() -> libc::c_double;
    fn get_urls_file(_: *const i8) -> *mut urlpos;
    fn get_urls_html(
        _: *const i8,
        _: *const i8,
        _: *mut bool,
        _: *mut iri,
    ) -> *mut urlpos;
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
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
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
pub enum C2RustUnnamed_4 {
    TEXTHTML = 0x1,
    RETROKF = 0x2,
    HEAD_ONLY = 0x4,
    SEND_NOCACHE = 0x8,
    ACCEPTRANGES = 0x10,
    ADDED_HTML_EXTENSION = 0x20,
    TEXTCSS = 0x40,
    IF_MODIFIED_SINCE = 0x80,
    METALINK_METADATA = 0x100,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::TEXTHTML => 0x1,
            C2RustUnnamed_4::RETROKF => 0x2,
            C2RustUnnamed_4::HEAD_ONLY => 0x4,
            C2RustUnnamed_4::SEND_NOCACHE => 0x8,
            C2RustUnnamed_4::ACCEPTRANGES => 0x10,
            C2RustUnnamed_4::ADDED_HTML_EXTENSION => 0x20,
            C2RustUnnamed_4::TEXTCSS => 0x40,
            C2RustUnnamed_4::IF_MODIFIED_SINCE => 0x80,
            C2RustUnnamed_4::METALINK_METADATA => 0x100,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            0x1 => C2RustUnnamed_4::TEXTHTML,
            0x2 => C2RustUnnamed_4::RETROKF,
            0x4 => C2RustUnnamed_4::HEAD_ONLY,
            0x8 => C2RustUnnamed_4::SEND_NOCACHE,
            0x10 => C2RustUnnamed_4::ACCEPTRANGES,
            0x20 => C2RustUnnamed_4::ADDED_HTML_EXTENSION,
            0x40 => C2RustUnnamed_4::TEXTCSS,
            0x80 => C2RustUnnamed_4::IF_MODIFIED_SINCE,
            0x100 => C2RustUnnamed_4::METALINK_METADATA,
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
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut i8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: i32,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    rb_read_exactly = 1,
    rb_skip_startpos = 2,
    rb_chunked_transfer_encoding = 4,
    rb_compressed_gzip = 8,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_5::rb_read_exactly => 1,
            C2RustUnnamed_5::rb_skip_startpos => 2,
            C2RustUnnamed_5::rb_chunked_transfer_encoding => 4,
            C2RustUnnamed_5::rb_compressed_gzip => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_5 {
        match value {
            1 => C2RustUnnamed_5::rb_read_exactly,
            2 => C2RustUnnamed_5::rb_skip_startpos,
            4 => C2RustUnnamed_5::rb_chunked_transfer_encoding,
            8 => C2RustUnnamed_5::rb_compressed_gzip,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub chunk_bytes: wgint,
    pub chunk_start: libc::c_double,
    pub sleep_adjust: libc::c_double,
}
pub type hunk_terminator_t = Option<
    unsafe extern "C" fn(*const i8, *const i8, i32) -> *const i8,
>;
pub type hsts_store_t = *mut hsts_store;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct urlpos {
    pub url: *mut url,
    pub local_name: *mut i8,
    #[bitfield(name = "ignore_when_downloading", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "link_relative_p", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "link_complete_p", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "link_base_p", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "link_inline_p", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "link_css_p", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "link_noquote_html_p", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "link_expect_html", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "link_expect_css", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "link_refresh_p", ty = "libc::c_uint", bits = "9..=9")]
    pub ignore_when_downloading_link_relative_p_link_complete_p_link_base_p_link_inline_p_link_css_p_link_noquote_html_p_link_expect_html_link_expect_css_link_refresh_p: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub refresh_timeout: i32,
    pub convert: convert_options,
    pub pos: i32,
    pub size: i32,
    pub next: *mut urlpos,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum convert_options {
    CO_NOCONVERT = 0,
    CO_CONVERT_TO_RELATIVE,
    CO_CONVERT_BASENAME_ONLY,
    CO_CONVERT_TO_COMPLETE,
}
impl convert_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            convert_options::CO_NOCONVERT => 0,
            convert_options::CO_CONVERT_TO_RELATIVE => 1,
            convert_options::CO_CONVERT_BASENAME_ONLY => 2,
            convert_options::CO_CONVERT_TO_COMPLETE => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> convert_options {
        match value {
            0 => convert_options::CO_NOCONVERT,
            1 => convert_options::CO_CONVERT_TO_RELATIVE,
            2 => convert_options::CO_CONVERT_BASENAME_ONLY,
            3 => convert_options::CO_CONVERT_TO_COMPLETE,
            _ => panic!("Invalid value for convert_options: {}", value),
        }
    }
}
impl AddAssign<u32> for convert_options {
    fn add_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for convert_options {
    fn sub_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for convert_options {
    fn mul_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for convert_options {
    fn div_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for convert_options {
    fn rem_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for convert_options {
    type Output = convert_options;
    fn add(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for convert_options {
    type Output = convert_options;
    fn sub(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for convert_options {
    type Output = convert_options;
    fn mul(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for convert_options {
    type Output = convert_options;
    fn div(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for convert_options {
    type Output = convert_options;
    fn rem(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub static mut total_downloaded_bytes: wgint = 0;
#[no_mangle]
pub static mut total_download_time: libc::c_double = 0.;
#[no_mangle]
pub static mut output_stream: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut output_stream_regular: bool = false;
static mut limit_data: C2RustUnnamed_6 = C2RustUnnamed_6 {
    chunk_bytes: 0,
    chunk_start: 0.,
    sleep_adjust: 0.,
};
unsafe extern "C" fn limit_bandwidth_reset() {
    memset(
        &mut limit_data as *mut C2RustUnnamed_6 as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<C2RustUnnamed_6>() as u64,
    );
}
unsafe extern "C" fn zalloc(
    mut opaque: voidpf,
    mut items: u32,
    mut size: u32,
) -> voidpf {
    return xcalloc(items as size_t, size as size_t);
}
unsafe extern "C" fn zfree(mut opaque: voidpf, mut address: voidpf) {
    rpl_free(address);
    address = 0 as *mut libc::c_void;
}
unsafe extern "C" fn limit_bandwidth(mut bytes: wgint, mut timer: *mut ptimer) {
    let mut delta_t: libc::c_double = ptimer_read(timer) - limit_data.chunk_start;
    let mut expected: libc::c_double = 0.;
    limit_data.chunk_bytes += bytes;
    expected = limit_data.chunk_bytes as libc::c_double
        / opt.limit_rate as libc::c_double;
    if expected > delta_t {
        let mut slp: libc::c_double = expected - delta_t + limit_data.sleep_adjust;
        let mut t0: libc::c_double = 0.;
        let mut t1: libc::c_double = 0.;
        if slp < 0.2f64 {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"deferring a %.2f ms sleep (%s/%.2f).\n\0" as *const u8
                        as *const i8,
                    slp * 1000 as i32 as libc::c_double,
                    number_to_static_string(limit_data.chunk_bytes),
                    delta_t,
                );
            }
            return;
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"\nsleeping %.2f ms for %s bytes, adjust %.2f ms\n\0" as *const u8
                    as *const i8,
                slp * 1000 as i32 as libc::c_double,
                number_to_static_string(limit_data.chunk_bytes),
                limit_data.sleep_adjust,
            );
        }
        t0 = ptimer_read(timer);
        xsleep(slp);
        t1 = ptimer_measure(timer);
        limit_data.sleep_adjust = slp - (t1 - t0);
        if limit_data.sleep_adjust > 0.5f64 {
            limit_data.sleep_adjust = 0.5f64;
        } else if limit_data.sleep_adjust < -0.5f64 {
            limit_data.sleep_adjust = -0.5f64;
        }
    }
    limit_data.chunk_bytes = 0 as i32 as wgint;
    limit_data.chunk_start = ptimer_read(timer);
}
unsafe extern "C" fn write_data(
    mut out: *mut FILE,
    mut out2: *mut FILE,
    mut buf: *const i8,
    mut bufsize: i32,
    mut skip: *mut wgint,
    mut written: *mut wgint,
) -> i32 {
    if out.is_null() && out2.is_null() {
        return 1 as i32;
    }
    if !skip.is_null() {
        if *skip > bufsize as i64 {
            *skip -= bufsize as i64;
            return 1 as i32;
        }
        if *skip != 0 {
            buf = buf.offset(*skip as isize);
            bufsize = (bufsize as i64 - *skip) as i32;
            *skip = 0 as i32 as wgint;
            if bufsize == 0 as i32 {
                return 1 as i32;
            }
        }
    }
    if !out.is_null() {
        fwrite(buf as *const libc::c_void, 1 as i32 as size_t, bufsize as size_t, out);
    }
    if !out2.is_null() {
        fwrite(buf as *const libc::c_void, 1 as i32 as size_t, bufsize as size_t, out2);
    }
    if !written.is_null() {
        *written += bufsize as i64;
    }
    if !out.is_null() {
        rpl_fflush(out);
    }
    if !out2.is_null() {
        rpl_fflush(out2);
    }
    if !out.is_null() && ferror(out) != 0 {
        return -(2 as i32)
    } else if !out2.is_null() && ferror(out2) != 0 {
        return -(3 as i32)
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn fd_read_body(
    mut downloaded_filename: *const i8,
    mut fd: i32,
    mut out: *mut FILE,
    mut toread: wgint,
    mut startpos: wgint,
    mut qtyread: *mut wgint,
    mut qtywritten: *mut wgint,
    mut elapsed: *mut libc::c_double,
    mut flags: i32,
    mut out2: *mut FILE,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0 as i32;
    let mut dlbufsize: i32 = if 8192 as i32 >= 64 as i32 * 1024 as i32 {
        8192 as i32
    } else {
        64 as i32 * 1024 as i32
    };
    let mut dlbuf: *mut i8 = xmalloc(dlbufsize as size_t) as *mut i8;
    let mut timer: *mut ptimer = 0 as *mut ptimer;
    let mut last_successful_read_tm: libc::c_double = 0 as i32 as libc::c_double;
    let mut progress: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut progress_interactive: bool = 0 as i32 != 0;
    let mut exact: bool = flags & C2RustUnnamed_5::rb_read_exactly as i32 != 0;
    let mut chunked: bool = flags & C2RustUnnamed_5::rb_chunked_transfer_encoding as i32
        != 0;
    let mut skip: wgint = 0 as i32 as wgint;
    let mut sum_read: wgint = 0 as i32 as wgint;
    let mut sum_written: wgint = 0 as i32 as wgint;
    let mut remaining_chunk_size: wgint = 0 as i32 as wgint;
    let mut gzbufsize: u32 = (dlbufsize * 4 as i32) as u32;
    let mut gzbuf: *mut i8 = 0 as *mut i8;
    let mut gzstream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut i8,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    if flags & C2RustUnnamed_5::rb_compressed_gzip as i32 != 0 {
        gzbuf = xmalloc(gzbufsize as size_t) as *mut i8;
        gzstream.zalloc = Some(
            zalloc as unsafe extern "C" fn(voidpf, u32, u32) -> voidpf,
        );
        gzstream.zfree = Some(zfree as unsafe extern "C" fn(voidpf, voidpf) -> ());
        gzstream.opaque = 0 as voidpf;
        gzstream.next_in = 0 as *mut Bytef;
        gzstream.avail_in = 0 as i32 as uInt;
        ret = inflateInit2_(
            &mut gzstream,
            32 as i32 | 15 as i32,
            b"1.2.11\0" as *const u8 as *const i8,
            ::core::mem::size_of::<z_stream>() as u64 as i32,
        );
        if ret != 0 as i32 {
            rpl_free(gzbuf as *mut libc::c_void);
            gzbuf = 0 as *mut i8;
            *__errno_location() = if ret == -(4 as i32) { 12 as i32 } else { 22 as i32 };
            ret = -(1 as i32);
            current_block = 9120506883806762962;
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    match current_block {
        17833034027772472439 => {
            if flags & C2RustUnnamed_5::rb_skip_startpos as i32 != 0 {
                skip = startpos;
            }
            if opt.show_progress != 0 {
                let mut filename_progress: *const i8 = 0 as *const i8;
                let mut start: wgint = if skip != 0 {
                    0 as i32 as i64
                } else {
                    startpos
                };
                if !(opt.dir_prefix).is_null() {
                    filename_progress = downloaded_filename
                        .offset(strlen(opt.dir_prefix) as isize)
                        .offset(1 as i32 as isize);
                } else {
                    filename_progress = downloaded_filename;
                }
                progress = progress_create(filename_progress, start, start + toread);
                progress_interactive = progress_interactive_p(progress);
            }
            if opt.limit_rate != 0 {
                limit_bandwidth_reset();
            }
            if !progress.is_null() || opt.limit_rate != 0 || !elapsed.is_null() {
                timer = ptimer_new();
                last_successful_read_tm = 0 as i32 as libc::c_double;
            }
            if opt.limit_rate != 0 && opt.limit_rate < dlbufsize as i64 {
                dlbufsize = opt.limit_rate as i32;
            }
            's_160: loop {
                if !(!exact || sum_read < toread) {
                    current_block = 16667286137552459707;
                    break;
                }
                let mut rdsize: i32 = 0;
                let mut tmout: libc::c_double = opt.read_timeout;
                if chunked {
                    if remaining_chunk_size == 0 as i32 as i64 {
                        let mut line: *mut i8 = fd_read_line(fd);
                        let mut endl: *mut i8 = 0 as *mut i8;
                        if line.is_null() {
                            ret = -(1 as i32);
                            current_block = 16667286137552459707;
                            break;
                        } else {
                            if !out2.is_null() {
                                fwrite(
                                    line as *const libc::c_void,
                                    1 as i32 as size_t,
                                    strlen(line),
                                    out2,
                                );
                            }
                            remaining_chunk_size = rpl_strtol(
                                line,
                                &mut endl,
                                16 as i32,
                            );
                            rpl_free(line as *mut libc::c_void);
                            line = 0 as *mut i8;
                            if remaining_chunk_size < 0 as i32 as i64 {
                                ret = -(1 as i32);
                                current_block = 16667286137552459707;
                                break;
                            } else if remaining_chunk_size == 0 as i32 as i64 {
                                ret = 0 as i32;
                                line = fd_read_line(fd);
                                if line.is_null() {
                                    ret = -(1 as i32);
                                } else {
                                    if !out2.is_null() {
                                        fwrite(
                                            line as *const libc::c_void,
                                            1 as i32 as size_t,
                                            strlen(line),
                                            out2,
                                        );
                                    }
                                    rpl_free(line as *mut libc::c_void);
                                    line = 0 as *mut i8;
                                }
                                current_block = 16667286137552459707;
                                break;
                            }
                        }
                    }
                    rdsize = (if remaining_chunk_size <= dlbufsize as i64 {
                        remaining_chunk_size
                    } else {
                        dlbufsize as i64
                    }) as i32;
                } else {
                    rdsize = (if exact as i32 != 0 {
                        if toread - sum_read <= dlbufsize as i64 {
                            toread - sum_read
                        } else {
                            dlbufsize as i64
                        }
                    } else {
                        dlbufsize as i64
                    }) as i32;
                }
                if progress_interactive {
                    tmout = 0.95f64;
                    *__errno_location() = 0 as i32;
                    if opt.read_timeout != 0. {
                        let mut waittm: libc::c_double = 0.;
                        waittm = ptimer_read(timer) - last_successful_read_tm;
                        if waittm + tmout > opt.read_timeout {
                            tmout = opt.read_timeout - waittm;
                            if tmout <= 0 as i32 as libc::c_double {
                                ret = -(1 as i32);
                                *__errno_location() = 110 as i32;
                                current_block = 16667286137552459707;
                                break;
                            }
                        }
                    }
                }
                ret = fd_read(fd, dlbuf, rdsize, tmout);
                if progress_interactive as i32 != 0 && ret < 0 as i32
                    && *__errno_location() == 110 as i32
                {
                    ret = 0 as i32;
                } else if ret <= 0 as i32 {
                    current_block = 16667286137552459707;
                    break;
                }
                if !progress.is_null() || opt.limit_rate != 0 || !elapsed.is_null() {
                    ptimer_measure(timer);
                    if ret > 0 as i32 {
                        last_successful_read_tm = ptimer_read(timer);
                    }
                }
                if ret > 0 as i32 {
                    let mut write_res: i32 = 0;
                    sum_read += ret as i64;
                    if !gzbuf.is_null() {
                        let mut err: i32 = 0;
                        let mut towrite: i32 = 0;
                        write_res = write_data(
                            0 as *mut FILE,
                            out2,
                            dlbuf,
                            ret,
                            0 as *mut wgint,
                            0 as *mut wgint,
                        );
                        if write_res < 0 as i32 {
                            ret = write_res;
                            current_block = 9120506883806762962;
                            break;
                        } else {
                            gzstream.avail_in = ret as uInt;
                            gzstream.next_in = dlbuf as *mut u8;
                            loop {
                                gzstream.avail_out = gzbufsize;
                                gzstream.next_out = gzbuf as *mut u8;
                                err = inflate(&mut gzstream, 0 as i32);
                                match err {
                                    -4 => {
                                        *__errno_location() = 12 as i32;
                                        ret = -(1 as i32);
                                        current_block = 9120506883806762962;
                                        break 's_160;
                                    }
                                    2 | -3 => {
                                        *__errno_location() = 22 as i32;
                                        ret = -(1 as i32);
                                        current_block = 9120506883806762962;
                                        break 's_160;
                                    }
                                    1 => {
                                        if exact as i32 != 0 && sum_read != toread {
                                            if opt.debug as i64 != 0 {
                                                debug_logprintf(
                                                    b"zlib stream ended unexpectedly after %ld/%ld bytes\n\0"
                                                        as *const u8 as *const i8,
                                                    sum_read,
                                                    toread,
                                                );
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                towrite = gzbufsize.wrapping_sub(gzstream.avail_out) as i32;
                                write_res = write_data(
                                    out,
                                    0 as *mut FILE,
                                    gzbuf,
                                    towrite,
                                    &mut skip,
                                    &mut sum_written,
                                );
                                if write_res < 0 as i32 {
                                    ret = write_res;
                                    current_block = 9120506883806762962;
                                    break 's_160;
                                } else if !(gzstream.avail_out == 0 as i32 as u32) {
                                    break;
                                }
                            }
                        }
                    } else {
                        write_res = write_data(
                            out,
                            out2,
                            dlbuf,
                            ret,
                            &mut skip,
                            &mut sum_written,
                        );
                        if write_res < 0 as i32 {
                            ret = write_res;
                            current_block = 9120506883806762962;
                            break;
                        }
                    }
                    if chunked {
                        remaining_chunk_size -= ret as i64;
                        if remaining_chunk_size == 0 as i32 as i64 {
                            let mut line_0: *mut i8 = fd_read_line(fd);
                            if line_0.is_null() {
                                ret = -(1 as i32);
                                current_block = 16667286137552459707;
                                break;
                            } else {
                                if !out2.is_null() {
                                    fwrite(
                                        line_0 as *const libc::c_void,
                                        1 as i32 as size_t,
                                        strlen(line_0),
                                        out2,
                                    );
                                }
                                rpl_free(line_0 as *mut libc::c_void);
                                line_0 = 0 as *mut i8;
                            }
                        }
                    }
                }
                if opt.limit_rate != 0 {
                    limit_bandwidth(ret as wgint, timer);
                }
                if !progress.is_null() {
                    progress_update(progress, ret as wgint, ptimer_read(timer));
                }
            }
            match current_block {
                9120506883806762962 => {}
                _ => {
                    if ret < -(1 as i32) {
                        ret = -(1 as i32);
                    }
                }
            }
        }
        _ => {}
    }
    if !progress.is_null() {
        progress_finish(progress, ptimer_read(timer));
    }
    if !timer.is_null() {
        if !elapsed.is_null() {
            *elapsed = ptimer_read(timer);
        }
        ptimer_destroy(timer);
    }
    if !gzbuf.is_null() {
        let mut err_0: i32 = inflateEnd(&mut gzstream);
        if ret >= 0 as i32 {
            if err_0 == 0 as i32 {
                ret = 0 as i32;
            } else {
                *__errno_location() = 22 as i32;
                ret = -(1 as i32);
            }
        }
        rpl_free(gzbuf as *mut libc::c_void);
        gzbuf = 0 as *mut i8;
        if gzstream.total_in != sum_read as uLong {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"zlib read size differs from raw read size (%lu/%ld)\n\0"
                        as *const u8 as *const i8,
                    gzstream.total_in,
                    sum_read,
                );
            }
        }
    }
    if !qtyread.is_null() {
        *qtyread += sum_read;
    }
    if !qtywritten.is_null() {
        *qtywritten += sum_written;
    }
    rpl_free(dlbuf as *mut libc::c_void);
    dlbuf = 0 as *mut i8;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fd_read_hunk(
    mut fd: i32,
    mut terminator: hunk_terminator_t,
    mut sizehint: i64,
    mut maxsize: i64,
) -> *mut i8 {
    let mut bufsize: i64 = sizehint;
    let mut hunk: *mut i8 = xmalloc(bufsize as size_t) as *mut i8;
    let mut tail: i32 = 0 as i32;
    loop {
        let mut end: *const i8 = 0 as *const i8;
        let mut pklen: i32 = 0;
        let mut rdlen: i32 = 0;
        let mut remain: i32 = 0;
        pklen = fd_peek(
            fd,
            hunk.offset(tail as isize),
            (bufsize - 1 as i32 as i64 - tail as i64) as i32,
            -(1 as i32) as libc::c_double,
        );
        if pklen < 0 as i32 {
            rpl_free(hunk as *mut libc::c_void);
            hunk = 0 as *mut i8;
            return 0 as *mut i8;
        }
        end = terminator
            .expect(
                "non-null function pointer",
            )(hunk, hunk.offset(tail as isize), pklen);
        if !end.is_null() {
            remain = end.offset_from(hunk.offset(tail as isize)) as i64 as i32;
            if remain == 0 as i32 {
                *hunk.offset(tail as isize) = '\0' as i32 as i8;
                return hunk;
            }
            if (bufsize - 1 as i32 as i64) < (tail + remain) as i64 {
                bufsize = (tail + remain + 1 as i32) as i64;
                hunk = xrealloc(hunk as *mut libc::c_void, bufsize as size_t) as *mut i8;
            }
        } else {
            remain = pklen;
        }
        rdlen = fd_read(
            fd,
            hunk.offset(tail as isize),
            remain,
            0 as i32 as libc::c_double,
        );
        if rdlen < 0 as i32 {
            rpl_free(hunk as *mut libc::c_void);
            hunk = 0 as *mut i8;
            return 0 as *mut i8;
        }
        tail += rdlen;
        *hunk.offset(tail as isize) = '\0' as i32 as i8;
        if rdlen == 0 as i32 {
            if tail == 0 as i32 {
                rpl_free(hunk as *mut libc::c_void);
                hunk = 0 as *mut i8;
                *__errno_location() = 0 as i32;
                return 0 as *mut i8;
            } else {
                return hunk
            }
        }
        if !end.is_null() && rdlen == remain {
            return hunk;
        }
        if tail as i64 == bufsize - 1 as i32 as i64 {
            if maxsize != 0 && bufsize >= maxsize {
                rpl_free(hunk as *mut libc::c_void);
                hunk = 0 as *mut i8;
                *__errno_location() = 12 as i32;
                return 0 as *mut i8;
            }
            bufsize <<= 1 as i32;
            if maxsize != 0 && bufsize > maxsize {
                bufsize = maxsize;
            }
            hunk = xrealloc(hunk as *mut libc::c_void, bufsize as size_t) as *mut i8;
        }
    };
}
unsafe extern "C" fn line_terminator(
    mut start: *const i8,
    mut peeked: *const i8,
    mut peeklen: i32,
) -> *const i8 {
    let mut p: *const i8 = memchr(
        peeked as *const libc::c_void,
        '\n' as i32,
        peeklen as u64,
    ) as *const i8;
    if !p.is_null() {
        return p.offset(1 as i32 as isize);
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn fd_read_line(mut fd: i32) -> *mut i8 {
    return fd_read_hunk(
        fd,
        Some(
            line_terminator
                as unsafe extern "C" fn(*const i8, *const i8, i32) -> *const i8,
        ),
        128 as i32 as i64,
        4096 as i32 as i64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn retr_rate(
    mut bytes: wgint,
    mut secs: libc::c_double,
) -> *const i8 {
    static mut res: [i8; 20] = [0; 20];
    static mut rate_names: [*const i8; 4] = [
        b"B/s\0" as *const u8 as *const i8,
        b"KB/s\0" as *const u8 as *const i8,
        b"MB/s\0" as *const u8 as *const i8,
        b"GB/s\0" as *const u8 as *const i8,
    ];
    static mut rate_names_bits: [*const i8; 4] = [
        b"b/s\0" as *const u8 as *const i8,
        b"Kb/s\0" as *const u8 as *const i8,
        b"Mb/s\0" as *const u8 as *const i8,
        b"Gb/s\0" as *const u8 as *const i8,
    ];
    let mut units: i32 = 0;
    let mut dlrate: libc::c_double = calc_rate(bytes, secs, &mut units);
    snprintf(
        res.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 20]>() as u64,
        b"%.*f %s\0" as *const u8 as *const i8,
        if dlrate >= 99.95f64 {
            0 as i32
        } else if dlrate >= 9.995f64 {
            1 as i32
        } else {
            2 as i32
        },
        dlrate,
        if !opt.report_bps {
            rate_names[units as usize]
        } else {
            rate_names_bits[units as usize]
        },
    );
    return res.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn calc_rate(
    mut bytes: wgint,
    mut secs: libc::c_double,
    mut units: *mut i32,
) -> libc::c_double {
    let mut dlrate: libc::c_double = 0.;
    let mut bibyte: libc::c_double = 0.;
    if !opt.report_bps {
        bibyte = 1024.0f64;
    } else {
        bibyte = 1000.0f64;
    }
    if secs == 0 as i32 as libc::c_double {
        secs = ptimer_resolution() / 2.0f64;
    }
    dlrate = if secs != 0. {
        convert_to_bits(bytes) as libc::c_double / secs
    } else {
        0 as i32 as libc::c_double
    };
    if dlrate < bibyte {
        *units = 0 as i32;
    } else if dlrate < bibyte * bibyte {
        *units = 1 as i32;
        dlrate /= bibyte;
    } else if dlrate < bibyte * bibyte * bibyte {
        *units = 2 as i32;
        dlrate /= bibyte * bibyte;
    } else if dlrate < bibyte * bibyte * bibyte * bibyte {
        *units = 3 as i32;
        dlrate /= bibyte * bibyte * bibyte;
    } else {
        *units = 4 as i32;
        dlrate /= bibyte * bibyte * bibyte * bibyte;
        if dlrate > 99.99f64 {
            dlrate = 99.99f64;
        }
    }
    return dlrate;
}
#[no_mangle]
pub unsafe extern "C" fn retrieve_url(
    mut orig_parsed: *mut url,
    mut origurl: *const i8,
    mut file: *mut *mut i8,
    mut newloc: *mut *mut i8,
    mut refurl: *const i8,
    mut dt: *mut i32,
    mut recursive: bool,
    mut iri: *mut iri,
    mut register_status: bool,
) -> uerr_t {
    let mut current_block: u64;
    let mut result: uerr_t = uerr_t::NOCONERROR;
    let mut url: *mut i8 = 0 as *mut i8;
    let mut location_changed: bool = false;
    let mut iri_fallbacked: bool = 0 as i32 != 0;
    let mut dummy: i32 = 0;
    let mut mynewloc: *mut i8 = 0 as *mut i8;
    let mut proxy: *mut i8 = 0 as *mut i8;
    let mut u: *mut url = orig_parsed;
    let mut proxy_url: *mut url = 0 as *mut url;
    let mut up_error_code: i32 = 0;
    let mut local_file: *mut i8 = 0 as *mut i8;
    let mut redirection_count: i32 = 0 as i32;
    let mut method_suspended: bool = 0 as i32 != 0;
    let mut saved_body_data: *mut i8 = 0 as *mut i8;
    let mut saved_method: *mut i8 = 0 as *mut i8;
    let mut saved_body_file_name: *mut i8 = 0 as *mut i8;
    if dt.is_null() {
        dt = &mut dummy;
        dummy = 0 as i32;
    }
    url = xstrdup(origurl);
    if !newloc.is_null() {
        *newloc = 0 as *mut i8;
    }
    if !file.is_null() {
        *file = 0 as *mut i8;
    }
    if refurl.is_null() {
        refurl = opt.referer;
    }
    loop {
        result = uerr_t::NOCONERROR;
        mynewloc = 0 as *mut i8;
        rpl_free(local_file as *mut libc::c_void);
        local_file = 0 as *mut i8;
        proxy_url = 0 as *mut url;
        proxy = getproxy(u);
        if !proxy.is_null() {
            let mut pi: *mut iri = iri_new();
            set_uri_encoding(pi, opt.locale, 1 as i32 != 0);
            (*pi).utf8_encode = 0 as i32 != 0;
            proxy_url = url_parse(proxy, &mut up_error_code, pi, 1 as i32 != 0);
            if proxy_url.is_null() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error parsing proxy URL %s: %s.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    proxy,
                    url_error(up_error_code),
                );
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut i8;
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut i8;
                iri_free(pi);
                if method_suspended {
                    opt.body_data = saved_body_data;
                    opt.body_file = saved_body_file_name;
                    opt.method = saved_method;
                    method_suspended = 0 as i32 != 0;
                }
                result = uerr_t::PROXERR;
                if orig_parsed != u {
                    url_free(u);
                }
                current_block = 9056604636947273900;
                break;
            } else if (*proxy_url).scheme as u32 != url_scheme::SCHEME_HTTP as i32 as u32
                && (*proxy_url).scheme as u32 != (*u).scheme as u32
            {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error in proxy URL %s: Must be HTTP.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    proxy,
                );
                url_free(proxy_url);
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut i8;
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut i8;
                iri_free(pi);
                if method_suspended {
                    opt.body_data = saved_body_data;
                    opt.body_file = saved_body_file_name;
                    opt.method = saved_method;
                    method_suspended = 0 as i32 != 0;
                }
                result = uerr_t::PROXERR;
                if orig_parsed != u {
                    url_free(u);
                }
                current_block = 9056604636947273900;
                break;
            } else {
                iri_free(pi);
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut i8;
            }
        }
        if (*u).scheme as u32 == url_scheme::SCHEME_HTTP as i32 as u32
            || (*u).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32
            || !proxy_url.is_null()
                && (*proxy_url).scheme as u32 == url_scheme::SCHEME_HTTP as i32 as u32
        {
            extern "C" {
                static mut hsts_store: hsts_store_t;
            }
            if opt.hsts as i32 != 0 && !hsts_store.is_null() {
                if hsts_match(hsts_store, u) {
                    logprintf(
                        log_options::LOG_VERBOSE,
                        b"URL transformed to HTTPS due to an HSTS policy\n\0"
                            as *const u8 as *const i8,
                    );
                }
            }
            result = http_loop(
                u,
                orig_parsed,
                &mut mynewloc,
                &mut local_file,
                refurl,
                dt,
                proxy_url,
                iri,
            );
        } else if (*u).scheme as u32 == url_scheme::SCHEME_FTP as i32 as u32
            || (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32
        {
            let mut oldrec: bool = recursive;
            let mut glob: bool = opt.ftp_glob;
            if redirection_count != 0 {
                glob = 0 as i32 != 0;
                oldrec = glob;
            }
            result = ftp_loop(
                u,
                orig_parsed,
                &mut local_file,
                dt,
                proxy_url,
                recursive,
                glob,
            );
            recursive = oldrec;
            if redirection_count != 0 && !local_file.is_null()
                && ((*u).scheme as u32 == url_scheme::SCHEME_FTP as i32 as u32
                    || (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32)
            {
                if has_html_suffix_p(local_file) {
                    *dt |= C2RustUnnamed_4::TEXTHTML as i32;
                }
            }
        }
        if !proxy_url.is_null() {
            url_free(proxy_url);
            proxy_url = 0 as *mut url;
        }
        location_changed = result as u32 == uerr_t::NEWLOCATION as i32 as u32
            || result as u32 == uerr_t::NEWLOCATION_KEEP_POST as i32 as u32;
        if location_changed {
            let mut construced_newloc: *mut i8 = 0 as *mut i8;
            let mut newloc_parsed: *mut url = 0 as *mut url;
            rpl_free(local_file as *mut libc::c_void);
            local_file = 0 as *mut i8;
            construced_newloc = uri_merge(
                url,
                if !mynewloc.is_null() {
                    mynewloc
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
            rpl_free(mynewloc as *mut libc::c_void);
            mynewloc = 0 as *mut i8;
            mynewloc = construced_newloc;
            (*iri).utf8_encode = opt.enable_iri;
            if !(opt.encoding_remote).is_null() {
                set_uri_encoding(iri, opt.encoding_remote, 1 as i32 != 0);
            }
            set_content_encoding(iri, 0 as *const i8);
            rpl_free((*iri).orig_url as *mut libc::c_void);
            (*iri).orig_url = 0 as *mut i8;
            newloc_parsed = url_parse(mynewloc, &mut up_error_code, iri, 1 as i32 != 0);
            if newloc_parsed.is_null() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"%s: %s.\n\0" as *const u8 as *const i8,
                    escnonprint_uri(mynewloc),
                    url_error(up_error_code),
                );
                if orig_parsed != u {
                    url_free(u);
                }
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut i8;
                rpl_free(mynewloc as *mut libc::c_void);
                mynewloc = 0 as *mut i8;
                if method_suspended {
                    opt.body_data = saved_body_data;
                    opt.body_file = saved_body_file_name;
                    opt.method = saved_method;
                    method_suspended = 0 as i32 != 0;
                }
                current_block = 9056604636947273900;
                break;
            } else {
                rpl_free(mynewloc as *mut libc::c_void);
                mynewloc = 0 as *mut i8;
                mynewloc = xstrdup((*newloc_parsed).url);
                redirection_count += 1;
                if redirection_count > opt.max_redirect {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"%d redirections exceeded.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        opt.max_redirect,
                    );
                    url_free(newloc_parsed);
                    if orig_parsed != u {
                        url_free(u);
                    }
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut i8;
                    rpl_free(mynewloc as *mut libc::c_void);
                    mynewloc = 0 as *mut i8;
                    if method_suspended {
                        opt.body_data = saved_body_data;
                        opt.body_file = saved_body_file_name;
                        opt.method = saved_method;
                        method_suspended = 0 as i32 != 0;
                    }
                    result = uerr_t::WRONGCODE;
                    current_block = 9056604636947273900;
                    break;
                } else {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut i8;
                    url = mynewloc;
                    if orig_parsed != u {
                        url_free(u);
                    }
                    u = newloc_parsed;
                    if result as u32 != uerr_t::NEWLOCATION_KEEP_POST as i32 as u32
                        && !method_suspended
                    {
                        method_suspended = 1 as i32 != 0;
                        saved_body_data = opt.body_data;
                        saved_body_file_name = opt.body_file;
                        saved_method = opt.method;
                        opt.body_data = 0 as *mut i8;
                        opt.body_file = 0 as *mut i8;
                        opt.method = 0 as *mut i8;
                    }
                }
            }
        } else {
            rpl_free(mynewloc as *mut libc::c_void);
            mynewloc = 0 as *mut i8;
            if !(*dt & C2RustUnnamed_4::RETROKF as i32 == 0
                && (*iri).utf8_encode as i32 != 0)
            {
                current_block = 16708048892964637133;
                break;
            }
            (*iri).utf8_encode = 0 as i32 != 0;
            if orig_parsed != u {
                url_free(u);
            }
            u = url_parse(origurl, 0 as *mut i32, iri, 1 as i32 != 0);
            if !u.is_null() {
                if strcmp((*u).url, (*orig_parsed).url) != 0 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"[IRI fallbacking to non-utf8 for %s\n\0" as *const u8
                                as *const i8,
                            quote(url),
                        );
                    }
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut i8;
                    url = xstrdup((*u).url);
                    iri_fallbacked = 1 as i32 != 0;
                } else {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"[Needn't fallback to non-utf8 for %s\n\0" as *const u8
                                as *const i8,
                            quote(url),
                        );
                    }
                    current_block = 16708048892964637133;
                    break;
                }
            } else {
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"[Couldn't fallback to non-utf8 for %s\n\0" as *const u8
                            as *const i8,
                        quote(url),
                    );
                }
                current_block = 16708048892964637133;
                break;
            }
        }
    }
    match current_block {
        16708048892964637133 => {
            if !local_file.is_null() && !u.is_null()
                && (*dt & C2RustUnnamed_4::RETROKF as i32 != 0
                    || opt.content_on_error as i32 != 0)
            {
                register_download((*u).url, local_file);
                if !opt.spider && redirection_count != 0
                    && 0 as i32 != strcmp(origurl, (*u).url)
                {
                    register_redirection(origurl, (*u).url);
                }
                if *dt & C2RustUnnamed_4::TEXTHTML as i32 != 0 {
                    register_html(local_file);
                }
                if *dt & C2RustUnnamed_4::TEXTCSS as i32 != 0 {
                    register_css(local_file);
                }
            }
            if !file.is_null() {
                *file = if !local_file.is_null() { local_file } else { 0 as *mut i8 };
            } else {
                rpl_free(local_file as *mut libc::c_void);
                local_file = 0 as *mut i8;
            }
            if orig_parsed != u {
                url_free(u);
            }
            if redirection_count != 0 || iri_fallbacked as i32 != 0 {
                if !newloc.is_null() {
                    *newloc = url;
                } else {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut i8;
                }
            } else {
                if !newloc.is_null() {
                    *newloc = 0 as *mut i8;
                }
                rpl_free(url as *mut libc::c_void);
                url = 0 as *mut i8;
            }
            if method_suspended {
                opt.body_data = saved_body_data;
                opt.body_file = saved_body_file_name;
                opt.method = saved_method;
                method_suspended = 0 as i32 != 0;
            }
        }
        _ => {}
    }
    if register_status {
        inform_exit_status(result);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn retrieve_from_file(
    mut file: *const i8,
    mut html: bool,
    mut count: *mut i32,
) -> uerr_t {
    let mut status: uerr_t = uerr_t::NOCONERROR;
    let mut url_list: *mut urlpos = 0 as *mut urlpos;
    let mut cur_url: *mut urlpos = 0 as *mut urlpos;
    let mut iri: *mut iri = iri_new();
    let mut input_file: *mut i8 = 0 as *mut i8;
    let mut url_file: *mut i8 = 0 as *mut i8;
    let mut url: *const i8 = file;
    status = uerr_t::RETROK;
    *count = 0 as i32;
    set_uri_encoding(iri, opt.locale, 1 as i32 != 0);
    set_content_encoding(iri, opt.locale);
    if url_valid_scheme(url) {
        let mut dt: i32 = 0;
        let mut url_err: i32 = 0;
        let mut url_parsed: *mut url = url_parse(url, &mut url_err, iri, 1 as i32 != 0);
        if url_parsed.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s: %s.\n\0" as *const u8 as *const i8,
                url,
                url_error(url_err),
            );
            iri_free(iri);
            return uerr_t::URLERROR;
        }
        if (opt.base_href).is_null() {
            opt.base_href = xstrdup(url);
        }
        status = retrieve_url(
            url_parsed,
            url,
            &mut url_file,
            0 as *mut *mut i8,
            0 as *const i8,
            &mut dt,
            0 as i32 != 0,
            iri,
            1 as i32 != 0,
        );
        url_free(url_parsed);
        if url_file.is_null() || status as u32 != uerr_t::RETROK as i32 as u32 {
            return status;
        }
        if dt & C2RustUnnamed_4::TEXTHTML as i32 != 0 {
            html = 1 as i32 != 0;
        }
        if (*iri).content_encoding != opt.locale as *mut i8 {
            set_uri_encoding(iri, (*iri).content_encoding, 0 as i32 != 0);
        }
        (*iri).utf8_encode = opt.enable_iri;
        rpl_free((*iri).orig_url as *mut libc::c_void);
        (*iri).orig_url = 0 as *mut i8;
        input_file = url_file;
    } else {
        input_file = file as *mut i8;
    }
    url_list = if html as i32 != 0 {
        get_urls_html(input_file, 0 as *const i8, 0 as *mut bool, iri)
    } else {
        get_urls_file(input_file)
    };
    rpl_free(url_file as *mut libc::c_void);
    url_file = 0 as *mut i8;
    cur_url = url_list;
    while !cur_url.is_null() {
        let mut filename: *mut i8 = 0 as *mut i8;
        let mut new_file: *mut i8 = 0 as *mut i8;
        let mut proxy: *mut i8 = 0 as *mut i8;
        let mut dt_0: i32 = 0 as i32;
        let mut tmpiri: *mut iri = iri_dup(iri);
        let mut parsed_url: *mut url = 0 as *mut url;
        if !((*cur_url).ignore_when_downloading() != 0) {
            if opt.quota != 0 && total_downloaded_bytes > opt.quota {
                status = uerr_t::QUOTEXC;
                break;
            } else {
                parsed_url = url_parse(
                    (*(*cur_url).url).url,
                    0 as *mut i32,
                    tmpiri,
                    1 as i32 != 0,
                );
                proxy = getproxy((*cur_url).url);
                if (opt.recursive as i32 != 0 || opt.page_requisites as i32 != 0)
                    && ((*(*cur_url).url).scheme as u32
                        != url_scheme::SCHEME_FTP as i32 as u32
                        && (*(*cur_url).url).scheme as u32
                            != url_scheme::SCHEME_FTPS as i32 as u32 || !proxy.is_null())
                {
                    let mut old_follow_ftp: i32 = opt.follow_ftp as i32;
                    if (*(*cur_url).url).scheme as u32
                        == url_scheme::SCHEME_FTP as i32 as u32
                        || (*(*cur_url).url).scheme as u32
                            == url_scheme::SCHEME_FTPS as i32 as u32
                    {
                        opt.follow_ftp = 1 as i32 != 0;
                    }
                    status = retrieve_tree(
                        if !parsed_url.is_null() { parsed_url } else { (*cur_url).url },
                        tmpiri,
                    );
                    opt.follow_ftp = old_follow_ftp != 0;
                } else {
                    status = retrieve_url(
                        if !parsed_url.is_null() { parsed_url } else { (*cur_url).url },
                        (*(*cur_url).url).url,
                        &mut filename,
                        &mut new_file,
                        0 as *const i8,
                        &mut dt_0,
                        opt.recursive,
                        tmpiri,
                        1 as i32 != 0,
                    );
                }
                rpl_free(proxy as *mut libc::c_void);
                proxy = 0 as *mut i8;
                if !parsed_url.is_null() {
                    url_free(parsed_url);
                }
                if !filename.is_null() && opt.delete_after as i32 != 0
                    && file_exists_p(filename, 0 as *mut file_stats_t) as i32 != 0
                {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"Removing file due to --delete-after in retrieve_from_file():\n\0"
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
                            b"Failed to unlink %s: (%d) %s\n\0" as *const u8
                                as *const i8,
                            filename,
                            *__errno_location(),
                            strerror(*__errno_location()),
                        );
                    }
                    dt_0 &= !(C2RustUnnamed_4::RETROKF as i32);
                }
                rpl_free(new_file as *mut libc::c_void);
                new_file = 0 as *mut i8;
                rpl_free(filename as *mut libc::c_void);
                filename = 0 as *mut i8;
                iri_free(tmpiri);
            }
        }
        cur_url = (*cur_url).next;
        *count += 1;
        *count;
    }
    free_urlpos(url_list);
    iri_free(iri);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn printwhat(mut n1: i32, mut n2: i32) {
    logputs(
        log_options::LOG_VERBOSE,
        if n1 == n2 {
            dcgettext(
                0 as *const i8,
                b"Giving up.\n\n\0" as *const u8 as *const i8,
                5 as i32,
            )
        } else {
            dcgettext(
                0 as *const i8,
                b"Retrying.\n\n\0" as *const u8 as *const i8,
                5 as i32,
            )
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn sleep_between_retrievals(mut count: i32) {
    static mut first_retrieval: bool = 1 as i32 != 0;
    if first_retrieval {
        first_retrieval = 0 as i32 != 0;
        return;
    }
    if opt.waitretry != 0. && count > 1 as i32 {
        if count as libc::c_double <= opt.waitretry {
            xsleep((count - 1 as i32) as libc::c_double);
        } else {
            xsleep(opt.waitretry);
        }
    } else if opt.wait != 0. {
        if !opt.random_wait || count > 1 as i32 {
            xsleep(opt.wait);
        } else {
            let mut waitsecs: libc::c_double = (0.5f64 + random_float()) * opt.wait;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"sleep_between_retrievals: avg=%f,sleep=%f\n\0" as *const u8
                        as *const i8,
                    opt.wait,
                    waitsecs,
                );
            }
            xsleep(waitsecs);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_urlpos(mut l: *mut urlpos) {
    while !l.is_null() {
        let mut next: *mut urlpos = (*l).next;
        if !((*l).url).is_null() {
            url_free((*l).url);
        }
        rpl_free((*l).local_name as *mut libc::c_void);
        (*l).local_name = 0 as *mut i8;
        rpl_free(l as *mut libc::c_void);
        l = 0 as *mut urlpos;
        l = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rotate_backups(mut fname: *const i8) {
    let mut from: [i8; 1024] = [0; 1024];
    let mut to: [i8; 1024] = [0; 1024];
    let mut sb: stat = stat {
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
    let mut overflow: bool = false;
    let mut i: i32 = 0;
    if stat(fname, &mut sb) == 0 as i32 {
        if (sb.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) as i32
            == 0 as i32
        {
            return;
        }
    }
    i = opt.backups;
    while i > 1 as i32 {
        overflow = snprintf(
            to.as_mut_ptr(),
            1024 as i32 as u64,
            b"%s%s%d\0" as *const u8 as *const i8,
            fname,
            b".\0" as *const u8 as *const i8,
            i,
        ) as u32 >= 1024 as i32 as u32;
        overflow = (overflow as i32
            | (snprintf(
                from.as_mut_ptr(),
                1024 as i32 as u64,
                b"%s%s%d\0" as *const u8 as *const i8,
                fname,
                b".\0" as *const u8 as *const i8,
                i - 1 as i32,
            ) as u32 >= 1024 as i32 as u32) as i32) as bool;
        if overflow {
            *__errno_location() = 36 as i32;
        }
        if overflow as i32 != 0 || rename(from.as_mut_ptr(), to.as_mut_ptr()) != 0 {
            if *__errno_location() != 2 as i32 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"Failed to rename %s to %s: (%d) %s\n\0" as *const u8 as *const i8,
                    from.as_mut_ptr(),
                    to.as_mut_ptr(),
                    *__errno_location(),
                    strerror(*__errno_location()),
                );
            }
        }
        i -= 1;
        i;
    }
    overflow = snprintf(
        to.as_mut_ptr(),
        1024 as i32 as u64,
        b"%s%s%d\0" as *const u8 as *const i8,
        fname,
        b".\0" as *const u8 as *const i8,
        1 as i32,
    ) as u32 >= 1024 as i32 as u32;
    if overflow {
        *__errno_location() = 36 as i32;
    }
    if overflow as i32 != 0 || rename(fname, to.as_mut_ptr()) != 0 {
        if *__errno_location() != 2 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"Failed to rename %s to %s: (%d) %s\n\0" as *const u8 as *const i8,
                from.as_mut_ptr(),
                to.as_mut_ptr(),
                *__errno_location(),
                strerror(*__errno_location()),
            );
        }
    }
}
unsafe extern "C" fn getproxy(mut u: *mut url) -> *mut i8 {
    let mut proxy: *mut i8 = 0 as *mut i8;
    let mut rewritten_url: *mut i8 = 0 as *mut i8;
    if !opt.use_proxy {
        return 0 as *mut i8;
    }
    if no_proxy_match((*u).host, opt.no_proxy as *mut *const i8) {
        return 0 as *mut i8;
    }
    match (*u).scheme as u32 {
        0 => {
            proxy = if !(opt.http_proxy).is_null() {
                opt.http_proxy
            } else {
                getenv(b"http_proxy\0" as *const u8 as *const i8)
            };
        }
        1 => {
            proxy = if !(opt.https_proxy).is_null() {
                opt.https_proxy
            } else {
                getenv(b"https_proxy\0" as *const u8 as *const i8)
            };
        }
        3 => {
            proxy = if !(opt.ftp_proxy).is_null() {
                opt.ftp_proxy
            } else {
                getenv(b"ftps_proxy\0" as *const u8 as *const i8)
            };
        }
        2 => {
            proxy = if !(opt.ftp_proxy).is_null() {
                opt.ftp_proxy
            } else {
                getenv(b"ftp_proxy\0" as *const u8 as *const i8)
            };
        }
        4 | _ => {}
    }
    if proxy.is_null() || *proxy == 0 {
        return 0 as *mut i8;
    }
    rewritten_url = rewrite_shorthand_url(proxy);
    if !rewritten_url.is_null() {
        return rewritten_url;
    }
    return strdup(proxy);
}
#[no_mangle]
pub unsafe extern "C" fn url_uses_proxy(mut u: *mut url) -> bool {
    let mut ret: bool = false;
    let mut proxy: *mut i8 = 0 as *mut i8;
    if u.is_null() {
        return 0 as i32 != 0;
    }
    proxy = getproxy(u);
    ret = !proxy.is_null();
    rpl_free(proxy as *mut libc::c_void);
    proxy = 0 as *mut i8;
    return ret;
}
unsafe extern "C" fn no_proxy_match(
    mut host: *const i8,
    mut no_proxy: *mut *const i8,
) -> bool {
    if no_proxy.is_null() {
        return 0 as i32 != 0
    } else {
        return sufmatch(no_proxy, host)
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_local_file(
    mut file: *mut *const i8,
    mut default_file: *const i8,
) {
    if !(opt.output_document).is_null() {
        if output_stream_regular {
            *file = opt.output_document;
        }
    } else {
        *file = default_file;
    };
}
#[no_mangle]
pub unsafe extern "C" fn input_file_url(mut input_file: *const i8) -> bool {
    static mut first: bool = 1 as i32 != 0;
    if !input_file.is_null() && url_has_scheme(input_file) as i32 != 0
        && first as i32 != 0
    {
        first = 0 as i32 != 0;
        return 1 as i32 != 0;
    } else {
        return 0 as i32 != 0
    };
}