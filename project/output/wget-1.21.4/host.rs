use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type hash_table;
    static mut opt: options;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    fn abort() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn quotearg_style(s: quoting_style, arg: *const i8) -> *mut i8;
    fn idn_decode(host: *const i8) -> *mut i8;
    fn inet_ntop(
        __af: i32,
        __cp: *const libc::c_void,
        __buf: *mut i8,
        __len: socklen_t,
    ) -> *const i8;
    fn getaddrinfo(
        __name: *const i8,
        __service: *const i8,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> i32;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: i32) -> *const i8;
    fn __errno_location() -> *mut i32;
    fn xstrdup_lower(_: *const i8) -> *mut i8;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn run_with_timeout(
        _: libc::c_double,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> bool;
    fn stable_sort(
        _: *mut libc::c_void,
        _: size_t,
        _: size_t,
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>,
    );
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> i32;
    fn make_nocase_string_hash_table(_: i32) -> *mut hash_table;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __socklen_t = u32;
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
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
    custom_quoting_style,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::literal_quoting_style => 0,
            quoting_style::shell_quoting_style => 1,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::c_quoting_style => 5,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::escape_quoting_style => 7,
            quoting_style::locale_quoting_style => 8,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::custom_quoting_style => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            0 => quoting_style::literal_quoting_style,
            1 => quoting_style::shell_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            5 => quoting_style::c_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            7 => quoting_style::escape_quoting_style,
            8 => quoting_style::locale_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            10 => quoting_style::custom_quoting_style,
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
pub type socklen_t = __socklen_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_NONBLOCK = 2048,
    SOCK_CLOEXEC = 524288,
    SOCK_PACKET = 10,
    SOCK_DCCP = 6,
    SOCK_SEQPACKET = 5,
    SOCK_RDM = 4,
    SOCK_RAW = 3,
    SOCK_DGRAM = 2,
    SOCK_STREAM = 1,
}
impl __socket_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __socket_type::SOCK_NONBLOCK => 2048,
            __socket_type::SOCK_CLOEXEC => 524288,
            __socket_type::SOCK_PACKET => 10,
            __socket_type::SOCK_DCCP => 6,
            __socket_type::SOCK_SEQPACKET => 5,
            __socket_type::SOCK_RDM => 4,
            __socket_type::SOCK_RAW => 3,
            __socket_type::SOCK_DGRAM => 2,
            __socket_type::SOCK_STREAM => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> __socket_type {
        match value {
            2048 => __socket_type::SOCK_NONBLOCK,
            524288 => __socket_type::SOCK_CLOEXEC,
            10 => __socket_type::SOCK_PACKET,
            6 => __socket_type::SOCK_DCCP,
            5 => __socket_type::SOCK_SEQPACKET,
            4 => __socket_type::SOCK_RDM,
            3 => __socket_type::SOCK_RAW,
            2 => __socket_type::SOCK_DGRAM,
            1 => __socket_type::SOCK_STREAM,
            _ => panic!("Invalid value for __socket_type: {}", value),
        }
    }
}
impl AddAssign<u32> for __socket_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for __socket_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for __socket_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for __socket_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for __socket_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for __socket_type {
    type Output = __socket_type;
    fn add(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for __socket_type {
    type Output = __socket_type;
    fn sub(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for __socket_type {
    type Output = __socket_type;
    fn mul(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for __socket_type {
    type Output = __socket_type;
    fn div(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for __socket_type {
    type Output = __socket_type;
    fn rem(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut i8,
    pub ai_next: *mut addrinfo,
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
pub struct address_list {
    pub count: i32,
    pub addresses: *mut ip_address,
    pub faulty: i32,
    pub connected: bool,
    pub refcount: i32,
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
    LH_SILENT = 1,
    LH_BIND = 2,
    LH_REFRESH = 4,
}
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_6::LH_SILENT => 1,
            C2RustUnnamed_6::LH_BIND => 2,
            C2RustUnnamed_6::LH_REFRESH => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_6 {
        match value {
            1 => C2RustUnnamed_6::LH_SILENT,
            2 => C2RustUnnamed_6::LH_BIND,
            4 => C2RustUnnamed_6::LH_REFRESH,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaiwt_context {
    pub node: *const i8,
    pub service: *const i8,
    pub hints: *const addrinfo,
    pub res: *mut *mut addrinfo,
    pub exit_code: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_7 {
    ns_inaddrsz = 4,
    ns_in6addrsz = 16,
    ns_int16sz = 2,
}
impl C2RustUnnamed_7 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_7::ns_inaddrsz => 4,
            C2RustUnnamed_7::ns_in6addrsz => 16,
            C2RustUnnamed_7::ns_int16sz => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_7 {
        match value {
            4 => C2RustUnnamed_7::ns_inaddrsz,
            16 => C2RustUnnamed_7::ns_in6addrsz,
            2 => C2RustUnnamed_7::ns_int16sz,
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
unsafe extern "C" fn _unhex(mut c: u8) -> u8 {
    return (if c as i32 <= '9' as i32 {
        c as i32 - '0' as i32
    } else if c as i32 <= 'F' as i32 {
        c as i32 - 'A' as i32 + 10 as i32
    } else {
        c as i32 - 'a' as i32 + 10 as i32
    }) as u8;
}
#[no_mangle]
pub unsafe extern "C" fn address_list_get_bounds(
    mut al: *const address_list,
    mut start: *mut i32,
    mut end: *mut i32,
) {
    *start = (*al).faulty;
    *end = (*al).count;
}
#[no_mangle]
pub unsafe extern "C" fn address_list_address_at(
    mut al: *const address_list,
    mut pos: i32,
) -> *const ip_address {
    return ((*al).addresses).offset(pos as isize);
}
#[no_mangle]
pub unsafe extern "C" fn address_list_contains(
    mut al: *const address_list,
    mut ip: *const ip_address,
) -> bool {
    let mut i: i32 = 0;
    match (*ip).family {
        2 => {
            i = 0 as i32;
            while i < (*al).count {
                let mut cur: *mut ip_address = ((*al).addresses).offset(i as isize);
                if (*cur).family == 2 as i32
                    && (*cur).data.d4.s_addr == (*ip).data.d4.s_addr
                {
                    return 1 as i32 != 0;
                }
                i += 1;
                i;
            }
            return 0 as i32 != 0;
        }
        10 => {
            i = 0 as i32;
            while i < (*al).count {
                let mut cur_0: *mut ip_address = ((*al).addresses).offset(i as isize);
                if (*cur_0).family == 10 as i32
                    && (*cur_0).ipv6_scope == (*ip).ipv6_scope
                    && ({
                        let mut __a: *const in6_addr = &mut (*cur_0).data.d6
                            as *mut in6_addr as *const in6_addr;
                        let mut __b: *const in6_addr = &(*ip).data.d6 as *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as i32 as usize]
                            == (*__b).__in6_u.__u6_addr32[0 as i32 as usize]
                            && (*__a).__in6_u.__u6_addr32[1 as i32 as usize]
                                == (*__b).__in6_u.__u6_addr32[1 as i32 as usize]
                            && (*__a).__in6_u.__u6_addr32[2 as i32 as usize]
                                == (*__b).__in6_u.__u6_addr32[2 as i32 as usize]
                            && (*__a).__in6_u.__u6_addr32[3 as i32 as usize]
                                == (*__b).__in6_u.__u6_addr32[3 as i32 as usize]) as i32
                    }) != 0
                {
                    return 1 as i32 != 0;
                }
                i += 1;
                i;
            }
            return 0 as i32 != 0;
        }
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn address_list_set_faulty(
    mut al: *mut address_list,
    mut index: i32,
) {
    if index != (*al).faulty {
        logprintf(
            log_options::LOG_ALWAYS,
            b"index: %d\nal->faulty: %d\n\0" as *const u8 as *const i8,
            index,
            (*al).faulty,
        );
        logprintf(
            log_options::LOG_ALWAYS,
            dcgettext(
                0 as *const i8,
                b"Error in handling the address list.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        logprintf(
            log_options::LOG_ALWAYS,
            dcgettext(
                0 as *const i8,
                b"Please report this issue to bug-wget@gnu.org\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        abort();
    }
    (*al).faulty += 1;
    (*al).faulty;
    if (*al).faulty >= (*al).count {
        (*al).faulty = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn address_list_set_connected(mut al: *mut address_list) {
    (*al).connected = 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn address_list_connected_p(mut al: *const address_list) -> bool {
    return (*al).connected;
}
unsafe extern "C" fn address_list_from_addrinfo(
    mut ai: *const addrinfo,
) -> *mut address_list {
    let mut al: *mut address_list = 0 as *mut address_list;
    let mut ptr: *const addrinfo = 0 as *const addrinfo;
    let mut cnt: i32 = 0;
    let mut ip: *mut ip_address = 0 as *mut ip_address;
    cnt = 0 as i32;
    ptr = ai;
    while !ptr.is_null() {
        if (*ptr).ai_family == 2 as i32 || (*ptr).ai_family == 10 as i32 {
            cnt += 1;
            cnt;
        }
        ptr = (*ptr).ai_next;
    }
    if cnt == 0 as i32 {
        return 0 as *mut address_list;
    }
    al = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<address_list>() as u64)
        as *mut address_list;
    (*al).addresses = xmalloc(
        (cnt as u64).wrapping_mul(::core::mem::size_of::<ip_address>() as u64),
    ) as *mut ip_address;
    (*al).count = cnt;
    (*al).refcount = 1 as i32;
    ip = (*al).addresses;
    ptr = ai;
    while !ptr.is_null() {
        if (*ptr).ai_family == 10 as i32 {
            let mut sin6: *const sockaddr_in6 = (*ptr).ai_addr as *const sockaddr_in6;
            (*ip).family = 10 as i32;
            (*ip).data.d6 = (*sin6).sin6_addr;
            (*ip).ipv6_scope = (*sin6).sin6_scope_id as i32;
            ip = ip.offset(1);
            ip;
        } else if (*ptr).ai_family == 2 as i32 {
            let mut sin: *const sockaddr_in = (*ptr).ai_addr as *const sockaddr_in;
            (*ip).family = 2 as i32;
            (*ip).data.d4 = (*sin).sin_addr;
            ip = ip.offset(1);
            ip;
        }
        ptr = (*ptr).ai_next;
    }
    return al;
}
unsafe extern "C" fn cmp_prefer_ipv4(
    mut addr1: *const libc::c_void,
    mut addr2: *const libc::c_void,
) -> i32 {
    return !((*(addr1 as *const ip_address)).family == 2 as i32) as i32
        - !((*(addr2 as *const ip_address)).family == 2 as i32) as i32;
}
unsafe extern "C" fn cmp_prefer_ipv6(
    mut addr1: *const libc::c_void,
    mut addr2: *const libc::c_void,
) -> i32 {
    return !((*(addr1 as *const ip_address)).family == 10 as i32) as i32
        - !((*(addr2 as *const ip_address)).family == 10 as i32) as i32;
}
unsafe extern "C" fn address_list_delete(mut al: *mut address_list) {
    rpl_free((*al).addresses as *mut libc::c_void);
    (*al).addresses = 0 as *mut ip_address;
    rpl_free(al as *mut libc::c_void);
    al = 0 as *mut address_list;
}
#[no_mangle]
pub unsafe extern "C" fn address_list_release(mut al: *mut address_list) {
    (*al).refcount -= 1;
    (*al).refcount;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Releasing 0x%0*lx (new refcount %d).\n\0" as *const u8 as *const i8,
            (2 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64) as i32,
            al as u64,
            (*al).refcount,
        );
    }
    if (*al).refcount <= 0 as i32 {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Deleting unused 0x%0*lx.\n\0" as *const u8 as *const i8,
                (2 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as u64)
                    as i32,
                al as u64,
            );
        }
        address_list_delete(al);
    }
}
unsafe extern "C" fn getaddrinfo_with_timeout_callback(mut arg: *mut libc::c_void) {
    let mut ctx: *mut gaiwt_context = arg as *mut gaiwt_context;
    (*ctx).exit_code = getaddrinfo(
        (*ctx).node,
        (*ctx).service,
        (*ctx).hints,
        (*ctx).res,
    );
}
unsafe extern "C" fn getaddrinfo_with_timeout(
    mut node: *const i8,
    mut service: *const i8,
    mut hints: *const addrinfo,
    mut res: *mut *mut addrinfo,
    mut timeout: libc::c_double,
) -> i32 {
    let mut ctx: gaiwt_context = gaiwt_context {
        node: 0 as *const i8,
        service: 0 as *const i8,
        hints: 0 as *const addrinfo,
        res: 0 as *mut *mut addrinfo,
        exit_code: 0,
    };
    ctx.node = node;
    ctx.service = service;
    ctx.hints = hints;
    ctx.res = res;
    if run_with_timeout(
        timeout,
        Some(
            getaddrinfo_with_timeout_callback
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
        &mut ctx as *mut gaiwt_context as *mut libc::c_void,
    ) {
        *__errno_location() = 110 as i32;
        return -(11 as i32);
    }
    return ctx.exit_code;
}
#[no_mangle]
pub unsafe extern "C" fn print_address(mut addr: *const ip_address) -> *const i8 {
    static mut buf: [i8; 64] = [0; 64];
    if (inet_ntop(
        (*addr).family,
        &(*addr).data as *const C2RustUnnamed_5 as *mut libc::c_void,
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 64]>() as u64 as socklen_t,
    ))
        .is_null()
    {
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 64]>() as u64,
            b"<error: %s>\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn is_valid_ipv4_address(
    mut str: *const i8,
    mut end: *const i8,
) -> bool {
    let mut saw_digit: bool = 0 as i32 != 0;
    let mut octets: i32 = 0 as i32;
    let mut val: i32 = 0 as i32;
    while str < end {
        let fresh0 = str;
        str = str.offset(1);
        let mut ch: i32 = *fresh0 as i32;
        if ch >= '0' as i32 && ch <= '9' as i32 {
            val = val * 10 as i32 + (ch - '0' as i32);
            if val > 255 as i32 {
                return 0 as i32 != 0;
            }
            if !saw_digit {
                octets += 1;
                if octets > 4 as i32 {
                    return 0 as i32 != 0;
                }
                saw_digit = 1 as i32 != 0;
            }
        } else if ch == '.' as i32 && saw_digit as i32 != 0 {
            if octets == 4 as i32 {
                return 0 as i32 != 0;
            }
            val = 0 as i32;
            saw_digit = 0 as i32 != 0;
        } else {
            return 0 as i32 != 0
        }
    }
    if octets < 4 as i32 {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_ipv6_address(
    mut str: *const i8,
    mut end: *const i8,
) -> bool {
    let mut curtok: *const i8 = 0 as *const i8;
    let mut tp: i32 = 0;
    let mut colonp: *const i8 = 0 as *const i8;
    let mut saw_xdigit: bool = false;
    let mut val: u32 = 0;
    tp = 0 as i32;
    colonp = 0 as *const i8;
    if str == end {
        return 0 as i32 != 0;
    }
    if *str as i32 == ':' as i32 {
        str = str.offset(1);
        str;
        if str == end || *str as i32 != ':' as i32 {
            return 0 as i32 != 0;
        }
    }
    curtok = str;
    saw_xdigit = 0 as i32 != 0;
    val = 0 as i32 as u32;
    while str < end {
        let fresh1 = str;
        str = str.offset(1);
        let mut ch: i32 = *fresh1 as i32;
        if c_isxdigit(ch) {
            val <<= 4 as i32;
            val |= _unhex(ch as u8) as u32;
            if val > 0xffff as i32 as u32 {
                return 0 as i32 != 0;
            }
            saw_xdigit = 1 as i32 != 0;
        } else if ch == ':' as i32 {
            curtok = str;
            if !saw_xdigit {
                if !colonp.is_null() {
                    return 0 as i32 != 0;
                }
                colonp = str.offset(tp as isize);
            } else {
                if str == end {
                    return 0 as i32 != 0;
                }
                if tp
                    > C2RustUnnamed_7::ns_in6addrsz as i32
                        - C2RustUnnamed_7::ns_int16sz as i32
                {
                    return 0 as i32 != 0;
                }
                tp += C2RustUnnamed_7::ns_int16sz as i32;
                saw_xdigit = 0 as i32 != 0;
                val = 0 as i32 as u32;
            }
        } else if ch == '.' as i32
            && tp
                <= C2RustUnnamed_7::ns_in6addrsz as i32
                    - C2RustUnnamed_7::ns_inaddrsz as i32
            && is_valid_ipv4_address(curtok, end) as i32 == 1 as i32
        {
            tp += C2RustUnnamed_7::ns_inaddrsz as i32;
            saw_xdigit = 0 as i32 != 0;
            break;
        } else {
            return 0 as i32 != 0
        }
    }
    if saw_xdigit {
        if tp > C2RustUnnamed_7::ns_in6addrsz as i32 - C2RustUnnamed_7::ns_int16sz as i32
        {
            return 0 as i32 != 0;
        }
        tp += C2RustUnnamed_7::ns_int16sz as i32;
    }
    if !colonp.is_null() {
        if tp == C2RustUnnamed_7::ns_in6addrsz as i32 {
            return 0 as i32 != 0;
        }
        tp = C2RustUnnamed_7::ns_in6addrsz as i32;
    }
    if tp != C2RustUnnamed_7::ns_in6addrsz as i32 {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
static mut host_name_addresses_map: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn cache_query(mut host: *const i8) -> *mut address_list {
    let mut al: *mut address_list = 0 as *mut address_list;
    if host_name_addresses_map.is_null() {
        return 0 as *mut address_list;
    }
    al = hash_table_get(host_name_addresses_map, host as *const libc::c_void)
        as *mut address_list;
    if !al.is_null() {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Found %s in host_name_addresses_map (%p)\n\0" as *const u8
                    as *const i8,
                host,
                al as *mut libc::c_void,
            );
        }
        (*al).refcount += 1;
        (*al).refcount;
        return al;
    }
    return 0 as *mut address_list;
}
unsafe extern "C" fn cache_store(mut host: *const i8, mut al: *mut address_list) {
    if host_name_addresses_map.is_null() {
        host_name_addresses_map = make_nocase_string_hash_table(0 as i32);
    }
    (*al).refcount += 1;
    (*al).refcount;
    hash_table_put(
        host_name_addresses_map,
        xstrdup_lower(host) as *const libc::c_void,
        al as *const libc::c_void,
    );
    if opt.debug as i64 != 0 {
        let mut i: i32 = 0;
        debug_logprintf(b"Caching %s =>\0" as *const u8 as *const i8, host);
        i = 0 as i32;
        while i < (*al).count {
            debug_logprintf(
                b" %s\0" as *const u8 as *const i8,
                print_address(((*al).addresses).offset(i as isize)),
            );
            i += 1;
            i;
        }
        debug_logprintf(b"\n\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn cache_remove(mut host: *const i8) {
    let mut al: *mut address_list = 0 as *mut address_list;
    if host_name_addresses_map.is_null() {
        return;
    }
    al = hash_table_get(host_name_addresses_map, host as *const libc::c_void)
        as *mut address_list;
    if !al.is_null() {
        address_list_release(al);
        hash_table_remove(host_name_addresses_map, host as *const libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lookup_host(
    mut host: *const i8,
    mut flags: i32,
) -> *mut address_list {
    let mut al: *mut address_list = 0 as *mut address_list;
    let mut silent: bool = flags & C2RustUnnamed_6::LH_SILENT as i32 != 0;
    let mut use_cache: bool = false;
    let mut numeric_address: bool = 0 as i32 != 0;
    let mut timeout: libc::c_double = opt.dns_timeout;
    let mut end: *const i8 = host.offset(strlen(host) as isize);
    if is_valid_ipv4_address(host, end) as i32 != 0
        || is_valid_ipv6_address(host, end) as i32 != 0
    {
        numeric_address = 1 as i32 != 0;
    }
    use_cache = opt.dns_cache;
    if flags & C2RustUnnamed_6::LH_BIND as i32 != 0 || numeric_address as i32 != 0 {
        use_cache = 0 as i32 != 0;
    }
    if use_cache {
        if flags & C2RustUnnamed_6::LH_REFRESH as i32 == 0 {
            al = cache_query(host);
            if !al.is_null() {
                return al;
            }
        } else {
            cache_remove(host);
        }
    }
    if !silent && !numeric_address {
        let mut str: *mut i8 = 0 as *mut i8;
        let mut name: *mut i8 = 0 as *mut i8;
        if opt.enable_iri as i32 != 0
            && {
                name = idn_decode(host as *mut i8);
                !name.is_null()
            }
        {
            str = aprintf(b"%s (%s)\0" as *const u8 as *const i8, name, host);
            rpl_free(name as *mut libc::c_void);
            name = 0 as *mut i8;
        }
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Resolving %s... \0" as *const u8 as *const i8,
                5 as i32,
            ),
            quotearg_style(
                quoting_style::escape_quoting_style,
                if !str.is_null() { str } else { host },
            ),
        );
        rpl_free(str as *mut libc::c_void);
        str = 0 as *mut i8;
    }
    let mut err: i32 = 0;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut i8,
        ai_next: 0 as *mut addrinfo,
    };
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<addrinfo>() as u64,
    );
    hints.ai_socktype = __socket_type::SOCK_STREAM as i32;
    if opt.ipv4_only {
        hints.ai_family = 2 as i32;
    } else if opt.ipv6_only {
        hints.ai_family = 10 as i32;
    } else {
        hints.ai_family = 0 as i32;
    }
    if flags & C2RustUnnamed_6::LH_BIND as i32 != 0 {
        hints.ai_flags |= 0x1 as i32;
    }
    if numeric_address {
        hints.ai_flags |= 0x4 as i32;
        timeout = 0 as i32 as libc::c_double;
    }
    err = getaddrinfo_with_timeout(host, 0 as *const i8, &mut hints, &mut res, timeout);
    if err != 0 as i32 || res.is_null() {
        if !silent {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"failed: %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                if err != -(11 as i32) {
                    gai_strerror(err)
                } else {
                    strerror(*__errno_location())
                },
            );
        }
        return 0 as *mut address_list;
    }
    al = address_list_from_addrinfo(res);
    freeaddrinfo(res);
    if al.is_null() {
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"failed: No IPv4/IPv6 addresses for host.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        return 0 as *mut address_list;
    }
    if (*al).count > 1 as i32
        && opt.prefer_family as u32 != C2RustUnnamed::prefer_none as i32 as u32
    {
        stable_sort(
            (*al).addresses as *mut libc::c_void,
            (*al).count as size_t,
            ::core::mem::size_of::<ip_address>() as u64,
            if opt.prefer_family as u32 == C2RustUnnamed::prefer_ipv4 as i32 as u32 {
                Some(
                    cmp_prefer_ipv4
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> i32,
                )
            } else {
                Some(
                    cmp_prefer_ipv6
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> i32,
                )
            },
        );
    }
    if !silent && !numeric_address {
        let mut i: i32 = 0;
        let mut printmax: i32 = (*al).count;
        if !opt.show_all_dns_entries && printmax > 3 as i32 {
            printmax = 3 as i32;
        }
        i = 0 as i32;
        while i < printmax {
            logputs(
                log_options::LOG_VERBOSE,
                print_address(((*al).addresses).offset(i as isize)),
            );
            if i < printmax - 1 as i32 {
                logputs(log_options::LOG_VERBOSE, b", \0" as *const u8 as *const i8);
            }
            i += 1;
            i;
        }
        if printmax != (*al).count {
            logputs(log_options::LOG_VERBOSE, b", ...\0" as *const u8 as *const i8);
        }
        logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
    }
    if use_cache {
        cache_store(host, al);
    }
    return al;
}
#[no_mangle]
pub unsafe extern "C" fn accept_domain(mut u: *mut url) -> bool {
    if !(opt.domains).is_null() {
        if !sufmatch(opt.domains as *mut *const i8, (*u).host) {
            return 0 as i32 != 0;
        }
    }
    if !(opt.exclude_domains).is_null() {
        if sufmatch(opt.exclude_domains as *mut *const i8, (*u).host) {
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn sufmatch(
    mut list: *mut *const i8,
    mut what: *const i8,
) -> bool {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut lw: i32 = 0;
    lw = strlen(what) as i32;
    i = 0 as i32;
    while !(*list.offset(i as isize)).is_null() {
        j = strlen(*list.offset(i as isize)) as i32;
        if !(lw < j) {
            k = lw;
            while j >= 0 as i32 && k >= 0 as i32 {
                if c_tolower(*(*list.offset(i as isize)).offset(j as isize) as i32)
                    != c_tolower(*what.offset(k as isize) as i32)
                {
                    break;
                }
                j -= 1;
                j;
                k -= 1;
                k;
            }
            if j == -(1 as i32)
                && (k == -(1 as i32) || *what.offset(k as isize) as i32 == '.' as i32
                    || *(*list.offset(i as isize)).offset(0 as i32 as isize) as i32
                        == '.' as i32)
            {
                return 1 as i32 != 0;
            }
        }
        i += 1;
        i;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_ip_address(mut name: *const i8) -> bool {
    let mut endp: *const i8 = 0 as *const i8;
    endp = name.offset(strlen(name) as isize);
    if is_valid_ipv4_address(name, endp) {
        return 1 as i32 != 0;
    }
    if is_valid_ipv6_address(name, endp) {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}