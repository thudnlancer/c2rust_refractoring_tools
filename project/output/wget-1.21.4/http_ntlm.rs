#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn debug_logprintf(_: *const i8, _: ...);
    fn wget_base64_encode(_: *const libc::c_void, _: size_t, _: *mut i8) -> size_t;
    fn wget_base64_decode(_: *const i8, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn nettle_md4_init(ctx: *mut md4_ctx);
    fn nettle_md4_update(ctx: *mut md4_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md4_digest(ctx: *mut md4_ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_des_set_key(ctx: *mut des_ctx, key: *const uint8_t) -> i32;
    fn nettle_des_encrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub enum wgetntlm {
    NTLMSTATE_LAST = 4,
    NTLMSTATE_TYPE3 = 3,
    NTLMSTATE_TYPE2 = 2,
    NTLMSTATE_TYPE1 = 1,
    NTLMSTATE_NONE = 0,
}
impl wgetntlm {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            wgetntlm::NTLMSTATE_LAST => 4,
            wgetntlm::NTLMSTATE_TYPE3 => 3,
            wgetntlm::NTLMSTATE_TYPE2 => 2,
            wgetntlm::NTLMSTATE_TYPE1 => 1,
            wgetntlm::NTLMSTATE_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> wgetntlm {
        match value {
            4 => wgetntlm::NTLMSTATE_LAST,
            3 => wgetntlm::NTLMSTATE_TYPE3,
            2 => wgetntlm::NTLMSTATE_TYPE2,
            1 => wgetntlm::NTLMSTATE_TYPE1,
            0 => wgetntlm::NTLMSTATE_NONE,
            _ => panic!("Invalid value for wgetntlm: {}", value),
        }
    }
}
impl AddAssign<u32> for wgetntlm {
    fn add_assign(&mut self, rhs: u32) {
        *self = wgetntlm::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for wgetntlm {
    fn sub_assign(&mut self, rhs: u32) {
        *self = wgetntlm::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for wgetntlm {
    fn mul_assign(&mut self, rhs: u32) {
        *self = wgetntlm::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for wgetntlm {
    fn div_assign(&mut self, rhs: u32) {
        *self = wgetntlm::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for wgetntlm {
    fn rem_assign(&mut self, rhs: u32) {
        *self = wgetntlm::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for wgetntlm {
    type Output = wgetntlm;
    fn add(self, rhs: u32) -> wgetntlm {
        wgetntlm::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for wgetntlm {
    type Output = wgetntlm;
    fn sub(self, rhs: u32) -> wgetntlm {
        wgetntlm::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for wgetntlm {
    type Output = wgetntlm;
    fn mul(self, rhs: u32) -> wgetntlm {
        wgetntlm::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for wgetntlm {
    type Output = wgetntlm;
    fn div(self, rhs: u32) -> wgetntlm {
        wgetntlm::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for wgetntlm {
    type Output = wgetntlm;
    fn rem(self, rhs: u32) -> wgetntlm {
        wgetntlm::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ntlmdata {
    pub state: wgetntlm,
    pub nonce: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md4_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub block: [uint8_t; 64],
    pub index: u32,
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as i32 != 0,
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
pub unsafe extern "C" fn ntlm_input(
    mut ntlm: *mut ntlmdata,
    mut header: *const i8,
) -> bool {
    if 0 as i32 != strncmp(header, b"NTLM\0" as *const u8 as *const i8, 4 as i32 as u64)
    {
        return 0 as i32 != 0;
    }
    header = header.offset(4 as i32 as isize);
    while *header as i32 != 0 && c_isspace(*header as i32) as i32 != 0 {
        header = header.offset(1);
        header;
    }
    if *header != 0 {
        let mut size: ssize_t = 0;
        let mut buffer: [i8; 48] = [0; 48];
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Received a type-2 NTLM message.\n\0" as *const u8 as *const i8,
            );
        }
        size = wget_base64_decode(
            header,
            buffer.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<[i8; 48]>() as u64,
        );
        if size < 0 as i32 as i64 {
            return 0 as i32 != 0;
        }
        (*ntlm).state = wgetntlm::NTLMSTATE_TYPE2;
        if size as size_t >= ::core::mem::size_of::<[i8; 48]>() as u64 {
            memcpy(
                ((*ntlm).nonce).as_mut_ptr() as *mut libc::c_void,
                &mut *buffer.as_mut_ptr().offset(24 as i32 as isize) as *mut i8
                    as *const libc::c_void,
                8 as i32 as u64,
            );
        }
    } else {
        if (*ntlm).state as u32 == wgetntlm::NTLMSTATE_LAST as i32 as u32 {
            if opt.debug as i64 != 0 {
                debug_logprintf(b"NTLM auth restarted.\n\0" as *const u8 as *const i8);
            }
        } else if (*ntlm).state as u32 == wgetntlm::NTLMSTATE_TYPE3 as i32 as u32 {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"NTLM handshake rejected.\n\0" as *const u8 as *const i8,
                );
            }
            (*ntlm).state = wgetntlm::NTLMSTATE_NONE;
            return 0 as i32 != 0;
        } else if (*ntlm).state as u32 >= wgetntlm::NTLMSTATE_TYPE1 as i32 as u32 {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Unexpected empty NTLM message.\n\0" as *const u8 as *const i8,
                );
            }
            return 0 as i32 != 0;
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Empty NTLM message, (re)starting transaction.\n\0" as *const u8
                    as *const i8,
            );
        }
        (*ntlm).state = wgetntlm::NTLMSTATE_TYPE1;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn setup_des_key(mut key_56: *mut u8, mut des: *mut des_ctx) {
    let mut key: [u8; 8] = [0; 8];
    key[0 as i32 as usize] = *key_56.offset(0 as i32 as isize);
    key[1 as i32 as usize] = ((*key_56.offset(0 as i32 as isize) as i32) << 7 as i32
        & 0xff as i32 | *key_56.offset(1 as i32 as isize) as i32 >> 1 as i32) as u8;
    key[2 as i32 as usize] = ((*key_56.offset(1 as i32 as isize) as i32) << 6 as i32
        & 0xff as i32 | *key_56.offset(2 as i32 as isize) as i32 >> 2 as i32) as u8;
    key[3 as i32 as usize] = ((*key_56.offset(2 as i32 as isize) as i32) << 5 as i32
        & 0xff as i32 | *key_56.offset(3 as i32 as isize) as i32 >> 3 as i32) as u8;
    key[4 as i32 as usize] = ((*key_56.offset(3 as i32 as isize) as i32) << 4 as i32
        & 0xff as i32 | *key_56.offset(4 as i32 as isize) as i32 >> 4 as i32) as u8;
    key[5 as i32 as usize] = ((*key_56.offset(4 as i32 as isize) as i32) << 3 as i32
        & 0xff as i32 | *key_56.offset(5 as i32 as isize) as i32 >> 5 as i32) as u8;
    key[6 as i32 as usize] = ((*key_56.offset(5 as i32 as isize) as i32) << 2 as i32
        & 0xff as i32 | *key_56.offset(6 as i32 as isize) as i32 >> 6 as i32) as u8;
    key[7 as i32 as usize] = ((*key_56.offset(6 as i32 as isize) as i32) << 1 as i32
        & 0xff as i32) as u8;
    nettle_des_set_key(des, key.as_mut_ptr());
}
unsafe extern "C" fn calc_resp(
    mut keys: *mut u8,
    mut plaintext: *mut u8,
    mut results: *mut u8,
) {
    let mut des: des_ctx = des_ctx { key: [0; 32] };
    setup_des_key(keys, &mut des);
    nettle_des_encrypt(&mut des, 8 as i32 as size_t, results, plaintext);
    setup_des_key(keys.offset(7 as i32 as isize), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as i32 as size_t,
        results.offset(8 as i32 as isize),
        plaintext,
    );
    setup_des_key(keys.offset(14 as i32 as isize), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as i32 as size_t,
        results.offset(16 as i32 as isize),
        plaintext,
    );
}
unsafe extern "C" fn mkhash(
    mut password: *const i8,
    mut nonce: *mut u8,
    mut lmresp: *mut u8,
    mut ntresp: *mut u8,
) {
    let mut lmbuffer: [u8; 21] = [0; 21];
    let mut ntbuffer: [u8; 21] = [0; 21];
    let mut pw: [u8; 14] = [0; 14];
    static mut magic: [u8; 8] = [
        0x4b as i32 as u8,
        0x47 as i32 as u8,
        0x53 as i32 as u8,
        0x21 as i32 as u8,
        0x40 as i32 as u8,
        0x23 as i32 as u8,
        0x24 as i32 as u8,
        0x25 as i32 as u8,
    ];
    let mut i: size_t = 0;
    let mut len: size_t = strlen(password);
    if len > ::core::mem::size_of::<[u8; 14]>() as u64 {
        len = ::core::mem::size_of::<[u8; 14]>() as u64;
    }
    i = 0 as i32 as size_t;
    while i < len {
        pw[i as usize] = c_toupper(*password.offset(i as isize) as i32) as u8;
        i = i.wrapping_add(1);
        i;
    }
    while i < ::core::mem::size_of::<[u8; 14]>() as u64 {
        pw[i as usize] = 0 as i32 as u8;
        i = i.wrapping_add(1);
        i;
    }
    let mut des: des_ctx = des_ctx { key: [0; 32] };
    setup_des_key(pw.as_mut_ptr(), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as i32 as size_t,
        lmbuffer.as_mut_ptr(),
        magic.as_ptr(),
    );
    setup_des_key(pw.as_mut_ptr().offset(7 as i32 as isize), &mut des);
    nettle_des_encrypt(
        &mut des,
        8 as i32 as size_t,
        lmbuffer.as_mut_ptr().offset(8 as i32 as isize),
        magic.as_ptr(),
    );
    memset(
        lmbuffer.as_mut_ptr().offset(16 as i32 as isize) as *mut libc::c_void,
        0 as i32,
        5 as i32 as u64,
    );
    calc_resp(lmbuffer.as_mut_ptr(), nonce, lmresp);
    let mut MD4: md4_ctx = md4_ctx {
        state: [0; 4],
        count: 0,
        block: [0; 64],
        index: 0,
    };
    let mut pw4: [u8; 64] = [0; 64];
    len = strlen(password);
    if len > (::core::mem::size_of::<[u8; 64]>() as u64).wrapping_div(2 as i32 as u64) {
        len = (::core::mem::size_of::<[u8; 64]>() as u64).wrapping_div(2 as i32 as u64);
    }
    i = 0 as i32 as size_t;
    while i < len {
        pw4[(2 as i32 as u64).wrapping_mul(i) as usize] = *password.offset(i as isize)
            as u8;
        pw4[(2 as i32 as u64).wrapping_mul(i).wrapping_add(1 as i32 as u64) as usize] = 0
            as i32 as u8;
        i = i.wrapping_add(1);
        i;
    }
    nettle_md4_init(&mut MD4);
    nettle_md4_update(
        &mut MD4,
        (2 as i32 as u64).wrapping_mul(len) as u32 as size_t,
        pw4.as_mut_ptr(),
    );
    nettle_md4_digest(&mut MD4, 16 as i32 as size_t, ntbuffer.as_mut_ptr());
    memset(
        ntbuffer.as_mut_ptr().offset(16 as i32 as isize) as *mut libc::c_void,
        0 as i32,
        5 as i32 as u64,
    );
    calc_resp(ntbuffer.as_mut_ptr(), nonce, ntresp);
}
#[no_mangle]
pub unsafe extern "C" fn ntlm_output(
    mut ntlm: *mut ntlmdata,
    mut user: *const i8,
    mut passwd: *const i8,
    mut ready: *mut bool,
) -> *mut i8 {
    let mut domain: *const i8 = b"\0" as *const u8 as *const i8;
    let mut host: *const i8 = b"\0" as *const u8 as *const i8;
    let mut domlen: size_t = strlen(domain);
    let mut hostlen: size_t = strlen(host);
    let mut hostoff: size_t = 0;
    let mut domoff: size_t = 0;
    let mut size: size_t = 0;
    let mut ntlmbuf: [i8; 256] = [0; 256];
    let mut output: *mut i8 = 0 as *mut i8;
    *ready = 0 as i32 != 0;
    if user.is_null() {
        user = b"\0" as *const u8 as *const i8;
    }
    if passwd.is_null() {
        passwd = b"\0" as *const u8 as *const i8;
    }
    match (*ntlm).state as u32 {
        1 | 0 | 4 => {
            hostoff = 32 as i32 as size_t;
            domoff = hostoff.wrapping_add(hostlen);
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Creating a type-1 NTLM message.\n\0" as *const u8 as *const i8,
                );
            }
            snprintf(
                ntlmbuf.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 256]>() as u64,
                b"NTLMSSP%c\x01%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%s%s\0"
                    as *const u8 as *const i8,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                ((1 as i32) << 1 as i32 | (1 as i32) << 9 as i32) & 0xff as i32,
                ((1 as i32) << 1 as i32 | (1 as i32) << 9 as i32) >> 8 as i32
                    & 0xff as i32,
                ((1 as i32) << 1 as i32 | (1 as i32) << 9 as i32) >> 16 as i32
                    & 0xff as i32,
                ((1 as i32) << 1 as i32 | (1 as i32) << 9 as i32) >> 24 as i32,
                (domlen & 0xff as i32 as u64) as i8 as i32,
                (domlen >> 8 as i32) as i8 as i32,
                (domlen & 0xff as i32 as u64) as i8 as i32,
                (domlen >> 8 as i32) as i8 as i32,
                (domoff & 0xff as i32 as u64) as i8 as i32,
                (domoff >> 8 as i32) as i8 as i32,
                0 as i32,
                0 as i32,
                (hostlen & 0xff as i32 as u64) as i8 as i32,
                (hostlen >> 8 as i32) as i8 as i32,
                (hostlen & 0xff as i32 as u64) as i8 as i32,
                (hostlen >> 8 as i32) as i8 as i32,
                (hostoff & 0xff as i32 as u64) as i8 as i32,
                (hostoff >> 8 as i32) as i8 as i32,
                0 as i32,
                0 as i32,
                host,
                domain,
            );
            size = (32 as i32 as u64).wrapping_add(hostlen).wrapping_add(domlen);
            output = xmalloc(
                (5 as i32 as u64)
                    .wrapping_add(
                        (4 as i32 as u64)
                            .wrapping_mul(
                                size
                                    .wrapping_add(2 as i32 as u64)
                                    .wrapping_div(3 as i32 as u64),
                            ),
                    )
                    .wrapping_add(1 as i32 as u64),
            ) as *mut i8;
            memcpy(
                output as *mut libc::c_void,
                b"NTLM \0" as *const u8 as *const i8 as *const libc::c_void,
                5 as i32 as u64,
            );
            wget_base64_encode(
                ntlmbuf.as_mut_ptr() as *const libc::c_void,
                size,
                output.offset(5 as i32 as isize),
            );
        }
        2 => {
            let mut lmrespoff: size_t = 0;
            let mut ntrespoff: size_t = 0;
            let mut useroff: size_t = 0;
            let mut lmresp: [u8; 24] = [0; 24];
            let mut ntresp: [u8; 24] = [0; 24];
            let mut usr: *const i8 = 0 as *const i8;
            let mut userlen: size_t = 0;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Creating a type-3 NTLM message.\n\0" as *const u8 as *const i8,
                );
            }
            usr = strchr(user, '\\' as i32);
            if usr.is_null() {
                usr = strchr(user, '/' as i32);
            }
            if !usr.is_null() {
                domain = user;
                domlen = usr.offset_from(domain) as i64 as size_t;
                usr = usr.offset(1);
                usr;
            } else {
                usr = user;
            }
            userlen = strlen(usr);
            mkhash(
                passwd,
                &mut *((*ntlm).nonce).as_mut_ptr().offset(0 as i32 as isize),
                lmresp.as_mut_ptr(),
                ntresp.as_mut_ptr(),
            );
            domoff = 64 as i32 as size_t;
            useroff = domoff.wrapping_add(domlen);
            hostoff = useroff.wrapping_add(userlen);
            lmrespoff = hostoff.wrapping_add(hostlen);
            ntrespoff = lmrespoff.wrapping_add(0x18 as i32 as u64);
            snprintf(
                ntlmbuf.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 256]>() as u64,
                b"NTLMSSP%c\x03%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c\xFF\xFF%c%c\x01\x82%c%c\0"
                    as *const u8 as *const i8,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                (0x18 as i32 & 0xff as i32) as i8 as i32,
                (0x18 as i32 >> 8 as i32) as i8 as i32,
                (0x18 as i32 & 0xff as i32) as i8 as i32,
                (0x18 as i32 >> 8 as i32) as i8 as i32,
                (lmrespoff & 0xff as i32 as u64) as i8 as i32,
                (lmrespoff >> 8 as i32) as i8 as i32,
                0 as i32,
                0 as i32,
                (0x18 as i32 & 0xff as i32) as i8 as i32,
                (0x18 as i32 >> 8 as i32) as i8 as i32,
                (0x18 as i32 & 0xff as i32) as i8 as i32,
                (0x18 as i32 >> 8 as i32) as i8 as i32,
                (ntrespoff & 0xff as i32 as u64) as i8 as i32,
                (ntrespoff >> 8 as i32) as i8 as i32,
                0 as i32,
                0 as i32,
                (domlen & 0xff as i32 as u64) as i8 as i32,
                (domlen >> 8 as i32) as i8 as i32,
                (domlen & 0xff as i32 as u64) as i8 as i32,
                (domlen >> 8 as i32) as i8 as i32,
                (domoff & 0xff as i32 as u64) as i8 as i32,
                (domoff >> 8 as i32) as i8 as i32,
                0 as i32,
                0 as i32,
                (userlen & 0xff as i32 as u64) as i8 as i32,
                (userlen >> 8 as i32) as i8 as i32,
                (userlen & 0xff as i32 as u64) as i8 as i32,
                (userlen >> 8 as i32) as i8 as i32,
                (useroff & 0xff as i32 as u64) as i8 as i32,
                (useroff >> 8 as i32) as i8 as i32,
                0 as i32,
                0 as i32,
                (hostlen & 0xff as i32 as u64) as i8 as i32,
                (hostlen >> 8 as i32) as i8 as i32,
                (hostlen & 0xff as i32 as u64) as i8 as i32,
                (hostlen >> 8 as i32) as i8 as i32,
                (hostoff & 0xff as i32 as u64) as i8 as i32,
                (hostoff >> 8 as i32) as i8 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
            );
            size = 64 as i32 as size_t;
            ntlmbuf[63 as i32 as usize] = 0 as i32 as i8;
            ntlmbuf[62 as i32 as usize] = ntlmbuf[63 as i32 as usize];
            if size.wrapping_add(userlen).wrapping_add(domlen)
                >= ::core::mem::size_of::<[i8; 256]>() as u64
            {
                return 0 as *mut i8;
            }
            memcpy(
                &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut i8
                    as *mut libc::c_void,
                domain as *const libc::c_void,
                domlen,
            );
            size = (size as u64).wrapping_add(domlen) as size_t as size_t;
            memcpy(
                &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut i8
                    as *mut libc::c_void,
                usr as *const libc::c_void,
                userlen,
            );
            size = (size as u64).wrapping_add(userlen) as size_t as size_t;
            if size
                < (::core::mem::size_of::<[i8; 256]>() as u64)
                    .wrapping_sub(0x18 as i32 as u64)
            {
                memcpy(
                    &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut i8
                        as *mut libc::c_void,
                    lmresp.as_mut_ptr() as *const libc::c_void,
                    0x18 as i32 as u64,
                );
                size = (size as u64).wrapping_add(0x18 as i32 as u64) as size_t
                    as size_t;
            }
            if size
                < (::core::mem::size_of::<[i8; 256]>() as u64)
                    .wrapping_sub(0x18 as i32 as u64)
            {
                memcpy(
                    &mut *ntlmbuf.as_mut_ptr().offset(size as isize) as *mut i8
                        as *mut libc::c_void,
                    ntresp.as_mut_ptr() as *const libc::c_void,
                    0x18 as i32 as u64,
                );
                size = (size as u64).wrapping_add(0x18 as i32 as u64) as size_t
                    as size_t;
            }
            ntlmbuf[56 as i32 as usize] = (size & 0xff as i32 as u64) as i8;
            ntlmbuf[57 as i32 as usize] = (size >> 8 as i32) as i8;
            output = xmalloc(
                (5 as i32 as u64)
                    .wrapping_add(
                        (4 as i32 as u64)
                            .wrapping_mul(
                                size
                                    .wrapping_add(2 as i32 as u64)
                                    .wrapping_div(3 as i32 as u64),
                            ),
                    )
                    .wrapping_add(1 as i32 as u64),
            ) as *mut i8;
            memcpy(
                output as *mut libc::c_void,
                b"NTLM \0" as *const u8 as *const i8 as *const libc::c_void,
                5 as i32 as u64,
            );
            wget_base64_encode(
                ntlmbuf.as_mut_ptr() as *const libc::c_void,
                size,
                output.offset(5 as i32 as isize),
            );
            (*ntlm).state = wgetntlm::NTLMSTATE_TYPE3;
            *ready = 1 as i32 != 0;
        }
        3 => {
            *ready = 1 as i32 != 0;
            output = 0 as *mut i8;
        }
        _ => {}
    }
    return output;
}