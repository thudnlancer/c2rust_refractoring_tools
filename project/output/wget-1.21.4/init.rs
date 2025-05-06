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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut exec_name: *const i8;
    fn rpl_strtol(string: *const i8, endptr: *mut *mut i8, base: i32) -> i64;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn debug_logprintf(_: *const i8, _: ...);
    fn log_close();
    fn quote(arg: *const i8) -> *const i8;
    fn inform_exit_status(err: uerr_t);
    fn getuid() -> __uid_t;
    fn __errno_location() -> *mut i32;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn strdupdelim(_: *const i8, _: *const i8) -> *mut i8;
    fn sepstring(_: *const i8) -> *mut *mut i8;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn concat_strings(_: *const i8, _: ...) -> *mut i8;
    fn file_exists_p(_: *const i8, _: *mut file_stats_t) -> bool;
    fn fopen_stat(_: *const i8, _: *const i8, _: *mut file_stats_t) -> *mut FILE;
    fn free_vec(_: *mut *mut i8);
    fn merge_vecs(_: *mut *mut i8, _: *mut *mut i8) -> *mut *mut i8;
    fn vec_append(_: *mut *mut i8, _: *const i8) -> *mut *mut i8;
    fn valid_progress_implementation_p(_: *const i8) -> bool;
    static mut output_stream: *mut FILE;
    fn warc_close();
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    static mut cleaned_up: i32;
}
pub type __int64_t = i64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
pub type wgint = int64_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum CHECK_CERT_MODES {
    CHECK_CERT_OFF,
    CHECK_CERT_ON,
    CHECK_CERT_QUIET,
}
impl CHECK_CERT_MODES {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            CHECK_CERT_MODES::CHECK_CERT_OFF => 0,
            CHECK_CERT_MODES::CHECK_CERT_ON => 1,
            CHECK_CERT_MODES::CHECK_CERT_QUIET => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> CHECK_CERT_MODES {
        match value {
            0 => CHECK_CERT_MODES::CHECK_CERT_OFF,
            1 => CHECK_CERT_MODES::CHECK_CERT_ON,
            2 => CHECK_CERT_MODES::CHECK_CERT_QUIET,
            _ => panic!("Invalid value for CHECK_CERT_MODES: {}", value),
        }
    }
}
impl AddAssign<u32> for CHECK_CERT_MODES {
    fn add_assign(&mut self, rhs: u32) {
        *self = CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for CHECK_CERT_MODES {
    fn sub_assign(&mut self, rhs: u32) {
        *self = CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for CHECK_CERT_MODES {
    fn mul_assign(&mut self, rhs: u32) {
        *self = CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for CHECK_CERT_MODES {
    fn div_assign(&mut self, rhs: u32) {
        *self = CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for CHECK_CERT_MODES {
    fn rem_assign(&mut self, rhs: u32) {
        *self = CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for CHECK_CERT_MODES {
    type Output = CHECK_CERT_MODES;
    fn add(self, rhs: u32) -> CHECK_CERT_MODES {
        CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for CHECK_CERT_MODES {
    type Output = CHECK_CERT_MODES;
    fn sub(self, rhs: u32) -> CHECK_CERT_MODES {
        CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for CHECK_CERT_MODES {
    type Output = CHECK_CERT_MODES;
    fn mul(self, rhs: u32) -> CHECK_CERT_MODES {
        CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for CHECK_CERT_MODES {
    type Output = CHECK_CERT_MODES;
    fn div(self, rhs: u32) -> CHECK_CERT_MODES {
        CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for CHECK_CERT_MODES {
    type Output = CHECK_CERT_MODES;
    fn rem(self, rhs: u32) -> CHECK_CERT_MODES {
        CHECK_CERT_MODES::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
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
pub enum parse_line {
    line_ok,
    line_empty,
    line_syntax_error,
    line_unknown_command,
}
impl parse_line {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            parse_line::line_ok => 0,
            parse_line::line_empty => 1,
            parse_line::line_syntax_error => 2,
            parse_line::line_unknown_command => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> parse_line {
        match value {
            0 => parse_line::line_ok,
            1 => parse_line::line_empty,
            2 => parse_line::line_syntax_error,
            3 => parse_line::line_unknown_command,
            _ => panic!("Invalid value for parse_line: {}", value),
        }
    }
}
impl AddAssign<u32> for parse_line {
    fn add_assign(&mut self, rhs: u32) {
        *self = parse_line::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for parse_line {
    fn sub_assign(&mut self, rhs: u32) {
        *self = parse_line::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for parse_line {
    fn mul_assign(&mut self, rhs: u32) {
        *self = parse_line::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for parse_line {
    fn div_assign(&mut self, rhs: u32) {
        *self = parse_line::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for parse_line {
    fn rem_assign(&mut self, rhs: u32) {
        *self = parse_line::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for parse_line {
    type Output = parse_line;
    fn add(self, rhs: u32) -> parse_line {
        parse_line::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for parse_line {
    type Output = parse_line;
    fn sub(self, rhs: u32) -> parse_line {
        parse_line::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for parse_line {
    type Output = parse_line;
    fn mul(self, rhs: u32) -> parse_line {
        parse_line::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for parse_line {
    type Output = parse_line;
    fn div(self, rhs: u32) -> parse_line {
        parse_line::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for parse_line {
    type Output = parse_line;
    fn rem(self, rhs: u32) -> parse_line {
        parse_line::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub name: *const i8,
    pub place: *mut libc::c_void,
    pub action: Option<
        unsafe extern "C" fn(*const i8, *const i8, *mut libc::c_void) -> bool,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decode_item {
    pub name: *const i8,
    pub code: i32,
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
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
static mut commands: [C2RustUnnamed_5; 168] = [C2RustUnnamed_5 {
    name: 0 as *const i8,
    place: 0 as *mut libc::c_void,
    action: None,
}; 168];
unsafe extern "C" fn command_by_name(mut cmdname: *const i8) -> i32 {
    let mut lo: i32 = 0 as i32;
    let mut hi: i32 = (::core::mem::size_of::<[C2RustUnnamed_5; 168]>() as u64)
        .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as u64)
        .wrapping_sub(1 as i32 as u64) as i32;
    while lo <= hi {
        let mut mid: i32 = lo + hi >> 1 as i32;
        let mut cmp: i32 = c_strcasecmp(cmdname, commands[mid as usize].name);
        if cmp < 0 as i32 {
            hi = mid - 1 as i32;
        } else if cmp > 0 as i32 {
            lo = mid + 1 as i32;
        } else {
            return mid
        }
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn defaults() {
    let mut tmp: *mut i8 = 0 as *mut i8;
    memset(
        &mut opt as *mut options as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<options>() as u64,
    );
    opt.cookies = 1 as i32 != 0;
    opt.verbose = -(1 as i32);
    opt.ntry = 20 as i32;
    opt.reclevel = 5 as i32;
    opt.add_hostdir = 1 as i32 != 0;
    opt.netrc = 1 as i32 != 0;
    opt.ftp_glob = 1 as i32 != 0;
    opt.htmlify = 1 as i32 != 0;
    opt.http_keep_alive = 1 as i32 != 0;
    opt.use_proxy = 1 as i32 != 0;
    opt.convert_file_only = 0 as i32 != 0;
    tmp = getenv(b"no_proxy\0" as *const u8 as *const i8);
    if !tmp.is_null() {
        opt.no_proxy = sepstring(tmp);
    }
    opt.prefer_family = C2RustUnnamed::prefer_none;
    opt.allow_cache = 1 as i32 != 0;
    opt.if_modified_since = 1 as i32 != 0;
    opt.read_timeout = 900 as i32 as libc::c_double;
    opt.use_robots = 1 as i32 != 0;
    opt.remove_listing = 1 as i32 != 0;
    opt.dot_bytes = 1024 as i32 as wgint;
    opt.dot_spacing = 10 as i32;
    opt.dots_in_line = 50 as i32;
    opt.dns_cache = 1 as i32 != 0;
    opt.ftp_pasv = 1 as i32 != 0;
    opt.retr_symlinks = 1 as i32 != 0;
    opt.check_cert = CHECK_CERT_MODES::CHECK_CERT_ON as i32;
    opt.ftps_resume_ssl = 1 as i32 != 0;
    opt.ftps_fallback_to_ftp = 0 as i32 != 0;
    opt.ftps_implicit = 0 as i32 != 0;
    opt.ftps_clear_data_connection = 0 as i32 != 0;
    opt.compression = compression_options::compression_none;
    opt.restrict_files_os = C2RustUnnamed_1::restrict_unix;
    opt.restrict_files_ctrl = 1 as i32 != 0;
    opt.restrict_files_nonascii = 0 as i32 != 0;
    opt.restrict_files_case = C2RustUnnamed_0::restrict_no_case_restriction;
    opt.regex_type = C2RustUnnamed_3::regex_type_posix;
    opt.max_redirect = 20 as i32;
    opt.waitretry = 10 as i32 as libc::c_double;
    opt.enable_iri = 1 as i32 != 0;
    opt.locale = 0 as *const i8;
    opt.encoding_remote = 0 as *mut i8;
    opt.useservertimestamps = 1 as i32 != 0;
    opt.show_all_dns_entries = 0 as i32 != 0;
    opt.warc_maxsize = 0 as i32 as wgint;
    opt.warc_compression_enabled = 1 as i32 != 0;
    opt.warc_digests_enabled = 1 as i32 != 0;
    opt.warc_cdx_enabled = 0 as i32 != 0;
    opt.warc_cdx_dedup_filename = 0 as *mut i8;
    opt.warc_tempdir = 0 as *mut i8;
    opt.warc_keep_log = 1 as i32 != 0;
    opt.start_pos = -(1 as i32) as wgint;
    opt.show_progress = -(1 as i32);
    opt.noscroll = 0 as i32 != 0;
    opt.hsts = 1 as i32 != 0;
    opt.enable_xattr = 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn home_dir() -> *mut i8 {
    static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut home: *mut i8 = 0 as *const i8 as *mut i8;
    static mut ret: *mut i8 = 0 as *const i8 as *mut i8;
    if home.is_null() {
        home = getenv(b"HOME\0" as *const u8 as *const i8);
        if home.is_null() {
            let mut pwd: *mut passwd = getpwuid(getuid());
            if pwd.is_null() || ((*pwd).pw_dir).is_null() {
                return 0 as *mut i8;
            }
            home = (*pwd).pw_dir;
        }
    }
    ret = if !home.is_null() { xstrdup(home) } else { 0 as *mut i8 };
    rpl_free(buf as *mut libc::c_void);
    buf = 0 as *mut i8;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wgetrc_env_file_name() -> *mut i8 {
    let mut env: *mut i8 = getenv(b"WGETRC\0" as *const u8 as *const i8);
    if !env.is_null() && *env as i32 != 0 {
        let mut flstat: file_stats_t = file_stats_t {
            access_err: 0,
            st_ino: 0,
            st_dev: 0,
        };
        if !file_exists_p(env, &mut flstat) {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s: WGETRC points to %s, which couldn't be accessed because of error: %s.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                exec_name,
                env,
                strerror(flstat.access_err),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
        }
        return xstrdup(env);
    }
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn ajoin_dir_file(
    mut dir: *const i8,
    mut file: *const i8,
) -> *mut i8 {
    let mut dir_file: *mut i8 = 0 as *mut i8;
    dir_file = aprintf(b"%s/%s\0" as *const u8 as *const i8, dir, file);
    return dir_file;
}
#[no_mangle]
pub unsafe extern "C" fn wgetrc_user_file_name() -> *mut i8 {
    let mut file: *mut i8 = 0 as *mut i8;
    if !(opt.homedir).is_null() {
        file = ajoin_dir_file(opt.homedir, b".wgetrc\0" as *const u8 as *const i8);
    }
    if file.is_null() {
        return 0 as *mut i8;
    }
    if !file_exists_p(file, 0 as *mut file_stats_t) {
        rpl_free(file as *mut libc::c_void);
        file = 0 as *mut i8;
        return 0 as *mut i8;
    }
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn wgetrc_file_name() -> *mut i8 {
    let mut file: *mut i8 = wgetrc_env_file_name();
    if !file.is_null() && *file as i32 != 0 {
        return file;
    }
    file = wgetrc_user_file_name();
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn run_wgetrc(
    mut file: *const i8,
    mut flstats: *mut file_stats_t,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut i8 = 0 as *mut i8;
    let mut bufsize: size_t = 0 as i32 as size_t;
    let mut ln: i32 = 0;
    let mut errcnt: i32 = 0 as i32;
    fp = fopen_stat(file, b"r\0" as *const u8 as *const i8, flstats);
    if fp.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: Cannot read %s (%s).\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            file,
            strerror(*__errno_location()),
        );
        return 1 as i32 != 0;
    }
    ln = 1 as i32;
    while getline(&mut line, &mut bufsize, fp) > 0 as i32 as i64 {
        let mut com: *mut i8 = 0 as *mut i8;
        let mut val: *mut i8 = 0 as *mut i8;
        let mut comind: i32 = 0;
        match parse_line(line, &mut com, &mut val, &mut comind) as u32 {
            0 => {
                if !setval_internal_tilde(comind, com, val) {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: Error in %s at line %d.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        exec_name,
                        file,
                        ln,
                    );
                    errcnt += 1;
                    errcnt;
                }
            }
            2 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: Syntax error in %s at line %d.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    exec_name,
                    file,
                    ln,
                );
                errcnt += 1;
                errcnt;
            }
            3 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: Unknown command %s in %s at line %d.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    exec_name,
                    quote(com),
                    file,
                    ln,
                );
                errcnt += 1;
                errcnt;
            }
            1 => {}
            _ => {
                abort();
            }
        }
        rpl_free(com as *mut libc::c_void);
        com = 0 as *mut i8;
        rpl_free(val as *mut libc::c_void);
        val = 0 as *mut i8;
        ln += 1;
        ln;
    }
    rpl_free(line as *mut libc::c_void);
    line = 0 as *mut i8;
    fclose(fp);
    return errcnt == 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn initialize() -> i32 {
    let mut env_sysrc: *mut i8 = 0 as *mut i8;
    let mut flstats: file_stats_t = file_stats_t {
        access_err: 0,
        st_ino: 0,
        st_dev: 0,
    };
    let mut ok: bool = 1 as i32 != 0;
    memset(
        &mut flstats as *mut file_stats_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<file_stats_t>() as u64,
    );
    env_sysrc = getenv(b"SYSTEM_WGETRC\0" as *const u8 as *const i8);
    if !env_sysrc.is_null() && file_exists_p(env_sysrc, &mut flstats) as i32 != 0 {
        ok = (ok as i32 & run_wgetrc(env_sysrc, &mut flstats) as i32) as bool;
        if !ok {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"Parsing system wgetrc file (env SYSTEM_WGETRC) failed.  Please check\n'%s',\nor specify a different file using --config.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                env_sysrc,
            );
            return C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32;
        }
    } else if file_exists_p(
        b"/usr/local/etc/wgetrc\0" as *const u8 as *const i8,
        &mut flstats,
    ) {
        ok = (ok as i32
            & run_wgetrc(
                b"/usr/local/etc/wgetrc\0" as *const u8 as *const i8,
                &mut flstats,
            ) as i32) as bool;
    }
    if !ok {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Parsing system wgetrc file failed.  Please check\n'%s',\nor specify a different file using --config.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            b"/usr/local/etc/wgetrc\0" as *const u8 as *const i8,
        );
        return C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32;
    }
    opt.wgetrcfile = wgetrc_file_name();
    if (opt.wgetrcfile).is_null() {
        return 0 as i32;
    }
    if strcmp(opt.wgetrcfile, b"/usr/local/etc/wgetrc\0" as *const u8 as *const i8) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: Warning: Both system and user wgetrc point to %s.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            exec_name,
            quote(opt.wgetrcfile),
        );
    } else if file_exists_p(opt.wgetrcfile, &mut flstats) {
        ok = (ok as i32 & run_wgetrc(opt.wgetrcfile, &mut flstats) as i32) as bool;
    }
    rpl_free(opt.wgetrcfile as *mut libc::c_void);
    opt.wgetrcfile = 0 as *const i8;
    if !ok {
        return C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn dehyphen(mut s: *mut i8) {
    let mut t: *mut i8 = s;
    let mut h: *mut i8 = s;
    while *h != 0 {
        if *h as i32 == '_' as i32 || *h as i32 == '-' as i32 {
            h = h.offset(1);
            h;
        } else {
            let fresh0 = h;
            h = h.offset(1);
            let fresh1 = t;
            t = t.offset(1);
            *fresh1 = *fresh0;
        }
    }
    *t = '\0' as i32 as i8;
}
unsafe extern "C" fn parse_line(
    mut line: *const i8,
    mut com: *mut *mut i8,
    mut val: *mut *mut i8,
    mut comind: *mut i32,
) -> parse_line {
    let mut p: *const i8 = 0 as *const i8;
    let mut end: *const i8 = line.offset(strlen(line) as isize);
    let mut cmdstart: *const i8 = 0 as *const i8;
    let mut cmdend: *const i8 = 0 as *const i8;
    let mut valstart: *const i8 = 0 as *const i8;
    let mut valend: *const i8 = 0 as *const i8;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut len: size_t = 0;
    let mut cmdcopy: *mut i8 = 0 as *mut i8;
    let mut ind: i32 = 0;
    while *line as i32 != 0 && c_isspace(*line as i32) as i32 != 0 {
        line = line.offset(1);
        line;
    }
    while end > line && c_isspace(*end.offset(-(1 as i32) as isize) as i32) as i32 != 0 {
        end = end.offset(-1);
        end;
    }
    if *line == 0 || *line as i32 == '#' as i32 {
        return parse_line::line_empty;
    }
    p = line;
    cmdstart = p;
    while p < end
        && (c_isalnum(*p as i32) as i32 != 0 || *p as i32 == '_' as i32
            || *p as i32 == '-' as i32)
    {
        p = p.offset(1);
        p;
    }
    cmdend = p;
    while p < end && c_isspace(*p as i32) as i32 != 0 {
        p = p.offset(1);
        p;
    }
    if p == end || *p as i32 != '=' as i32 {
        return parse_line::line_syntax_error;
    }
    p = p.offset(1);
    p;
    while p < end && c_isspace(*p as i32) as i32 != 0 {
        p = p.offset(1);
        p;
    }
    valstart = p;
    valend = end;
    *com = strdupdelim(cmdstart, cmdend);
    *val = strdupdelim(valstart, valend);
    len = cmdend.offset_from(cmdstart) as i64 as size_t;
    if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
        cmdcopy = buf.as_mut_ptr();
    } else {
        cmdcopy = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
    }
    memcpy(cmdcopy as *mut libc::c_void, cmdstart as *const libc::c_void, len);
    *cmdcopy.offset(len as isize) = 0 as i32 as i8;
    dehyphen(cmdcopy);
    ind = command_by_name(cmdcopy);
    if cmdcopy != buf.as_mut_ptr() {
        rpl_free(cmdcopy as *mut libc::c_void);
        cmdcopy = 0 as *mut i8;
    }
    if ind == -(1 as i32) {
        return parse_line::line_unknown_command;
    }
    *comind = ind;
    return parse_line::line_ok;
}
unsafe extern "C" fn setval_internal(
    mut comind: i32,
    mut com: *const i8,
    mut val: *const i8,
) -> bool {
    if comind as u32 as u64
        >= (::core::mem::size_of::<[C2RustUnnamed_5; 168]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as u64)
    {
        return !(0 as *mut libc::c_void).is_null();
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Setting %s (%s) to %s\n\0" as *const u8 as *const i8,
            com,
            commands[comind as usize].name,
            val,
        );
    }
    return (commands[comind as usize].action)
        .expect("non-null function pointer")(com, val, commands[comind as usize].place);
}
unsafe extern "C" fn setval_internal_tilde(
    mut comind: i32,
    mut com: *const i8,
    mut val: *const i8,
) -> bool {
    let mut ret: bool = false;
    let mut homelen: i32 = 0;
    let mut pstring: *mut *mut i8 = 0 as *mut *mut i8;
    ret = setval_internal(comind, com, val);
    if (commands[comind as usize].action
        == Some(
            cmd_file
                as unsafe extern "C" fn(*const i8, *const i8, *mut libc::c_void) -> bool,
        )
        || commands[comind as usize].action
            == Some(
                cmd_directory
                    as unsafe extern "C" fn(
                        *const i8,
                        *const i8,
                        *mut libc::c_void,
                    ) -> bool,
            )) && ret as i32 != 0
        && (*val as i32 == '~' as i32
            && *val.offset(1 as i32 as isize) as i32 == '/' as i32)
    {
        pstring = commands[comind as usize].place as *mut *mut i8;
        if !(opt.homedir).is_null() {
            let mut home: *mut i8 = xstrdup(opt.homedir);
            homelen = strlen(home) as i32;
            while homelen != 0
                && *home.offset((homelen - 1 as i32) as isize) as i32 == '/' as i32
            {
                homelen -= 1;
                *home.offset(homelen as isize) = '\0' as i32 as i8;
            }
            rpl_free(*pstring as *mut libc::c_void);
            *pstring = 0 as *mut i8;
            val = val
                .offset(
                    (strspn(
                        val.offset(1 as i32 as isize),
                        b"/\0" as *const u8 as *const i8,
                    ))
                        .wrapping_add(1 as i32 as u64) as isize,
                );
            *pstring = concat_strings(
                home,
                b"/\0" as *const u8 as *const i8,
                val,
                0 as *mut i8,
            );
            rpl_free(home as *mut libc::c_void);
            home = 0 as *mut i8;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn setoptval(
    mut com: *const i8,
    mut val: *const i8,
    mut optname: *const i8,
) {
    let mut dd_optname: [i8; 29] = [0; 29];
    if snprintf(
        dd_optname.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 29]>() as u64,
        b"--%s\0" as *const u8 as *const i8,
        optname,
    ) as u32 as u64 > ::core::mem::size_of::<[i8; 29]>() as u64
    {
        exit(C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32);
    }
    if !setval_internal(command_by_name(com), dd_optname.as_mut_ptr(), val) {
        exit(C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn run_command(mut cmdopt: *const i8) {
    let mut com: *mut i8 = 0 as *mut i8;
    let mut val: *mut i8 = 0 as *mut i8;
    let mut comind: i32 = 0;
    match parse_line(cmdopt, &mut com, &mut val, &mut comind) as u32 {
        0 => {
            if !setval_internal(comind, com, val) {
                exit(C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32);
            }
            rpl_free(com as *mut libc::c_void);
            com = 0 as *mut i8;
            rpl_free(val as *mut libc::c_void);
            val = 0 as *mut i8;
        }
        _ => {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s: Invalid --execute command %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                exec_name,
                quote(cmdopt),
            );
            exit(C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR as i32);
        }
    };
}
unsafe extern "C" fn cmd_boolean_internal(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> i32 {
    if c_tolower(*val.offset(0 as i32 as isize) as i32) == 'o' as i32
        && c_tolower(*val.offset(1 as i32 as isize) as i32) == 'n' as i32
        && *val.offset(2 as i32 as isize) as i32 == '\0' as i32
        || c_tolower(*val.offset(0 as i32 as isize) as i32) == 'y' as i32
            && c_tolower(*val.offset(1 as i32 as isize) as i32) == 'e' as i32
            && c_tolower(*val.offset(2 as i32 as isize) as i32) == 's' as i32
            && *val.offset(3 as i32 as isize) as i32 == '\0' as i32
        || c_tolower(*val.offset(0 as i32 as isize) as i32) == '1' as i32
            && *val.offset(1 as i32 as isize) as i32 == '\0' as i32
    {
        return 1 as i32
    } else if c_tolower(*val.offset(0 as i32 as isize) as i32) == 'o' as i32
        && c_tolower(*val.offset(1 as i32 as isize) as i32) == 'f' as i32
        && c_tolower(*val.offset(2 as i32 as isize) as i32) == 'f' as i32
        && *val.offset(3 as i32 as isize) as i32 == '\0' as i32
        || c_tolower(*val.offset(0 as i32 as isize) as i32) == 'n' as i32
            && c_tolower(*val.offset(1 as i32 as isize) as i32) == 'o' as i32
            && *val.offset(2 as i32 as isize) as i32 == '\0' as i32
        || c_tolower(*val.offset(0 as i32 as isize) as i32) == '0' as i32
            && *val.offset(1 as i32 as isize) as i32 == '\0' as i32
    {
        return 0 as i32
    }
    return -(1 as i32);
}
unsafe extern "C" fn cmd_boolean(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut value: bool = false;
    match cmd_boolean_internal(com, val, place) {
        0 => {
            value = 0 as i32 != 0;
        }
        1 => {
            value = 1 as i32 != 0;
        }
        _ => {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s: %s: Invalid boolean %s; use `on' or `off'.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                exec_name,
                com,
                quote(val),
            );
            return 0 as i32 != 0;
        }
    }
    *(place as *mut bool) = value;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_check_cert(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut value: i32 = 0;
    match cmd_boolean_internal(com, val, place) {
        0 => {
            value = CHECK_CERT_MODES::CHECK_CERT_OFF as i32;
        }
        1 => {
            value = CHECK_CERT_MODES::CHECK_CERT_ON as i32;
        }
        _ => {
            if c_strcasecmp(val, b"quiet\0" as *const u8 as *const i8) == 0 {
                value = CHECK_CERT_MODES::CHECK_CERT_QUIET as i32;
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: %s: Invalid %s; use `on', `off' or `quiet'.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    exec_name,
                    com,
                    quote(val),
                );
                return 0 as i32 != 0;
            }
        }
    }
    *(place as *mut i32) = value;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_number(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut l: i64 = rpl_strtol(val, 0 as *mut *mut i8, 10 as i32);
    if (l == -(9223372036854775807 as i64) - 1 as i64 || l == 9223372036854775807 as i64)
        && *__errno_location() == 34 as i32 || l < 0 as i32 as i64
        || l > 2147483647 as i32 as i64
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid number %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as i32 != 0;
    }
    *(place as *mut i32) = l as i32;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_number_inf(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    if c_strcasecmp(val, b"inf\0" as *const u8 as *const i8) == 0 {
        *(place as *mut i32) = 0 as i32;
        return 1 as i32 != 0;
    }
    return cmd_number(com, val, place);
}
unsafe extern "C" fn cmd_string(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pstring: *mut *mut i8 = place as *mut *mut i8;
    rpl_free(*pstring as *mut libc::c_void);
    *pstring = 0 as *mut i8;
    *pstring = xstrdup(val);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_string_uppercase(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut q: *mut i8 = 0 as *mut i8;
    let mut pstring: *mut *mut i8 = 0 as *mut *mut i8;
    pstring = place as *mut *mut i8;
    rpl_free(*pstring as *mut libc::c_void);
    *pstring = 0 as *mut i8;
    *pstring = xmalloc((strlen(val)).wrapping_add(1 as i32 as u64)) as *mut i8;
    q = *pstring;
    while *val != 0 {
        *q = c_toupper(*val as i32) as i8;
        val = val.offset(1);
        val;
        q = q.offset(1);
        q;
    }
    *q = '\0' as i32 as i8;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_file(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pstring: *mut *mut i8 = place as *mut *mut i8;
    rpl_free(*pstring as *mut libc::c_void);
    *pstring = 0 as *mut i8;
    *pstring = xstrdup(val);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_file_once(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    if !(*(place as *mut *mut i8)).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s must only be used once\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
        );
        return 0 as i32 != 0;
    }
    return cmd_file(com, val, place);
}
unsafe extern "C" fn cmd_directory(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut t: *mut i8 = 0 as *mut i8;
    if !cmd_file(com, val, place) {
        return 0 as i32 != 0;
    }
    s = *(place as *mut *mut i8);
    t = s.offset(strlen(s) as isize);
    while t > s
        && {
            t = t.offset(-1);
            *t as i32 == '/' as i32
        }
    {
        *t = '\0' as i32 as i8;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_vector(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pvec: *mut *mut *mut i8 = place as *mut *mut *mut i8;
    if *val != 0 {
        *pvec = merge_vecs(*pvec, sepstring(val));
    } else {
        free_vec(*pvec);
        *pvec = 0 as *mut *mut i8;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_directory_vector(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut pvec: *mut *mut *mut i8 = place as *mut *mut *mut i8;
    if *val != 0 {
        let mut t: *mut *mut i8 = 0 as *mut *mut i8;
        let mut seps: *mut *mut i8 = 0 as *mut *mut i8;
        seps = sepstring(val);
        t = seps;
        while !t.is_null() && !(*t).is_null() {
            let mut len: i32 = strlen(*t) as i32;
            if len > 1 as i32 {
                if *(*t).offset((len - 1 as i32) as isize) as i32 == '/' as i32 {
                    *(*t).offset((len - 1 as i32) as isize) = '\0' as i32 as i8;
                }
            }
            t = t.offset(1);
            t;
        }
        *pvec = merge_vecs(*pvec, seps);
    } else {
        free_vec(*pvec);
        *pvec = 0 as *mut *mut i8;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn parse_bytes_helper(
    mut val: *const i8,
    mut result: *mut libc::c_double,
) -> bool {
    let mut number: libc::c_double = 0.;
    let mut mult: libc::c_double = 0.;
    let mut end: *const i8 = val.offset(strlen(val) as isize);
    if 0 as i32 == strcmp(val, b"inf\0" as *const u8 as *const i8) {
        *result = 0 as i32 as libc::c_double;
        return 1 as i32 != 0;
    }
    while val < end && c_isspace(*end.offset(-(1 as i32) as isize) as i32) as i32 != 0 {
        end = end.offset(-1);
        end;
    }
    if val == end {
        return 0 as i32 != 0;
    }
    match c_tolower(*end.offset(-(1 as i32) as isize) as i32) {
        107 => {
            end = end.offset(-1);
            end;
            mult = 1024.0f64;
        }
        109 => {
            end = end.offset(-1);
            end;
            mult = 1048576.0f64;
        }
        103 => {
            end = end.offset(-1);
            end;
            mult = 1073741824.0f64;
        }
        116 => {
            end = end.offset(-1);
            end;
            mult = 1099511627776.0f64;
        }
        _ => {
            mult = 1 as i32 as libc::c_double;
        }
    }
    while val < end && c_isspace(*val as i32) as i32 != 0 {
        val = val.offset(1);
        val;
    }
    while val < end && c_isspace(*end.offset(-(1 as i32) as isize) as i32) as i32 != 0 {
        end = end.offset(-1);
        end;
    }
    if val == end {
        return 0 as i32 != 0;
    }
    if !simple_atof(val, end, &mut number) || number < 0 as i32 as libc::c_double {
        return 0 as i32 != 0;
    }
    *result = number * mult;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_bytes(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut byte_value: libc::c_double = 0.;
    if !parse_bytes_helper(val, &mut byte_value) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid byte value %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as i32 != 0;
    }
    *(place as *mut wgint) = byte_value as wgint;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_bytes_sum(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut byte_value: libc::c_double = 0.;
    if !parse_bytes_helper(val, &mut byte_value)
        || byte_value
            < (-(9223372036854775807 as i64) - 1 as i32 as i64) as libc::c_double
        || byte_value > 9223372036854775807 as i64 as libc::c_double
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid byte value %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as i32 != 0;
    }
    *(place as *mut wgint) = byte_value as wgint;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_time(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut number: libc::c_double = 0.;
    let mut mult: libc::c_double = 0.;
    let mut end: *const i8 = val.offset(strlen(val) as isize);
    while val < end && c_isspace(*end.offset(-(1 as i32) as isize) as i32) as i32 != 0 {
        end = end.offset(-1);
        end;
    }
    if !(val == end) {
        match c_tolower(*end.offset(-(1 as i32) as isize) as i32) {
            115 => {
                end = end.offset(-1);
                end;
                mult = 1 as i32 as libc::c_double;
            }
            109 => {
                end = end.offset(-1);
                end;
                mult = 60 as i32 as libc::c_double;
            }
            104 => {
                end = end.offset(-1);
                end;
                mult = 3600 as i32 as libc::c_double;
            }
            100 => {
                end = end.offset(-1);
                end;
                mult = 86400.0f64;
            }
            119 => {
                end = end.offset(-1);
                end;
                mult = 604800.0f64;
            }
            _ => {
                mult = 1 as i32 as libc::c_double;
            }
        }
        while val < end && c_isspace(*val as i32) as i32 != 0 {
            val = val.offset(1);
            val;
        }
        while val < end
            && c_isspace(*end.offset(-(1 as i32) as isize) as i32) as i32 != 0
        {
            end = end.offset(-1);
            end;
        }
        if !(val == end) {
            if simple_atof(val, end, &mut number) {
                if number < 0 as i32 as libc::c_double {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: %s: Negative time period %s\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        exec_name,
                        com,
                        quote(val),
                    );
                    return 0 as i32 != 0;
                }
                *(place as *mut libc::c_double) = number * mult;
                return 1 as i32 != 0;
            }
        }
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const i8,
            b"%s: %s: Invalid time period %s\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        exec_name,
        com,
        quote(val),
    );
    return 0 as i32 != 0;
}
unsafe extern "C" fn cmd_use_askpass(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut env_name: *const i8 = b"WGET_ASKPASS\0" as *const u8 as *const i8;
    let mut env: *const i8 = 0 as *const i8;
    if !val.is_null() && *val as i32 != 0 {
        return cmd_string(com, val, place);
    }
    env = getenv(env_name);
    if !(!env.is_null() && *env as i32 != 0) {
        env_name = b"SSH_ASKPASS\0" as *const u8 as *const i8;
        env = getenv(env_name);
    }
    if !(!env.is_null() && *env as i32 != 0) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"use-askpass requires a string or either environment variable WGET_ASKPASS or SSH_ASKPASS to be set.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32 != 0;
    }
    return cmd_string(com, env, place);
}
unsafe extern "C" fn cmd_cert_type(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 3] = [
        {
            let mut init = decode_item {
                name: b"pem\0" as *const u8 as *const i8,
                code: keyfile_type::keyfile_pem as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"der\0" as *const u8 as *const i8,
                code: keyfile_type::keyfile_asn1 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"asn1\0" as *const u8 as *const i8,
                code: keyfile_type::keyfile_asn1 as i32,
            };
            init
        },
    ];
    let mut ok: i32 = decode_string(
        val,
        choices.as_ptr(),
        (::core::mem::size_of::<[decode_item; 3]>() as u64)
            .wrapping_div(::core::mem::size_of::<decode_item>() as u64) as i32,
        place as *mut i32,
    ) as i32;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_compression(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 3] = [
        {
            let mut init = decode_item {
                name: b"auto\0" as *const u8 as *const i8,
                code: compression_options::compression_auto as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"gzip\0" as *const u8 as *const i8,
                code: compression_options::compression_gzip as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"none\0" as *const u8 as *const i8,
                code: compression_options::compression_none as i32,
            };
            init
        },
    ];
    let mut ok: i32 = decode_string(
        val,
        choices.as_ptr(),
        (::core::mem::size_of::<[decode_item; 3]>() as u64)
            .wrapping_div(::core::mem::size_of::<decode_item>() as u64) as i32,
        place as *mut i32,
    ) as i32;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_dirstruct(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !cmd_boolean(com, val, &mut opt.dirstruct as *mut bool as *mut libc::c_void) {
        return 0 as i32 != 0;
    }
    if opt.dirstruct {
        opt.no_dirstruct = 0 as i32 != 0;
    } else {
        opt.no_dirstruct = 1 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_header(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if *val as i32 == '\0' as i32 {
        free_vec(opt.user_headers);
        opt.user_headers = 0 as *mut *mut i8;
        return 1 as i32 != 0;
    }
    if !check_user_specified_header(val) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid header %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as i32 != 0;
    }
    opt.user_headers = vec_append(opt.user_headers, val);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_warc_header(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if *val as i32 == '\0' as i32 {
        free_vec(opt.warc_user_headers);
        opt.warc_user_headers = 0 as *mut *mut i8;
        return 1 as i32 != 0;
    }
    if !check_user_specified_header(val) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid WARC header %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as i32 != 0;
    }
    opt.warc_user_headers = vec_append(opt.warc_user_headers, val);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_htmlify(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut flag: i32 = cmd_boolean(
        com,
        val,
        &mut opt.htmlify as *mut bool as *mut libc::c_void,
    ) as i32;
    if flag != 0 && !opt.htmlify {
        opt.remove_listing = 0 as i32 != 0;
    }
    return flag != 0;
}
unsafe extern "C" fn cmd_spec_mirror(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut mirror: bool = false;
    if !cmd_boolean(com, val, &mut mirror as *mut bool as *mut libc::c_void) {
        return 0 as i32 != 0;
    }
    if mirror {
        opt.recursive = 1 as i32 != 0;
        if !opt.no_dirstruct {
            opt.dirstruct = 1 as i32 != 0;
        }
        opt.timestamping = 1 as i32 != 0;
        opt.reclevel = -(1 as i32);
        opt.remove_listing = 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_prefer_family(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 3] = [
        {
            let mut init = decode_item {
                name: b"IPv4\0" as *const u8 as *const i8,
                code: C2RustUnnamed::prefer_ipv4 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"IPv6\0" as *const u8 as *const i8,
                code: C2RustUnnamed::prefer_ipv6 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"none\0" as *const u8 as *const i8,
                code: C2RustUnnamed::prefer_none as i32,
            };
            init
        },
    ];
    let mut prefer_family: i32 = C2RustUnnamed::prefer_none as i32;
    let mut ok: i32 = decode_string(
        val,
        choices.as_ptr(),
        (::core::mem::size_of::<[decode_item; 3]>() as u64)
            .wrapping_div(::core::mem::size_of::<decode_item>() as u64) as i32,
        &mut prefer_family,
    ) as i32;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    opt.prefer_family = C2RustUnnamed::from_libc_c_uint(prefer_family as u32);
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_progress(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !valid_progress_implementation_p(val) {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid progress type %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as i32 != 0;
    }
    rpl_free(opt.progress_type as *mut libc::c_void);
    opt.progress_type = 0 as *mut i8;
    opt.progress_type = xstrdup(val);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_recursive(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !cmd_boolean(com, val, &mut opt.recursive as *mut bool as *mut libc::c_void) {
        return 0 as i32 != 0
    } else if opt.recursive as i32 != 0 && !opt.no_dirstruct {
        opt.dirstruct = 1 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_regex_type(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 2] = [
        {
            let mut init = decode_item {
                name: b"posix\0" as *const u8 as *const i8,
                code: C2RustUnnamed_3::regex_type_posix as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"pcre\0" as *const u8 as *const i8,
                code: C2RustUnnamed_3::regex_type_pcre as i32,
            };
            init
        },
    ];
    let mut regex_type: i32 = C2RustUnnamed_3::regex_type_posix as i32;
    let mut ok: i32 = decode_string(
        val,
        choices.as_ptr(),
        (::core::mem::size_of::<[decode_item; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<decode_item>() as u64) as i32,
        &mut regex_type,
    ) as i32;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    opt.regex_type = C2RustUnnamed_3::from_libc_c_uint(regex_type as u32);
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_restrict_file_names(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut restrict_os: i32 = opt.restrict_files_os as i32;
    let mut restrict_ctrl: i32 = opt.restrict_files_ctrl as i32;
    let mut restrict_case: i32 = opt.restrict_files_case as i32;
    let mut restrict_nonascii: i32 = opt.restrict_files_nonascii as i32;
    let mut end: *const i8 = 0 as *const i8;
    loop {
        end = strchr(val, ',' as i32);
        if end.is_null() {
            end = val.offset(strlen(val) as isize);
        }
        if end.offset_from(val) as i64 as u64
            == (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                val as *const libc::c_void,
                b"unix\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0
        {
            restrict_os = C2RustUnnamed_1::restrict_unix as i32;
        } else if end.offset_from(val) as i64 as u64
            == (::core::mem::size_of::<[i8; 4]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                val as *const libc::c_void,
                b"vms\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 4]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0
        {
            restrict_os = C2RustUnnamed_1::restrict_vms as i32;
        } else if end.offset_from(val) as i64 as u64
            == (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                val as *const libc::c_void,
                b"windows\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0
        {
            restrict_os = C2RustUnnamed_1::restrict_windows as i32;
        } else if end.offset_from(val) as i64 as u64
            == (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                val as *const libc::c_void,
                b"lowercase\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0
        {
            restrict_case = C2RustUnnamed_0::restrict_lowercase as i32;
        } else if end.offset_from(val) as i64 as u64
            == (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                val as *const libc::c_void,
                b"uppercase\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0
        {
            restrict_case = C2RustUnnamed_0::restrict_uppercase as i32;
        } else if end.offset_from(val) as i64 as u64
            == (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                val as *const libc::c_void,
                b"nocontrol\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 10]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0
        {
            restrict_ctrl = 0 as i32;
        } else if end.offset_from(val) as i64 as u64
            == (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
            && memcmp(
                val as *const libc::c_void,
                b"ascii\0" as *const u8 as *const i8 as *const libc::c_void,
                (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0
        {
            restrict_nonascii = 1 as i32;
        } else {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s: %s: Invalid restriction %s,\n    use [unix|vms|windows],[lowercase|uppercase],[nocontrol],[ascii].\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                exec_name,
                com,
                quote(val),
            );
            return 0 as i32 != 0;
        }
        if *end != 0 {
            val = end.offset(1 as i32 as isize);
        }
        if !(*val as i32 != 0 && *end as i32 != 0) {
            break;
        }
    }
    opt.restrict_files_os = C2RustUnnamed_1::from_libc_c_uint(restrict_os as u32);
    opt.restrict_files_ctrl = restrict_ctrl != 0;
    opt.restrict_files_case = C2RustUnnamed_0::from_libc_c_uint(restrict_case as u32);
    opt.restrict_files_nonascii = restrict_nonascii != 0;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_report_speed(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    opt.report_bps = c_strcasecmp(val, b"bits\0" as *const u8 as *const i8) == 0 as i32;
    if !opt.report_bps {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    return opt.report_bps;
}
unsafe extern "C" fn cmd_spec_secure_protocol(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    static mut choices: [decode_item; 8] = [
        {
            let mut init = decode_item {
                name: b"auto\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_auto as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"sslv2\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_sslv2 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"sslv3\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_sslv3 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"tlsv1\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_tlsv1 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"tlsv1_1\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_tlsv1_1 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"tlsv1_2\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_tlsv1_2 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"tlsv1_3\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_tlsv1_3 as i32,
            };
            init
        },
        {
            let mut init = decode_item {
                name: b"pfs\0" as *const u8 as *const i8,
                code: C2RustUnnamed_2::secure_protocol_pfs as i32,
            };
            init
        },
    ];
    snprintf(
        (opt.secure_protocol_name).as_mut_ptr(),
        ::core::mem::size_of::<[i8; 8]>() as u64,
        b"%s\0" as *const u8 as *const i8,
        val,
    );
    let mut ok: i32 = decode_string(
        val,
        choices.as_ptr(),
        (::core::mem::size_of::<[decode_item; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<decode_item>() as u64) as i32,
        place as *mut i32,
    ) as i32;
    if ok == 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
    }
    return ok != 0;
}
unsafe extern "C" fn cmd_spec_timeout(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut value: libc::c_double = 0.;
    if !cmd_time(com, val, &mut value as *mut libc::c_double as *mut libc::c_void) {
        return 0 as i32 != 0;
    }
    opt.read_timeout = value;
    opt.connect_timeout = value;
    opt.dns_timeout = value;
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_useragent(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    if !(strchr(val, '\n' as i32)).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: %s: Invalid value %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            com,
            quote(val),
        );
        return 0 as i32 != 0;
    }
    rpl_free(opt.useragent as *mut libc::c_void);
    opt.useragent = 0 as *mut i8;
    opt.useragent = xstrdup(val);
    return 1 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_progressdisp(
    mut com: *const i8,
    mut val: *const i8,
    mut place: *mut libc::c_void,
) -> bool {
    let mut flag: bool = false;
    if cmd_boolean(com, val, &mut flag as *mut bool as *mut libc::c_void) {
        opt.show_progress = flag as i32;
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn cmd_spec_verbose(
    mut com: *const i8,
    mut val: *const i8,
    mut place_ignored: *mut libc::c_void,
) -> bool {
    let mut flag: bool = false;
    if cmd_boolean(com, val, &mut flag as *mut bool as *mut libc::c_void) {
        opt.verbose = flag as i32;
        opt.show_progress = -(1 as i32);
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn simple_atof(
    mut beg: *const i8,
    mut end: *const i8,
    mut dest: *mut libc::c_double,
) -> bool {
    let mut result: libc::c_double = 0 as i32 as libc::c_double;
    let mut negative: bool = 0 as i32 != 0;
    let mut seen_dot: bool = 0 as i32 != 0;
    let mut seen_digit: bool = 0 as i32 != 0;
    let mut divider: libc::c_double = 1 as i32 as libc::c_double;
    let mut p: *const i8 = beg;
    while p < end && c_isspace(*p as i32) as i32 != 0 {
        p = p.offset(1);
        p;
    }
    if p < end && (*p as i32 == '-' as i32 || *p as i32 == '+' as i32) {
        negative = *p as i32 == '-' as i32;
        p = p.offset(1);
        p;
    }
    while p < end {
        let mut ch: i8 = *p;
        if c_isdigit(ch as i32) {
            if !seen_dot {
                result = 10 as i32 as libc::c_double * result
                    + (ch as i32 - '0' as i32) as libc::c_double;
            } else {
                divider *= 10 as i32 as libc::c_double;
                result += (ch as i32 - '0' as i32) as libc::c_double / divider;
            }
            seen_digit = 1 as i32 != 0;
        } else if ch as i32 == '.' as i32 {
            if !seen_dot {
                seen_dot = 1 as i32 != 0;
            } else {
                return 0 as i32 != 0
            }
        } else {
            return 0 as i32 != 0
        }
        p = p.offset(1);
        p;
    }
    if !seen_digit {
        return 0 as i32 != 0;
    }
    if negative {
        result = -result;
    }
    *dest = result;
    return 1 as i32 != 0;
}
unsafe extern "C" fn check_user_specified_header(mut s: *const i8) -> bool {
    let mut p: *const i8 = 0 as *const i8;
    p = s;
    while *p as i32 != 0 && *p as i32 != ':' as i32 && !c_isspace(*p as i32) {
        p = p.offset(1);
        p;
    }
    if *p as i32 != ':' as i32 || p == s {
        return 0 as i32 != 0;
    }
    if !(strchr(s, '\n' as i32)).is_null() {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn decode_string(
    mut val: *const i8,
    mut items: *const decode_item,
    mut itemcount: i32,
    mut place: *mut i32,
) -> bool {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < itemcount {
        if 0 as i32 == c_strcasecmp(val, (*items.offset(i as isize)).name) {
            *place = (*items.offset(i as isize)).code;
            return 1 as i32 != 0;
        }
        i += 1;
        i;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cleanup() {
    let fresh2 = cleaned_up;
    cleaned_up = cleaned_up + 1;
    if fresh2 != 0 {
        return;
    }
    if !(opt.warc_filename).is_null() {
        warc_close();
    }
    log_close();
    if !output_stream.is_null() && output_stream != stderr {
        let mut fp: *mut FILE = output_stream;
        output_stream = 0 as *mut FILE;
        if fclose(fp) == -(1 as i32) {
            inform_exit_status(uerr_t::CLOSEFAILED);
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    commands = [
        {
            let mut init = C2RustUnnamed_5 {
                name: b"accept\0" as *const u8 as *const i8,
                place: &mut opt.accepts as *mut *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"acceptregex\0" as *const u8 as *const i8,
                place: &mut opt.acceptregex_s as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"addhostdir\0" as *const u8 as *const i8,
                place: &mut opt.add_hostdir as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"adjustextension\0" as *const u8 as *const i8,
                place: &mut opt.adjust_extension as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"alwaysrest\0" as *const u8 as *const i8,
                place: &mut opt.always_rest as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"askpassword\0" as *const u8 as *const i8,
                place: &mut opt.ask_passwd as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"authnochallenge\0" as *const u8 as *const i8,
                place: &mut opt.auth_without_challenge as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"background\0" as *const u8 as *const i8,
                place: &mut opt.background as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"backupconverted\0" as *const u8 as *const i8,
                place: &mut opt.backup_converted as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"backups\0" as *const u8 as *const i8,
                place: &mut opt.backups as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"base\0" as *const u8 as *const i8,
                place: &mut opt.base_href as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"bindaddress\0" as *const u8 as *const i8,
                place: &mut opt.bind_address as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"bodydata\0" as *const u8 as *const i8,
                place: &mut opt.body_data as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"bodyfile\0" as *const u8 as *const i8,
                place: &mut opt.body_file as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"cacertificate\0" as *const u8 as *const i8,
                place: &mut opt.ca_cert as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"cache\0" as *const u8 as *const i8,
                place: &mut opt.allow_cache as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"cadirectory\0" as *const u8 as *const i8,
                place: &mut opt.ca_directory as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_directory
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"certificate\0" as *const u8 as *const i8,
                place: &mut opt.cert_file as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"certificatetype\0" as *const u8 as *const i8,
                place: &mut opt.cert_type as *mut keyfile_type as *mut libc::c_void,
                action: Some(
                    cmd_cert_type
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"checkcertificate\0" as *const u8 as *const i8,
                place: &mut opt.check_cert as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_check_cert
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"chooseconfig\0" as *const u8 as *const i8,
                place: &mut opt.choose_config as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ciphers\0" as *const u8 as *const i8,
                place: &mut opt.tls_ciphers_string as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"compression\0" as *const u8 as *const i8,
                place: &mut opt.compression as *mut compression_options
                    as *mut libc::c_void,
                action: Some(
                    cmd_spec_compression
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"connecttimeout\0" as *const u8 as *const i8,
                place: &mut opt.connect_timeout as *mut libc::c_double
                    as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"contentdisposition\0" as *const u8 as *const i8,
                place: &mut opt.content_disposition as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"contentonerror\0" as *const u8 as *const i8,
                place: &mut opt.content_on_error as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"continue\0" as *const u8 as *const i8,
                place: &mut opt.always_rest as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"convertfileonly\0" as *const u8 as *const i8,
                place: &mut opt.convert_file_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"convertlinks\0" as *const u8 as *const i8,
                place: &mut opt.convert_links as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"cookies\0" as *const u8 as *const i8,
                place: &mut opt.cookies as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"crlfile\0" as *const u8 as *const i8,
                place: &mut opt.crl_file as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file_once
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"cutdirs\0" as *const u8 as *const i8,
                place: &mut opt.cut_dirs as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"debug\0" as *const u8 as *const i8,
                place: &mut opt.debug as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"defaultpage\0" as *const u8 as *const i8,
                place: &mut opt.default_page as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"deleteafter\0" as *const u8 as *const i8,
                place: &mut opt.delete_after as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dirprefix\0" as *const u8 as *const i8,
                place: &mut opt.dir_prefix as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_directory
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dirstruct\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_dirstruct
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dnscache\0" as *const u8 as *const i8,
                place: &mut opt.dns_cache as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dnstimeout\0" as *const u8 as *const i8,
                place: &mut opt.dns_timeout as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"domains\0" as *const u8 as *const i8,
                place: &mut opt.domains as *mut *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dotbytes\0" as *const u8 as *const i8,
                place: &mut opt.dot_bytes as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dotsinline\0" as *const u8 as *const i8,
                place: &mut opt.dots_in_line as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dotspacing\0" as *const u8 as *const i8,
                place: &mut opt.dot_spacing as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"dotstyle\0" as *const u8 as *const i8,
                place: &mut opt.dot_style as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"egdfile\0" as *const u8 as *const i8,
                place: &mut opt.egd_file as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"excludedirectories\0" as *const u8 as *const i8,
                place: &mut opt.excludes as *mut *mut *const i8 as *mut libc::c_void,
                action: Some(
                    cmd_directory_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"excludedomains\0" as *const u8 as *const i8,
                place: &mut opt.exclude_domains as *mut *mut *mut i8
                    as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"followftp\0" as *const u8 as *const i8,
                place: &mut opt.follow_ftp as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"followtags\0" as *const u8 as *const i8,
                place: &mut opt.follow_tags as *mut *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"forcehtml\0" as *const u8 as *const i8,
                place: &mut opt.force_html as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftppasswd\0" as *const u8 as *const i8,
                place: &mut opt.ftp_passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftppassword\0" as *const u8 as *const i8,
                place: &mut opt.ftp_passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftpproxy\0" as *const u8 as *const i8,
                place: &mut opt.ftp_proxy as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftpscleardataconnection\0" as *const u8 as *const i8,
                place: &mut opt.ftps_clear_data_connection as *mut bool
                    as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftpsfallbacktoftp\0" as *const u8 as *const i8,
                place: &mut opt.ftps_fallback_to_ftp as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftpsimplicit\0" as *const u8 as *const i8,
                place: &mut opt.ftps_implicit as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftpsresumessl\0" as *const u8 as *const i8,
                place: &mut opt.ftps_resume_ssl as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ftpuser\0" as *const u8 as *const i8,
                place: &mut opt.ftp_user as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"glob\0" as *const u8 as *const i8,
                place: &mut opt.ftp_glob as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"header\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_header
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"hsts\0" as *const u8 as *const i8,
                place: &mut opt.hsts as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"hstsfile\0" as *const u8 as *const i8,
                place: &mut opt.hsts_file as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"htmlextension\0" as *const u8 as *const i8,
                place: &mut opt.adjust_extension as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"htmlify\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_htmlify
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"httpkeepalive\0" as *const u8 as *const i8,
                place: &mut opt.http_keep_alive as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"httppasswd\0" as *const u8 as *const i8,
                place: &mut opt.http_passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"httppassword\0" as *const u8 as *const i8,
                place: &mut opt.http_passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"httpproxy\0" as *const u8 as *const i8,
                place: &mut opt.http_proxy as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"httpsonly\0" as *const u8 as *const i8,
                place: &mut opt.https_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"httpsproxy\0" as *const u8 as *const i8,
                place: &mut opt.https_proxy as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"httpuser\0" as *const u8 as *const i8,
                place: &mut opt.http_user as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ifmodifiedsince\0" as *const u8 as *const i8,
                place: &mut opt.if_modified_since as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ignorecase\0" as *const u8 as *const i8,
                place: &mut opt.ignore_case as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ignorelength\0" as *const u8 as *const i8,
                place: &mut opt.ignore_length as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"ignoretags\0" as *const u8 as *const i8,
                place: &mut opt.ignore_tags as *mut *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"includedirectories\0" as *const u8 as *const i8,
                place: &mut opt.includes as *mut *mut *const i8 as *mut libc::c_void,
                action: Some(
                    cmd_directory_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"inet4only\0" as *const u8 as *const i8,
                place: &mut opt.ipv4_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"inet6only\0" as *const u8 as *const i8,
                place: &mut opt.ipv6_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"input\0" as *const u8 as *const i8,
                place: &mut opt.input_filename as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"iri\0" as *const u8 as *const i8,
                place: &mut opt.enable_iri as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"keepbadhash\0" as *const u8 as *const i8,
                place: &mut opt.keep_badhash as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"keepsessioncookies\0" as *const u8 as *const i8,
                place: &mut opt.keep_session_cookies as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"limitrate\0" as *const u8 as *const i8,
                place: &mut opt.limit_rate as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"loadcookies\0" as *const u8 as *const i8,
                place: &mut opt.cookies_input as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"localencoding\0" as *const u8 as *const i8,
                place: &mut opt.locale as *mut *const i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"logfile\0" as *const u8 as *const i8,
                place: &mut opt.lfilename as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"login\0" as *const u8 as *const i8,
                place: &mut opt.ftp_user as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"maxredirect\0" as *const u8 as *const i8,
                place: &mut opt.max_redirect as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"method\0" as *const u8 as *const i8,
                place: &mut opt.method as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string_uppercase
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"mirror\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_mirror
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"netrc\0" as *const u8 as *const i8,
                place: &mut opt.netrc as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"noclobber\0" as *const u8 as *const i8,
                place: &mut opt.noclobber as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"noconfig\0" as *const u8 as *const i8,
                place: &mut opt.noconfig as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"noparent\0" as *const u8 as *const i8,
                place: &mut opt.no_parent as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"noproxy\0" as *const u8 as *const i8,
                place: &mut opt.no_proxy as *mut *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"numtries\0" as *const u8 as *const i8,
                place: &mut opt.ntry as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number_inf
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"outputdocument\0" as *const u8 as *const i8,
                place: &mut opt.output_document as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"pagerequisites\0" as *const u8 as *const i8,
                place: &mut opt.page_requisites as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"passiveftp\0" as *const u8 as *const i8,
                place: &mut opt.ftp_pasv as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"passwd\0" as *const u8 as *const i8,
                place: &mut opt.ftp_passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"password\0" as *const u8 as *const i8,
                place: &mut opt.passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"pinnedpubkey\0" as *const u8 as *const i8,
                place: &mut opt.pinnedpubkey as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"postdata\0" as *const u8 as *const i8,
                place: &mut opt.post_data as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"postfile\0" as *const u8 as *const i8,
                place: &mut opt.post_file_name as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"preferfamily\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_prefer_family
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"preservepermissions\0" as *const u8 as *const i8,
                place: &mut opt.preserve_perm as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"privatekey\0" as *const u8 as *const i8,
                place: &mut opt.private_key as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"privatekeytype\0" as *const u8 as *const i8,
                place: &mut opt.private_key_type as *mut keyfile_type
                    as *mut libc::c_void,
                action: Some(
                    cmd_cert_type
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"progress\0" as *const u8 as *const i8,
                place: &mut opt.progress_type as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_spec_progress
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"protocoldirectories\0" as *const u8 as *const i8,
                place: &mut opt.protocol_directories as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"proxypasswd\0" as *const u8 as *const i8,
                place: &mut opt.proxy_passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"proxypassword\0" as *const u8 as *const i8,
                place: &mut opt.proxy_passwd as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"proxyuser\0" as *const u8 as *const i8,
                place: &mut opt.proxy_user as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"quiet\0" as *const u8 as *const i8,
                place: &mut opt.quiet as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"quota\0" as *const u8 as *const i8,
                place: &mut opt.quota as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes_sum
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"randomfile\0" as *const u8 as *const i8,
                place: &mut opt.random_file as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"randomwait\0" as *const u8 as *const i8,
                place: &mut opt.random_wait as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"readtimeout\0" as *const u8 as *const i8,
                place: &mut opt.read_timeout as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"reclevel\0" as *const u8 as *const i8,
                place: &mut opt.reclevel as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number_inf
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"recursive\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_recursive
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"referer\0" as *const u8 as *const i8,
                place: &mut opt.referer as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"regextype\0" as *const u8 as *const i8,
                place: &mut opt.regex_type as *mut C2RustUnnamed_3 as *mut libc::c_void,
                action: Some(
                    cmd_spec_regex_type
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"reject\0" as *const u8 as *const i8,
                place: &mut opt.rejects as *mut *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_vector
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"rejectedlog\0" as *const u8 as *const i8,
                place: &mut opt.rejected_log as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"rejectregex\0" as *const u8 as *const i8,
                place: &mut opt.rejectregex_s as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"relativeonly\0" as *const u8 as *const i8,
                place: &mut opt.relative_only as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"remoteencoding\0" as *const u8 as *const i8,
                place: &mut opt.encoding_remote as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"removelisting\0" as *const u8 as *const i8,
                place: &mut opt.remove_listing as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"reportspeed\0" as *const u8 as *const i8,
                place: &mut opt.report_bps as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_spec_report_speed
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"restrictfilenames\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_restrict_file_names
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"retrsymlinks\0" as *const u8 as *const i8,
                place: &mut opt.retr_symlinks as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"retryconnrefused\0" as *const u8 as *const i8,
                place: &mut opt.retry_connrefused as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"retryonhosterror\0" as *const u8 as *const i8,
                place: &mut opt.retry_on_host_error as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"retryonhttperror\0" as *const u8 as *const i8,
                place: &mut opt.retry_on_http_error as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"robots\0" as *const u8 as *const i8,
                place: &mut opt.use_robots as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"savecookies\0" as *const u8 as *const i8,
                place: &mut opt.cookies_output as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"saveheaders\0" as *const u8 as *const i8,
                place: &mut opt.save_headers as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"secureprotocol\0" as *const u8 as *const i8,
                place: &mut opt.secure_protocol as *mut C2RustUnnamed_2
                    as *mut libc::c_void,
                action: Some(
                    cmd_spec_secure_protocol
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"serverresponse\0" as *const u8 as *const i8,
                place: &mut opt.server_response as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"showalldnsentries\0" as *const u8 as *const i8,
                place: &mut opt.show_all_dns_entries as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"showprogress\0" as *const u8 as *const i8,
                place: &mut opt.show_progress as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_spec_progressdisp
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"spanhosts\0" as *const u8 as *const i8,
                place: &mut opt.spanhost as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"spider\0" as *const u8 as *const i8,
                place: &mut opt.spider as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"startpos\0" as *const u8 as *const i8,
                place: &mut opt.start_pos as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"strictcomments\0" as *const u8 as *const i8,
                place: &mut opt.strict_comments as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"timeout\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_timeout
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"timestamping\0" as *const u8 as *const i8,
                place: &mut opt.timestamping as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"tries\0" as *const u8 as *const i8,
                place: &mut opt.ntry as *mut i32 as *mut libc::c_void,
                action: Some(
                    cmd_number_inf
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"trustservernames\0" as *const u8 as *const i8,
                place: &mut opt.trustservernames as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"unlink\0" as *const u8 as *const i8,
                place: &mut opt.unlink_requested as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"useaskpass\0" as *const u8 as *const i8,
                place: &mut opt.use_askpass as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_use_askpass
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"useproxy\0" as *const u8 as *const i8,
                place: &mut opt.use_proxy as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"user\0" as *const u8 as *const i8,
                place: &mut opt.user as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_string
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"useragent\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_useragent
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"useservertimestamps\0" as *const u8 as *const i8,
                place: &mut opt.useservertimestamps as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"verbose\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_verbose
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"wait\0" as *const u8 as *const i8,
                place: &mut opt.wait as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"waitretry\0" as *const u8 as *const i8,
                place: &mut opt.waitretry as *mut libc::c_double as *mut libc::c_void,
                action: Some(
                    cmd_time
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warccdx\0" as *const u8 as *const i8,
                place: &mut opt.warc_cdx_enabled as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warccdxdedup\0" as *const u8 as *const i8,
                place: &mut opt.warc_cdx_dedup_filename as *mut *mut i8
                    as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warccompression\0" as *const u8 as *const i8,
                place: &mut opt.warc_compression_enabled as *mut bool
                    as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warcdigests\0" as *const u8 as *const i8,
                place: &mut opt.warc_digests_enabled as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warcfile\0" as *const u8 as *const i8,
                place: &mut opt.warc_filename as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_file
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warcheader\0" as *const u8 as *const i8,
                place: 0 as *mut libc::c_void,
                action: Some(
                    cmd_spec_warc_header
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warckeeplog\0" as *const u8 as *const i8,
                place: &mut opt.warc_keep_log as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warcmaxsize\0" as *const u8 as *const i8,
                place: &mut opt.warc_maxsize as *mut wgint as *mut libc::c_void,
                action: Some(
                    cmd_bytes
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"warctempdir\0" as *const u8 as *const i8,
                place: &mut opt.warc_tempdir as *mut *mut i8 as *mut libc::c_void,
                action: Some(
                    cmd_directory
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                name: b"xattr\0" as *const u8 as *const i8,
                place: &mut opt.enable_xattr as *mut bool as *mut libc::c_void,
                action: Some(
                    cmd_boolean
                        as unsafe extern "C" fn(
                            *const i8,
                            *const i8,
                            *mut libc::c_void,
                        ) -> bool,
                ),
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];