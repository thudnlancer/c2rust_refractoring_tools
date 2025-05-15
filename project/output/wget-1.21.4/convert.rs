use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type hash_table;
    pub type ptimer;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn set_uri_encoding(i: *mut iri, charset: *const i8, force: bool);
    fn iri_free(i: *mut iri);
    fn rename(__old: *const i8, __new: *const i8) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn iri_new() -> *mut iri;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn quote(arg: *const i8) -> *const i8;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn unlink(__name: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn url_parse(
        _: *const i8,
        _: *mut i32,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_free(_: *mut url);
    fn uri_merge(_: *const i8, _: *const i8) -> *mut i8;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn wget_read_file(_: *const i8) -> *mut file_memory;
    fn wget_read_file_free(_: *mut file_memory);
    fn string_set_add(_: *mut hash_table, _: *const i8);
    fn string_set_contains(_: *mut hash_table, _: *const i8) -> i32;
    fn string_set_to_array(_: *mut hash_table, _: *mut *mut i8);
    fn print_decimal(_: libc::c_double) -> *const i8;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_get_pair(
        _: *const hash_table,
        _: *const libc::c_void,
        _: *mut libc::c_void,
        _: *mut libc::c_void,
    ) -> i32;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_for_each(
        _: *mut hash_table,
        _: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> i32,
        >,
        _: *mut libc::c_void,
    );
    fn hash_table_count(_: *const hash_table) -> i32;
    fn make_string_hash_table(_: i32) -> *mut hash_table;
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn get_urls_html(
        _: *const i8,
        _: *const i8,
        _: *mut bool,
        _: *mut iri,
    ) -> *mut urlpos;
    fn free_urlpos(_: *mut urlpos);
    fn get_urls_css_file(_: *const i8, _: *const i8) -> *mut urlpos;
    fn xstrndup(string: *const i8, n: size_t) -> *mut i8;
}
pub type __int64_t = i64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub enum convert_options {
    CO_NOCONVERT = 0,
    CO_CONVERT_TO_RELATIVE,
    CO_CONVERT_BASENAME_ONLY,
    CO_CONVERT_TO_COMPLETE,
}
impl convert_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            convert_options::CO_NOCONVERT => 0,
            convert_options::CO_CONVERT_TO_RELATIVE => 1,
            convert_options::CO_CONVERT_BASENAME_ONLY => 2,
            convert_options::CO_CONVERT_TO_COMPLETE => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> convert_options {
        match value {
            0 => convert_options::CO_NOCONVERT,
            1 => convert_options::CO_CONVERT_TO_RELATIVE,
            2 => convert_options::CO_CONVERT_BASENAME_ONLY,
            3 => convert_options::CO_CONVERT_TO_COMPLETE,
            _ => panic!("Invalid value for convert_options: {}", value),
        }
    }
}
impl AddAssign<u32> for convert_options {
    fn add_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for convert_options {
    fn sub_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for convert_options {
    fn mul_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for convert_options {
    fn div_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for convert_options {
    fn rem_assign(&mut self, rhs: u32) {
        *self = convert_options::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for convert_options {
    type Output = convert_options;
    fn add(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for convert_options {
    type Output = convert_options;
    fn sub(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for convert_options {
    type Output = convert_options;
    fn mul(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for convert_options {
    type Output = convert_options;
    fn div(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for convert_options {
    type Output = convert_options;
    fn rem(self, rhs: u32) -> convert_options {
        convert_options::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct urlpos {
    pub url: *mut url,
    pub local_name: *mut i8,
    #[bitfield(name = "ignore_when_downloading", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "link_relative_p", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "link_complete_p", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "link_base_p", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "link_inline_p", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "link_css_p", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "link_noquote_html_p", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "link_expect_html", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "link_expect_css", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "link_refresh_p", ty = "libc::c_uint", bits = "9..=9")]
    pub ignore_when_downloading_link_relative_p_link_complete_p_link_base_p_link_inline_p_link_css_p_link_noquote_html_p_link_expect_html_link_expect_css_link_refresh_p: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub refresh_timeout: i32,
    pub convert: convert_options,
    pub pos: i32,
    pub size: i32,
    pub next: *mut urlpos,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum downloaded_file_t {
    FILE_NOT_ALREADY_DOWNLOADED = 0,
    FILE_DOWNLOADED_NORMALLY,
    FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED,
    CHECK_FOR_FILE,
}
impl downloaded_file_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            downloaded_file_t::FILE_NOT_ALREADY_DOWNLOADED => 0,
            downloaded_file_t::FILE_DOWNLOADED_NORMALLY => 1,
            downloaded_file_t::FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED => 2,
            downloaded_file_t::CHECK_FOR_FILE => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> downloaded_file_t {
        match value {
            0 => downloaded_file_t::FILE_NOT_ALREADY_DOWNLOADED,
            1 => downloaded_file_t::FILE_DOWNLOADED_NORMALLY,
            2 => downloaded_file_t::FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED,
            3 => downloaded_file_t::CHECK_FOR_FILE,
            _ => panic!("Invalid value for downloaded_file_t: {}", value),
        }
    }
}
impl AddAssign<u32> for downloaded_file_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for downloaded_file_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for downloaded_file_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for downloaded_file_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for downloaded_file_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for downloaded_file_t {
    type Output = downloaded_file_t;
    fn add(self, rhs: u32) -> downloaded_file_t {
        downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for downloaded_file_t {
    type Output = downloaded_file_t;
    fn sub(self, rhs: u32) -> downloaded_file_t {
        downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for downloaded_file_t {
    type Output = downloaded_file_t;
    fn mul(self, rhs: u32) -> downloaded_file_t {
        downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for downloaded_file_t {
    type Output = downloaded_file_t;
    fn div(self, rhs: u32) -> downloaded_file_t {
        downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for downloaded_file_t {
    type Output = downloaded_file_t;
    fn rem(self, rhs: u32) -> downloaded_file_t {
        downloaded_file_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_memory {
    pub content: *mut i8,
    pub length: i64,
    pub mmap_p: i32,
}
static mut dl_file_url_map: *mut hash_table = 0 as *const hash_table as *mut hash_table;
#[no_mangle]
pub static mut dl_url_file_map: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
#[no_mangle]
pub static mut downloaded_html_set: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
#[no_mangle]
pub static mut downloaded_css_set: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn convert_links_in_hashtable(
    mut downloaded_set: *mut hash_table,
    mut is_css: i32,
    mut file_count: *mut i32,
) {
    let mut i: i32 = 0;
    let mut cnt: i32 = 0 as i32;
    let mut arr: [*mut i8; 1024] = [0 as *mut i8; 1024];
    let mut file_array: *mut *mut i8 = 0 as *mut *mut i8;
    if downloaded_set.is_null()
        || {
            cnt = hash_table_count(downloaded_set);
            cnt == 0 as i32
        }
    {
        return;
    }
    if cnt
        <= (::core::mem::size_of::<[*mut i8; 1024]>() as u64)
            .wrapping_div(::core::mem::size_of::<*mut i8>() as u64) as i32
    {
        file_array = arr.as_mut_ptr();
    } else {
        file_array = xmalloc(
            (cnt as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
        ) as *mut *mut i8;
    }
    string_set_to_array(downloaded_set, file_array);
    i = 0 as i32;
    while i < cnt {
        let mut urls: *mut urlpos = 0 as *mut urlpos;
        let mut cur_url: *mut urlpos = 0 as *mut urlpos;
        let mut url: *mut i8 = 0 as *mut i8;
        let mut file: *mut i8 = *file_array.offset(i as isize);
        url = hash_table_get(dl_file_url_map, file as *const libc::c_void) as *mut i8;
        if url.is_null() {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Apparently %s has been removed.\n\0" as *const u8 as *const i8,
                    file,
                );
            }
        } else {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Scanning %s (from %s)\n\0" as *const u8 as *const i8,
                    file,
                    url,
                );
            }
            urls = if is_css != 0 {
                get_urls_css_file(file, url)
            } else {
                get_urls_html(file, url, 0 as *mut bool, 0 as *mut iri)
            };
            cur_url = urls;
            while !cur_url.is_null() {
                let mut local_name: *mut i8 = 0 as *mut i8;
                let mut u: *mut url = 0 as *mut url;
                let mut pi: *mut iri = 0 as *mut iri;
                if (*cur_url).link_base_p() != 0 {
                    (*cur_url).convert = CO_NULLIFY_BASE;
                } else {
                    pi = iri_new();
                    set_uri_encoding(pi, opt.locale, 1 as i32 != 0);
                    u = url_parse(
                        (*(*cur_url).url).url,
                        0 as *mut i32,
                        pi,
                        1 as i32 != 0,
                    );
                    if !u.is_null() {
                        local_name = hash_table_get(
                            dl_url_file_map,
                            (*u).url as *const libc::c_void,
                        ) as *mut i8;
                        if !local_name.is_null() {
                            (*cur_url).convert = convert_options::from_libc_c_uint(
                                (if opt.convert_file_only as i32 != 0 {
                                    convert_options::CO_CONVERT_BASENAME_ONLY as i32
                                } else {
                                    convert_options::CO_CONVERT_TO_RELATIVE as i32
                                }) as u32,
                            );
                            (*cur_url).local_name = xstrdup(local_name);
                            if opt.debug as i64 != 0 {
                                debug_logprintf(
                                    b"will convert url %s to local %s\n\0" as *const u8
                                        as *const i8,
                                    (*u).url,
                                    local_name,
                                );
                            }
                        } else {
                            if (*cur_url).link_complete_p() == 0 {
                                (*cur_url).convert = convert_options::CO_CONVERT_TO_COMPLETE;
                            }
                            (*cur_url).local_name = 0 as *mut i8;
                            if opt.debug as i64 != 0 {
                                debug_logprintf(
                                    b"will convert url %s to complete\n\0" as *const u8
                                        as *const i8,
                                    (*u).url,
                                );
                            }
                        }
                        url_free(u);
                        iri_free(pi);
                    }
                }
                cur_url = (*cur_url).next;
            }
            convert_links(file, urls);
            *file_count += 1;
            *file_count;
            free_urlpos(urls);
        }
        i += 1;
        i;
    }
    if file_array != arr.as_mut_ptr() {
        rpl_free(file_array as *mut libc::c_void);
        file_array = 0 as *mut *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn convert_all_links() {
    let mut secs: libc::c_double = 0.;
    let mut file_count: i32 = 0 as i32;
    let mut timer: *mut ptimer = ptimer_new();
    convert_links_in_hashtable(downloaded_html_set, 0 as i32, &mut file_count);
    convert_links_in_hashtable(downloaded_css_set, 1 as i32, &mut file_count);
    secs = ptimer_measure(timer);
    logprintf(
        log_options::LOG_VERBOSE,
        dcgettext(
            0 as *const i8,
            b"Converted links in %d files in %s seconds.\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        file_count,
        print_decimal(secs),
    );
    ptimer_destroy(timer);
}
unsafe extern "C" fn convert_links(mut file: *const i8, mut links: *mut urlpos) {
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut p: *const i8 = 0 as *const i8;
    let mut downloaded_file_return: downloaded_file_t = downloaded_file_t::FILE_NOT_ALREADY_DOWNLOADED;
    let mut link: *mut urlpos = 0 as *mut urlpos;
    let mut to_url_count: i32 = 0 as i32;
    let mut to_file_count: i32 = 0 as i32;
    logprintf(
        log_options::LOG_VERBOSE,
        dcgettext(
            0 as *const i8,
            b"Converting links in %s... \0" as *const u8 as *const i8,
            5 as i32,
        ),
        file,
    );
    let mut dry_count: i32 = 0 as i32;
    let mut dry: *mut urlpos = 0 as *mut urlpos;
    dry = links;
    while !dry.is_null() {
        if (*dry).convert as u32 != convert_options::CO_NOCONVERT as i32 as u32 {
            dry_count += 1;
            dry_count;
        }
        dry = (*dry).next;
    }
    if dry_count == 0 {
        logputs(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"nothing to do.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return;
    }
    logprintf(
        log_options::LOG_VERBOSE,
        dcgettext(0 as *const i8, b"%d.\n\0" as *const u8 as *const i8, 5 as i32),
        dry_count,
    );
    fm = wget_read_file(file);
    if fm.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Cannot convert links in %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file,
            strerror(*__errno_location()),
        );
        return;
    }
    downloaded_file_return = downloaded_file(downloaded_file_t::CHECK_FOR_FILE, file);
    if opt.backup_converted as i32 != 0 && downloaded_file_return as u32 != 0 {
        write_backup_file(file, downloaded_file_return);
    }
    if unlink(file) < 0 as i32 && *__errno_location() != 2 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Unable to delete %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(file),
            strerror(*__errno_location()),
        );
        wget_read_file_free(fm);
        return;
    }
    fp = rpl_fopen(file, b"wb\0" as *const u8 as *const i8);
    if fp.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Cannot convert links in %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file,
            strerror(*__errno_location()),
        );
        wget_read_file_free(fm);
        return;
    }
    p = (*fm).content;
    link = links;
    while !link.is_null() {
        let mut url_start: *mut i8 = ((*fm).content).offset((*link).pos as isize);
        if (*link).pos as i64 >= (*fm).length {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Something strange is going on.  Please investigate.\0" as *const u8
                        as *const i8,
                );
            }
            break;
        } else {
            if (*link).convert as u32 == convert_options::CO_NOCONVERT as i32 as u32 {
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Skipping %s at position %d.\n\0" as *const u8 as *const i8,
                        (*(*link).url).url,
                        (*link).pos,
                    );
                }
            } else {
                fwrite(
                    p as *const libc::c_void,
                    1 as i32 as size_t,
                    url_start.offset_from(p) as i64 as size_t,
                    fp,
                );
                p = url_start;
                match (*link).convert as u32 {
                    1 => {
                        if !((*link).local_name).is_null() {
                            let mut newname: *mut i8 = construct_relative(
                                file,
                                (*link).local_name,
                            );
                            let mut quoted_newname: *mut i8 = local_quote_string(
                                newname,
                                (*link).link_css_p() != 0,
                            );
                            if (*link).link_css_p() as i32 != 0
                                || (*link).link_noquote_html_p() as i32 != 0
                            {
                                p = replace_plain(p, (*link).size, fp, quoted_newname);
                            } else if (*link).link_refresh_p() == 0 {
                                p = replace_attr(p, (*link).size, fp, quoted_newname);
                            } else {
                                p = replace_attr_refresh_hack(
                                    p,
                                    (*link).size,
                                    fp,
                                    quoted_newname,
                                    (*link).refresh_timeout,
                                );
                            }
                            if opt.debug as i64 != 0 {
                                debug_logprintf(
                                    b"TO_RELATIVE: %s to %s at position %d in %s.\n\0"
                                        as *const u8 as *const i8,
                                    (*(*link).url).url,
                                    newname,
                                    (*link).pos,
                                    file,
                                );
                            }
                            rpl_free(newname as *mut libc::c_void);
                            newname = 0 as *mut i8;
                            rpl_free(quoted_newname as *mut libc::c_void);
                            quoted_newname = 0 as *mut i8;
                            to_file_count += 1;
                            to_file_count;
                        }
                    }
                    2 => {
                        let mut newname_0: *mut i8 = convert_basename(p, link);
                        let mut quoted_newname_0: *mut i8 = local_quote_string(
                            newname_0,
                            (*link).link_css_p() != 0,
                        );
                        if (*link).link_css_p() as i32 != 0
                            || (*link).link_noquote_html_p() as i32 != 0
                        {
                            p = replace_plain(p, (*link).size, fp, quoted_newname_0);
                        } else if (*link).link_refresh_p() == 0 {
                            p = replace_attr(p, (*link).size, fp, quoted_newname_0);
                        } else {
                            p = replace_attr_refresh_hack(
                                p,
                                (*link).size,
                                fp,
                                quoted_newname_0,
                                (*link).refresh_timeout,
                            );
                        }
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"Converted file part only: %s to %s at position %d in %s.\n\0"
                                    as *const u8 as *const i8,
                                (*(*link).url).url,
                                newname_0,
                                (*link).pos,
                                file,
                            );
                        }
                        rpl_free(newname_0 as *mut libc::c_void);
                        newname_0 = 0 as *mut i8;
                        rpl_free(quoted_newname_0 as *mut libc::c_void);
                        quoted_newname_0 = 0 as *mut i8;
                        to_file_count += 1;
                        to_file_count;
                    }
                    3 => {
                        let mut newlink: *mut i8 = (*(*link).url).url;
                        let mut quoted_newlink: *mut i8 = html_quote_string(newlink);
                        if (*link).link_css_p() as i32 != 0
                            || (*link).link_noquote_html_p() as i32 != 0
                        {
                            p = replace_plain(p, (*link).size, fp, newlink);
                        } else if (*link).link_refresh_p() == 0 {
                            p = replace_attr(p, (*link).size, fp, quoted_newlink);
                        } else {
                            p = replace_attr_refresh_hack(
                                p,
                                (*link).size,
                                fp,
                                quoted_newlink,
                                (*link).refresh_timeout,
                            );
                        }
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"TO_COMPLETE: <something> to %s at position %d in %s.\n\0"
                                    as *const u8 as *const i8,
                                newlink,
                                (*link).pos,
                                file,
                            );
                        }
                        rpl_free(quoted_newlink as *mut libc::c_void);
                        quoted_newlink = 0 as *mut i8;
                        to_url_count += 1;
                        to_url_count;
                    }
                    4 => {
                        p = replace_attr(
                            p,
                            (*link).size,
                            fp,
                            b"\0" as *const u8 as *const i8,
                        );
                    }
                    0 => {
                        abort();
                    }
                    _ => {}
                }
            }
            link = (*link).next;
        }
    }
    if (p.offset_from((*fm).content) as i64) < (*fm).length {
        fwrite(
            p as *const libc::c_void,
            1 as i32 as size_t,
            ((*fm).length - p.offset_from((*fm).content) as i64) as size_t,
            fp,
        );
    }
    fclose(fp);
    wget_read_file_free(fm);
    logprintf(
        log_options::LOG_VERBOSE,
        b"%d-%d\n\0" as *const u8 as *const i8,
        to_file_count,
        to_url_count,
    );
}
unsafe extern "C" fn construct_relative(
    mut basefile: *const i8,
    mut linkfile: *const i8,
) -> *mut i8 {
    let mut link: *mut i8 = 0 as *mut i8;
    let mut basedirs: i32 = 0;
    let mut b: *const i8 = 0 as *const i8;
    let mut l: *const i8 = 0 as *const i8;
    let mut i: i32 = 0;
    let mut start: i32 = 0;
    start = 0 as i32;
    b = basefile;
    l = linkfile;
    while *b as i32 == *l as i32 && *b as i32 != '\0' as i32 {
        if *b as i32 == '/' as i32 {
            start = (b.offset_from(basefile) as i64 + 1 as i32 as i64) as i32;
        }
        b = b.offset(1);
        b;
        l = l.offset(1);
        l;
    }
    basefile = basefile.offset(start as isize);
    linkfile = linkfile.offset(start as isize);
    basedirs = 0 as i32;
    b = basefile;
    while *b != 0 {
        if *b as i32 == '/' as i32 {
            basedirs += 1;
            basedirs;
        }
        b = b.offset(1);
        b;
    }
    if basedirs == 0
        && {
            b = strpbrk(linkfile, b"/:\0" as *const u8 as *const i8);
            !b.is_null()
        } && *b as i32 == ':' as i32
    {
        link = xmalloc(
            (2 as i32 as u64)
                .wrapping_add(strlen(linkfile))
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        memcpy(
            link as *mut libc::c_void,
            b"./\0" as *const u8 as *const i8 as *const libc::c_void,
            2 as i32 as u64,
        );
        strcpy(link.offset(2 as i32 as isize), linkfile);
    } else {
        link = xmalloc(
            ((3 as i32 * basedirs) as u64)
                .wrapping_add(strlen(linkfile))
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        i = 0 as i32;
        while i < basedirs {
            memcpy(
                link.offset((3 as i32 * i) as isize) as *mut libc::c_void,
                b"../\0" as *const u8 as *const i8 as *const libc::c_void,
                3 as i32 as u64,
            );
            i += 1;
            i;
        }
        strcpy(link.offset((3 as i32 * i) as isize), linkfile);
    }
    return link;
}
unsafe extern "C" fn convert_basename(
    mut p: *const i8,
    mut link: *const urlpos,
) -> *mut i8 {
    let mut len: i32 = (*link).size;
    let mut url: *mut i8 = 0 as *mut i8;
    let mut org_basename: *mut i8 = 0 as *mut i8;
    let mut local_basename: *mut i8 = 0 as *mut i8;
    let mut result: *mut i8 = 0 as *mut i8;
    if *p as i32 == '"' as i32 || *p as i32 == '\'' as i32 {
        len -= 2 as i32;
        p = p.offset(1);
        p;
    }
    url = xstrndup(p, len as size_t);
    org_basename = strrchr(url, '/' as i32);
    if !org_basename.is_null() {
        org_basename = org_basename.offset(1);
        org_basename;
    } else {
        org_basename = url;
    }
    local_basename = if !((*link).local_name).is_null() {
        strrchr((*link).local_name, '/' as i32)
    } else {
        0 as *mut i8
    };
    if !local_basename.is_null() {
        local_basename = local_basename.offset(1);
        local_basename;
    } else {
        local_basename = url;
    }
    if strcmp(org_basename, local_basename) == 0 as i32 {
        result = url;
    } else {
        result = uri_merge(url, local_basename);
        rpl_free(url as *mut libc::c_void);
        url = 0 as *mut i8;
    }
    return result;
}
static mut converted_files: *mut hash_table = 0 as *const hash_table as *mut hash_table;
unsafe extern "C" fn write_backup_file(
    mut file: *const i8,
    mut downloaded_file_return: downloaded_file_t,
) {
    if converted_files.is_null() {
        converted_files = make_string_hash_table(0 as i32);
    }
    if string_set_contains(converted_files, file) == 0 {
        let mut buf: [i8; 1024] = [0; 1024];
        let mut filename_len: size_t = strlen(file);
        let mut filename_plus_orig_suffix: *mut i8 = 0 as *mut i8;
        if filename_len
            < (::core::mem::size_of::<[i8; 1024]>() as u64).wrapping_sub(5 as i32 as u64)
        {
            filename_plus_orig_suffix = buf.as_mut_ptr();
        } else {
            filename_plus_orig_suffix = xmalloc(
                filename_len.wrapping_add(5 as i32 as u64).wrapping_add(1 as i32 as u64),
            ) as *mut i8;
        }
        if downloaded_file_return as u32
            == downloaded_file_t::FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED as i32 as u32
        {
            memcpy(
                filename_plus_orig_suffix as *mut libc::c_void,
                file as *const libc::c_void,
                filename_len.wrapping_sub(4 as i32 as u64),
            );
            memcpy(
                filename_plus_orig_suffix
                    .offset(filename_len as isize)
                    .offset(-(4 as i32 as isize)) as *mut libc::c_void,
                b"orig\0" as *const u8 as *const i8 as *const libc::c_void,
                5 as i32 as u64,
            );
        } else {
            memcpy(
                filename_plus_orig_suffix as *mut libc::c_void,
                file as *const libc::c_void,
                filename_len,
            );
            strcpy(
                filename_plus_orig_suffix.offset(filename_len as isize),
                b".orig\0" as *const u8 as *const i8,
            );
        }
        if rename(file, filename_plus_orig_suffix) != 0 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Cannot back up %s as %s: %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                file,
                filename_plus_orig_suffix,
                strerror(*__errno_location()),
            );
        }
        if filename_plus_orig_suffix != buf.as_mut_ptr() {
            rpl_free(filename_plus_orig_suffix as *mut libc::c_void);
            filename_plus_orig_suffix = 0 as *mut i8;
        }
        string_set_add(converted_files, file);
    }
}
unsafe extern "C" fn replace_plain(
    mut p: *const i8,
    mut size: i32,
    mut fp: *mut FILE,
    mut new_text: *const i8,
) -> *const i8 {
    fputs(new_text, fp);
    p = p.offset(size as isize);
    return p;
}
unsafe extern "C" fn replace_attr(
    mut p: *const i8,
    mut size: i32,
    mut fp: *mut FILE,
    mut new_text: *const i8,
) -> *const i8 {
    let mut quote_flag: bool = 0 as i32 != 0;
    let mut quote_char: i8 = '"' as i32 as i8;
    let mut frag_beg: *const i8 = 0 as *const i8;
    let mut frag_end: *const i8 = 0 as *const i8;
    if *p as i32 == '"' as i32 || *p as i32 == '\'' as i32 {
        quote_char = *p;
        quote_flag = 1 as i32 != 0;
        p = p.offset(1);
        p;
        size -= 2 as i32;
    }
    _IO_putc(quote_char as i32, fp);
    fputs(new_text, fp);
    if find_fragment(p, size, &mut frag_beg, &mut frag_end) {
        fwrite(
            frag_beg as *const libc::c_void,
            1 as i32 as size_t,
            frag_end.offset_from(frag_beg) as i64 as size_t,
            fp,
        );
    }
    p = p.offset(size as isize);
    if quote_flag {
        p = p.offset(1);
        p;
    }
    _IO_putc(quote_char as i32, fp);
    return p;
}
unsafe extern "C" fn replace_attr_refresh_hack(
    mut p: *const i8,
    mut size: i32,
    mut fp: *mut FILE,
    mut new_text: *const i8,
    mut timeout: i32,
) -> *const i8 {
    let mut new_with_timeout: [i8; 1024] = [0; 1024];
    if snprintf(
        new_with_timeout.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 1024]>() as u64,
        b"%d; URL=%s\0" as *const u8 as *const i8,
        timeout,
        new_text,
    ) as u32 as u64 >= ::core::mem::size_of::<[i8; 1024]>() as u64
    {
        let mut tmp: *mut i8 = aprintf(
            b"%d; URL=%s\0" as *const u8 as *const i8,
            timeout,
            new_text,
        );
        let mut res: *const i8 = replace_attr(p, size, fp, tmp);
        rpl_free(tmp as *mut libc::c_void);
        tmp = 0 as *mut i8;
        return res;
    }
    return replace_attr(p, size, fp, new_with_timeout.as_mut_ptr());
}
unsafe extern "C" fn find_fragment(
    mut beg: *const i8,
    mut size: i32,
    mut bp: *mut *const i8,
    mut ep: *mut *const i8,
) -> bool {
    let mut end: *const i8 = beg.offset(size as isize);
    let mut saw_amp: bool = 0 as i32 != 0;
    while beg < end {
        let mut current_block_6: u64;
        match *beg as i32 {
            38 => {
                saw_amp = 1 as i32 != 0;
                current_block_6 = 5720623009719927633;
            }
            35 => {
                if !saw_amp {
                    *bp = beg;
                    *ep = end;
                    return 1 as i32 != 0;
                }
                current_block_6 = 17975318942413047110;
            }
            _ => {
                current_block_6 = 17975318942413047110;
            }
        }
        match current_block_6 {
            17975318942413047110 => {
                saw_amp = 0 as i32 != 0;
            }
            _ => {}
        }
        beg = beg.offset(1);
        beg;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn local_quote_string(
    mut file: *const i8,
    mut no_html_quote: bool,
) -> *mut i8 {
    let mut from: *const i8 = 0 as *const i8;
    let mut newname: *mut i8 = 0 as *mut i8;
    let mut to: *mut i8 = 0 as *mut i8;
    let mut res: *mut i8 = 0 as *mut i8;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut tolen: size_t = 0;
    let mut any: *mut i8 = strpbrk(file, b"?#%;\0" as *const u8 as *const i8);
    if any.is_null() {
        return if no_html_quote as i32 != 0 {
            strdup(file)
        } else {
            html_quote_string(file)
        };
    }
    tolen = (3 as i32 as u64).wrapping_mul(strlen(file));
    if tolen < ::core::mem::size_of::<[i8; 1024]>() as u64 {
        newname = buf.as_mut_ptr();
        to = newname;
    } else {
        newname = xmalloc(tolen.wrapping_add(1 as i32 as u64)) as *mut i8;
        to = newname;
    }
    from = file;
    while *from != 0 {
        let mut current_block_19: u64;
        match *from as i32 {
            37 => {
                let fresh0 = to;
                to = to.offset(1);
                *fresh0 = '%' as i32 as i8;
                let fresh1 = to;
                to = to.offset(1);
                *fresh1 = '2' as i32 as i8;
                let fresh2 = to;
                to = to.offset(1);
                *fresh2 = '5' as i32 as i8;
                current_block_19 = 4495394744059808450;
            }
            35 => {
                let fresh3 = to;
                to = to.offset(1);
                *fresh3 = '%' as i32 as i8;
                let fresh4 = to;
                to = to.offset(1);
                *fresh4 = '2' as i32 as i8;
                let fresh5 = to;
                to = to.offset(1);
                *fresh5 = '3' as i32 as i8;
                current_block_19 = 4495394744059808450;
            }
            59 => {
                let fresh6 = to;
                to = to.offset(1);
                *fresh6 = '%' as i32 as i8;
                let fresh7 = to;
                to = to.offset(1);
                *fresh7 = '3' as i32 as i8;
                let fresh8 = to;
                to = to.offset(1);
                *fresh8 = 'B' as i32 as i8;
                current_block_19 = 4495394744059808450;
            }
            63 => {
                if opt.adjust_extension {
                    let fresh9 = to;
                    to = to.offset(1);
                    *fresh9 = '%' as i32 as i8;
                    let fresh10 = to;
                    to = to.offset(1);
                    *fresh10 = '3' as i32 as i8;
                    let fresh11 = to;
                    to = to.offset(1);
                    *fresh11 = 'F' as i32 as i8;
                    current_block_19 = 4495394744059808450;
                } else {
                    current_block_19 = 54135129716598956;
                }
            }
            _ => {
                current_block_19 = 54135129716598956;
            }
        }
        match current_block_19 {
            54135129716598956 => {
                let fresh12 = to;
                to = to.offset(1);
                *fresh12 = *from;
            }
            _ => {}
        }
        from = from.offset(1);
        from;
    }
    *to = '\0' as i32 as i8;
    if newname == buf.as_mut_ptr() {
        return if no_html_quote as i32 != 0 {
            strdup(newname)
        } else {
            html_quote_string(newname)
        };
    }
    if no_html_quote {
        return newname;
    }
    res = html_quote_string(newname);
    rpl_free(newname as *mut libc::c_void);
    newname = 0 as *mut i8;
    return res;
}
unsafe extern "C" fn match_except_index(mut s1: *const i8, mut s2: *const i8) -> bool {
    let mut i: i32 = 0;
    let mut lng: *const i8 = 0 as *const i8;
    i = 0 as i32;
    while *s1 as i32 != 0 && *s2 as i32 != 0 && *s1 as i32 == *s2 as i32 {
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
        i += 1;
        i;
    }
    if i == 0 as i32 {
        return 0 as i32 != 0;
    }
    if *s1 == 0 && *s2 == 0 {
        return 1 as i32 != 0
    } else if *s1 as i32 != 0 && *s2 as i32 != 0 {
        return 0 as i32 != 0
    } else if *s1 != 0 {
        lng = s1;
    } else {
        lng = s2;
    }
    if *lng as i32 != '/' as i32 {
        lng = lng.offset(-1);
        lng;
    }
    if *lng as i32 == '/' as i32 && *lng.offset(1 as i32 as isize) as i32 == '\0' as i32
    {
        return 1 as i32 != 0;
    }
    return 0 as i32 == strcmp(lng, b"/index.html\0" as *const u8 as *const i8);
}
unsafe extern "C" fn dissociate_urls_from_file_mapper(
    mut key: *mut libc::c_void,
    mut value: *mut libc::c_void,
    mut arg: *mut libc::c_void,
) -> i32 {
    let mut mapping_url: *mut i8 = key as *mut i8;
    let mut mapping_file: *mut i8 = value as *mut i8;
    let mut file: *mut i8 = arg as *mut i8;
    if 0 as i32 == strcmp(mapping_file, file) {
        hash_table_remove(dl_url_file_map, mapping_url as *const libc::c_void);
        rpl_free(mapping_url as *mut libc::c_void);
        mapping_url = 0 as *mut i8;
        rpl_free(mapping_file as *mut libc::c_void);
        mapping_file = 0 as *mut i8;
    }
    return 0 as i32;
}
unsafe extern "C" fn dissociate_urls_from_file(mut file: *const i8) {
    hash_table_for_each(
        dl_url_file_map,
        Some(
            dissociate_urls_from_file_mapper
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> i32,
        ),
        file as *mut i8 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn register_download(mut url: *const i8, mut file: *const i8) {
    let mut current_block: u64;
    let mut old_file: *mut i8 = 0 as *mut i8;
    let mut old_url: *mut i8 = 0 as *mut i8;
    if dl_file_url_map.is_null() {
        dl_file_url_map = make_string_hash_table(0 as i32);
    }
    if dl_url_file_map.is_null() {
        dl_url_file_map = make_string_hash_table(0 as i32);
    }
    if hash_table_get_pair(
        dl_file_url_map,
        file as *const libc::c_void,
        &mut old_file as *mut *mut i8 as *mut libc::c_void,
        &mut old_url as *mut *mut i8 as *mut libc::c_void,
    ) != 0
    {
        if 0 as i32 == strcmp(url, old_url) {
            return;
        }
        if match_except_index(url, old_url) as i32 != 0
            && hash_table_contains(dl_url_file_map, url as *const libc::c_void) == 0
        {
            current_block = 3023573594330954718;
        } else {
            hash_table_remove(dl_file_url_map, file as *const libc::c_void);
            rpl_free(old_file as *mut libc::c_void);
            old_file = 0 as *mut i8;
            rpl_free(old_url as *mut libc::c_void);
            old_url = 0 as *mut i8;
            dissociate_urls_from_file(file);
            current_block = 5143058163439228106;
        }
    } else {
        current_block = 5143058163439228106;
    }
    match current_block {
        5143058163439228106 => {
            hash_table_put(
                dl_file_url_map,
                xstrdup(file) as *const libc::c_void,
                xstrdup(url) as *const libc::c_void,
            );
        }
        _ => {}
    }
    if hash_table_get_pair(
        dl_url_file_map,
        url as *const libc::c_void,
        &mut old_url as *mut *mut i8 as *mut libc::c_void,
        &mut old_file as *mut *mut i8 as *mut libc::c_void,
    ) != 0
    {
        hash_table_remove(dl_url_file_map, url as *const libc::c_void);
        rpl_free(old_url as *mut libc::c_void);
        old_url = 0 as *mut i8;
        rpl_free(old_file as *mut libc::c_void);
        old_file = 0 as *mut i8;
    }
    hash_table_put(
        dl_url_file_map,
        xstrdup(url) as *const libc::c_void,
        xstrdup(file) as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn register_redirection(mut from: *const i8, mut to: *const i8) {
    let mut file: *mut i8 = 0 as *mut i8;
    if dl_file_url_map.is_null() {
        dl_file_url_map = make_string_hash_table(0 as i32);
    }
    if dl_url_file_map.is_null() {
        dl_url_file_map = make_string_hash_table(0 as i32);
    }
    file = hash_table_get(dl_url_file_map, to as *const libc::c_void) as *mut i8;
    if hash_table_contains(dl_url_file_map, from as *const libc::c_void) == 0 {
        hash_table_put(
            dl_url_file_map,
            xstrdup(from) as *const libc::c_void,
            xstrdup(file) as *const libc::c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn register_delete_file(mut file: *const i8) {
    let mut old_url: *mut i8 = 0 as *mut i8;
    let mut old_file: *mut i8 = 0 as *mut i8;
    if dl_file_url_map.is_null() {
        dl_file_url_map = make_string_hash_table(0 as i32);
    }
    if dl_url_file_map.is_null() {
        dl_url_file_map = make_string_hash_table(0 as i32);
    }
    if hash_table_get_pair(
        dl_file_url_map,
        file as *const libc::c_void,
        &mut old_file as *mut *mut i8 as *mut libc::c_void,
        &mut old_url as *mut *mut i8 as *mut libc::c_void,
    ) == 0
    {
        return;
    }
    hash_table_remove(dl_file_url_map, file as *const libc::c_void);
    rpl_free(old_file as *mut libc::c_void);
    old_file = 0 as *mut i8;
    rpl_free(old_url as *mut libc::c_void);
    old_url = 0 as *mut i8;
    dissociate_urls_from_file(file);
}
#[no_mangle]
pub unsafe extern "C" fn register_html(mut file: *const i8) {
    if downloaded_html_set.is_null() {
        downloaded_html_set = make_string_hash_table(0 as i32);
    }
    string_set_add(downloaded_html_set, file);
}
#[no_mangle]
pub unsafe extern "C" fn register_css(mut file: *const i8) {
    if downloaded_css_set.is_null() {
        downloaded_css_set = make_string_hash_table(0 as i32);
    }
    string_set_add(downloaded_css_set, file);
}
static mut downloaded_files_hash: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn downloaded_mode_to_ptr(
    mut mode: downloaded_file_t,
) -> *mut downloaded_file_t {
    static mut v1: downloaded_file_t = downloaded_file_t::FILE_NOT_ALREADY_DOWNLOADED;
    static mut v2: downloaded_file_t = downloaded_file_t::FILE_DOWNLOADED_NORMALLY;
    static mut v3: downloaded_file_t = downloaded_file_t::FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED;
    static mut v4: downloaded_file_t = downloaded_file_t::CHECK_FOR_FILE;
    match mode as u32 {
        0 => return &mut v1,
        1 => return &mut v2,
        2 => return &mut v3,
        3 => return &mut v4,
        _ => {}
    }
    return 0 as *mut downloaded_file_t;
}
#[no_mangle]
pub unsafe extern "C" fn downloaded_file(
    mut mode: downloaded_file_t,
    mut file: *const i8,
) -> downloaded_file_t {
    let mut ptr: *mut downloaded_file_t = 0 as *mut downloaded_file_t;
    if mode as u32 == downloaded_file_t::CHECK_FOR_FILE as i32 as u32 {
        if downloaded_files_hash.is_null() {
            return downloaded_file_t::FILE_NOT_ALREADY_DOWNLOADED;
        }
        ptr = hash_table_get(downloaded_files_hash, file as *const libc::c_void)
            as *mut downloaded_file_t;
        if ptr.is_null() {
            return downloaded_file_t::FILE_NOT_ALREADY_DOWNLOADED;
        }
        return *ptr;
    }
    if downloaded_files_hash.is_null() {
        downloaded_files_hash = make_string_hash_table(0 as i32);
    }
    ptr = hash_table_get(downloaded_files_hash, file as *const libc::c_void)
        as *mut downloaded_file_t;
    if !ptr.is_null() {
        return *ptr;
    }
    ptr = downloaded_mode_to_ptr(mode);
    hash_table_put(
        downloaded_files_hash,
        xstrdup(file) as *const libc::c_void,
        ptr as *const libc::c_void,
    );
    return downloaded_file_t::FILE_NOT_ALREADY_DOWNLOADED;
}
#[no_mangle]
pub unsafe extern "C" fn html_quote_string(mut s: *const i8) -> *mut i8 {
    let mut b: *const i8 = s;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut res: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    i = 0 as i32;
    while *s != 0 {
        if *s as i32 == '&' as i32 {
            i += 4 as i32;
        } else if *s as i32 == '<' as i32 || *s as i32 == '>' as i32 {
            i += 3 as i32;
        } else if *s as i32 == '"' as i32 {
            i += 5 as i32;
        } else if *s as i32 == ' ' as i32 {
            i += 4 as i32;
        }
        s = s.offset(1);
        s;
        i += 1;
        i;
    }
    res = xmalloc((i + 1 as i32) as size_t) as *mut i8;
    s = b;
    p = res;
    while *s != 0 {
        match *s as i32 {
            38 => {
                let fresh13 = p;
                p = p.offset(1);
                *fresh13 = '&' as i32 as i8;
                let fresh14 = p;
                p = p.offset(1);
                *fresh14 = 'a' as i32 as i8;
                let fresh15 = p;
                p = p.offset(1);
                *fresh15 = 'm' as i32 as i8;
                let fresh16 = p;
                p = p.offset(1);
                *fresh16 = 'p' as i32 as i8;
                let fresh17 = p;
                p = p.offset(1);
                *fresh17 = ';' as i32 as i8;
            }
            60 | 62 => {
                let fresh18 = p;
                p = p.offset(1);
                *fresh18 = '&' as i32 as i8;
                let fresh19 = p;
                p = p.offset(1);
                *fresh19 = (if *s as i32 == '<' as i32 {
                    'l' as i32
                } else {
                    'g' as i32
                }) as i8;
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = 't' as i32 as i8;
                let fresh21 = p;
                p = p.offset(1);
                *fresh21 = ';' as i32 as i8;
            }
            34 => {
                let fresh22 = p;
                p = p.offset(1);
                *fresh22 = '&' as i32 as i8;
                let fresh23 = p;
                p = p.offset(1);
                *fresh23 = 'q' as i32 as i8;
                let fresh24 = p;
                p = p.offset(1);
                *fresh24 = 'u' as i32 as i8;
                let fresh25 = p;
                p = p.offset(1);
                *fresh25 = 'o' as i32 as i8;
                let fresh26 = p;
                p = p.offset(1);
                *fresh26 = 't' as i32 as i8;
                let fresh27 = p;
                p = p.offset(1);
                *fresh27 = ';' as i32 as i8;
            }
            32 => {
                let fresh28 = p;
                p = p.offset(1);
                *fresh28 = '&' as i32 as i8;
                let fresh29 = p;
                p = p.offset(1);
                *fresh29 = '#' as i32 as i8;
                let fresh30 = p;
                p = p.offset(1);
                *fresh30 = '3' as i32 as i8;
                let fresh31 = p;
                p = p.offset(1);
                *fresh31 = '2' as i32 as i8;
                let fresh32 = p;
                p = p.offset(1);
                *fresh32 = ';' as i32 as i8;
            }
            _ => {
                let fresh33 = p;
                p = p.offset(1);
                *fresh33 = *s;
            }
        }
        s = s.offset(1);
        s;
    }
    *p = '\0' as i32 as i8;
    return res;
}