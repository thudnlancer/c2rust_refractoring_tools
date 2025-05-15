use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn rpl_strtoll(
        string: *const i8,
        endptr: *mut *mut i8,
        base: i32,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn quotearg_style(s: quoting_style, arg: *const i8) -> *mut i8;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn __errno_location() -> *mut i32;
    fn concat_strings(_: *const i8, _: ...) -> *mut i8;
    fn number_to_static_string(_: wgint) -> *mut i8;
    fn fd_close(_: i32);
    fn fd_write(_: i32, _: *mut i8, _: i32, _: libc::c_double) -> i32;
    fn bind_local(_: *const ip_address, _: *mut i32) -> i32;
    fn socket_ip_address(_: i32, _: *mut ip_address, _: i32) -> bool;
    fn print_address(_: *const ip_address) -> *const i8;
    fn skey_response(_: i32, _: *const i8, _: *const i8) -> *const i8;
    fn fd_read_line(_: i32) -> *mut i8;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn c_strncasecmp(s1: *const i8, s2: *const i8, n: size_t) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type size_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub enum quoting_style {
    custom_quoting_style = 10,
    clocale_quoting_style = 9,
    locale_quoting_style = 8,
    escape_quoting_style = 7,
    c_maybe_quoting_style = 6,
    c_quoting_style = 5,
    shell_escape_always_quoting_style = 4,
    shell_escape_quoting_style = 3,
    shell_always_quoting_style = 2,
    shell_quoting_style = 1,
    literal_quoting_style = 0,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::custom_quoting_style => 10,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::locale_quoting_style => 8,
            quoting_style::escape_quoting_style => 7,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::c_quoting_style => 5,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_quoting_style => 1,
            quoting_style::literal_quoting_style => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            10 => quoting_style::custom_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            8 => quoting_style::locale_quoting_style,
            7 => quoting_style::escape_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            5 => quoting_style::c_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            1 => quoting_style::shell_quoting_style,
            0 => quoting_style::literal_quoting_style,
            _ => panic!("Invalid value for quoting_style: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_style {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_style {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_style {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_style {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_style {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_style {
    type Output = quoting_style;
    fn add(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_style {
    type Output = quoting_style;
    fn sub(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_style {
    type Output = quoting_style;
    fn mul(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_style {
    type Output = quoting_style;
    fn div(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_style {
    type Output = quoting_style;
    fn rem(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_address {
    pub family: i32,
    pub data: C2RustUnnamed_5,
    pub ipv6_scope: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_6 {
    ENDPOINT_PEER = 1,
    ENDPOINT_LOCAL = 0,
}
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_6::ENDPOINT_PEER => 1,
            C2RustUnnamed_6::ENDPOINT_LOCAL => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_6 {
        match value {
            1 => C2RustUnnamed_6::ENDPOINT_PEER,
            0 => C2RustUnnamed_6::ENDPOINT_LOCAL,
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
pub enum ustype {
    UST_OTHER = 2,
    UST_MULTINET = 1,
    UST_TYPE_L8 = 0,
}
impl ustype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ustype::UST_OTHER => 2,
            ustype::UST_MULTINET => 1,
            ustype::UST_TYPE_L8 => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> ustype {
        match value {
            2 => ustype::UST_OTHER,
            1 => ustype::UST_MULTINET,
            0 => ustype::UST_TYPE_L8,
            _ => panic!("Invalid value for ustype: {}", value),
        }
    }
}
impl AddAssign<u32> for ustype {
    fn add_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ustype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ustype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ustype {
    fn div_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ustype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ustype {
    type Output = ustype;
    fn add(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ustype {
    type Output = ustype;
    fn sub(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ustype {
    type Output = ustype;
    fn mul(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ustype {
    type Output = ustype;
    fn div(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ustype {
    type Output = ustype;
    fn rem(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum prot_level {
    PROT_PRIVATE = 80,
    PROT_CONFIDENTIAL = 69,
    PROT_SAFE = 83,
    PROT_CLEAR = 67,
}
impl prot_level {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            prot_level::PROT_PRIVATE => 80,
            prot_level::PROT_CONFIDENTIAL => 69,
            prot_level::PROT_SAFE => 83,
            prot_level::PROT_CLEAR => 67,
        }
    }
    fn from_libc_c_uint(value: u32) -> prot_level {
        match value {
            80 => prot_level::PROT_PRIVATE,
            69 => prot_level::PROT_CONFIDENTIAL,
            83 => prot_level::PROT_SAFE,
            67 => prot_level::PROT_CLEAR,
            _ => panic!("Invalid value for prot_level: {}", value),
        }
    }
}
impl AddAssign<u32> for prot_level {
    fn add_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for prot_level {
    fn sub_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for prot_level {
    fn mul_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for prot_level {
    fn div_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for prot_level {
    fn rem_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for prot_level {
    type Output = prot_level;
    fn add(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for prot_level {
    type Output = prot_level;
    fn sub(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for prot_level {
    type Output = prot_level;
    fn mul(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for prot_level {
    type Output = prot_level;
    fn div(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for prot_level {
    type Output = prot_level;
    fn rem(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
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
#[no_mangle]
pub unsafe extern "C" fn ftp_response(
    mut fd: i32,
    mut ret_line: *mut *mut i8,
) -> uerr_t {
    loop {
        let mut p: *mut i8 = 0 as *mut i8;
        let mut line: *mut i8 = fd_read_line(fd);
        if line.is_null() {
            return uerr_t::FTPRERR;
        }
        p = strpbrk(line, b"\r\n\0" as *const u8 as *const i8);
        if !p.is_null() {
            *p = 0 as i32 as i8;
        }
        if opt.server_response {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s\n\0" as *const u8 as *const i8,
                quotearg_style(quoting_style::escape_quoting_style, line),
            );
        } else if opt.debug as i64 != 0 {
            debug_logprintf(
                b"%s\n\0" as *const u8 as *const i8,
                quotearg_style(quoting_style::escape_quoting_style, line),
            );
        }
        if c_isdigit(*line.offset(0 as i32 as isize) as i32) as i32 != 0
            && c_isdigit(*line.offset(1 as i32 as isize) as i32) as i32 != 0
            && c_isdigit(*line.offset(2 as i32 as isize) as i32) as i32 != 0
            && *line.offset(3 as i32 as isize) as i32 == ' ' as i32
        {
            *ret_line = line;
            return uerr_t::FTPOK;
        }
        rpl_free(line as *mut libc::c_void);
        line = 0 as *mut i8;
    };
}
unsafe extern "C" fn ftp_request(
    mut command: *const i8,
    mut value: *const i8,
) -> *mut i8 {
    let mut res: *mut i8 = 0 as *mut i8;
    if !value.is_null() {
        let mut defanged: *mut i8 = 0 as *mut i8;
        let mut buf: [i8; 256] = [0; 256];
        if !(strpbrk(value, b"\r\n\0" as *const u8 as *const i8)).is_null() {
            let mut p: *mut i8 = 0 as *mut i8;
            let mut len: size_t = strlen(value);
            if len < ::core::mem::size_of::<[i8; 256]>() as u64 {
                defanged = buf.as_mut_ptr();
            } else {
                defanged = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
            }
            memcpy(
                defanged as *mut libc::c_void,
                value as *const libc::c_void,
                len.wrapping_add(1 as i32 as u64),
            );
            p = defanged;
            while *p != 0 {
                if *p as i32 == '\r' as i32 || *p as i32 == '\n' as i32 {
                    *p = ' ' as i32 as i8;
                }
                p = p.offset(1);
                p;
            }
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"\nDetected newlines in %s \"%s\"; changing to %s \"%s\"\n\0"
                        as *const u8 as *const i8,
                    command,
                    quotearg_style(quoting_style::escape_quoting_style, value),
                    command,
                    quotearg_style(quoting_style::escape_quoting_style, defanged),
                );
            }
            value = defanged;
        }
        res = concat_strings(
            command,
            b" \0" as *const u8 as *const i8,
            value,
            b"\r\n\0" as *const u8 as *const i8,
            0 as *mut i8,
        );
        if defanged != buf.as_mut_ptr() {
            rpl_free(defanged as *mut libc::c_void);
            defanged = 0 as *mut i8;
        }
    } else {
        res = concat_strings(command, b"\r\n\0" as *const u8 as *const i8, 0 as *mut i8);
    }
    if opt.server_response {
        if strncmp(res, b"PASS\0" as *const u8 as *const i8, 4 as i32 as u64) != 0 as i32
        {
            logprintf(
                log_options::LOG_ALWAYS,
                b"--> %s\n\0" as *const u8 as *const i8,
                res,
            );
        } else {
            logputs(
                log_options::LOG_ALWAYS,
                b"--> PASS Turtle Power!\n\n\0" as *const u8 as *const i8,
            );
        }
    } else if opt.debug as i64 != 0 {
        debug_logprintf(b"\n--> %s\n\0" as *const u8 as *const i8, res);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_greeting(mut csock: i32) -> uerr_t {
    let mut err: uerr_t = uerr_t::FTPOK;
    let mut response: *mut i8 = 0 as *mut i8;
    err = ftp_response(csock, &mut response);
    if !(err as u32 != uerr_t::FTPOK as i32 as u32) {
        if *response as i32 != '2' as i32 {
            err = uerr_t::FTPSRVERR;
        }
    }
    if !response.is_null() {
        rpl_free(response as *mut libc::c_void);
        response = 0 as *mut i8;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_login(
    mut csock: i32,
    mut acc: *const i8,
    mut pass: *const i8,
) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    request = ftp_request(b"USER\0" as *const u8 as *const i8, acc);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 == '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPOK;
    }
    if *respline as i32 != '3' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPLOGREFUSED;
    }
    static mut skey_head: [*const i8; 2] = [
        b"331 s/key \0" as *const u8 as *const i8,
        b"331 opiekey \0" as *const u8 as *const i8,
    ];
    let mut i: size_t = 0;
    let mut seed: *const i8 = 0 as *const i8;
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[*const i8; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
    {
        let mut l: i32 = strlen(skey_head[i as usize]) as i32;
        if 0 as i32 == c_strncasecmp(skey_head[i as usize], respline, l as size_t) {
            seed = respline.offset(l as isize);
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if !seed.is_null() {
        let mut skey_sequence: i32 = 0 as i32;
        while c_isdigit(*seed as i32) {
            skey_sequence = 10 as i32 * skey_sequence + *seed as i32 - '0' as i32;
            seed = seed.offset(1);
            seed;
        }
        if *seed as i32 == ' ' as i32 {
            seed = seed.offset(1);
            seed;
        } else {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut i8;
            return uerr_t::FTPLOGREFUSED;
        }
        pass = skey_response(skey_sequence, seed, pass);
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    request = ftp_request(b"PASS\0" as *const u8 as *const i8, pass);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPLOGINC;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
unsafe extern "C" fn ip_address_to_port_repr(
    mut addr: *const ip_address,
    mut port: i32,
    mut buf: *mut i8,
    mut buflen: size_t,
) {
    let mut ptr: *mut u8 = 0 as *mut u8;
    ptr = &(*addr).data as *const C2RustUnnamed_5 as *mut libc::c_void as *mut u8;
    snprintf(
        buf,
        buflen,
        b"%d,%d,%d,%d,%d,%d\0" as *const u8 as *const i8,
        *ptr.offset(0 as i32 as isize) as i32,
        *ptr.offset(1 as i32 as isize) as i32,
        *ptr.offset(2 as i32 as isize) as i32,
        *ptr.offset(3 as i32 as isize) as i32,
        (port & 0xff00 as i32) >> 8 as i32,
        port & 0xff as i32,
    );
    *buf.offset(buflen.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_port(mut csock: i32, mut local_sock: *mut i32) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut addr: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_5 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut nwritten: i32 = 0;
    let mut port: i32 = 0;
    let mut bytes: [i8; 25] = [0; 25];
    if !socket_ip_address(csock, &mut addr, C2RustUnnamed_6::ENDPOINT_LOCAL as i32) {
        return uerr_t::FTPSYSERR;
    }
    port = 0 as i32;
    *local_sock = bind_local(&mut addr, &mut port);
    if *local_sock < 0 as i32 {
        return uerr_t::FTPSYSERR;
    }
    ip_address_to_port_repr(
        &mut addr,
        port,
        bytes.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 25]>() as u64,
    );
    request = ftp_request(b"PORT\0" as *const u8 as *const i8, bytes.as_mut_ptr());
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        fd_close(*local_sock);
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        fd_close(*local_sock);
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        fd_close(*local_sock);
        return uerr_t::FTPPORTERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
unsafe extern "C" fn ip_address_to_lprt_repr(
    mut addr: *const ip_address,
    mut port: i32,
    mut buf: *mut i8,
    mut buflen: size_t,
) {
    let mut ptr: *mut u8 = &(*addr).data as *const C2RustUnnamed_5 as *mut libc::c_void
        as *mut u8;
    match (*addr).family {
        2 => {
            snprintf(
                buf,
                buflen,
                b"%d,%d,%d,%d,%d,%d,%d,%d,%d\0" as *const u8 as *const i8,
                4 as i32,
                4 as i32,
                *ptr.offset(0 as i32 as isize) as i32,
                *ptr.offset(1 as i32 as isize) as i32,
                *ptr.offset(2 as i32 as isize) as i32,
                *ptr.offset(3 as i32 as isize) as i32,
                2 as i32,
                (port & 0xff00 as i32) >> 8 as i32,
                port & 0xff as i32,
            );
        }
        10 => {
            snprintf(
                buf,
                buflen,
                b"%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d\0"
                    as *const u8 as *const i8,
                6 as i32,
                16 as i32,
                *ptr.offset(0 as i32 as isize) as i32,
                *ptr.offset(1 as i32 as isize) as i32,
                *ptr.offset(2 as i32 as isize) as i32,
                *ptr.offset(3 as i32 as isize) as i32,
                *ptr.offset(4 as i32 as isize) as i32,
                *ptr.offset(5 as i32 as isize) as i32,
                *ptr.offset(6 as i32 as isize) as i32,
                *ptr.offset(7 as i32 as isize) as i32,
                *ptr.offset(8 as i32 as isize) as i32,
                *ptr.offset(9 as i32 as isize) as i32,
                *ptr.offset(10 as i32 as isize) as i32,
                *ptr.offset(11 as i32 as isize) as i32,
                *ptr.offset(12 as i32 as isize) as i32,
                *ptr.offset(13 as i32 as isize) as i32,
                *ptr.offset(14 as i32 as isize) as i32,
                *ptr.offset(15 as i32 as isize) as i32,
                2 as i32,
                (port & 0xff00 as i32) >> 8 as i32,
                port & 0xff as i32,
            );
        }
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ftp_lprt(mut csock: i32, mut local_sock: *mut i32) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut addr: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_5 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut nwritten: i32 = 0;
    let mut port: i32 = 0;
    let mut bytes: [i8; 85] = [0; 85];
    if !socket_ip_address(csock, &mut addr, C2RustUnnamed_6::ENDPOINT_LOCAL as i32) {
        return uerr_t::FTPSYSERR;
    }
    port = 0 as i32;
    *local_sock = bind_local(&mut addr, &mut port);
    if *local_sock < 0 as i32 {
        return uerr_t::FTPSYSERR;
    }
    ip_address_to_lprt_repr(
        &mut addr,
        port,
        bytes.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 85]>() as u64,
    );
    request = ftp_request(b"LPRT\0" as *const u8 as *const i8, bytes.as_mut_ptr());
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        fd_close(*local_sock);
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        fd_close(*local_sock);
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        fd_close(*local_sock);
        return uerr_t::FTPPORTERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
unsafe extern "C" fn ip_address_to_eprt_repr(
    mut addr: *const ip_address,
    mut port: i32,
    mut buf: *mut i8,
    mut buflen: size_t,
) {
    let mut afnum: i32 = 0;
    afnum = if (*addr).family == 2 as i32 { 1 as i32 } else { 2 as i32 };
    snprintf(
        buf,
        buflen,
        b"|%d|%s|%d|\0" as *const u8 as *const i8,
        afnum,
        print_address(addr),
        port,
    );
    *buf.offset(buflen.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_eprt(mut csock: i32, mut local_sock: *mut i32) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut addr: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_5 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut nwritten: i32 = 0;
    let mut port: i32 = 0;
    let mut bytes: [i8; 57] = [0; 57];
    if !socket_ip_address(csock, &mut addr, C2RustUnnamed_6::ENDPOINT_LOCAL as i32) {
        return uerr_t::FTPSYSERR;
    }
    port = 0 as i32;
    *local_sock = bind_local(&mut addr, &mut port);
    if *local_sock < 0 as i32 {
        return uerr_t::FTPSYSERR;
    }
    ip_address_to_eprt_repr(
        &mut addr,
        port,
        bytes.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 57]>() as u64,
    );
    request = ftp_request(b"EPRT\0" as *const u8 as *const i8, bytes.as_mut_ptr());
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        fd_close(*local_sock);
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        fd_close(*local_sock);
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        fd_close(*local_sock);
        return uerr_t::FTPPORTERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_auth(mut csock: i32, mut scheme: url_scheme) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut written: i32 = 0 as i32;
    let mut request: *mut i8 = 0 as *mut i8;
    let mut response: *mut i8 = 0 as *mut i8;
    if scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32 {
        request = ftp_request(
            b"AUTH\0" as *const u8 as *const i8,
            b"TLS\0" as *const u8 as *const i8,
        );
        written = fd_write(
            csock,
            request,
            strlen(request) as i32,
            -(1 as i32) as libc::c_double,
        );
        if written < 0 as i32 {
            err = uerr_t::WRITEFAILED;
        } else {
            err = ftp_response(csock, &mut response);
            if !(err as u32 != uerr_t::FTPOK as i32 as u32) {
                if *response as i32 != '2' as i32 {
                    err = uerr_t::FTPNOAUTH;
                }
            }
        }
    } else {
        err = uerr_t::FTPNOAUTH;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    rpl_free(response as *mut libc::c_void);
    response = 0 as *mut i8;
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_pbsz(mut csock: i32, mut pbsz: i32) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut written: i32 = 0 as i32;
    let mut spbsz: [i8; 5] = [0; 5];
    let mut request: *mut i8 = 0 as *mut i8;
    let mut response: *mut i8 = 0 as *mut i8;
    snprintf(
        spbsz.as_mut_ptr(),
        5 as i32 as u64,
        b"%d\0" as *const u8 as *const i8,
        pbsz,
    );
    request = ftp_request(b"PBSZ\0" as *const u8 as *const i8, spbsz.as_mut_ptr());
    written = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if written < 0 as i32 {
        err = uerr_t::WRITEFAILED;
    } else {
        err = ftp_response(csock, &mut response);
        if !(err as u32 != uerr_t::FTPOK as i32 as u32) {
            if *response as i32 != '2' as i32 {
                err = uerr_t::FTPNOPBSZ;
            }
        }
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    rpl_free(response as *mut libc::c_void);
    response = 0 as *mut i8;
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_prot(mut csock: i32, mut prot: prot_level) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut written: i32 = 0 as i32;
    let mut request: *mut i8 = 0 as *mut i8;
    let mut response: *mut i8 = 0 as *mut i8;
    let mut value: [i8; 2] = [0; 2];
    value[0 as i32 as usize] = prot as i8;
    value[1 as i32 as usize] = '\0' as i32 as i8;
    request = ftp_request(b"PROT\0" as *const u8 as *const i8, value.as_mut_ptr());
    written = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if written < 0 as i32 {
        err = uerr_t::WRITEFAILED;
    } else {
        err = ftp_response(csock, &mut response);
        if !(err as u32 != uerr_t::FTPOK as i32 as u32) {
            if *response as i32 != '2' as i32 {
                err = uerr_t::FTPNOPROT;
            }
        }
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    rpl_free(response as *mut libc::c_void);
    response = 0 as *mut i8;
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_pasv(
    mut csock: i32,
    mut addr: *mut ip_address,
    mut port: *mut i32,
) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut i: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut tmp: [u8; 6] = [0; 6];
    memset(
        addr as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<ip_address>() as u64,
    );
    request = ftp_request(b"PASV\0" as *const u8 as *const i8, 0 as *const i8);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPNOPASV;
    }
    s = respline;
    s = s.offset(4 as i32 as isize);
    while *s as i32 != 0 && !c_isdigit(*s as i32) {
        s = s.offset(1);
        s;
    }
    if *s == 0 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    i = 0 as i32;
    while i < 6 as i32 {
        tmp[i as usize] = 0 as i32 as u8;
        while c_isdigit(*s as i32) {
            tmp[i as usize] = (*s as i32 - '0' as i32
                + 10 as i32 * tmp[i as usize] as i32) as u8;
            s = s.offset(1);
            s;
        }
        if *s as i32 == ',' as i32 {
            s = s.offset(1);
            s;
        } else if i < 5 as i32 {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut i8;
            return uerr_t::FTPINVPASV;
        }
        i += 1;
        i;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    (*addr).family = 2 as i32;
    memcpy(
        &mut (*addr).data as *mut C2RustUnnamed_5 as *mut libc::c_void,
        tmp.as_mut_ptr() as *const libc::c_void,
        4 as i32 as u64,
    );
    *port = ((tmp[4 as i32 as usize] as i32) << 8 as i32 & 0xff00 as i32)
        + tmp[5 as i32 as usize] as i32;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_lpsv(
    mut csock: i32,
    mut addr: *mut ip_address,
    mut port: *mut i32,
) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut i: i32 = 0;
    let mut af: i32 = 0;
    let mut addrlen: i32 = 0;
    let mut portlen: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut tmp: [u8; 16] = [0; 16];
    let mut tmpprt: [u8; 2] = [0; 2];
    memset(
        addr as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<ip_address>() as u64,
    );
    request = ftp_request(b"LPSV\0" as *const u8 as *const i8, 0 as *const i8);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPNOPASV;
    }
    s = respline;
    s = s.offset(4 as i32 as isize);
    while *s as i32 != 0 && !c_isdigit(*s as i32) {
        s = s.offset(1);
        s;
    }
    if *s == 0 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    af = 0 as i32;
    while c_isdigit(*s as i32) {
        af = *s as i32 - '0' as i32 + 10 as i32 * af;
        s = s.offset(1);
        s;
    }
    if af != 4 as i32 && af != 6 as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    if *s == 0
        || {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 as i32 != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    addrlen = 0 as i32;
    while c_isdigit(*s as i32) {
        addrlen = *s as i32 - '0' as i32 + 10 as i32 * addrlen;
        s = s.offset(1);
        s;
    }
    if *s == 0
        || {
            let fresh1 = s;
            s = s.offset(1);
            *fresh1 as i32 != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    if addrlen > 16 as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    if af == 4 as i32 && addrlen != 4 as i32 || af == 6 as i32 && addrlen != 16 as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    i = 0 as i32;
    while i < addrlen {
        tmp[i as usize] = 0 as i32 as u8;
        while c_isdigit(*s as i32) {
            tmp[i as usize] = (*s as i32 - '0' as i32
                + 10 as i32 * tmp[i as usize] as i32) as u8;
            s = s.offset(1);
            s;
        }
        if *s as i32 == ',' as i32 {
            s = s.offset(1);
            s;
        } else {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut i8;
            return uerr_t::FTPINVPASV;
        }
        i += 1;
        i;
    }
    portlen = 0 as i32;
    while c_isdigit(*s as i32) {
        portlen = *s as i32 - '0' as i32 + 10 as i32 * portlen;
        s = s.offset(1);
        s;
    }
    if *s == 0
        || {
            let fresh2 = s;
            s = s.offset(1);
            *fresh2 as i32 != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    if portlen > 2 as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    tmpprt[0 as i32 as usize] = 0 as i32 as u8;
    while c_isdigit(*s as i32) {
        tmpprt[0 as i32 as usize] = (*s as i32 - '0' as i32
            + 10 as i32 * tmpprt[0 as i32 as usize] as i32) as u8;
        s = s.offset(1);
        s;
    }
    if *s == 0
        || {
            let fresh3 = s;
            s = s.offset(1);
            *fresh3 as i32 != ',' as i32
        }
    {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    tmpprt[1 as i32 as usize] = 0 as i32 as u8;
    while c_isdigit(*s as i32) {
        tmpprt[1 as i32 as usize] = (*s as i32 - '0' as i32
            + 10 as i32 * tmpprt[1 as i32 as usize] as i32) as u8;
        s = s.offset(1);
        s;
    }
    if af == 4 as i32 {
        (*addr).family = 2 as i32;
        memcpy(
            &mut (*addr).data as *mut C2RustUnnamed_5 as *mut libc::c_void,
            tmp.as_mut_ptr() as *const libc::c_void,
            4 as i32 as u64,
        );
        *port = ((tmpprt[0 as i32 as usize] as i32) << 8 as i32 & 0xff00 as i32)
            + tmpprt[1 as i32 as usize] as i32;
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"lpsv addr is: %s\n\0" as *const u8 as *const i8,
                print_address(addr),
            );
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"tmpprt[0] is: %d\n\0" as *const u8 as *const i8,
                tmpprt[0 as i32 as usize] as i32,
            );
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"tmpprt[1] is: %d\n\0" as *const u8 as *const i8,
                tmpprt[1 as i32 as usize] as i32,
            );
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(b"*port is: %d\n\0" as *const u8 as *const i8, *port);
        }
    } else {
        (*addr).family = 10 as i32;
        memcpy(
            &mut (*addr).data as *mut C2RustUnnamed_5 as *mut libc::c_void,
            tmp.as_mut_ptr() as *const libc::c_void,
            16 as i32 as u64,
        );
        *port = ((tmpprt[0 as i32 as usize] as i32) << 8 as i32 & 0xff00 as i32)
            + tmpprt[1 as i32 as usize] as i32;
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"lpsv addr is: %s\n\0" as *const u8 as *const i8,
                print_address(addr),
            );
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"tmpprt[0] is: %d\n\0" as *const u8 as *const i8,
                tmpprt[0 as i32 as usize] as i32,
            );
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"tmpprt[1] is: %d\n\0" as *const u8 as *const i8,
                tmpprt[1 as i32 as usize] as i32,
            );
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(b"*port is: %d\n\0" as *const u8 as *const i8, *port);
        }
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_epsv(
    mut csock: i32,
    mut ip: *mut ip_address,
    mut port: *mut i32,
) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut start: *mut i8 = 0 as *mut i8;
    let mut delim: i8 = 0;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut i: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut tport: i32 = 0;
    request = ftp_request(
        b"EPSV\0" as *const u8 as *const i8,
        if (*ip).family == 2 as i32 {
            b"1\0" as *const u8 as *const i8
        } else {
            b"2\0" as *const u8 as *const i8
        },
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPNOPASV;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"respline is %s\n\0" as *const u8 as *const i8, respline);
    }
    start = strchr(respline, '(' as i32);
    if start.is_null() {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    s = start.offset(1 as i32 as isize);
    let fresh4 = s;
    s = s.offset(1);
    delim = *fresh4;
    if (delim as i32) < 33 as i32 || delim as i32 > 126 as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    i = 0 as i32;
    while i < 2 as i32 {
        let fresh5 = s;
        s = s.offset(1);
        if *fresh5 as i32 != delim as i32 {
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut i8;
            return uerr_t::FTPINVPASV;
        }
        i += 1;
        i;
    }
    tport = 0 as i32;
    i = 0 as i32;
    while i < 5 as i32 && c_isdigit(*s as i32) as i32 != 0 {
        tport = *s as i32 - '0' as i32 + 10 as i32 * tport;
        i += 1;
        i;
        s = s.offset(1);
        s;
    }
    let fresh6 = s;
    s = s.offset(1);
    if *fresh6 as i32 != delim as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    if *s as i32 != ')' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPINVPASV;
    }
    *port = tport;
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_type(mut csock: i32, mut type_0: i32) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut stype: [i8; 2] = [0; 2];
    stype[0 as i32 as usize] = type_0 as i8;
    stype[1 as i32 as usize] = 0 as i32 as i8;
    request = ftp_request(b"TYPE\0" as *const u8 as *const i8, stype.as_mut_ptr());
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPUNKNOWNTYPE;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_cwd(mut csock: i32, mut dir: *const i8) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    request = ftp_request(b"CWD\0" as *const u8 as *const i8, dir);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPNSFOD;
    }
    if *respline as i32 != '2' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPRERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_rest(mut csock: i32, mut offset: wgint) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    request = ftp_request(
        b"REST\0" as *const u8 as *const i8,
        number_to_static_string(offset),
    );
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 != '3' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPRESTFAIL;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_retr(mut csock: i32, mut file: *const i8) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    request = ftp_request(b"RETR\0" as *const u8 as *const i8, file);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPNSFOD;
    }
    if *respline as i32 != '1' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPRERR;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_list(
    mut csock: i32,
    mut file: *const i8,
    mut avoid_list_a: bool,
    mut avoid_list: bool,
    mut list_a_used: *mut bool,
) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut ok: bool = 0 as i32 != 0;
    let mut i: size_t = 0 as i32 as size_t;
    static mut list_commands: [*const i8; 2] = [
        b"LIST -a\0" as *const u8 as *const i8,
        b"LIST\0" as *const u8 as *const i8,
    ];
    *list_a_used = 0 as i32 != 0;
    if avoid_list_a {
        i = (::core::mem::size_of::<[*const i8; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
            .wrapping_sub(1 as i32 as u64);
        if opt.debug as i64 != 0 {
            debug_logprintf(b"(skipping \"LIST -a\")\0" as *const u8 as *const i8);
        }
    }
    loop {
        request = ftp_request(list_commands[i as usize], file);
        nwritten = fd_write(
            csock,
            request,
            strlen(request) as i32,
            -(1 as i32) as libc::c_double,
        );
        if nwritten < 0 as i32 {
            rpl_free(request as *mut libc::c_void);
            request = 0 as *mut i8;
            return uerr_t::WRITEFAILED;
        }
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        err = ftp_response(csock, &mut respline);
        if err as u32 == uerr_t::FTPOK as i32 as u32 {
            if *respline as i32 == '5' as i32 {
                err = uerr_t::FTPNSFOD;
            } else if *respline as i32 == '1' as i32 {
                err = uerr_t::FTPOK;
                ok = 1 as i32 != 0;
                *list_a_used = i == 0 as i32 as u64;
            } else {
                err = uerr_t::FTPRERR;
            }
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut i8;
        }
        i = i.wrapping_add(1);
        i;
        if avoid_list as i32 != 0 && i == 1 as i32 as u64 {
            i = i.wrapping_add(1);
            i;
            if opt.debug as i64 != 0 {
                debug_logprintf(b"(skipping \"LIST\")\0" as *const u8 as *const i8);
            }
        }
        if !(i
            < (::core::mem::size_of::<[*const i8; 2]>() as u64)
                .wrapping_div(::core::mem::size_of::<*const i8>() as u64) && !ok)
        {
            break;
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_syst(
    mut csock: i32,
    mut server_type: *mut stype,
    mut unix_type: *mut ustype,
) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut ftp_last_respline: *mut i8 = 0 as *mut i8;
    request = ftp_request(b"SYST\0" as *const u8 as *const i8, 0 as *const i8);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if *respline as i32 == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        return uerr_t::FTPSRVERR;
    }
    ftp_last_respline = strdup(respline);
    strtok(respline, b" \0" as *const u8 as *const i8);
    request = strtok(0 as *mut i8, b" \0" as *const u8 as *const i8);
    *unix_type = ustype::UST_OTHER;
    if request.is_null() {
        *server_type = stype::ST_OTHER;
    } else if c_strcasecmp(request, b"VMS\0" as *const u8 as *const i8) == 0 {
        *server_type = stype::ST_VMS;
    } else if c_strcasecmp(request, b"UNIX\0" as *const u8 as *const i8) == 0 {
        *server_type = stype::ST_UNIX;
        if c_strncasecmp(
            ftp_last_respline,
            b"215 UNIX Type: L8\0" as *const u8 as *const i8,
            17 as i32 as size_t,
        ) == 0
        {
            *unix_type = ustype::UST_TYPE_L8;
        } else if c_strncasecmp(
            ftp_last_respline,
            b"215 UNIX MultiNet Unix Emulation V5.3(93)\0" as *const u8 as *const i8,
            41 as i32 as size_t,
        ) == 0
        {
            *unix_type = ustype::UST_MULTINET;
        }
    } else if c_strcasecmp(request, b"WINDOWS_NT\0" as *const u8 as *const i8) == 0
        || c_strcasecmp(request, b"WINDOWS2000\0" as *const u8 as *const i8) == 0
    {
        *server_type = stype::ST_WINNT;
    } else if c_strcasecmp(request, b"MACOS\0" as *const u8 as *const i8) == 0 {
        *server_type = stype::ST_MACOS;
    } else if c_strcasecmp(request, b"OS/400\0" as *const u8 as *const i8) == 0 {
        *server_type = stype::ST_OS400;
    } else {
        *server_type = stype::ST_OTHER;
    }
    rpl_free(ftp_last_respline as *mut libc::c_void);
    ftp_last_respline = 0 as *mut i8;
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_pwd(mut csock: i32, mut pwd: *mut *mut i8) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    request = ftp_request(b"PWD\0" as *const u8 as *const i8, 0 as *const i8);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        return err;
    }
    if !(*respline as i32 == '5' as i32) {
        strtok(respline, b"\"\0" as *const u8 as *const i8);
        request = strtok(0 as *mut i8, b"\"\0" as *const u8 as *const i8);
        if !request.is_null() {
            rpl_free(*pwd as *mut libc::c_void);
            *pwd = 0 as *mut i8;
            *pwd = xstrdup(request);
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut i8;
            return uerr_t::FTPOK;
        }
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPSRVERR;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_size(
    mut csock: i32,
    mut file: *const i8,
    mut size: *mut wgint,
) -> uerr_t {
    let mut request: *mut i8 = 0 as *mut i8;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut nwritten: i32 = 0;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    request = ftp_request(b"SIZE\0" as *const u8 as *const i8, file);
    nwritten = fd_write(
        csock,
        request,
        strlen(request) as i32,
        -(1 as i32) as libc::c_double,
    );
    if nwritten < 0 as i32 {
        rpl_free(request as *mut libc::c_void);
        request = 0 as *mut i8;
        *size = 0 as i32 as wgint;
        return uerr_t::WRITEFAILED;
    }
    rpl_free(request as *mut libc::c_void);
    request = 0 as *mut i8;
    err = ftp_response(csock, &mut respline);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        *size = 0 as i32 as wgint;
        return err;
    }
    if *respline as i32 == '5' as i32 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        *size = 0 as i32 as wgint;
        return uerr_t::FTPOK;
    }
    *__errno_location() = 0 as i32;
    *size = rpl_strtoll(respline.offset(4 as i32 as isize), 0 as *mut *mut i8, 10 as i32)
        as wgint;
    if *__errno_location() != 0 {
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        *size = 0 as i32 as wgint;
        return uerr_t::FTPOK;
    }
    rpl_free(respline as *mut libc::c_void);
    respline = 0 as *mut i8;
    return uerr_t::FTPOK;
}
#[no_mangle]
pub unsafe extern "C" fn ftp_process_type(mut params: *const i8) -> i8 {
    if !params.is_null()
        && 0 as i32
            == strncasecmp(params, b"type=\0" as *const u8 as *const i8, 5 as i32 as u64)
        && *params.offset(5 as i32 as isize) as i32 != '\0' as i32
    {
        return c_toupper(*params.offset(5 as i32 as isize) as i32) as i8
    } else {
        return 'I' as i32 as i8
    };
}