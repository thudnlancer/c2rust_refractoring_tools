use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    static mut opt: options;
    fn debug_logprintf(_: *const i8, _: ...);
    fn quote(arg: *const i8) -> *const i8;
    fn md5_init_ctx(ctx: *mut md5_ctx);
    fn md5_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut md5_ctx);
    fn md5_finish_ctx(ctx: *mut md5_ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
}
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type size_t = u64;
pub type int64_t = __int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
}
static mut Wp: [[i8; 4]; 2048] = [
    ['A' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'G' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'P' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'P' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'P' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'U' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'W' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'W' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'X' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'Y' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'H' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['B' as i32 as i8, 'Y' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'G' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'G' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'T' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'W' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['E' as i32 as i8, 'Y' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'Y' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['G' as i32 as i8, 'Y' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'H' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'C' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'F' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'Q' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['I' as i32 as i8, 'V' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['L' as i32 as i8, 'Y' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['M' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'U' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['N' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'F' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'H' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'R' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'T' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'W' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['O' as i32 as i8, 'X' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'E' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'E' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'I' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'U' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['P' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['Q' as i32 as i8, 'U' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['R' as i32 as i8, 'Y' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'I' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'P' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'P' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['T' as i32 as i8, 'W' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['U' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['U' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['U' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['U' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['V' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'B' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8, '\0' as i32 as i8],
    ['W' as i32 as i8, 'U' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'P' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'E' as i32 as i8, '\0' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, '\0' as i32 as i8],
    ['Y' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, '\0' as i32 as i8],
    ['A' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['A' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['A' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['A' as i32 as i8, 'B' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'B' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['A' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['A' as i32 as i8, 'F' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['A' as i32 as i8, 'F' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8],
    ['A' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8],
    ['A' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8],
    ['A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['A' as i32 as i8, 'J' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['A' as i32 as i8, 'K' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'G' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'S' as i32 as i8, 'O' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8],
    ['A' as i32 as i8, 'L' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'M' as i32 as i8, 'O' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'M' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['A' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8],
    ['A' as i32 as i8, 'Q' as i32 as i8, 'U' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'G' as i32 as i8, 'O' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8, 'Y' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8, 'Y' as i32 as i8],
    ['A' as i32 as i8, 'S' as i32 as i8, 'I' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8],
    ['A' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['A' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8],
    ['A' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8],
    ['A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['A' as i32 as i8, 'V' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['A' as i32 as i8, 'V' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8],
    ['A' as i32 as i8, 'V' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['A' as i32 as i8, 'V' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['A' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['A' as i32 as i8, 'W' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'M' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'B' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8],
    ['B' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['B' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8],
    ['B' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'A' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'H' as i32 as i8, 'R' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'B' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'E' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8],
    ['B' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'B' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'G' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'L' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['B' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'Y' as i32 as i8],
    ['B' as i32 as i8, 'Y' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'F' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'M' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8],
    ['C' as i32 as i8, 'H' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8],
    ['C' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'Y' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8],
    ['C' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'A' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'O' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'B' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['C' as i32 as i8, 'R' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, 'A' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'Y' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'B' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'L' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['C' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'Y' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['D' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, 'S' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'B' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8, 'Y' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'Y' as i32 as i8],
    ['D' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'C' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8],
    ['D' as i32 as i8, 'R' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, 'B' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['D' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'Y' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'L' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['E' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'Y' as i32 as i8],
    ['E' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['E' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8],
    ['E' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['E' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['E' as i32 as i8, 'D' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['E' as i32 as i8, 'D' as i32 as i8, 'G' as i32 as i8, 'Y' as i32 as i8],
    ['E' as i32 as i8, 'D' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['E' as i32 as i8, 'D' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['E' as i32 as i8, 'G' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'B' as i32 as i8, 'A' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8],
    ['E' as i32 as i8, 'L' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['E' as i32 as i8, 'M' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['E' as i32 as i8, 'M' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['E' as i32 as i8, 'M' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8],
    ['E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8, 'S' as i32 as i8],
    ['E' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8],
    ['E' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8],
    ['E' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['E' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['E' as i32 as i8, 'V' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['E' as i32 as i8, 'Y' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'E' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, 'S' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'M' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['F' as i32 as i8, 'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8],
    ['F' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, 'Y' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['F' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8],
    ['F' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['F' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'B' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'K' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['G' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'L' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8],
    ['G' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'F' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'F' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'G' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['G' as i32 as i8, 'R' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'F' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'U' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['G' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['G' as i32 as i8, 'W' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['G' as i32 as i8, 'W' as i32 as i8, 'Y' as i32 as i8, 'N' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'F' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'S' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, 'S' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'M' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'B' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'S' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['H' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, 'H' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['H' as i32 as i8, 'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, 'O' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'M' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'F' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, 'H' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, 'O' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'L' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['H' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['H' as i32 as i8, 'Y' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['H' as i32 as i8, 'Y' as i32 as i8, 'M' as i32 as i8, 'N' as i32 as i8],
    ['I' as i32 as i8, 'B' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8],
    ['I' as i32 as i8, 'C' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['I' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8],
    ['I' as i32 as i8, 'D' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['I' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8, 'Y' as i32 as i8],
    ['I' as i32 as i8, 'N' as i32 as i8, 'C' as i32 as i8, 'A' as i32 as i8],
    ['I' as i32 as i8, 'N' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8],
    ['I' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'S' as i32 as i8],
    ['I' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8],
    ['I' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8],
    ['I' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8],
    ['I' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8],
    ['I' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['I' as i32 as i8, 'S' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['I' as i32 as i8, 'T' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['I' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8],
    ['I' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8],
    ['J' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['J' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8],
    ['J' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['J' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['J' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['J' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['J' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['J' as i32 as i8, 'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, 'S' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'H' as i32 as i8, 'N' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['J' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'O' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'J' as i32 as i8, 'U' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['J' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['K' as i32 as i8, 'A' as i32 as i8, 'H' as i32 as i8, 'N' as i32 as i8],
    ['K' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['K' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['K' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['K' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'L' as i32 as i8],
    ['K' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'R' as i32 as i8],
    ['K' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8, 'S' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['K' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['K' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['K' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8],
    ['K' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['K' as i32 as i8, 'N' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['K' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8],
    ['K' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['K' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['K' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['K' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['K' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'O' as i32 as i8],
    ['K' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['K' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['K' as i32 as i8, 'Y' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'Y' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'B' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'U' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'F' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, 'B' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'A' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'I' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'Y' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['L' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['L' as i32 as i8, 'Y' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['L' as i32 as i8, 'Y' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'Y' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['L' as i32 as i8, 'Y' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, 'I' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'Y' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'C' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'S' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8],
    ['M' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, 'O' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8, 'O' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'U' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['M' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, 'I' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'I' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['M' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'D' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'F' as i32 as i8, 'F' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['M' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'T' as i32 as i8],
    ['M' as i32 as i8, 'Y' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8],
    ['M' as i32 as i8, 'Y' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, 'Y' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'Y' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, 'S' as i32 as i8],
    ['N' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, 'T' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'B' as i32 as i8, 'S' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['N' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'H' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8],
    ['N' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8],
    ['N' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['N' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['N' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, 'B' as i32 as i8],
    ['O' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['O' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8],
    ['O' as i32 as i8, 'B' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8],
    ['O' as i32 as i8, 'D' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['O' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, 'O' as i32 as i8],
    ['O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8],
    ['O' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['O' as i32 as i8, 'K' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['O' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8],
    ['O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['O' as i32 as i8, 'L' as i32 as i8, 'G' as i32 as i8, 'A' as i32 as i8],
    ['O' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['O' as i32 as i8, 'M' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['O' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['O' as i32 as i8, 'M' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['O' as i32 as i8, 'N' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8],
    ['O' as i32 as i8, 'N' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8],
    ['O' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8],
    ['O' as i32 as i8, 'N' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8],
    ['O' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['O' as i32 as i8, 'R' as i32 as i8, 'G' as i32 as i8, 'Y' as i32 as i8],
    ['O' as i32 as i8, 'S' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8],
    ['O' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8],
    ['O' as i32 as i8, 'T' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8],
    ['O' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['O' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['O' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['O' as i32 as i8, 'V' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['O' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8, 'Y' as i32 as i8],
    ['O' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8, 'S' as i32 as i8],
    ['Q' as i32 as i8, 'U' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['Q' as i32 as i8, 'U' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['Q' as i32 as i8, 'U' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'Y' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, 'S' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'F' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'F' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'A' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'S' as i32 as i8, 'Y' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'S' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, 'Y' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'S' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['R' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'F' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, 'S' as i32 as i8],
    ['S' as i32 as i8, 'C' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'C' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['S' as i32 as i8, 'C' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'C' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'F' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['S' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'H' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, 'H' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['S' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'U' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'K' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'L' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['S' as i32 as i8, 'M' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'M' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8, 'B' as i32 as i8],
    ['S' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'N' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8],
    ['S' as i32 as i8, 'N' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'F' as i32 as i8, 'A' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['S' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8],
    ['S' as i32 as i8, 'T' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8, 'S' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8, 'S' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['S' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'F' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8, 'B' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8],
    ['S' as i32 as i8, 'W' as i32 as i8, 'U' as i32 as i8, 'M' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'A' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'C' as i32 as i8, 'H' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'S' as i32 as i8],
    ['T' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'M' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'Y' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'U' as i32 as i8, 'D' as i32 as i8],
    ['T' as i32 as i8, 'H' as i32 as i8, 'U' as i32 as i8, 'G' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8, 'Y' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'Y' as i32 as i8],
    ['T' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, 'O' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'Y' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'U' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'O' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'M' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'O' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'O' as i32 as i8, 'Y' as i32 as i8],
    ['T' as i32 as i8, 'R' as i32 as i8, 'U' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, 'A' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'B' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'F' as i32 as i8, 'T' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'A' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'F' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'U' as i32 as i8, 'S' as i32 as i8, 'K' as i32 as i8],
    ['T' as i32 as i8, 'W' as i32 as i8, 'I' as i32 as i8, 'G' as i32 as i8],
    ['T' as i32 as i8, 'W' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['T' as i32 as i8, 'W' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['U' as i32 as i8, 'L' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['U' as i32 as i8, 'N' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['U' as i32 as i8, 'R' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['U' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['U' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8],
    ['U' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8],
    ['U' as i32 as i8, 'T' as i32 as i8, 'A' as i32 as i8, 'H' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['V' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8, 'A' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'B' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'Y' as i32 as i8],
    ['V' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8, 'O' as i32 as i8],
    ['V' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'E' as i32 as i8],
    ['V' as i32 as i8, 'I' as i32 as i8, 'E' as i32 as i8, 'W' as i32 as i8],
    ['V' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['V' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['V' as i32 as i8, 'O' as i32 as i8, 'I' as i32 as i8, 'D' as i32 as i8],
    ['V' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['V' as i32 as i8, 'O' as i32 as i8, 'T' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'D' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'G' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'K' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'S' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'V' as i32 as i8, 'Y' as i32 as i8],
    ['W' as i32 as i8, 'A' as i32 as i8, 'Y' as i32 as i8, 'S' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'K' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8, 'K' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'R' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'E' as i32 as i8, 'S' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'M' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'A' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'N' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'E' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'A' as i32 as i8],
    ['W' as i32 as i8, 'H' as i32 as i8, 'O' as i32 as i8, 'M' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'C' as i32 as i8, 'K' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'F' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'N' as i32 as i8, 'O' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'S' as i32 as i8, 'H' as i32 as i8],
    ['W' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8, 'H' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8, 'F' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'N' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'O' as i32 as i8, 'L' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'K' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'M' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['W' as i32 as i8, 'O' as i32 as i8, 'V' as i32 as i8, 'E' as i32 as i8],
    ['W' as i32 as i8, 'R' as i32 as i8, 'I' as i32 as i8, 'T' as i32 as i8],
    ['W' as i32 as i8, 'Y' as i32 as i8, 'N' as i32 as i8, 'N' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'L' as i32 as i8, 'E' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'G' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'N' as i32 as i8, 'K' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'D' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8, 'N' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'L' as i32 as i8],
    ['Y' as i32 as i8, 'A' as i32 as i8, 'W' as i32 as i8, 'N' as i32 as i8],
    ['Y' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'H' as i32 as i8],
    ['Y' as i32 as i8, 'E' as i32 as i8, 'A' as i32 as i8, 'R' as i32 as i8],
    ['Y' as i32 as i8, 'E' as i32 as i8, 'L' as i32 as i8, 'L' as i32 as i8],
    ['Y' as i32 as i8, 'O' as i32 as i8, 'G' as i32 as i8, 'A' as i32 as i8],
    ['Y' as i32 as i8, 'O' as i32 as i8, 'K' as i32 as i8, 'E' as i32 as i8],
];
unsafe extern "C" fn extract(
    mut s: *const u8,
    mut start: i32,
    mut length: i32,
) -> uint32_t {
    let mut cl: u8 = *s.offset((start / 8 as i32) as isize);
    let mut cc: u8 = *s.offset((start / 8 as i32 + 1 as i32) as isize);
    let mut cr: u8 = *s.offset((start / 8 as i32 + 2 as i32) as isize);
    let mut x: uint32_t = 0;
    x = (((cl as i32) << 8 as i32 | cc as i32) as uint32_t) << 8 as i32 | cr as u32;
    x >>= 24 as i32 - (length + start % 8 as i32);
    x &= (0xffff as i32 >> 16 as i32 - length) as u32;
    return x;
}
unsafe extern "C" fn btoe(mut store: *mut i8, mut c: *const u8) -> *mut i8 {
    let mut cp: [u8; 10] = [0; 10];
    let mut p: i32 = 0;
    let mut i: i32 = 0;
    let mut store_beg: *mut i8 = store;
    *store = '\0' as i32 as i8;
    memset(
        &mut cp as *mut [u8; 10] as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<[u8; 10]>() as u64,
    );
    memcpy(
        cp.as_mut_ptr() as *mut libc::c_void,
        c as *const libc::c_void,
        8 as i32 as u64,
    );
    p = 0 as i32;
    i = 0 as i32;
    while i < 64 as i32 {
        p = (p as u32).wrapping_add(extract(cp.as_mut_ptr(), i, 2 as i32)) as i32 as i32;
        i += 2 as i32;
    }
    cp[8 as i32 as usize] = ((p as i8 as i32) << 6 as i32) as u8;
    memcpy(
        store as *mut libc::c_void,
        &mut *(*Wp
            .as_mut_ptr()
            .offset(
                (extract
                    as unsafe extern "C" fn(
                        *const u8,
                        i32,
                        i32,
                    ) -> uint32_t)(cp.as_mut_ptr(), 0 as i32, 11 as i32) as isize,
            ))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i8 as *const libc::c_void,
        4 as i32 as u64,
    );
    store = store
        .offset(
            (if *store.offset(1 as i32 as isize) == 0 {
                1 as i32
            } else if *store.offset(2 as i32 as isize) == 0 {
                2 as i32
            } else if *store.offset(3 as i32 as isize) == 0 {
                3 as i32
            } else {
                4 as i32
            }) as isize,
        );
    let fresh0 = store;
    store = store.offset(1);
    *fresh0 = ' ' as i32 as i8;
    memcpy(
        store as *mut libc::c_void,
        &mut *(*Wp
            .as_mut_ptr()
            .offset(
                (extract
                    as unsafe extern "C" fn(
                        *const u8,
                        i32,
                        i32,
                    ) -> uint32_t)(cp.as_mut_ptr(), 11 as i32, 11 as i32) as isize,
            ))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i8 as *const libc::c_void,
        4 as i32 as u64,
    );
    store = store
        .offset(
            (if *store.offset(1 as i32 as isize) == 0 {
                1 as i32
            } else if *store.offset(2 as i32 as isize) == 0 {
                2 as i32
            } else if *store.offset(3 as i32 as isize) == 0 {
                3 as i32
            } else {
                4 as i32
            }) as isize,
        );
    let fresh1 = store;
    store = store.offset(1);
    *fresh1 = ' ' as i32 as i8;
    memcpy(
        store as *mut libc::c_void,
        &mut *(*Wp
            .as_mut_ptr()
            .offset(
                (extract
                    as unsafe extern "C" fn(
                        *const u8,
                        i32,
                        i32,
                    ) -> uint32_t)(cp.as_mut_ptr(), 22 as i32, 11 as i32) as isize,
            ))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i8 as *const libc::c_void,
        4 as i32 as u64,
    );
    store = store
        .offset(
            (if *store.offset(1 as i32 as isize) == 0 {
                1 as i32
            } else if *store.offset(2 as i32 as isize) == 0 {
                2 as i32
            } else if *store.offset(3 as i32 as isize) == 0 {
                3 as i32
            } else {
                4 as i32
            }) as isize,
        );
    let fresh2 = store;
    store = store.offset(1);
    *fresh2 = ' ' as i32 as i8;
    memcpy(
        store as *mut libc::c_void,
        &mut *(*Wp
            .as_mut_ptr()
            .offset(
                (extract
                    as unsafe extern "C" fn(
                        *const u8,
                        i32,
                        i32,
                    ) -> uint32_t)(cp.as_mut_ptr(), 33 as i32, 11 as i32) as isize,
            ))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i8 as *const libc::c_void,
        4 as i32 as u64,
    );
    store = store
        .offset(
            (if *store.offset(1 as i32 as isize) == 0 {
                1 as i32
            } else if *store.offset(2 as i32 as isize) == 0 {
                2 as i32
            } else if *store.offset(3 as i32 as isize) == 0 {
                3 as i32
            } else {
                4 as i32
            }) as isize,
        );
    let fresh3 = store;
    store = store.offset(1);
    *fresh3 = ' ' as i32 as i8;
    memcpy(
        store as *mut libc::c_void,
        &mut *(*Wp
            .as_mut_ptr()
            .offset(
                (extract
                    as unsafe extern "C" fn(
                        *const u8,
                        i32,
                        i32,
                    ) -> uint32_t)(cp.as_mut_ptr(), 44 as i32, 11 as i32) as isize,
            ))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i8 as *const libc::c_void,
        4 as i32 as u64,
    );
    store = store
        .offset(
            (if *store.offset(1 as i32 as isize) == 0 {
                1 as i32
            } else if *store.offset(2 as i32 as isize) == 0 {
                2 as i32
            } else if *store.offset(3 as i32 as isize) == 0 {
                3 as i32
            } else {
                4 as i32
            }) as isize,
        );
    let fresh4 = store;
    store = store.offset(1);
    *fresh4 = ' ' as i32 as i8;
    memcpy(
        store as *mut libc::c_void,
        &mut *(*Wp
            .as_mut_ptr()
            .offset(
                (extract
                    as unsafe extern "C" fn(
                        *const u8,
                        i32,
                        i32,
                    ) -> uint32_t)(cp.as_mut_ptr(), 55 as i32, 11 as i32) as isize,
            ))
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut i8 as *const libc::c_void,
        4 as i32 as u64,
    );
    *store.offset(4 as i32 as isize) = '\0' as i32 as i8;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"wrote %s to STORE\n\0" as *const u8 as *const i8,
            quote(store_beg),
        );
    }
    return store_beg;
}
#[no_mangle]
pub unsafe extern "C" fn skey_response(
    mut sequence: i32,
    mut seed: *const i8,
    mut pass: *const i8,
) -> *const i8 {
    let mut key: [u8; 8] = [0; 8];
    static mut english: [i8; 30] = [0; 30];
    let mut ctx: md5_ctx = md5_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut checksum: [uint32_t; 4] = [0; 4];
    md5_init_ctx(&mut ctx);
    md5_process_bytes(seed as *const u8 as *const libc::c_void, strlen(seed), &mut ctx);
    md5_process_bytes(pass as *const u8 as *const libc::c_void, strlen(pass), &mut ctx);
    md5_finish_ctx(&mut ctx, checksum.as_mut_ptr() as *mut u8 as *mut libc::c_void);
    checksum[0 as i32 as usize] ^= checksum[2 as i32 as usize];
    checksum[1 as i32 as usize] ^= checksum[3 as i32 as usize];
    memcpy(
        key.as_mut_ptr() as *mut libc::c_void,
        checksum.as_mut_ptr() as *const libc::c_void,
        8 as i32 as u64,
    );
    loop {
        let fresh5 = sequence;
        sequence = sequence - 1;
        if !(fresh5 > 0 as i32) {
            break;
        }
        md5_init_ctx(&mut ctx);
        md5_process_bytes(
            key.as_mut_ptr() as *const libc::c_void,
            8 as i32 as size_t,
            &mut ctx,
        );
        md5_finish_ctx(&mut ctx, checksum.as_mut_ptr() as *mut u8 as *mut libc::c_void);
        checksum[0 as i32 as usize] ^= checksum[2 as i32 as usize];
        checksum[1 as i32 as usize] ^= checksum[3 as i32 as usize];
        memcpy(
            key.as_mut_ptr() as *mut libc::c_void,
            checksum.as_mut_ptr() as *const libc::c_void,
            8 as i32 as u64,
        );
    }
    return btoe(english.as_mut_ptr(), key.as_mut_ptr());
}