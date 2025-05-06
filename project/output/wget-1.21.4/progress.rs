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
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strchrnul(__s: *const i8, __c: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn fileno(__stream: *mut FILE) -> i32;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn log_set_flush(_: bool);
    fn log_set_save_context(_: bool) -> bool;
    fn quote(arg: *const i8) -> *const i8;
    fn isatty(__fd: i32) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn wcwidth(__c: wchar_t) -> i32;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn iswcntrl(__wc: wint_t) -> i32;
    fn iswprint(__wc: wint_t) -> i32;
    static is_basic_table: [u32; 0];
    fn human_readable(_: wgint, _: i32, _: i32) -> *mut i8;
    fn numdigit(_: wgint) -> i32;
    fn number_to_static_string(_: wgint) -> *mut i8;
    fn determine_screen_width() -> i32;
    fn print_decimal(_: libc::c_double) -> *const i8;
    fn calc_rate(_: wgint, _: libc::c_double, _: *mut i32) -> libc::c_double;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
}
pub type __int64_t = i64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __sig_atomic_t = i32;
pub type size_t = u64;
pub type int64_t = __int64_t;
pub type wchar_t = i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub __wch: u32,
    pub __wchb: [i8; 4],
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
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
pub type wint_t = u32;
pub type mbstate_t = __mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const i8,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [i8; 24],
}
pub type mbchar_t = mbchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbiter_multi {
    pub limit: *const i8,
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
pub type mbi_iterator_t = mbiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct progress_implementation {
    pub name: *const i8,
    pub interactive: bool,
    pub create: Option<
        unsafe extern "C" fn(*const i8, wgint, wgint) -> *mut libc::c_void,
    >,
    pub update: Option<
        unsafe extern "C" fn(*mut libc::c_void, wgint, libc::c_double) -> (),
    >,
    pub draw: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub finish: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> ()>,
    pub set_params: Option<unsafe extern "C" fn(*const i8) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_progress {
    pub f_download: *mut i8,
    pub initial_length: wgint,
    pub total_length: wgint,
    pub count: wgint,
    pub last_screen_update: libc::c_double,
    pub dltime: libc::c_double,
    pub width: i32,
    pub buffer: *mut i8,
    pub tick: i32,
    pub hist: bar_progress_hist,
    pub recent_start: libc::c_double,
    pub recent_bytes: wgint,
    pub stalled: bool,
    pub last_eta_time: libc::c_double,
    pub last_eta_value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bar_progress_hist {
    pub pos: i32,
    pub times: [libc::c_double; 20],
    pub bytes: [wgint; 20],
    pub total_time: libc::c_double,
    pub total_bytes: wgint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dot_progress {
    pub initial_length: wgint,
    pub total_length: wgint,
    pub accumulated: wgint,
    pub dltime: libc::c_double,
    pub rows: wgint,
    pub dots: i32,
    pub last_timer_value: libc::c_double,
}
#[inline]
unsafe extern "C" fn mbiter_multi_next(mut iter: *mut mbiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 606308080656718511;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as i32 as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as i32 != 0;
        current_block = 15089075282327824602;
    } else {
        (*iter).in_shift = 1 as i32 != 0;
        current_block = 606308080656718511;
    }
    match current_block {
        606308080656718511 => {
            (*iter).cur.bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                ((*iter).limit).offset_from((*iter).cur.ptr) as i64 as size_t,
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as i32) as size_t {
                (*iter).cur.bytes = 1 as i32 as size_t;
                (*iter).cur.wc_valid = 0 as i32 != 0;
            } else if (*iter).cur.bytes == -(2 as i32) as size_t {
                (*iter).cur.bytes = ((*iter).limit).offset_from((*iter).cur.ptr) as i64
                    as size_t;
                (*iter).cur.wc_valid = 0 as i32 != 0;
            } else {
                if (*iter).cur.bytes == 0 as i32 as u64 {
                    (*iter).cur.bytes = 1 as i32 as size_t;
                }
                (*iter).cur.wc_valid = 1 as i32 != 0;
                if mbsinit(&mut (*iter).state) != 0 {
                    (*iter).in_shift = 0 as i32 != 0;
                }
            }
        }
        _ => {}
    }
    (*iter).next_done = 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn mb_width_aux(mut wc: wint_t) -> i32 {
    let mut w: i32 = wcwidth(wc as wchar_t);
    return if w >= 0 as i32 {
        w
    } else if iswcntrl(wc) != 0 {
        0 as i32
    } else {
        1 as i32
    };
}
#[inline]
unsafe extern "C" fn is_basic(mut c: i8) -> bool {
    return *is_basic_table.as_ptr().offset((c as u8 as i32 >> 5 as i32) as isize)
        >> (c as u8 as i32 & 31 as i32) & 1 as i32 as u32 != 0;
}
static mut implementations: [progress_implementation; 2] = unsafe {
    [
        {
            let mut init = progress_implementation {
                name: b"dot\0" as *const u8 as *const i8,
                interactive: 0 as i32 != 0,
                create: Some(
                    dot_create
                        as unsafe extern "C" fn(
                            *const i8,
                            wgint,
                            wgint,
                        ) -> *mut libc::c_void,
                ),
                update: Some(
                    dot_update
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            wgint,
                            libc::c_double,
                        ) -> (),
                ),
                draw: Some(dot_draw as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                finish: Some(
                    dot_finish
                        as unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> (),
                ),
                set_params: Some(dot_set_params as unsafe extern "C" fn(*const i8) -> ()),
            };
            init
        },
        {
            let mut init = progress_implementation {
                name: b"bar\0" as *const u8 as *const i8,
                interactive: 1 as i32 != 0,
                create: Some(
                    bar_create
                        as unsafe extern "C" fn(
                            *const i8,
                            wgint,
                            wgint,
                        ) -> *mut libc::c_void,
                ),
                update: Some(
                    bar_update
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            wgint,
                            libc::c_double,
                        ) -> (),
                ),
                draw: Some(bar_draw as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                finish: Some(
                    bar_finish
                        as unsafe extern "C" fn(*mut libc::c_void, libc::c_double) -> (),
                ),
                set_params: Some(bar_set_params as unsafe extern "C" fn(*const i8) -> ()),
            };
            init
        },
    ]
};
static mut current_impl: *mut progress_implementation = 0
    as *const progress_implementation as *mut progress_implementation;
static mut current_impl_locked: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn valid_progress_implementation_p(mut name: *const i8) -> bool {
    let mut i: size_t = 0;
    let mut pi: *mut progress_implementation = implementations.as_mut_ptr();
    let mut colon: *mut i8 = strchr(name, ':' as i32);
    let mut namelen: size_t = if !colon.is_null() {
        colon.offset_from(name) as i64 as size_t
    } else {
        strlen(name)
    };
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[progress_implementation; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<progress_implementation>() as u64)
    {
        if strncmp((*pi).name, name, namelen) == 0 {
            return 1 as i32 != 0;
        }
        i = i.wrapping_add(1);
        i;
        pi = pi.offset(1);
        pi;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn set_progress_implementation(mut name: *const i8) {
    let mut i: size_t = 0;
    let mut namelen: size_t = 0;
    let mut pi: *mut progress_implementation = implementations.as_mut_ptr();
    let mut colon: *const i8 = 0 as *const i8;
    if name.is_null() {
        name = b"bar\0" as *const u8 as *const i8;
    }
    colon = strchr(name, ':' as i32);
    namelen = if !colon.is_null() {
        colon.offset_from(name) as i64 as size_t
    } else {
        strlen(name)
    };
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[progress_implementation; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<progress_implementation>() as u64)
    {
        if strncmp((*pi).name, name, namelen) == 0 {
            current_impl = pi;
            current_impl_locked = 0 as i32;
            if !colon.is_null() {
                colon = colon.offset(1);
                colon;
            }
            if ((*pi).set_params).is_some() {
                ((*pi).set_params).expect("non-null function pointer")(colon);
            }
            return;
        }
        i = i.wrapping_add(1);
        i;
        pi = pi.offset(1);
        pi;
    }
    abort();
}
static mut output_redirected: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn progress_schedule_redirect() {
    output_redirected = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn progress_create(
    mut f_download: *const i8,
    mut initial: wgint,
    mut total: wgint,
) -> *mut libc::c_void {
    if output_redirected != 0 {
        if current_impl_locked == 0 {
            set_progress_implementation(b"dot\0" as *const u8 as *const i8);
        }
        output_redirected = 0 as i32;
    }
    return ((*current_impl).create)
        .expect("non-null function pointer")(f_download, initial, total);
}
#[no_mangle]
pub unsafe extern "C" fn progress_interactive_p(
    mut progress: *mut libc::c_void,
) -> bool {
    return (*current_impl).interactive;
}
#[no_mangle]
pub unsafe extern "C" fn progress_update(
    mut progress: *mut libc::c_void,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    if dltime >= 2147483647 as i32 as libc::c_double {
        dltime = (2147483647 as i32 - 1 as i32) as libc::c_double;
    } else if dltime < 0 as i32 as libc::c_double {
        dltime = 0 as i32 as libc::c_double;
    }
    if howmuch < 0 as i32 as i64 {
        howmuch = 0 as i32 as wgint;
    }
    ((*current_impl).update)
        .expect("non-null function pointer")(progress, howmuch, dltime);
    ((*current_impl).draw).expect("non-null function pointer")(progress);
}
#[no_mangle]
pub unsafe extern "C" fn progress_finish(
    mut progress: *mut libc::c_void,
    mut dltime: libc::c_double,
) {
    if dltime >= 2147483647 as i32 as libc::c_double {
        dltime = (2147483647 as i32 - 1 as i32) as libc::c_double;
    } else if dltime < 0 as i32 as libc::c_double {
        dltime = 0 as i32 as libc::c_double;
    }
    ((*current_impl).finish).expect("non-null function pointer")(progress, dltime);
}
unsafe extern "C" fn dot_create(
    mut f_download: *const i8,
    mut initial: wgint,
    mut total: wgint,
) -> *mut libc::c_void {
    let mut dp: *mut dot_progress = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<dot_progress>() as u64,
    ) as *mut dot_progress;
    (*dp).initial_length = initial;
    (*dp).total_length = total;
    if (*dp).initial_length != 0 {
        let mut dot_bytes: i32 = opt.dot_bytes as i32;
        let ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as i64;
        let mut remainder: i32 = ((*dp).initial_length % ROW_BYTES) as i32;
        let mut skipped: wgint = (*dp).initial_length - remainder as i64;
        if skipped != 0 {
            let mut skipped_k: wgint = skipped / 1024 as i32 as i64;
            let mut skipped_k_len: i32 = numdigit(skipped_k);
            if skipped_k_len < 6 as i32 {
                skipped_k_len = 6 as i32;
            }
            logprintf(
                log_options::LOG_PROGRESS,
                dcgettext(
                    0 as *const i8,
                    b"\n%*s[ skipping %sK ]\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                2 as i32 + skipped_k_len,
                b"\0" as *const u8 as *const i8,
                number_to_static_string(skipped_k),
            );
        }
        logprintf(
            log_options::LOG_PROGRESS,
            b"\n%6sK\0" as *const u8 as *const i8,
            number_to_static_string(skipped / 1024 as i32 as i64),
        );
        while remainder >= dot_bytes {
            if (*dp).dots % opt.dot_spacing == 0 as i32 {
                logputs(log_options::LOG_PROGRESS, b" \0" as *const u8 as *const i8);
            }
            logputs(log_options::LOG_PROGRESS, b",\0" as *const u8 as *const i8);
            (*dp).dots += 1;
            (*dp).dots;
            remainder -= dot_bytes;
        }
        (*dp).accumulated = remainder as wgint;
        (*dp).rows = skipped / ROW_BYTES;
    }
    return dp as *mut libc::c_void;
}
unsafe extern "C" fn print_row_stats(
    mut dp: *mut dot_progress,
    mut dltime: libc::c_double,
    mut last: bool,
    mut added_rows: wgint,
) {
    let ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as i64;
    let mut bytes_displayed: wgint = (*dp).rows * ROW_BYTES
        + (*dp).dots as i64 * opt.dot_bytes;
    if last {
        bytes_displayed += (*dp).accumulated;
    }
    if bytes_displayed < 0 as i32 as i64 {
        bytes_displayed = 0 as i32 as wgint;
    }
    if (*dp).total_length != 0 {
        let mut percentage: i32 = (100.0f64 * bytes_displayed as libc::c_double
            / (*dp).total_length as libc::c_double) as i32;
        logprintf(
            log_options::LOG_PROGRESS,
            b"%3d%%\0" as *const u8 as *const i8,
            percentage,
        );
    }
    static mut names: [i8; 5] = [
        ' ' as i32 as i8,
        'K' as i32 as i8,
        'M' as i32 as i8,
        'G' as i32 as i8,
        'T' as i32 as i8,
    ];
    let mut units: i32 = 0;
    let mut rate: libc::c_double = 0.;
    let mut bytes_this_row: wgint = 0;
    if !last {
        bytes_this_row = ROW_BYTES * added_rows;
    } else {
        bytes_this_row = (*dp).dots as i64 * opt.dot_bytes + (*dp).accumulated;
    }
    if (*dp).rows == (*dp).initial_length / ROW_BYTES {
        bytes_this_row -= (*dp).initial_length % ROW_BYTES;
    }
    rate = calc_rate(bytes_this_row, dltime - (*dp).last_timer_value, &mut units);
    logprintf(
        log_options::LOG_PROGRESS,
        b" %4.*f%c\0" as *const u8 as *const i8,
        if rate >= 99.95f64 {
            0 as i32
        } else if rate >= 9.995f64 {
            1 as i32
        } else {
            2 as i32
        },
        rate,
        names[units as usize] as i32,
    );
    (*dp).last_timer_value = dltime;
    if !last {
        if (*dp).total_length != 0 {
            let mut bytes_remaining: wgint = if (*dp).total_length > bytes_displayed {
                (*dp).total_length - bytes_displayed
            } else {
                0 as i32 as i64
            };
            let mut bytes_sofar: wgint = if bytes_displayed > (*dp).initial_length {
                bytes_displayed - (*dp).initial_length
            } else {
                1 as i32 as i64
            };
            let mut eta: libc::c_double = dltime * bytes_remaining as libc::c_double
                / bytes_sofar as libc::c_double;
            if eta < 0 as i32 as libc::c_double {
                eta = 0 as i32 as libc::c_double;
            }
            if eta < (2147483647 as i32 - 1 as i32) as libc::c_double {
                logprintf(
                    log_options::LOG_PROGRESS,
                    b" %s\0" as *const u8 as *const i8,
                    eta_to_human_short((eta + 0.5f64) as i32, 1 as i32 != 0),
                );
            }
        }
    } else if dltime >= 10 as i32 as libc::c_double {
        logprintf(
            log_options::LOG_PROGRESS,
            b"=%s\0" as *const u8 as *const i8,
            eta_to_human_short((dltime + 0.5f64) as i32, 1 as i32 != 0),
        );
    } else {
        logprintf(
            log_options::LOG_PROGRESS,
            b"=%ss\0" as *const u8 as *const i8,
            print_decimal(dltime),
        );
    };
}
unsafe extern "C" fn dot_update(
    mut progress: *mut libc::c_void,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    if dltime >= 2147483647 as i32 as libc::c_double {
        dltime = (2147483647 as i32 - 1 as i32) as libc::c_double;
    } else if dltime < 0 as i32 as libc::c_double {
        dltime = 0 as i32 as libc::c_double;
    }
    if howmuch < 0 as i32 as i64 {
        howmuch = 0 as i32 as wgint;
    }
    let mut dp: *mut dot_progress = progress as *mut dot_progress;
    (*dp).accumulated += howmuch;
    (*dp).dltime = dltime;
}
unsafe extern "C" fn dot_draw(mut progress: *mut libc::c_void) {
    let mut dp: *mut dot_progress = progress as *mut dot_progress;
    let mut dot_bytes: i32 = opt.dot_bytes as i32;
    let mut ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as i64;
    log_set_flush(0 as i32 != 0);
    while (*dp).accumulated >= dot_bytes as i64 {
        (*dp).accumulated -= dot_bytes as i64;
        if (*dp).dots == 0 as i32 {
            logprintf(
                log_options::LOG_PROGRESS,
                b"\n%6sK\0" as *const u8 as *const i8,
                number_to_static_string((*dp).rows * ROW_BYTES / 1024 as i32 as i64),
            );
        }
        if (*dp).dots % opt.dot_spacing == 0 as i32 {
            logputs(log_options::LOG_PROGRESS, b" \0" as *const u8 as *const i8);
        }
        logputs(log_options::LOG_PROGRESS, b".\0" as *const u8 as *const i8);
        (*dp).dots += 1;
        (*dp).dots;
        if (*dp).dots >= opt.dots_in_line {
            (*dp).dots = 0 as i32;
            let mut added_rows: wgint = 1 as i32 as wgint;
            if (*dp).accumulated >= ROW_BYTES << 2 as i32 {
                added_rows += (*dp).accumulated / ROW_BYTES;
                (*dp).accumulated %= ROW_BYTES;
            }
            if 9223372036854775807 as i64 - (*dp).rows >= added_rows {
                (*dp).rows += added_rows;
            } else {
                (*dp).rows = 9223372036854775807 as i64;
            }
            print_row_stats(dp, (*dp).dltime, 0 as i32 != 0, added_rows);
        }
    }
    log_set_flush(1 as i32 != 0);
}
unsafe extern "C" fn dot_finish(
    mut progress: *mut libc::c_void,
    mut dltime: libc::c_double,
) {
    let mut dp: *mut dot_progress = progress as *mut dot_progress;
    let mut ROW_BYTES: wgint = opt.dot_bytes * opt.dots_in_line as i64;
    let mut i: i32 = 0;
    log_set_flush(0 as i32 != 0);
    if (*dp).dots == 0 as i32 {
        logprintf(
            log_options::LOG_PROGRESS,
            b"\n%6sK\0" as *const u8 as *const i8,
            number_to_static_string((*dp).rows * ROW_BYTES / 1024 as i32 as i64),
        );
    }
    i = (*dp).dots;
    while i < opt.dots_in_line {
        if i % opt.dot_spacing == 0 as i32 {
            logputs(log_options::LOG_PROGRESS, b" \0" as *const u8 as *const i8);
        }
        logputs(log_options::LOG_PROGRESS, b" \0" as *const u8 as *const i8);
        i += 1;
        i;
    }
    if dltime >= 2147483647 as i32 as libc::c_double {
        dltime = (2147483647 as i32 - 1 as i32) as libc::c_double;
    } else if dltime < 0 as i32 as libc::c_double {
        dltime = 0 as i32 as libc::c_double;
    }
    print_row_stats(dp, dltime, 1 as i32 != 0, 1 as i32 as wgint);
    logputs(log_options::LOG_PROGRESS, b"\n\n\0" as *const u8 as *const i8);
    log_set_flush(0 as i32 != 0);
    rpl_free(dp as *mut libc::c_void);
    dp = 0 as *mut dot_progress;
}
unsafe extern "C" fn dot_set_params(mut params: *const i8) {
    (*current_impl).interactive = 0 as i32 != 0;
    if params.is_null() || *params == 0 {
        params = opt.dot_style;
    }
    if params.is_null() {
        return;
    }
    if c_strcasecmp(params, b"default\0" as *const u8 as *const i8) == 0 {
        opt.dot_bytes = 1024 as i32 as wgint;
        opt.dot_spacing = 10 as i32;
        opt.dots_in_line = 50 as i32;
    } else if c_strcasecmp(params, b"binary\0" as *const u8 as *const i8) == 0 {
        opt.dot_bytes = 8192 as i32 as wgint;
        opt.dot_spacing = 16 as i32;
        opt.dots_in_line = 48 as i32;
    } else if c_strcasecmp(params, b"mega\0" as *const u8 as *const i8) == 0 {
        opt.dot_bytes = 65536 as i64;
        opt.dot_spacing = 8 as i32;
        opt.dots_in_line = 48 as i32;
    } else if c_strcasecmp(params, b"giga\0" as *const u8 as *const i8) == 0 {
        opt.dot_bytes = (1 as i64) << 20 as i32;
        opt.dot_spacing = 8 as i32;
        opt.dots_in_line = 32 as i32;
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Invalid dot style specification %s; leaving unchanged.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            quote(params),
        );
    };
}
static mut screen_width: i32 = 0;
static mut received_sigwinch: sig_atomic_t = 0;
unsafe extern "C" fn prepare_filename(mut dest: *mut i8, mut src: *const i8) -> size_t {
    let mut ret: size_t = 1 as i32 as size_t;
    if !src.is_null() {
        let mut iter: mbi_iterator_t = mbi_iterator_t {
            limit: 0 as *const i8,
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed_4 { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const i8,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        let mut mbc: mbchar_t = mbchar {
            ptr: 0 as *const i8,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        };
        iter.cur.ptr = src;
        iter.limit = (iter.cur.ptr).offset(strlen(src) as isize);
        iter.in_shift = 0 as i32 != 0;
        memset(
            &mut iter.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        iter.next_done = 0 as i32 != 0;
        while iter.cur.ptr < iter.limit
            && {
                mbiter_multi_next(&mut iter);
                1 as i32 != 0
            }
        {
            let mut i: size_t = 0;
            mbc = iter.cur;
            if !(mbc.wc_valid as i32 != 0 && iswprint(mbc.wc as wint_t) != 0)
                || (if mbc.wc_valid as i32 != 0 {
                    mb_width_aux(mbc.wc as wint_t)
                } else {
                    1 as i32
                }) == 0
            {
                i = 0 as i32 as size_t;
                while i < mbc.bytes {
                    if !dest.is_null() {
                        dest = dest
                            .offset(
                                sprintf(
                                    dest,
                                    b"%%%02x\0" as *const u8 as *const i8,
                                    *(mbc.ptr).offset(i as isize) as u8 as i32,
                                ) as isize,
                            );
                    }
                    ret = (ret as u64).wrapping_add(3 as i32 as u64) as size_t as size_t;
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                if !dest.is_null() {
                    i = 0 as i32 as size_t;
                    while i < mbc.bytes {
                        let fresh0 = dest;
                        dest = dest.offset(1);
                        *fresh0 = *(mbc.ptr).offset(i as isize);
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                ret = (ret as u64).wrapping_add(mbc.bytes) as size_t as size_t;
            }
            iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
            iter.next_done = 0 as i32 != 0;
        }
    }
    if !dest.is_null() {
        *dest = 0 as i32 as i8;
    }
    return ret;
}
unsafe extern "C" fn bar_create(
    mut f_download: *const i8,
    mut initial: wgint,
    mut total: wgint,
) -> *mut libc::c_void {
    let mut bp: *mut bar_progress = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<bar_progress>() as u64,
    ) as *mut bar_progress;
    if initial > total {
        total = initial;
    }
    (*bp).initial_length = initial;
    (*bp).total_length = total;
    (*bp).f_download = xmalloc(prepare_filename(0 as *mut i8, f_download)) as *mut i8;
    prepare_filename((*bp).f_download, f_download);
    if screen_width == 0 || received_sigwinch != 0 {
        screen_width = determine_screen_width();
        if screen_width == 0 {
            screen_width = 80 as i32;
        } else if screen_width < 51 as i32 {
            screen_width = 51 as i32;
        }
        ::core::ptr::write_volatile(
            &mut received_sigwinch as *mut sig_atomic_t,
            0 as i32,
        );
    }
    (*bp).width = screen_width - 1 as i32;
    (*bp).buffer = xcalloc(
        ((*bp).width * 2 as i32 + 100 as i32) as size_t,
        1 as i32 as size_t,
    ) as *mut i8;
    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
    create_image(bp, 0 as i32 as libc::c_double, 0 as i32 != 0);
    display_image((*bp).buffer);
    return bp as *mut libc::c_void;
}
unsafe extern "C" fn bar_update(
    mut progress: *mut libc::c_void,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    let mut bp: *mut bar_progress = progress as *mut bar_progress;
    (*bp).dltime = dltime;
    if 9223372036854775807 as i64 - ((*bp).count + (*bp).initial_length) >= howmuch {
        (*bp).count += howmuch;
    } else {
        (*bp).count = 9223372036854775807 as i64 - (*bp).initial_length;
    }
    if (*bp).total_length > 0 as i32 as i64
        && (*bp).count + (*bp).initial_length > (*bp).total_length
    {
        (*bp).total_length = (*bp).initial_length + (*bp).count;
    }
    update_speed_ring(bp, howmuch, dltime);
}
unsafe extern "C" fn bar_draw(mut progress: *mut libc::c_void) {
    let mut force_screen_update: bool = 0 as i32 != 0;
    let mut bp: *mut bar_progress = progress as *mut bar_progress;
    if received_sigwinch != 0 {
        let mut old_width: i32 = screen_width;
        screen_width = determine_screen_width();
        if screen_width == 0 {
            screen_width = 80 as i32;
        } else if screen_width < 51 as i32 {
            screen_width = 51 as i32;
        }
        if screen_width != old_width {
            (*bp).width = screen_width - 1 as i32;
            (*bp).buffer = xrealloc(
                (*bp).buffer as *mut libc::c_void,
                ((*bp).width * 2 as i32 + 100 as i32) as size_t,
            ) as *mut i8;
            force_screen_update = 1 as i32 != 0;
        }
        ::core::ptr::write_volatile(
            &mut received_sigwinch as *mut sig_atomic_t,
            0 as i32,
        );
    }
    if (*bp).dltime - (*bp).last_screen_update < 0.2f64 && !force_screen_update {
        return;
    }
    create_image(bp, (*bp).dltime, 0 as i32 != 0);
    display_image((*bp).buffer);
    (*bp).last_screen_update = (*bp).dltime;
}
unsafe extern "C" fn bar_finish(
    mut progress: *mut libc::c_void,
    mut dltime: libc::c_double,
) {
    let mut bp: *mut bar_progress = progress as *mut bar_progress;
    if (*bp).total_length > 0 as i32 as i64
        && (*bp).count + (*bp).initial_length > (*bp).total_length
    {
        (*bp).total_length = (*bp).initial_length + (*bp).count;
    }
    create_image(bp, dltime, 1 as i32 != 0);
    display_image((*bp).buffer);
    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
    logputs(log_options::LOG_PROGRESS, b"\n\0" as *const u8 as *const i8);
    rpl_free((*bp).f_download as *mut libc::c_void);
    (*bp).f_download = 0 as *mut i8;
    rpl_free((*bp).buffer as *mut libc::c_void);
    (*bp).buffer = 0 as *mut i8;
    rpl_free(bp as *mut libc::c_void);
    bp = 0 as *mut bar_progress;
}
unsafe extern "C" fn update_speed_ring(
    mut bp: *mut bar_progress,
    mut howmuch: wgint,
    mut dltime: libc::c_double,
) {
    let mut hist: *mut bar_progress_hist = &mut (*bp).hist;
    let mut recent_age: libc::c_double = dltime - (*bp).recent_start;
    (*bp).recent_bytes += howmuch;
    if recent_age < 0.15f64 {
        return;
    }
    if howmuch == 0 as i32 as i64 {
        if recent_age >= 5 as i32 as libc::c_double {
            (*bp).stalled = 1 as i32 != 0;
            memset(
                hist as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<bar_progress_hist>() as u64,
            );
            (*bp).recent_bytes = 0 as i32 as wgint;
        }
        return;
    }
    if (*bp).stalled {
        (*bp).stalled = 0 as i32 != 0;
        recent_age = 1 as i32 as libc::c_double;
    }
    (*hist).total_time -= (*hist).times[(*hist).pos as usize];
    (*hist).total_bytes -= (*hist).bytes[(*hist).pos as usize];
    (*hist).times[(*hist).pos as usize] = recent_age;
    (*hist).bytes[(*hist).pos as usize] = (*bp).recent_bytes;
    (*hist).total_time += recent_age;
    (*hist).total_bytes += (*bp).recent_bytes;
    (*bp).recent_start = dltime;
    (*bp).recent_bytes = 0 as i32 as wgint;
    (*hist).pos += 1;
    if (*hist).pos == 20 as i32 {
        (*hist).pos = 0 as i32;
    }
}
unsafe extern "C" fn count_cols(mut mbs: *const i8) -> i32 {
    let mut mbc: mbchar_t = mbchar {
        ptr: 0 as *const i8,
        bytes: 0,
        wc_valid: false,
        wc: 0,
        buf: [0; 24],
    };
    let mut iter: mbi_iterator_t = mbi_iterator_t {
        limit: 0 as *const i8,
        in_shift: false,
        state: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed_4 { __wch: 0 },
        },
        next_done: false,
        cur: mbchar {
            ptr: 0 as *const i8,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        },
    };
    let mut cols: i32 = 0 as i32;
    iter.cur.ptr = mbs;
    iter.limit = (iter.cur.ptr).offset(strlen(mbs) as isize);
    iter.in_shift = 0 as i32 != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    iter.next_done = 0 as i32 != 0;
    while iter.cur.ptr < iter.limit
        && {
            mbiter_multi_next(&mut iter);
            1 as i32 != 0
        }
    {
        mbc = iter.cur;
        cols
            += if mbc.wc_valid as i32 != 0 {
                mb_width_aux(mbc.wc as wint_t)
            } else {
                1 as i32
            };
        iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
        iter.next_done = 0 as i32 != 0;
    }
    return cols;
}
unsafe extern "C" fn cols_to_bytes(
    mut mbs: *const i8,
    cols: i32,
    mut ncols: *mut i32,
) -> i32 {
    let mut p_cols: i32 = 0 as i32;
    let mut bytes: i32 = 0 as i32;
    let mut mbc: mbchar_t = mbchar {
        ptr: 0 as *const i8,
        bytes: 0,
        wc_valid: false,
        wc: 0,
        buf: [0; 24],
    };
    let mut iter: mbi_iterator_t = mbi_iterator_t {
        limit: 0 as *const i8,
        in_shift: false,
        state: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed_4 { __wch: 0 },
        },
        next_done: false,
        cur: mbchar {
            ptr: 0 as *const i8,
            bytes: 0,
            wc_valid: false,
            wc: 0,
            buf: [0; 24],
        },
    };
    iter.cur.ptr = mbs;
    iter.limit = (iter.cur.ptr).offset(strlen(mbs) as isize);
    iter.in_shift = 0 as i32 != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    iter.next_done = 0 as i32 != 0;
    while p_cols < cols
        && (iter.cur.ptr < iter.limit
            && {
                mbiter_multi_next(&mut iter);
                1 as i32 != 0
            })
    {
        mbc = iter.cur;
        p_cols
            += if mbc.wc_valid as i32 != 0 {
                mb_width_aux(mbc.wc as wint_t)
            } else {
                1 as i32
            };
        if p_cols > cols {
            p_cols
                -= if mbc.wc_valid as i32 != 0 {
                    mb_width_aux(mbc.wc as wint_t)
                } else {
                    1 as i32
                };
            break;
        } else {
            bytes = (bytes as u64).wrapping_add(mbc.bytes) as i32 as i32;
            iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
            iter.next_done = 0 as i32 != 0;
        }
    }
    *ncols = p_cols;
    return bytes;
}
unsafe extern "C" fn get_eta(mut bcd: *mut i32) -> *const i8 {
    static mut eta_str: [i8; 11] = unsafe {
        *::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"    eta %s\0")
    };
    static mut eta_trans: *const i8 = 0 as *const i8;
    static mut bytes_cols_diff: i32 = 0;
    if eta_trans.is_null() {
        let mut nbytes: i32 = 0;
        let mut ncols: i32 = 0;
        eta_trans = dcgettext(0 as *const i8, eta_str.as_ptr(), 5 as i32);
        nbytes = strlen(eta_trans) as i32;
        ncols = count_cols(eta_trans);
        bytes_cols_diff = nbytes - ncols;
    }
    if !bcd.is_null() {
        *bcd = bytes_cols_diff;
    }
    return eta_trans;
}
unsafe extern "C" fn create_image(
    mut bp: *mut bar_progress,
    mut dl_total_time: libc::c_double,
    mut done: bool,
) {
    let MAX_FILENAME_COLS: i32 = (*bp).width / 4 as i32;
    let mut p: *mut i8 = (*bp).buffer;
    let mut size: wgint = (*bp).initial_length + (*bp).count;
    let mut hist: *mut bar_progress_hist = &mut (*bp).hist;
    let mut orig_filename_cols: i32 = count_cols((*bp).f_download);
    let mut padding: i32 = 0;
    let mut progress_size: i32 = (*bp).width
        - (MAX_FILENAME_COLS + 1 as i32 + 4 as i32 + 2 as i32 + 7 as i32 + 1 as i32
            + 8 as i32 + 2 as i32 + 15 as i32);
    let mut bytes_cols_diff: i32 = 0 as i32;
    let mut cols_diff: i32 = 0;
    let mut down_size: *const i8 = 0 as *const i8;
    if progress_size < 5 as i32 {
        progress_size = 0 as i32;
    }
    if dl_total_time >= 2147483647 as i32 as libc::c_double {
        dl_total_time = (2147483647 as i32 - 1 as i32) as libc::c_double;
    } else if dl_total_time < 0 as i32 as libc::c_double {
        dl_total_time = 0 as i32 as libc::c_double;
    }
    if orig_filename_cols < MAX_FILENAME_COLS {
        p = p
            .offset(
                sprintf(p, b"%s\0" as *const u8 as *const i8, (*bp).f_download) as isize,
            );
        padding = MAX_FILENAME_COLS - orig_filename_cols + 1 as i32;
        memset(p as *mut libc::c_void, ' ' as i32, padding as u64);
        p = p.offset(padding as isize);
    } else {
        let mut offset_cols: i32 = 0;
        let mut bytes_in_filename: i32 = 0;
        let mut offset_bytes: i32 = 0;
        let mut col: i32 = 0;
        let mut cols_ret: *mut i32 = &mut col;
        if orig_filename_cols > MAX_FILENAME_COLS + 5 as i32 && !opt.noscroll && !done {
            offset_cols = ((*bp).tick + orig_filename_cols
                + MAX_FILENAME_COLS / 2 as i32)
                % (orig_filename_cols + MAX_FILENAME_COLS);
            if offset_cols > orig_filename_cols {
                padding = MAX_FILENAME_COLS - (offset_cols - orig_filename_cols);
                memset(p as *mut libc::c_void, ' ' as i32, padding as u64);
                p = p.offset(padding as isize);
                offset_cols = 0 as i32;
            } else {
                padding = 0 as i32;
            }
        } else {
            padding = 0 as i32;
            offset_cols = 0 as i32;
        }
        offset_bytes = cols_to_bytes((*bp).f_download, offset_cols, cols_ret);
        bytes_in_filename = cols_to_bytes(
            ((*bp).f_download).offset(offset_bytes as isize),
            MAX_FILENAME_COLS - padding,
            cols_ret,
        );
        memcpy(
            p as *mut libc::c_void,
            ((*bp).f_download).offset(offset_bytes as isize) as *const libc::c_void,
            bytes_in_filename as u64,
        );
        p = p.offset(bytes_in_filename as isize);
        padding = MAX_FILENAME_COLS - (padding + *cols_ret);
        memset(p as *mut libc::c_void, ' ' as i32, (padding + 1 as i32) as u64);
        p = p.offset((padding + 1 as i32) as isize);
    }
    if (*bp).total_length > 0 as i32 as i64 {
        let mut percentage: i32 = (100.0f64 * size as libc::c_double
            / (*bp).total_length as libc::c_double) as i32;
        p = p
            .offset(
                sprintf(p, b"%3d%%\0" as *const u8 as *const i8, percentage) as isize,
            );
    } else {
        memset(p as *mut libc::c_void, ' ' as i32, 4 as i32 as u64);
        p = p.offset(4 as i32 as isize);
    }
    if progress_size != 0 && (*bp).total_length > 0 as i32 as i64 {
        let mut insz: i32 = ((*bp).initial_length as libc::c_double
            / (*bp).total_length as libc::c_double * progress_size as libc::c_double)
            as i32;
        let mut dlsz: i32 = (size as libc::c_double
            / (*bp).total_length as libc::c_double * progress_size as libc::c_double)
            as i32;
        let mut begin: *mut i8 = 0 as *mut i8;
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = '[' as i32 as i8;
        begin = p;
        memset(p as *mut libc::c_void, '+' as i32, insz as u64);
        p = p.offset(insz as isize);
        dlsz -= insz;
        if dlsz > 0 as i32 {
            memset(p as *mut libc::c_void, '=' as i32, (dlsz - 1 as i32) as u64);
            p = p.offset((dlsz - 1 as i32) as isize);
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = '>' as i32 as i8;
        }
        memset(
            p as *mut libc::c_void,
            ' ' as i32,
            (progress_size as i64 - p.offset_from(begin) as i64) as u64,
        );
        p = p.offset((progress_size as i64 - p.offset_from(begin) as i64) as isize);
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = ']' as i32 as i8;
    } else if progress_size != 0 {
        let mut ind: i32 = (*bp).tick % (progress_size * 2 as i32 - 6 as i32);
        let mut i: i32 = 0;
        let mut pos: i32 = 0;
        if ind < progress_size - 2 as i32 {
            pos = ind + 1 as i32;
        } else {
            pos = progress_size - (ind - progress_size + 5 as i32);
        }
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = '[' as i32 as i8;
        i = 0 as i32;
        while i < progress_size {
            if i == pos - 1 as i32 {
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 = '<' as i32 as i8;
            } else if i == pos {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = '=' as i32 as i8;
            } else if i == pos + 1 as i32 {
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = '>' as i32 as i8;
            } else {
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = ' ' as i32 as i8;
            }
            i += 1;
            i;
        }
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = ']' as i32 as i8;
    }
    (*bp).tick += 1;
    (*bp).tick;
    down_size = human_readable(size, 1000 as i32, 2 as i32);
    cols_diff = 7 as i32 + 1 as i32 - count_cols(down_size);
    if cols_diff > 0 as i32 {
        memset(p as *mut libc::c_void, ' ' as i32, cols_diff as u64);
        p = p.offset(cols_diff as isize);
    }
    p = p.offset(sprintf(p, b"%s\0" as *const u8 as *const i8, down_size) as isize);
    if (*hist).total_time > 0 as i32 as libc::c_double && (*hist).total_bytes != 0 {
        static mut short_units: [*const i8; 5] = [
            b" B/s\0" as *const u8 as *const i8,
            b"KB/s\0" as *const u8 as *const i8,
            b"MB/s\0" as *const u8 as *const i8,
            b"GB/s\0" as *const u8 as *const i8,
            b"TB/s\0" as *const u8 as *const i8,
        ];
        static mut short_units_bits: [*const i8; 5] = [
            b" b/s\0" as *const u8 as *const i8,
            b"Kb/s\0" as *const u8 as *const i8,
            b"Mb/s\0" as *const u8 as *const i8,
            b"Gb/s\0" as *const u8 as *const i8,
            b"Tb/s\0" as *const u8 as *const i8,
        ];
        let mut units: i32 = 0 as i32;
        let mut dlquant: wgint = (*hist).total_bytes + (*bp).recent_bytes;
        let mut dltime: libc::c_double = (*hist).total_time
            + (dl_total_time - (*bp).recent_start);
        let mut dlspeed: libc::c_double = calc_rate(dlquant, dltime, &mut units);
        p = p
            .offset(
                sprintf(
                    p,
                    b"  %4.*f%s\0" as *const u8 as *const i8,
                    if dlspeed >= 99.95f64 {
                        0 as i32
                    } else if dlspeed >= 9.995f64 {
                        1 as i32
                    } else {
                        2 as i32
                    },
                    dlspeed,
                    if !opt.report_bps {
                        short_units[units as usize]
                    } else {
                        short_units_bits[units as usize]
                    },
                ) as isize,
            );
    } else {
        memcpy(
            p as *mut libc::c_void,
            b"  --.-KB/s\0" as *const u8 as *const i8 as *const libc::c_void,
            (::core::mem::size_of::<[i8; 11]>() as u64).wrapping_sub(1 as i32 as u64),
        );
        p = p
            .offset(
                (::core::mem::size_of::<[i8; 11]>() as u64).wrapping_sub(1 as i32 as u64)
                    as isize,
            );
    }
    if !done {
        let mut current_block_95: u64;
        if (*bp).total_length > 0 as i32 as i64 && (*bp).count > 0 as i32 as i64
            && dl_total_time > 3 as i32 as libc::c_double
        {
            let mut eta: i32 = 0;
            if (*bp).total_length != size && (*bp).last_eta_value != 0 as i32
                && dl_total_time - (*bp).last_eta_time < 0.99f64
            {
                eta = (*bp).last_eta_value;
                current_block_95 = 9879896046554623444;
            } else {
                let mut bytes_remaining: wgint = (*bp).total_length - size;
                let mut eta_: libc::c_double = dl_total_time
                    * bytes_remaining as libc::c_double / (*bp).count as libc::c_double;
                if eta_ >= (2147483647 as i32 - 1 as i32) as libc::c_double {
                    current_block_95 = 12428075126322162520;
                } else {
                    eta = (eta_ + 0.5f64) as i32;
                    (*bp).last_eta_value = eta;
                    (*bp).last_eta_time = dl_total_time;
                    current_block_95 = 9879896046554623444;
                }
            }
            match current_block_95 {
                12428075126322162520 => {}
                _ => {
                    p = p
                        .offset(
                            sprintf(
                                p,
                                get_eta(&mut bytes_cols_diff),
                                eta_to_human_short(eta, 0 as i32 != 0),
                            ) as isize,
                        );
                    current_block_95 = 1209030638129645089;
                }
            }
        } else if (*bp).total_length > 0 as i32 as i64 {
            current_block_95 = 12428075126322162520;
        } else {
            current_block_95 = 1209030638129645089;
        }
        match current_block_95 {
            12428075126322162520 => {
                memset(p as *mut libc::c_void, ' ' as i32, 15 as i32 as u64);
                p = p.offset(15 as i32 as isize);
            }
            _ => {}
        }
    } else {
        let mut nbytes: i32 = 0;
        let mut ncols: i32 = 0;
        strcpy(
            p,
            dcgettext(0 as *const i8, b"    in \0" as *const u8 as *const i8, 5 as i32),
        );
        nbytes = strlen(p) as i32;
        ncols = count_cols(p);
        bytes_cols_diff = nbytes - ncols;
        if dl_total_time >= 10 as i32 as libc::c_double {
            ncols
                += sprintf(
                    p.offset(nbytes as isize),
                    b"%s\0" as *const u8 as *const i8,
                    eta_to_human_short((dl_total_time + 0.5f64) as i32, 0 as i32 != 0),
                );
        } else {
            ncols
                += sprintf(
                    p.offset(nbytes as isize),
                    b"%ss\0" as *const u8 as *const i8,
                    print_decimal(dl_total_time),
                );
        }
        p = p.offset((ncols + bytes_cols_diff) as isize);
        if ncols < 15 as i32 {
            memset(p as *mut libc::c_void, ' ' as i32, (15 as i32 - ncols) as u64);
            p = p.offset((15 as i32 - ncols) as isize);
        }
    }
    *p = '\0' as i32 as i8;
    padding = (*bp).width - count_cols((*bp).buffer);
    if padding > 0 as i32 {
        memset(p as *mut libc::c_void, ' ' as i32, padding as u64);
        p = p.offset(padding as isize);
        *p = '\0' as i32 as i8;
    }
}
unsafe extern "C" fn display_image(mut buf: *mut i8) {
    let mut old: bool = log_set_save_context(0 as i32 != 0);
    logputs(log_options::LOG_PROGRESS, b"\r\0" as *const u8 as *const i8);
    logputs(log_options::LOG_PROGRESS, buf);
    log_set_save_context(old);
}
unsafe extern "C" fn bar_set_params(mut params: *const i8) {
    (*current_impl).interactive = 1 as i32 != 0;
    if !params.is_null() {
        let mut param: *const i8 = params;
        while *param != 0 {
            if strncmp(param, b"force\0" as *const u8 as *const i8, 5 as i32 as u64) == 0
            {
                current_impl_locked = 1 as i32;
            } else if strncmp(
                param,
                b"noscroll\0" as *const u8 as *const i8,
                8 as i32 as u64,
            ) == 0
            {
                opt.noscroll = 1 as i32 != 0;
            }
            param = strchrnul(param, ':' as i32);
            if *param != 0 {
                param = param.offset(1);
                param;
            }
        }
    }
    if (!(opt.lfilename).is_null() && opt.show_progress != 1 as i32
        || isatty(fileno(stderr)) == 0) && current_impl_locked == 0
    {
        set_progress_implementation(b"dot\0" as *const u8 as *const i8);
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn progress_handle_sigwinch(mut sig: i32) {
    ::core::ptr::write_volatile(&mut received_sigwinch as *mut sig_atomic_t, 1 as i32);
    signal(28 as i32, Some(progress_handle_sigwinch as unsafe extern "C" fn(i32) -> ()));
}
unsafe extern "C" fn eta_to_human_short(
    mut secs: i32,
    mut condensed: bool,
) -> *const i8 {
    static mut buf: [i8; 16] = [0; 16];
    static mut last: i32 = -(1 as i32);
    let mut space: *const i8 = if condensed as i32 != 0 {
        b"\0" as *const u8 as *const i8
    } else {
        b" \0" as *const u8 as *const i8
    };
    if secs == last {
        return buf.as_mut_ptr();
    }
    last = secs;
    if secs < 100 as i32 {
        sprintf(buf.as_mut_ptr(), b"%ds\0" as *const u8 as *const i8, secs);
    } else if secs < 100 as i32 * 60 as i32 {
        sprintf(
            buf.as_mut_ptr(),
            b"%dm%s%ds\0" as *const u8 as *const i8,
            secs / 60 as i32,
            space,
            secs % 60 as i32,
        );
    } else if secs < 48 as i32 * 3600 as i32 {
        sprintf(
            buf.as_mut_ptr(),
            b"%dh%s%dm\0" as *const u8 as *const i8,
            secs / 3600 as i32,
            space,
            secs / 60 as i32 % 60 as i32,
        );
    } else if secs < 100 as i32 * 86400 as i32 {
        sprintf(
            buf.as_mut_ptr(),
            b"%dd%s%dh\0" as *const u8 as *const i8,
            secs / 86400 as i32,
            space,
            secs / 3600 as i32 % 24 as i32,
        );
    } else {
        sprintf(
            buf.as_mut_ptr(),
            b"%dd\0" as *const u8 as *const i8,
            secs / 86400 as i32,
        );
    }
    return buf.as_mut_ptr();
}