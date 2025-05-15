use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn quote_n(n: i32, arg: *const i8) -> *const i8;
    fn idn_encode(i: *const iri, host: *const i8) -> *mut i8;
    fn remote_to_utf8(i: *const iri, str: *const i8, newstr: *mut *mut i8) -> bool;
    fn unlink(__name: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn strdupdelim(_: *const i8, _: *const i8) -> *mut i8;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn concat_strings(_: *const i8, _: ...) -> *mut i8;
    fn file_exists_p(_: *const i8, _: *mut file_stats_t) -> bool;
    fn file_non_directory_p(_: *const i8) -> bool;
    fn make_directory(_: *const i8) -> i32;
    fn unique_name_passthrough(_: *const i8) -> *mut i8;
    fn numdigit(_: wgint) -> i32;
    fn number_to_string(_: *mut i8, _: wgint) -> *mut i8;
    fn get_max_length(path: *const i8, length: i32, name: i32) -> i64;
    fn is_valid_ipv6_address(_: *const i8, _: *const i8) -> bool;
    fn c_strncasecmp(s1: *const i8, s2: *const i8, n: size_t) -> i32;
    fn iconv_open(__tocode: *const i8, __fromcode: *const i8) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut i8,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut i8,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn iconv_close(__cd: iconv_t) -> i32;
    fn nl_langinfo(__item: nl_item) -> *mut i8;
}
pub type __int64_t = i64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
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
    _PC_2_SYMLINKS = 20,
    _PC_SYMLINK_MAX = 19,
    _PC_ALLOC_SIZE_MIN = 18,
    _PC_REC_XFER_ALIGN = 17,
    _PC_REC_MIN_XFER_SIZE = 16,
    _PC_REC_MAX_XFER_SIZE = 15,
    _PC_REC_INCR_XFER_SIZE = 14,
    _PC_FILESIZEBITS = 13,
    _PC_SOCK_MAXBUF = 12,
    _PC_PRIO_IO = 11,
    _PC_ASYNC_IO = 10,
    _PC_SYNC_IO = 9,
    _PC_VDISABLE = 8,
    _PC_NO_TRUNC = 7,
    _PC_CHOWN_RESTRICTED = 6,
    _PC_PIPE_BUF = 5,
    _PC_PATH_MAX = 4,
    _PC_NAME_MAX = 3,
    _PC_MAX_INPUT = 2,
    _PC_MAX_CANON = 1,
    _PC_LINK_MAX = 0,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::_PC_2_SYMLINKS => 20,
            C2RustUnnamed_4::_PC_SYMLINK_MAX => 19,
            C2RustUnnamed_4::_PC_ALLOC_SIZE_MIN => 18,
            C2RustUnnamed_4::_PC_REC_XFER_ALIGN => 17,
            C2RustUnnamed_4::_PC_REC_MIN_XFER_SIZE => 16,
            C2RustUnnamed_4::_PC_REC_MAX_XFER_SIZE => 15,
            C2RustUnnamed_4::_PC_REC_INCR_XFER_SIZE => 14,
            C2RustUnnamed_4::_PC_FILESIZEBITS => 13,
            C2RustUnnamed_4::_PC_SOCK_MAXBUF => 12,
            C2RustUnnamed_4::_PC_PRIO_IO => 11,
            C2RustUnnamed_4::_PC_ASYNC_IO => 10,
            C2RustUnnamed_4::_PC_SYNC_IO => 9,
            C2RustUnnamed_4::_PC_VDISABLE => 8,
            C2RustUnnamed_4::_PC_NO_TRUNC => 7,
            C2RustUnnamed_4::_PC_CHOWN_RESTRICTED => 6,
            C2RustUnnamed_4::_PC_PIPE_BUF => 5,
            C2RustUnnamed_4::_PC_PATH_MAX => 4,
            C2RustUnnamed_4::_PC_NAME_MAX => 3,
            C2RustUnnamed_4::_PC_MAX_INPUT => 2,
            C2RustUnnamed_4::_PC_MAX_CANON => 1,
            C2RustUnnamed_4::_PC_LINK_MAX => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            20 => C2RustUnnamed_4::_PC_2_SYMLINKS,
            19 => C2RustUnnamed_4::_PC_SYMLINK_MAX,
            18 => C2RustUnnamed_4::_PC_ALLOC_SIZE_MIN,
            17 => C2RustUnnamed_4::_PC_REC_XFER_ALIGN,
            16 => C2RustUnnamed_4::_PC_REC_MIN_XFER_SIZE,
            15 => C2RustUnnamed_4::_PC_REC_MAX_XFER_SIZE,
            14 => C2RustUnnamed_4::_PC_REC_INCR_XFER_SIZE,
            13 => C2RustUnnamed_4::_PC_FILESIZEBITS,
            12 => C2RustUnnamed_4::_PC_SOCK_MAXBUF,
            11 => C2RustUnnamed_4::_PC_PRIO_IO,
            10 => C2RustUnnamed_4::_PC_ASYNC_IO,
            9 => C2RustUnnamed_4::_PC_SYNC_IO,
            8 => C2RustUnnamed_4::_PC_VDISABLE,
            7 => C2RustUnnamed_4::_PC_NO_TRUNC,
            6 => C2RustUnnamed_4::_PC_CHOWN_RESTRICTED,
            5 => C2RustUnnamed_4::_PC_PIPE_BUF,
            4 => C2RustUnnamed_4::_PC_PATH_MAX,
            3 => C2RustUnnamed_4::_PC_NAME_MAX,
            2 => C2RustUnnamed_4::_PC_MAX_INPUT,
            1 => C2RustUnnamed_4::_PC_MAX_CANON,
            0 => C2RustUnnamed_4::_PC_LINK_MAX,
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
pub enum url_auth_mode {
    URL_AUTH_SHOW,
    URL_AUTH_HIDE_PASSWD,
    URL_AUTH_HIDE,
}
impl url_auth_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            url_auth_mode::URL_AUTH_SHOW => 0,
            url_auth_mode::URL_AUTH_HIDE_PASSWD => 1,
            url_auth_mode::URL_AUTH_HIDE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> url_auth_mode {
        match value {
            0 => url_auth_mode::URL_AUTH_SHOW,
            1 => url_auth_mode::URL_AUTH_HIDE_PASSWD,
            2 => url_auth_mode::URL_AUTH_HIDE,
            _ => panic!("Invalid value for url_auth_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for url_auth_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for url_auth_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for url_auth_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for url_auth_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for url_auth_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn add(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn sub(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn mul(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn div(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn rem(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
pub enum C2RustUnnamed_7 {
    urlchr_reserved = 1,
    urlchr_unsafe = 2,
}
impl C2RustUnnamed_7 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_7::urlchr_reserved => 1,
            C2RustUnnamed_7::urlchr_unsafe => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_7 {
        match value {
            1 => C2RustUnnamed_7::urlchr_reserved,
            2 => C2RustUnnamed_7::urlchr_unsafe,
            _ => panic!("Invalid value for C2RustUnnamed_7: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_7 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_7 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_7 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_7 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_7 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn add(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn sub(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn mul(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn div(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn rem(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scheme_data {
    pub name: *const i8,
    pub leading_string: *const i8,
    pub default_port: i32,
    pub flags: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_6 {
    scm_disabled = 1,
    scm_has_params = 2,
    scm_has_query = 4,
    scm_has_fragment = 8,
}
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_6::scm_disabled => 1,
            C2RustUnnamed_6::scm_has_params => 2,
            C2RustUnnamed_6::scm_has_query => 4,
            C2RustUnnamed_6::scm_has_fragment => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_6 {
        match value {
            1 => C2RustUnnamed_6::scm_disabled,
            2 => C2RustUnnamed_6::scm_has_params,
            4 => C2RustUnnamed_6::scm_has_query,
            8 => C2RustUnnamed_6::scm_has_fragment,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_8 {
    PE_NO_ERROR = 0,
    PE_UNSUPPORTED_SCHEME,
    PE_UNSUPPORTED_SCHEME_HTTPS,
    PE_UNSUPPORTED_SCHEME_FTPS,
    PE_MISSING_SCHEME,
    PE_INVALID_HOST_NAME,
    PE_BAD_PORT_NUMBER,
    PE_INVALID_USER_NAME,
    PE_UNTERMINATED_IPV6_ADDRESS,
    PE_IPV6_NOT_SUPPORTED,
    PE_INVALID_IPV6_ADDRESS,
}
impl C2RustUnnamed_8 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_8::PE_NO_ERROR => 0,
            C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME => 1,
            C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_HTTPS => 2,
            C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_FTPS => 3,
            C2RustUnnamed_8::PE_MISSING_SCHEME => 4,
            C2RustUnnamed_8::PE_INVALID_HOST_NAME => 5,
            C2RustUnnamed_8::PE_BAD_PORT_NUMBER => 6,
            C2RustUnnamed_8::PE_INVALID_USER_NAME => 7,
            C2RustUnnamed_8::PE_UNTERMINATED_IPV6_ADDRESS => 8,
            C2RustUnnamed_8::PE_IPV6_NOT_SUPPORTED => 9,
            C2RustUnnamed_8::PE_INVALID_IPV6_ADDRESS => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_8 {
        match value {
            0 => C2RustUnnamed_8::PE_NO_ERROR,
            1 => C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME,
            2 => C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_HTTPS,
            3 => C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_FTPS,
            4 => C2RustUnnamed_8::PE_MISSING_SCHEME,
            5 => C2RustUnnamed_8::PE_INVALID_HOST_NAME,
            6 => C2RustUnnamed_8::PE_BAD_PORT_NUMBER,
            7 => C2RustUnnamed_8::PE_INVALID_USER_NAME,
            8 => C2RustUnnamed_8::PE_UNTERMINATED_IPV6_ADDRESS,
            9 => C2RustUnnamed_8::PE_IPV6_NOT_SUPPORTED,
            10 => C2RustUnnamed_8::PE_INVALID_IPV6_ADDRESS,
            _ => panic!("Invalid value for C2RustUnnamed_8: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_8 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_8 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_8 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_8 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_8 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn add(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn sub(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn mul(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn div(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn rem(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct growable {
    pub base: *mut i8,
    pub size: i32,
    pub tail: i32,
}
pub type iconv_t = *mut libc::c_void;
pub const CODESET: C2RustUnnamed_5 = 14;
pub type nl_item = i32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_9 {
    filechr_not_unix = 1,
    filechr_not_vms = 2,
    filechr_not_windows = 4,
    filechr_control = 8,
}
impl C2RustUnnamed_9 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_9::filechr_not_unix => 1,
            C2RustUnnamed_9::filechr_not_vms => 2,
            C2RustUnnamed_9::filechr_not_windows => 4,
            C2RustUnnamed_9::filechr_control => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_9 {
        match value {
            1 => C2RustUnnamed_9::filechr_not_unix,
            2 => C2RustUnnamed_9::filechr_not_vms,
            4 => C2RustUnnamed_9::filechr_not_windows,
            8 => C2RustUnnamed_9::filechr_control,
            _ => panic!("Invalid value for C2RustUnnamed_9: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_9 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_9 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_9 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_9 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_9 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn add(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn sub(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn mul(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn div(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn rem(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type C2RustUnnamed_5 = u32;
pub const _NL_NUM: C2RustUnnamed_5 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_5 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_5 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_5 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_5 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_5 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_5 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_5 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_5 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_5 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_5 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_5 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_5 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_5 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_5 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_5 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_5 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_5 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_5 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_5 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_5 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_5 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_5 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_5 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_5 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_5 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_5 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_5 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_5 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_5 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_5 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_5 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_5 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_5 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_5 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_5 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_5 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_5 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_5 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_5 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_5 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_5 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_5 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_5 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_5 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_5 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_5 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_5 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_5 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_5 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_5 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_5 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_5 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_5 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_5 = 327684;
pub const __NOSTR: C2RustUnnamed_5 = 327683;
pub const __YESSTR: C2RustUnnamed_5 = 327682;
pub const __NOEXPR: C2RustUnnamed_5 = 327681;
pub const __YESEXPR: C2RustUnnamed_5 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_5 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_5 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_5 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_5 = 65539;
pub const __GROUPING: C2RustUnnamed_5 = 65538;
pub const THOUSEP: C2RustUnnamed_5 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_5 = 65537;
pub const RADIXCHAR: C2RustUnnamed_5 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_5 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_5 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_5 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_5 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_5 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_5 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_5 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_5 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_5 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_5 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_5 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_5 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_5 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_5 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_5 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_5 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_5 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_5 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_5 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_5 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_5 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_5 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_5 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_5 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_5 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_5 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_5 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_5 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_5 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_5 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_5 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_5 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_5 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_5 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_5 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_5 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_5 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_5 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_5 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_5 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_5 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_5 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_5 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_5 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_5 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_5 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_5 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_5 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_5 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_5 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_5 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_5 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_5 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_5 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_5 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_5 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_5 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_5 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_5 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_5 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_5 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_5 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_5 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_5 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_5 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_5 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_5 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_5 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_5 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_5 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_5 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_5 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_5 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_5 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_5 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_5 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_5 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_5 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_5 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_5 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_5 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_5 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_5 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_5 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_5 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_5 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_5 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_5 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_5 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_5 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_5 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_5 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_5 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_5 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_5 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_5 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_5 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_5 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_5 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_5 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_5 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_5 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_5 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_5 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_5 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_5 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_5 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_5 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_5 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_5 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_5 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_5 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_5 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_5 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_5 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_5 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_5 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_5 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_5 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_5 = 15;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_5 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_5 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_5 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_5 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_5 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_5 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_5 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_5 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_5 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_5 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_5 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_5 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_5 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_5 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_5 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_5 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_5 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_5 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_5 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_5 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_5 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_5 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_5 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_5 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_5 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_5 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_5 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_5 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_5 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_5 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_5 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_5 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_5 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_5 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_5 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_5 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_5 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_5 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_5 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_5 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_5 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_5 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_5 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_5 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_5 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_5 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_5 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_5 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_5 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_5 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_5 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_5 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_5 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_5 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_5 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_5 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_5 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_5 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_5 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_5 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_5 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_5 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_5 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_5 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_5 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_5 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_5 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_5 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_5 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_5 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_5 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_5 = 131195;
pub const __ALTMON_12: C2RustUnnamed_5 = 131194;
pub const __ALTMON_11: C2RustUnnamed_5 = 131193;
pub const __ALTMON_10: C2RustUnnamed_5 = 131192;
pub const __ALTMON_9: C2RustUnnamed_5 = 131191;
pub const __ALTMON_8: C2RustUnnamed_5 = 131190;
pub const __ALTMON_7: C2RustUnnamed_5 = 131189;
pub const __ALTMON_6: C2RustUnnamed_5 = 131188;
pub const __ALTMON_5: C2RustUnnamed_5 = 131187;
pub const __ALTMON_4: C2RustUnnamed_5 = 131186;
pub const __ALTMON_3: C2RustUnnamed_5 = 131185;
pub const __ALTMON_2: C2RustUnnamed_5 = 131184;
pub const __ALTMON_1: C2RustUnnamed_5 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_5 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_5 = 131181;
pub const _DATE_FMT: C2RustUnnamed_5 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_5 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_5 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_5 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_5 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_5 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_5 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_5 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_5 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_5 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_5 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_5 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_5 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_5 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_5 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_5 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_5 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_5 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_5 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_5 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_5 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_5 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_5 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_5 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_5 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_5 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_5 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_5 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_5 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_5 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_5 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_5 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_5 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_5 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_5 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_5 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_5 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_5 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_5 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_5 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_5 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_5 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_5 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_5 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_5 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_5 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_5 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_5 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_5 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_5 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_5 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_5 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_5 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_5 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_5 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_5 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_5 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_5 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_5 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_5 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_5 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_5 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_5 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_5 = 131117;
pub const ERA: C2RustUnnamed_5 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_5 = 131115;
pub const T_FMT: C2RustUnnamed_5 = 131114;
pub const D_FMT: C2RustUnnamed_5 = 131113;
pub const D_T_FMT: C2RustUnnamed_5 = 131112;
pub const PM_STR: C2RustUnnamed_5 = 131111;
pub const AM_STR: C2RustUnnamed_5 = 131110;
pub const MON_12: C2RustUnnamed_5 = 131109;
pub const MON_11: C2RustUnnamed_5 = 131108;
pub const MON_10: C2RustUnnamed_5 = 131107;
pub const MON_9: C2RustUnnamed_5 = 131106;
pub const MON_8: C2RustUnnamed_5 = 131105;
pub const MON_7: C2RustUnnamed_5 = 131104;
pub const MON_6: C2RustUnnamed_5 = 131103;
pub const MON_5: C2RustUnnamed_5 = 131102;
pub const MON_4: C2RustUnnamed_5 = 131101;
pub const MON_3: C2RustUnnamed_5 = 131100;
pub const MON_2: C2RustUnnamed_5 = 131099;
pub const MON_1: C2RustUnnamed_5 = 131098;
pub const ABMON_12: C2RustUnnamed_5 = 131097;
pub const ABMON_11: C2RustUnnamed_5 = 131096;
pub const ABMON_10: C2RustUnnamed_5 = 131095;
pub const ABMON_9: C2RustUnnamed_5 = 131094;
pub const ABMON_8: C2RustUnnamed_5 = 131093;
pub const ABMON_7: C2RustUnnamed_5 = 131092;
pub const ABMON_6: C2RustUnnamed_5 = 131091;
pub const ABMON_5: C2RustUnnamed_5 = 131090;
pub const ABMON_4: C2RustUnnamed_5 = 131089;
pub const ABMON_3: C2RustUnnamed_5 = 131088;
pub const ABMON_2: C2RustUnnamed_5 = 131087;
pub const ABMON_1: C2RustUnnamed_5 = 131086;
pub const DAY_7: C2RustUnnamed_5 = 131085;
pub const DAY_6: C2RustUnnamed_5 = 131084;
pub const DAY_5: C2RustUnnamed_5 = 131083;
pub const DAY_4: C2RustUnnamed_5 = 131082;
pub const DAY_3: C2RustUnnamed_5 = 131081;
pub const DAY_2: C2RustUnnamed_5 = 131080;
pub const DAY_1: C2RustUnnamed_5 = 131079;
pub const ABDAY_7: C2RustUnnamed_5 = 131078;
pub const ABDAY_6: C2RustUnnamed_5 = 131077;
pub const ABDAY_5: C2RustUnnamed_5 = 131076;
pub const ABDAY_4: C2RustUnnamed_5 = 131075;
pub const ABDAY_3: C2RustUnnamed_5 = 131074;
pub const ABDAY_2: C2RustUnnamed_5 = 131073;
pub const ABDAY_1: C2RustUnnamed_5 = 131072;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isascii(mut c: i32) -> bool {
    match c {
        32 | 7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16
        | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127
        | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_iscntrl(mut c: i32) -> bool {
    match c {
        7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16 | 17
        | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127 => {
            return 1 as i32 != 0;
        }
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isupper(mut c: i32) -> bool {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as i32 != 0,
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
unsafe extern "C" fn c_toupper(mut c: i32) -> i32 {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 => {
            return c - 'a' as i32 + 'A' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn _unhex(mut c: u8) -> u8 {
    return (if c as i32 <= '9' as i32 {
        c as i32 - '0' as i32
    } else if c as i32 <= 'F' as i32 {
        c as i32 - 'A' as i32 + 10 as i32
    } else {
        c as i32 - 'a' as i32 + 10 as i32
    }) as u8;
}
static mut supported_schemes: [scheme_data; 5] = [
    {
        let mut init = scheme_data {
            name: b"http\0" as *const u8 as *const i8,
            leading_string: b"http://\0" as *const u8 as *const i8,
            default_port: 80 as i32,
            flags: C2RustUnnamed_6::scm_has_query as i32
                | C2RustUnnamed_6::scm_has_fragment as i32,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: b"https\0" as *const u8 as *const i8,
            leading_string: b"https://\0" as *const u8 as *const i8,
            default_port: 443 as i32,
            flags: C2RustUnnamed_6::scm_has_query as i32
                | C2RustUnnamed_6::scm_has_fragment as i32,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: b"ftp\0" as *const u8 as *const i8,
            leading_string: b"ftp://\0" as *const u8 as *const i8,
            default_port: 21 as i32,
            flags: C2RustUnnamed_6::scm_has_params as i32
                | C2RustUnnamed_6::scm_has_fragment as i32,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: b"ftps\0" as *const u8 as *const i8,
            leading_string: b"ftps://\0" as *const u8 as *const i8,
            default_port: 21 as i32,
            flags: C2RustUnnamed_6::scm_has_params as i32
                | C2RustUnnamed_6::scm_has_fragment as i32,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: 0 as *const i8,
            leading_string: 0 as *const i8,
            default_port: -(1 as i32),
            flags: 0 as i32,
        };
        init
    },
];
static mut urlchr_table: [u8; 256] = [
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    (C2RustUnnamed_7::urlchr_reserved as i32 | C2RustUnnamed_7::urlchr_unsafe as i32)
        as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
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
    (C2RustUnnamed_7::urlchr_reserved as i32 | C2RustUnnamed_7::urlchr_unsafe as i32)
        as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_reserved as i32 as u8,
    (C2RustUnnamed_7::urlchr_reserved as i32 | C2RustUnnamed_7::urlchr_unsafe as i32)
        as u8,
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
    (C2RustUnnamed_7::urlchr_reserved as i32 | C2RustUnnamed_7::urlchr_unsafe as i32)
        as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    (C2RustUnnamed_7::urlchr_reserved as i32 | C2RustUnnamed_7::urlchr_unsafe as i32)
        as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
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
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
    C2RustUnnamed_7::urlchr_unsafe as i32 as u8,
];
unsafe extern "C" fn url_unescape_1(mut s: *mut i8, mut mask: u8) {
    let mut current_block: u64;
    let mut t: *mut u8 = s as *mut u8;
    let mut h: *mut u8 = s as *mut u8;
    while *h != 0 {
        if *h as i32 != '%' as i32 {
            current_block = 18150253981776513775;
        } else {
            let mut c: u8 = 0;
            if *h.offset(1 as i32 as isize) == 0 || *h.offset(2 as i32 as isize) == 0
                || !(c_isxdigit(*h.offset(1 as i32 as isize) as i32) as i32 != 0
                    && c_isxdigit(*h.offset(2 as i32 as isize) as i32) as i32 != 0)
            {
                current_block = 18150253981776513775;
            } else {
                c = (((_unhex(*h.offset(1 as i32 as isize)) as i32) << 4 as i32)
                    + _unhex(*h.offset(2 as i32 as isize)) as i32) as u8;
                if urlchr_table[c as usize] as i32 & mask as i32 != 0 {
                    current_block = 18150253981776513775;
                } else if c as i32 == '\0' as i32 {
                    current_block = 18150253981776513775;
                } else {
                    *t = c;
                    h = h.offset(2 as i32 as isize);
                    current_block = 16668937799742929182;
                }
            }
        }
        match current_block {
            18150253981776513775 => {
                *t = *h;
            }
            _ => {}
        }
        h = h.offset(1);
        h;
        t = t.offset(1);
        t;
    }
    *t = '\0' as i32 as u8;
}
#[no_mangle]
pub unsafe extern "C" fn url_unescape(mut s: *mut i8) {
    url_unescape_1(s, 0 as i32 as u8);
}
#[no_mangle]
pub unsafe extern "C" fn url_unescape_except_reserved(mut s: *mut i8) {
    url_unescape_1(s, C2RustUnnamed_7::urlchr_reserved as i32 as u8);
}
unsafe extern "C" fn url_escape_1(
    mut s: *const i8,
    mut mask: u8,
    mut allow_passthrough: bool,
) -> *mut i8 {
    let mut p1: *const i8 = 0 as *const i8;
    let mut p2: *mut i8 = 0 as *mut i8;
    let mut newstr: *mut i8 = 0 as *mut i8;
    let mut newlen: i32 = 0;
    let mut addition: i32 = 0 as i32;
    p1 = s;
    while *p1 != 0 {
        if urlchr_table[*p1 as u8 as usize] as i32 & mask as i32 != 0 {
            addition += 2 as i32;
        }
        p1 = p1.offset(1);
        p1;
    }
    if addition == 0 {
        return if allow_passthrough as i32 != 0 { s as *mut i8 } else { xstrdup(s) };
    }
    newlen = (p1.offset_from(s) as i64 + addition as i64) as i32;
    newstr = xmalloc((newlen + 1 as i32) as size_t) as *mut i8;
    p1 = s;
    p2 = newstr;
    while *p1 != 0 {
        if urlchr_table[*p1 as u8 as usize] as i32 & mask as i32 != 0 {
            let fresh0 = p1;
            p1 = p1.offset(1);
            let mut c: u8 = *fresh0 as u8;
            let fresh1 = p2;
            p2 = p2.offset(1);
            *fresh1 = '%' as i32 as i8;
            let fresh2 = p2;
            p2 = p2.offset(1);
            *fresh2 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[i8; 17],
            >(b"0123456789ABCDEF\0"))[(c as i32 >> 4 as i32) as usize] as i32 + 0 as i32)
                as i8;
            let fresh3 = p2;
            p2 = p2.offset(1);
            *fresh3 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[i8; 17],
            >(b"0123456789ABCDEF\0"))[(c as i32 & 0xf as i32) as usize] as i32
                + 0 as i32) as i8;
        } else {
            let fresh4 = p1;
            p1 = p1.offset(1);
            let fresh5 = p2;
            p2 = p2.offset(1);
            *fresh5 = *fresh4;
        }
    }
    *p2 = '\0' as i32 as i8;
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn url_escape(mut s: *const i8) -> *mut i8 {
    return url_escape_1(s, C2RustUnnamed_7::urlchr_unsafe as i32 as u8, 0 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn url_escape_unsafe_and_reserved(mut s: *const i8) -> *mut i8 {
    return url_escape_1(
        s,
        (C2RustUnnamed_7::urlchr_unsafe as i32 | C2RustUnnamed_7::urlchr_reserved as i32)
            as u8,
        0 as i32 != 0,
    );
}
unsafe extern "C" fn url_escape_allow_passthrough(mut s: *const i8) -> *mut i8 {
    return url_escape_1(s, C2RustUnnamed_7::urlchr_unsafe as i32 as u8, 1 as i32 != 0);
}
#[inline]
unsafe extern "C" fn char_needs_escaping(mut p: *const i8) -> bool {
    if *p as i32 == '%' as i32 {
        if c_isxdigit(*p.offset(1 as i32 as isize) as i32) as i32 != 0
            && c_isxdigit(*p.offset(2 as i32 as isize) as i32) as i32 != 0
        {
            return 0 as i32 != 0
        } else {
            return 1 as i32 != 0
        }
    } else if urlchr_table[*p as u8 as usize] as i32
        & C2RustUnnamed_7::urlchr_unsafe as i32 != 0
        && urlchr_table[*p as u8 as usize] as i32
            & C2RustUnnamed_7::urlchr_reserved as i32 == 0
    {
        return 1 as i32 != 0
    } else {
        return 0 as i32 != 0
    };
}
unsafe extern "C" fn reencode_escapes(mut s: *const i8) -> *mut i8 {
    let mut p1: *const i8 = 0 as *const i8;
    let mut newstr: *mut i8 = 0 as *mut i8;
    let mut p2: *mut i8 = 0 as *mut i8;
    let mut oldlen: i32 = 0;
    let mut newlen: i32 = 0;
    let mut encode_count: i32 = 0 as i32;
    p1 = s;
    while *p1 != 0 {
        if char_needs_escaping(p1) {
            encode_count += 1;
            encode_count;
        }
        p1 = p1.offset(1);
        p1;
    }
    if encode_count == 0 {
        return s as *mut i8;
    }
    oldlen = p1.offset_from(s) as i64 as i32;
    newlen = oldlen + 2 as i32 * encode_count;
    newstr = xmalloc((newlen + 1 as i32) as size_t) as *mut i8;
    p1 = s;
    p2 = newstr;
    while *p1 != 0 {
        if char_needs_escaping(p1) {
            let fresh6 = p1;
            p1 = p1.offset(1);
            let mut c: u8 = *fresh6 as u8;
            let fresh7 = p2;
            p2 = p2.offset(1);
            *fresh7 = '%' as i32 as i8;
            let fresh8 = p2;
            p2 = p2.offset(1);
            *fresh8 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[i8; 17],
            >(b"0123456789ABCDEF\0"))[(c as i32 >> 4 as i32) as usize] as i32 + 0 as i32)
                as i8;
            let fresh9 = p2;
            p2 = p2.offset(1);
            *fresh9 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[i8; 17],
            >(b"0123456789ABCDEF\0"))[(c as i32 & 0xf as i32) as usize] as i32
                + 0 as i32) as i8;
        } else {
            let fresh10 = p1;
            p1 = p1.offset(1);
            let fresh11 = p2;
            p2 = p2.offset(1);
            *fresh11 = *fresh10;
        }
    }
    *p2 = '\0' as i32 as i8;
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn url_scheme(mut url: *const i8) -> url_scheme {
    let mut i: i32 = 0;
    i = 0 as i32;
    while !(supported_schemes[i as usize].leading_string).is_null() {
        if 0 as i32
            == c_strncasecmp(
                url,
                supported_schemes[i as usize].leading_string,
                strlen(supported_schemes[i as usize].leading_string),
            )
        {
            if supported_schemes[i as usize].flags & C2RustUnnamed_6::scm_disabled as i32
                == 0
            {
                return url_scheme::from_libc_c_uint(i as u32)
            } else {
                return url_scheme::SCHEME_INVALID
            }
        }
        i += 1;
        i;
    }
    return url_scheme::SCHEME_INVALID;
}
#[no_mangle]
pub unsafe extern "C" fn url_has_scheme(mut url: *const i8) -> bool {
    let mut p: *const i8 = url;
    if *p == 0
        || !(c_isalnum(*p as i32) as i32 != 0 || *p as i32 == '-' as i32
            || *p as i32 == '+' as i32)
    {
        return 0 as i32 != 0;
    }
    p = p.offset(1);
    p;
    while *p as i32 != 0
        && (c_isalnum(*p as i32) as i32 != 0 || *p as i32 == '-' as i32
            || *p as i32 == '+' as i32)
    {
        p = p.offset(1);
        p;
    }
    return *p as i32 == ':' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn url_valid_scheme(mut url: *const i8) -> bool {
    let mut scheme: url_scheme = url_scheme(url);
    return scheme as u32 != url_scheme::SCHEME_INVALID as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn scheme_default_port(mut scheme: url_scheme) -> i32 {
    return supported_schemes[scheme as usize].default_port;
}
#[no_mangle]
pub unsafe extern "C" fn scheme_disable(mut scheme: url_scheme) {
    supported_schemes[scheme as usize].flags |= C2RustUnnamed_6::scm_disabled as i32;
}
#[no_mangle]
pub unsafe extern "C" fn scheme_leading_string(mut scheme: url_scheme) -> *const i8 {
    return supported_schemes[scheme as usize].leading_string;
}
unsafe extern "C" fn url_skip_credentials(mut url: *const i8) -> *const i8 {
    let mut p: *const i8 = strpbrk(url, b"@/?#;\0" as *const u8 as *const i8)
        as *const i8;
    if p.is_null() || *p as i32 != '@' as i32 {
        return url;
    }
    return p.offset(1 as i32 as isize);
}
unsafe extern "C" fn parse_credentials(
    mut beg: *const i8,
    mut end: *const i8,
    mut user: *mut *mut i8,
    mut passwd: *mut *mut i8,
) -> bool {
    let mut colon: *mut i8 = 0 as *mut i8;
    let mut userend: *const i8 = 0 as *const i8;
    if beg == end {
        return 0 as i32 != 0;
    }
    colon = memchr(
        beg as *const libc::c_void,
        ':' as i32,
        end.offset_from(beg) as i64 as u64,
    ) as *mut i8;
    if colon == beg as *mut i8 {
        return 0 as i32 != 0;
    }
    if !colon.is_null() {
        *passwd = strdupdelim(colon.offset(1 as i32 as isize), end);
        userend = colon;
        url_unescape(*passwd);
    } else {
        *passwd = 0 as *mut i8;
        userend = end;
    }
    *user = strdupdelim(beg, userend);
    url_unescape(*user);
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn rewrite_shorthand_url(mut url: *const i8) -> *mut i8 {
    let mut p: *const i8 = 0 as *const i8;
    let mut ret: *mut i8 = 0 as *mut i8;
    if url_scheme(url) as u32 != url_scheme::SCHEME_INVALID as i32 as u32 {
        return 0 as *mut i8;
    }
    p = strpbrk(url, b":/\0" as *const u8 as *const i8);
    if p == url {
        return 0 as *mut i8;
    }
    if !p.is_null() && *p.offset(0 as i32 as isize) as i32 == ':' as i32
        && *p.offset(1 as i32 as isize) as i32 == '/' as i32
        && *p.offset(2 as i32 as isize) as i32 == '/' as i32
    {
        return 0 as *mut i8;
    }
    let mut current_block_9: u64;
    if !p.is_null() && *p as i32 == ':' as i32 {
        let mut digits: i32 = strspn(
            p.offset(1 as i32 as isize),
            b"0123456789\0" as *const u8 as *const i8,
        ) as i32;
        if digits != 0
            && (*p.offset((1 as i32 + digits) as isize) as i32 == '/' as i32
                || *p.offset((1 as i32 + digits) as isize) as i32 == '\0' as i32)
        {
            current_block_9 = 9810375517332381765;
        } else {
            ret = aprintf(b"ftp://%s\0" as *const u8 as *const i8, url);
            if !ret.is_null() {
                *ret.offset((6 as i32 as i64 + p.offset_from(url) as i64) as isize) = '/'
                    as i32 as i8;
            }
            current_block_9 = 7746791466490516765;
        }
    } else {
        current_block_9 = 9810375517332381765;
    }
    match current_block_9 {
        9810375517332381765 => {
            ret = aprintf(b"http://%s\0" as *const u8 as *const i8, url);
        }
        _ => {}
    }
    return ret;
}
#[inline]
unsafe extern "C" fn strpbrk_or_eos(mut s: *const i8, mut accept: *const i8) -> *mut i8 {
    let mut p: *mut i8 = strpbrk(s, accept);
    if p.is_null() {
        p = strchr(s, '\0' as i32);
    }
    return p;
}
unsafe extern "C" fn lowercase_str(mut str: *mut i8) -> bool {
    let mut changed: bool = 0 as i32 != 0;
    while *str != 0 {
        if c_isupper(*str as i32) {
            changed = 1 as i32 != 0;
            *str = c_tolower(*str as i32) as i8;
        }
        str = str.offset(1);
        str;
    }
    return changed;
}
unsafe extern "C" fn init_seps(mut scheme: url_scheme) -> *const i8 {
    static mut seps: [i8; 8] = unsafe {
        *::core::mem::transmute::<&[u8; 8], &mut [i8; 8]>(b":/\0\0\0\0\0\0")
    };
    let mut p: *mut i8 = seps.as_mut_ptr().offset(2 as i32 as isize);
    let mut flags: i32 = supported_schemes[scheme as usize].flags;
    if flags & C2RustUnnamed_6::scm_has_params as i32 != 0 {
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = ';' as i32 as i8;
    }
    if flags & C2RustUnnamed_6::scm_has_query as i32 != 0 {
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = '?' as i32 as i8;
    }
    if flags & C2RustUnnamed_6::scm_has_fragment as i32 != 0 {
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = '#' as i32 as i8;
    }
    *p = '\0' as i32 as i8;
    return seps.as_mut_ptr();
}
static mut parse_errors: [*const i8; 11] = [
    b"No error\0" as *const u8 as *const i8,
    b"Unsupported scheme\0" as *const u8 as *const i8,
    b"HTTPS support not compiled in\0" as *const u8 as *const i8,
    b"FTPS support not compiled in\0" as *const u8 as *const i8,
    b"Scheme missing\0" as *const u8 as *const i8,
    b"Invalid host name\0" as *const u8 as *const i8,
    b"Bad port number\0" as *const u8 as *const i8,
    b"Invalid user name\0" as *const u8 as *const i8,
    b"Unterminated IPv6 numeric address\0" as *const u8 as *const i8,
    b"IPv6 addresses not supported\0" as *const u8 as *const i8,
    b"Invalid IPv6 numeric address\0" as *const u8 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn url_parse(
    mut url: *const i8,
    mut error: *mut i32,
    mut iri: *mut iri,
    mut percent_encode: bool,
) -> *mut url {
    let mut current_block: u64;
    let mut u: *mut url = 0 as *mut url;
    let mut p: *const i8 = 0 as *const i8;
    let mut path_modified: bool = false;
    let mut host_modified: bool = false;
    let mut scheme: url_scheme = url_scheme::SCHEME_HTTP;
    let mut seps: *const i8 = 0 as *const i8;
    let mut uname_b: *const i8 = 0 as *const i8;
    let mut uname_e: *const i8 = 0 as *const i8;
    let mut host_b: *const i8 = 0 as *const i8;
    let mut host_e: *const i8 = 0 as *const i8;
    let mut path_b: *const i8 = 0 as *const i8;
    let mut path_e: *const i8 = 0 as *const i8;
    let mut params_b: *const i8 = 0 as *const i8;
    let mut params_e: *const i8 = 0 as *const i8;
    let mut query_b: *const i8 = 0 as *const i8;
    let mut query_e: *const i8 = 0 as *const i8;
    let mut fragment_b: *const i8 = 0 as *const i8;
    let mut fragment_e: *const i8 = 0 as *const i8;
    let mut port: i32 = 0;
    let mut user: *mut i8 = 0 as *mut i8;
    let mut passwd: *mut i8 = 0 as *mut i8;
    let mut url_encoded: *const i8 = 0 as *const i8;
    let mut error_code: i32 = 0;
    scheme = url_scheme(url);
    if scheme as u32 == url_scheme::SCHEME_INVALID as i32 as u32 {
        if !url_has_scheme(url) {
            error_code = C2RustUnnamed_8::PE_MISSING_SCHEME as i32;
        } else if c_strncasecmp(
            url,
            b"https:\0" as *const u8 as *const i8,
            6 as i32 as size_t,
        ) == 0
        {
            error_code = C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_HTTPS as i32;
        } else if c_strncasecmp(
            url,
            b"ftps:\0" as *const u8 as *const i8,
            5 as i32 as size_t,
        ) == 0
        {
            error_code = C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_FTPS as i32;
        } else {
            error_code = C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME as i32;
        }
    } else {
        url_encoded = url;
        if !iri.is_null() && (*iri).utf8_encode as i32 != 0 {
            let mut new_url: *mut i8 = 0 as *mut i8;
            (*iri).utf8_encode = remote_to_utf8(
                iri,
                if !((*iri).orig_url).is_null() { (*iri).orig_url } else { url },
                &mut new_url,
            );
            if !(*iri).utf8_encode {
                new_url = 0 as *mut i8;
            } else {
                rpl_free((*iri).orig_url as *mut libc::c_void);
                (*iri).orig_url = 0 as *mut i8;
                (*iri).orig_url = xstrdup(url);
                url_encoded = reencode_escapes(new_url);
                if url_encoded != new_url {
                    rpl_free(new_url as *mut libc::c_void);
                    new_url = 0 as *mut i8;
                }
                percent_encode = 0 as i32 != 0;
            }
        }
        if percent_encode {
            url_encoded = reencode_escapes(url);
        }
        p = url_encoded;
        p = p.offset(strlen(supported_schemes[scheme as usize].leading_string) as isize);
        uname_b = p;
        p = url_skip_credentials(p);
        uname_e = p;
        path_e = 0 as *const i8;
        path_b = path_e;
        params_e = 0 as *const i8;
        params_b = params_e;
        query_e = 0 as *const i8;
        query_b = query_e;
        fragment_e = 0 as *const i8;
        fragment_b = fragment_e;
        seps = init_seps(scheme);
        host_b = p;
        if *p as i32 == '[' as i32 {
            host_b = p.offset(1 as i32 as isize);
            host_e = strchr(host_b, ']' as i32);
            if host_e.is_null() {
                error_code = C2RustUnnamed_8::PE_UNTERMINATED_IPV6_ADDRESS as i32;
                current_block = 11473658817087337075;
            } else if !is_valid_ipv6_address(host_b, host_e) {
                error_code = C2RustUnnamed_8::PE_INVALID_IPV6_ADDRESS as i32;
                current_block = 11473658817087337075;
            } else {
                p = host_e.offset(1 as i32 as isize);
                if (strchr(seps, *p as i32)).is_null() {
                    error_code = C2RustUnnamed_8::PE_INVALID_HOST_NAME as i32;
                    current_block = 11473658817087337075;
                } else {
                    current_block = 1924505913685386279;
                }
            }
        } else {
            p = strpbrk_or_eos(p, seps);
            host_e = p;
            current_block = 1924505913685386279;
        }
        match current_block {
            11473658817087337075 => {}
            _ => {
                seps = seps.offset(1);
                seps;
                if host_b == host_e {
                    error_code = C2RustUnnamed_8::PE_INVALID_HOST_NAME as i32;
                } else {
                    port = scheme_default_port(scheme);
                    if *p as i32 == ':' as i32 {
                        let mut port_b: *const i8 = 0 as *const i8;
                        let mut port_e: *const i8 = 0 as *const i8;
                        let mut pp: *const i8 = 0 as *const i8;
                        p = p.offset(1);
                        p;
                        port_b = p;
                        p = strpbrk_or_eos(p, seps);
                        port_e = p;
                        if port_b != port_e {
                            port = 0 as i32;
                            pp = port_b;
                            loop {
                                if !(pp < port_e) {
                                    current_block = 3580086814630675314;
                                    break;
                                }
                                if !c_isdigit(*pp as i32) {
                                    error_code = C2RustUnnamed_8::PE_BAD_PORT_NUMBER as i32;
                                    current_block = 11473658817087337075;
                                    break;
                                } else {
                                    port = 10 as i32 * port + (*pp as i32 - '0' as i32);
                                    if port > 0xffff as i32 {
                                        error_code = C2RustUnnamed_8::PE_BAD_PORT_NUMBER as i32;
                                        current_block = 11473658817087337075;
                                        break;
                                    } else {
                                        pp = pp.offset(1);
                                        pp;
                                    }
                                }
                            }
                        } else {
                            current_block = 3580086814630675314;
                        }
                    } else {
                        current_block = 3580086814630675314;
                    }
                    match current_block {
                        11473658817087337075 => {}
                        _ => {
                            seps = seps.offset(1);
                            seps;
                            if *p as i32 == '/' as i32 {
                                p = p.offset(1);
                                path_b = p;
                                p = strpbrk_or_eos(p, seps);
                                path_e = p;
                            }
                            seps = seps.offset(1);
                            seps;
                            if supported_schemes[scheme as usize].flags
                                & C2RustUnnamed_6::scm_has_params as i32 != 0
                            {
                                if *p as i32 == ';' as i32 {
                                    p = p.offset(1);
                                    params_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    params_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if supported_schemes[scheme as usize].flags
                                & C2RustUnnamed_6::scm_has_query as i32 != 0
                            {
                                if *p as i32 == '?' as i32 {
                                    p = p.offset(1);
                                    query_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    query_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if supported_schemes[scheme as usize].flags
                                & C2RustUnnamed_6::scm_has_fragment as i32 != 0
                            {
                                if *p as i32 == '#' as i32 {
                                    p = p.offset(1);
                                    fragment_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    fragment_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if uname_b != uname_e {
                                if !parse_credentials(
                                    uname_b,
                                    uname_e.offset(-(1 as i32 as isize)),
                                    &mut user,
                                    &mut passwd,
                                ) {
                                    error_code = C2RustUnnamed_8::PE_INVALID_USER_NAME as i32;
                                    current_block = 11473658817087337075;
                                } else {
                                    current_block = 200744462051969938;
                                }
                            } else {
                                current_block = 200744462051969938;
                            }
                            match current_block {
                                11473658817087337075 => {}
                                _ => {
                                    u = xcalloc(
                                        1 as i32 as size_t,
                                        ::core::mem::size_of::<url>() as u64,
                                    ) as *mut url;
                                    (*u).scheme = scheme;
                                    (*u).host = strdupdelim(host_b, host_e);
                                    (*u).port = port;
                                    (*u).user = user;
                                    (*u).passwd = passwd;
                                    (*u).path = strdupdelim(path_b, path_e);
                                    path_modified = path_simplify(scheme, (*u).path);
                                    split_path((*u).path, &mut (*u).dir, &mut (*u).file);
                                    host_modified = lowercase_str((*u).host);
                                    if !(strchr((*u).host, '%' as i32)).is_null() {
                                        url_unescape((*u).host);
                                        host_modified = 1 as i32 != 0;
                                        p = (*u).host;
                                        loop {
                                            if !(*p != 0) {
                                                current_block = 5700653730392116747;
                                                break;
                                            }
                                            if c_iscntrl(*p as i32) {
                                                url_free(u);
                                                error_code = C2RustUnnamed_8::PE_INVALID_HOST_NAME as i32;
                                                current_block = 11473658817087337075;
                                                break;
                                            } else {
                                                p = p.offset(1);
                                                p;
                                            }
                                        }
                                        match current_block {
                                            11473658817087337075 => {}
                                            _ => {
                                                if opt.enable_iri as i32 != 0 && !iri.is_null() {
                                                    let mut new: *mut i8 = idn_encode(iri, (*u).host);
                                                    if !new.is_null() {
                                                        rpl_free((*u).host as *mut libc::c_void);
                                                        (*u).host = 0 as *mut i8;
                                                        (*u).host = new;
                                                        host_modified = 1 as i32 != 0;
                                                    }
                                                }
                                                current_block = 9775647934248138666;
                                            }
                                        }
                                    } else {
                                        current_block = 9775647934248138666;
                                    }
                                    match current_block {
                                        11473658817087337075 => {}
                                        _ => {
                                            if !params_b.is_null() {
                                                (*u).params = strdupdelim(params_b, params_e);
                                            }
                                            if !query_b.is_null() {
                                                (*u).query = strdupdelim(query_b, query_e);
                                            }
                                            if !fragment_b.is_null() {
                                                (*u).fragment = strdupdelim(fragment_b, fragment_e);
                                            }
                                            if opt.enable_iri as i32 != 0 || path_modified as i32 != 0
                                                || !((*u).fragment).is_null() || host_modified as i32 != 0
                                                || path_b == path_e
                                            {
                                                (*u).url = url_string(u, url_auth_mode::URL_AUTH_SHOW);
                                                if url_encoded != url {
                                                    rpl_free(url_encoded as *mut libc::c_void);
                                                    url_encoded = 0 as *const i8;
                                                }
                                            } else if url_encoded == url {
                                                (*u).url = xstrdup(url);
                                            } else {
                                                (*u).url = url_encoded as *mut i8;
                                            }
                                            return u;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !url_encoded.is_null() && url_encoded != url {
        rpl_free(url_encoded as *mut libc::c_void);
        url_encoded = 0 as *const i8;
    }
    if !error.is_null() {
        *error = error_code;
    }
    return 0 as *mut url;
}
#[no_mangle]
pub unsafe extern "C" fn url_error(mut error_code: i32) -> *const i8 {
    if error_code >= 0 as i32
        && error_code
            < (::core::mem::size_of::<[*const i8; 11]>() as u64)
                .wrapping_div(::core::mem::size_of::<*const i8>() as u64) as i32
    {
        return dcgettext(0 as *const i8, parse_errors[error_code as usize], 5 as i32);
    }
    return b"\0" as *const u8 as *const i8;
}
unsafe extern "C" fn split_path(
    mut path: *const i8,
    mut dir: *mut *mut i8,
    mut file: *mut *mut i8,
) {
    let mut last_slash: *mut i8 = strrchr(path, '/' as i32);
    if last_slash.is_null() {
        *dir = xstrdup(b"\0" as *const u8 as *const i8);
        *file = xstrdup(path);
    } else {
        *dir = strdupdelim(path, last_slash);
        *file = xstrdup(last_slash.offset(1 as i32 as isize));
    }
    url_unescape(*dir);
    url_unescape(*file);
}
unsafe extern "C" fn full_path_length(mut url: *const url) -> i32 {
    let mut len: i32 = 0 as i32;
    if !((*url).path).is_null() {
        len = (len as u64)
            .wrapping_add((1 as i32 as u64).wrapping_add(strlen((*url).path))) as i32
            as i32;
    }
    if !((*url).params).is_null() {
        len = (len as u64)
            .wrapping_add((1 as i32 as u64).wrapping_add(strlen((*url).params))) as i32
            as i32;
    }
    if !((*url).query).is_null() {
        len = (len as u64)
            .wrapping_add((1 as i32 as u64).wrapping_add(strlen((*url).query))) as i32
            as i32;
    }
    return len;
}
unsafe extern "C" fn full_path_write(mut url: *const url, mut where_0: *mut i8) {
    let mut f_el: *mut i8 = (*url).path;
    if !f_el.is_null() {
        let mut l: i32 = strlen(f_el) as i32;
        let fresh15 = where_0;
        where_0 = where_0.offset(1);
        *fresh15 = '/' as i32 as i8;
        memcpy(where_0 as *mut libc::c_void, f_el as *const libc::c_void, l as u64);
        where_0 = where_0.offset(l as isize);
    }
    let mut f_el_0: *mut i8 = (*url).params;
    if !f_el_0.is_null() {
        let mut l_0: i32 = strlen(f_el_0) as i32;
        let fresh16 = where_0;
        where_0 = where_0.offset(1);
        *fresh16 = ';' as i32 as i8;
        memcpy(where_0 as *mut libc::c_void, f_el_0 as *const libc::c_void, l_0 as u64);
        where_0 = where_0.offset(l_0 as isize);
    }
    let mut f_el_1: *mut i8 = (*url).query;
    if !f_el_1.is_null() {
        let mut l_1: i32 = strlen(f_el_1) as i32;
        let fresh17 = where_0;
        where_0 = where_0.offset(1);
        *fresh17 = '?' as i32 as i8;
        memcpy(where_0 as *mut libc::c_void, f_el_1 as *const libc::c_void, l_1 as u64);
        where_0 = where_0.offset(l_1 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn url_full_path(mut url: *const url) -> *mut i8 {
    let mut length: i32 = full_path_length(url);
    let mut full_path: *mut i8 = xmalloc((length + 1 as i32) as size_t) as *mut i8;
    full_path_write(url, full_path);
    *full_path.offset(length as isize) = '\0' as i32 as i8;
    return full_path;
}
unsafe extern "C" fn unescape_single_char(mut str: *mut i8, mut chr: i8) {
    let c1: i8 = ((*::core::mem::transmute::<
        &[u8; 17],
        &[i8; 17],
    >(b"0123456789ABCDEF\0"))[(chr as i32 >> 4 as i32) as usize] as i32 + 0 as i32)
        as i8;
    let c2: i8 = ((*::core::mem::transmute::<
        &[u8; 17],
        &[i8; 17],
    >(b"0123456789ABCDEF\0"))[(chr as i32 & 0xf as i32) as usize] as i32 + 0 as i32)
        as i8;
    let mut h: *mut i8 = str;
    let mut t: *mut i8 = str;
    while *h != 0 {
        if *h.offset(0 as i32 as isize) as i32 == '%' as i32
            && *h.offset(1 as i32 as isize) as i32 == c1 as i32
            && *h.offset(2 as i32 as isize) as i32 == c2 as i32
        {
            *t = chr;
            h = h.offset(2 as i32 as isize);
        } else {
            *t = *h;
        }
        h = h.offset(1);
        h;
        t = t.offset(1);
        t;
    }
    *t = '\0' as i32 as i8;
}
unsafe extern "C" fn url_escape_dir(mut dir: *const i8) -> *mut i8 {
    let mut newdir: *mut i8 = url_escape_1(
        dir,
        (C2RustUnnamed_7::urlchr_unsafe as i32 | C2RustUnnamed_7::urlchr_reserved as i32)
            as u8,
        1 as i32 != 0,
    );
    if newdir == dir as *mut i8 {
        return dir as *mut i8;
    }
    unescape_single_char(newdir, '/' as i32 as i8);
    return newdir;
}
unsafe extern "C" fn sync_path(mut u: *mut url) {
    let mut newpath: *mut i8 = 0 as *mut i8;
    let mut efile: *mut i8 = 0 as *mut i8;
    let mut edir: *mut i8 = 0 as *mut i8;
    rpl_free((*u).path as *mut libc::c_void);
    (*u).path = 0 as *mut i8;
    edir = url_escape_dir((*u).dir);
    efile = url_escape_1(
        (*u).file,
        (C2RustUnnamed_7::urlchr_unsafe as i32 | C2RustUnnamed_7::urlchr_reserved as i32)
            as u8,
        1 as i32 != 0,
    );
    if *edir == 0 {
        newpath = xstrdup(efile);
    } else {
        let mut dirlen: i32 = strlen(edir) as i32;
        let mut filelen: i32 = strlen(efile) as i32;
        newpath = xmalloc((dirlen + 1 as i32 + filelen + 1 as i32) as size_t) as *mut i8;
        let mut p: *mut i8 = newpath;
        memcpy(p as *mut libc::c_void, edir as *const libc::c_void, dirlen as u64);
        p = p.offset(dirlen as isize);
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = '/' as i32 as i8;
        memcpy(p as *mut libc::c_void, efile as *const libc::c_void, filelen as u64);
        p = p.offset(filelen as isize);
        *p = '\0' as i32 as i8;
    }
    (*u).path = newpath;
    if edir != (*u).dir {
        rpl_free(edir as *mut libc::c_void);
        edir = 0 as *mut i8;
    }
    if efile != (*u).file {
        rpl_free(efile as *mut libc::c_void);
        efile = 0 as *mut i8;
    }
    rpl_free((*u).url as *mut libc::c_void);
    (*u).url = 0 as *mut i8;
    (*u).url = url_string(u, url_auth_mode::URL_AUTH_SHOW);
}
#[no_mangle]
pub unsafe extern "C" fn url_set_dir(mut url: *mut url, mut newdir: *const i8) {
    rpl_free((*url).dir as *mut libc::c_void);
    (*url).dir = 0 as *mut i8;
    (*url).dir = xstrdup(newdir);
    sync_path(url);
}
#[no_mangle]
pub unsafe extern "C" fn url_set_file(mut url: *mut url, mut newfile: *const i8) {
    rpl_free((*url).file as *mut libc::c_void);
    (*url).file = 0 as *mut i8;
    (*url).file = xstrdup(newfile);
    sync_path(url);
}
#[no_mangle]
pub unsafe extern "C" fn url_free(mut url: *mut url) {
    if !url.is_null() {
        rpl_free((*url).host as *mut libc::c_void);
        (*url).host = 0 as *mut i8;
        rpl_free((*url).path as *mut libc::c_void);
        (*url).path = 0 as *mut i8;
        rpl_free((*url).url as *mut libc::c_void);
        (*url).url = 0 as *mut i8;
        rpl_free((*url).params as *mut libc::c_void);
        (*url).params = 0 as *mut i8;
        rpl_free((*url).query as *mut libc::c_void);
        (*url).query = 0 as *mut i8;
        rpl_free((*url).fragment as *mut libc::c_void);
        (*url).fragment = 0 as *mut i8;
        rpl_free((*url).user as *mut libc::c_void);
        (*url).user = 0 as *mut i8;
        rpl_free((*url).passwd as *mut libc::c_void);
        (*url).passwd = 0 as *mut i8;
        rpl_free((*url).dir as *mut libc::c_void);
        (*url).dir = 0 as *mut i8;
        rpl_free((*url).file as *mut libc::c_void);
        (*url).file = 0 as *mut i8;
        rpl_free(url as *mut libc::c_void);
        url = 0 as *mut url;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mkalldirs(mut path: *const i8) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut t: *mut i8 = 0 as *mut i8;
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
    let mut res: i32 = 0;
    p = strrchr(path, '/' as i32);
    p = if p.is_null() { path } else { p };
    if p == path && *p as i32 != '/' as i32 {
        return 0 as i32;
    }
    t = strdupdelim(path, p);
    if stat(t, &mut st) == 0 as i32 {
        if st.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
            rpl_free(t as *mut libc::c_void);
            t = 0 as *mut i8;
            return 0 as i32;
        } else {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Removing %s because of directory danger!\n\0" as *const u8
                        as *const i8,
                    t,
                );
            }
            if unlink(t) != 0 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"Failed to unlink %s (%d): %s\n\0" as *const u8 as *const i8,
                    t,
                    *__errno_location(),
                    strerror(*__errno_location()),
                );
            }
        }
    }
    res = make_directory(t);
    if res != 0 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const i8,
            t,
            strerror(*__errno_location()),
        );
    }
    rpl_free(t as *mut libc::c_void);
    t = 0 as *mut i8;
    return res;
}
unsafe extern "C" fn append_null(mut dest: *mut growable) {
    let mut G_: *mut growable = dest;
    let mut DR_needed_size: i64 = ((*G_).tail + 1 as i32) as i64;
    let mut DR_newsize: i64 = 0 as i32 as i64;
    while ((*G_).size as i64) < DR_needed_size {
        DR_newsize = ((*G_).size << 1 as i32) as i64;
        if DR_newsize < 16 as i32 as i64 {
            DR_newsize = 16 as i32 as i64;
        }
        (*G_).size = DR_newsize as i32;
    }
    if DR_newsize != 0 {
        (*G_).base = xrealloc(
            (*G_).base as *mut libc::c_void,
            (DR_newsize as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ) as *mut i8;
    }
    *((*dest).base).offset((*dest).tail as isize) = 0 as i32 as i8;
}
unsafe extern "C" fn append_char(mut ch: i8, mut dest: *mut growable) {
    if ch != 0 {
        let mut G_: *mut growable = dest;
        let mut DR_needed_size: i64 = ((*G_).tail + 1 as i32) as i64;
        let mut DR_newsize: i64 = 0 as i32 as i64;
        while ((*G_).size as i64) < DR_needed_size {
            DR_newsize = ((*G_).size << 1 as i32) as i64;
            if DR_newsize < 16 as i32 as i64 {
                DR_newsize = 16 as i32 as i64;
            }
            (*G_).size = DR_newsize as i32;
        }
        if DR_newsize != 0 {
            (*G_).base = xrealloc(
                (*G_).base as *mut libc::c_void,
                (DR_newsize as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            ) as *mut i8;
        }
        *((*dest).base).offset((*dest).tail as isize) = ch;
        (*dest).tail += 1 as i32;
    }
    append_null(dest);
}
unsafe extern "C" fn append_string(mut str: *const i8, mut dest: *mut growable) {
    let mut l: i32 = strlen(str) as i32;
    if l != 0 {
        let mut G_: *mut growable = dest;
        let mut DR_needed_size: i64 = ((*G_).tail + l) as i64;
        let mut DR_newsize: i64 = 0 as i32 as i64;
        while ((*G_).size as i64) < DR_needed_size {
            DR_newsize = ((*G_).size << 1 as i32) as i64;
            if DR_newsize < 16 as i32 as i64 {
                DR_newsize = 16 as i32 as i64;
            }
            (*G_).size = DR_newsize as i32;
        }
        if DR_newsize != 0 {
            (*G_).base = xrealloc(
                (*G_).base as *mut libc::c_void,
                (DR_newsize as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            ) as *mut i8;
        }
        memcpy(
            ((*dest).base).offset((*dest).tail as isize) as *mut libc::c_void,
            str as *const libc::c_void,
            l as u64,
        );
        (*dest).tail += l;
    }
    append_null(dest);
}
static mut filechr_table: [u8; 256] = [
    (C2RustUnnamed_9::filechr_not_unix as i32 | C2RustUnnamed_9::filechr_not_vms as i32
        | C2RustUnnamed_9::filechr_not_windows as i32
        | C2RustUnnamed_9::filechr_control as i32) as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32 | C2RustUnnamed_9::filechr_control as i32)
        as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_9::filechr_not_windows as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32
        | C2RustUnnamed_9::filechr_not_windows as i32) as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    (C2RustUnnamed_9::filechr_not_unix as i32
        | C2RustUnnamed_9::filechr_not_windows as i32) as u8,
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
    C2RustUnnamed_9::filechr_not_windows as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_9::filechr_not_windows as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_9::filechr_not_windows as i32 as u8,
    (C2RustUnnamed_9::filechr_not_vms as i32
        | C2RustUnnamed_9::filechr_not_windows as i32) as u8,
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
    C2RustUnnamed_9::filechr_not_windows as i32 as u8,
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
    C2RustUnnamed_9::filechr_not_windows as i32 as u8,
    0 as i32 as u8,
    0 as i32 as u8,
    C2RustUnnamed_9::filechr_control as i32 as u8,
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
];
unsafe extern "C" fn append_uri_pathel(
    mut b: *const i8,
    mut e: *const i8,
    mut escaped: bool,
    mut dest: *mut growable,
) {
    let mut p: *const i8 = 0 as *const i8;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut unescaped: *mut i8 = 0 as *mut i8;
    let mut quoted: i32 = 0;
    let mut outlen: i32 = 0;
    let mut mask: i32 = 0;
    let mut max_length: i32 = 0;
    if dest.is_null() {
        return;
    }
    if opt.restrict_files_os as u32 == C2RustUnnamed_1::restrict_unix as i32 as u32 {
        mask = C2RustUnnamed_9::filechr_not_unix as i32;
    } else if opt.restrict_files_os as u32 == C2RustUnnamed_1::restrict_vms as i32 as u32
    {
        mask = C2RustUnnamed_9::filechr_not_vms as i32;
    } else {
        mask = C2RustUnnamed_9::filechr_not_windows as i32;
    }
    if opt.restrict_files_ctrl {
        mask |= C2RustUnnamed_9::filechr_control as i32;
    }
    if escaped {
        let mut len: size_t = e.offset_from(b) as i64 as size_t;
        if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
            unescaped = buf.as_mut_ptr();
        } else {
            unescaped = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
        }
        memcpy(unescaped as *mut libc::c_void, b as *const libc::c_void, len);
        *unescaped.offset(len as isize) = 0 as i32 as i8;
        url_unescape(unescaped);
        b = unescaped;
        e = unescaped.offset(strlen(unescaped) as isize);
    }
    if e.offset_from(b) as i64 == 2 as i32 as i64
        && *b.offset(0 as i32 as isize) as i32 == '.' as i32
        && *b.offset(1 as i32 as isize) as i32 == '.' as i32
    {
        b = b"%2E%2E\0" as *const u8 as *const i8;
        e = b.offset(6 as i32 as isize);
    }
    quoted = 0 as i32;
    p = b;
    while p < e {
        if opt.restrict_files_nonascii as i32 != 0 && !c_isascii(*p as u8 as i32)
            || filechr_table[*p as u8 as usize] as i32 & mask != 0
        {
            quoted += 1;
            quoted;
        }
        p = p.offset(1);
        p;
    }
    outlen = (e.offset_from(b) as i64 + (2 as i32 * quoted) as i64) as i32;
    max_length = get_max_length(
        (*dest).base,
        (*dest).tail,
        C2RustUnnamed_4::_PC_NAME_MAX as i32,
    ) as i32;
    max_length -= 19 as i32;
    if max_length > 0 as i32 && outlen > max_length {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"The destination name is too long (%d), reducing to %d\n\0" as *const u8
                as *const i8,
            outlen,
            max_length,
        );
        outlen = max_length;
    }
    let mut G_: *mut growable = dest;
    let mut DR_needed_size: i64 = ((*G_).tail + outlen) as i64;
    let mut DR_newsize: i64 = 0 as i32 as i64;
    while ((*G_).size as i64) < DR_needed_size {
        DR_newsize = ((*G_).size << 1 as i32) as i64;
        if DR_newsize < 16 as i32 as i64 {
            DR_newsize = 16 as i32 as i64;
        }
        (*G_).size = DR_newsize as i32;
    }
    if DR_newsize != 0 {
        (*G_).base = xrealloc(
            (*G_).base as *mut libc::c_void,
            (DR_newsize as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ) as *mut i8;
    }
    if ((*dest).base).is_null() {
        return;
    }
    if quoted == 0 {
        memcpy(
            ((*dest).base).offset((*dest).tail as isize) as *mut libc::c_void,
            b as *const libc::c_void,
            outlen as u64,
        );
    } else {
        let mut q: *mut i8 = ((*dest).base).offset((*dest).tail as isize);
        let mut i: i32 = 0;
        i = 0 as i32;
        p = b;
        while p < e {
            if !(opt.restrict_files_nonascii as i32 != 0 && !c_isascii(*p as u8 as i32)
                || filechr_table[*p as u8 as usize] as i32 & mask != 0)
            {
                if i == outlen {
                    break;
                }
                let fresh19 = q;
                q = q.offset(1);
                *fresh19 = *p;
                i += 1;
                i;
            } else {
                if i + 3 as i32 > outlen {
                    break;
                }
                let mut ch: u8 = *p as u8;
                let fresh20 = q;
                q = q.offset(1);
                *fresh20 = '%' as i32 as i8;
                let fresh21 = q;
                q = q.offset(1);
                *fresh21 = ((*::core::mem::transmute::<
                    &[u8; 17],
                    &[i8; 17],
                >(b"0123456789ABCDEF\0"))[(ch as i32 >> 4 as i32) as usize] as i32
                    + 0 as i32) as i8;
                let fresh22 = q;
                q = q.offset(1);
                *fresh22 = ((*::core::mem::transmute::<
                    &[u8; 17],
                    &[i8; 17],
                >(b"0123456789ABCDEF\0"))[(ch as i32 & 0xf as i32) as usize] as i32
                    + 0 as i32) as i8;
                i += 3 as i32;
            }
            p = p.offset(1);
            p;
        }
    }
    if opt.restrict_files_case as u32
        == C2RustUnnamed_0::restrict_lowercase as i32 as u32
        || opt.restrict_files_case as u32
            == C2RustUnnamed_0::restrict_uppercase as i32 as u32
    {
        let mut q_0: *mut i8 = 0 as *mut i8;
        q_0 = ((*dest).base).offset((*dest).tail as isize);
        while q_0 < ((*dest).base).offset((*dest).tail as isize).offset(outlen as isize)
        {
            if opt.restrict_files_case as u32
                == C2RustUnnamed_0::restrict_lowercase as i32 as u32
            {
                *q_0 = c_tolower(*q_0 as i32) as i8;
            } else {
                *q_0 = c_toupper(*q_0 as i32) as i8;
            }
            q_0 = q_0.offset(1);
            q_0;
        }
    }
    (*dest).tail += outlen;
    append_null(dest);
    if !unescaped.is_null() && unescaped != buf.as_mut_ptr() {
        rpl_free(unescaped as *mut libc::c_void);
    }
}
unsafe extern "C" fn convert_fname(mut fname: *mut i8) -> *mut i8 {
    let mut converted_fname: *mut i8 = 0 as *mut i8;
    let mut from_encoding: *const i8 = opt.encoding_remote;
    let mut to_encoding: *const i8 = opt.locale;
    let mut cd: iconv_t = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut done: size_t = 0;
    let mut inlen: size_t = 0;
    let mut outlen: size_t = 0;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut orig_fname: *const i8 = 0 as *const i8;
    if from_encoding.is_null() {
        from_encoding = b"UTF-8\0" as *const u8 as *const i8;
    }
    if to_encoding.is_null() {
        to_encoding = nl_langinfo(CODESET as i32);
    }
    cd = iconv_open(to_encoding, from_encoding);
    if cd == -(1 as i32) as iconv_t {
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Conversion from %s to %s isn't supported\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            quote_n(0 as i32, from_encoding),
            quote_n(1 as i32, to_encoding),
        );
        return fname;
    }
    orig_fname = fname;
    inlen = strlen(fname);
    outlen = inlen.wrapping_mul(2 as i32 as u64);
    len = outlen;
    s = xmalloc(outlen.wrapping_add(1 as i32 as u64)) as *mut i8;
    converted_fname = s;
    done = 0 as i32 as size_t;
    loop {
        *__errno_location() = 0 as i32;
        if iconv(cd, &mut fname as *mut *mut i8, &mut inlen, &mut s, &mut outlen)
            == 0 as i32 as u64
            && iconv(cd, 0 as *mut *mut i8, 0 as *mut size_t, &mut s, &mut outlen)
                == 0 as i32 as u64
        {
            *converted_fname
                .offset(len as isize)
                .offset(-(outlen as isize))
                .offset(-(done as isize)) = '\0' as i32 as i8;
            iconv_close(cd);
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Converted file name '%s' (%s) -> '%s' (%s)\n\0" as *const u8
                        as *const i8,
                    orig_fname,
                    from_encoding,
                    converted_fname,
                    to_encoding,
                );
            }
            rpl_free(orig_fname as *mut libc::c_void);
            orig_fname = 0 as *const i8;
            return converted_fname;
        }
        if *__errno_location() == 22 as i32 || *__errno_location() == 84 as i32
            || *__errno_location() == 0 as i32
        {
            if *__errno_location() != 0 {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Incomplete or invalid multibyte sequence encountered\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            } else {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Unconvertable multibyte sequence encountered\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            rpl_free(converted_fname as *mut libc::c_void);
            converted_fname = 0 as *mut i8;
            converted_fname = orig_fname as *mut i8;
            break;
        } else if *__errno_location() == 7 as i32 {
            done = len;
            outlen = done.wrapping_add(inlen.wrapping_mul(2 as i32 as u64));
            len = outlen;
            converted_fname = xrealloc(
                converted_fname as *mut libc::c_void,
                outlen.wrapping_add(1 as i32 as u64),
            ) as *mut i8;
            s = converted_fname.offset(done as isize);
        } else {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Unhandled errno %d\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *__errno_location(),
            );
            rpl_free(converted_fname as *mut libc::c_void);
            converted_fname = 0 as *mut i8;
            converted_fname = orig_fname as *mut i8;
            break;
        }
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Failed to convert file name '%s' (%s) -> '?' (%s)\n\0" as *const u8
                as *const i8,
            orig_fname,
            from_encoding,
            to_encoding,
        );
    }
    iconv_close(cd);
    return converted_fname;
}
unsafe extern "C" fn append_dir_structure(mut u: *const url, mut dest: *mut growable) {
    let mut pathel: *mut i8 = 0 as *mut i8;
    let mut next: *mut i8 = 0 as *mut i8;
    let mut cut: i32 = opt.cut_dirs;
    pathel = (*u).path;
    loop {
        next = strchr(pathel, '/' as i32);
        if next.is_null() {
            break;
        }
        let fresh23 = cut;
        cut = cut - 1;
        if !(fresh23 > 0 as i32) {
            if !(pathel == next) {
                if (*dest).tail != 0 {
                    append_char('/' as i32 as i8, dest);
                }
                append_uri_pathel(pathel, next, 1 as i32 != 0, dest);
            }
        }
        pathel = next.offset(1 as i32 as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn url_file_name(
    mut u: *const url,
    mut replaced_filename: *mut i8,
) -> *mut i8 {
    let mut fnres: growable = growable {
        base: 0 as *mut i8,
        size: 0,
        tail: 0,
    };
    let mut temp_fnres: growable = growable {
        base: 0 as *mut i8,
        size: 0,
        tail: 0,
    };
    let mut u_file: *const i8 = 0 as *const i8;
    let mut fname: *mut i8 = 0 as *mut i8;
    let mut unique: *mut i8 = 0 as *mut i8;
    let mut fname_len_check: *mut i8 = 0 as *mut i8;
    let mut index_filename: *const i8 = b"index.html\0" as *const u8 as *const i8;
    fnres.base = 0 as *mut i8;
    fnres.size = 0 as i32;
    fnres.tail = 0 as i32;
    temp_fnres.base = 0 as *mut i8;
    temp_fnres.size = 0 as i32;
    temp_fnres.tail = 0 as i32;
    if !(opt.default_page).is_null() {
        index_filename = opt.default_page;
    }
    if !(opt.dir_prefix).is_null() {
        append_string(opt.dir_prefix, &mut fnres);
    }
    if opt.dirstruct {
        if opt.protocol_directories {
            if temp_fnres.tail != 0 {
                append_char('/' as i32 as i8, &mut temp_fnres);
            }
            append_string(supported_schemes[(*u).scheme as usize].name, &mut temp_fnres);
        }
        if opt.add_hostdir {
            if temp_fnres.tail != 0 {
                append_char('/' as i32 as i8, &mut temp_fnres);
            }
            if 0 as i32 != strcmp((*u).host, b"..\0" as *const u8 as *const i8) {
                append_string((*u).host, &mut temp_fnres);
            } else {
                append_string(b"%2E%2E\0" as *const u8 as *const i8, &mut temp_fnres);
            }
            if (*u).port != scheme_default_port((*u).scheme) {
                let mut portstr: [i8; 24] = [0; 24];
                number_to_string(portstr.as_mut_ptr(), (*u).port as wgint);
                append_char(
                    (if opt.restrict_files_os as u32
                        != C2RustUnnamed_1::restrict_windows as i32 as u32
                    {
                        ':' as i32
                    } else {
                        '+' as i32
                    }) as i8,
                    &mut temp_fnres,
                );
                append_string(portstr.as_mut_ptr(), &mut temp_fnres);
            }
        }
        append_dir_structure(u, &mut temp_fnres);
    }
    if replaced_filename.is_null() {
        u_file = if *(*u).file as i32 != 0 { (*u).file } else { index_filename };
        if !((*u).query).is_null() {
            fname_len_check = concat_strings(
                u_file,
                if opt.restrict_files_os as u32
                    != C2RustUnnamed_1::restrict_vms as i32 as u32
                    && opt.restrict_files_os as u32
                        != C2RustUnnamed_1::restrict_windows as i32 as u32
                {
                    b"?\0" as *const u8 as *const i8
                } else {
                    b"@\0" as *const u8 as *const i8
                },
                (*u).query,
                0 as *mut libc::c_void,
            );
        } else {
            fname_len_check = strdupdelim(
                u_file,
                u_file.offset(strlen(u_file) as isize),
            );
        }
    } else {
        u_file = replaced_filename;
        fname_len_check = strdupdelim(u_file, u_file.offset(strlen(u_file) as isize));
    }
    if temp_fnres.tail != 0 {
        append_char('/' as i32 as i8, &mut temp_fnres);
    }
    append_uri_pathel(
        fname_len_check,
        fname_len_check.offset(strlen(fname_len_check) as isize),
        1 as i32 != 0,
        &mut temp_fnres,
    );
    append_char('\0' as i32 as i8, &mut temp_fnres);
    fname = convert_fname(temp_fnres.base);
    temp_fnres.base = 0 as *mut i8;
    temp_fnres.size = 0 as i32;
    temp_fnres.tail = 0 as i32;
    append_string(fname, &mut temp_fnres);
    rpl_free(fname as *mut libc::c_void);
    fname = 0 as *mut i8;
    rpl_free(fname_len_check as *mut libc::c_void);
    fname_len_check = 0 as *mut i8;
    if fnres.tail != 0 {
        append_char('/' as i32 as i8, &mut fnres);
    }
    append_string(temp_fnres.base, &mut fnres);
    fname = fnres.base;
    rpl_free(temp_fnres.base as *mut libc::c_void);
    temp_fnres.base = 0 as *mut i8;
    if (opt.noclobber as i32 != 0 || opt.always_rest as i32 != 0
        || opt.timestamping as i32 != 0 || opt.dirstruct as i32 != 0
        || !(opt.output_document).is_null() || opt.backups > 0 as i32)
        && !(file_exists_p(fname, 0 as *mut file_stats_t) as i32 != 0
            && !file_non_directory_p(fname))
    {
        unique = fname;
    } else {
        unique = unique_name_passthrough(fname);
        if unique != fname {
            rpl_free(fname as *mut libc::c_void);
            fname = 0 as *mut i8;
        }
    }
    return unique;
}
unsafe extern "C" fn path_simplify(mut scheme: url_scheme, mut path: *mut i8) -> bool {
    let mut h: *mut i8 = path;
    let mut t: *mut i8 = path;
    let mut beg: *mut i8 = path;
    let mut end: *mut i8 = strchr(path, '\0' as i32);
    while h < end {
        if *h.offset(0 as i32 as isize) as i32 == '.' as i32
            && (*h.offset(1 as i32 as isize) as i32 == '/' as i32
                || *h.offset(1 as i32 as isize) as i32 == '\0' as i32)
        {
            h = h.offset(2 as i32 as isize);
        } else {
            let mut current_block_18: u64;
            if *h.offset(0 as i32 as isize) as i32 == '.' as i32
                && *h.offset(1 as i32 as isize) as i32 == '.' as i32
                && (*h.offset(2 as i32 as isize) as i32 == '/' as i32
                    || *h.offset(2 as i32 as isize) as i32 == '\0' as i32)
            {
                if t > beg {
                    t = t.offset(-1);
                    t;
                    while t > beg && *t.offset(-(1 as i32) as isize) as i32 != '/' as i32
                    {
                        t = t.offset(-1);
                        t;
                    }
                    current_block_18 = 2968425633554183086;
                } else if scheme as u32 == url_scheme::SCHEME_FTP as i32 as u32
                    || scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32
                {
                    beg = t.offset(3 as i32 as isize);
                    current_block_18 = 3511509469051899454;
                } else {
                    current_block_18 = 2968425633554183086;
                }
                match current_block_18 {
                    3511509469051899454 => {}
                    _ => {
                        h = h.offset(3 as i32 as isize);
                        current_block_18 = 11307063007268554308;
                    }
                }
            } else {
                current_block_18 = 3511509469051899454;
            }
            match current_block_18 {
                3511509469051899454 => {
                    if t == h {
                        while h < end && *h as i32 != '/' as i32 {
                            t = t.offset(1);
                            t;
                            h = h.offset(1);
                            h;
                        }
                        if h < end {
                            t = t.offset(1);
                            t;
                            h = h.offset(1);
                            h;
                        }
                    } else {
                        while h < end && *h as i32 != '/' as i32 {
                            let fresh24 = h;
                            h = h.offset(1);
                            let fresh25 = t;
                            t = t.offset(1);
                            *fresh25 = *fresh24;
                        }
                        if h < end {
                            let fresh26 = h;
                            h = h.offset(1);
                            let fresh27 = t;
                            t = t.offset(1);
                            *fresh27 = *fresh26;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if t != h {
        *t = '\0' as i32 as i8;
    }
    return t != h;
}
unsafe extern "C" fn path_end(mut url: *const i8) -> *const i8 {
    let mut scheme: url_scheme = url_scheme(url);
    let mut seps: *const i8 = 0 as *const i8;
    if scheme as u32 == url_scheme::SCHEME_INVALID as i32 as u32 {
        scheme = url_scheme::SCHEME_HTTP;
    }
    seps = (init_seps(scheme)).offset(2 as i32 as isize);
    return strpbrk_or_eos(url, seps);
}
#[no_mangle]
pub unsafe extern "C" fn uri_merge(mut base: *const i8, mut link: *const i8) -> *mut i8 {
    let mut linklength: i32 = 0;
    let mut end: *const i8 = 0 as *const i8;
    let mut merge: *mut i8 = 0 as *mut i8;
    if url_has_scheme(link) {
        return xstrdup(link);
    }
    end = path_end(base);
    linklength = strlen(link) as i32;
    if *link == 0 {
        return xstrdup(base)
    } else if *link as i32 == '?' as i32 {
        let mut baselength: i32 = end.offset_from(base) as i64 as i32;
        merge = xmalloc((baselength + linklength + 1 as i32) as size_t) as *mut i8;
        memcpy(
            merge as *mut libc::c_void,
            base as *const libc::c_void,
            baselength as u64,
        );
        memcpy(
            merge.offset(baselength as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as u64,
        );
        *merge.offset((baselength + linklength) as isize) = '\0' as i32 as i8;
    } else if *link as i32 == '#' as i32 {
        let mut baselength_0: i32 = 0;
        let mut end1: *const i8 = strchr(base, '#' as i32);
        if end1.is_null() {
            end1 = base.offset(strlen(base) as isize);
        }
        baselength_0 = end1.offset_from(base) as i64 as i32;
        merge = xmalloc((baselength_0 + linklength + 1 as i32) as size_t) as *mut i8;
        memcpy(
            merge as *mut libc::c_void,
            base as *const libc::c_void,
            baselength_0 as u64,
        );
        memcpy(
            merge.offset(baselength_0 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as u64,
        );
        *merge.offset((baselength_0 + linklength) as isize) = '\0' as i32 as i8;
    } else if *link as i32 == '/' as i32
        && *link.offset(1 as i32 as isize) as i32 == '/' as i32
    {
        let mut span: i32 = 0;
        let mut slash: *const i8 = 0 as *const i8;
        let mut start_insert: *const i8 = 0 as *const i8;
        slash = memchr(
            base as *const libc::c_void,
            '/' as i32,
            end.offset_from(base) as i64 as u64,
        ) as *const i8;
        if !slash.is_null() && *slash.offset(1 as i32 as isize) as i32 == '/' as i32 {
            start_insert = slash;
        } else {
            start_insert = base;
        }
        span = start_insert.offset_from(base) as i64 as i32;
        merge = xmalloc((span + linklength + 1 as i32) as size_t) as *mut i8;
        if span != 0 {
            memcpy(merge as *mut libc::c_void, base as *const libc::c_void, span as u64);
        }
        memcpy(
            merge.offset(span as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as u64,
        );
        *merge.offset((span + linklength) as isize) = '\0' as i32 as i8;
    } else if *link as i32 == '/' as i32 {
        let mut span_0: i32 = 0;
        let mut slash_0: *const i8 = 0 as *const i8;
        let mut start_insert_0: *const i8 = 0 as *const i8;
        let mut pos: *const i8 = base;
        let mut seen_slash_slash: bool = 0 as i32 != 0;
        loop {
            slash_0 = memchr(
                pos as *const libc::c_void,
                '/' as i32,
                end.offset_from(pos) as i64 as u64,
            ) as *const i8;
            if !(!slash_0.is_null() && !seen_slash_slash) {
                break;
            }
            if !(*slash_0.offset(1 as i32 as isize) as i32 == '/' as i32) {
                break;
            }
            pos = slash_0.offset(2 as i32 as isize);
            seen_slash_slash = 1 as i32 != 0;
        }
        if slash_0.is_null() && !seen_slash_slash {
            start_insert_0 = base;
        } else if slash_0.is_null() && seen_slash_slash as i32 != 0 {
            start_insert_0 = end;
        } else if !slash_0.is_null() && !seen_slash_slash {
            start_insert_0 = base;
        } else if !slash_0.is_null() && seen_slash_slash as i32 != 0 {
            start_insert_0 = slash_0;
        }
        span_0 = start_insert_0.offset_from(base) as i64 as i32;
        merge = xmalloc((span_0 + linklength + 1 as i32) as size_t) as *mut i8;
        if span_0 != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span_0 as u64,
            );
        }
        memcpy(
            merge.offset(span_0 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as u64,
        );
        *merge.offset((span_0 + linklength) as isize) = '\0' as i32 as i8;
    } else {
        let mut need_explicit_slash: bool = 0 as i32 != 0;
        let mut span_1: i32 = 0;
        let mut start_insert_1: *const i8 = 0 as *const i8;
        let mut last_slash: *const i8 = memrchr(
            base as *const libc::c_void,
            '/' as i32,
            end.offset_from(base) as i64 as size_t,
        ) as *const i8;
        if last_slash.is_null() {
            start_insert_1 = base;
        } else if !last_slash.is_null() && last_slash >= base.offset(2 as i32 as isize)
            && *last_slash.offset(-(2 as i32) as isize) as i32 == ':' as i32
            && *last_slash.offset(-(1 as i32) as isize) as i32 == '/' as i32
        {
            start_insert_1 = end.offset(1 as i32 as isize);
            need_explicit_slash = 1 as i32 != 0;
        } else {
            start_insert_1 = last_slash.offset(1 as i32 as isize);
        }
        span_1 = start_insert_1.offset_from(base) as i64 as i32;
        merge = xmalloc((span_1 + linklength + 1 as i32) as size_t) as *mut i8;
        if span_1 != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span_1 as u64,
            );
        }
        if need_explicit_slash {
            *merge.offset((span_1 - 1 as i32) as isize) = '/' as i32 as i8;
        }
        memcpy(
            merge.offset(span_1 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as u64,
        );
        *merge.offset((span_1 + linklength) as isize) = '\0' as i32 as i8;
    }
    return merge;
}
#[no_mangle]
pub unsafe extern "C" fn url_string(
    mut url: *const url,
    mut auth_mode: url_auth_mode,
) -> *mut i8 {
    let mut size: i32 = 0;
    let mut result: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut quoted_host: *mut i8 = 0 as *mut i8;
    let mut quoted_user: *mut i8 = 0 as *mut i8;
    let mut quoted_passwd: *mut i8 = 0 as *mut i8;
    let mut scheme_port: i32 = supported_schemes[(*url).scheme as usize].default_port;
    let mut scheme_str: *const i8 = supported_schemes[(*url).scheme as usize]
        .leading_string;
    let mut fplen: i32 = full_path_length(url);
    let mut brackets_around_host: bool = false;
    if !((*url).user).is_null() {
        if auth_mode as u32 != url_auth_mode::URL_AUTH_HIDE as i32 as u32 {
            quoted_user = url_escape_allow_passthrough((*url).user);
            if !((*url).passwd).is_null() {
                if auth_mode as u32 == url_auth_mode::URL_AUTH_HIDE_PASSWD as i32 as u32
                {
                    quoted_passwd = b"*password*\0" as *const u8 as *const i8 as *mut i8;
                } else {
                    quoted_passwd = url_escape_allow_passthrough((*url).passwd);
                }
            }
        }
    }
    quoted_host = url_escape_allow_passthrough((*url).host);
    if quoted_host != (*url).host {
        unescape_single_char(quoted_host, ':' as i32 as i8);
    }
    brackets_around_host = !(strchr(quoted_host, ':' as i32)).is_null();
    size = (strlen(scheme_str))
        .wrapping_add(strlen(quoted_host))
        .wrapping_add(
            (if brackets_around_host as i32 != 0 { 2 as i32 } else { 0 as i32 }) as u64,
        )
        .wrapping_add(fplen as u64)
        .wrapping_add(1 as i32 as u64) as i32;
    if (*url).port != scheme_port {
        size += 1 as i32 + numdigit((*url).port as wgint);
    }
    if !quoted_user.is_null() {
        size = (size as u64)
            .wrapping_add((1 as i32 as u64).wrapping_add(strlen(quoted_user))) as i32
            as i32;
        if !quoted_passwd.is_null() {
            size = (size as u64)
                .wrapping_add((1 as i32 as u64).wrapping_add(strlen(quoted_passwd)))
                as i32 as i32;
        }
    }
    result = xmalloc(size as size_t) as *mut i8;
    p = result;
    let mut len: i32 = strlen(scheme_str) as i32;
    memcpy(p as *mut libc::c_void, scheme_str as *const libc::c_void, len as u64);
    p = p.offset(len as isize);
    if !quoted_user.is_null() {
        let mut len_0: i32 = strlen(quoted_user) as i32;
        memcpy(p as *mut libc::c_void, quoted_user as *const libc::c_void, len_0 as u64);
        p = p.offset(len_0 as isize);
        if !quoted_passwd.is_null() {
            let fresh28 = p;
            p = p.offset(1);
            *fresh28 = ':' as i32 as i8;
            let mut len_1: i32 = strlen(quoted_passwd) as i32;
            memcpy(
                p as *mut libc::c_void,
                quoted_passwd as *const libc::c_void,
                len_1 as u64,
            );
            p = p.offset(len_1 as isize);
        }
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = '@' as i32 as i8;
    }
    if brackets_around_host {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = '[' as i32 as i8;
    }
    let mut len_2: i32 = strlen(quoted_host) as i32;
    memcpy(p as *mut libc::c_void, quoted_host as *const libc::c_void, len_2 as u64);
    p = p.offset(len_2 as isize);
    if brackets_around_host {
        let fresh31 = p;
        p = p.offset(1);
        *fresh31 = ']' as i32 as i8;
    }
    if (*url).port != scheme_port {
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = ':' as i32 as i8;
        p = number_to_string(p, (*url).port as wgint);
    }
    full_path_write(url, p);
    p = p.offset(fplen as isize);
    let fresh33 = p;
    p = p.offset(1);
    *fresh33 = '\0' as i32 as i8;
    if !quoted_user.is_null() && quoted_user != (*url).user {
        rpl_free(quoted_user as *mut libc::c_void);
        quoted_user = 0 as *mut i8;
    }
    if !quoted_passwd.is_null()
        && auth_mode as u32 == url_auth_mode::URL_AUTH_SHOW as i32 as u32
        && quoted_passwd != (*url).passwd
    {
        rpl_free(quoted_passwd as *mut libc::c_void);
        quoted_passwd = 0 as *mut i8;
    }
    if quoted_host != (*url).host {
        rpl_free(quoted_host as *mut libc::c_void);
        quoted_host = 0 as *mut i8;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn schemes_are_similar_p(
    mut a: url_scheme,
    mut b: url_scheme,
) -> bool {
    if a as u32 == b as u32 {
        return 1 as i32 != 0;
    }
    if a as u32 == url_scheme::SCHEME_HTTP as i32 as u32
        && b as u32 == url_scheme::SCHEME_HTTPS as i32 as u32
        || a as u32 == url_scheme::SCHEME_HTTPS as i32 as u32
            && b as u32 == url_scheme::SCHEME_HTTP as i32 as u32
    {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn getchar_from_escaped_string(
    mut str: *const i8,
    mut c: *mut i8,
) -> i32 {
    let mut p: *const i8 = str;
    if *p.offset(0 as i32 as isize) as i32 == '%' as i32 {
        if !c_isxdigit(*p.offset(1 as i32 as isize) as i32)
            || !c_isxdigit(*p.offset(2 as i32 as isize) as i32)
        {
            *c = '%' as i32 as i8;
            return 1 as i32;
        } else {
            if *p.offset(2 as i32 as isize) as i32 == 0 as i32 {
                return 0 as i32;
            }
            *c = (((_unhex(*p.offset(1 as i32 as isize) as u8) as i32) << 4 as i32)
                + _unhex(*p.offset(2 as i32 as isize) as u8) as i32) as i8;
            if urlchr_table[*c as u8 as usize] as i32
                & C2RustUnnamed_7::urlchr_reserved as i32 != 0
            {
                *c = '%' as i32 as i8;
                return 1 as i32;
            } else {
                return 3 as i32
            }
        }
    } else {
        *c = *p.offset(0 as i32 as isize);
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn are_urls_equal(mut u1: *const i8, mut u2: *const i8) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    let mut q: *const i8 = 0 as *const i8;
    let mut pp: i32 = 0;
    let mut qq: i32 = 0;
    let mut ch1: i8 = 0;
    let mut ch2: i8 = 0;
    p = u1;
    q = u2;
    while *p as i32 != 0 && *q as i32 != 0
        && {
            pp = getchar_from_escaped_string(p, &mut ch1);
            pp != 0
        }
        && {
            qq = getchar_from_escaped_string(q, &mut ch2);
            qq != 0
        } && c_tolower(ch1 as i32) == c_tolower(ch2 as i32)
    {
        p = p.offset(pp as isize);
        q = q.offset(qq as isize);
    }
    return if *p as i32 == 0 as i32 && *q as i32 == 0 as i32 {
        1 as i32
    } else {
        0 as i32
    } != 0;
}