use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type hash_table;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn quote(arg: *const i8) -> *const i8;
    fn iri_new() -> *mut iri;
    fn iri_free(i: *mut iri);
    fn set_uri_encoding(i: *mut iri, charset: *const i8, force: bool);
    fn __errno_location() -> *mut i32;
    fn strdupdelim(_: *const i8, _: *const i8) -> *mut i8;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn wget_read_file(_: *const i8) -> *mut file_memory;
    fn wget_read_file_free(_: *mut file_memory);
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
    fn make_nocase_string_hash_table(_: i32) -> *mut hash_table;
    fn url_parse(
        _: *const i8,
        _: *mut i32,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: i32) -> *const i8;
    fn url_free(_: *mut url);
    fn uri_merge(_: *const i8, _: *const i8) -> *mut i8;
    fn are_urls_equal(u1: *const i8, u2: *const i8) -> bool;
    fn retrieve_url(
        _: *mut url,
        _: *const i8,
        _: *mut *mut i8,
        _: *mut *mut i8,
        _: *const i8,
        _: *mut i32,
        _: bool,
        _: *mut iri,
        _: bool,
    ) -> uerr_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_memory {
    pub content: *mut i8,
    pub length: i64,
    pub mmap_p: i32,
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
pub struct robot_specs {
    pub count: i32,
    pub size: i32,
    pub paths: *mut path_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_info {
    pub path: *mut i8,
    pub allowedp: bool,
    pub user_agent_exact_p: bool,
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
unsafe extern "C" fn c_isspace(mut c: i32) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
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
unsafe extern "C" fn _unhex(mut c: u8) -> u8 {
    return (if c as i32 <= '9' as i32 {
        c as i32 - '0' as i32
    } else if c as i32 <= 'F' as i32 {
        c as i32 - 'A' as i32 + 10 as i32
    } else {
        c as i32 - 'a' as i32 + 10 as i32
    }) as u8;
}
unsafe extern "C" fn match_user_agent(
    mut agent: *const i8,
    mut length: i32,
    mut matches_0: *mut bool,
    mut exact_match: *mut bool,
) {
    if length == 1 as i32 && *agent as i32 == '*' as i32 {
        *matches_0 = 1 as i32 != 0;
        *exact_match = 0 as i32 != 0;
    } else if agent.offset(length as isize).offset_from(agent) as i64 as u64
        == (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
        && c_strncasecmp(
            agent,
            b"wget\0" as *const u8 as *const i8,
            (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0
    {
        *matches_0 = 1 as i32 != 0;
        *exact_match = 1 as i32 != 0;
    } else {
        *matches_0 = 0 as i32 != 0;
        *exact_match = 0 as i32 != 0;
    };
}
unsafe extern "C" fn add_path(
    mut specs: *mut robot_specs,
    mut path_b: *const i8,
    mut path_e: *const i8,
    mut allowedp: bool,
    mut exactp: bool,
) {
    let mut pp: path_info = path_info {
        path: 0 as *mut i8,
        allowedp: false,
        user_agent_exact_p: false,
    };
    if path_b < path_e && *path_b as i32 == '/' as i32 {
        path_b = path_b.offset(1);
        path_b;
    }
    pp.path = strdupdelim(path_b, path_e);
    pp.allowedp = allowedp;
    pp.user_agent_exact_p = exactp;
    (*specs).count += 1;
    (*specs).count;
    if (*specs).count > (*specs).size {
        if (*specs).size == 0 as i32 {
            (*specs).size = 1 as i32;
        } else {
            (*specs).size <<= 1 as i32;
        }
        (*specs).paths = xrealloc(
            (*specs).paths as *mut libc::c_void,
            ((*specs).size as u64)
                .wrapping_mul(::core::mem::size_of::<path_info>() as u64),
        ) as *mut path_info;
    }
    *((*specs).paths).offset(((*specs).count - 1 as i32) as isize) = pp;
}
unsafe extern "C" fn prune_non_exact(mut specs: *mut robot_specs) {
    let mut newpaths: *mut path_info = 0 as *mut path_info;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cnt: i32 = 0;
    cnt = 0 as i32;
    i = 0 as i32;
    while i < (*specs).count {
        if (*((*specs).paths).offset(i as isize)).user_agent_exact_p {
            cnt += 1;
            cnt;
        }
        i += 1;
        i;
    }
    newpaths = xmalloc(
        (cnt as u64).wrapping_mul(::core::mem::size_of::<path_info>() as u64),
    ) as *mut path_info;
    i = 0 as i32;
    j = 0 as i32;
    while i < (*specs).count {
        if (*((*specs).paths).offset(i as isize)).user_agent_exact_p {
            let fresh0 = j;
            j = j + 1;
            *newpaths.offset(fresh0 as isize) = *((*specs).paths).offset(i as isize);
        } else {
            rpl_free((*((*specs).paths).offset(i as isize)).path as *mut libc::c_void);
            let ref mut fresh1 = (*((*specs).paths).offset(i as isize)).path;
            *fresh1 = 0 as *mut i8;
        }
        i += 1;
        i;
    }
    rpl_free((*specs).paths as *mut libc::c_void);
    (*specs).paths = 0 as *mut path_info;
    (*specs).paths = newpaths;
    (*specs).count = cnt;
    (*specs).size = cnt;
}
#[no_mangle]
pub unsafe extern "C" fn res_parse(
    mut source: *const i8,
    mut length: i32,
) -> *mut robot_specs {
    let mut line_count: i32 = 1 as i32;
    let mut p: *const i8 = source;
    let mut end: *const i8 = source.offset(length as isize);
    let mut user_agent_applies: bool = 0 as i32 != 0;
    let mut user_agent_exact: bool = 0 as i32 != 0;
    let mut found_exact: bool = 0 as i32 != 0;
    let mut record_count: i32 = 0 as i32;
    let mut specs: *mut robot_specs = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<robot_specs>() as u64,
    ) as *mut robot_specs;
    loop {
        let mut lineend: *const i8 = 0 as *const i8;
        let mut lineend_real: *const i8 = 0 as *const i8;
        let mut field_b: *const i8 = 0 as *const i8;
        let mut field_e: *const i8 = 0 as *const i8;
        let mut value_b: *const i8 = 0 as *const i8;
        let mut value_e: *const i8 = 0 as *const i8;
        if p == end {
            break;
        }
        lineend_real = memchr(
            p as *const libc::c_void,
            '\n' as i32,
            end.offset_from(p) as i64 as u64,
        ) as *const i8;
        if !lineend_real.is_null() {
            lineend_real = lineend_real.offset(1);
            lineend_real;
        } else {
            lineend_real = end;
        }
        lineend = lineend_real;
        while !(p >= lineend) && c_isspace(*p as i32) as i32 != 0 {
            p = p.offset(1);
            p;
        }
        if !(p >= lineend || *p as i32 == '#' as i32) {
            lineend = p;
            while lineend < lineend_real {
                if (lineend == p
                    || c_isspace(*lineend.offset(-(1 as i32 as isize)) as i32) as i32
                        != 0) && *lineend as i32 == '#' as i32
                {
                    break;
                }
                lineend = lineend.offset(1);
                lineend;
            }
            while lineend > p
                && c_isspace(*lineend.offset(-(1 as i32 as isize)) as i32) as i32 != 0
            {
                lineend = lineend.offset(-1);
                lineend;
            }
            field_b = p;
            while !(p >= lineend)
                && (c_isalnum(*p as i32) as i32 != 0 || *p as i32 == '-' as i32)
            {
                p = p.offset(1);
                p;
            }
            field_e = p;
            while !(p >= lineend) && c_isspace(*p as i32) as i32 != 0 {
                p = p.offset(1);
                p;
            }
            if field_b == field_e || p >= lineend || *p as i32 != ':' as i32 {
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Ignoring malformed line %d\n\0" as *const u8 as *const i8,
                        line_count,
                    );
                }
            } else {
                p = p.offset(1);
                p;
                while !(p >= lineend) && c_isspace(*p as i32) as i32 != 0 {
                    p = p.offset(1);
                    p;
                }
                value_b = p;
                while !(p >= lineend) {
                    p = p.offset(1);
                    p;
                }
                value_e = p;
                if field_e.offset_from(field_b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 11]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        field_b,
                        b"user-agent\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 11]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    if record_count != 0 as i32 || user_agent_applies as i32 == 0 as i32
                    {
                        match_user_agent(
                            value_b,
                            value_e.offset_from(value_b) as i64 as i32,
                            &mut user_agent_applies,
                            &mut user_agent_exact,
                        );
                    }
                    if user_agent_exact {
                        found_exact = 1 as i32 != 0;
                    }
                    record_count = 0 as i32;
                } else if field_e.offset_from(field_b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        field_b,
                        b"allow\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    if user_agent_applies {
                        add_path(
                            specs,
                            value_b,
                            value_e,
                            1 as i32 != 0,
                            user_agent_exact,
                        );
                    }
                    record_count += 1;
                    record_count;
                } else if field_e.offset_from(field_b) as i64 as u64
                    == (::core::mem::size_of::<[i8; 9]>() as u64)
                        .wrapping_sub(1 as i32 as u64)
                    && c_strncasecmp(
                        field_b,
                        b"disallow\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 9]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                {
                    if user_agent_applies {
                        let mut allowed: bool = 0 as i32 != 0;
                        if value_b == value_e {
                            allowed = 1 as i32 != 0;
                        }
                        add_path(specs, value_b, value_e, allowed, user_agent_exact);
                    }
                    record_count += 1;
                    record_count;
                } else if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Ignoring unknown field at line %d\n\0" as *const u8
                            as *const i8,
                        line_count,
                    );
                }
            }
        }
        p = lineend_real;
        line_count += 1;
        line_count;
    }
    if found_exact {
        prune_non_exact(specs);
    } else if (*specs).size > (*specs).count {
        (*specs).paths = xrealloc(
            (*specs).paths as *mut libc::c_void,
            ((*specs).count as u64)
                .wrapping_mul(::core::mem::size_of::<path_info>() as u64),
        ) as *mut path_info;
        (*specs).size = (*specs).count;
    }
    return specs;
}
#[no_mangle]
pub unsafe extern "C" fn res_parse_from_file(
    mut filename: *const i8,
) -> *mut robot_specs {
    let mut specs: *mut robot_specs = 0 as *mut robot_specs;
    let mut fm: *mut file_memory = wget_read_file(filename);
    if fm.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Cannot open %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename,
            strerror(*__errno_location()),
        );
        return 0 as *mut robot_specs;
    }
    specs = res_parse((*fm).content, (*fm).length as i32);
    wget_read_file_free(fm);
    return specs;
}
unsafe extern "C" fn free_specs(mut specs: *mut robot_specs) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*specs).count {
        rpl_free((*((*specs).paths).offset(i as isize)).path as *mut libc::c_void);
        let ref mut fresh2 = (*((*specs).paths).offset(i as isize)).path;
        *fresh2 = 0 as *mut i8;
        i += 1;
        i;
    }
    rpl_free((*specs).paths as *mut libc::c_void);
    (*specs).paths = 0 as *mut path_info;
    rpl_free(specs as *mut libc::c_void);
    specs = 0 as *mut robot_specs;
}
unsafe extern "C" fn matches(
    mut record_path: *const i8,
    mut url_path: *const i8,
) -> bool {
    let mut rp: *const i8 = record_path;
    let mut up: *const i8 = url_path;
    loop {
        let mut rc: i8 = *rp;
        let mut uc: i8 = *up;
        if rc == 0 {
            return 1 as i32 != 0;
        }
        if uc == 0 {
            return 0 as i32 != 0;
        }
        if rc as i32 == '%' as i32
            && c_isxdigit(*rp.offset(1 as i32 as isize) as i32) as i32 != 0
            && c_isxdigit(*rp.offset(2 as i32 as isize) as i32) as i32 != 0
        {
            let mut decoded: u8 = (((_unhex(*rp.offset(1 as i32 as isize) as u8) as i32)
                << 4 as i32) + _unhex(*rp.offset(2 as i32 as isize) as u8) as i32) as u8;
            if decoded as i32 != '/' as i32 {
                rc = decoded as i8;
                rp = rp.offset(2 as i32 as isize);
            }
        }
        if uc as i32 == '%' as i32
            && c_isxdigit(*up.offset(1 as i32 as isize) as i32) as i32 != 0
            && c_isxdigit(*up.offset(2 as i32 as isize) as i32) as i32 != 0
        {
            let mut decoded_0: u8 = (((_unhex(*up.offset(1 as i32 as isize) as u8)
                as i32) << 4 as i32)
                + _unhex(*up.offset(2 as i32 as isize) as u8) as i32) as u8;
            if decoded_0 as i32 != '/' as i32 {
                uc = decoded_0 as i8;
                up = up.offset(2 as i32 as isize);
            }
        }
        if rc as i32 != uc as i32 {
            return 0 as i32 != 0;
        }
        rp = rp.offset(1);
        rp;
        up = up.offset(1);
        up;
    };
}
#[no_mangle]
pub unsafe extern "C" fn res_match_path(
    mut specs: *const robot_specs,
    mut path: *const i8,
) -> bool {
    let mut i: i32 = 0;
    if specs.is_null() {
        return 1 as i32 != 0;
    }
    i = 0 as i32;
    while i < (*specs).count {
        if matches((*((*specs).paths).offset(i as isize)).path, path) {
            let mut allowedp: bool = (*((*specs).paths).offset(i as isize)).allowedp;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"%s path %s because of rule %s.\n\0" as *const u8 as *const i8,
                    if allowedp as i32 != 0 {
                        b"Allowing\0" as *const u8 as *const i8
                    } else {
                        b"Rejecting\0" as *const u8 as *const i8
                    },
                    path,
                    quote((*((*specs).paths).offset(i as isize)).path),
                );
            }
            return allowedp;
        }
        i += 1;
        i;
    }
    return 1 as i32 != 0;
}
static mut registered_specs: *mut hash_table = 0 as *const hash_table as *mut hash_table;
#[no_mangle]
pub unsafe extern "C" fn res_register_specs(
    mut host: *const i8,
    mut port: i32,
    mut specs: *mut robot_specs,
) {
    let mut old: *mut robot_specs = 0 as *mut robot_specs;
    let mut buf: [i8; 256] = [0; 256];
    let mut hp: *mut i8 = 0 as *mut i8;
    let mut hp_old: *mut i8 = 0 as *mut i8;
    if snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 256]>() as u64,
        b"%s:%d\0" as *const u8 as *const i8,
        host,
        port,
    ) as u32 as u64 >= ::core::mem::size_of::<[i8; 256]>() as u64
    {
        hp = aprintf(b"%s:%d\0" as *const u8 as *const i8, host, port);
    } else {
        hp = buf.as_mut_ptr();
    }
    if registered_specs.is_null() {
        registered_specs = make_nocase_string_hash_table(0 as i32);
    }
    if hash_table_get_pair(
        registered_specs,
        hp as *const libc::c_void,
        &mut hp_old as *mut *mut i8 as *mut libc::c_void,
        &mut old as *mut *mut robot_specs as *mut libc::c_void,
    ) != 0
    {
        if hp != buf.as_mut_ptr() {
            rpl_free(hp as *mut libc::c_void);
            hp = 0 as *mut i8;
        }
        if !old.is_null() {
            free_specs(old);
        }
        hash_table_put(
            registered_specs,
            hp_old as *const libc::c_void,
            specs as *const libc::c_void,
        );
    } else {
        hash_table_put(
            registered_specs,
            (if hp == buf.as_mut_ptr() { xstrdup(hp) } else { hp })
                as *const libc::c_void,
            specs as *const libc::c_void,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn res_get_specs(
    mut host: *const i8,
    mut port: i32,
) -> *mut robot_specs {
    let mut buf: [i8; 256] = [0; 256];
    let mut hp: *mut i8 = 0 as *mut i8;
    if registered_specs.is_null() {
        return 0 as *mut robot_specs;
    }
    if snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 256]>() as u64,
        b"%s:%d\0" as *const u8 as *const i8,
        host,
        port,
    ) as u32 as u64 >= ::core::mem::size_of::<[i8; 256]>() as u64
    {
        hp = aprintf(b"%s:%d\0" as *const u8 as *const i8, host, port);
    } else {
        hp = buf.as_mut_ptr();
    }
    return hash_table_get(registered_specs, hp as *const libc::c_void)
        as *mut robot_specs;
}
#[no_mangle]
pub unsafe extern "C" fn res_retrieve_file(
    mut url: *const i8,
    mut file: *mut *mut i8,
    mut iri: *mut iri,
) -> bool {
    let mut i: *mut iri = iri_new();
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut robots_url: *mut i8 = uri_merge(
        url,
        b"/robots.txt\0" as *const u8 as *const i8,
    );
    let mut saved_ts_val: i32 = opt.timestamping as i32;
    let mut saved_sp_val: i32 = opt.spider as i32;
    let mut url_err: i32 = 0;
    let mut url_parsed: *mut url = 0 as *mut url;
    set_uri_encoding(i, (*iri).uri_encoding, 0 as i32 != 0);
    (*i).utf8_encode = 0 as i32 != 0;
    logputs(
        log_options::LOG_VERBOSE,
        dcgettext(
            0 as *const i8,
            b"Loading robots.txt; please ignore errors.\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    *file = 0 as *mut i8;
    opt.timestamping = 0 as i32 != 0;
    opt.spider = 0 as i32 != 0;
    url_parsed = url_parse(robots_url, &mut url_err, i, 1 as i32 != 0);
    if url_parsed.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"%s: %s.\n\0" as *const u8 as *const i8,
            robots_url,
            url_error(url_err),
        );
        err = uerr_t::URLERROR;
    } else {
        err = retrieve_url(
            url_parsed,
            robots_url,
            file,
            0 as *mut *mut i8,
            0 as *const i8,
            0 as *mut i32,
            0 as i32 != 0,
            i,
            0 as i32 != 0,
        );
        url_free(url_parsed);
    }
    opt.timestamping = saved_ts_val != 0;
    opt.spider = saved_sp_val != 0;
    rpl_free(robots_url as *mut libc::c_void);
    robots_url = 0 as *mut i8;
    iri_free(i);
    if err as u32 != uerr_t::RETROK as i32 as u32 && !(*file).is_null() {
        rpl_free(*file as *mut libc::c_void);
        *file = 0 as *mut i8;
    }
    return err as u32 == uerr_t::RETROK as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn is_robots_txt_url(mut url: *const i8) -> bool {
    let mut robots_url: *mut i8 = uri_merge(
        url,
        b"/robots.txt\0" as *const u8 as *const i8,
    );
    let mut ret: bool = are_urls_equal(url, robots_url);
    rpl_free(robots_url as *mut libc::c_void);
    robots_url = 0 as *mut i8;
    return ret;
}