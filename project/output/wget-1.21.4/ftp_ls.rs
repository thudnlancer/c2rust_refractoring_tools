use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn fclose(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn strptime(__s: *const i8, __fmt: *const i8, __tp: *mut tm) -> *mut i8;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn rpl_mktime(__tp: *mut tm) -> time_t;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn abort() -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_strtoll(
        string: *const i8,
        endptr: *mut *mut i8,
        base: i32,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn rewind(__stream: *mut FILE);
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncat(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn concat_strings(_: *const i8, _: ...) -> *mut i8;
    fn number_to_static_string(_: wgint) -> *mut i8;
    static mut char_prop: [u8; 0];
    fn url_escape(_: *const i8) -> *mut i8;
    fn url_escape_unsafe_and_reserved(_: *const i8) -> *mut i8;
    fn html_quote_string(_: *const i8) -> *mut i8;
    static mut output_stream: *mut FILE;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn c_strncasecmp(s1: *const i8, s2: *const i8, n: size_t) -> i32;
}
pub type __int64_t = i64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type time_t = __time_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
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
    LOG_PROGRESS = 4,
    LOG_ALWAYS = 3,
    LOG_NONVERBOSE = 2,
    LOG_NOTQUIET = 1,
    LOG_VERBOSE = 0,
}
impl log_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_options::LOG_PROGRESS => 4,
            log_options::LOG_ALWAYS => 3,
            log_options::LOG_NONVERBOSE => 2,
            log_options::LOG_NOTQUIET => 1,
            log_options::LOG_VERBOSE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_options {
        match value {
            4 => log_options::LOG_PROGRESS,
            3 => log_options::LOG_ALWAYS,
            2 => log_options::LOG_NONVERBOSE,
            1 => log_options::LOG_NOTQUIET,
            0 => log_options::LOG_VERBOSE,
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
pub enum uerr_t {
    METALINK_SIZE_ERROR = 62,
    RETR_WITH_METALINK = 61,
    METALINK_MISSING_RESOURCE = 60,
    METALINK_SIG_ERROR = 59,
    METALINK_CHKSUM_ERROR = 58,
    METALINK_RETR_ERROR = 57,
    METALINK_PARSE_ERROR = 56,
    TIMECONV_ERR = 55,
    WARC_TMP_FWRITEERR = 54,
    WARC_TMP_FOPENERR = 53,
    WARC_ERR = 52,
    UNKNOWNATTR = 51,
    ATTRMISSING = 50,
    CLOSEFAILED = 49,
    NEWLOCATION_KEEP_POST = 48,
    UNLINKERR = 47,
    VERIFCERTERR = 46,
    SSLINITFAILED = 45,
    WRITEFAILED = 44,
    QUOTEXC = 43,
    AUTHFAILED = 42,
    PROXERR = 41,
    RETRBADPATTERN = 40,
    RANGEERR = 39,
    FILEBADFILE = 38,
    TRYLIMEXC = 37,
    READERR = 36,
    RETRFINISHED = 35,
    RETRUNNEEDED = 34,
    CONTNOTSUPPORTED = 33,
    FTPNOAUTH = 32,
    FTPNOPROT = 31,
    FTPNOPBSZ = 30,
    FTPNOPASV = 29,
    FTPINVPASV = 28,
    WRONGCODE = 27,
    RECLEVELEXC = 26,
    RETROK = 25,
    HERR = 24,
    GATEWAYTIMEOUT = 23,
    HEOF = 22,
    FWRITEERR = 21,
    FOPEN_EXCL_ERR = 20,
    FOPENERR = 19,
    URLERROR = 18,
    FTPRESTFAIL = 17,
    FTPRETRINT = 16,
    FTPSRVERR = 15,
    FTPRERR = 14,
    FTPUNKNOWNTYPE = 13,
    FTPNSFOD = 12,
    FTPSYSERR = 11,
    FTPPORTERR = 10,
    FTPLOGREFUSED = 9,
    FTPLOGINC = 8,
    FTPOK = 7,
    NEWLOCATION = 6,
    CONIMPOSSIBLE = 5,
    CONSSLERR = 4,
    CONERROR = 3,
    CONSOCKERR = 2,
    HOSTERR = 1,
    NOCONERROR = 0,
}
impl uerr_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            uerr_t::METALINK_SIZE_ERROR => 62,
            uerr_t::RETR_WITH_METALINK => 61,
            uerr_t::METALINK_MISSING_RESOURCE => 60,
            uerr_t::METALINK_SIG_ERROR => 59,
            uerr_t::METALINK_CHKSUM_ERROR => 58,
            uerr_t::METALINK_RETR_ERROR => 57,
            uerr_t::METALINK_PARSE_ERROR => 56,
            uerr_t::TIMECONV_ERR => 55,
            uerr_t::WARC_TMP_FWRITEERR => 54,
            uerr_t::WARC_TMP_FOPENERR => 53,
            uerr_t::WARC_ERR => 52,
            uerr_t::UNKNOWNATTR => 51,
            uerr_t::ATTRMISSING => 50,
            uerr_t::CLOSEFAILED => 49,
            uerr_t::NEWLOCATION_KEEP_POST => 48,
            uerr_t::UNLINKERR => 47,
            uerr_t::VERIFCERTERR => 46,
            uerr_t::SSLINITFAILED => 45,
            uerr_t::WRITEFAILED => 44,
            uerr_t::QUOTEXC => 43,
            uerr_t::AUTHFAILED => 42,
            uerr_t::PROXERR => 41,
            uerr_t::RETRBADPATTERN => 40,
            uerr_t::RANGEERR => 39,
            uerr_t::FILEBADFILE => 38,
            uerr_t::TRYLIMEXC => 37,
            uerr_t::READERR => 36,
            uerr_t::RETRFINISHED => 35,
            uerr_t::RETRUNNEEDED => 34,
            uerr_t::CONTNOTSUPPORTED => 33,
            uerr_t::FTPNOAUTH => 32,
            uerr_t::FTPNOPROT => 31,
            uerr_t::FTPNOPBSZ => 30,
            uerr_t::FTPNOPASV => 29,
            uerr_t::FTPINVPASV => 28,
            uerr_t::WRONGCODE => 27,
            uerr_t::RECLEVELEXC => 26,
            uerr_t::RETROK => 25,
            uerr_t::HERR => 24,
            uerr_t::GATEWAYTIMEOUT => 23,
            uerr_t::HEOF => 22,
            uerr_t::FWRITEERR => 21,
            uerr_t::FOPEN_EXCL_ERR => 20,
            uerr_t::FOPENERR => 19,
            uerr_t::URLERROR => 18,
            uerr_t::FTPRESTFAIL => 17,
            uerr_t::FTPRETRINT => 16,
            uerr_t::FTPSRVERR => 15,
            uerr_t::FTPRERR => 14,
            uerr_t::FTPUNKNOWNTYPE => 13,
            uerr_t::FTPNSFOD => 12,
            uerr_t::FTPSYSERR => 11,
            uerr_t::FTPPORTERR => 10,
            uerr_t::FTPLOGREFUSED => 9,
            uerr_t::FTPLOGINC => 8,
            uerr_t::FTPOK => 7,
            uerr_t::NEWLOCATION => 6,
            uerr_t::CONIMPOSSIBLE => 5,
            uerr_t::CONSSLERR => 4,
            uerr_t::CONERROR => 3,
            uerr_t::CONSOCKERR => 2,
            uerr_t::HOSTERR => 1,
            uerr_t::NOCONERROR => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> uerr_t {
        match value {
            62 => uerr_t::METALINK_SIZE_ERROR,
            61 => uerr_t::RETR_WITH_METALINK,
            60 => uerr_t::METALINK_MISSING_RESOURCE,
            59 => uerr_t::METALINK_SIG_ERROR,
            58 => uerr_t::METALINK_CHKSUM_ERROR,
            57 => uerr_t::METALINK_RETR_ERROR,
            56 => uerr_t::METALINK_PARSE_ERROR,
            55 => uerr_t::TIMECONV_ERR,
            54 => uerr_t::WARC_TMP_FWRITEERR,
            53 => uerr_t::WARC_TMP_FOPENERR,
            52 => uerr_t::WARC_ERR,
            51 => uerr_t::UNKNOWNATTR,
            50 => uerr_t::ATTRMISSING,
            49 => uerr_t::CLOSEFAILED,
            48 => uerr_t::NEWLOCATION_KEEP_POST,
            47 => uerr_t::UNLINKERR,
            46 => uerr_t::VERIFCERTERR,
            45 => uerr_t::SSLINITFAILED,
            44 => uerr_t::WRITEFAILED,
            43 => uerr_t::QUOTEXC,
            42 => uerr_t::AUTHFAILED,
            41 => uerr_t::PROXERR,
            40 => uerr_t::RETRBADPATTERN,
            39 => uerr_t::RANGEERR,
            38 => uerr_t::FILEBADFILE,
            37 => uerr_t::TRYLIMEXC,
            36 => uerr_t::READERR,
            35 => uerr_t::RETRFINISHED,
            34 => uerr_t::RETRUNNEEDED,
            33 => uerr_t::CONTNOTSUPPORTED,
            32 => uerr_t::FTPNOAUTH,
            31 => uerr_t::FTPNOPROT,
            30 => uerr_t::FTPNOPBSZ,
            29 => uerr_t::FTPNOPASV,
            28 => uerr_t::FTPINVPASV,
            27 => uerr_t::WRONGCODE,
            26 => uerr_t::RECLEVELEXC,
            25 => uerr_t::RETROK,
            24 => uerr_t::HERR,
            23 => uerr_t::GATEWAYTIMEOUT,
            22 => uerr_t::HEOF,
            21 => uerr_t::FWRITEERR,
            20 => uerr_t::FOPEN_EXCL_ERR,
            19 => uerr_t::FOPENERR,
            18 => uerr_t::URLERROR,
            17 => uerr_t::FTPRESTFAIL,
            16 => uerr_t::FTPRETRINT,
            15 => uerr_t::FTPSRVERR,
            14 => uerr_t::FTPRERR,
            13 => uerr_t::FTPUNKNOWNTYPE,
            12 => uerr_t::FTPNSFOD,
            11 => uerr_t::FTPSYSERR,
            10 => uerr_t::FTPPORTERR,
            9 => uerr_t::FTPLOGREFUSED,
            8 => uerr_t::FTPLOGINC,
            7 => uerr_t::FTPOK,
            6 => uerr_t::NEWLOCATION,
            5 => uerr_t::CONIMPOSSIBLE,
            4 => uerr_t::CONSSLERR,
            3 => uerr_t::CONERROR,
            2 => uerr_t::CONSOCKERR,
            1 => uerr_t::HOSTERR,
            0 => uerr_t::NOCONERROR,
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
pub enum url_scheme {
    SCHEME_INVALID = 4,
    SCHEME_FTPS = 3,
    SCHEME_FTP = 2,
    SCHEME_HTTPS = 1,
    SCHEME_HTTP = 0,
}
impl url_scheme {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            url_scheme::SCHEME_INVALID => 4,
            url_scheme::SCHEME_FTPS => 3,
            url_scheme::SCHEME_FTP => 2,
            url_scheme::SCHEME_HTTPS => 1,
            url_scheme::SCHEME_HTTP => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> url_scheme {
        match value {
            4 => url_scheme::SCHEME_INVALID,
            3 => url_scheme::SCHEME_FTPS,
            2 => url_scheme::SCHEME_FTP,
            1 => url_scheme::SCHEME_HTTPS,
            0 => url_scheme::SCHEME_HTTP,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stype {
    ST_OTHER = 5,
    ST_OS400 = 4,
    ST_MACOS = 3,
    ST_WINNT = 2,
    ST_VMS = 1,
    ST_UNIX = 0,
}
impl stype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            stype::ST_OTHER => 5,
            stype::ST_OS400 => 4,
            stype::ST_MACOS => 3,
            stype::ST_WINNT => 2,
            stype::ST_VMS => 1,
            stype::ST_UNIX => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> stype {
        match value {
            5 => stype::ST_OTHER,
            4 => stype::ST_OS400,
            3 => stype::ST_MACOS,
            2 => stype::ST_WINNT,
            1 => stype::ST_VMS,
            0 => stype::ST_UNIX,
            _ => panic!("Invalid value for stype: {}", value),
        }
    }
}
impl AddAssign<u32> for stype {
    fn add_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for stype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for stype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for stype {
    fn div_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for stype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for stype {
    type Output = stype;
    fn add(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for stype {
    type Output = stype;
    fn sub(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for stype {
    type Output = stype;
    fn mul(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for stype {
    type Output = stype;
    fn div(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for stype {
    type Output = stype;
    fn rem(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ftype {
    FT_UNKNOWN = 3,
    FT_SYMLINK = 2,
    FT_DIRECTORY = 1,
    FT_PLAINFILE = 0,
}
impl ftype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ftype::FT_UNKNOWN => 3,
            ftype::FT_SYMLINK => 2,
            ftype::FT_DIRECTORY => 1,
            ftype::FT_PLAINFILE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> ftype {
        match value {
            3 => ftype::FT_UNKNOWN,
            2 => ftype::FT_SYMLINK,
            1 => ftype::FT_DIRECTORY,
            0 => ftype::FT_PLAINFILE,
            _ => panic!("Invalid value for ftype: {}", value),
        }
    }
}
impl AddAssign<u32> for ftype {
    fn add_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ftype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ftype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ftype {
    fn div_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ftype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ftype {
    type Output = ftype;
    fn add(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ftype {
    type Output = ftype;
    fn sub(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ftype {
    type Output = ftype;
    fn mul(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ftype {
    type Output = ftype;
    fn div(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ftype {
    type Output = ftype;
    fn rem(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum parsetype {
    TT_DAY = 1,
    TT_HOUR_MIN = 0,
}
impl parsetype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            parsetype::TT_DAY => 1,
            parsetype::TT_HOUR_MIN => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> parsetype {
        match value {
            1 => parsetype::TT_DAY,
            0 => parsetype::TT_HOUR_MIN,
            _ => panic!("Invalid value for parsetype: {}", value),
        }
    }
}
impl AddAssign<u32> for parsetype {
    fn add_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for parsetype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for parsetype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for parsetype {
    fn div_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for parsetype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for parsetype {
    type Output = parsetype;
    fn add(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for parsetype {
    type Output = parsetype;
    fn sub(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for parsetype {
    type Output = parsetype;
    fn mul(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for parsetype {
    type Output = parsetype;
    fn div(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for parsetype {
    type Output = parsetype;
    fn rem(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileinfo {
    pub type_0: ftype,
    pub name: *mut i8,
    pub size: wgint,
    pub tstamp: i64,
    pub ptype: parsetype,
    pub perms: i32,
    pub linkto: *mut i8,
    pub prev: *mut fileinfo,
    pub next: *mut fileinfo,
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
unsafe extern "C" fn symperms(mut s: *const i8) -> i32 {
    let mut perms: i32 = 0 as i32;
    let mut i: i32 = 0;
    if strlen(s) < 9 as i32 as u64 {
        return 0 as i32;
    }
    i = 0 as i32;
    while i < 3 as i32 {
        perms <<= 3 as i32;
        perms
            += (((*s.offset(0 as i32 as isize) as i32 == 'r' as i32) as i32) << 2 as i32)
                + (((*s.offset(1 as i32 as isize) as i32 == 'w' as i32) as i32)
                    << 1 as i32)
                + (*s.offset(2 as i32 as isize) as i32 == 'x' as i32
                    || *s.offset(2 as i32 as isize) as i32 == 's' as i32) as i32;
        i += 1;
        i;
        s = s.offset(3 as i32 as isize);
    }
    return perms;
}
unsafe extern "C" fn clean_line(mut line: *mut i8, mut len: i32) -> i32 {
    if len <= 0 as i32 {
        return 0 as i32;
    }
    while len > 0 as i32
        && (*line.offset((len - 1 as i32) as isize) as i32 == '\n' as i32
            || *line.offset((len - 1 as i32) as isize) as i32 == '\r' as i32)
    {
        len -= 1;
        *line.offset(len as isize) = '\0' as i32 as i8;
    }
    if len == 0 {
        return 0 as i32;
    }
    while *line != 0 {
        if *line as i32 == '\t' as i32 {
            *line = ' ' as i32 as i8;
        }
        line = line.offset(1);
        line;
    }
    return len;
}
unsafe extern "C" fn ftp_parse_unix_ls(
    mut fp: *mut FILE,
    mut ignore_perms: i32,
) -> *mut fileinfo {
    static mut months: [*const i8; 12] = [
        b"Jan\0" as *const u8 as *const i8,
        b"Feb\0" as *const u8 as *const i8,
        b"Mar\0" as *const u8 as *const i8,
        b"Apr\0" as *const u8 as *const i8,
        b"May\0" as *const u8 as *const i8,
        b"Jun\0" as *const u8 as *const i8,
        b"Jul\0" as *const u8 as *const i8,
        b"Aug\0" as *const u8 as *const i8,
        b"Sep\0" as *const u8 as *const i8,
        b"Oct\0" as *const u8 as *const i8,
        b"Nov\0" as *const u8 as *const i8,
        b"Dec\0" as *const u8 as *const i8,
    ];
    let mut next: i32 = 0;
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut error: i32 = 0;
    let mut ignore: i32 = 0;
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hour: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0;
    let mut ptype: i32 = 0;
    let mut timestruct: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    };
    let mut tnow: *mut tm = 0 as *mut tm;
    let mut timenow: time_t = 0;
    let mut bufsize: size_t = 0 as i32 as size_t;
    let mut line: *mut i8 = 0 as *mut i8;
    let mut tok: *mut i8 = 0 as *mut i8;
    let mut ptok: *mut i8 = 0 as *mut i8;
    let mut dir: *mut fileinfo = 0 as *mut fileinfo;
    let mut l: *mut fileinfo = 0 as *mut fileinfo;
    let mut cur: fileinfo = fileinfo {
        type_0: ftype::FT_PLAINFILE,
        name: 0 as *mut i8,
        size: 0,
        tstamp: 0,
        ptype: parsetype::TT_HOUR_MIN,
        perms: 0,
        linkto: 0 as *mut i8,
        prev: 0 as *mut fileinfo,
        next: 0 as *mut fileinfo,
    };
    l = 0 as *mut fileinfo;
    dir = l;
    loop {
        len = getline(&mut line, &mut bufsize, fp) as i32;
        if !(len > 0 as i32) {
            break;
        }
        len = clean_line(line, len);
        if c_strncasecmp(line, b"total\0" as *const u8 as *const i8, 5 as i32 as size_t)
            == 0
        {
            continue;
        }
        tok = strtok(line, b" \0" as *const u8 as *const i8);
        if tok.is_null() {
            continue;
        }
        cur.name = 0 as *mut i8;
        cur.linkto = 0 as *mut i8;
        match *tok as i32 {
            45 => {
                cur.type_0 = ftype::FT_PLAINFILE;
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"PLAINFILE; \0" as *const u8 as *const i8);
                }
            }
            100 => {
                cur.type_0 = ftype::FT_DIRECTORY;
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"DIRECTORY; \0" as *const u8 as *const i8);
                }
            }
            108 => {
                cur.type_0 = ftype::FT_SYMLINK;
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"SYMLINK; \0" as *const u8 as *const i8);
                }
            }
            _ => {
                cur.type_0 = ftype::FT_UNKNOWN;
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"UNKNOWN; \0" as *const u8 as *const i8);
                }
            }
        }
        if ignore_perms != 0 {
            match cur.type_0 as u32 {
                0 => {
                    cur.perms = 0o644 as i32;
                }
                1 => {
                    cur.perms = 0o755 as i32;
                }
                _ => {
                    cur.perms = 0o644 as i32;
                }
            }
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"implicit perms %0o; \0" as *const u8 as *const i8,
                    cur.perms as u32,
                );
            }
        } else {
            cur.perms = symperms(tok.offset(1 as i32 as isize));
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"perms %0o; \0" as *const u8 as *const i8,
                    cur.perms as u32,
                );
            }
        }
        ignore = 0 as i32;
        error = ignore;
        sec = 0 as i32;
        min = sec;
        hour = min;
        year = hour;
        day = 0 as i32;
        month = day;
        ptype = parsetype::TT_DAY as i32;
        next = -(1 as i32);
        tok = line;
        loop {
            ptok = tok;
            tok = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
            if !(tok != 0 as *mut libc::c_void as *mut i8) {
                break;
            }
            next -= 1;
            next;
            if next < 0 as i32 {
                i = 0 as i32;
                while i < 12 as i32 {
                    if c_strcasecmp(tok, months[i as usize]) == 0 {
                        break;
                    }
                    i += 1;
                    i;
                }
                if !(i != 12 as i32) {
                    continue;
                }
                let mut size: wgint = 0;
                if ptok == line {
                    error = 1 as i32;
                    break;
                } else {
                    *__errno_location() = 0 as i32;
                    size = rpl_strtoll(ptok, 0 as *mut *mut i8, 10 as i32) as wgint;
                    if size == 9223372036854775807 as i64
                        && *__errno_location() == 34 as i32
                    {
                        cur.size = 0 as i32 as wgint;
                    } else {
                        cur.size = size;
                    }
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"size: %s; \0" as *const u8 as *const i8,
                            number_to_static_string(cur.size),
                        );
                    }
                    month = i;
                    next = 5 as i32;
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"month: %s; \0" as *const u8 as *const i8,
                            months[month as usize],
                        );
                    }
                }
            } else if next == 4 as i32 {
                if *tok.offset(1 as i32 as isize) != 0 {
                    day = 10 as i32 * (*tok as i32 - '0' as i32)
                        + *tok.offset(1 as i32 as isize) as i32 - '0' as i32;
                } else {
                    day = *tok as i32 - '0' as i32;
                }
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"day: %d; \0" as *const u8 as *const i8, day);
                }
            } else if next == 3 as i32 {
                year = 0 as i32;
                sec = 0 as i32;
                hour = sec;
                min = hour;
                if c_isdigit(*tok as i32) {
                    while c_isdigit(*tok as i32) as i32 != 0 && year <= 99999 as i32 {
                        year = *tok as i32 - '0' as i32 + 10 as i32 * year;
                        tok = tok.offset(1);
                        tok;
                    }
                    if *tok as i32 == ':' as i32 {
                        let mut n: i32 = 0;
                        hour = year;
                        year = 0 as i32;
                        ptype = parsetype::TT_HOUR_MIN as i32;
                        tok = tok.offset(1);
                        tok;
                        n = 0 as i32;
                        while c_isdigit(*tok as i32) as i32 != 0 && n < 2 as i32 {
                            min = *tok as i32 - '0' as i32 + 10 as i32 * min;
                            tok = tok.offset(1);
                            tok;
                            n += 1;
                            n;
                        }
                        if *tok as i32 == ':' as i32 {
                            tok = tok.offset(1);
                            tok;
                            n = 0 as i32;
                            while c_isdigit(*tok as i32) as i32 != 0 && n < 2 as i32 {
                                sec = *tok as i32 - '0' as i32 + 10 as i32 * sec;
                                tok = tok.offset(1);
                                tok;
                                n += 1;
                                n;
                            }
                        }
                    }
                }
                if year != 0 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"year: %d (no tm); \0" as *const u8 as *const i8,
                            year,
                        );
                    }
                } else if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"time: %02d:%02d:%02d (no yr); \0" as *const u8 as *const i8,
                        hour,
                        min,
                        sec,
                    );
                }
            } else if next == 2 as i32 {
                let mut fnlen: i32 = 0;
                let mut p: *mut i8 = 0 as *mut i8;
                fnlen = strlen(tok) as i32;
                if (fnlen as i64) < len as i64 - tok.offset_from(line) as i64 {
                    *tok.offset(fnlen as isize) = ' ' as i32 as i8;
                    if cur.type_0 as u32 == ftype::FT_SYMLINK as i32 as u32 {
                        p = strstr(tok, b" -> \0" as *const u8 as *const i8);
                        if p.is_null() {
                            error = 1 as i32;
                            break;
                        } else {
                            cur.linkto = xstrdup(p.offset(4 as i32 as isize));
                            if opt.debug as i64 != 0 {
                                debug_logprintf(
                                    b"link to: %s\n\0" as *const u8 as *const i8,
                                    cur.linkto,
                                );
                            }
                            *p = '\0' as i32 as i8;
                        }
                    }
                }
                if strcmp(tok, b".\0" as *const u8 as *const i8) == 0
                    || strcmp(tok, b"..\0" as *const u8 as *const i8) == 0
                {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"\nIgnoring `.' and `..'; \0" as *const u8 as *const i8,
                        );
                    }
                    ignore = 1 as i32;
                    break;
                } else {
                    fnlen = strlen(tok) as i32;
                    cur.name = xmalloc((fnlen + 1 as i32) as size_t) as *mut i8;
                    memcpy(
                        cur.name as *mut libc::c_void,
                        tok as *const libc::c_void,
                        (fnlen + 1 as i32) as u64,
                    );
                    if fnlen != 0 {
                        if cur.type_0 as u32 == ftype::FT_DIRECTORY as i32 as u32
                            && *(cur.name).offset((fnlen - 1 as i32) as isize) as i32
                                == '/' as i32
                        {
                            *(cur.name).offset((fnlen - 1 as i32) as isize) = '\0' as i32
                                as i8;
                            if opt.debug as i64 != 0 {
                                debug_logprintf(
                                    b"trailing `/' on dir.\n\0" as *const u8 as *const i8,
                                );
                            }
                        } else if cur.type_0 as u32 == ftype::FT_SYMLINK as i32 as u32
                            && *(cur.name).offset((fnlen - 1 as i32) as isize) as i32
                                == '@' as i32
                        {
                            *(cur.name).offset((fnlen - 1 as i32) as isize) = '\0' as i32
                                as i8;
                            if opt.debug as i64 != 0 {
                                debug_logprintf(
                                    b"trailing `@' on link.\n\0" as *const u8 as *const i8,
                                );
                            }
                        } else if cur.type_0 as u32 == ftype::FT_PLAINFILE as i32 as u32
                            && cur.perms & 0o111 as i32 != 0
                            && *(cur.name).offset((fnlen - 1 as i32) as isize) as i32
                                == '*' as i32
                        {
                            *(cur.name).offset((fnlen - 1 as i32) as isize) = '\0' as i32
                                as i8;
                            if opt.debug as i64 != 0 {
                                debug_logprintf(
                                    b"trailing `*' on exec.\n\0" as *const u8 as *const i8,
                                );
                            }
                        }
                    } else {
                        error = 1 as i32;
                    }
                    break;
                }
            } else {
                abort();
            }
        }
        if (cur.name).is_null()
            || cur.type_0 as u32 == ftype::FT_SYMLINK as i32 as u32
                && (cur.linkto).is_null()
        {
            error = 1 as i32;
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"%s\n\0" as *const u8 as *const i8,
                if !(cur.name).is_null() {
                    cur.name
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
        }
        if error != 0 || ignore != 0 {
            if opt.debug as i64 != 0 {
                debug_logprintf(b"Skipping.\n\0" as *const u8 as *const i8);
            }
            rpl_free(cur.name as *mut libc::c_void);
            cur.name = 0 as *mut i8;
            rpl_free(cur.linkto as *mut libc::c_void);
            cur.linkto = 0 as *mut i8;
        } else {
            if dir.is_null() {
                dir = xmalloc(::core::mem::size_of::<fileinfo>() as u64)
                    as *mut fileinfo;
                l = dir;
                memcpy(
                    l as *mut libc::c_void,
                    &mut cur as *mut fileinfo as *const libc::c_void,
                    ::core::mem::size_of::<fileinfo>() as u64,
                );
                (*l).next = 0 as *mut fileinfo;
                (*l).prev = (*l).next;
            } else {
                cur.prev = l;
                (*l).next = xmalloc(::core::mem::size_of::<fileinfo>() as u64)
                    as *mut fileinfo;
                l = (*l).next;
                memcpy(
                    l as *mut libc::c_void,
                    &mut cur as *mut fileinfo as *const libc::c_void,
                    ::core::mem::size_of::<fileinfo>() as u64,
                );
                (*l).next = 0 as *mut fileinfo;
            }
            timenow = time(0 as *mut time_t);
            tnow = localtime(&mut timenow);
            timestruct.tm_sec = sec;
            timestruct.tm_min = min;
            timestruct.tm_hour = hour;
            timestruct.tm_mday = day;
            timestruct.tm_mon = month;
            if year == 0 as i32 {
                if month > (*tnow).tm_mon {
                    timestruct.tm_year = (*tnow).tm_year - 1 as i32;
                } else {
                    timestruct.tm_year = (*tnow).tm_year;
                }
            } else {
                timestruct.tm_year = year;
            }
            if timestruct.tm_year >= 1900 as i32 {
                timestruct.tm_year -= 1900 as i32;
            }
            timestruct.tm_wday = 0 as i32;
            timestruct.tm_yday = 0 as i32;
            timestruct.tm_isdst = -(1 as i32);
            (*l).tstamp = rpl_mktime(&mut timestruct);
            (*l).ptype = parsetype::from_libc_c_uint(ptype as u32);
        }
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut i8;
    return dir;
}
unsafe extern "C" fn ftp_parse_winnt_ls(mut fp: *mut FILE) -> *mut fileinfo {
    let mut len: i32 = 0;
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hour: i32 = 0;
    let mut min: i32 = 0;
    let mut bufsize: size_t = 0 as i32 as size_t;
    let mut timestruct: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    };
    let mut line: *mut i8 = 0 as *mut i8;
    let mut tok: *mut i8 = 0 as *mut i8;
    let mut filename: *mut i8 = 0 as *mut i8;
    let mut dir: *mut fileinfo = 0 as *mut fileinfo;
    let mut l: *mut fileinfo = 0 as *mut fileinfo;
    let mut cur: fileinfo = fileinfo {
        type_0: ftype::FT_PLAINFILE,
        name: 0 as *mut i8,
        size: 0,
        tstamp: 0,
        ptype: parsetype::TT_HOUR_MIN,
        perms: 0,
        linkto: 0 as *mut i8,
        prev: 0 as *mut fileinfo,
        next: 0 as *mut fileinfo,
    };
    l = 0 as *mut fileinfo;
    dir = l;
    cur.name = 0 as *mut i8;
    loop {
        len = getline(&mut line, &mut bufsize, fp) as i32;
        if !(len > 0 as i32) {
            break;
        }
        len = clean_line(line, len);
        if len < 40 as i32 {
            continue;
        }
        filename = line.offset(39 as i32 as isize);
        tok = strtok(line, b"-\0" as *const u8 as *const i8);
        if tok.is_null() {
            continue;
        }
        month = atoi(tok);
        if month < 0 as i32 {
            month = 0 as i32;
        } else {
            month -= 1;
            month;
        }
        tok = strtok(0 as *mut i8, b"-\0" as *const u8 as *const i8);
        if tok.is_null() {
            continue;
        }
        day = atoi(tok);
        tok = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
        if tok.is_null() {
            continue;
        }
        year = atoi(tok);
        if year <= 70 as i32 {
            year += 100 as i32;
        } else if year >= 1900 as i32 {
            year -= 1900 as i32;
            if len < 42 as i32 {
                continue;
            }
            filename = filename.offset(2 as i32 as isize);
        }
        rpl_free(cur.name as *mut libc::c_void);
        cur.name = 0 as *mut i8;
        memset(
            &mut cur as *mut fileinfo as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<fileinfo>() as u64,
        );
        cur.name = xstrdup(filename);
        if opt.debug as i64 != 0 {
            debug_logprintf(b"Name: '%s'\n\0" as *const u8 as *const i8, cur.name);
        }
        tok = strtok(0 as *mut i8, b":\0" as *const u8 as *const i8);
        if tok.is_null() {
            continue;
        }
        hour = atoi(tok);
        tok = strtok(0 as *mut i8, b"M\0" as *const u8 as *const i8);
        if tok.is_null() {
            continue;
        }
        min = atoi(tok);
        if *tok.offset(0 as i32 as isize) as i32 != 0
            && *tok.offset(1 as i32 as isize) as i32 != 0
        {
            tok = tok.offset(2 as i32 as isize);
        }
        if hour >= 12 as i32 || hour < 0 as i32 {
            hour = 0 as i32;
        }
        if *tok as i32 == 'P' as i32 {
            hour += 12 as i32;
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"YYYY/MM/DD HH:MM - %d/%02d/%02d %02d:%02d\n\0" as *const u8
                    as *const i8,
                year + 1900 as i32,
                month,
                day,
                hour,
                min,
            );
        }
        timestruct.tm_sec = 0 as i32;
        timestruct.tm_min = min;
        timestruct.tm_hour = hour;
        timestruct.tm_mday = day;
        timestruct.tm_mon = month;
        timestruct.tm_year = year;
        timestruct.tm_wday = 0 as i32;
        timestruct.tm_yday = 0 as i32;
        timestruct.tm_isdst = -(1 as i32);
        cur.tstamp = rpl_mktime(&mut timestruct);
        cur.ptype = parsetype::TT_HOUR_MIN;
        if opt.debug as i64 != 0 {
            debug_logprintf(b"Timestamp: %ld\n\0" as *const u8 as *const i8, cur.tstamp);
        }
        tok = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
        if tok.is_null() {
            continue;
        }
        while !tok.is_null() && *tok as i32 == '\0' as i32 {
            tok = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
        }
        if tok.is_null() {
            continue;
        }
        if *tok as i32 == '<' as i32 {
            cur.type_0 = ftype::FT_DIRECTORY;
            cur.size = 0 as i32 as wgint;
            cur.perms = 0o755 as i32;
            if opt.debug as i64 != 0 {
                debug_logprintf(b"Directory\n\0" as *const u8 as *const i8);
            }
        } else {
            let mut size: wgint = 0;
            cur.type_0 = ftype::FT_PLAINFILE;
            *__errno_location() = 0 as i32;
            size = rpl_strtoll(tok, 0 as *mut *mut i8, 10 as i32) as wgint;
            if size == 9223372036854775807 as i64 && *__errno_location() == 34 as i32 {
                cur.size = 0 as i32 as wgint;
            } else {
                cur.size = size;
            }
            cur.perms = 0o644 as i32;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"File, size %s bytes\n\0" as *const u8 as *const i8,
                    number_to_static_string(cur.size),
                );
            }
        }
        cur.linkto = 0 as *mut i8;
        if dir.is_null() {
            dir = xmalloc(::core::mem::size_of::<fileinfo>() as u64) as *mut fileinfo;
            l = dir;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::core::mem::size_of::<fileinfo>() as u64,
            );
            (*l).next = 0 as *mut fileinfo;
            (*l).prev = (*l).next;
        } else {
            cur.prev = l;
            (*l).next = xmalloc(::core::mem::size_of::<fileinfo>() as u64)
                as *mut fileinfo;
            l = (*l).next;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::core::mem::size_of::<fileinfo>() as u64,
            );
            (*l).next = 0 as *mut fileinfo;
        }
        cur.name = 0 as *mut i8;
    }
    rpl_free(cur.name as *mut libc::c_void);
    cur.name = 0 as *mut i8;
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut i8;
    return dir;
}
unsafe extern "C" fn eat_carets(mut str: *mut i8) {
    let mut strd: *mut i8 = 0 as *mut i8;
    let mut hdgt: i8 = 0;
    let mut uchr: u8 = 0;
    while *str as i32 != '\0' as i32 && *str as i32 != '^' as i32 {
        str = str.offset(1);
        str;
    }
    if *str as i32 != '\0' as i32 {
        strd = str;
        while *str as i32 != '\0' as i32 {
            uchr = *str as u8;
            if uchr as i32 == '^' as i32 {
                if *char_prop
                    .as_mut_ptr()
                    .offset(*str.offset(1 as i32 as isize) as u8 as isize) as i32
                    & 64 as i32 != 0
                    && *char_prop
                        .as_mut_ptr()
                        .offset(*str.offset(2 as i32 as isize) as u8 as isize) as i32
                        & 64 as i32 != 0
                {
                    str = str.offset(1);
                    uchr = *str as u8;
                    if uchr as i32 <= '9' as i32 {
                        hdgt = (uchr as i32 - '0' as i32) as i8;
                    } else {
                        hdgt = ((uchr as i32 - 'A' as i32 & 7 as i32) + 10 as i32) as i8;
                    }
                    hdgt = ((hdgt as i32) << 4 as i32) as i8;
                    str = str.offset(1);
                    uchr = *str as u8;
                    if uchr as i32 <= '9' as i32 {
                        uchr = (hdgt as i32 + uchr as i32 - '0' as i32) as u8;
                    } else {
                        uchr = (hdgt as i32 + (uchr as i32 - 'A' as i32 & 15 as i32)
                            + 10 as i32) as u8;
                    }
                } else if uchr as i32 == '_' as i32 {
                    uchr = ' ' as i32 as u8;
                } else if uchr as i32 == '/' as i32 {
                    uchr = '?' as i32 as u8;
                }
            }
            *strd = uchr as i8;
            strd = strd.offset(1);
            strd;
            str = str.offset(1);
            str;
        }
        *strd = '\0' as i32 as i8;
    }
}
unsafe extern "C" fn ftp_parse_vms_ls(mut fp: *mut FILE) -> *mut fileinfo {
    let mut dt: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut perms: i32 = 0;
    let mut bufsize: size_t = 0 as i32 as size_t;
    let mut timenow: time_t = 0;
    let mut timestruct: *mut tm = 0 as *mut tm;
    let mut date_str: [i8; 32] = [0; 32];
    let mut line: *mut i8 = 0 as *mut i8;
    let mut tok: *mut i8 = 0 as *mut i8;
    let mut dir: *mut fileinfo = 0 as *mut fileinfo;
    let mut l: *mut fileinfo = 0 as *mut fileinfo;
    let mut cur: fileinfo = fileinfo {
        type_0: ftype::FT_PLAINFILE,
        name: 0 as *mut i8,
        size: 0,
        tstamp: 0,
        ptype: parsetype::TT_HOUR_MIN,
        perms: 0,
        linkto: 0 as *mut i8,
        prev: 0 as *mut fileinfo,
        next: 0 as *mut fileinfo,
    };
    l = 0 as *mut fileinfo;
    dir = l;
    j = 0 as i32;
    loop {
        i = getline(&mut line, &mut bufsize, fp) as i32;
        if !(i > 0 as i32) {
            break;
        }
        i = clean_line(line, i);
        if i <= 0 as i32 {
            continue;
        }
        if j == 0 as i32 && *line.offset((i - 1 as i32) as isize) as i32 == ']' as i32 {
            j = 1 as i32;
        } else {
            if !(strncmp(line, b"Total of \0" as *const u8 as *const i8, 9 as i32 as u64)
                == 0)
            {
                break;
            }
            i = 0 as i32;
            break;
        }
    }
    cur.name = 0 as *mut i8;
    while i > 0 as i32 {
        let mut p: *mut i8 = 0 as *mut i8;
        tok = strtok(line, b" \0" as *const u8 as *const i8);
        if tok.is_null() {
            tok = line;
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(b"file name:   '%s'\n\0" as *const u8 as *const i8, tok);
        }
        p = tok.offset(strlen(tok) as isize);
        loop {
            p = p.offset(-1);
            if !(p > tok && c_isdigit(*p as i32) as i32 != 0) {
                break;
            }
        }
        if p > tok && *p as i32 == ';' as i32
            && *p.offset(-(1 as i32 as isize)) as i32 != '^' as i32
        {
            *p = '\0' as i32 as i8;
        }
        eat_carets(tok);
        if opt.debug as i64 != 0 {
            debug_logprintf(b"file name-^: '%s'\n\0" as *const u8 as *const i8, tok);
        }
        len = strlen(tok) as i32;
        if len >= 4 as i32
            && c_strncasecmp(
                tok.offset((len - 4 as i32) as isize),
                b".DIR\0" as *const u8 as *const i8,
                4 as i32 as size_t,
            ) == 0
        {
            *tok.offset((len - 4 as i32) as isize) = '\0' as i32 as i8;
            cur.type_0 = ftype::FT_DIRECTORY;
            cur.perms = 0o755 as i32;
            if opt.debug as i64 != 0 {
                debug_logprintf(b"Directory (nv)\n\0" as *const u8 as *const i8);
            }
        } else if len >= 6 as i32
            && c_strncasecmp(
                tok.offset(len as isize).offset(-(6 as i32 as isize)),
                b".DIR;1\0" as *const u8 as *const i8,
                6 as i32 as size_t,
            ) == 0
        {
            *tok.offset((len - 6 as i32) as isize) = '\0' as i32 as i8;
            cur.type_0 = ftype::FT_DIRECTORY;
            cur.perms = 0o755 as i32;
            if opt.debug as i64 != 0 {
                debug_logprintf(b"Directory (v)\n\0" as *const u8 as *const i8);
            }
        } else {
            cur.type_0 = ftype::FT_PLAINFILE;
            cur.perms = 0o644 as i32;
            if opt.debug as i64 != 0 {
                debug_logprintf(b"File\n\0" as *const u8 as *const i8);
            }
        }
        rpl_free(cur.name as *mut libc::c_void);
        cur.name = 0 as *mut i8;
        cur.name = xstrdup(tok);
        if opt.debug as i64 != 0 {
            debug_logprintf(b"Name: '%s'\n\0" as *const u8 as *const i8, cur.name);
        }
        *date_str.as_mut_ptr() = '\0' as i32 as i8;
        cur.linkto = 0 as *mut i8;
        cur.size = 0 as i32 as wgint;
        tok = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
        if tok.is_null() {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Getting additional line.\n\0" as *const u8 as *const i8,
                );
            }
            i = getline(&mut line, &mut bufsize, fp) as i32;
            if i <= 0 as i32 {
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"EOF.  Leaving listing parser.\n\0" as *const u8 as *const i8,
                    );
                }
                break;
            } else {
                i = clean_line(line, i);
                if i <= 0 as i32 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"Blank line.  Leaving listing parser.\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    break;
                } else if *line.offset(0 as i32 as isize) as i32 != ' ' as i32 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"Non-blank in column 1.  Must be a new file name?\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    continue;
                } else {
                    tok = strtok(line, b" \0" as *const u8 as *const i8);
                    if tok.is_null() {
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"Null token.  Leaving listing parser.\n\0" as *const u8
                                    as *const i8,
                            );
                        }
                        break;
                    }
                }
            }
        }
        while !tok.is_null() {
            if opt.debug as i64 != 0 {
                debug_logprintf(b"Token: >%s<: \0" as *const u8 as *const i8, tok);
            }
            if strlen(tok) < 12 as i32 as u64 && !(strchr(tok, '-' as i32)).is_null() {
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"Date.\n\0" as *const u8 as *const i8);
                }
                snprintf(
                    date_str.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 32]>() as u64,
                    b"%s \0" as *const u8 as *const i8,
                    tok,
                );
            } else if strlen(tok) < 12 as i32 as u64
                && !(strchr(tok, ':' as i32)).is_null()
            {
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"Time. \0" as *const u8 as *const i8);
                }
                strncat(
                    date_str.as_mut_ptr(),
                    tok,
                    (::core::mem::size_of::<[i8; 32]>() as u64)
                        .wrapping_sub(strlen(date_str.as_mut_ptr()))
                        .wrapping_sub(1 as i32 as u64),
                );
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Date time: >%s<\n\0" as *const u8 as *const i8,
                        date_str.as_mut_ptr(),
                    );
                }
            } else if !(strchr(tok, '[' as i32)).is_null() {
                if opt.debug as i64 != 0 {
                    debug_logprintf(b"Owner.\n\0" as *const u8 as *const i8);
                }
            } else if !(strchr(tok, '(' as i32)).is_null() {
                perms = 0 as i32;
                j = 0 as i32;
                i = 0 as i32;
                while i < strlen(tok) as i32 {
                    match *tok.offset(i as isize) as i32 {
                        44 => {
                            if j == 0 as i32 {
                                perms = 0 as i32;
                            } else if j < 4 as i32 {
                                perms <<= 3 as i32;
                            }
                            j += 1;
                            j;
                        }
                        82 => {
                            perms |= 4 as i32;
                        }
                        87 => {
                            perms |= 2 as i32;
                        }
                        69 => {
                            perms |= 1 as i32;
                        }
                        68 => {
                            perms |= 2 as i32;
                        }
                        40 | 41 | _ => {}
                    }
                    i += 1;
                    i;
                }
                cur.perms = perms;
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Prot.  perms = %0o.\n\0" as *const u8 as *const i8,
                        cur.perms as u32,
                    );
                }
            } else if opt.debug as i64 != 0 {
                debug_logprintf(b"Ignored (size?).\n\0" as *const u8 as *const i8);
            }
            tok = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
        }
        timenow = time(0 as *mut time_t);
        timestruct = localtime(&mut timenow);
        strptime(
            date_str.as_mut_ptr(),
            b"%d-%b-%Y %H:%M:%S\0" as *const u8 as *const i8,
            timestruct,
        );
        timenow = rpl_mktime(timestruct);
        tok = getenv(b"WGET_TIMEZONE_DIFFERENTIAL\0" as *const u8 as *const i8);
        if !tok.is_null() {
            dt = atoi(tok);
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Time differential = %d.\n\0" as *const u8 as *const i8,
                    dt,
                );
            }
        } else {
            dt = 0 as i32;
        }
        if dt >= 0 as i32 {
            timenow += dt as i64;
        } else {
            timenow -= -dt as i64;
        }
        cur.tstamp = timenow;
        if opt.debug as i64 != 0 {
            debug_logprintf(b"Timestamp: %ld\n\0" as *const u8 as *const i8, cur.tstamp);
        }
        cur.ptype = parsetype::TT_HOUR_MIN;
        if dir.is_null() {
            dir = xmalloc(::core::mem::size_of::<fileinfo>() as u64) as *mut fileinfo;
            l = dir;
            cur.next = 0 as *mut fileinfo;
            cur.prev = cur.next;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::core::mem::size_of::<fileinfo>() as u64,
            );
        } else {
            cur.prev = l;
            cur.next = 0 as *mut fileinfo;
            (*l).next = xmalloc(::core::mem::size_of::<fileinfo>() as u64)
                as *mut fileinfo;
            l = (*l).next;
            memcpy(
                l as *mut libc::c_void,
                &mut cur as *mut fileinfo as *const libc::c_void,
                ::core::mem::size_of::<fileinfo>() as u64,
            );
        }
        cur.name = 0 as *mut i8;
        i = getline(&mut line, &mut bufsize, fp) as i32;
        if !(i > 0 as i32) {
            continue;
        }
        i = clean_line(line, i);
        if i <= 0 as i32 {
            break;
        }
    }
    rpl_free(cur.name as *mut libc::c_void);
    cur.name = 0 as *mut i8;
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut i8;
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_parse_ls(
    mut file: *const i8,
    system_type: stype,
) -> *mut fileinfo {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fi: *mut fileinfo = 0 as *mut fileinfo;
    fp = rpl_fopen(file, b"rb\0" as *const u8 as *const i8);
    if fp.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const i8,
            file,
            strerror(*__errno_location()),
        );
        return 0 as *mut fileinfo;
    }
    fi = ftp_parse_ls_fp(fp, system_type);
    fclose(fp);
    return fi;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_parse_ls_fp(
    mut fp: *mut FILE,
    system_type: stype,
) -> *mut fileinfo {
    match system_type as u32 {
        0 => return ftp_parse_unix_ls(fp, 0 as i32),
        2 => {
            let mut c: i32 = fgetc(fp);
            rewind(fp);
            if c >= '0' as i32 && c <= '9' as i32 {
                return ftp_parse_winnt_ls(fp)
            } else {
                return ftp_parse_unix_ls(fp, 1 as i32)
            }
        }
        1 => return ftp_parse_vms_ls(fp),
        3 => return ftp_parse_unix_ls(fp, 1 as i32),
        _ => {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Unsupported listing type, trying Unix listing parser.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            return ftp_parse_unix_ls(fp, 0 as i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ftp_index(
    mut file: *const i8,
    mut u: *mut url,
    mut f: *mut fileinfo,
) -> uerr_t {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut upwd: *mut i8 = 0 as *mut i8;
    let mut htcldir: *mut i8 = 0 as *mut i8;
    let mut htclfile: *mut i8 = 0 as *mut i8;
    let mut urlclfile: *mut i8 = 0 as *mut i8;
    if output_stream.is_null() {
        fp = rpl_fopen(file, b"wb\0" as *const u8 as *const i8);
        if fp.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s: %s\n\0" as *const u8 as *const i8,
                file,
                strerror(*__errno_location()),
            );
            return uerr_t::FOPENERR;
        }
    } else {
        fp = output_stream;
    }
    if !((*u).user).is_null() {
        let mut tmpu: *mut i8 = 0 as *mut i8;
        let mut tmpp: *mut i8 = 0 as *mut i8;
        tmpu = url_escape((*u).user);
        tmpp = if !((*u).passwd).is_null() {
            url_escape((*u).passwd)
        } else {
            0 as *mut i8
        };
        if !tmpp.is_null() {
            upwd = concat_strings(
                tmpu,
                b":\0" as *const u8 as *const i8,
                tmpp,
                b"@\0" as *const u8 as *const i8,
                0 as *mut i8,
            );
        } else {
            upwd = concat_strings(tmpu, b"@\0" as *const u8 as *const i8, 0 as *mut i8);
        }
        rpl_free(tmpu as *mut libc::c_void);
        tmpu = 0 as *mut i8;
        rpl_free(tmpp as *mut libc::c_void);
        tmpp = 0 as *mut i8;
    } else {
        upwd = xstrdup(b"\0" as *const u8 as *const i8);
    }
    htcldir = html_quote_string((*u).dir);
    fprintf(
        fp,
        b"<!DOCTYPE HTML PUBLIC \"-//IETF//DTD HTML 2.0//EN\">\n\0" as *const u8
            as *const i8,
    );
    fprintf(fp, b"<html>\n<head>\n<title>\0" as *const u8 as *const i8);
    fprintf(
        fp,
        dcgettext(
            0 as *const i8,
            b"Index of /%s on %s:%d\0" as *const u8 as *const i8,
            5 as i32,
        ),
        htcldir,
        (*u).host,
        (*u).port,
    );
    fprintf(fp, b"</title>\n</head>\n<body>\n<h1>\0" as *const u8 as *const i8);
    fprintf(
        fp,
        dcgettext(
            0 as *const i8,
            b"Index of /%s on %s:%d\0" as *const u8 as *const i8,
            5 as i32,
        ),
        htcldir,
        (*u).host,
        (*u).port,
    );
    fprintf(fp, b"</h1>\n<hr>\n<pre>\n\0" as *const u8 as *const i8);
    while !f.is_null() {
        fprintf(fp, b"  \0" as *const u8 as *const i8);
        if (*f).tstamp != -(1 as i32) as i64 {
            static mut months: [*const i8; 12] = [
                b"Jan\0" as *const u8 as *const i8,
                b"Feb\0" as *const u8 as *const i8,
                b"Mar\0" as *const u8 as *const i8,
                b"Apr\0" as *const u8 as *const i8,
                b"May\0" as *const u8 as *const i8,
                b"Jun\0" as *const u8 as *const i8,
                b"Jul\0" as *const u8 as *const i8,
                b"Aug\0" as *const u8 as *const i8,
                b"Sep\0" as *const u8 as *const i8,
                b"Oct\0" as *const u8 as *const i8,
                b"Nov\0" as *const u8 as *const i8,
                b"Dec\0" as *const u8 as *const i8,
            ];
            let mut tstamp: time_t = (*f).tstamp;
            let mut ptm: *mut tm = localtime(&mut tstamp);
            fprintf(
                fp,
                b"%d %s %02d \0" as *const u8 as *const i8,
                (*ptm).tm_year + 1900 as i32,
                months[(*ptm).tm_mon as usize],
                (*ptm).tm_mday,
            );
            if (*f).ptype as u32 == parsetype::TT_HOUR_MIN as i32 as u32 {
                fprintf(
                    fp,
                    b"%02d:%02d  \0" as *const u8 as *const i8,
                    (*ptm).tm_hour,
                    (*ptm).tm_min,
                );
            } else {
                fprintf(fp, b"       \0" as *const u8 as *const i8);
            }
        } else {
            fprintf(
                fp,
                dcgettext(
                    0 as *const i8,
                    b"time unknown       \0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        match (*f).type_0 as u32 {
            0 => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const i8,
                        b"File        \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            1 => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const i8,
                        b"Directory   \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            2 => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const i8,
                        b"Link        \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            _ => {
                fprintf(
                    fp,
                    dcgettext(
                        0 as *const i8,
                        b"Not sure    \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
        }
        htclfile = html_quote_string((*f).name);
        urlclfile = url_escape_unsafe_and_reserved((*f).name);
        fprintf(
            fp,
            b"<a href=\"ftp://%s%s:%d\0" as *const u8 as *const i8,
            upwd,
            (*u).host,
            (*u).port,
        );
        if *(*u).dir as i32 != '/' as i32 {
            _IO_putc('/' as i32, fp);
        }
        fprintf(fp, b"%s\0" as *const u8 as *const i8, htcldir);
        if *(*u).dir != 0 {
            _IO_putc('/' as i32, fp);
        }
        fprintf(fp, b"%s\0" as *const u8 as *const i8, urlclfile);
        if (*f).type_0 as u32 == ftype::FT_DIRECTORY as i32 as u32 {
            _IO_putc('/' as i32, fp);
        }
        fprintf(fp, b"\">%s\0" as *const u8 as *const i8, htclfile);
        if (*f).type_0 as u32 == ftype::FT_DIRECTORY as i32 as u32 {
            _IO_putc('/' as i32, fp);
        }
        fprintf(fp, b"</a> \0" as *const u8 as *const i8);
        if (*f).type_0 as u32 == ftype::FT_PLAINFILE as i32 as u32 {
            fprintf(
                fp,
                dcgettext(
                    0 as *const i8,
                    b" (%s bytes)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                number_to_static_string((*f).size),
            );
        } else if (*f).type_0 as u32 == ftype::FT_SYMLINK as i32 as u32 {
            fprintf(
                fp,
                b"-> %s\0" as *const u8 as *const i8,
                if !((*f).linkto).is_null() {
                    (*f).linkto
                } else {
                    b"(nil)\0" as *const u8 as *const i8
                },
            );
        }
        _IO_putc('\n' as i32, fp);
        rpl_free(htclfile as *mut libc::c_void);
        htclfile = 0 as *mut i8;
        rpl_free(urlclfile as *mut libc::c_void);
        urlclfile = 0 as *mut i8;
        f = (*f).next;
    }
    fprintf(fp, b"</pre>\n</body>\n</html>\n\0" as *const u8 as *const i8);
    rpl_free(htcldir as *mut libc::c_void);
    htcldir = 0 as *mut i8;
    rpl_free(upwd as *mut libc::c_void);
    upwd = 0 as *mut i8;
    if output_stream.is_null() {
        fclose(fp);
    } else {
        rpl_fflush(fp);
    }
    return uerr_t::FTPOK;
}