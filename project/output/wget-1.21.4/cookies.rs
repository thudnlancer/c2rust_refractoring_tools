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
    pub type hash_table;
    fn time(__timer: *mut time_t) -> time_t;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn quote(arg: *const i8) -> *const i8;
    fn quotearg_style(s: quoting_style, arg: *const i8) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn datetime_str(_: time_t) -> *mut i8;
    fn strdupdelim(_: *const i8, _: *const i8) -> *mut i8;
    fn match_tail(_: *const i8, _: *const i8, _: bool) -> bool;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_get_pair(
        _: *const hash_table,
        _: *const libc::c_void,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> i32;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_iterate(_: *mut hash_table, _: *mut hash_table_iterator);
    fn hash_table_iter_next(_: *mut hash_table_iterator) -> i32;
    fn hash_table_count(_: *const hash_table) -> i32;
    fn make_nocase_string_hash_table(_: i32) -> *mut hash_table;
    fn extract_param(
        _: *mut *const i8,
        _: *mut param_token,
        _: *mut param_token,
        _: i8,
        _: *mut bool,
    ) -> bool;
    fn http_atotm(_: *const i8) -> time_t;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cookie_jar {
    pub chains: *mut hash_table,
    pub cookie_count: i32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct cookie {
    pub domain: *mut i8,
    pub port: i32,
    pub path: *mut i8,
    #[bitfield(name = "discard_requested", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "secure", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "domain_exact", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "permanent", ty = "libc::c_uint", bits = "3..=3")]
    pub discard_requested_secure_domain_exact_permanent: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub expiry_time: time_t,
    pub attr: *mut i8,
    pub value: *mut i8,
    pub next: *mut cookie,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct param_token {
    pub b: *const i8,
    pub e: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct weighed_cookie {
    pub cookie: *mut cookie,
    pub domain_goodness: i32,
    pub path_goodness: i32,
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: i32) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
static mut cookies_now: time_t = 0;
#[no_mangle]
pub unsafe extern "C" fn cookie_jar_new() -> *mut cookie_jar {
    let mut jar: *mut cookie_jar = xmalloc(::core::mem::size_of::<cookie_jar>() as u64)
        as *mut cookie_jar;
    (*jar).chains = make_nocase_string_hash_table(0 as i32);
    (*jar).cookie_count = 0 as i32;
    return jar;
}
unsafe extern "C" fn cookie_new() -> *mut cookie {
    let mut cookie: *mut cookie = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<cookie>() as u64,
    ) as *mut cookie;
    (*cookie).port = -(1 as i32);
    return cookie;
}
unsafe extern "C" fn cookie_expired_p(mut c: *const cookie) -> bool {
    return (*c).expiry_time != 0 as i32 as i64 && (*c).expiry_time < cookies_now;
}
unsafe extern "C" fn delete_cookie(mut cookie: *mut cookie) {
    rpl_free((*cookie).domain as *mut libc::c_void);
    (*cookie).domain = 0 as *mut i8;
    rpl_free((*cookie).path as *mut libc::c_void);
    (*cookie).path = 0 as *mut i8;
    rpl_free((*cookie).attr as *mut libc::c_void);
    (*cookie).attr = 0 as *mut i8;
    rpl_free((*cookie).value as *mut libc::c_void);
    (*cookie).value = 0 as *mut i8;
    rpl_free(cookie as *mut libc::c_void);
    cookie = 0 as *mut cookie;
}
unsafe extern "C" fn find_matching_cookie(
    mut jar: *mut cookie_jar,
    mut cookie: *mut cookie,
    mut prevptr: *mut *mut cookie,
) -> *mut cookie {
    let mut chain: *mut cookie = 0 as *mut cookie;
    let mut prev: *mut cookie = 0 as *mut cookie;
    chain = hash_table_get((*jar).chains, (*cookie).domain as *const libc::c_void)
        as *mut cookie;
    if !chain.is_null() {
        prev = 0 as *mut cookie;
        while !chain.is_null() {
            if 0 as i32 == strcmp((*cookie).path, (*chain).path)
                && 0 as i32 == strcmp((*cookie).attr, (*chain).attr)
                && (*cookie).port == (*chain).port
            {
                *prevptr = prev;
                return chain;
            }
            prev = chain;
            chain = (*chain).next;
        }
    }
    *prevptr = 0 as *mut cookie;
    return 0 as *mut cookie;
}
unsafe extern "C" fn store_cookie(mut jar: *mut cookie_jar, mut cookie: *mut cookie) {
    let mut chain_head: *mut cookie = 0 as *mut cookie;
    let mut chain_key: *mut i8 = 0 as *mut i8;
    if hash_table_get_pair(
        (*jar).chains,
        (*cookie).domain as *const libc::c_void,
        &mut chain_key as *mut *mut i8 as *mut libc::c_void,
        &mut chain_head as *mut *mut cookie as *mut libc::c_void,
    ) != 0
    {
        let mut prev: *mut cookie = 0 as *mut cookie;
        let mut victim: *mut cookie = find_matching_cookie(jar, cookie, &mut prev);
        if !victim.is_null() {
            if !prev.is_null() {
                (*prev).next = (*victim).next;
                (*cookie).next = chain_head;
            } else {
                (*cookie).next = (*victim).next;
            }
            delete_cookie(victim);
            (*jar).cookie_count -= 1;
            (*jar).cookie_count;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Deleted old cookie (to be replaced.)\n\0" as *const u8 as *const i8,
                );
            }
        } else {
            (*cookie).next = chain_head;
        }
    } else {
        (*cookie).next = 0 as *mut cookie;
        chain_key = xstrdup((*cookie).domain);
    }
    hash_table_put(
        (*jar).chains,
        chain_key as *const libc::c_void,
        cookie as *const libc::c_void,
    );
    (*jar).cookie_count += 1;
    (*jar).cookie_count;
    if opt.debug as i64 != 0 {
        let mut exptime: time_t = (*cookie).expiry_time;
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"\nStored cookie %s %d%s %s <%s> <%s> [expiry %s] %s %s\n\0"
                    as *const u8 as *const i8,
                (*cookie).domain,
                (*cookie).port,
                if (*cookie).port == -(1 as i32) {
                    b" (ANY)\0" as *const u8 as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
                (*cookie).path,
                if (*cookie).permanent() as i32 != 0 {
                    b"permanent\0" as *const u8 as *const i8
                } else {
                    b"session\0" as *const u8 as *const i8
                },
                if (*cookie).secure() as i32 != 0 {
                    b"secure\0" as *const u8 as *const i8
                } else {
                    b"insecure\0" as *const u8 as *const i8
                },
                if (*cookie).expiry_time != 0 {
                    datetime_str(exptime)
                } else {
                    b"none\0" as *const u8 as *const i8
                },
                (*cookie).attr,
                (*cookie).value,
            );
        }
    }
}
unsafe extern "C" fn discard_matching_cookie(
    mut jar: *mut cookie_jar,
    mut cookie: *mut cookie,
) {
    let mut prev: *mut cookie = 0 as *mut cookie;
    let mut victim: *mut cookie = 0 as *mut cookie;
    if hash_table_count((*jar).chains) == 0 {
        return;
    }
    victim = find_matching_cookie(jar, cookie, &mut prev);
    if !victim.is_null() {
        if !prev.is_null() {
            (*prev).next = (*victim).next;
        } else {
            let mut chain_key: *mut i8 = 0 as *mut i8;
            let mut res: i32 = 0;
            res = hash_table_get_pair(
                (*jar).chains,
                (*victim).domain as *const libc::c_void,
                &mut chain_key as *mut *mut i8 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
            if res == 0 as i32 {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Unable to get cookie for %s\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*victim).domain,
                );
            }
            if ((*victim).next).is_null() {
                hash_table_remove(
                    (*jar).chains,
                    (*victim).domain as *const libc::c_void,
                );
                rpl_free(chain_key as *mut libc::c_void);
                chain_key = 0 as *mut i8;
            } else {
                hash_table_put(
                    (*jar).chains,
                    chain_key as *const libc::c_void,
                    (*victim).next as *const libc::c_void,
                );
            }
        }
        delete_cookie(victim);
        if opt.debug as i64 != 0 {
            debug_logprintf(b"Discarded old cookie.\n\0" as *const u8 as *const i8);
        }
    }
}
unsafe extern "C" fn parse_set_cookie(
    mut set_cookie: *const i8,
    mut silent: bool,
) -> *mut cookie {
    let mut current_block: u64;
    let mut ptr: *const i8 = set_cookie;
    let mut cookie: *mut cookie = cookie_new();
    let mut name: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    let mut value: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    if extract_param(&mut ptr, &mut name, &mut value, ';' as i32 as i8, 0 as *mut bool) {
        if !(value.b).is_null() {
            if *(value.b).offset(-(1 as i32 as isize)) as i32 == '"' as i32 {
                value.b = (value.b).offset(-1);
                value.b;
            }
            if *value.e as i32 == '"' as i32 {
                value.e = (value.e).offset(1);
                value.e;
            }
            (*cookie).attr = strdupdelim(name.b, name.e);
            (*cookie).value = strdupdelim(value.b, value.e);
            loop {
                if !extract_param(
                    &mut ptr,
                    &mut name,
                    &mut value,
                    ';' as i32 as i8,
                    0 as *mut bool,
                ) {
                    current_block = 16799951812150840583;
                    break;
                }
                if (name.e).offset_from(name.b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 7]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        name.b,
                        b"domain\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 7]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    if !(!(value.b).is_null() && value.b != value.e) {
                        current_block = 17112488718714263259;
                        break;
                    }
                    rpl_free((*cookie).domain as *mut libc::c_void);
                    (*cookie).domain = 0 as *mut i8;
                    if *value.b as i32 == '.' as i32 {
                        value.b = (value.b).offset(1);
                        value.b;
                    }
                    (*cookie).domain = strdupdelim(value.b, value.e);
                } else if (name.e).offset_from(name.b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        name.b,
                        b"path\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 5]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    if !(!(value.b).is_null() && value.b != value.e) {
                        current_block = 17112488718714263259;
                        break;
                    }
                    rpl_free((*cookie).path as *mut libc::c_void);
                    (*cookie).path = 0 as *mut i8;
                    (*cookie).path = strdupdelim(value.b, value.e);
                } else if (name.e).offset_from(name.b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 8]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        name.b,
                        b"expires\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 8]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    let mut value_copy: [i8; 128] = [0; 128];
                    let mut value_len: size_t = (value.e).offset_from(value.b) as i64
                        as size_t;
                    let mut expires: time_t = 0;
                    if !(!(value.b).is_null() && value.b != value.e)
                        || value_len >= ::core::mem::size_of::<[i8; 128]>() as u64
                    {
                        current_block = 17112488718714263259;
                        break;
                    }
                    memcpy(
                        value_copy.as_mut_ptr() as *mut libc::c_void,
                        value.b as *const libc::c_void,
                        value_len,
                    );
                    value_copy[value_len as usize] = 0 as i32 as i8;
                    expires = http_atotm(value_copy.as_mut_ptr());
                    if expires != -(1 as i32) as time_t {
                        (*cookie).set_permanent(1 as i32 as u32);
                        (*cookie).expiry_time = expires;
                        if (*cookie).expiry_time < cookies_now {
                            (*cookie).set_discard_requested(1 as i32 as u32);
                        }
                    }
                } else if (name.e).offset_from(name.b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 8]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        name.b,
                        b"max-age\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 8]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    let mut maxage: libc::c_double = -(1 as i32) as libc::c_double;
                    let mut value_copy_0: [i8; 32] = [0; 32];
                    let mut value_len_0: size_t = (value.e).offset_from(value.b) as i64
                        as size_t;
                    if !(!(value.b).is_null() && value.b != value.e)
                        || value_len_0 >= ::core::mem::size_of::<[i8; 32]>() as u64
                    {
                        current_block = 17112488718714263259;
                        break;
                    }
                    memcpy(
                        value_copy_0.as_mut_ptr() as *mut libc::c_void,
                        value.b as *const libc::c_void,
                        value_len_0,
                    );
                    value_copy_0[value_len_0 as usize] = 0 as i32 as i8;
                    sscanf(
                        value_copy_0.as_mut_ptr(),
                        b"%lf\0" as *const u8 as *const i8,
                        &mut maxage as *mut libc::c_double,
                    );
                    if maxage == -(1 as i32) as libc::c_double {
                        current_block = 17112488718714263259;
                        break;
                    } else {
                        (*cookie).set_permanent(1 as i32 as u32);
                        (*cookie).expiry_time = cookies_now + maxage as time_t;
                        if maxage == 0 as i32 as libc::c_double {
                            (*cookie).set_discard_requested(1 as i32 as u32);
                        }
                    }
                } else if (name.e).offset_from(name.b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 7]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        name.b,
                        b"secure\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 7]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    (*cookie).set_secure(1 as i32 as u32);
                }
            }
            match current_block {
                17112488718714263259 => {}
                _ => {
                    if !(*ptr != 0) {
                        return cookie;
                    }
                }
            }
        }
    }
    if !silent {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Syntax error in Set-Cookie: %s at position %d.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            quotearg_style(quoting_style::escape_quoting_style, set_cookie),
            ptr.offset_from(set_cookie) as i64 as i32,
        );
    }
    delete_cookie(cookie);
    return 0 as *mut cookie;
}
unsafe extern "C" fn numeric_address_p(mut addr: *const i8) -> bool {
    let mut p: *const i8 = addr;
    if !c_isdigit(*p as i32) {
        return 0 as i32 != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as i32) {
        p = p.offset(1);
        p;
    }
    let fresh0 = p;
    p = p.offset(1);
    if *fresh0 as i32 != '.' as i32 {
        return 0 as i32 != 0;
    }
    if !c_isdigit(*p as i32) {
        return 0 as i32 != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as i32) {
        p = p.offset(1);
        p;
    }
    let fresh1 = p;
    p = p.offset(1);
    if *fresh1 as i32 != '.' as i32 {
        return 0 as i32 != 0;
    }
    if !c_isdigit(*p as i32) {
        return 0 as i32 != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as i32) {
        p = p.offset(1);
        p;
    }
    let fresh2 = p;
    p = p.offset(1);
    if *fresh2 as i32 != '.' as i32 {
        return 0 as i32 != 0;
    }
    if !c_isdigit(*p as i32) {
        return 0 as i32 != 0;
    }
    p = p.offset(1);
    p;
    while c_isdigit(*p as i32) {
        p = p.offset(1);
        p;
    }
    if *p as i32 != '\0' as i32 {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn check_domain_match(
    mut cookie_domain: *const i8,
    mut host: *const i8,
) -> bool {
    if opt.debug as i64 != 0 {
        debug_logprintf(b"cdm: 2\n\0" as *const u8 as *const i8);
    }
    if 0 as i32 == strcasecmp(cookie_domain, host) {
        return 1 as i32 != 0;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"cdm: 3\n\0" as *const u8 as *const i8);
    }
    if !match_tail(host, cookie_domain, 1 as i32 != 0) {
        return 0 as i32 != 0;
    }
    let mut p: *const i8 = cookie_domain;
    let mut dccount: i32 = 1 as i32;
    let mut ldcl: i32 = 0 as i32;
    let mut nldcl: i32 = 0 as i32;
    let mut out: i32 = 0;
    if *p as i32 == '.' as i32 {
        p = p.offset(1);
        p;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"cdm: 4\n\0" as *const u8 as *const i8);
    }
    out = 0 as i32;
    while out == 0 {
        match *p as i32 {
            0 => {
                out = 1 as i32;
            }
            46 => {
                if ldcl == 0 as i32 {
                    return 0 as i32 != 0;
                }
                if *p.offset(1 as i32 as isize) as i32 == '\0' as i32 {
                    out = 1 as i32;
                } else {
                    nldcl = ldcl;
                    ldcl = 0 as i32;
                    dccount += 1;
                    dccount;
                }
            }
            _ => {
                ldcl += 1;
                ldcl;
            }
        }
        p = p.offset(1);
        p;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"cdm: 5\n\0" as *const u8 as *const i8);
    }
    if dccount < 2 as i32 {
        return 0 as i32 != 0;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"cdm: 6\n\0" as *const u8 as *const i8);
    }
    if dccount == 2 as i32 {
        let mut i: size_t = 0;
        let mut known_toplevel: i32 = 0 as i32;
        static mut known_toplevel_domains: [*const i8; 7] = [
            b".com\0" as *const u8 as *const i8,
            b".edu\0" as *const u8 as *const i8,
            b".net\0" as *const u8 as *const i8,
            b".org\0" as *const u8 as *const i8,
            b".gov\0" as *const u8 as *const i8,
            b".mil\0" as *const u8 as *const i8,
            b".int\0" as *const u8 as *const i8,
        ];
        i = 0 as i32 as size_t;
        while i
            < (::core::mem::size_of::<[*const i8; 7]>() as u64)
                .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
        {
            if match_tail(
                cookie_domain,
                known_toplevel_domains[i as usize],
                1 as i32 != 0,
            ) {
                known_toplevel = 1 as i32;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if known_toplevel == 0 && nldcl <= 3 as i32 {
            return 0 as i32 != 0;
        }
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"cdm: 7\n\0" as *const u8 as *const i8);
    }
    if *cookie_domain as i32 != '.' as i32 {
        let mut dlen: i32 = strlen(cookie_domain) as i32;
        let mut hlen: i32 = strlen(host) as i32;
        if hlen > dlen
            && *host.offset((hlen - dlen - 1 as i32) as isize) as i32 != '.' as i32
        {
            return 0 as i32 != 0;
        }
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"cdm: 8\n\0" as *const u8 as *const i8);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn check_path_match(
    mut cookie_path: *const i8,
    mut path: *const i8,
) -> bool {
    return path_matches(path, cookie_path) != 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn cookie_handle_set_cookie(
    mut jar: *mut cookie_jar,
    mut host: *const i8,
    mut port: i32,
    mut path: *const i8,
    mut set_cookie: *const i8,
) {
    let mut current_block: u64;
    let mut cookie: *mut cookie = 0 as *mut cookie;
    cookies_now = time(0 as *mut time_t);
    let mut buf: [i8; 1024] = [0; 1024];
    let mut tmp: *mut i8 = 0 as *mut i8;
    let mut pathlen: size_t = strlen(path);
    if pathlen
        < (::core::mem::size_of::<[i8; 1024]>() as u64).wrapping_sub(1 as i32 as u64)
    {
        tmp = buf.as_mut_ptr();
    } else {
        tmp = xmalloc(pathlen.wrapping_add(2 as i32 as u64)) as *mut i8;
    }
    *tmp = '/' as i32 as i8;
    memcpy(
        tmp.offset(1 as i32 as isize) as *mut libc::c_void,
        path as *const libc::c_void,
        pathlen.wrapping_add(1 as i32 as u64),
    );
    path = tmp;
    cookie = parse_set_cookie(set_cookie, 0 as i32 != 0);
    if !cookie.is_null() {
        if ((*cookie).domain).is_null() {
            (*cookie).domain = xstrdup(host);
            (*cookie).set_domain_exact(1 as i32 as u32);
            if port != 80 as i32 && port != 443 as i32 {
                (*cookie).port = port;
            }
        } else if !check_domain_match((*cookie).domain, host) {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Cookie coming from %s attempted to set domain to \0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_style(quoting_style::escape_quoting_style, host),
            );
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(0 as *const i8, b"%s\n\0" as *const u8 as *const i8, 5 as i32),
                quotearg_style(quoting_style::escape_quoting_style, (*cookie).domain),
            );
            (*cookie).set_discard_requested(1 as i32 as u32);
        }
        if ((*cookie).path).is_null() {
            let mut trailing_slash: *mut i8 = strrchr(path, '/' as i32);
            if !trailing_slash.is_null() {
                (*cookie).path = strdupdelim(
                    path,
                    trailing_slash.offset(1 as i32 as isize),
                );
            } else {
                (*cookie).path = xstrdup(path);
            }
            current_block = 15925075030174552612;
        } else if !check_path_match((*cookie).path, path) {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Attempt to fake the path: %s, %s\n\0" as *const u8 as *const i8,
                    (*cookie).path,
                    path,
                );
            }
            current_block = 14398485157480960384;
        } else {
            current_block = 15925075030174552612;
        }
        match current_block {
            14398485157480960384 => {}
            _ => {
                if (*cookie).discard_requested() != 0 {
                    discard_matching_cookie(jar, cookie);
                } else {
                    store_cookie(jar, cookie);
                    if tmp != buf.as_mut_ptr() {
                        rpl_free(tmp as *mut libc::c_void);
                        tmp = 0 as *mut i8;
                    }
                    return;
                }
            }
        }
    }
    if !cookie.is_null() {
        delete_cookie(cookie);
    }
    if tmp != buf.as_mut_ptr() {
        rpl_free(tmp as *mut libc::c_void);
        tmp = 0 as *mut i8;
    }
}
unsafe extern "C" fn count_char(mut string: *const i8, mut chr: i8) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut count: i32 = 0 as i32;
    p = string;
    while *p != 0 {
        if *p as i32 == chr as i32 {
            count += 1;
            count;
        }
        p = p.offset(1);
        p;
    }
    return count;
}
unsafe extern "C" fn find_chains_of_host(
    mut jar: *mut cookie_jar,
    mut host: *const i8,
    mut dest: *mut *mut cookie,
) -> i32 {
    let mut dest_count: i32 = 0 as i32;
    let mut passes: i32 = 0;
    let mut passcnt: i32 = 0;
    if hash_table_count((*jar).chains) == 0 {
        return 0 as i32;
    }
    if numeric_address_p(host) {
        passes = 1 as i32;
    } else {
        passes = count_char(host, '.' as i32 as i8);
    }
    passcnt = 0 as i32;
    loop {
        let mut chain: *mut cookie = hash_table_get(
            (*jar).chains,
            host as *const libc::c_void,
        ) as *mut cookie;
        if !chain.is_null() {
            let fresh3 = dest_count;
            dest_count = dest_count + 1;
            let ref mut fresh4 = *dest.offset(fresh3 as isize);
            *fresh4 = chain;
        }
        passcnt += 1;
        if passcnt >= passes {
            break;
        }
        host = (strchr(host, '.' as i32)).offset(1 as i32 as isize);
    }
    return dest_count;
}
unsafe extern "C" fn path_matches(
    mut full_path: *const i8,
    mut prefix: *const i8,
) -> i32 {
    let mut len: i32 = strlen(prefix) as i32;
    if 0 as i32 != strncmp(full_path, prefix, len as u64) {
        return 0 as i32;
    }
    return len + 1 as i32;
}
unsafe extern "C" fn cookie_matches_url(
    mut cookie: *const cookie,
    mut host: *const i8,
    mut port: i32,
    mut path: *const i8,
    mut secflag: bool,
    mut path_goodness: *mut i32,
) -> bool {
    let mut pg: i32 = 0;
    if cookie_expired_p(cookie) {
        return 0 as i32 != 0;
    }
    if (*cookie).secure() as i32 != 0 && !secflag {
        return 0 as i32 != 0;
    }
    if (*cookie).port != -(1 as i32) && (*cookie).port != port {
        return 0 as i32 != 0;
    }
    if (*cookie).domain_exact() as i32 != 0
        && 0 as i32 != strcasecmp(host, (*cookie).domain)
    {
        return 0 as i32 != 0;
    }
    pg = path_matches(path, (*cookie).path);
    if pg == 0 as i32 {
        return 0 as i32 != 0;
    }
    if !path_goodness.is_null() {
        *path_goodness = pg;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn equality_comparator(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut wc1: *mut weighed_cookie = p1 as *mut weighed_cookie;
    let mut wc2: *mut weighed_cookie = p2 as *mut weighed_cookie;
    let mut namecmp: i32 = strcmp((*(*wc1).cookie).attr, (*(*wc2).cookie).attr);
    let mut valuecmp: i32 = strcmp((*(*wc1).cookie).value, (*(*wc2).cookie).value);
    return if namecmp != 0 { namecmp } else { valuecmp };
}
unsafe extern "C" fn eliminate_dups(
    mut outgoing: *mut weighed_cookie,
    mut count: i32,
) -> i32 {
    let mut h: *mut weighed_cookie = 0 as *mut weighed_cookie;
    let mut t: *mut weighed_cookie = 0 as *mut weighed_cookie;
    let mut end: *mut weighed_cookie = outgoing.offset(count as isize);
    qsort(
        outgoing as *mut libc::c_void,
        count as size_t,
        ::core::mem::size_of::<weighed_cookie>() as u64,
        Some(
            equality_comparator
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    let mut current_block_4: u64;
    t = outgoing;
    h = t;
    while h < end {
        if h != end.offset(-(1 as i32 as isize)) {
            let mut c0: *mut cookie = (*h.offset(0 as i32 as isize)).cookie;
            let mut c1: *mut cookie = (*h.offset(1 as i32 as isize)).cookie;
            if strcmp((*c0).attr, (*c1).attr) == 0
                && strcmp((*c0).value, (*c1).value) == 0
            {
                current_block_4 = 4988723283678924448;
            } else {
                current_block_4 = 2473556513754201174;
            }
        } else {
            current_block_4 = 2473556513754201174;
        }
        match current_block_4 {
            2473556513754201174 => {
                if h != t {
                    let fresh5 = t;
                    t = t.offset(1);
                    *fresh5 = *h;
                } else {
                    t = t.offset(1);
                    t;
                }
            }
            _ => {}
        }
        h = h.offset(1);
        h;
    }
    return t.offset_from(outgoing) as i64 as i32;
}
unsafe extern "C" fn goodness_comparator(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut wc1: *mut weighed_cookie = p1 as *mut weighed_cookie;
    let mut wc2: *mut weighed_cookie = p2 as *mut weighed_cookie;
    let mut dgdiff: i32 = (*wc2).domain_goodness - (*wc1).domain_goodness;
    let mut pgdiff: i32 = (*wc2).path_goodness - (*wc1).path_goodness;
    return if dgdiff != 0 { dgdiff } else { pgdiff };
}
#[no_mangle]
pub unsafe extern "C" fn cookie_header(
    mut jar: *mut cookie_jar,
    mut host: *const i8,
    mut port: i32,
    mut path: *const i8,
    mut secflag: bool,
) -> *mut i8 {
    let mut chains: [*mut cookie; 32] = [0 as *mut cookie; 32];
    let mut chain_count: i32 = 0;
    let mut cookie: *mut cookie = 0 as *mut cookie;
    let mut outgoing: *mut weighed_cookie = 0 as *mut weighed_cookie;
    let mut count: size_t = 0;
    let mut i: size_t = 0;
    let mut ocnt: size_t = 0;
    let mut result: *mut i8 = 0 as *mut i8;
    let mut result_size: i32 = 0;
    let mut pos: i32 = 0;
    let mut pathbuf: [i8; 1024] = [0; 1024];
    chain_count = 1 as i32 + count_char(host, '.' as i32 as i8);
    if chain_count
        > (::core::mem::size_of::<[*mut cookie; 32]>() as u64)
            .wrapping_div(::core::mem::size_of::<*mut cookie>() as u64) as i32
    {
        return 0 as *mut i8;
    }
    chain_count = find_chains_of_host(jar, host, chains.as_mut_ptr());
    if chain_count <= 0 as i32 {
        return 0 as *mut i8;
    }
    let mut tmp: *mut i8 = 0 as *mut i8;
    let mut pathlen: size_t = strlen(path);
    if pathlen
        < (::core::mem::size_of::<[i8; 1024]>() as u64).wrapping_sub(1 as i32 as u64)
    {
        tmp = pathbuf.as_mut_ptr();
    } else {
        tmp = xmalloc(pathlen.wrapping_add(2 as i32 as u64)) as *mut i8;
    }
    *tmp = '/' as i32 as i8;
    memcpy(
        tmp.offset(1 as i32 as isize) as *mut libc::c_void,
        path as *const libc::c_void,
        pathlen.wrapping_add(1 as i32 as u64),
    );
    path = tmp;
    cookies_now = time(0 as *mut time_t);
    count = 0 as i32 as size_t;
    i = 0 as i32 as size_t;
    while i < chain_count as u32 as u64 {
        cookie = chains[i as usize];
        while !cookie.is_null() {
            if cookie_matches_url(cookie, host, port, path, secflag, 0 as *mut i32) {
                count = count.wrapping_add(1);
                count;
            }
            cookie = (*cookie).next;
        }
        i = i.wrapping_add(1);
        i;
    }
    if !(count == 0) {
        if !(count
            > (18446744073709551615 as u64)
                .wrapping_div(::core::mem::size_of::<weighed_cookie>() as u64))
        {
            outgoing = xmalloc(
                count.wrapping_mul(::core::mem::size_of::<weighed_cookie>() as u64),
            ) as *mut weighed_cookie;
            ocnt = 0 as i32 as size_t;
            i = 0 as i32 as size_t;
            while i < chain_count as u32 as u64 {
                cookie = chains[i as usize];
                while !cookie.is_null() {
                    let mut pg: i32 = 0;
                    if cookie_matches_url(cookie, host, port, path, secflag, &mut pg) {
                        let ref mut fresh6 = (*outgoing.offset(ocnt as isize)).cookie;
                        *fresh6 = cookie;
                        (*outgoing.offset(ocnt as isize)).domain_goodness = strlen(
                            (*cookie).domain,
                        ) as i32;
                        (*outgoing.offset(ocnt as isize)).path_goodness = pg;
                        ocnt = ocnt.wrapping_add(1);
                        ocnt;
                    }
                    cookie = (*cookie).next;
                }
                i = i.wrapping_add(1);
                i;
            }
            count = eliminate_dups(outgoing, count as i32) as size_t;
            qsort(
                outgoing as *mut libc::c_void,
                count,
                ::core::mem::size_of::<weighed_cookie>() as u64,
                Some(
                    goodness_comparator
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> i32,
                ),
            );
            result_size = 0 as i32;
            i = 0 as i32 as size_t;
            while i < count {
                let mut c: *mut cookie = (*outgoing.offset(i as isize)).cookie;
                result_size = (result_size as u64)
                    .wrapping_add(
                        (strlen((*c).attr))
                            .wrapping_add(1 as i32 as u64)
                            .wrapping_add(strlen((*c).value)),
                    ) as i32 as i32;
                i = i.wrapping_add(1);
                i;
            }
            result_size = (result_size as u64)
                .wrapping_add(
                    count.wrapping_sub(1 as i32 as u64).wrapping_mul(2 as i32 as u64),
                )
                .wrapping_add(1 as i32 as u64) as i32;
            result = xmalloc(result_size as size_t) as *mut i8;
            pos = 0 as i32;
            i = 0 as i32 as size_t;
            while i < count {
                let mut c_0: *mut cookie = (*outgoing.offset(i as isize)).cookie;
                let mut namlen: i32 = strlen((*c_0).attr) as i32;
                let mut vallen: i32 = strlen((*c_0).value) as i32;
                memcpy(
                    result.offset(pos as isize) as *mut libc::c_void,
                    (*c_0).attr as *const libc::c_void,
                    namlen as u64,
                );
                pos += namlen;
                let fresh7 = pos;
                pos = pos + 1;
                *result.offset(fresh7 as isize) = '=' as i32 as i8;
                memcpy(
                    result.offset(pos as isize) as *mut libc::c_void,
                    (*c_0).value as *const libc::c_void,
                    vallen as u64,
                );
                pos += vallen;
                if i < count.wrapping_sub(1 as i32 as u64) {
                    let fresh8 = pos;
                    pos = pos + 1;
                    *result.offset(fresh8 as isize) = ';' as i32 as i8;
                    let fresh9 = pos;
                    pos = pos + 1;
                    *result.offset(fresh9 as isize) = ' ' as i32 as i8;
                }
                i = i.wrapping_add(1);
                i;
            }
            let fresh10 = pos;
            pos = pos + 1;
            *result.offset(fresh10 as isize) = '\0' as i32 as i8;
            rpl_free(outgoing as *mut libc::c_void);
            outgoing = 0 as *mut weighed_cookie;
        }
    }
    if path != pathbuf.as_mut_ptr() {
        rpl_free(path as *mut libc::c_void);
        path = 0 as *const i8;
    }
    return result;
}
unsafe extern "C" fn domain_port(
    mut domain_b: *const i8,
    mut domain_e: *const i8,
    mut domain_e_ptr: *mut *const i8,
) -> i32 {
    let mut port: i32 = 0 as i32;
    let mut p: *const i8 = 0 as *const i8;
    let mut colon: *const i8 = memchr(
        domain_b as *const libc::c_void,
        ':' as i32,
        domain_e.offset_from(domain_b) as i64 as u64,
    ) as *const i8;
    if colon.is_null() {
        return 0 as i32;
    }
    p = colon.offset(1 as i32 as isize);
    while p < domain_e && c_isdigit(*p as i32) as i32 != 0 {
        port = 10 as i32 * port + (*p as i32 - '0' as i32);
        p = p.offset(1);
        p;
    }
    if p < domain_e {
        return 0 as i32;
    }
    *domain_e_ptr = colon;
    return port;
}
#[no_mangle]
pub unsafe extern "C" fn cookie_jar_load(mut jar: *mut cookie_jar, mut file: *const i8) {
    let mut line: *mut i8 = 0 as *mut i8;
    let mut bufsize: size_t = 0 as i32 as size_t;
    let mut fp: *mut FILE = rpl_fopen(file, b"r\0" as *const u8 as *const i8);
    if fp.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Cannot open cookies file %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
        return;
    }
    cookies_now = time(0 as *mut time_t);
    while getline(&mut line, &mut bufsize, fp) > 0 as i32 as i64 {
        let mut cookie: *mut cookie = 0 as *mut cookie;
        let mut p: *mut i8 = line;
        let mut expiry: libc::c_double = 0.;
        let mut port: i32 = 0;
        let mut domain_b: *mut i8 = 0 as *mut i8;
        let mut domain_e: *mut i8 = 0 as *mut i8;
        let mut domflag_b: *mut i8 = 0 as *mut i8;
        let mut domflag_e: *mut i8 = 0 as *mut i8;
        let mut path_b: *mut i8 = 0 as *mut i8;
        let mut path_e: *mut i8 = 0 as *mut i8;
        let mut secure_b: *mut i8 = 0 as *mut i8;
        let mut secure_e: *mut i8 = 0 as *mut i8;
        let mut expires_b: *mut i8 = 0 as *mut i8;
        let mut expires_e: *mut i8 = 0 as *mut i8;
        let mut name_b: *mut i8 = 0 as *mut i8;
        let mut name_e: *mut i8 = 0 as *mut i8;
        let mut value_b: *mut i8 = 0 as *mut i8;
        let mut value_e: *mut i8 = 0 as *mut i8;
        while *p as i32 != 0 && c_isspace(*p as i32) as i32 != 0 {
            p = p.offset(1);
            p;
        }
        if *p == 0 || *p as i32 == '#' as i32 {
            continue;
        }
        domain_b = p;
        while *p as i32 != 0 && *p as i32 != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        domain_e = p;
        if domain_b == domain_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        domflag_b = p;
        while *p as i32 != 0 && *p as i32 != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        domflag_e = p;
        if domflag_b == domflag_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        path_b = p;
        while *p as i32 != 0 && *p as i32 != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        path_e = p;
        if path_b == path_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        secure_b = p;
        while *p as i32 != 0 && *p as i32 != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        secure_e = p;
        if secure_b == secure_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        expires_b = p;
        while *p as i32 != 0 && *p as i32 != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        expires_e = p;
        if expires_b == expires_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        name_b = p;
        while *p as i32 != 0 && *p as i32 != '\t' as i32 {
            p = p.offset(1);
            p;
        }
        name_e = p;
        if name_b == name_e || *p == 0 {
            continue;
        }
        p = p.offset(1);
        p;
        value_b = p;
        value_e = p.offset(strlen(p) as isize);
        if value_e > value_b
            && *value_e.offset(-(1 as i32) as isize) as i32 == '\n' as i32
        {
            value_e = value_e.offset(-1);
            value_e;
        }
        if value_e > value_b
            && *value_e.offset(-(1 as i32) as isize) as i32 == '\r' as i32
        {
            value_e = value_e.offset(-1);
            value_e;
        }
        cookie = cookie_new();
        (*cookie).attr = strdupdelim(name_b, name_e);
        (*cookie).value = strdupdelim(value_b, value_e);
        (*cookie).path = strdupdelim(path_b, path_e);
        (*cookie)
            .set_secure(
                (secure_e.offset_from(secure_b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && memcmp(
                        secure_b as *const libc::c_void,
                        b"TRUE\0" as *const u8 as *const i8 as *const libc::c_void,
                        (::core::mem::size_of::<[i8; 5]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0) as i32 as u32,
            );
        (*cookie)
            .set_domain_exact(
                !(domflag_e.offset_from(domflag_b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && memcmp(
                        domflag_b as *const libc::c_void,
                        b"TRUE\0" as *const u8 as *const i8 as *const libc::c_void,
                        (::core::mem::size_of::<[i8; 5]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0) as i32 as u32,
            );
        port = domain_port(
            domain_b,
            domain_e,
            &mut domain_e as *mut *mut i8 as *mut *const i8,
        );
        if port != 0 {
            (*cookie).port = port;
        }
        if *domain_b as i32 == '.' as i32 {
            domain_b = domain_b.offset(1);
            domain_b;
        }
        (*cookie).domain = strdupdelim(domain_b, domain_e);
        expiry = cookies_now as libc::c_double - 1 as i32 as libc::c_double;
        *expires_e = '\0' as i32 as i8;
        sscanf(
            expires_b,
            b"%lf\0" as *const u8 as *const i8,
            &mut expiry as *mut libc::c_double,
        );
        if !(expiry == 0 as i32 as libc::c_double) {
            if expiry < cookies_now as libc::c_double {
                delete_cookie(cookie);
                continue;
            } else {
                (*cookie).expiry_time = expiry as time_t;
                (*cookie).set_permanent(1 as i32 as u32);
            }
        }
        store_cookie(jar, cookie);
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut i8;
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn cookie_jar_save(mut jar: *mut cookie_jar, mut file: *const i8) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    if opt.debug as i64 != 0 {
        debug_logprintf(b"Saving cookies to %s.\n\0" as *const u8 as *const i8, file);
    }
    cookies_now = time(0 as *mut time_t);
    fp = rpl_fopen(file, b"w\0" as *const u8 as *const i8);
    if fp.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Cannot open cookies file %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
        return;
    }
    fputs(b"# HTTP Cookie File\n\0" as *const u8 as *const i8, fp);
    fprintf(
        fp,
        b"# Generated by Wget on %s.\n\0" as *const u8 as *const i8,
        datetime_str(cookies_now),
    );
    fputs(b"# Edit at your own risk.\n\n\0" as *const u8 as *const i8, fp);
    hash_table_iterate((*jar).chains, &mut iter);
    's_56: while hash_table_iter_next(&mut iter) != 0 {
        let mut domain: *const i8 = iter.key as *const i8;
        let mut cookie: *mut cookie = iter.value as *mut cookie;
        while !cookie.is_null() {
            if !((*cookie).permanent() == 0 && !opt.keep_session_cookies) {
                if !cookie_expired_p(cookie) {
                    if (*cookie).domain_exact() == 0 {
                        fputc('.' as i32, fp);
                    }
                    fputs(domain, fp);
                    if (*cookie).port != -(1 as i32) {
                        fprintf(fp, b":%d\0" as *const u8 as *const i8, (*cookie).port);
                    }
                    fprintf(
                        fp,
                        b"\t%s\t%s\t%s\t%.0f\t%s\t%s\n\0" as *const u8 as *const i8,
                        if (*cookie).domain_exact() as i32 != 0 {
                            b"FALSE\0" as *const u8 as *const i8
                        } else {
                            b"TRUE\0" as *const u8 as *const i8
                        },
                        (*cookie).path,
                        if (*cookie).secure() as i32 != 0 {
                            b"TRUE\0" as *const u8 as *const i8
                        } else {
                            b"FALSE\0" as *const u8 as *const i8
                        },
                        (*cookie).expiry_time as libc::c_double,
                        (*cookie).attr,
                        (*cookie).value,
                    );
                    if ferror(fp) != 0 {
                        break 's_56;
                    }
                }
            }
            cookie = (*cookie).next;
        }
    }
    if ferror(fp) != 0 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Error writing to %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
    }
    if fclose(fp) < 0 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Error closing %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"Done saving cookies.\n\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cookie_jar_delete(mut jar: *mut cookie_jar) {
    let mut iter: hash_table_iterator = hash_table_iterator {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
        pos: 0 as *mut libc::c_void,
        end: 0 as *mut libc::c_void,
    };
    hash_table_iterate((*jar).chains, &mut iter);
    while hash_table_iter_next(&mut iter) != 0 {
        let mut chain: *mut cookie = iter.value as *mut cookie;
        rpl_free(iter.key);
        iter.key = 0 as *mut libc::c_void;
        while !chain.is_null() {
            let mut next: *mut cookie = (*chain).next;
            delete_cookie(chain);
            chain = next;
        }
    }
    hash_table_destroy((*jar).chains);
    rpl_free(jar as *mut libc::c_void);
    jar = 0 as *mut cookie_jar;
}