use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type hash_table;
    pub type robot_specs;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn set_uri_encoding(i: *mut iri, charset: *const i8, force: bool);
    fn iri_free(i: *mut iri);
    fn iri_new() -> *mut iri;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn quote_n(n: i32, arg: *const i8) -> *const i8;
    fn quote(arg: *const i8) -> *const i8;
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn unlink(__name: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn url_escape(_: *const i8) -> *mut i8;
    fn url_unescape(_: *mut i8);
    fn url_parse(
        _: *const i8,
        _: *mut i32,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    fn url_error(_: i32) -> *const i8;
    fn url_free(_: *mut url);
    fn url_string(_: *const url, _: url_auth_mode) -> *mut i8;
    fn schemes_are_similar_p(a: url_scheme, b: url_scheme) -> bool;
    fn subdir_p(_: *const i8, _: *const i8) -> bool;
    fn acceptable(_: *const i8) -> bool;
    fn accept_url(_: *const i8) -> bool;
    fn accdir(s: *const i8) -> bool;
    fn match_tail(_: *const i8, _: *const i8, _: bool) -> bool;
    fn has_html_suffix_p(_: *const i8) -> bool;
    fn string_set_add(_: *mut hash_table, _: *const i8);
    fn string_set_contains(_: *mut hash_table, _: *const i8) -> i32;
    fn string_set_free(_: *mut hash_table);
    static mut total_downloaded_bytes: wgint;
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
    fn accept_domain(_: *mut url) -> bool;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> i32;
    fn make_string_hash_table(_: i32) -> *mut hash_table;
    fn res_parse(_: *const i8, _: i32) -> *mut robot_specs;
    fn res_parse_from_file(_: *const i8) -> *mut robot_specs;
    fn res_match_path(_: *const robot_specs, _: *const i8) -> bool;
    fn res_register_specs(_: *const i8, _: i32, _: *mut robot_specs);
    fn res_get_specs(_: *const i8, _: i32) -> *mut robot_specs;
    fn res_retrieve_file(_: *const i8, _: *mut *mut i8, _: *mut iri) -> bool;
    static mut dl_url_file_map: *mut hash_table;
    static mut downloaded_html_set: *mut hash_table;
    static mut downloaded_css_set: *mut hash_table;
    fn register_delete_file(_: *const i8);
    fn get_urls_html(
        _: *const i8,
        _: *const i8,
        _: *mut bool,
        _: *mut iri,
    ) -> *mut urlpos;
    fn free_urlpos(_: *mut urlpos);
    fn get_urls_css_file(_: *const i8, _: *const i8) -> *mut urlpos;
    fn inform_exit_status(err: uerr_t);
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
pub struct iri {
    pub uri_encoding: *mut i8,
    pub content_encoding: *mut i8,
    pub orig_url: *mut i8,
    pub utf8_encode: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    TEXTHTML = 0x1,
    RETROKF = 0x2,
    HEAD_ONLY = 0x4,
    SEND_NOCACHE = 0x8,
    ACCEPTRANGES = 0x10,
    ADDED_HTML_EXTENSION = 0x20,
    TEXTCSS = 0x40,
    IF_MODIFIED_SINCE = 0x80,
    METALINK_METADATA = 0x100,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::TEXTHTML => 0x1,
            C2RustUnnamed_4::RETROKF => 0x2,
            C2RustUnnamed_4::HEAD_ONLY => 0x4,
            C2RustUnnamed_4::SEND_NOCACHE => 0x8,
            C2RustUnnamed_4::ACCEPTRANGES => 0x10,
            C2RustUnnamed_4::ADDED_HTML_EXTENSION => 0x20,
            C2RustUnnamed_4::TEXTCSS => 0x40,
            C2RustUnnamed_4::IF_MODIFIED_SINCE => 0x80,
            C2RustUnnamed_4::METALINK_METADATA => 0x100,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            0x1 => C2RustUnnamed_4::TEXTHTML,
            0x2 => C2RustUnnamed_4::RETROKF,
            0x4 => C2RustUnnamed_4::HEAD_ONLY,
            0x8 => C2RustUnnamed_4::SEND_NOCACHE,
            0x10 => C2RustUnnamed_4::ACCEPTRANGES,
            0x20 => C2RustUnnamed_4::ADDED_HTML_EXTENSION,
            0x40 => C2RustUnnamed_4::TEXTCSS,
            0x80 => C2RustUnnamed_4::IF_MODIFIED_SINCE,
            0x100 => C2RustUnnamed_4::METALINK_METADATA,
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
pub enum url_auth_mode {
    URL_AUTH_SHOW,
    URL_AUTH_HIDE_PASSWD,
    URL_AUTH_HIDE,
}
impl url_auth_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            url_auth_mode::URL_AUTH_SHOW => 0,
            url_auth_mode::URL_AUTH_HIDE_PASSWD => 1,
            url_auth_mode::URL_AUTH_HIDE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> url_auth_mode {
        match value {
            0 => url_auth_mode::URL_AUTH_SHOW,
            1 => url_auth_mode::URL_AUTH_HIDE_PASSWD,
            2 => url_auth_mode::URL_AUTH_HIDE,
            _ => panic!("Invalid value for url_auth_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for url_auth_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for url_auth_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for url_auth_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for url_auth_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for url_auth_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn add(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn sub(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn mul(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn div(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for url_auth_mode {
    type Output = url_auth_mode;
    fn rem(self, rhs: u32) -> url_auth_mode {
        url_auth_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
pub struct url_queue {
    pub head: *mut queue_element,
    pub tail: *mut queue_element,
    pub count: i32,
    pub maxcount: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queue_element {
    pub url: *const i8,
    pub referer: *const i8,
    pub depth: i32,
    pub html_allowed: bool,
    pub iri: *mut iri,
    pub css_allowed: bool,
    pub next: *mut queue_element,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum reject_reason {
    WG_RR_SUCCESS,
    WG_RR_BLACKLIST,
    WG_RR_NOTHTTPS,
    WG_RR_NONHTTP,
    WG_RR_ABSOLUTE,
    WG_RR_DOMAIN,
    WG_RR_PARENT,
    WG_RR_LIST,
    WG_RR_REGEX,
    WG_RR_RULES,
    WG_RR_SPANNEDHOST,
    WG_RR_ROBOTS,
}
impl reject_reason {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            reject_reason::WG_RR_SUCCESS => 0,
            reject_reason::WG_RR_BLACKLIST => 1,
            reject_reason::WG_RR_NOTHTTPS => 2,
            reject_reason::WG_RR_NONHTTP => 3,
            reject_reason::WG_RR_ABSOLUTE => 4,
            reject_reason::WG_RR_DOMAIN => 5,
            reject_reason::WG_RR_PARENT => 6,
            reject_reason::WG_RR_LIST => 7,
            reject_reason::WG_RR_REGEX => 8,
            reject_reason::WG_RR_RULES => 9,
            reject_reason::WG_RR_SPANNEDHOST => 10,
            reject_reason::WG_RR_ROBOTS => 11,
        }
    }
    fn from_libc_c_uint(value: u32) -> reject_reason {
        match value {
            0 => reject_reason::WG_RR_SUCCESS,
            1 => reject_reason::WG_RR_BLACKLIST,
            2 => reject_reason::WG_RR_NOTHTTPS,
            3 => reject_reason::WG_RR_NONHTTP,
            4 => reject_reason::WG_RR_ABSOLUTE,
            5 => reject_reason::WG_RR_DOMAIN,
            6 => reject_reason::WG_RR_PARENT,
            7 => reject_reason::WG_RR_LIST,
            8 => reject_reason::WG_RR_REGEX,
            9 => reject_reason::WG_RR_RULES,
            10 => reject_reason::WG_RR_SPANNEDHOST,
            11 => reject_reason::WG_RR_ROBOTS,
            _ => panic!("Invalid value for reject_reason: {}", value),
        }
    }
}
impl AddAssign<u32> for reject_reason {
    fn add_assign(&mut self, rhs: u32) {
        *self = reject_reason::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for reject_reason {
    fn sub_assign(&mut self, rhs: u32) {
        *self = reject_reason::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for reject_reason {
    fn mul_assign(&mut self, rhs: u32) {
        *self = reject_reason::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for reject_reason {
    fn div_assign(&mut self, rhs: u32) {
        *self = reject_reason::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for reject_reason {
    fn rem_assign(&mut self, rhs: u32) {
        *self = reject_reason::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for reject_reason {
    type Output = reject_reason;
    fn add(self, rhs: u32) -> reject_reason {
        reject_reason::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for reject_reason {
    type Output = reject_reason;
    fn sub(self, rhs: u32) -> reject_reason {
        reject_reason::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for reject_reason {
    type Output = reject_reason;
    fn mul(self, rhs: u32) -> reject_reason {
        reject_reason::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for reject_reason {
    type Output = reject_reason;
    fn div(self, rhs: u32) -> reject_reason {
        reject_reason::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for reject_reason {
    type Output = reject_reason;
    fn rem(self, rhs: u32) -> reject_reason {
        reject_reason::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
unsafe extern "C" fn url_queue_new() -> *mut url_queue {
    let mut queue: *mut url_queue = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<url_queue>() as u64,
    ) as *mut url_queue;
    return queue;
}
unsafe extern "C" fn url_queue_delete(mut queue: *mut url_queue) {
    rpl_free(queue as *mut libc::c_void);
    queue = 0 as *mut url_queue;
}
unsafe extern "C" fn url_enqueue(
    mut queue: *mut url_queue,
    mut i: *mut iri,
    mut url: *const i8,
    mut referer: *const i8,
    mut depth: i32,
    mut html_allowed: bool,
    mut css_allowed: bool,
) {
    let mut qel: *mut queue_element = xmalloc(
        ::core::mem::size_of::<queue_element>() as u64,
    ) as *mut queue_element;
    (*qel).iri = i;
    (*qel).url = url;
    (*qel).referer = referer;
    (*qel).depth = depth;
    (*qel).html_allowed = html_allowed;
    (*qel).css_allowed = css_allowed;
    (*qel).next = 0 as *mut queue_element;
    (*queue).count += 1;
    (*queue).count;
    if (*queue).count > (*queue).maxcount {
        (*queue).maxcount = (*queue).count;
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Enqueuing %s at depth %d\n\0" as *const u8 as *const i8,
            quotearg_n_style(0 as i32, quoting_style::escape_quoting_style, url),
            depth,
        );
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Queue count %d, maxcount %d.\n\0" as *const u8 as *const i8,
            (*queue).count,
            (*queue).maxcount,
        );
    }
    if !i.is_null() {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"[IRI Enqueuing %s with %s\n\0" as *const u8 as *const i8,
                quote_n(0 as i32, url),
                if !((*i).uri_encoding).is_null() {
                    quote_n(1 as i32, (*i).uri_encoding)
                } else {
                    b"None\0" as *const u8 as *const i8
                },
            );
        }
    }
    if !((*queue).tail).is_null() {
        (*(*queue).tail).next = qel;
    }
    (*queue).tail = qel;
    if ((*queue).head).is_null() {
        (*queue).head = (*queue).tail;
    }
}
unsafe extern "C" fn url_dequeue(
    mut queue: *mut url_queue,
    mut i: *mut *mut iri,
    mut url: *mut *const i8,
    mut referer: *mut *const i8,
    mut depth: *mut i32,
    mut html_allowed: *mut bool,
    mut css_allowed: *mut bool,
) -> bool {
    let mut qel: *mut queue_element = (*queue).head;
    if qel.is_null() {
        return 0 as i32 != 0;
    }
    (*queue).head = (*(*queue).head).next;
    if ((*queue).head).is_null() {
        (*queue).tail = 0 as *mut queue_element;
    }
    *i = (*qel).iri;
    *url = (*qel).url;
    *referer = (*qel).referer;
    *depth = (*qel).depth;
    *html_allowed = (*qel).html_allowed;
    *css_allowed = (*qel).css_allowed;
    (*queue).count -= 1;
    (*queue).count;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Dequeuing %s at depth %d\n\0" as *const u8 as *const i8,
            quotearg_n_style(0 as i32, quoting_style::escape_quoting_style, (*qel).url),
            (*qel).depth,
        );
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Queue count %d, maxcount %d.\n\0" as *const u8 as *const i8,
            (*queue).count,
            (*queue).maxcount,
        );
    }
    rpl_free(qel as *mut libc::c_void);
    qel = 0 as *mut queue_element;
    return 1 as i32 != 0;
}
unsafe extern "C" fn blacklist_add(mut blacklist: *mut hash_table, mut url: *const i8) {
    let mut url_unescaped: *mut i8 = xstrdup(url);
    url_unescape(url_unescaped);
    string_set_add(blacklist, url_unescaped);
    rpl_free(url_unescaped as *mut libc::c_void);
    url_unescaped = 0 as *mut i8;
}
unsafe extern "C" fn blacklist_contains(
    mut blacklist: *mut hash_table,
    mut url: *const i8,
) -> i32 {
    let mut url_unescaped: *mut i8 = xstrdup(url);
    let mut ret: i32 = 0;
    url_unescape(url_unescaped);
    ret = string_set_contains(blacklist, url_unescaped);
    rpl_free(url_unescaped as *mut libc::c_void);
    url_unescaped = 0 as *mut i8;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn retrieve_tree(
    mut start_url_parsed: *mut url,
    mut pi: *mut iri,
) -> uerr_t {
    let mut status: uerr_t = uerr_t::RETROK;
    let mut queue: *mut url_queue = 0 as *mut url_queue;
    let mut blacklist: *mut hash_table = 0 as *mut hash_table;
    let mut i: *mut iri = iri_new();
    let mut rejectedlog: *mut FILE = 0 as *mut FILE;
    if !pi.is_null() {
        (*i).uri_encoding = if !((*pi).uri_encoding).is_null() {
            xstrdup((*pi).uri_encoding)
        } else {
            0 as *mut i8
        };
        (*i).content_encoding = if !((*pi).content_encoding).is_null() {
            xstrdup((*pi).content_encoding)
        } else {
            0 as *mut i8
        };
        (*i).utf8_encode = (*pi).utf8_encode;
    } else {
        set_uri_encoding(i, opt.locale, 1 as i32 != 0);
    }
    queue = url_queue_new();
    blacklist = make_string_hash_table(0 as i32);
    url_enqueue(
        queue,
        i,
        xstrdup((*start_url_parsed).url),
        0 as *const i8,
        0 as i32,
        1 as i32 != 0,
        0 as i32 != 0,
    );
    blacklist_add(blacklist, (*start_url_parsed).url);
    if !(opt.rejected_log).is_null() {
        rejectedlog = rpl_fopen(opt.rejected_log, b"w\0" as *const u8 as *const i8);
        write_reject_log_header(rejectedlog);
        if rejectedlog.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s: %s\n\0" as *const u8 as *const i8,
                opt.rejected_log,
                strerror(*__errno_location()),
            );
        }
    }
    loop {
        let mut descend: bool = 0 as i32 != 0;
        let mut url: *mut i8 = 0 as *mut i8;
        let mut referer: *mut i8 = 0 as *mut i8;
        let mut file: *mut i8 = 0 as *mut i8;
        let mut depth: i32 = 0;
        let mut html_allowed: bool = false;
        let mut css_allowed: bool = false;
        let mut is_css: bool = 0 as i32 != 0;
        let mut dash_p_leaf_HTML: bool = 0 as i32 != 0;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            break;
        }
        if status as u32 == uerr_t::FWRITEERR as i32 as u32 {
            break;
        }
        if !url_dequeue(
            queue,
            &mut i as *mut *mut iri,
            &mut url as *mut *mut i8 as *mut *const i8,
            &mut referer as *mut *mut i8 as *mut *const i8,
            &mut depth,
            &mut html_allowed,
            &mut css_allowed,
        ) {
            break;
        }
        if !dl_url_file_map.is_null()
            && hash_table_contains(dl_url_file_map, url as *const libc::c_void) != 0
        {
            let mut is_css_bool: bool = false;
            file = xstrdup(
                hash_table_get(dl_url_file_map, url as *const libc::c_void) as *const i8,
            );
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Already downloaded \"%s\", reusing it from \"%s\".\n\0"
                        as *const u8 as *const i8,
                    url,
                    file,
                );
            }
            is_css_bool = css_allowed as i32 != 0 && !downloaded_css_set.is_null()
                && string_set_contains(downloaded_css_set, file) != 0;
            if is_css_bool as i32 != 0
                || html_allowed as i32 != 0 && !downloaded_html_set.is_null()
                    && string_set_contains(downloaded_html_set, file) != 0
            {
                descend = 1 as i32 != 0;
                is_css = is_css_bool;
            }
        } else {
            let mut dt: i32 = 0 as i32;
            let mut url_err: i32 = 0;
            let mut redirected: *mut i8 = 0 as *mut i8;
            let mut url_parsed: *mut url = url_parse(
                url,
                &mut url_err,
                i,
                1 as i32 != 0,
            );
            if url_parsed.is_null() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"%s: %s.\n\0" as *const u8 as *const i8,
                    url,
                    url_error(url_err),
                );
                inform_exit_status(uerr_t::URLERROR);
            } else {
                status = retrieve_url(
                    url_parsed,
                    url,
                    &mut file,
                    &mut redirected,
                    referer,
                    &mut dt,
                    0 as i32 != 0,
                    i,
                    1 as i32 != 0,
                );
                if html_allowed as i32 != 0 && !file.is_null()
                    && status as u32 == uerr_t::RETROK as i32 as u32
                    && dt & C2RustUnnamed_4::RETROKF as i32 != 0
                    && dt & C2RustUnnamed_4::TEXTHTML as i32 != 0
                {
                    descend = 1 as i32 != 0;
                    is_css = 0 as i32 != 0;
                }
                if !file.is_null() && status as u32 == uerr_t::RETROK as i32 as u32
                    && dt & C2RustUnnamed_4::RETROKF as i32 != 0
                    && (dt & C2RustUnnamed_4::TEXTCSS as i32 != 0
                        || css_allowed as i32 != 0)
                {
                    descend = 1 as i32 != 0;
                    is_css = 1 as i32 != 0;
                }
                if !redirected.is_null() {
                    if descend {
                        let mut r: reject_reason = descend_redirect(
                            redirected,
                            url_parsed,
                            depth,
                            start_url_parsed,
                            blacklist,
                            i,
                        );
                        if r as u32 == reject_reason::WG_RR_SUCCESS as i32 as u32 {
                            blacklist_add(blacklist, url);
                        } else {
                            write_reject_log_reason(
                                rejectedlog,
                                r,
                                url_parsed,
                                start_url_parsed,
                            );
                            descend = 0 as i32 != 0;
                        }
                    }
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut i8;
                    url = redirected;
                } else {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut i8;
                    url = xstrdup((*url_parsed).url);
                }
                url_free(url_parsed);
            }
        }
        opt.spider;
        if descend as i32 != 0 && depth >= opt.reclevel && opt.reclevel != -(1 as i32) {
            if opt.page_requisites as i32 != 0
                && (depth == opt.reclevel || depth == opt.reclevel + 1 as i32)
            {
                dash_p_leaf_HTML = 1 as i32 != 0;
            } else {
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Not descending further; at depth %d, max. %d.\n\0" as *const u8
                            as *const i8,
                        depth,
                        opt.reclevel,
                    );
                }
                descend = 0 as i32 != 0;
            }
        }
        if descend {
            let mut meta_disallow_follow: bool = 0 as i32 != 0;
            let mut children: *mut urlpos = if is_css as i32 != 0 {
                get_urls_css_file(file, url)
            } else {
                get_urls_html(file, url, &mut meta_disallow_follow, i)
            };
            if opt.use_robots as i32 != 0 && meta_disallow_follow as i32 != 0 {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"nofollow attribute found in %s. Will not follow any links on this page\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    file,
                );
                free_urlpos(children);
                children = 0 as *mut urlpos;
            }
            if !children.is_null() {
                let mut child: *mut urlpos = children;
                let mut url_parsed_0: *mut url = url_parse(
                    url,
                    0 as *mut i32,
                    i,
                    1 as i32 != 0,
                );
                let mut ci: *mut iri = 0 as *mut iri;
                let mut referer_url: *mut i8 = url;
                let mut strip_auth: bool = false;
                if url_parsed_0.is_null() {
                    continue;
                }
                strip_auth = !url_parsed_0.is_null()
                    && !((*url_parsed_0).user).is_null();
                if strip_auth {
                    referer_url = url_string(url_parsed_0, url_auth_mode::URL_AUTH_HIDE);
                }
                while !child.is_null() {
                    let mut r_0: reject_reason = reject_reason::WG_RR_SUCCESS;
                    if (*child).ignore_when_downloading() != 0 {
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"Not following due to 'ignore' flag: %s\n\0" as *const u8
                                    as *const i8,
                                (*(*child).url).url,
                            );
                        }
                    } else if dash_p_leaf_HTML as i32 != 0
                        && (*child).link_inline_p() == 0
                    {
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"Not following due to 'link inline' flag: %s\n\0"
                                    as *const u8 as *const i8,
                                (*(*child).url).url,
                            );
                        }
                    } else {
                        r_0 = download_child(
                            child,
                            url_parsed_0,
                            depth,
                            start_url_parsed,
                            blacklist,
                            i,
                        );
                        if r_0 as u32 == reject_reason::WG_RR_SUCCESS as i32 as u32 {
                            ci = iri_new();
                            set_uri_encoding(ci, (*i).content_encoding, 0 as i32 != 0);
                            url_enqueue(
                                queue,
                                ci,
                                xstrdup((*(*child).url).url),
                                xstrdup(referer_url),
                                depth + 1 as i32,
                                (*child).link_expect_html() != 0,
                                (*child).link_expect_css() != 0,
                            );
                            blacklist_add(blacklist, (*(*child).url).url);
                        } else {
                            write_reject_log_reason(
                                rejectedlog,
                                r_0,
                                (*child).url,
                                url_parsed_0,
                            );
                        }
                    }
                    child = (*child).next;
                }
                if strip_auth {
                    rpl_free(referer_url as *mut libc::c_void);
                    referer_url = 0 as *mut i8;
                }
                url_free(url_parsed_0);
                free_urlpos(children);
            }
        }
        if !file.is_null()
            && (opt.delete_after as i32 != 0 || opt.spider as i32 != 0
                || !acceptable(file))
        {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Removing file due to %s in recursive_retrieve():\n\0" as *const u8
                        as *const i8,
                    if opt.delete_after as i32 != 0 {
                        b"--delete-after\0" as *const u8 as *const i8
                    } else if opt.spider as i32 != 0 {
                        b"--spider\0" as *const u8 as *const i8
                    } else {
                        b"recursive rejection criteria\0" as *const u8 as *const i8
                    },
                );
            }
            logprintf(
                log_options::LOG_VERBOSE,
                if opt.delete_after as i32 != 0 || opt.spider as i32 != 0 {
                    dcgettext(
                        0 as *const i8,
                        b"Removing %s.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    )
                } else {
                    dcgettext(
                        0 as *const i8,
                        b"Removing %s since it should be rejected.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    )
                },
                file,
            );
            if unlink(file) != 0 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"unlink: %s\n\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
            }
            logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
            register_delete_file(file);
        }
        rpl_free(url as *mut libc::c_void);
        url = 0 as *mut i8;
        rpl_free(referer as *mut libc::c_void);
        referer = 0 as *mut i8;
        rpl_free(file as *mut libc::c_void);
        file = 0 as *mut i8;
        iri_free(i);
    }
    if !rejectedlog.is_null() {
        fclose(rejectedlog);
        rejectedlog = 0 as *mut FILE;
    }
    let mut d1: *mut i8 = 0 as *mut i8;
    let mut d2: *mut i8 = 0 as *mut i8;
    let mut d3: i32 = 0;
    let mut d4: bool = false;
    let mut d5: bool = false;
    let mut d6: *mut iri = 0 as *mut iri;
    while url_dequeue(
        queue,
        &mut d6 as *mut *mut iri,
        &mut d1 as *mut *mut i8 as *mut *const i8,
        &mut d2 as *mut *mut i8 as *mut *const i8,
        &mut d3,
        &mut d4,
        &mut d5,
    ) {
        iri_free(d6);
        rpl_free(d1 as *mut libc::c_void);
        d1 = 0 as *mut i8;
        rpl_free(d2 as *mut libc::c_void);
        d2 = 0 as *mut i8;
    }
    url_queue_delete(queue);
    string_set_free(blacklist);
    if opt.quota != 0 && total_downloaded_bytes > opt.quota {
        return uerr_t::QUOTEXC
    } else if status as u32 == uerr_t::FWRITEERR as i32 as u32 {
        return uerr_t::FWRITEERR
    } else {
        return uerr_t::RETROK
    };
}
unsafe extern "C" fn download_child(
    mut upos: *const urlpos,
    mut parent: *mut url,
    mut depth: i32,
    mut start_url_parsed: *mut url,
    mut blacklist: *mut hash_table,
    mut iri: *mut iri,
) -> reject_reason {
    let mut current_block: u64;
    let mut u: *mut url = (*upos).url;
    let mut url: *const i8 = (*u).url;
    let mut u_scheme_like_http: bool = false;
    let mut reason: reject_reason = reject_reason::WG_RR_SUCCESS;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Deciding whether to enqueue \"%s\".\n\0" as *const u8 as *const i8,
            url,
        );
    }
    if blacklist_contains(blacklist, url) != 0 {
        if opt.spider {
            let mut referrer: *mut i8 = url_string(
                parent,
                url_auth_mode::URL_AUTH_HIDE_PASSWD,
            );
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"download_child: parent->url is: %s\n\0" as *const u8 as *const i8,
                    quote((*parent).url),
                );
            }
            rpl_free(referrer as *mut libc::c_void);
            referrer = 0 as *mut i8;
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(b"Already on the black list.\n\0" as *const u8 as *const i8);
        }
        reason = reject_reason::WG_RR_BLACKLIST;
    } else if opt.https_only as i32 != 0
        && (*u).scheme as u32 != url_scheme::SCHEME_HTTPS as i32 as u32
    {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Not following non-HTTPS links.\n\0" as *const u8 as *const i8,
            );
        }
        reason = reject_reason::WG_RR_NOTHTTPS;
    } else {
        u_scheme_like_http = schemes_are_similar_p((*u).scheme, url_scheme::SCHEME_HTTP);
        if !u_scheme_like_http
            && !(((*u).scheme as u32 == url_scheme::SCHEME_FTP as i32 as u32
                || (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32)
                && opt.follow_ftp as i32 != 0)
        {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Not following non-HTTP schemes.\n\0" as *const u8 as *const i8,
                );
            }
            reason = reject_reason::WG_RR_NONHTTP;
        } else {
            if u_scheme_like_http {
                if opt.relative_only as i32 != 0 && (*upos).link_relative_p() == 0 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"It doesn't really look like a relative link.\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    reason = reject_reason::WG_RR_ABSOLUTE;
                    current_block = 13264993963312919656;
                } else {
                    current_block = 2543120759711851213;
                }
            } else {
                current_block = 2543120759711851213;
            }
            match current_block {
                13264993963312919656 => {}
                _ => {
                    if !accept_domain(u) {
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"The domain was not accepted.\n\0" as *const u8
                                    as *const i8,
                            );
                        }
                        reason = reject_reason::WG_RR_DOMAIN;
                    } else {
                        if opt.no_parent as i32 != 0
                            && schemes_are_similar_p(
                                (*u).scheme,
                                (*start_url_parsed).scheme,
                            ) as i32 != 0
                            && 0 as i32
                                == strcasecmp((*u).host, (*start_url_parsed).host)
                            && ((*u).scheme as u32 != (*start_url_parsed).scheme as u32
                                || (*u).port == (*start_url_parsed).port)
                            && !(opt.page_requisites as i32 != 0
                                && (*upos).link_inline_p() as i32 != 0)
                        {
                            if !subdir_p((*start_url_parsed).dir, (*u).dir) {
                                if opt.debug as i64 != 0 {
                                    debug_logprintf(
                                        b"Going to \"%s\" would escape \"%s\" with no_parent on.\n\0"
                                            as *const u8 as *const i8,
                                        (*u).dir,
                                        (*start_url_parsed).dir,
                                    );
                                }
                                reason = reject_reason::WG_RR_PARENT;
                                current_block = 13264993963312919656;
                            } else {
                                current_block = 5235537862154438448;
                            }
                        } else {
                            current_block = 5235537862154438448;
                        }
                        match current_block {
                            13264993963312919656 => {}
                            _ => {
                                if !(opt.includes).is_null() || !(opt.excludes).is_null() {
                                    if !accdir((*u).dir) {
                                        if opt.debug as i64 != 0 {
                                            debug_logprintf(
                                                b"%s (%s) is excluded/not-included.\n\0" as *const u8
                                                    as *const i8,
                                                url,
                                                (*u).dir,
                                            );
                                        }
                                        reason = reject_reason::WG_RR_LIST;
                                        current_block = 13264993963312919656;
                                    } else {
                                        current_block = 307447392441238883;
                                    }
                                } else {
                                    current_block = 307447392441238883;
                                }
                                match current_block {
                                    13264993963312919656 => {}
                                    _ => {
                                        if !accept_url(url) {
                                            if opt.debug as i64 != 0 {
                                                debug_logprintf(
                                                    b"%s is excluded/not-included through regex.\n\0"
                                                        as *const u8 as *const i8,
                                                    url,
                                                );
                                            }
                                            reason = reject_reason::WG_RR_REGEX;
                                        } else {
                                            if *((*u).file).offset(0 as i32 as isize) as i32
                                                != '\0' as i32
                                                && !(has_html_suffix_p((*u).file) as i32 != 0
                                                    && (opt.reclevel == -(1 as i32)
                                                        || depth < opt.reclevel - 1 as i32
                                                        || opt.page_requisites as i32 != 0))
                                            {
                                                if !acceptable((*u).file) {
                                                    if opt.debug as i64 != 0 {
                                                        debug_logprintf(
                                                            b"%s (%s) does not match acc/rej rules.\n\0" as *const u8
                                                                as *const i8,
                                                            url,
                                                            (*u).file,
                                                        );
                                                    }
                                                    reason = reject_reason::WG_RR_RULES;
                                                    current_block = 13264993963312919656;
                                                } else {
                                                    current_block = 851619935621435220;
                                                }
                                            } else {
                                                current_block = 851619935621435220;
                                            }
                                            match current_block {
                                                13264993963312919656 => {}
                                                _ => {
                                                    if schemes_are_similar_p((*u).scheme, (*parent).scheme) {
                                                        if !opt.spanhost
                                                            && 0 as i32 != strcasecmp((*parent).host, (*u).host)
                                                        {
                                                            if opt.debug as i64 != 0 {
                                                                debug_logprintf(
                                                                    b"This is not the same hostname as the parent's (%s and %s).\n\0"
                                                                        as *const u8 as *const i8,
                                                                    (*u).host,
                                                                    (*parent).host,
                                                                );
                                                            }
                                                            reason = reject_reason::WG_RR_SPANNEDHOST;
                                                            current_block = 13264993963312919656;
                                                        } else {
                                                            current_block = 9199578309995299736;
                                                        }
                                                    } else {
                                                        current_block = 9199578309995299736;
                                                    }
                                                    match current_block {
                                                        13264993963312919656 => {}
                                                        _ => {
                                                            if opt.use_robots as i32 != 0
                                                                && u_scheme_like_http as i32 != 0
                                                            {
                                                                let mut specs: *mut robot_specs = res_get_specs(
                                                                    (*u).host,
                                                                    (*u).port,
                                                                );
                                                                if specs.is_null() {
                                                                    let mut rfile: *mut i8 = 0 as *mut i8;
                                                                    if res_retrieve_file(url, &mut rfile, iri) {
                                                                        specs = res_parse_from_file(rfile);
                                                                        if opt.delete_after as i32 != 0 || opt.spider as i32 != 0
                                                                            || match_tail(
                                                                                rfile,
                                                                                b".tmp\0" as *const u8 as *const i8,
                                                                                0 as i32 != 0,
                                                                            ) as i32 != 0
                                                                        {
                                                                            logprintf(
                                                                                log_options::LOG_VERBOSE,
                                                                                dcgettext(
                                                                                    0 as *const i8,
                                                                                    b"Removing %s.\n\0" as *const u8 as *const i8,
                                                                                    5 as i32,
                                                                                ),
                                                                                rfile,
                                                                            );
                                                                            if unlink(rfile) != 0 {
                                                                                logprintf(
                                                                                    log_options::LOG_NOTQUIET,
                                                                                    b"unlink: %s\n\0" as *const u8 as *const i8,
                                                                                    strerror(*__errno_location()),
                                                                                );
                                                                            }
                                                                        }
                                                                        rpl_free(rfile as *mut libc::c_void);
                                                                        rfile = 0 as *mut i8;
                                                                    } else {
                                                                        specs = res_parse(
                                                                            b"\0" as *const u8 as *const i8,
                                                                            0 as i32,
                                                                        );
                                                                    }
                                                                    res_register_specs((*u).host, (*u).port, specs);
                                                                }
                                                                if !res_match_path(specs, (*u).path) {
                                                                    if opt.debug as i64 != 0 {
                                                                        debug_logprintf(
                                                                            b"Not following %s because robots.txt forbids it.\n\0"
                                                                                as *const u8 as *const i8,
                                                                            url,
                                                                        );
                                                                    }
                                                                    blacklist_add(blacklist, url);
                                                                    reason = reject_reason::WG_RR_ROBOTS;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if reason as u32 == reject_reason::WG_RR_SUCCESS as i32 as u32 {
        if opt.debug as i64 != 0 {
            debug_logprintf(b"Decided to load it.\n\0" as *const u8 as *const i8);
        }
    } else if opt.debug as i64 != 0 {
        debug_logprintf(b"Decided NOT to load it.\n\0" as *const u8 as *const i8);
    }
    return reason;
}
unsafe extern "C" fn descend_redirect(
    mut redirected: *const i8,
    mut orig_parsed: *mut url,
    mut depth: i32,
    mut start_url_parsed: *mut url,
    mut blacklist: *mut hash_table,
    mut iri: *mut iri,
) -> reject_reason {
    let mut new_parsed: *mut url = 0 as *mut url;
    let mut upos: *mut urlpos = 0 as *mut urlpos;
    let mut reason: reject_reason = reject_reason::WG_RR_SUCCESS;
    new_parsed = url_parse(redirected, 0 as *mut i32, 0 as *mut iri, 0 as i32 != 0);
    upos = xcalloc(1 as i32 as size_t, ::core::mem::size_of::<urlpos>() as u64)
        as *mut urlpos;
    (*upos).url = new_parsed;
    reason = download_child(upos, orig_parsed, depth, start_url_parsed, blacklist, iri);
    if reason as u32 == reject_reason::WG_RR_SUCCESS as i32 as u32 {
        blacklist_add(blacklist, (*(*upos).url).url);
    } else if reason as u32 == reject_reason::WG_RR_LIST as i32 as u32
        || reason as u32 == reject_reason::WG_RR_REGEX as i32 as u32
    {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Ignoring decision for redirects, decided to load it.\n\0" as *const u8
                    as *const i8,
            );
        }
        blacklist_add(blacklist, (*(*upos).url).url);
        reason = reject_reason::WG_RR_SUCCESS;
    } else if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Redirection \"%s\" failed the test.\n\0" as *const u8 as *const i8,
            redirected,
        );
    }
    url_free(new_parsed);
    rpl_free(upos as *mut libc::c_void);
    upos = 0 as *mut urlpos;
    return reason;
}
unsafe extern "C" fn write_reject_log_header(mut f: *mut FILE) {
    if f.is_null() {
        return;
    }
    fprintf(
        f,
        b"REASON\tU_URL\tU_SCHEME\tU_HOST\tU_PORT\tU_PATH\tU_PARAMS\tU_QUERY\tU_FRAGMENT\tP_URL\tP_SCHEME\tP_HOST\tP_PORT\tP_PATH\tP_PARAMS\tP_QUERY\tP_FRAGMENT\n\0"
            as *const u8 as *const i8,
    );
}
unsafe extern "C" fn write_reject_log_url(mut fp: *mut FILE, mut url: *const url) {
    let mut escaped_str: *const i8 = 0 as *const i8;
    let mut scheme_str: *const i8 = 0 as *const i8;
    if fp.is_null() {
        return;
    }
    escaped_str = url_escape((*url).url);
    match (*url).scheme as u32 {
        0 => {
            scheme_str = b"url_scheme::SCHEME_HTTP\0" as *const u8 as *const i8;
        }
        1 => {
            scheme_str = b"url_scheme::SCHEME_HTTPS\0" as *const u8 as *const i8;
        }
        3 => {
            scheme_str = b"url_scheme::SCHEME_FTPS\0" as *const u8 as *const i8;
        }
        2 => {
            scheme_str = b"url_scheme::SCHEME_FTP\0" as *const u8 as *const i8;
        }
        _ => {
            scheme_str = b"url_scheme::SCHEME_INVALID\0" as *const u8 as *const i8;
        }
    }
    fprintf(
        fp,
        b"%s\t%s\t%s\t%i\t%s\t%s\t%s\t%s\0" as *const u8 as *const i8,
        escaped_str,
        scheme_str,
        (*url).host,
        (*url).port,
        (*url).path,
        if !((*url).params).is_null() {
            (*url).params
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !((*url).query).is_null() {
            (*url).query
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !((*url).fragment).is_null() {
            (*url).fragment
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    rpl_free(escaped_str as *mut libc::c_void);
    escaped_str = 0 as *const i8;
}
unsafe extern "C" fn write_reject_log_reason(
    mut fp: *mut FILE,
    mut reason: reject_reason,
    mut url: *const url,
    mut parent: *const url,
) {
    let mut reason_str: *const i8 = 0 as *const i8;
    if fp.is_null() {
        return;
    }
    match reason as u32 {
        0 => {
            reason_str = b"SUCCESS\0" as *const u8 as *const i8;
        }
        1 => {
            reason_str = b"BLACKLIST\0" as *const u8 as *const i8;
        }
        2 => {
            reason_str = b"NOTHTTPS\0" as *const u8 as *const i8;
        }
        3 => {
            reason_str = b"NONHTTP\0" as *const u8 as *const i8;
        }
        4 => {
            reason_str = b"ABSOLUTE\0" as *const u8 as *const i8;
        }
        5 => {
            reason_str = b"DOMAIN\0" as *const u8 as *const i8;
        }
        6 => {
            reason_str = b"PARENT\0" as *const u8 as *const i8;
        }
        7 => {
            reason_str = b"LIST\0" as *const u8 as *const i8;
        }
        8 => {
            reason_str = b"REGEX\0" as *const u8 as *const i8;
        }
        9 => {
            reason_str = b"RULES\0" as *const u8 as *const i8;
        }
        10 => {
            reason_str = b"SPANNEDHOST\0" as *const u8 as *const i8;
        }
        11 => {
            reason_str = b"ROBOTS\0" as *const u8 as *const i8;
        }
        _ => {
            reason_str = b"UNKNOWN\0" as *const u8 as *const i8;
        }
    }
    fprintf(fp, b"%s\t\0" as *const u8 as *const i8, reason_str);
    write_reject_log_url(fp, url);
    fprintf(fp, b"\t\0" as *const u8 as *const i8);
    write_reject_log_url(fp, parent);
    fprintf(fp, b"\n\0" as *const u8 as *const i8);
}