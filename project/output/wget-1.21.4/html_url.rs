use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type hash_table;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strcspn(_: *const i8, _: *const i8) -> u64;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn rpl_strtol(string: *const i8, endptr: *mut *mut i8, base: i32) -> i64;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn set_content_encoding(i: *mut iri, charset: *const i8);
    fn set_uri_encoding(i: *mut iri, charset: *const i8, force: bool);
    fn iri_free(i: *mut iri);
    fn iri_new() -> *mut iri;
    fn parse_charset(str: *const i8) -> *mut i8;
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn quote(arg: *const i8) -> *const i8;
    fn quote_n(n: i32, arg: *const i8) -> *const i8;
    fn debug_logprintf(_: *const i8, _: ...);
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn __errno_location() -> *mut i32;
    fn inform_exit_status(err: uerr_t);
    fn map_html_tags(
        _: *const i8,
        _: i32,
        _: Option<unsafe extern "C" fn(*mut taginfo, *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
        _: i32,
        _: *const hash_table,
        _: *const hash_table,
    );
    fn url_parse(
        _: *const i8,
        _: *mut i32,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: i32) -> *const i8;
    fn url_has_scheme(_: *const i8) -> bool;
    fn uri_merge(_: *const i8, _: *const i8) -> *mut i8;
    fn rewrite_shorthand_url(_: *const i8) -> *mut i8;
    fn strdupdelim(_: *const i8, _: *const i8) -> *mut i8;
    fn wget_read_file(_: *const i8) -> *mut file_memory;
    fn wget_read_file_free(_: *mut file_memory);
    fn number_to_static_string(_: wgint) -> *mut i8;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> i32;
    fn make_nocase_string_hash_table(_: i32) -> *mut hash_table;
    fn get_urls_css(_: *mut map_context, _: i32, _: i32);
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn c_strncasecmp(s1: *const i8, s2: *const i8, n: size_t) -> i32;
}
pub type __int64_t = i64;
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
pub struct attr_pair {
    pub name: *mut i8,
    pub value: *mut i8,
    pub value_raw_beginning: *const i8,
    pub value_raw_size: i32,
    pub name_pool_index: i32,
    pub value_pool_index: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taginfo {
    pub name: *mut i8,
    pub end_tag_p: i32,
    pub nattrs: i32,
    pub attrs: *mut attr_pair,
    pub start_position: *const i8,
    pub end_position: *const i8,
    pub contents_begin: *const i8,
    pub contents_end: *const i8,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_memory {
    pub content: *mut i8,
    pub length: i64,
    pub mmap_p: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum convert_options {
    CO_NULLIFY_BASE = 4,
    CO_CONVERT_TO_COMPLETE = 3,
    CO_CONVERT_BASENAME_ONLY = 2,
    CO_CONVERT_TO_RELATIVE = 1,
    CO_NOCONVERT = 0,
}
impl convert_options {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            convert_options::CO_NULLIFY_BASE => 4,
            convert_options::CO_CONVERT_TO_COMPLETE => 3,
            convert_options::CO_CONVERT_BASENAME_ONLY => 2,
            convert_options::CO_CONVERT_TO_RELATIVE => 1,
            convert_options::CO_NOCONVERT => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> convert_options {
        match value {
            4 => convert_options::CO_NULLIFY_BASE,
            3 => convert_options::CO_CONVERT_TO_COMPLETE,
            2 => convert_options::CO_CONVERT_BASENAME_ONLY,
            1 => convert_options::CO_CONVERT_TO_RELATIVE,
            0 => convert_options::CO_NOCONVERT,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map_context {
    pub text: *mut i8,
    pub base: *mut i8,
    pub parent_base: *const i8,
    pub document_file: *const i8,
    pub nofollow: bool,
    pub head: *mut urlpos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct known_tag {
    pub tagid: i32,
    pub name: *const i8,
    pub handler: tag_handler_t,
}
pub type tag_handler_t = Option<
    unsafe extern "C" fn(i32, *mut taginfo, *mut map_context) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub tagid: i32,
    pub attr_name: *const i8,
    pub flags: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    TAG_SOURCE = 24,
    TAG_AUDIO = 23,
    TAG_VIDEO = 22,
    TAG_TH = 21,
    TAG_TD = 20,
    TAG_TABLE = 19,
    TAG_SCRIPT = 18,
    TAG_OVERLAY = 17,
    TAG_OBJECT = 16,
    TAG_LAYER = 13,
    TAG_INPUT = 12,
    TAG_IMG = 11,
    TAG_IFRAME = 10,
    TAG_FRAME = 9,
    TAG_FIG = 7,
    TAG_EMBED = 6,
    TAG_BODY = 5,
    TAG_BGSOUND = 4,
    TAG_AREA = 2,
    TAG_APPLET = 1,
    TAG_A = 0,
    TAG_META = 15,
    TAG_LINK = 14,
    TAG_FORM = 8,
    TAG_BASE = 3,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_5::TAG_SOURCE => 24,
            C2RustUnnamed_5::TAG_AUDIO => 23,
            C2RustUnnamed_5::TAG_VIDEO => 22,
            C2RustUnnamed_5::TAG_TH => 21,
            C2RustUnnamed_5::TAG_TD => 20,
            C2RustUnnamed_5::TAG_TABLE => 19,
            C2RustUnnamed_5::TAG_SCRIPT => 18,
            C2RustUnnamed_5::TAG_OVERLAY => 17,
            C2RustUnnamed_5::TAG_OBJECT => 16,
            C2RustUnnamed_5::TAG_LAYER => 13,
            C2RustUnnamed_5::TAG_INPUT => 12,
            C2RustUnnamed_5::TAG_IMG => 11,
            C2RustUnnamed_5::TAG_IFRAME => 10,
            C2RustUnnamed_5::TAG_FRAME => 9,
            C2RustUnnamed_5::TAG_FIG => 7,
            C2RustUnnamed_5::TAG_EMBED => 6,
            C2RustUnnamed_5::TAG_BODY => 5,
            C2RustUnnamed_5::TAG_BGSOUND => 4,
            C2RustUnnamed_5::TAG_AREA => 2,
            C2RustUnnamed_5::TAG_APPLET => 1,
            C2RustUnnamed_5::TAG_A => 0,
            C2RustUnnamed_5::TAG_META => 15,
            C2RustUnnamed_5::TAG_LINK => 14,
            C2RustUnnamed_5::TAG_FORM => 8,
            C2RustUnnamed_5::TAG_BASE => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_5 {
        match value {
            24 => C2RustUnnamed_5::TAG_SOURCE,
            23 => C2RustUnnamed_5::TAG_AUDIO,
            22 => C2RustUnnamed_5::TAG_VIDEO,
            21 => C2RustUnnamed_5::TAG_TH,
            20 => C2RustUnnamed_5::TAG_TD,
            19 => C2RustUnnamed_5::TAG_TABLE,
            18 => C2RustUnnamed_5::TAG_SCRIPT,
            17 => C2RustUnnamed_5::TAG_OVERLAY,
            16 => C2RustUnnamed_5::TAG_OBJECT,
            13 => C2RustUnnamed_5::TAG_LAYER,
            12 => C2RustUnnamed_5::TAG_INPUT,
            11 => C2RustUnnamed_5::TAG_IMG,
            10 => C2RustUnnamed_5::TAG_IFRAME,
            9 => C2RustUnnamed_5::TAG_FRAME,
            7 => C2RustUnnamed_5::TAG_FIG,
            6 => C2RustUnnamed_5::TAG_EMBED,
            5 => C2RustUnnamed_5::TAG_BODY,
            4 => C2RustUnnamed_5::TAG_BGSOUND,
            2 => C2RustUnnamed_5::TAG_AREA,
            1 => C2RustUnnamed_5::TAG_APPLET,
            0 => C2RustUnnamed_5::TAG_A,
            15 => C2RustUnnamed_5::TAG_META,
            14 => C2RustUnnamed_5::TAG_LINK,
            8 => C2RustUnnamed_5::TAG_FORM,
            3 => C2RustUnnamed_5::TAG_BASE,
            _ => panic!("Invalid value for C2RustUnnamed_5: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_5 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_5 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_5 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_5 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_5 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn add(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn sub(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn mul(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn div(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_5 {
    type Output = C2RustUnnamed_5;
    fn rem(self, rhs: u32) -> C2RustUnnamed_5 {
        C2RustUnnamed_5::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
static mut known_tags: [known_tag; 25] = unsafe {
    [
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_A as i32,
                name: b"a\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_APPLET as i32,
                name: b"applet\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_AREA as i32,
                name: b"area\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_BASE as i32,
                name: b"base\0" as *const u8 as *const i8,
                handler: Some(
                    tag_handle_base
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_BGSOUND as i32,
                name: b"bgsound\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_BODY as i32,
                name: b"body\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_EMBED as i32,
                name: b"embed\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_FIG as i32,
                name: b"fig\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_FORM as i32,
                name: b"form\0" as *const u8 as *const i8,
                handler: Some(
                    tag_handle_form
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_FRAME as i32,
                name: b"frame\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_IFRAME as i32,
                name: b"iframe\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_IMG as i32,
                name: b"img\0" as *const u8 as *const i8,
                handler: Some(
                    tag_handle_img
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_INPUT as i32,
                name: b"input\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_LAYER as i32,
                name: b"layer\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_LINK as i32,
                name: b"link\0" as *const u8 as *const i8,
                handler: Some(
                    tag_handle_link
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_META as i32,
                name: b"meta\0" as *const u8 as *const i8,
                handler: Some(
                    tag_handle_meta
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_OBJECT as i32,
                name: b"object\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_OVERLAY as i32,
                name: b"overlay\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_SCRIPT as i32,
                name: b"script\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_TABLE as i32,
                name: b"table\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_TD as i32,
                name: b"td\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_TH as i32,
                name: b"th\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_VIDEO as i32,
                name: b"video\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_AUDIO as i32,
                name: b"audio\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = known_tag {
                tagid: C2RustUnnamed_5::TAG_SOURCE as i32,
                name: b"source\0" as *const u8 as *const i8,
                handler: Some(
                    tag_find_urls
                        as unsafe extern "C" fn(
                            i32,
                            *mut taginfo,
                            *mut map_context,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
static mut tag_url_attributes: [C2RustUnnamed_4; 26] = [
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_A as i32,
            attr_name: b"href\0" as *const u8 as *const i8,
            flags: 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_APPLET as i32,
            attr_name: b"code\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_AREA as i32,
            attr_name: b"href\0" as *const u8 as *const i8,
            flags: 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_BGSOUND as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_BODY as i32,
            attr_name: b"background\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_EMBED as i32,
            attr_name: b"href\0" as *const u8 as *const i8,
            flags: 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_EMBED as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32 | 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_FIG as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_FRAME as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32 | 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_IFRAME as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32 | 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_IMG as i32,
            attr_name: b"href\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_IMG as i32,
            attr_name: b"lowsrc\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_IMG as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_INPUT as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_LAYER as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32 | 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_OBJECT as i32,
            attr_name: b"data\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_OVERLAY as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32 | 2 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_SCRIPT as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_TABLE as i32,
            attr_name: b"background\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_TD as i32,
            attr_name: b"background\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_TH as i32,
            attr_name: b"background\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_VIDEO as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_VIDEO as i32,
            attr_name: b"poster\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_AUDIO as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_AUDIO as i32,
            attr_name: b"poster\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_4 {
            tagid: C2RustUnnamed_5::TAG_SOURCE as i32,
            attr_name: b"src\0" as *const u8 as *const i8,
            flags: 1 as i32,
        };
        init
    },
];
static mut additional_attributes: [*const i8; 8] = [
    b"rel\0" as *const u8 as *const i8,
    b"type\0" as *const u8 as *const i8,
    b"http-equiv\0" as *const u8 as *const i8,
    b"name\0" as *const u8 as *const i8,
    b"content\0" as *const u8 as *const i8,
    b"action\0" as *const u8 as *const i8,
    b"style\0" as *const u8 as *const i8,
    b"srcset\0" as *const u8 as *const i8,
];
static mut interesting_tags: *mut hash_table = 0 as *const hash_table as *mut hash_table;
static mut interesting_attributes: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
static mut meta_charset: *mut i8 = 0 as *const i8 as *mut i8;
unsafe extern "C" fn init_interesting() {
    let mut i: size_t = 0;
    interesting_tags = make_nocase_string_hash_table(
        (::core::mem::size_of::<[known_tag; 25]>() as u64)
            .wrapping_div(::core::mem::size_of::<known_tag>() as u64) as i32,
    );
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[known_tag; 25]>() as u64)
            .wrapping_div(::core::mem::size_of::<known_tag>() as u64)
    {
        hash_table_put(
            interesting_tags,
            known_tags[i as usize].name as *const libc::c_void,
            known_tags.as_mut_ptr().offset(i as isize) as *const libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    if !(opt.ignore_tags).is_null() {
        let mut ignored: *mut *mut i8 = 0 as *mut *mut i8;
        ignored = opt.ignore_tags;
        while !(*ignored).is_null() {
            hash_table_remove(interesting_tags, *ignored as *const libc::c_void);
            ignored = ignored.offset(1);
            ignored;
        }
    }
    if !(opt.follow_tags).is_null() {
        let mut intersect: *mut hash_table = make_nocase_string_hash_table(0 as i32);
        let mut followed: *mut *mut i8 = 0 as *mut *mut i8;
        followed = opt.follow_tags;
        while !(*followed).is_null() {
            let mut t: *mut known_tag = hash_table_get(
                interesting_tags,
                *followed as *const libc::c_void,
            ) as *mut known_tag;
            if !t.is_null() {
                hash_table_put(
                    intersect,
                    *followed as *const libc::c_void,
                    t as *const libc::c_void,
                );
            }
            followed = followed.offset(1);
            followed;
        }
        hash_table_destroy(interesting_tags);
        interesting_tags = intersect;
    }
    interesting_attributes = make_nocase_string_hash_table(10 as i32);
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[*const i8; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
    {
        hash_table_put(
            interesting_attributes,
            additional_attributes[i as usize] as *const libc::c_void,
            b"1\0" as *const u8 as *const i8 as *const libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed_4; 26]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_4>() as u64)
    {
        hash_table_put(
            interesting_attributes,
            tag_url_attributes[i as usize].attr_name as *const libc::c_void,
            b"1\0" as *const u8 as *const i8 as *const libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn find_attr(
    mut tag: *mut taginfo,
    mut name: *const i8,
    mut attrind: *mut i32,
) -> *mut i8 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*tag).nattrs {
        if c_strcasecmp((*((*tag).attrs).offset(i as isize)).name, name) == 0 {
            if !attrind.is_null() {
                *attrind = i;
            }
            return (*((*tag).attrs).offset(i as isize)).value;
        }
        i += 1;
        i;
    }
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn append_url(
    mut link_uri: *const i8,
    mut position: i32,
    mut size: i32,
    mut ctx: *mut map_context,
) -> *mut urlpos {
    let mut link_has_scheme: i32 = url_has_scheme(link_uri) as i32;
    let mut newel: *mut urlpos = 0 as *mut urlpos;
    let mut base: *const i8 = if !((*ctx).base).is_null() {
        (*ctx).base
    } else {
        (*ctx).parent_base
    };
    let mut url: *mut url = 0 as *mut url;
    let mut iri: *mut iri = iri_new();
    set_uri_encoding(iri, opt.locale, 1 as i32 != 0);
    (*iri).utf8_encode = 1 as i32 != 0;
    if base.is_null() {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"%s: no base, merge will use \"%s\".\n\0" as *const u8 as *const i8,
                (*ctx).document_file,
                link_uri,
            );
        }
        if link_has_scheme == 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: Cannot resolve incomplete link %s.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*ctx).document_file,
                link_uri,
            );
            iri_free(iri);
            return 0 as *mut urlpos;
        }
        url = url_parse(link_uri, 0 as *mut i32, iri, 0 as i32 != 0);
        if url.is_null() {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"%s: link \"%s\" doesn't parse.\n\0" as *const u8 as *const i8,
                    (*ctx).document_file,
                    link_uri,
                );
            }
            iri_free(iri);
            return 0 as *mut urlpos;
        }
    } else {
        let mut complete_uri: *mut i8 = uri_merge(base, link_uri);
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"%s: merge(%s, %s) -> %s\n\0" as *const u8 as *const i8,
                quotearg_n_style(
                    0 as i32,
                    quoting_style::escape_quoting_style,
                    (*ctx).document_file,
                ),
                quote_n(1 as i32, base),
                quote_n(2 as i32, link_uri),
                quotearg_n_style(
                    3 as i32,
                    quoting_style::escape_quoting_style,
                    complete_uri,
                ),
            );
        }
        url = url_parse(complete_uri, 0 as *mut i32, iri, 0 as i32 != 0);
        if url.is_null() {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"%s: merged link \"%s\" doesn't parse.\n\0" as *const u8
                        as *const i8,
                    (*ctx).document_file,
                    complete_uri,
                );
            }
            rpl_free(complete_uri as *mut libc::c_void);
            complete_uri = 0 as *mut i8;
            iri_free(iri);
            return 0 as *mut urlpos;
        }
        rpl_free(complete_uri as *mut libc::c_void);
        complete_uri = 0 as *mut i8;
    }
    iri_free(iri);
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"appending %s to urlpos.\n\0" as *const u8 as *const i8,
            quote((*url).url),
        );
    }
    newel = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<urlpos>() as u64)
        as *mut urlpos;
    (*newel).url = url;
    (*newel).pos = position;
    (*newel).size = size;
    if link_has_scheme == 0 && *link_uri as i32 != '/' as i32 {
        (*newel).set_link_relative_p(1 as i32 as u32);
    } else if link_has_scheme != 0 {
        (*newel).set_link_complete_p(1 as i32 as u32);
    }
    if ((*ctx).head).is_null() {
        (*ctx).head = newel;
    } else {
        let mut it: *mut urlpos = 0 as *mut urlpos;
        let mut prev: *mut urlpos = 0 as *mut urlpos;
        it = (*ctx).head;
        while !it.is_null() && position > (*it).pos {
            prev = it;
            it = (*it).next;
        }
        (*newel).next = it;
        if !prev.is_null() {
            (*prev).next = newel;
        } else {
            (*ctx).head = newel;
        }
    }
    return newel;
}
unsafe extern "C" fn check_style_attr(mut tag: *mut taginfo, mut ctx: *mut map_context) {
    let mut attrind: i32 = 0;
    let mut raw_start: i32 = 0;
    let mut raw_len: i32 = 0;
    let mut style: *mut i8 = find_attr(
        tag,
        b"style\0" as *const u8 as *const i8,
        &mut attrind,
    );
    if style.is_null() {
        return;
    }
    raw_start = ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
        .offset_from((*ctx).text) as i64 as i32;
    raw_len = (*((*tag).attrs).offset(attrind as isize)).value_raw_size;
    if *((*ctx).text).offset(raw_start as isize) as i32 == '\'' as i32
        || *((*ctx).text).offset(raw_start as isize) as i32 == '"' as i32
    {
        raw_start += 1 as i32;
        raw_len -= 2 as i32;
    }
    if raw_len <= 0 as i32 {
        return;
    }
    get_urls_css(ctx, raw_start, raw_len);
}
unsafe extern "C" fn tag_find_urls(
    mut tagid: i32,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut i: size_t = 0;
    let mut attrind: i32 = 0;
    let mut first: i32 = -(1 as i32);
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed_4; 26]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_4>() as u64)
    {
        if tag_url_attributes[i as usize].tagid == tagid {
            first = i as i32;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    attrind = 0 as i32;
    while attrind < (*tag).nattrs {
        let mut link: *mut i8 = (*((*tag).attrs).offset(attrind as isize)).value;
        let size: size_t = (::core::mem::size_of::<[C2RustUnnamed_4; 26]>() as u64)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_4>() as u64);
        i = first as size_t;
        while i < size && tag_url_attributes[i as usize].tagid == tagid {
            if 0 as i32
                == strcasecmp(
                    (*((*tag).attrs).offset(attrind as isize)).name,
                    tag_url_attributes[i as usize].attr_name,
                )
            {
                let mut up: *mut urlpos = append_url(
                    link,
                    ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                        .offset_from((*ctx).text) as i64 as i32,
                    (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
                    ctx,
                );
                if !up.is_null() {
                    let mut flags: i32 = tag_url_attributes[i as usize].flags;
                    if flags & 1 as i32 != 0 {
                        (*up).set_link_inline_p(1 as i32 as u32);
                    }
                    if flags & 2 as i32 != 0 {
                        (*up).set_link_expect_html(1 as i32 as u32);
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        attrind += 1;
        attrind;
    }
}
unsafe extern "C" fn tag_handle_base(
    mut tagid: i32,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut base_urlpos: *mut urlpos = 0 as *mut urlpos;
    let mut attrind: i32 = 0;
    let mut newbase: *mut i8 = find_attr(
        tag,
        b"href\0" as *const u8 as *const i8,
        &mut attrind,
    );
    if newbase.is_null() {
        return;
    }
    base_urlpos = append_url(
        newbase,
        ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
            .offset_from((*ctx).text) as i64 as i32,
        (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
        ctx,
    );
    if base_urlpos.is_null() {
        return;
    }
    (*base_urlpos).set_ignore_when_downloading(1 as i32 as u32);
    (*base_urlpos).set_link_base_p(1 as i32 as u32);
    rpl_free((*ctx).base as *mut libc::c_void);
    (*ctx).base = 0 as *mut i8;
    if !((*ctx).parent_base).is_null() {
        (*ctx).base = uri_merge((*ctx).parent_base, newbase);
    } else {
        (*ctx).base = xstrdup(newbase);
    };
}
unsafe extern "C" fn tag_handle_form(
    mut tagid: i32,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut attrind: i32 = 0;
    let mut action: *mut i8 = find_attr(
        tag,
        b"action\0" as *const u8 as *const i8,
        &mut attrind,
    );
    if !action.is_null() {
        let mut up: *mut urlpos = append_url(
            action,
            ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                .offset_from((*ctx).text) as i64 as i32,
            (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
            ctx,
        );
        if !up.is_null() {
            (*up).set_ignore_when_downloading(1 as i32 as u32);
        }
    }
}
unsafe extern "C" fn tag_handle_link(
    mut tagid: i32,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut attrind: i32 = 0;
    let mut href: *mut i8 = find_attr(
        tag,
        b"href\0" as *const u8 as *const i8,
        &mut attrind,
    );
    if !href.is_null() {
        let mut up: *mut urlpos = append_url(
            href,
            ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                .offset_from((*ctx).text) as i64 as i32,
            (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
            ctx,
        );
        if !up.is_null() {
            let mut rel: *mut i8 = find_attr(
                tag,
                b"rel\0" as *const u8 as *const i8,
                0 as *mut i32,
            );
            if !rel.is_null() {
                if 0 as i32
                    == c_strcasecmp(rel, b"stylesheet\0" as *const u8 as *const i8)
                    || 0 as i32
                        == c_strcasecmp(
                            rel,
                            b"alternate stylesheet\0" as *const u8 as *const i8,
                        )
                {
                    (*up).set_link_inline_p(1 as i32 as u32);
                    (*up).set_link_expect_css(1 as i32 as u32);
                } else if 0 as i32
                    == c_strcasecmp(rel, b"shortcut icon\0" as *const u8 as *const i8)
                    || 0 as i32 == c_strcasecmp(rel, b"icon\0" as *const u8 as *const i8)
                {
                    (*up).set_link_inline_p(1 as i32 as u32);
                } else if 0 as i32
                    == c_strcasecmp(rel, b"manifest\0" as *const u8 as *const i8)
                {
                    (*up).set_link_inline_p(1 as i32 as u32);
                } else {
                    let mut type_0: *mut i8 = find_attr(
                        tag,
                        b"type\0" as *const u8 as *const i8,
                        0 as *mut i32,
                    );
                    if type_0.is_null()
                        || c_strcasecmp(type_0, b"text/html\0" as *const u8 as *const i8)
                            == 0 as i32
                    {
                        (*up).set_link_expect_html(1 as i32 as u32);
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn tag_handle_meta(
    mut tagid: i32,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut name: *mut i8 = find_attr(
        tag,
        b"name\0" as *const u8 as *const i8,
        0 as *mut i32,
    );
    let mut http_equiv: *mut i8 = find_attr(
        tag,
        b"http-equiv\0" as *const u8 as *const i8,
        0 as *mut i32,
    );
    if !http_equiv.is_null()
        && 0 as i32 == c_strcasecmp(http_equiv, b"refresh\0" as *const u8 as *const i8)
    {
        let mut entry: *mut urlpos = 0 as *mut urlpos;
        let mut attrind: i32 = 0;
        let mut timeout: i32 = 0;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut refresh: *mut i8 = find_attr(
            tag,
            b"content\0" as *const u8 as *const i8,
            &mut attrind,
        );
        if refresh.is_null() {
            return;
        }
        timeout = rpl_strtol(refresh, &mut p, 10 as i32) as i32;
        if timeout < 0 as i32
            || {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 as i32 != ';' as i32
            }
        {
            return;
        }
        while c_isspace(*p as i32) {
            p = p.offset(1);
            p;
        }
        if !(c_toupper(*p as i32) == 'U' as i32
            && c_toupper(*p.offset(1 as i32 as isize) as i32) == 'R' as i32
            && c_toupper(*p.offset(2 as i32 as isize) as i32) == 'L' as i32
            && *p.offset(3 as i32 as isize) as i32 == '=' as i32)
        {
            return;
        }
        p = p.offset(4 as i32 as isize);
        while c_isspace(*p as i32) {
            p = p.offset(1);
            p;
        }
        entry = append_url(
            p,
            ((*((*tag).attrs).offset(attrind as isize)).value_raw_beginning)
                .offset_from((*ctx).text) as i64 as i32,
            (*((*tag).attrs).offset(attrind as isize)).value_raw_size,
            ctx,
        );
        if !entry.is_null() {
            (*entry).set_link_refresh_p(1 as i32 as u32);
            (*entry).refresh_timeout = timeout;
            (*entry).set_link_expect_html(1 as i32 as u32);
        }
    } else if !http_equiv.is_null()
        && 0 as i32
            == c_strcasecmp(http_equiv, b"content-type\0" as *const u8 as *const i8)
    {
        let mut mcharset: *mut i8 = 0 as *mut i8;
        let mut content: *mut i8 = find_attr(
            tag,
            b"content\0" as *const u8 as *const i8,
            0 as *mut i32,
        );
        if content.is_null() {
            return;
        }
        mcharset = parse_charset(content);
        if mcharset.is_null() {
            return;
        }
        rpl_free(meta_charset as *mut libc::c_void);
        meta_charset = 0 as *mut i8;
        meta_charset = mcharset;
    } else if !name.is_null()
        && 0 as i32 == c_strcasecmp(name, b"robots\0" as *const u8 as *const i8)
    {
        let mut content_0: *mut i8 = find_attr(
            tag,
            b"content\0" as *const u8 as *const i8,
            0 as *mut i32,
        );
        if content_0.is_null() {
            return;
        }
        if c_strcasecmp(content_0, b"none\0" as *const u8 as *const i8) == 0 {
            (*ctx).nofollow = 1 as i32 != 0;
        } else {
            while *content_0 != 0 {
                let mut end: *mut i8 = 0 as *mut i8;
                content_0 = content_0
                    .offset(
                        strspn(content_0, b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8)
                            as isize,
                    );
                end = content_0
                    .offset(
                        strcspn(
                            content_0,
                            b", \x0C\n\r\t\x0B\0" as *const u8 as *const i8,
                        ) as isize,
                    );
                if c_strncasecmp(
                    content_0,
                    b"nofollow\0" as *const u8 as *const i8,
                    end.offset_from(content_0) as i64 as size_t,
                ) == 0
                {
                    (*ctx).nofollow = 1 as i32 != 0;
                }
                if *end as i32 == ',' as i32 {
                    end = end.offset(1);
                    end;
                } else {
                    end = strchr(end, ',' as i32);
                    if !end.is_null() {
                        end = end.offset(1);
                        end;
                    } else {
                        end = content_0.offset(strlen(content_0) as isize);
                    }
                }
                content_0 = end;
            }
        }
    }
}
unsafe extern "C" fn tag_handle_img(
    mut tagid: i32,
    mut tag: *mut taginfo,
    mut ctx: *mut map_context,
) {
    let mut attrind: i32 = 0;
    let mut srcset: *mut i8 = 0 as *mut i8;
    tag_find_urls(tagid, tag, ctx);
    srcset = find_attr(tag, b"srcset\0" as *const u8 as *const i8, &mut attrind);
    if !srcset.is_null() {
        let mut base_ind: i32 = ((*((*tag).attrs).offset(attrind as isize))
            .value_raw_beginning)
            .offset_from((*ctx).text) as i64 as i32;
        let mut size: i32 = strlen(srcset) as i32;
        let mut offset: i32 = 0;
        let mut url_start: i32 = 0;
        let mut url_end: i32 = 0;
        if *((*ctx).text).offset(base_ind as isize) as i32 == '"' as i32
            || *((*ctx).text).offset(base_ind as isize) as i32 == '\'' as i32
        {
            base_ind += 1;
            base_ind;
        }
        offset = 0 as i32;
        while offset < size {
            let mut has_descriptor: bool = 1 as i32 != 0;
            url_start = (offset as u64)
                .wrapping_add(
                    strspn(
                        srcset.offset(offset as isize),
                        b" \x0C\n\r\t,\0" as *const u8 as *const i8,
                    ),
                ) as i32;
            if url_start == size {
                return;
            }
            url_end = (url_start as u64)
                .wrapping_add(
                    strcspn(
                        srcset.offset(url_start as isize),
                        b" \x0C\n\r\t\0" as *const u8 as *const i8,
                    ),
                ) as i32;
            while url_end - 1 as i32 > url_start
                && *srcset.offset((url_end - 1 as i32) as isize) as i32 == ',' as i32
            {
                has_descriptor = 0 as i32 != 0;
                url_end -= 1;
                url_end;
            }
            if url_end > url_start {
                let mut url_text: *mut i8 = strdupdelim(
                    srcset.offset(url_start as isize),
                    srcset.offset(url_end as isize),
                );
                let mut up: *mut urlpos = append_url(
                    url_text,
                    base_ind + url_start,
                    url_end - url_start,
                    ctx,
                );
                if !up.is_null() {
                    (*up).set_link_inline_p(1 as i32 as u32);
                    (*up).set_link_noquote_html_p(1 as i32 as u32);
                }
                rpl_free(url_text as *mut libc::c_void);
                url_text = 0 as *mut i8;
            }
            if has_descriptor {
                let mut in_paren: bool = 0 as i32 != 0;
                offset = url_end;
                while offset < size {
                    let mut c: i8 = *srcset.offset(offset as isize);
                    if c as i32 == '(' as i32 {
                        in_paren = 1 as i32 != 0;
                    } else if c as i32 == ')' as i32 && in_paren as i32 != 0 {
                        in_paren = 0 as i32 != 0;
                    } else if c as i32 == ',' as i32 && !in_paren {
                        break;
                    }
                    offset += 1;
                    offset;
                }
            } else {
                offset = url_end;
            }
        }
    }
}
unsafe extern "C" fn collect_tags_mapper(
    mut tag: *mut taginfo,
    mut arg: *mut libc::c_void,
) {
    let mut ctx: *mut map_context = arg as *mut map_context;
    let mut t: *mut known_tag = hash_table_get(
        interesting_tags,
        (*tag).name as *const libc::c_void,
    ) as *mut known_tag;
    if !t.is_null() {
        ((*t).handler).expect("non-null function pointer")((*t).tagid, tag, ctx);
    }
    check_style_attr(tag, ctx);
    if (*tag).end_tag_p != 0
        && 0 as i32 == c_strcasecmp((*tag).name, b"style\0" as *const u8 as *const i8)
        && !((*tag).contents_begin).is_null() && !((*tag).contents_end).is_null()
        && (*tag).contents_begin <= (*tag).contents_end
    {
        get_urls_css(
            ctx,
            ((*tag).contents_begin).offset_from((*ctx).text) as i64 as i32,
            ((*tag).contents_end).offset_from((*tag).contents_begin) as i64 as i32,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_urls_html_fm(
    mut file: *const i8,
    mut fm: *const file_memory,
    mut url: *const i8,
    mut meta_disallow_follow: *mut bool,
    mut iri: *mut iri,
) -> *mut urlpos {
    let mut ctx: map_context = map_context {
        text: 0 as *mut i8,
        base: 0 as *mut i8,
        parent_base: 0 as *const i8,
        document_file: 0 as *const i8,
        nofollow: false,
        head: 0 as *mut urlpos,
    };
    let mut flags: i32 = 0;
    ctx.text = (*fm).content;
    ctx.head = 0 as *mut urlpos;
    ctx.base = 0 as *mut i8;
    ctx.parent_base = if !url.is_null() { url } else { opt.base_href };
    ctx.document_file = file;
    ctx.nofollow = 0 as i32 != 0;
    if interesting_tags.is_null() {
        init_interesting();
    }
    flags = 2 as i32;
    if opt.strict_comments {
        flags |= 1 as i32;
    }
    map_html_tags(
        (*fm).content,
        (*fm).length as i32,
        Some(
            collect_tags_mapper
                as unsafe extern "C" fn(*mut taginfo, *mut libc::c_void) -> (),
        ),
        &mut ctx as *mut map_context as *mut libc::c_void,
        flags,
        0 as *const hash_table,
        interesting_attributes,
    );
    if !iri.is_null() && ((*iri).content_encoding).is_null() && !meta_charset.is_null() {
        set_content_encoding(iri, meta_charset);
    }
    rpl_free(meta_charset as *mut libc::c_void);
    meta_charset = 0 as *mut i8;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"nofollow in %s: %d\n\0" as *const u8 as *const i8,
            file,
            ctx.nofollow as i32,
        );
    }
    if !meta_disallow_follow.is_null() {
        *meta_disallow_follow = ctx.nofollow;
    }
    rpl_free(ctx.base as *mut libc::c_void);
    ctx.base = 0 as *mut i8;
    return ctx.head;
}
#[no_mangle]
pub unsafe extern "C" fn get_urls_html(
    mut file: *const i8,
    mut url: *const i8,
    mut meta_disallow_follow: *mut bool,
    mut iri: *mut iri,
) -> *mut urlpos {
    let mut urls: *mut urlpos = 0 as *mut urlpos;
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    fm = wget_read_file(file);
    if fm.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const i8,
            file,
            strerror(*__errno_location()),
        );
        return 0 as *mut urlpos;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Loaded %s (size %s).\n\0" as *const u8 as *const i8,
            file,
            number_to_static_string((*fm).length),
        );
    }
    urls = get_urls_html_fm(file, fm, url, meta_disallow_follow, iri);
    wget_read_file_free(fm);
    return urls;
}
#[no_mangle]
pub unsafe extern "C" fn get_urls_file(mut file: *const i8) -> *mut urlpos {
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut head: *mut urlpos = 0 as *mut urlpos;
    let mut tail: *mut urlpos = 0 as *mut urlpos;
    let mut text: *const i8 = 0 as *const i8;
    let mut text_end: *const i8 = 0 as *const i8;
    fm = wget_read_file(file);
    if fm.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const i8,
            file,
            strerror(*__errno_location()),
        );
        return 0 as *mut urlpos;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Loaded %s (size %s).\n\0" as *const u8 as *const i8,
            file,
            number_to_static_string((*fm).length),
        );
    }
    tail = 0 as *mut urlpos;
    head = tail;
    text = (*fm).content;
    text_end = ((*fm).content).offset((*fm).length as isize);
    while text < text_end {
        let mut up_error_code: i32 = 0;
        let mut url_text: *mut i8 = 0 as *mut i8;
        let mut new_url: *mut i8 = 0 as *mut i8;
        let mut entry: *mut urlpos = 0 as *mut urlpos;
        let mut url: *mut url = 0 as *mut url;
        let mut line_beg: *const i8 = text;
        let mut line_end: *const i8 = memchr(
            text as *const libc::c_void,
            '\n' as i32,
            text_end.offset_from(text) as i64 as u64,
        ) as *const i8;
        if line_end.is_null() {
            line_end = text_end;
        } else {
            line_end = line_end.offset(1);
            line_end;
        }
        text = line_end;
        while line_beg < line_end && c_isspace(*line_beg as i32) as i32 != 0 {
            line_beg = line_beg.offset(1);
            line_beg;
        }
        while line_end > line_beg
            && c_isspace(*line_end.offset(-(1 as i32 as isize)) as i32) as i32 != 0
        {
            line_end = line_end.offset(-1);
            line_end;
        }
        if line_beg == line_end {
            continue;
        }
        url_text = strdupdelim(line_beg, line_end);
        if !(opt.base_href).is_null() {
            let mut merged: *mut i8 = uri_merge(opt.base_href, url_text);
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut i8;
            url_text = merged;
        }
        new_url = rewrite_shorthand_url(url_text);
        if !new_url.is_null() {
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut i8;
            url_text = new_url;
        }
        url = url_parse(url_text, &mut up_error_code, 0 as *mut iri, 0 as i32 != 0);
        if url.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: Invalid URL %s: %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                file,
                url_text,
                url_error(up_error_code),
            );
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut i8;
            inform_exit_status(uerr_t::URLERROR);
        } else {
            rpl_free(url_text as *mut libc::c_void);
            url_text = 0 as *mut i8;
            entry = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<urlpos>() as u64)
                as *mut urlpos;
            (*entry).url = url;
            if head.is_null() {
                head = entry;
            } else {
                (*tail).next = entry;
            }
            tail = entry;
        }
    }
    wget_read_file_free(fm);
    return head;
}