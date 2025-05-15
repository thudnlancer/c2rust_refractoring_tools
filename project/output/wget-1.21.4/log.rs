use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn quote(arg: *const i8) -> *const i8;
    static mut exec_name: *const i8;
    fn getpgrp() -> __pid_t;
    fn isatty(__fd: i32) -> i32;
    fn tcgetpgrp(__fd: i32) -> __pid_t;
    fn __errno_location() -> *mut i32;
    fn strdupdelim(_: *const i8, _: *const i8) -> *mut i8;
    fn unique_create(_: *const i8, _: bool, _: *mut *mut i8) -> *mut FILE;
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
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type pid_t = __pid_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
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
pub struct logvprintf_state {
    pub bigmsg: *mut i8,
    pub expected_size: i32,
    pub allocated: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct log_ln {
    pub static_line: [i8; 129],
    pub malloced_line: *mut i8,
    pub content: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringel {
    pub buffer: *mut i8,
    pub size: i32,
}
#[inline]
unsafe extern "C" fn c_isprint(mut c: i32) -> bool {
    match c {
        32 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
static mut logfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut stdlogfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut filelogfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut logfile: *mut i8 = 0 as *const i8 as *mut i8;
static mut shell_is_interactive: i32 = 0;
static mut warclogfp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut inhibit_logging: bool = false;
static mut save_context_p: bool = false;
static mut flush_log_p: bool = 1 as i32 != 0;
static mut needs_flushing: bool = false;
static mut log_lines: [log_ln; 24] = [log_ln {
    static_line: [0; 129],
    malloced_line: 0 as *const i8 as *mut i8,
    content: 0 as *const i8 as *mut i8,
}; 24];
static mut log_line_current: i32 = -(1 as i32);
static mut trailing_line: bool = false;
unsafe extern "C" fn free_log_line(mut num: i32) {
    let mut ln: *mut log_ln = log_lines.as_mut_ptr().offset(num as isize);
    rpl_free((*ln).malloced_line as *mut libc::c_void);
    (*ln).malloced_line = 0 as *mut i8;
    (*ln).content = 0 as *mut i8;
}
unsafe extern "C" fn saved_append_1(mut start: *const i8, mut end: *const i8) {
    let mut len: i32 = end.offset_from(start) as i64 as i32;
    if len == 0 {
        return;
    }
    if !trailing_line {
        let mut ln: *mut log_ln = 0 as *mut log_ln;
        if log_line_current == -(1 as i32) {
            log_line_current = 0 as i32;
        } else {
            free_log_line(log_line_current);
        }
        ln = log_lines.as_mut_ptr().offset(log_line_current as isize);
        if len > 128 as i32 {
            (*ln).malloced_line = strdupdelim(start, end);
            (*ln).content = (*ln).malloced_line;
        } else {
            memcpy(
                ((*ln).static_line).as_mut_ptr() as *mut libc::c_void,
                start as *const libc::c_void,
                len as u64,
            );
            (*ln).static_line[len as usize] = '\0' as i32 as i8;
            (*ln).content = ((*ln).static_line).as_mut_ptr();
        }
    } else {
        let mut ln_0: *mut log_ln = log_lines
            .as_mut_ptr()
            .offset(log_line_current as isize);
        if !((*ln_0).malloced_line).is_null() {
            let mut old_len: i32 = strlen((*ln_0).malloced_line) as i32;
            (*ln_0).malloced_line = xrealloc(
                (*ln_0).malloced_line as *mut libc::c_void,
                (old_len + len + 1 as i32) as size_t,
            ) as *mut i8;
            memcpy(
                ((*ln_0).malloced_line).offset(old_len as isize) as *mut libc::c_void,
                start as *const libc::c_void,
                len as u64,
            );
            *((*ln_0).malloced_line).offset((old_len + len) as isize) = '\0' as i32
                as i8;
            (*ln_0).content = (*ln_0).malloced_line;
        } else {
            let mut old_len_0: i32 = strlen(((*ln_0).static_line).as_mut_ptr()) as i32;
            if old_len_0 + len > 128 as i32 {
                (*ln_0).malloced_line = xmalloc((old_len_0 + len + 1 as i32) as size_t)
                    as *mut i8;
                memcpy(
                    (*ln_0).malloced_line as *mut libc::c_void,
                    ((*ln_0).static_line).as_mut_ptr() as *const libc::c_void,
                    old_len_0 as u64,
                );
                memcpy(
                    ((*ln_0).malloced_line).offset(old_len_0 as isize)
                        as *mut libc::c_void,
                    start as *const libc::c_void,
                    len as u64,
                );
                *((*ln_0).malloced_line).offset((old_len_0 + len) as isize) = '\0' as i32
                    as i8;
                (*ln_0).content = (*ln_0).malloced_line;
            } else {
                memcpy(
                    ((*ln_0).static_line).as_mut_ptr().offset(old_len_0 as isize)
                        as *mut libc::c_void,
                    start as *const libc::c_void,
                    len as u64,
                );
                (*ln_0).static_line[(old_len_0 + len) as usize] = '\0' as i32 as i8;
                (*ln_0).content = ((*ln_0).static_line).as_mut_ptr();
            }
        }
    }
    trailing_line = !(*end.offset(-(1 as i32) as isize) as i32 == '\n' as i32);
    if !trailing_line {
        log_line_current += 1;
        if log_line_current >= 24 as i32 {
            log_line_current = 0 as i32;
        }
    }
}
unsafe extern "C" fn saved_append(mut s: *const i8) {
    while *s != 0 {
        let mut end: *const i8 = strchr(s, '\n' as i32);
        if end.is_null() {
            end = s.offset(strlen(s) as isize);
        } else {
            end = end.offset(1);
            end;
        }
        saved_append_1(s, end);
        s = end;
    }
}
unsafe extern "C" fn get_log_fp() -> *mut FILE {
    if inhibit_logging {
        return 0 as *mut FILE;
    }
    if !logfp.is_null() {
        return logfp;
    }
    return stderr;
}
unsafe extern "C" fn get_progress_fp() -> *mut FILE {
    if opt.show_progress == 1 as i32 {
        return stderr;
    }
    return get_log_fp();
}
unsafe extern "C" fn get_warc_log_fp() -> *mut FILE {
    if inhibit_logging {
        return 0 as *mut FILE;
    }
    if !warclogfp.is_null() {
        return warclogfp;
    }
    if !logfp.is_null() {
        return 0 as *mut FILE;
    }
    return stderr;
}
#[no_mangle]
pub unsafe extern "C" fn log_set_warc_log_fp(mut fp: *mut FILE) {
    warclogfp = fp;
}
#[no_mangle]
pub unsafe extern "C" fn logputs(mut o: log_options, mut s: *const i8) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut warcfp: *mut FILE = 0 as *mut FILE;
    let mut errno_save: i32 = *__errno_location();
    check_redirect_output();
    if o as u32 == log_options::LOG_PROGRESS as i32 as u32 {
        fp = get_progress_fp();
    } else {
        fp = get_log_fp();
    }
    *__errno_location() = errno_save;
    if fp.is_null() {
        return;
    }
    warcfp = get_warc_log_fp();
    *__errno_location() = errno_save;
    match o as u32 {
        4 => {
            if opt.show_progress == 0 {
                return;
            }
        }
        1 => {
            if opt.quiet {
                return;
            }
        }
        2 => {
            if opt.verbose != 0 || opt.quiet as i32 != 0 {
                return;
            }
        }
        0 => {
            if opt.verbose == 0 {
                return;
            }
        }
        3 | _ => {}
    }
    fputs(s, fp);
    if !warcfp.is_null() {
        fputs(s, warcfp);
    }
    if save_context_p {
        saved_append(s);
    }
    if flush_log_p {
        logflush();
    } else {
        needs_flushing = 1 as i32 != 0;
    }
    *__errno_location() = errno_save;
}
unsafe extern "C" fn log_vprintf_internal(
    mut state: *mut logvprintf_state,
    mut fmt: *const i8,
    mut args: ::core::ffi::VaList,
) -> bool {
    let mut smallmsg: [i8; 128] = [0; 128];
    let mut write_ptr: *mut i8 = smallmsg.as_mut_ptr();
    let mut available_size: i32 = ::core::mem::size_of::<[i8; 128]>() as u64 as i32;
    let mut numwritten: i32 = 0;
    let mut fp: *mut FILE = get_log_fp();
    let mut warcfp: *mut FILE = get_warc_log_fp();
    if fp.is_null() {
        return 0 as i32 != 0;
    }
    if !save_context_p && warcfp.is_null() {
        vfprintf(fp, fmt, args.as_va_list());
    } else {
        if (*state).allocated != 0 as i32 {
            write_ptr = (*state).bigmsg;
            available_size = (*state).allocated;
        }
        numwritten = vsnprintf(write_ptr, available_size as u64, fmt, args.as_va_list());
        if numwritten == -(1 as i32) {
            let mut newsize: i32 = available_size << 1 as i32;
            (*state).bigmsg = xrealloc(
                (*state).bigmsg as *mut libc::c_void,
                newsize as size_t,
            ) as *mut i8;
            (*state).allocated = newsize;
            return 0 as i32 != 0;
        } else if numwritten >= available_size {
            let mut newsize_0: i32 = numwritten + 1 as i32;
            (*state).bigmsg = xrealloc(
                (*state).bigmsg as *mut libc::c_void,
                newsize_0 as size_t,
            ) as *mut i8;
            (*state).allocated = newsize_0;
            return 0 as i32 != 0;
        }
        if save_context_p {
            saved_append(write_ptr);
        }
        fputs(write_ptr, fp);
        if !warcfp.is_null() && warcfp != fp {
            fputs(write_ptr, warcfp);
        }
        rpl_free((*state).bigmsg as *mut libc::c_void);
        (*state).bigmsg = 0 as *mut i8;
    }
    if flush_log_p {
        logflush();
    } else {
        needs_flushing = 1 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn logflush() {
    let mut fp: *mut FILE = get_log_fp();
    let mut warcfp: *mut FILE = get_warc_log_fp();
    if !fp.is_null() {
        rpl_fflush(fp);
    }
    if !warcfp.is_null() {
        rpl_fflush(warcfp);
    }
    needs_flushing = 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn log_set_flush(mut flush: bool) {
    if flush as i32 == flush_log_p as i32 {
        return;
    }
    if flush as i32 == 0 as i32 {
        flush_log_p = 0 as i32 != 0;
    } else {
        if needs_flushing {
            logflush();
        }
        flush_log_p = 1 as i32 != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn log_set_save_context(mut savep: bool) -> bool {
    let mut old: bool = save_context_p;
    save_context_p = savep;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn logprintf(
    mut o: log_options,
    mut fmt: *const i8,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut lpstate: logvprintf_state = logvprintf_state {
        bigmsg: 0 as *mut i8,
        expected_size: 0,
        allocated: 0,
    };
    let mut done: bool = false;
    let mut errno_saved: i32 = *__errno_location();
    match o as u32 {
        4 => {
            if opt.show_progress == 0 {
                return;
            }
        }
        1 => {
            if opt.quiet {
                return;
            }
        }
        2 => {
            if opt.verbose != 0 || opt.quiet as i32 != 0 {
                return;
            }
        }
        0 => {
            if opt.verbose == 0 {
                return;
            }
        }
        3 | _ => {}
    }
    check_redirect_output();
    *__errno_location() = errno_saved;
    if inhibit_logging {
        return;
    }
    memset(
        &mut lpstate as *mut logvprintf_state as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<logvprintf_state>() as u64,
    );
    *__errno_location() = 0 as i32;
    loop {
        args_0 = args.clone();
        done = log_vprintf_internal(&mut lpstate, fmt, args_0.as_va_list());
        if done as i32 != 0 && *__errno_location() == 32 as i32 {
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
        if done {
            break;
        }
    }
    *__errno_location() = errno_saved;
}
#[no_mangle]
pub unsafe extern "C" fn debug_logprintf(mut fmt: *const i8, mut args: ...) {
    if opt.debug {
        let mut args_0: ::core::ffi::VaListImpl;
        let mut lpstate: logvprintf_state = logvprintf_state {
            bigmsg: 0 as *mut i8,
            expected_size: 0,
            allocated: 0,
        };
        let mut done: bool = false;
        check_redirect_output();
        if inhibit_logging {
            return;
        }
        memset(
            &mut lpstate as *mut logvprintf_state as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<logvprintf_state>() as u64,
        );
        loop {
            args_0 = args.clone();
            done = log_vprintf_internal(&mut lpstate, fmt, args_0.as_va_list());
            if done {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn log_init(mut file: *const i8, mut appendp: bool) {
    if !file.is_null() {
        if *file as i32 == '-' as i32 && *file.offset(1 as i32 as isize) == 0 {
            stdlogfp = stdout;
            logfp = stdlogfp;
        } else {
            filelogfp = rpl_fopen(
                file,
                if appendp as i32 != 0 {
                    b"a\0" as *const u8 as *const i8
                } else {
                    b"w\0" as *const u8 as *const i8
                },
            );
            if filelogfp.is_null() {
                fprintf(
                    stderr,
                    b"%s: %s: %s\n\0" as *const u8 as *const i8,
                    exec_name,
                    file,
                    strerror(*__errno_location()),
                );
                exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
            }
            logfp = filelogfp;
        }
    } else {
        stdlogfp = stderr;
        logfp = stdlogfp;
        if 1 as i32 != 0 && isatty(fileno(logfp)) != 0 {
            save_context_p = 1 as i32 != 0;
        }
    }
    shell_is_interactive = isatty(0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn log_close() {
    let mut i: i32 = 0;
    if !logfp.is_null() && logfp != stderr && logfp != stdout {
        if logfp == stdlogfp {
            stdlogfp = 0 as *mut FILE;
        }
        if logfp == filelogfp {
            filelogfp = 0 as *mut FILE;
        }
        fclose(logfp);
    }
    logfp = 0 as *mut FILE;
    inhibit_logging = 1 as i32 != 0;
    save_context_p = 0 as i32 != 0;
    i = 0 as i32;
    while i < 24 as i32 {
        free_log_line(i);
        i += 1;
        i;
    }
    log_line_current = -(1 as i32);
    trailing_line = 0 as i32 != 0;
}
unsafe extern "C" fn log_dump_context() {
    let mut num: i32 = log_line_current;
    let mut fp: *mut FILE = get_log_fp();
    let mut warcfp: *mut FILE = get_warc_log_fp();
    if fp.is_null() {
        return;
    }
    if num == -(1 as i32) {
        return;
    }
    if trailing_line {
        num += 1;
        if num >= 24 as i32 {
            num = 0 as i32;
        }
    }
    loop {
        let mut ln: *mut log_ln = log_lines.as_mut_ptr().offset(num as isize);
        if !((*ln).content).is_null() {
            fputs((*ln).content, fp);
            if !warcfp.is_null() {
                fputs((*ln).content, warcfp);
            }
        }
        num += 1;
        if num >= 24 as i32 {
            num = 0 as i32;
        }
        if !(num != log_line_current) {
            break;
        }
    }
    if trailing_line {
        if !(log_lines[log_line_current as usize].content).is_null() {
            fputs(log_lines[log_line_current as usize].content, fp);
            if !warcfp.is_null() {
                fputs(log_lines[log_line_current as usize].content, warcfp);
            }
        }
    }
    rpl_fflush(fp);
    rpl_fflush(warcfp);
}
unsafe extern "C" fn count_nonprint(mut source: *const i8) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut cnt: i32 = 0;
    p = source;
    cnt = 0 as i32;
    while *p != 0 {
        if !c_isprint(*p as i32) {
            cnt += 1;
            cnt;
        }
        p = p.offset(1);
        p;
    }
    return cnt;
}
unsafe extern "C" fn copy_and_escape(
    mut source: *const i8,
    mut dest: *mut i8,
    mut escape: i8,
    mut base: i32,
) {
    let mut from: *const i8 = source;
    let mut to: *mut i8 = dest;
    let mut c: u8 = 0;
    match base {
        8 => {
            loop {
                let fresh0 = from;
                from = from.offset(1);
                c = *fresh0 as u8;
                if !(c as i32 != '\0' as i32) {
                    break;
                }
                if c_isprint(c as i32) {
                    let fresh1 = to;
                    to = to.offset(1);
                    *fresh1 = c as i8;
                } else {
                    let fresh2 = to;
                    to = to.offset(1);
                    *fresh2 = escape;
                    let fresh3 = to;
                    to = to.offset(1);
                    *fresh3 = ('0' as i32 + (c as i32 >> 6 as i32)) as i8;
                    let fresh4 = to;
                    to = to.offset(1);
                    *fresh4 = ('0' as i32 + (c as i32 >> 3 as i32 & 7 as i32)) as i8;
                    let fresh5 = to;
                    to = to.offset(1);
                    *fresh5 = ('0' as i32 + (c as i32 & 7 as i32)) as i8;
                }
            }
        }
        16 => {
            loop {
                let fresh6 = from;
                from = from.offset(1);
                c = *fresh6 as u8;
                if !(c as i32 != '\0' as i32) {
                    break;
                }
                if c_isprint(c as i32) {
                    let fresh7 = to;
                    to = to.offset(1);
                    *fresh7 = c as i8;
                } else {
                    let fresh8 = to;
                    to = to.offset(1);
                    *fresh8 = escape;
                    let fresh9 = to;
                    to = to.offset(1);
                    *fresh9 = ((*::core::mem::transmute::<
                        &[u8; 17],
                        &[i8; 17],
                    >(b"0123456789ABCDEF\0"))[(c as i32 >> 4 as i32) as usize] as i32
                        + 0 as i32) as i8;
                    let fresh10 = to;
                    to = to.offset(1);
                    *fresh10 = ((*::core::mem::transmute::<
                        &[u8; 17],
                        &[i8; 17],
                    >(b"0123456789ABCDEF\0"))[(c as i32 & 0xf as i32) as usize] as i32
                        + 0 as i32) as i8;
                }
            }
        }
        _ => {
            abort();
        }
    }
    *to = '\0' as i32 as i8;
}
static mut ring: [ringel; 3] = [ringel {
    buffer: 0 as *const i8 as *mut i8,
    size: 0,
}; 3];
unsafe extern "C" fn escnonprint_internal(
    mut str: *const i8,
    mut escape: i8,
    mut base: i32,
) -> *const i8 {
    static mut ringpos: i32 = 0;
    let mut nprcnt: i32 = 0;
    nprcnt = count_nonprint(str);
    if nprcnt == 0 as i32 {
        return str;
    }
    let mut r: *mut ringel = ring.as_mut_ptr().offset(ringpos as isize);
    let mut needed_size: i32 = (strlen(str))
        .wrapping_add(1 as i32 as u64)
        .wrapping_add(
            (if base == 8 as i32 { 3 as i32 * nprcnt } else { 2 as i32 * nprcnt }) as u64,
        ) as i32;
    if ((*r).buffer).is_null() || (*r).size < needed_size {
        (*r).buffer = xrealloc((*r).buffer as *mut libc::c_void, needed_size as size_t)
            as *mut i8;
        (*r).size = needed_size;
    }
    copy_and_escape(str, (*r).buffer, escape, base);
    ringpos = (ringpos + 1 as i32) % 3 as i32;
    return (*r).buffer;
}
#[no_mangle]
pub unsafe extern "C" fn escnonprint(mut str: *const i8) -> *const i8 {
    return escnonprint_internal(str, '\\' as i32 as i8, 8 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn escnonprint_uri(mut str: *const i8) -> *const i8 {
    return escnonprint_internal(str, '%' as i32 as i8, 16 as i32);
}
static mut redirect_request_signal_name: *const i8 = 0 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn redirect_output(mut to_file: bool, mut signal_name: *const i8) {
    if to_file as i32 != 0 && logfp != filelogfp {
        if !signal_name.is_null() {
            fprintf(stderr, b"\n%s received.\0" as *const u8 as *const i8, signal_name);
        }
        if filelogfp.is_null() {
            filelogfp = unique_create(
                b"wget-log\0" as *const u8 as *const i8,
                0 as i32 != 0,
                &mut logfile,
            );
            if !filelogfp.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"\nRedirecting output to %s.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quote(logfile),
                );
                redirect_request_signal_name = signal_name;
                logfp = filelogfp;
                log_dump_context();
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: %s; disabling logging.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    if !logfile.is_null() {
                        logfile
                    } else {
                        b"wget-log\0" as *const u8 as *const i8
                    },
                    strerror(*__errno_location()),
                );
                inhibit_logging = 1 as i32 != 0;
            }
        } else {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"\nRedirecting output to %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote(logfile),
            );
            logfp = filelogfp;
            log_dump_context();
        }
    } else if !to_file && logfp != stdlogfp {
        logfp = stdlogfp;
        log_dump_context();
    }
}
unsafe extern "C" fn check_redirect_output() {
    if redirect_request_signal_name.is_null() && shell_is_interactive != 0
        && (opt.lfilename).is_null()
    {
        let mut foreground_pgrp: pid_t = tcgetpgrp(0 as i32);
        if foreground_pgrp != -(1 as i32) && foreground_pgrp != getpgrp() && !opt.quiet {
            redirect_output(1 as i32 != 0, 0 as *const i8);
        } else {
            redirect_output(0 as i32 != 0, 0 as *const i8);
        }
    }
}