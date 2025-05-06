#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type hash_table;
    pub type hsts_store;
    pub type cookie_jar;
    pub type address_list;
    fn time(__timer: *mut time_t) -> time_t;
    fn strptime(__s: *const i8, __fmt: *const i8, __tp: *mut tm) -> *mut i8;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn rpl_timegm(__tm: *mut tm) -> time_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn debug_logprintf(_: *const i8, _: ...);
    static mut exec_name: *const i8;
    fn set_content_encoding(i: *mut iri, charset: *const i8);
    fn parse_charset(str: *const i8) -> *mut i8;
    fn quotearg_style(s: quoting_style, arg: *const i8) -> *mut i8;
    fn quote(arg: *const i8) -> *const i8;
    fn escnonprint_uri(_: *const i8) -> *const i8;
    fn rpl_strtol(string: *const i8, endptr: *mut *mut i8, base: i32) -> i64;
    fn rpl_strtoll(
        string: *const i8,
        endptr: *mut *mut i8,
        base: i32,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn logputs(_: log_options, _: *const i8);
    fn fclose(__stream: *mut FILE) -> i32;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn feof(__stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn abort() -> !;
    fn unlink(__name: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn make_nocase_string_hash_table(_: i32) -> *mut hash_table;
    fn url_unescape(_: *mut i8);
    fn hsts_store_entry(
        _: hsts_store_t,
        _: url_scheme,
        _: *const i8,
        _: i32,
        _: int64_t,
        _: bool,
    ) -> bool;
    fn url_full_path(_: *const url) -> *mut i8;
    fn scheme_default_port(_: url_scheme) -> i32;
    fn scheme_disable(_: url_scheme);
    fn url_string(_: *const url, _: url_auth_mode) -> *mut i8;
    fn url_file_name(_: *const url, _: *mut i8) -> *mut i8;
    fn mkalldirs(_: *const i8) -> i32;
    fn datetime_str(_: time_t) -> *mut i8;
    fn strdupdelim(_: *const i8, _: *const i8) -> *mut i8;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn concat_strings(_: *const i8, _: ...) -> *mut i8;
    fn touch(_: *const i8, _: time_t);
    fn file_exists_p(_: *const i8, _: *mut file_stats_t) -> bool;
    fn file_size(_: *const i8) -> wgint;
    fn unique_name_passthrough(_: *const i8) -> *mut i8;
    fn fopen_excl(_: *const i8, _: i32) -> *mut FILE;
    fn acceptable(_: *const i8) -> bool;
    fn has_wildcards_p(_: *const i8) -> bool;
    fn has_html_suffix_p(_: *const i8) -> bool;
    fn human_readable(_: wgint, _: i32, _: i32) -> *mut i8;
    fn number_to_static_string(_: wgint) -> *mut i8;
    fn random_number(_: i32) -> i32;
    fn wget_base64_encode(_: *const libc::c_void, _: size_t, _: *mut i8) -> size_t;
    fn address_list_release(_: *mut address_list);
    fn address_list_contains(_: *const address_list, _: *const ip_address) -> bool;
    fn lookup_host(_: *const i8, _: i32) -> *mut address_list;
    static mut numurls: i32;
    static mut total_downloaded_bytes: wgint;
    static mut total_download_time: libc::c_double;
    static mut output_stream: *mut FILE;
    fn fd_read_body(
        _: *const i8,
        _: i32,
        _: *mut FILE,
        _: wgint,
        _: wgint,
        _: *mut wgint,
        _: *mut wgint,
        _: *mut libc::c_double,
        _: i32,
        _: *mut FILE,
    ) -> i32;
    fn fd_read_hunk(_: i32, _: hunk_terminator_t, _: i64, _: i64) -> *mut i8;
    fn fd_read_line(_: i32) -> *mut i8;
    fn retr_rate(_: wgint, _: libc::c_double) -> *const i8;
    fn printwhat(_: i32, _: i32);
    fn sleep_between_retrievals(_: i32);
    fn rotate_backups(_: *const i8);
    fn set_local_file(_: *mut *const i8, _: *const i8);
    fn connect_to_host(_: *const i8, _: i32) -> i32;
    fn socket_ip_address(_: i32, _: *mut ip_address, _: i32) -> bool;
    fn socket_family(sock: i32, endpoint: i32) -> i32;
    fn retryable_socket_connect_error(_: i32) -> bool;
    fn test_socket_open(_: i32) -> bool;
    fn fd_read(_: i32, _: *mut i8, _: i32, _: libc::c_double) -> i32;
    fn fd_write(_: i32, _: *mut i8, _: i32, _: libc::c_double) -> i32;
    fn fd_errstr(_: i32) -> *const i8;
    fn fd_close(_: i32);
    fn search_netrc(
        _: *const i8,
        _: *mut *const i8,
        _: *mut *const i8,
        _: i32,
        _: *mut FILE,
    );
    fn ssl_init() -> bool;
    fn ssl_connect_wget(_: i32, _: *const i8, _: *mut i32) -> bool;
    fn ssl_check_certificate(_: i32, _: *const i8) -> bool;
    fn ntlm_input(_: *mut ntlmdata, _: *const i8) -> bool;
    fn ntlm_output(
        _: *mut ntlmdata,
        _: *const i8,
        _: *const i8,
        _: *mut bool,
    ) -> *mut i8;
    fn cookie_jar_new() -> *mut cookie_jar;
    fn cookie_handle_set_cookie(
        _: *mut cookie_jar,
        _: *const i8,
        _: i32,
        _: *const i8,
        _: *const i8,
    );
    fn cookie_header(
        _: *mut cookie_jar,
        _: *const i8,
        _: i32,
        _: *const i8,
        _: bool,
    ) -> *mut i8;
    fn cookie_jar_load(_: *mut cookie_jar, _: *const i8);
    fn cookie_jar_save(_: *mut cookie_jar, _: *const i8);
    fn md5_init_ctx(ctx: *mut md5_ctx);
    fn md5_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut md5_ctx);
    fn md5_finish_ctx(ctx: *mut md5_ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn downloaded_file(_: downloaded_file_t, _: *const i8) -> downloaded_file_t;
    fn nonexisting_url(_: *const i8);
    fn warc_uuid_str(id_str: *mut i8, urn_size: size_t);
    fn warc_timestamp(timestamp: *mut i8, timestamp_size: size_t) -> *mut i8;
    fn warc_tempfile() -> *mut FILE;
    fn warc_write_request_record(
        url: *const i8,
        timestamp_str: *const i8,
        concurrent_to_uuid: *const i8,
        ip: *const ip_address,
        body: *mut FILE,
        payload_offset: off_t,
    ) -> bool;
    fn warc_write_response_record(
        url: *const i8,
        timestamp_str: *const i8,
        concurrent_to_uuid: *const i8,
        ip: *const ip_address,
        body: *mut FILE,
        payload_offset: off_t,
        mime_type: *const i8,
        response_code: i32,
        redirect_location: *const i8,
    ) -> bool;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn c_strncasecmp(s1: *const i8, s2: *const i8, n: size_t) -> i32;
    static mut version_string: *const i8;
    fn xstrndup(string: *const i8, n: size_t) -> *mut i8;
    fn set_file_metadata(
        origin_url: *const url,
        referrer_url: *const url,
        fp: *mut FILE,
    ) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type hsts_store_t = *mut hsts_store;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_stat {
    pub len: wgint,
    pub contlen: wgint,
    pub restval: wgint,
    pub res: i32,
    pub rderrmsg: *mut i8,
    pub newloc: *mut i8,
    pub remote_time: *mut i8,
    pub error: *mut i8,
    pub statcode: i32,
    pub message: *mut i8,
    pub rd_size: wgint,
    pub dltime: libc::c_double,
    pub referer: *const i8,
    pub local_file: *mut i8,
    pub existence_checked: bool,
    pub timestamp_checked: bool,
    pub orig_file_name: *mut i8,
    pub orig_file_size: wgint,
    pub orig_file_tstamp: time_t,
    pub local_encoding: encoding_t,
    pub remote_encoding: encoding_t,
    pub temporary: bool,
}
pub type encoding_t = i32;
pub const ENC_BROTLI: encoding_t = 4;
pub const ENC_COMPRESS: encoding_t = 3;
pub const ENC_DEFLATE: encoding_t = 2;
pub const ENC_GZIP: encoding_t = 1;
pub const ENC_NONE: encoding_t = 0;
pub const ENC_INVALID: encoding_t = -1;
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
pub struct request {
    pub method: *const i8,
    pub arg: *mut i8,
    pub headers: *mut request_header,
    pub hcount: i32,
    pub hcapacity: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_header {
    pub name: *mut i8,
    pub value: *mut i8,
    pub release_policy: rp,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum rp {
    rel_none,
    rel_name,
    rel_value,
    rel_both,
}
impl rp {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            rp::rel_none => 0,
            rp::rel_name => 1,
            rp::rel_value => 2,
            rp::rel_both => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> rp {
        match value {
            0 => rp::rel_none,
            1 => rp::rel_name,
            2 => rp::rel_value,
            3 => rp::rel_both,
            _ => panic!("Invalid value for rp: {}", value),
        }
    }
}
impl AddAssign<u32> for rp {
    fn add_assign(&mut self, rhs: u32) {
        *self = rp::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for rp {
    fn sub_assign(&mut self, rhs: u32) {
        *self = rp::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for rp {
    fn mul_assign(&mut self, rhs: u32) {
        *self = rp::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for rp {
    fn div_assign(&mut self, rhs: u32) {
        *self = rp::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for rp {
    fn rem_assign(&mut self, rhs: u32) {
        *self = rp::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for rp {
    type Output = rp;
    fn add(self, rhs: u32) -> rp {
        rp::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for rp {
    type Output = rp;
    fn sub(self, rhs: u32) -> rp {
        rp::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for rp {
    type Output = rp;
    fn mul(self, rhs: u32) -> rp {
        rp::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for rp {
    type Output = rp;
    fn div(self, rhs: u32) -> rp {
        rp::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for rp {
    type Output = rp;
    fn rem(self, rhs: u32) -> rp {
        rp::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct response {
    pub data: *const i8,
    pub headers: *mut *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub socket: i32,
    pub host: *mut i8,
    pub port: i32,
    pub ssl: bool,
    pub authorized: bool,
    pub ntlm: ntlmdata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ntlmdata {
    pub state: wgetntlm,
    pub nonce: [u8; 8],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum wgetntlm {
    NTLMSTATE_NONE,
    NTLMSTATE_TYPE1,
    NTLMSTATE_TYPE2,
    NTLMSTATE_TYPE3,
    NTLMSTATE_LAST,
}
impl wgetntlm {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            wgetntlm::NTLMSTATE_NONE => 0,
            wgetntlm::NTLMSTATE_TYPE1 => 1,
            wgetntlm::NTLMSTATE_TYPE2 => 2,
            wgetntlm::NTLMSTATE_TYPE3 => 3,
            wgetntlm::NTLMSTATE_LAST => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> wgetntlm {
        match value {
            0 => wgetntlm::NTLMSTATE_NONE,
            1 => wgetntlm::NTLMSTATE_TYPE1,
            2 => wgetntlm::NTLMSTATE_TYPE2,
            3 => wgetntlm::NTLMSTATE_TYPE3,
            4 => wgetntlm::NTLMSTATE_LAST,
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
pub struct ip_address {
    pub family: i32,
    pub data: C2RustUnnamed_6,
    pub ipv6_scope: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_10 {
    rb_read_exactly = 1,
    rb_skip_startpos = 2,
    rb_chunked_transfer_encoding = 4,
    rb_compressed_gzip = 8,
}
impl C2RustUnnamed_10 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_10::rb_read_exactly => 1,
            C2RustUnnamed_10::rb_skip_startpos => 2,
            C2RustUnnamed_10::rb_chunked_transfer_encoding => 4,
            C2RustUnnamed_10::rb_compressed_gzip => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_10 {
        match value {
            1 => C2RustUnnamed_10::rb_read_exactly,
            2 => C2RustUnnamed_10::rb_skip_startpos,
            4 => C2RustUnnamed_10::rb_chunked_transfer_encoding,
            8 => C2RustUnnamed_10::rb_compressed_gzip,
            _ => panic!("Invalid value for C2RustUnnamed_10: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_10 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_10 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_10 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_10 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_10 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn add(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn sub(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn mul(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn div(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn rem(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type file_stats_t = file_stat_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: i32,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_8 {
    SKIP_SIZE = 512,
    SKIP_THRESHOLD = 4096,
}
impl C2RustUnnamed_8 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_8::SKIP_SIZE => 512,
            C2RustUnnamed_8::SKIP_THRESHOLD => 4096,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_8 {
        match value {
            512 => C2RustUnnamed_8::SKIP_SIZE,
            4096 => C2RustUnnamed_8::SKIP_THRESHOLD,
            _ => panic!("Invalid value for C2RustUnnamed_8: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_8 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_8 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_8 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_8 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_8 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn add(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn sub(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn mul(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn div(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_8 {
    type Output = C2RustUnnamed_8;
    fn rem(self, rhs: u32) -> C2RustUnnamed_8 {
        C2RustUnnamed_8::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct param_token {
    pub b: *const i8,
    pub e: *const i8,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub name: *const i8,
    pub variable: *mut *mut i8,
}
pub type hunk_terminator_t = Option<
    unsafe extern "C" fn(*const i8, *const i8, i32) -> *const i8,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_12 {
    ENDPOINT_LOCAL,
    ENDPOINT_PEER,
}
impl C2RustUnnamed_12 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_12::ENDPOINT_LOCAL => 0,
            C2RustUnnamed_12::ENDPOINT_PEER => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_12 {
        match value {
            0 => C2RustUnnamed_12::ENDPOINT_LOCAL,
            1 => C2RustUnnamed_12::ENDPOINT_PEER,
            _ => panic!("Invalid value for C2RustUnnamed_12: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_12 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_12 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_12 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_12 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_12 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn add(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn sub(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn mul(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn div(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn rem(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub const E_HOST: C2RustUnnamed_11 = -100;
pub type C2RustUnnamed_11 = i32;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
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
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
static mut cookies_loaded_p: bool = false;
static mut wget_cookie_jar: *mut cookie_jar = 0 as *const cookie_jar as *mut cookie_jar;
unsafe extern "C" fn request_new(
    mut method: *const i8,
    mut arg: *mut i8,
) -> *mut request {
    let mut req: *mut request = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<request>() as u64,
    ) as *mut request;
    (*req).hcapacity = 8 as i32;
    (*req).headers = xmalloc(
        ((*req).hcapacity as u64)
            .wrapping_mul(::core::mem::size_of::<request_header>() as u64),
    ) as *mut request_header;
    (*req).method = method;
    (*req).arg = arg;
    return req;
}
unsafe extern "C" fn request_method(mut req: *const request) -> *const i8 {
    return (*req).method;
}
unsafe extern "C" fn release_header(mut hdr: *mut request_header) {
    match (*hdr).release_policy as u32 {
        1 => {
            rpl_free((*hdr).name as *mut libc::c_void);
            (*hdr).name = 0 as *mut i8;
        }
        2 => {
            rpl_free((*hdr).value as *mut libc::c_void);
            (*hdr).value = 0 as *mut i8;
        }
        3 => {
            rpl_free((*hdr).name as *mut libc::c_void);
            (*hdr).name = 0 as *mut i8;
            rpl_free((*hdr).value as *mut libc::c_void);
            (*hdr).value = 0 as *mut i8;
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn request_set_header(
    mut req: *mut request,
    mut name: *const i8,
    mut value: *const i8,
    mut release_policy: rp,
) {
    let mut hdr: *mut request_header = 0 as *mut request_header;
    let mut i: i32 = 0;
    if value.is_null() {
        if release_policy as u32 == rp::rel_name as i32 as u32
            || release_policy as u32 == rp::rel_both as i32 as u32
        {
            rpl_free(name as *mut libc::c_void);
            name = 0 as *const i8;
        }
        return;
    }
    i = 0 as i32;
    while i < (*req).hcount {
        hdr = &mut *((*req).headers).offset(i as isize) as *mut request_header;
        if 0 as i32 == c_strcasecmp(name, (*hdr).name) {
            release_header(hdr);
            (*hdr).name = name as *mut libc::c_void as *mut i8;
            (*hdr).value = value as *mut libc::c_void as *mut i8;
            (*hdr).release_policy = release_policy;
            return;
        }
        i += 1;
        i;
    }
    if (*req).hcount >= (*req).hcapacity {
        (*req).hcapacity <<= 1 as i32;
        (*req).headers = xrealloc(
            (*req).headers as *mut libc::c_void,
            ((*req).hcapacity as u64)
                .wrapping_mul(::core::mem::size_of::<request_header>() as u64),
        ) as *mut request_header;
    }
    let fresh0 = (*req).hcount;
    (*req).hcount = (*req).hcount + 1;
    hdr = &mut *((*req).headers).offset(fresh0 as isize) as *mut request_header;
    (*hdr).name = name as *mut libc::c_void as *mut i8;
    (*hdr).value = value as *mut libc::c_void as *mut i8;
    (*hdr).release_policy = release_policy;
}
unsafe extern "C" fn request_set_user_header(
    mut req: *mut request,
    mut header: *const i8,
) {
    let mut name: *const i8 = 0 as *const i8;
    let mut p: *const i8 = 0 as *const i8;
    p = strchr(header, ':' as i32);
    if p.is_null() {
        return;
    }
    name = xstrndup(header, p.offset_from(header) as i64 as size_t);
    p = p.offset(1);
    p;
    while c_isspace(*p as i32) {
        p = p.offset(1);
        p;
    }
    request_set_header(req, name, p, rp::rel_name);
}
unsafe extern "C" fn request_remove_header(
    mut req: *mut request,
    mut name: *const i8,
) -> bool {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*req).hcount {
        let mut hdr: *mut request_header = &mut *((*req).headers).offset(i as isize)
            as *mut request_header;
        if 0 as i32 == c_strcasecmp(name, (*hdr).name) {
            release_header(hdr);
            if i < (*req).hcount - 1 as i32 {
                memmove(
                    hdr as *mut libc::c_void,
                    hdr.offset(1 as i32 as isize) as *const libc::c_void,
                    (((*req).hcount - i - 1 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<request_header>() as u64),
                );
            }
            (*req).hcount -= 1;
            (*req).hcount;
            return 1 as i32 != 0;
        }
        i += 1;
        i;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn request_send(
    mut req: *const request,
    mut fd: i32,
    mut warc_tmp: *mut FILE,
) -> i32 {
    let mut request_string: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut size: i32 = 0;
    let mut write_error: i32 = 0;
    size = 0 as i32;
    size = (size as u64)
        .wrapping_add(
            (strlen((*req).method))
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(strlen((*req).arg))
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(8 as i32 as u64)
                .wrapping_add(2 as i32 as u64),
        ) as i32 as i32;
    i = 0 as i32;
    while i < (*req).hcount {
        let mut hdr: *mut request_header = &mut *((*req).headers).offset(i as isize)
            as *mut request_header;
        size = (size as u64)
            .wrapping_add(
                (strlen((*hdr).name))
                    .wrapping_add(2 as i32 as u64)
                    .wrapping_add(strlen((*hdr).value))
                    .wrapping_add(2 as i32 as u64),
            ) as i32 as i32;
        i += 1;
        i;
    }
    size += 3 as i32;
    request_string = xmalloc(size as size_t) as *mut i8;
    p = request_string;
    let mut A_len: i32 = strlen((*req).method) as i32;
    memcpy(p as *mut libc::c_void, (*req).method as *const libc::c_void, A_len as u64);
    p = p.offset(A_len as isize);
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = ' ' as i32 as i8;
    let mut A_len_0: i32 = strlen((*req).arg) as i32;
    memcpy(p as *mut libc::c_void, (*req).arg as *const libc::c_void, A_len_0 as u64);
    p = p.offset(A_len_0 as isize);
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = ' ' as i32 as i8;
    memcpy(
        p as *mut libc::c_void,
        b"HTTP/1.1\r\n\0" as *const u8 as *const i8 as *const libc::c_void,
        10 as i32 as u64,
    );
    p = p.offset(10 as i32 as isize);
    i = 0 as i32;
    while i < (*req).hcount {
        let mut hdr_0: *mut request_header = &mut *((*req).headers).offset(i as isize)
            as *mut request_header;
        let mut A_len_1: i32 = strlen((*hdr_0).name) as i32;
        memcpy(
            p as *mut libc::c_void,
            (*hdr_0).name as *const libc::c_void,
            A_len_1 as u64,
        );
        p = p.offset(A_len_1 as isize);
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = ':' as i32 as i8;
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ' ' as i32 as i8;
        let mut A_len_2: i32 = strlen((*hdr_0).value) as i32;
        memcpy(
            p as *mut libc::c_void,
            (*hdr_0).value as *const libc::c_void,
            A_len_2 as u64,
        );
        p = p.offset(A_len_2 as isize);
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '\r' as i32 as i8;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '\n' as i32 as i8;
        i += 1;
        i;
    }
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = '\r' as i32 as i8;
    let fresh8 = p;
    p = p.offset(1);
    *fresh8 = '\n' as i32 as i8;
    let fresh9 = p;
    p = p.offset(1);
    *fresh9 = '\0' as i32 as i8;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"\n---request begin---\n%s---request end---\n\0" as *const u8 as *const i8,
            request_string,
        );
    }
    write_error = fd_write(
        fd,
        request_string,
        size - 1 as i32,
        -(1 as i32) as libc::c_double,
    );
    if write_error < 0 as i32 {
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Failed writing HTTP request: %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fd_errstr(fd),
        );
    } else if !warc_tmp.is_null() {
        let mut warc_tmp_written: i32 = fwrite(
            request_string as *const libc::c_void,
            1 as i32 as size_t,
            (size - 1 as i32) as size_t,
            warc_tmp,
        ) as i32;
        if warc_tmp_written != size - 1 as i32 {
            write_error = -(2 as i32);
        }
    }
    rpl_free(request_string as *mut libc::c_void);
    request_string = 0 as *mut i8;
    return write_error;
}
unsafe extern "C" fn request_free(mut req_ref: *mut *mut request) {
    let mut i: i32 = 0;
    let mut req: *mut request = *req_ref;
    if req.is_null() {
        return;
    }
    rpl_free((*req).arg as *mut libc::c_void);
    (*req).arg = 0 as *mut i8;
    i = 0 as i32;
    while i < (*req).hcount {
        release_header(&mut *((*req).headers).offset(i as isize));
        i += 1;
        i;
    }
    rpl_free((*req).headers as *mut libc::c_void);
    (*req).headers = 0 as *mut request_header;
    rpl_free(req as *mut libc::c_void);
    req = 0 as *mut request;
    *req_ref = 0 as *mut request;
}
static mut basic_authed_hosts: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn maybe_send_basic_creds(
    mut hostname: *const i8,
    mut user: *const i8,
    mut passwd: *const i8,
    mut req: *mut request,
) -> bool {
    let mut do_challenge: bool = 0 as i32 != 0;
    if opt.auth_without_challenge {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Auth-without-challenge set, sending Basic credentials.\n\0"
                    as *const u8 as *const i8,
            );
        }
        do_challenge = 1 as i32 != 0;
    } else if !basic_authed_hosts.is_null()
        && hash_table_contains(basic_authed_hosts, hostname as *const libc::c_void) != 0
    {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Found %s in basic_authed_hosts.\n\0" as *const u8 as *const i8,
                quote(hostname),
            );
        }
        do_challenge = 1 as i32 != 0;
    } else if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Host %s has not issued a general basic challenge.\n\0" as *const u8
                as *const i8,
            quote(hostname),
        );
    }
    if do_challenge {
        request_set_header(
            req,
            b"Authorization\0" as *const u8 as *const i8,
            basic_authentication_encode(user, passwd),
            rp::rel_value,
        );
    }
    return do_challenge;
}
unsafe extern "C" fn register_basic_auth_host(mut hostname: *const i8) {
    if basic_authed_hosts.is_null() {
        basic_authed_hosts = make_nocase_string_hash_table(1 as i32);
    }
    if hash_table_contains(basic_authed_hosts, hostname as *const libc::c_void) == 0 {
        hash_table_put(
            basic_authed_hosts,
            xstrdup(hostname) as *const libc::c_void,
            0 as *const libc::c_void,
        );
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Inserted %s into basic_authed_hosts\n\0" as *const u8 as *const i8,
                quote(hostname),
            );
        }
    }
}
unsafe extern "C" fn body_file_send(
    mut sock: i32,
    mut file_name: *const i8,
    mut promised_size: wgint,
    mut warc_tmp: *mut FILE,
) -> i32 {
    static mut chunk: [i8; 8192] = [0; 8192];
    let mut written: wgint = 0 as i32 as wgint;
    let mut write_error: i32 = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"[writing BODY file %s ... \0" as *const u8 as *const i8,
            file_name,
        );
    }
    fp = rpl_fopen(file_name, b"rb\0" as *const u8 as *const i8);
    if fp.is_null() {
        return -(1 as i32);
    }
    while feof(fp) == 0 && written < promised_size {
        let mut towrite: i32 = 0;
        let mut length: i32 = fread(
            chunk.as_mut_ptr() as *mut libc::c_void,
            1 as i32 as size_t,
            ::core::mem::size_of::<[i8; 8192]>() as u64,
            fp,
        ) as i32;
        if length == 0 as i32 {
            break;
        }
        towrite = (if promised_size - written <= length as i64 {
            promised_size - written
        } else {
            length as i64
        }) as i32;
        write_error = fd_write(
            sock,
            chunk.as_mut_ptr(),
            towrite,
            -(1 as i32) as libc::c_double,
        );
        if write_error < 0 as i32 {
            fclose(fp);
            return -(1 as i32);
        }
        if !warc_tmp.is_null() {
            let mut warc_tmp_written: i32 = fwrite(
                chunk.as_mut_ptr() as *const libc::c_void,
                1 as i32 as size_t,
                towrite as size_t,
                warc_tmp,
            ) as i32;
            if warc_tmp_written != towrite {
                fclose(fp);
                return -(2 as i32);
            }
        }
        written += towrite as i64;
    }
    fclose(fp);
    if written < promised_size {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"done]\n\0" as *const u8 as *const i8);
    }
    return 0 as i32;
}
unsafe extern "C" fn response_head_terminator(
    mut start: *const i8,
    mut peeked: *const i8,
    mut peeklen: i32,
) -> *const i8 {
    let mut p: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    if start == peeked
        && 0 as i32
            != memcmp(
                start as *const libc::c_void,
                b"HTTP\0" as *const u8 as *const i8 as *const libc::c_void,
                (if peeklen <= 4 as i32 { peeklen } else { 4 as i32 }) as u64,
            )
    {
        return start;
    }
    p = if (peeked.offset_from(start) as i64) < 2 as i32 as i64 {
        start
    } else {
        peeked.offset(-(2 as i32 as isize))
    };
    end = peeked.offset(peeklen as isize);
    while p < end.offset(-(2 as i32 as isize)) {
        if *p as i32 == '\n' as i32 {
            if *p.offset(1 as i32 as isize) as i32 == '\r' as i32
                && *p.offset(2 as i32 as isize) as i32 == '\n' as i32
            {
                return p.offset(3 as i32 as isize)
            } else if *p.offset(1 as i32 as isize) as i32 == '\n' as i32 {
                return p.offset(2 as i32 as isize)
            }
        }
        p = p.offset(1);
        p;
    }
    if peeklen >= 2 as i32 && *p.offset(0 as i32 as isize) as i32 == '\n' as i32
        && *p.offset(1 as i32 as isize) as i32 == '\n' as i32
    {
        return p.offset(2 as i32 as isize);
    }
    return 0 as *const i8;
}
unsafe extern "C" fn read_http_response_head(mut fd: i32) -> *mut i8 {
    return fd_read_hunk(
        fd,
        Some(
            response_head_terminator
                as unsafe extern "C" fn(*const i8, *const i8, i32) -> *const i8,
        ),
        512 as i32 as i64,
        65536 as i32 as i64,
    );
}
unsafe extern "C" fn resp_new(mut head: *mut i8) -> *mut response {
    let mut hdr: *mut i8 = 0 as *mut i8;
    let mut count: i32 = 0;
    let mut size: i32 = 0;
    let mut resp: *mut response = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<response>() as u64,
    ) as *mut response;
    (*resp).data = head;
    if *head as i32 == '\0' as i32 {
        return resp;
    }
    count = 0 as i32;
    size = count;
    hdr = head;
    loop {
        let mut DR_needed_size: i64 = (count + 1 as i32) as i64;
        let mut DR_newsize: i64 = 0 as i32 as i64;
        while (size as i64) < DR_needed_size {
            DR_newsize = (size << 1 as i32) as i64;
            if DR_newsize < 16 as i32 as i64 {
                DR_newsize = 16 as i32 as i64;
            }
            size = DR_newsize as i32;
        }
        if DR_newsize != 0 {
            (*resp).headers = xrealloc(
                (*resp).headers as *mut libc::c_void,
                (DR_newsize as u64)
                    .wrapping_mul(::core::mem::size_of::<*const i8>() as u64),
            ) as *mut *const i8;
        }
        let fresh10 = count;
        count = count + 1;
        let ref mut fresh11 = *((*resp).headers).offset(fresh10 as isize);
        *fresh11 = hdr;
        if *hdr.offset(0 as i32 as isize) == 0
            || *hdr.offset(0 as i32 as isize) as i32 == '\r' as i32
                && *hdr.offset(1 as i32 as isize) as i32 == '\n' as i32
            || *hdr.offset(0 as i32 as isize) as i32 == '\n' as i32
        {
            break;
        }
        loop {
            let mut end: *mut i8 = strchr(hdr, '\n' as i32);
            if end.is_null() {
                hdr = hdr.offset(strlen(hdr) as isize);
                break;
            } else {
                hdr = end.offset(1 as i32 as isize);
                if *hdr as i32 != ' ' as i32 && *hdr as i32 != '\t' as i32 {
                    break;
                }
                *end = ' ' as i32 as i8;
                if end > head && *end.offset(-(1 as i32) as isize) as i32 == '\r' as i32
                {
                    *end.offset(-(1 as i32) as isize) = ' ' as i32 as i8;
                }
            }
        }
    }
    let mut DR_needed_size_0: i64 = (count + 1 as i32) as i64;
    let mut DR_newsize_0: i64 = 0 as i32 as i64;
    while (size as i64) < DR_needed_size_0 {
        DR_newsize_0 = (size << 1 as i32) as i64;
        if DR_newsize_0 < 16 as i32 as i64 {
            DR_newsize_0 = 16 as i32 as i64;
        }
        size = DR_newsize_0 as i32;
    }
    if DR_newsize_0 != 0 {
        (*resp).headers = xrealloc(
            (*resp).headers as *mut libc::c_void,
            (DR_newsize_0 as u64)
                .wrapping_mul(::core::mem::size_of::<*const i8>() as u64),
        ) as *mut *const i8;
    }
    let ref mut fresh12 = *((*resp).headers).offset(count as isize);
    *fresh12 = 0 as *const i8;
    return resp;
}
unsafe extern "C" fn resp_header_locate(
    mut resp: *const response,
    mut name: *const i8,
    mut start: i32,
    mut begptr: *mut *const i8,
    mut endptr: *mut *const i8,
) -> i32 {
    let mut i: i32 = 0;
    let mut headers: *mut *const i8 = (*resp).headers;
    let mut name_len: i32 = 0;
    if headers.is_null() || (*headers.offset(1 as i32 as isize)).is_null() {
        return -(1 as i32);
    }
    name_len = strlen(name) as i32;
    if start > 0 as i32 {
        i = start;
    } else {
        i = 1 as i32;
    }
    while !(*headers.offset((i + 1 as i32) as isize)).is_null() {
        let mut b: *const i8 = *headers.offset(i as isize);
        let mut e: *const i8 = *headers.offset((i + 1 as i32) as isize);
        if e.offset_from(b) as i64 > name_len as i64
            && *b.offset(name_len as isize) as i32 == ':' as i32
            && 0 as i32 == c_strncasecmp(b, name, name_len as size_t)
        {
            b = b.offset((name_len + 1 as i32) as isize);
            while b < e && c_isspace(*b as i32) as i32 != 0 {
                b = b.offset(1);
                b;
            }
            while b < e && c_isspace(*e.offset(-(1 as i32) as isize) as i32) as i32 != 0
            {
                e = e.offset(-1);
                e;
            }
            *begptr = b;
            *endptr = e;
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as i32);
}
unsafe extern "C" fn resp_header_get(
    mut resp: *const response,
    mut name: *const i8,
    mut begptr: *mut *const i8,
    mut endptr: *mut *const i8,
) -> bool {
    let mut pos: i32 = resp_header_locate(resp, name, 0 as i32, begptr, endptr);
    return pos != -(1 as i32);
}
unsafe extern "C" fn resp_header_copy(
    mut resp: *const response,
    mut name: *const i8,
    mut buf: *mut i8,
    mut bufsize: i32,
) -> bool {
    let mut b: *const i8 = 0 as *const i8;
    let mut e: *const i8 = 0 as *const i8;
    if !resp_header_get(resp, name, &mut b, &mut e) {
        return 0 as i32 != 0;
    }
    if bufsize != 0 {
        let mut len: i32 = (if e.offset_from(b) as i64 <= (bufsize - 1 as i32) as i64 {
            e.offset_from(b) as i64
        } else {
            (bufsize - 1 as i32) as i64
        }) as i32;
        memcpy(buf as *mut libc::c_void, b as *const libc::c_void, len as u64);
        *buf.offset(len as isize) = '\0' as i32 as i8;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn resp_header_strdup(
    mut resp: *const response,
    mut name: *const i8,
) -> *mut i8 {
    let mut b: *const i8 = 0 as *const i8;
    let mut e: *const i8 = 0 as *const i8;
    if !resp_header_get(resp, name, &mut b, &mut e) {
        return 0 as *mut i8;
    }
    return strdupdelim(b, e);
}
unsafe extern "C" fn resp_status(
    mut resp: *const response,
    mut message: *mut *mut i8,
) -> i32 {
    let mut status: i32 = 0;
    let mut p: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    if ((*resp).headers).is_null() {
        if !message.is_null() {
            *message = xstrdup(
                dcgettext(
                    0 as *const i8,
                    b"No headers, assuming HTTP/0.9\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        return 200 as i32;
    }
    p = *((*resp).headers).offset(0 as i32 as isize);
    end = *((*resp).headers).offset(1 as i32 as isize);
    if end.is_null() {
        return -(1 as i32);
    }
    if (end.offset_from(p) as i64) < 4 as i32 as i64
        || 0 as i32 != strncmp(p, b"HTTP\0" as *const u8 as *const i8, 4 as i32 as u64)
    {
        return -(1 as i32);
    }
    p = p.offset(4 as i32 as isize);
    if p < end && *p as i32 == '/' as i32 {
        p = p.offset(1);
        p;
        while p < end && c_isdigit(*p as i32) as i32 != 0 {
            p = p.offset(1);
            p;
        }
        if p < end && *p as i32 == '.' as i32 {
            p = p.offset(1);
            p;
        }
        while p < end && c_isdigit(*p as i32) as i32 != 0 {
            p = p.offset(1);
            p;
        }
    }
    while p < end && c_isspace(*p as i32) as i32 != 0 {
        p = p.offset(1);
        p;
    }
    if (end.offset_from(p) as i64) < 3 as i32 as i64
        || !c_isdigit(*p.offset(0 as i32 as isize) as i32)
        || !c_isdigit(*p.offset(1 as i32 as isize) as i32)
        || !c_isdigit(*p.offset(2 as i32 as isize) as i32)
    {
        return -(1 as i32);
    }
    status = 100 as i32 * (*p.offset(0 as i32 as isize) as i32 - '0' as i32)
        + 10 as i32 * (*p.offset(1 as i32 as isize) as i32 - '0' as i32)
        + (*p.offset(2 as i32 as isize) as i32 - '0' as i32);
    p = p.offset(3 as i32 as isize);
    if !message.is_null() {
        while p < end && c_isspace(*p as i32) as i32 != 0 {
            p = p.offset(1);
            p;
        }
        while p < end && c_isspace(*end.offset(-(1 as i32) as isize) as i32) as i32 != 0
        {
            end = end.offset(-1);
            end;
        }
        *message = strdupdelim(p, end);
    }
    return status;
}
unsafe extern "C" fn resp_free(mut resp_ref: *mut *mut response) {
    let mut resp: *mut response = *resp_ref;
    if resp.is_null() {
        return;
    }
    rpl_free((*resp).headers as *mut libc::c_void);
    (*resp).headers = 0 as *mut *const i8;
    rpl_free(resp as *mut libc::c_void);
    resp = 0 as *mut response;
    *resp_ref = 0 as *mut response;
}
unsafe extern "C" fn print_response_line(
    mut prefix: *const i8,
    mut b: *const i8,
    mut e: *const i8,
) {
    let mut buf: [i8; 1024] = [0; 1024];
    let mut copy: *mut i8 = 0 as *mut i8;
    let mut len: size_t = e.offset_from(b) as i64 as size_t;
    if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
        copy = buf.as_mut_ptr();
    } else {
        copy = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
    }
    memcpy(copy as *mut libc::c_void, b as *const libc::c_void, len);
    *copy.offset(len as isize) = 0 as i32 as i8;
    logprintf(
        log_options::LOG_ALWAYS,
        b"%s%s\n\0" as *const u8 as *const i8,
        prefix,
        quotearg_style(quoting_style::escape_quoting_style, copy),
    );
    if copy != buf.as_mut_ptr() {
        rpl_free(copy as *mut libc::c_void);
        copy = 0 as *mut i8;
    }
}
unsafe extern "C" fn print_server_response(
    mut resp: *const response,
    mut prefix: *const i8,
) {
    let mut i: i32 = 0;
    if ((*resp).headers).is_null() {
        return;
    }
    i = 0 as i32;
    while !(*((*resp).headers).offset((i + 1 as i32) as isize)).is_null() {
        let mut b: *const i8 = *((*resp).headers).offset(i as isize);
        let mut e: *const i8 = *((*resp).headers).offset((i + 1 as i32) as isize);
        if b < e && *e.offset(-(1 as i32) as isize) as i32 == '\n' as i32 {
            e = e.offset(-1);
            e;
        }
        if b < e && *e.offset(-(1 as i32) as isize) as i32 == '\r' as i32 {
            e = e.offset(-1);
            e;
        }
        print_response_line(prefix, b, e);
        i += 1;
        i;
    }
}
unsafe extern "C" fn parse_content_range(
    mut hdr: *const i8,
    mut first_byte_ptr: *mut wgint,
    mut last_byte_ptr: *mut wgint,
    mut entity_length_ptr: *mut wgint,
) -> bool {
    let mut num: wgint = 0;
    if 0 as i32
        == strncasecmp(hdr, b"bytes\0" as *const u8 as *const i8, 5 as i32 as u64)
    {
        hdr = hdr.offset(5 as i32 as isize);
        if *hdr as i32 == ':' as i32 {
            hdr = hdr.offset(1);
            hdr;
        }
        while c_isspace(*hdr as i32) {
            hdr = hdr.offset(1);
            hdr;
        }
        if *hdr == 0 {
            return 0 as i32 != 0;
        }
    }
    if !c_isdigit(*hdr as i32) {
        return 0 as i32 != 0;
    }
    num = 0 as i32 as wgint;
    while c_isdigit(*hdr as i32) {
        num = 10 as i32 as i64 * num + (*hdr as i32 - '0' as i32) as i64;
        hdr = hdr.offset(1);
        hdr;
    }
    if *hdr as i32 != '-' as i32 || !c_isdigit(*hdr.offset(1 as i32 as isize) as i32) {
        return 0 as i32 != 0;
    }
    *first_byte_ptr = num;
    hdr = hdr.offset(1);
    hdr;
    num = 0 as i32 as wgint;
    while c_isdigit(*hdr as i32) {
        num = 10 as i32 as i64 * num + (*hdr as i32 - '0' as i32) as i64;
        hdr = hdr.offset(1);
        hdr;
    }
    if *hdr as i32 != '/' as i32 {
        return 0 as i32 != 0;
    }
    *last_byte_ptr = num;
    if !(c_isdigit(*hdr.offset(1 as i32 as isize) as i32) as i32 != 0
        || *hdr.offset(1 as i32 as isize) as i32 == '*' as i32)
    {
        return 0 as i32 != 0;
    }
    if *last_byte_ptr < *first_byte_ptr {
        return 0 as i32 != 0;
    }
    hdr = hdr.offset(1);
    hdr;
    if *hdr as i32 == '*' as i32 {
        num = -(1 as i32) as wgint;
    } else {
        num = 0 as i32 as wgint;
        while c_isdigit(*hdr as i32) {
            num = 10 as i32 as i64 * num + (*hdr as i32 - '0' as i32) as i64;
            hdr = hdr.offset(1);
            hdr;
        }
    }
    *entity_length_ptr = num;
    if *entity_length_ptr <= *last_byte_ptr && *entity_length_ptr != -(1 as i32) as i64 {
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn skip_short_body(
    mut fd: i32,
    mut contlen: wgint,
    mut chunked: bool,
) -> bool {
    let mut remaining_chunk_size: wgint = 0 as i32 as wgint;
    let mut dlbuf: [i8; 513] = [0; 513];
    dlbuf[C2RustUnnamed_8::SKIP_SIZE as i32 as usize] = '\0' as i32 as i8;
    if contlen > C2RustUnnamed_8::SKIP_THRESHOLD as i32 as i64 {
        return 0 as i32 != 0;
    }
    while contlen > 0 as i32 as i64 || chunked as i32 != 0 {
        let mut ret: i32 = 0;
        if chunked {
            if remaining_chunk_size == 0 as i32 as i64 {
                let mut line: *mut i8 = fd_read_line(fd);
                let mut endl: *mut i8 = 0 as *mut i8;
                if line.is_null() {
                    break;
                }
                remaining_chunk_size = rpl_strtol(line, &mut endl, 16 as i32);
                rpl_free(line as *mut libc::c_void);
                line = 0 as *mut i8;
                if remaining_chunk_size < 0 as i32 as i64 {
                    return 0 as i32 != 0;
                }
                if remaining_chunk_size == 0 as i32 as i64 {
                    line = fd_read_line(fd);
                    rpl_free(line as *mut libc::c_void);
                    line = 0 as *mut i8;
                    break;
                }
            }
            contlen = if remaining_chunk_size <= C2RustUnnamed_8::SKIP_SIZE as i32 as i64
            {
                remaining_chunk_size
            } else {
                C2RustUnnamed_8::SKIP_SIZE as i32 as i64
            };
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"Skipping %s bytes of body: [\0" as *const u8 as *const i8,
                number_to_static_string(contlen),
            );
        }
        ret = fd_read(
            fd,
            dlbuf.as_mut_ptr(),
            (if contlen <= C2RustUnnamed_8::SKIP_SIZE as i32 as i64 {
                contlen
            } else {
                C2RustUnnamed_8::SKIP_SIZE as i32 as i64
            }) as i32,
            -(1 as i32) as libc::c_double,
        );
        if ret <= 0 as i32 {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"] aborting (%s).\n\0" as *const u8 as *const i8,
                    if ret < 0 as i32 {
                        fd_errstr(fd)
                    } else {
                        b"EOF received\0" as *const u8 as *const i8
                    },
                );
            }
            return 0 as i32 != 0;
        }
        contlen -= ret as i64;
        if chunked {
            remaining_chunk_size -= ret as i64;
            if remaining_chunk_size == 0 as i32 as i64 {
                let mut line_0: *mut i8 = fd_read_line(fd);
                if line_0.is_null() {
                    return 0 as i32 != 0
                } else {
                    rpl_free(line_0 as *mut libc::c_void);
                    line_0 = 0 as *mut i8;
                }
            }
        }
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"%.*s\0" as *const u8 as *const i8,
                ret,
                dlbuf.as_mut_ptr(),
            );
        }
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(b"] done.\n\0" as *const u8 as *const i8);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn modify_param_name(mut name: *mut param_token) -> i32 {
    let mut delim1: *const i8 = memchr(
        (*name).b as *const libc::c_void,
        '*' as i32,
        ((*name).e).offset_from((*name).b) as i64 as u64,
    ) as *const i8;
    let mut delim2: *const i8 = memrchr(
        (*name).b as *const libc::c_void,
        '*' as i32,
        ((*name).e).offset_from((*name).b) as i64 as size_t,
    ) as *const i8;
    let mut result: i32 = 0;
    if delim1.is_null() {
        result = 0 as i32;
    } else if delim1 == delim2 {
        if ((*name).e).offset(-(1 as i32 as isize)) == delim1 {
            result = 2 as i32;
        } else {
            result = 1 as i32;
        }
        (*name).e = delim1;
    } else {
        (*name).e = delim1;
        result = 2 as i32;
    }
    return result;
}
unsafe extern "C" fn modify_param_value(
    mut value: *mut param_token,
    mut encoding_type: i32,
) {
    if encoding_type == 2 as i32 {
        let mut delim: *const i8 = memrchr(
            (*value).b as *const libc::c_void,
            '\'' as i32,
            ((*value).e).offset_from((*value).b) as i64 as size_t,
        ) as *const i8;
        if !delim.is_null() {
            (*value).b = delim.offset(1 as i32 as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn extract_param(
    mut source: *mut *const i8,
    mut name: *mut param_token,
    mut value: *mut param_token,
    mut separator: i8,
    mut is_url_encoded: *mut bool,
) -> bool {
    let mut p: *const i8 = *source;
    let mut param_type: i32 = 0;
    if !is_url_encoded.is_null() {
        *is_url_encoded = 0 as i32 != 0;
    }
    while c_isspace(*p as i32) {
        p = p.offset(1);
        p;
    }
    if *p == 0 {
        *source = p;
        return 0 as i32 != 0;
    }
    (*name).b = p;
    while *p as i32 != 0 && !c_isspace(*p as i32) && *p as i32 != '=' as i32
        && *p as i32 != separator as i32
    {
        p = p.offset(1);
        p;
    }
    (*name).e = p;
    if (*name).b == (*name).e {
        return 0 as i32 != 0;
    }
    while c_isspace(*p as i32) {
        p = p.offset(1);
        p;
    }
    if *p as i32 == separator as i32 || *p == 0 {
        memset(
            value as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<param_token>() as u64,
        );
        if *p as i32 == separator as i32 {
            p = p.offset(1);
            p;
        }
        *source = p;
        return 1 as i32 != 0;
    }
    if *p as i32 != '=' as i32 {
        return 0 as i32 != 0;
    }
    p = p.offset(1);
    p;
    while c_isspace(*p as i32) {
        p = p.offset(1);
        p;
    }
    if *p as i32 == '"' as i32 {
        p = p.offset(1);
        (*value).b = p;
        while *p as i32 != 0 && *p as i32 != '"' as i32 {
            p = p.offset(1);
            p;
        }
        if *p == 0 {
            return 0 as i32 != 0;
        }
        let fresh13 = p;
        p = p.offset(1);
        (*value).e = fresh13;
        while c_isspace(*p as i32) {
            p = p.offset(1);
            p;
        }
        while *p as i32 != 0 && *p as i32 != separator as i32 {
            p = p.offset(1);
            p;
        }
        if *p as i32 == separator as i32 {
            p = p.offset(1);
            p;
        } else if *p != 0 {
            return 0 as i32 != 0
        }
    } else {
        (*value).b = p;
        while *p as i32 != 0 && *p as i32 != separator as i32 {
            p = p.offset(1);
            p;
        }
        (*value).e = p;
        while (*value).e != (*value).b
            && c_isspace(*((*value).e).offset(-(1 as i32) as isize) as i32) as i32 != 0
        {
            (*value).e = ((*value).e).offset(-1);
            (*value).e;
        }
        if *p as i32 == separator as i32 {
            p = p.offset(1);
            p;
        }
    }
    *source = p;
    param_type = modify_param_name(name);
    if param_type != 0 as i32 {
        if param_type == 2 as i32 && !is_url_encoded.is_null() {
            *is_url_encoded = 1 as i32 != 0;
        }
        modify_param_value(value, param_type);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn append_value_to_filename(
    mut filename: *mut *mut i8,
    value: *const param_token,
    mut is_url_encoded: bool,
) {
    let mut original_length: i32 = strlen(*filename) as i32;
    let mut new_length: i32 = (strlen(*filename))
        .wrapping_add(((*value).e).offset_from((*value).b) as i64 as u64) as i32;
    *filename = xrealloc(
        *filename as *mut libc::c_void,
        (new_length + 1 as i32) as size_t,
    ) as *mut i8;
    memcpy(
        (*filename).offset(original_length as isize) as *mut libc::c_void,
        (*value).b as *const libc::c_void,
        ((*value).e).offset_from((*value).b) as i64 as u64,
    );
    *(*filename).offset(new_length as isize) = '\0' as i32 as i8;
    if is_url_encoded {
        url_unescape((*filename).offset(original_length as isize));
    }
}
unsafe extern "C" fn parse_content_disposition(
    mut hdr: *const i8,
    mut filename: *mut *mut i8,
) -> bool {
    let mut name: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    let mut value: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    let mut is_url_encoded: bool = 0 as i32 != 0;
    let mut encodedFilename: *mut i8 = 0 as *mut i8;
    let mut unencodedFilename: *mut i8 = 0 as *mut i8;
    while extract_param(
        &mut hdr,
        &mut name,
        &mut value,
        ';' as i32 as i8,
        &mut is_url_encoded,
    ) {
        let mut isFilename: i32 = ((name.e).offset_from(name.b) as i64 as u64
            == (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64)
            && c_strncasecmp(
                name.b,
                b"filename\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64),
            ) == 0) as i32;
        if isFilename != 0 && !(value.b).is_null() {
            let mut isEncodedFilename: bool = false;
            let mut outFilename: *mut *mut i8 = 0 as *mut *mut i8;
            let mut last_slash: *const i8 = memrchr(
                value.b as *const libc::c_void,
                '/' as i32,
                (value.e).offset_from(value.b) as i64 as size_t,
            ) as *const i8;
            let mut last_bs: *const i8 = memrchr(
                value.b as *const libc::c_void,
                '\\' as i32,
                (value.e).offset_from(value.b) as i64 as size_t,
            ) as *const i8;
            if !last_slash.is_null() && !last_bs.is_null() {
                value.b = (if last_slash >= last_bs { last_slash } else { last_bs })
                    .offset(1 as i32 as isize);
            } else if !last_slash.is_null() || !last_bs.is_null() {
                value.b = (if !last_slash.is_null() { last_slash } else { last_bs })
                    .offset(1 as i32 as isize);
            }
            if !(value.b == value.e) {
                isEncodedFilename = *name.e as i32 == '*' as i32
                    && !c_isdigit(*(name.e).offset(1 as i32 as isize) as i32);
                outFilename = if isEncodedFilename as i32 != 0 {
                    &mut encodedFilename
                } else {
                    &mut unencodedFilename
                };
                if !(*outFilename).is_null() {
                    append_value_to_filename(outFilename, &mut value, is_url_encoded);
                } else {
                    *outFilename = strdupdelim(value.b, value.e);
                    if is_url_encoded {
                        url_unescape(*outFilename);
                    }
                }
            }
        }
        is_url_encoded = 0 as i32 != 0;
    }
    if !encodedFilename.is_null() {
        rpl_free(unencodedFilename as *mut libc::c_void);
        unencodedFilename = 0 as *mut i8;
        *filename = encodedFilename;
    } else {
        rpl_free(encodedFilename as *mut libc::c_void);
        encodedFilename = 0 as *mut i8;
        *filename = unencodedFilename;
    }
    if !(*filename).is_null() { return 1 as i32 != 0 } else { return 0 as i32 != 0 };
}
unsafe extern "C" fn parse_strict_transport_security(
    mut header: *const i8,
    mut max_age: *mut int64_t,
    mut include_subdomains: *mut bool,
) -> bool {
    let mut name: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    let mut value: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    let mut c_max_age: *const i8 = 0 as *const i8;
    let mut is: bool = 0 as i32 != 0;
    let mut is_url_encoded: bool = 0 as i32 != 0;
    let mut success: bool = 0 as i32 != 0;
    if !header.is_null() {
        while extract_param(
            &mut header,
            &mut name,
            &mut value,
            ';' as i32 as i8,
            &mut is_url_encoded,
        ) {
            if (name.e).offset_from(name.b) as i64 as u64
                == (::core::mem::size_of::<[i8; 8]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                && c_strncasecmp(
                    name.b,
                    b"max-age\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 8]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                ) == 0
            {
                rpl_free(c_max_age as *mut libc::c_void);
                c_max_age = 0 as *const i8;
                c_max_age = strdupdelim(value.b, value.e);
            } else if (name.e).offset_from(name.b) as i64 as u64
                == (::core::mem::size_of::<[i8; 18]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                && c_strncasecmp(
                    name.b,
                    b"includeSubDomains\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 18]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                ) == 0
            {
                is = 1 as i32 != 0;
            }
            is_url_encoded = 0 as i32 != 0;
        }
        if !c_max_age.is_null() {
            if !max_age.is_null() {
                *max_age = rpl_strtoll(c_max_age, 0 as *mut *mut i8, 10 as i32)
                    as int64_t;
            }
            if !include_subdomains.is_null() {
                *include_subdomains = is;
            }
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Parsed Strict-Transport-Security max-age = %s, includeSubDomains = %s\n\0"
                        as *const u8 as *const i8,
                    c_max_age,
                    if is as i32 != 0 {
                        b"true\0" as *const u8 as *const i8
                    } else {
                        b"false\0" as *const u8 as *const i8
                    },
                );
            }
            rpl_free(c_max_age as *mut libc::c_void);
            c_max_age = 0 as *const i8;
            success = 1 as i32 != 0;
        } else {
            logprintf(
                log_options::LOG_VERBOSE,
                b"Could not parse Strict-Transport-Security header\n\0" as *const u8
                    as *const i8,
            );
            success = 0 as i32 != 0;
        }
    }
    return success;
}
static mut pconn_active: bool = false;
static mut pconn: C2RustUnnamed_5 = C2RustUnnamed_5 {
    socket: 0,
    host: 0 as *const i8 as *mut i8,
    port: 0,
    ssl: false,
    authorized: false,
    ntlm: ntlmdata {
        state: wgetntlm::NTLMSTATE_NONE,
        nonce: [0; 8],
    },
};
unsafe extern "C" fn invalidate_persistent() {
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Disabling further reuse of socket %d.\n\0" as *const u8 as *const i8,
            pconn.socket,
        );
    }
    pconn_active = 0 as i32 != 0;
    fd_close(pconn.socket);
    rpl_free(pconn.host as *mut libc::c_void);
    pconn.host = 0 as *mut i8;
    memset(
        &mut pconn as *mut C2RustUnnamed_5 as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<C2RustUnnamed_5>() as u64,
    );
}
unsafe extern "C" fn register_persistent(
    mut host: *const i8,
    mut port: i32,
    mut fd: i32,
    mut ssl: bool,
) {
    if pconn_active {
        if pconn.socket == fd {
            return
        } else {
            invalidate_persistent();
        }
    }
    pconn_active = 1 as i32 != 0;
    pconn.socket = fd;
    pconn.host = xstrdup(host);
    pconn.port = port;
    pconn.ssl = ssl;
    pconn.authorized = 0 as i32 != 0;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Registered socket %d for persistent reuse.\n\0" as *const u8 as *const i8,
            fd,
        );
    }
}
unsafe extern "C" fn persistent_available_p(
    mut host: *const i8,
    mut port: i32,
    mut ssl: bool,
    mut host_lookup_failed: *mut bool,
) -> bool {
    if !pconn_active {
        return 0 as i32 != 0;
    }
    if ssl as i32 != pconn.ssl as i32 {
        return 0 as i32 != 0;
    }
    if port != pconn.port {
        return 0 as i32 != 0;
    }
    if 0 as i32 != strcasecmp(host, pconn.host) {
        let mut found: bool = false;
        let mut ip: ip_address = ip_address {
            family: 0,
            data: C2RustUnnamed_6 {
                d4: in_addr { s_addr: 0 },
            },
            ipv6_scope: 0,
        };
        let mut al: *mut address_list = 0 as *mut address_list;
        if ssl {
            return 0 as i32 != 0;
        }
        if !socket_ip_address(
            pconn.socket,
            &mut ip,
            C2RustUnnamed_12::ENDPOINT_PEER as i32,
        ) {
            invalidate_persistent();
            return 0 as i32 != 0;
        }
        al = lookup_host(host, 0 as i32);
        if al.is_null() {
            *host_lookup_failed = 1 as i32 != 0;
            return 0 as i32 != 0;
        }
        found = address_list_contains(al, &mut ip);
        address_list_release(al);
        if !found {
            return 0 as i32 != 0;
        }
    }
    if !test_socket_open(pconn.socket) {
        invalidate_persistent();
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn free_hstat(mut hs: *mut http_stat) {
    rpl_free((*hs).newloc as *mut libc::c_void);
    (*hs).newloc = 0 as *mut i8;
    rpl_free((*hs).remote_time as *mut libc::c_void);
    (*hs).remote_time = 0 as *mut i8;
    rpl_free((*hs).error as *mut libc::c_void);
    (*hs).error = 0 as *mut i8;
    rpl_free((*hs).rderrmsg as *mut libc::c_void);
    (*hs).rderrmsg = 0 as *mut i8;
    rpl_free((*hs).local_file as *mut libc::c_void);
    (*hs).local_file = 0 as *mut i8;
    rpl_free((*hs).orig_file_name as *mut libc::c_void);
    (*hs).orig_file_name = 0 as *mut i8;
    rpl_free((*hs).message as *mut libc::c_void);
    (*hs).message = 0 as *mut i8;
}
unsafe extern "C" fn get_file_flags(mut filename: *const i8, mut dt: *mut i32) {
    logprintf(
        log_options::LOG_VERBOSE,
        dcgettext(
            0 as *const i8,
            b"File %s already there; not retrieving.\n\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        quote(filename),
    );
    *dt |= C2RustUnnamed_4::RETROKF as i32;
    if has_html_suffix_p(filename) {
        *dt |= C2RustUnnamed_4::TEXTHTML as i32;
    }
}
unsafe extern "C" fn read_response_body(
    mut hs: *mut http_stat,
    mut sock: i32,
    mut fp: *mut FILE,
    mut contlen: wgint,
    mut contrange: wgint,
    mut chunked_transfer_encoding: bool,
    mut url: *mut i8,
    mut warc_timestamp_str: *mut i8,
    mut warc_request_uuid: *mut i8,
    mut warc_ip: *mut ip_address,
    mut type_0: *mut i8,
    mut statcode: i32,
    mut head: *mut i8,
) -> i32 {
    let mut warc_payload_offset: i32 = 0 as i32;
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut warcerr: i32 = 0 as i32;
    let mut flags: i32 = 0 as i32;
    if !(opt.warc_filename).is_null() {
        warc_tmp = warc_tempfile();
        if warc_tmp.is_null() {
            warcerr = uerr_t::WARC_TMP_FOPENERR as i32;
        }
        if warcerr == 0 as i32 {
            let mut head_len: i32 = strlen(head) as i32;
            let mut warc_tmp_written: i32 = fwrite(
                head as *const libc::c_void,
                1 as i32 as size_t,
                head_len as size_t,
                warc_tmp,
            ) as i32;
            if warc_tmp_written != head_len {
                warcerr = uerr_t::WARC_TMP_FWRITEERR as i32;
            }
            warc_payload_offset = head_len;
        }
        if warcerr != 0 as i32 {
            if !warc_tmp.is_null() {
                fclose(warc_tmp);
            }
            return warcerr;
        }
    }
    if !fp.is_null() {
        if opt.save_headers as i32 != 0 && (*hs).restval == 0 as i32 as i64 {
            fwrite(head as *const libc::c_void, 1 as i32 as size_t, strlen(head), fp);
        }
    }
    if contlen != -(1 as i32) as i64 {
        flags |= C2RustUnnamed_10::rb_read_exactly as i32;
    }
    if !fp.is_null() && (*hs).restval > 0 as i32 as i64 && contrange == 0 as i32 as i64 {
        flags |= C2RustUnnamed_10::rb_skip_startpos as i32;
    }
    if chunked_transfer_encoding {
        flags |= C2RustUnnamed_10::rb_chunked_transfer_encoding as i32;
    }
    if (*hs).remote_encoding as i32 == ENC_GZIP as i32 {
        flags |= C2RustUnnamed_10::rb_compressed_gzip as i32;
    }
    (*hs).len = (*hs).restval;
    (*hs).rd_size = 0 as i32 as wgint;
    (*hs).res = fd_read_body(
        (*hs).local_file,
        sock,
        fp,
        if contlen != -(1 as i32) as i64 { contlen } else { 0 as i32 as i64 },
        (*hs).restval,
        &mut (*hs).rd_size,
        &mut (*hs).len,
        &mut (*hs).dltime,
        flags,
        warc_tmp,
    );
    if (*hs).res >= 0 as i32 {
        if !warc_tmp.is_null() {
            let mut r: bool = warc_write_response_record(
                url,
                warc_timestamp_str,
                warc_request_uuid,
                warc_ip,
                warc_tmp,
                warc_payload_offset as off_t,
                type_0,
                statcode,
                (*hs).newloc,
            );
            if !r {
                return uerr_t::WARC_ERR as i32;
            }
        }
        return uerr_t::RETRFINISHED as i32;
    }
    if !warc_tmp.is_null() {
        fclose(warc_tmp);
    }
    if (*hs).res == -(2 as i32) {
        return uerr_t::FWRITEERR as i32
    } else if (*hs).res == -(3 as i32) {
        return uerr_t::WARC_TMP_FWRITEERR as i32
    } else {
        rpl_free((*hs).rderrmsg as *mut libc::c_void);
        (*hs).rderrmsg = 0 as *mut i8;
        (*hs).rderrmsg = xstrdup(fd_errstr(sock));
        return uerr_t::RETRFINISHED as i32;
    };
}
unsafe extern "C" fn time_to_rfc1123(
    mut time_0: time_t,
    mut buf: *mut i8,
    mut bufsize: size_t,
) -> uerr_t {
    static mut wkday: [*const i8; 7] = [
        b"Sun\0" as *const u8 as *const i8,
        b"Mon\0" as *const u8 as *const i8,
        b"Tue\0" as *const u8 as *const i8,
        b"Wed\0" as *const u8 as *const i8,
        b"Thu\0" as *const u8 as *const i8,
        b"Fri\0" as *const u8 as *const i8,
        b"Sat\0" as *const u8 as *const i8,
    ];
    static mut month: [*const i8; 12] = [
        b"Jan\0" as *const u8 as *const i8,
        b"Feb\0" as *const u8 as *const i8,
        b"Mar\0" as *const u8 as *const i8,
        b"Apr\0" as *const u8 as *const i8,
        b"May\0" as *const u8 as *const i8,
        b"Jun\0" as *const u8 as *const i8,
        b"Jul\0" as *const u8 as *const i8,
        b"Aug\0" as *const u8 as *const i8,
        b"Sep\0" as *const u8 as *const i8,
        b"Oct\0" as *const u8 as *const i8,
        b"Nov\0" as *const u8 as *const i8,
        b"Dec\0" as *const u8 as *const i8,
    ];
    let mut gtm: *mut tm = gmtime(&mut time_0);
    if gtm.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"gmtime failed. This is probably a bug.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return uerr_t::TIMECONV_ERR;
    }
    snprintf(
        buf,
        bufsize,
        b"%s, %02d %s %04d %02d:%02d:%02d GMT\0" as *const u8 as *const i8,
        wkday[(*gtm).tm_wday as usize],
        (*gtm).tm_mday,
        month[(*gtm).tm_mon as usize],
        (*gtm).tm_year + 1900 as i32,
        (*gtm).tm_hour,
        (*gtm).tm_min,
        (*gtm).tm_sec,
    );
    return uerr_t::RETROK;
}
unsafe extern "C" fn initialize_request(
    mut u: *const url,
    mut hs: *mut http_stat,
    mut dt: *mut i32,
    mut proxy: *mut url,
    mut inhibit_keep_alive: bool,
    mut basic_auth_finished: *mut bool,
    mut body_data_size: *mut wgint,
    mut user: *mut *mut i8,
    mut passwd: *mut *mut i8,
    mut ret: *mut uerr_t,
) -> *mut request {
    let mut head_only: bool = *dt & C2RustUnnamed_4::HEAD_ONLY as i32 != 0;
    let mut req: *mut request = 0 as *mut request;
    let mut meth_arg: *mut i8 = 0 as *mut i8;
    let mut meth: *const i8 = b"GET\0" as *const u8 as *const i8;
    if head_only {
        meth = b"HEAD\0" as *const u8 as *const i8;
    } else if !(opt.method).is_null() {
        meth = opt.method;
    }
    if !proxy.is_null() && (*u).scheme as u32 != url_scheme::SCHEME_HTTPS as i32 as u32 {
        meth_arg = xstrdup((*u).url);
    } else {
        meth_arg = url_full_path(u);
    }
    req = request_new(meth, meth_arg);
    static mut hfmt: [[*const i8; 2]; 2] = [
        [b"%s\0" as *const u8 as *const i8, b"[%s]\0" as *const u8 as *const i8],
        [b"%s:%d\0" as *const u8 as *const i8, b"[%s]:%d\0" as *const u8 as *const i8],
    ];
    let mut add_port: i32 = ((*u).port != scheme_default_port((*u).scheme)) as i32;
    let mut add_squares: i32 = (strchr((*u).host, ':' as i32)
        != 0 as *mut libc::c_void as *mut i8) as i32;
    request_set_header(
        req,
        b"Host\0" as *const u8 as *const i8,
        aprintf(hfmt[add_port as usize][add_squares as usize], (*u).host, (*u).port),
        rp::rel_value,
    );
    request_set_header(
        req,
        b"Referer\0" as *const u8 as *const i8,
        (*hs).referer,
        rp::rel_none,
    );
    if *dt & C2RustUnnamed_4::SEND_NOCACHE as i32 != 0 {
        request_set_header(
            req,
            b"Cache-Control\0" as *const u8 as *const i8,
            b"no-cache\0" as *const u8 as *const i8,
            rp::rel_none,
        );
        request_set_header(
            req,
            b"Pragma\0" as *const u8 as *const i8,
            b"no-cache\0" as *const u8 as *const i8,
            rp::rel_none,
        );
    }
    if *dt & C2RustUnnamed_4::IF_MODIFIED_SINCE as i32 != 0 {
        let mut strtime: [i8; 32] = [0; 32];
        let mut err: uerr_t = time_to_rfc1123(
            (*hs).orig_file_tstamp,
            strtime.as_mut_ptr(),
            (::core::mem::size_of::<[i8; 32]>() as u64)
                .wrapping_div(::core::mem::size_of::<i8>() as u64),
        );
        if err as u32 != uerr_t::RETROK as i32 as u32 {
            logputs(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Cannot convert timestamp to http format. Falling back to time 0 as last modification time.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            strcpy(
                strtime.as_mut_ptr(),
                b"Thu, 01 Jan 1970 00:00:00 GMT\0" as *const u8 as *const i8,
            );
        }
        request_set_header(
            req,
            b"If-Modified-Since\0" as *const u8 as *const i8,
            xstrdup(strtime.as_mut_ptr()),
            rp::rel_value,
        );
    }
    if (*hs).restval != 0 {
        request_set_header(
            req,
            b"Range\0" as *const u8 as *const i8,
            aprintf(
                b"bytes=%s-\0" as *const u8 as *const i8,
                number_to_static_string((*hs).restval),
            ),
            rp::rel_value,
        );
    }
    if (opt.useragent).is_null() {
        request_set_header(
            req,
            b"User-Agent\0" as *const u8 as *const i8,
            aprintf(b"Wget/%s\0" as *const u8 as *const i8, version_string),
            rp::rel_value,
        );
    } else if *opt.useragent != 0 {
        request_set_header(
            req,
            b"User-Agent\0" as *const u8 as *const i8,
            opt.useragent,
            rp::rel_none,
        );
    }
    request_set_header(
        req,
        b"Accept\0" as *const u8 as *const i8,
        b"*/*\0" as *const u8 as *const i8,
        rp::rel_none,
    );
    if opt.compression as u32 != compression_options::compression_none as i32 as u32 {
        request_set_header(
            req,
            b"Accept-Encoding\0" as *const u8 as *const i8,
            b"gzip\0" as *const u8 as *const i8,
            rp::rel_none,
        );
    } else {
        request_set_header(
            req,
            b"Accept-Encoding\0" as *const u8 as *const i8,
            b"identity\0" as *const u8 as *const i8,
            rp::rel_none,
        );
    }
    if !((*u).user).is_null() {
        *user = (*u).user;
    } else if !(opt.user).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as i32 != 0)
    {
        *user = opt.user;
    } else if !(opt.http_user).is_null() {
        *user = opt.http_user;
    } else if !(opt.user).is_null() {
        *user = opt.user;
    } else {
        *user = 0 as *mut i8;
    }
    if !((*u).passwd).is_null() {
        *passwd = (*u).passwd;
    } else if !(opt.passwd).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as i32 != 0)
    {
        *passwd = opt.passwd;
    } else if !(opt.http_passwd).is_null() {
        *passwd = opt.http_passwd;
    } else if !(opt.passwd).is_null() {
        *passwd = opt.passwd;
    } else {
        *passwd = 0 as *mut i8;
    }
    if opt.netrc as i32 != 0 && ((*user).is_null() || (*passwd).is_null()) {
        search_netrc(
            (*u).host,
            user as *mut *const i8,
            passwd as *mut *const i8,
            0 as i32,
            0 as *mut FILE,
        );
    }
    if !(*user).is_null() && !(*passwd).is_null()
        && (((*u).user).is_null() || opt.auth_without_challenge as i32 != 0)
    {
        *basic_auth_finished = maybe_send_basic_creds((*u).host, *user, *passwd, req);
    }
    if inhibit_keep_alive {
        request_set_header(
            req,
            b"Connection\0" as *const u8 as *const i8,
            b"Close\0" as *const u8 as *const i8,
            rp::rel_none,
        );
    } else {
        request_set_header(
            req,
            b"Connection\0" as *const u8 as *const i8,
            b"Keep-Alive\0" as *const u8 as *const i8,
            rp::rel_none,
        );
        if !proxy.is_null() {
            request_set_header(
                req,
                b"Proxy-Connection\0" as *const u8 as *const i8,
                b"Keep-Alive\0" as *const u8 as *const i8,
                rp::rel_none,
            );
        }
    }
    if !(opt.method).is_null() {
        if !(opt.body_data).is_null() || !(opt.body_file).is_null() {
            request_set_header(
                req,
                b"Content-Type\0" as *const u8 as *const i8,
                b"application/x-www-form-urlencoded\0" as *const u8 as *const i8,
                rp::rel_none,
            );
            if !(opt.body_data).is_null() {
                *body_data_size = strlen(opt.body_data) as wgint;
            } else {
                *body_data_size = file_size(opt.body_file);
                if *body_data_size == -(1 as i32) as i64 {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"BODY data file %s missing: %s\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        quote(opt.body_file),
                        strerror(*__errno_location()),
                    );
                    request_free(&mut req);
                    *ret = uerr_t::FILEBADFILE;
                    return 0 as *mut request;
                }
            }
            request_set_header(
                req,
                b"Content-Length\0" as *const u8 as *const i8,
                xstrdup(number_to_static_string(*body_data_size)),
                rp::rel_value,
            );
        } else if c_strcasecmp(opt.method, b"post\0" as *const u8 as *const i8)
            == 0 as i32
            || c_strcasecmp(opt.method, b"put\0" as *const u8 as *const i8) == 0 as i32
            || c_strcasecmp(opt.method, b"patch\0" as *const u8 as *const i8) == 0 as i32
        {
            request_set_header(
                req,
                b"Content-Length\0" as *const u8 as *const i8,
                b"0\0" as *const u8 as *const i8,
                rp::rel_none,
            );
        }
    }
    return req;
}
unsafe extern "C" fn initialize_proxy_configuration(
    mut u: *const url,
    mut req: *mut request,
    mut proxy: *mut url,
    mut proxyauth: *mut *mut i8,
) {
    let mut proxy_user: *mut i8 = 0 as *mut i8;
    let mut proxy_passwd: *mut i8 = 0 as *mut i8;
    if !(opt.proxy_user).is_null() && !(opt.proxy_passwd).is_null() {
        proxy_user = opt.proxy_user;
        proxy_passwd = opt.proxy_passwd;
    } else {
        proxy_user = (*proxy).user;
        proxy_passwd = (*proxy).passwd;
    }
    if !proxy_user.is_null() && !proxy_passwd.is_null() {
        *proxyauth = basic_authentication_encode(proxy_user, proxy_passwd);
    }
    if (*u).scheme as u32 != url_scheme::SCHEME_HTTPS as i32 as u32 {
        request_set_header(
            req,
            b"Proxy-Authorization\0" as *const u8 as *const i8,
            *proxyauth,
            rp::rel_value,
        );
    }
}
unsafe extern "C" fn establish_connection(
    mut u: *const url,
    mut conn_ref: *mut *const url,
    mut hs: *mut http_stat,
    mut proxy: *mut url,
    mut proxyauth: *mut *mut i8,
    mut req_ref: *mut *mut request,
    mut using_ssl: *mut bool,
    mut inhibit_keep_alive: bool,
    mut sock_ref: *mut i32,
) -> uerr_t {
    let mut host_lookup_failed: bool = 0 as i32 != 0;
    let mut sock: i32 = *sock_ref;
    let mut req: *mut request = *req_ref;
    let mut conn: *const url = *conn_ref;
    let mut resp: *mut response = 0 as *mut response;
    let mut write_error: i32 = 0;
    let mut statcode: i32 = 0;
    if !inhibit_keep_alive {
        let mut relevant: *const url = conn;
        if (*u).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32 {
            relevant = u;
        }
        if persistent_available_p(
            (*relevant).host,
            (*relevant).port,
            (*relevant).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32,
            &mut host_lookup_failed,
        ) {
            let mut family: i32 = socket_family(
                pconn.socket,
                C2RustUnnamed_12::ENDPOINT_PEER as i32,
            );
            sock = pconn.socket;
            *using_ssl = pconn.ssl;
            if family == 10 as i32 {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Reusing existing connection to [%s]:%d.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    quotearg_style(quoting_style::escape_quoting_style, pconn.host),
                    pconn.port,
                );
            } else {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Reusing existing connection to %s:%d.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    quotearg_style(quoting_style::escape_quoting_style, pconn.host),
                    pconn.port,
                );
            }
            if opt.debug as i64 != 0 {
                debug_logprintf(b"Reusing fd %d.\n\0" as *const u8 as *const i8, sock);
            }
            if pconn.authorized {
                request_remove_header(req, b"Authorization\0" as *const u8 as *const i8);
            }
        } else if host_lookup_failed {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: unable to resolve host address %s\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                exec_name,
                quote((*relevant).host),
            );
            return uerr_t::HOSTERR;
        } else if sock != -(1 as i32) {
            sock = -(1 as i32);
        }
    }
    if sock < 0 as i32 {
        sock = connect_to_host((*conn).host, (*conn).port);
        if sock == E_HOST as i32 {
            return uerr_t::HOSTERR
        } else if sock < 0 as i32 {
            return uerr_t::from_libc_c_uint(
                (if retryable_socket_connect_error(*__errno_location()) as i32 != 0 {
                    uerr_t::CONERROR as i32
                } else {
                    uerr_t::CONIMPOSSIBLE as i32
                }) as u32,
            )
        }
        if !proxy.is_null()
            && (*u).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32
        {
            's_452: {
                let mut head: *mut i8 = 0 as *mut i8;
                let mut message: *mut i8 = 0 as *mut i8;
                let mut connreq: *mut request = request_new(
                    b"CONNECT\0" as *const u8 as *const i8,
                    aprintf(b"%s:%d\0" as *const u8 as *const i8, (*u).host, (*u).port),
                );
                if (opt.useragent).is_null() {
                    request_set_header(
                        connreq,
                        b"User-Agent\0" as *const u8 as *const i8,
                        aprintf(b"Wget/%s\0" as *const u8 as *const i8, version_string),
                        rp::rel_value,
                    );
                } else if *opt.useragent != 0 {
                    request_set_header(
                        connreq,
                        b"User-Agent\0" as *const u8 as *const i8,
                        opt.useragent,
                        rp::rel_none,
                    );
                }
                if !proxyauth.is_null() {
                    request_set_header(
                        connreq,
                        b"Proxy-Authorization\0" as *const u8 as *const i8,
                        *proxyauth,
                        rp::rel_value,
                    );
                    *proxyauth = 0 as *mut i8;
                }
                request_set_header(
                    connreq,
                    b"Host\0" as *const u8 as *const i8,
                    aprintf(b"%s:%d\0" as *const u8 as *const i8, (*u).host, (*u).port),
                    rp::rel_value,
                );
                write_error = request_send(connreq, sock, 0 as *mut FILE);
                request_free(&mut connreq);
                if write_error < 0 as i32 {
                    if pconn_active as i32 != 0 && sock == pconn.socket {
                        invalidate_persistent();
                    } else {
                        fd_close(sock);
                    }
                    sock = -(1 as i32);
                    return uerr_t::WRITEFAILED;
                }
                head = read_http_response_head(sock);
                if head.is_null() {
                    logprintf(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"Failed reading proxy response: %s\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        fd_errstr(sock),
                    );
                    if pconn_active as i32 != 0 && sock == pconn.socket {
                        invalidate_persistent();
                    } else {
                        fd_close(sock);
                    }
                    sock = -(1 as i32);
                    return uerr_t::HERR;
                }
                message = 0 as *mut i8;
                if *head == 0 {
                    rpl_free(head as *mut libc::c_void);
                    head = 0 as *mut i8;
                } else {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"proxy responded with: [%s]\n\0" as *const u8 as *const i8,
                            head,
                        );
                    }
                    resp = resp_new(head);
                    statcode = resp_status(resp, &mut message);
                    if statcode < 0 as i32 {
                        let mut tms: *mut i8 = datetime_str(time(0 as *mut time_t));
                        logprintf(
                            log_options::LOG_VERBOSE,
                            b"%d\n\0" as *const u8 as *const i8,
                            statcode,
                        );
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"%s ERROR %d: %s.\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            tms,
                            statcode,
                            quotearg_style(
                                quoting_style::escape_quoting_style,
                                dcgettext(
                                    0 as *const i8,
                                    b"Malformed status line\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            ),
                        );
                        rpl_free(head as *mut libc::c_void);
                        head = 0 as *mut i8;
                        return uerr_t::HERR;
                    }
                    rpl_free((*hs).message as *mut libc::c_void);
                    (*hs).message = 0 as *mut i8;
                    (*hs).message = xstrdup(message);
                    resp_free(&mut resp);
                    rpl_free(head as *mut libc::c_void);
                    head = 0 as *mut i8;
                    if !(statcode != 200 as i32) {
                        rpl_free(message as *mut libc::c_void);
                        message = 0 as *mut i8;
                        conn = u;
                        break 's_452;
                    }
                }
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Proxy tunneling failed: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    if !message.is_null() {
                        quotearg_style(quoting_style::escape_quoting_style, message)
                    } else {
                        b"?\0" as *const u8 as *const i8
                    },
                );
                rpl_free(message as *mut libc::c_void);
                message = 0 as *mut i8;
                return uerr_t::CONSSLERR;
            }
        }
        if (*conn).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32 {
            if !ssl_connect_wget(sock, (*u).host, 0 as *mut i32) {
                if pconn_active as i32 != 0 && sock == pconn.socket {
                    invalidate_persistent();
                } else {
                    fd_close(sock);
                }
                sock = -(1 as i32);
                return uerr_t::CONSSLERR;
            } else if !ssl_check_certificate(sock, (*u).host) {
                if pconn_active as i32 != 0 && sock == pconn.socket {
                    invalidate_persistent();
                } else {
                    fd_close(sock);
                }
                sock = -(1 as i32);
                return uerr_t::VERIFCERTERR;
            }
            *using_ssl = 1 as i32 != 0;
        }
    }
    *conn_ref = conn;
    *req_ref = req;
    *sock_ref = sock;
    return uerr_t::RETROK;
}
unsafe extern "C" fn set_file_timestamp(mut hs: *mut http_stat) -> uerr_t {
    let mut local_dot_orig_file_exists: bool = 0 as i32 != 0;
    let mut local_filename: *mut i8 = 0 as *mut i8;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut buf: [i8; 1024] = [0; 1024];
    if opt.backup_converted {
        let mut filename_len: size_t = strlen((*hs).local_file);
        let mut filename_plus_orig_suffix: *mut i8 = 0 as *mut i8;
        if filename_len.wrapping_add(::core::mem::size_of::<[i8; 6]>() as u64)
            > ::core::mem::size_of::<[i8; 1024]>() as u64
        {
            filename_plus_orig_suffix = xmalloc(
                filename_len.wrapping_add(::core::mem::size_of::<[i8; 6]>() as u64),
            ) as *mut i8;
        } else {
            filename_plus_orig_suffix = buf.as_mut_ptr();
        }
        memcpy(
            filename_plus_orig_suffix as *mut libc::c_void,
            (*hs).local_file as *const libc::c_void,
            filename_len,
        );
        memcpy(
            filename_plus_orig_suffix.offset(filename_len as isize) as *mut libc::c_void,
            b".orig\0" as *const u8 as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<[i8; 6]>() as u64,
        );
        if stat(filename_plus_orig_suffix, &mut st) == 0 as i32 {
            local_dot_orig_file_exists = 1 as i32 != 0;
            local_filename = filename_plus_orig_suffix;
        }
    }
    if !local_dot_orig_file_exists {
        if stat((*hs).local_file, &mut st) == 0 as i32 {
            if local_filename != buf.as_mut_ptr() {
                rpl_free(local_filename as *mut libc::c_void);
                local_filename = 0 as *mut i8;
            }
            local_filename = (*hs).local_file;
        }
    }
    if !local_filename.is_null() {
        if local_filename == buf.as_mut_ptr() || local_filename == (*hs).local_file {
            (*hs).orig_file_name = xstrdup(local_filename);
        } else {
            (*hs).orig_file_name = local_filename;
        }
        (*hs).orig_file_size = st.st_size;
        (*hs).orig_file_tstamp = st.st_mtim.tv_sec;
        (*hs).timestamp_checked = 1 as i32 != 0;
    }
    return uerr_t::RETROK;
}
unsafe extern "C" fn check_file_output(
    mut u: *const url,
    mut hs: *mut http_stat,
    mut resp: *mut response,
    mut hdrval: *mut i8,
    mut hdrsize: size_t,
) -> uerr_t {
    if ((*hs).local_file).is_null() {
        let mut local_file: *mut i8 = 0 as *mut i8;
        if !opt.content_disposition
            || !resp_header_copy(
                resp,
                b"Content-Disposition\0" as *const u8 as *const i8,
                hdrval,
                hdrsize as i32,
            ) || !parse_content_disposition(hdrval, &mut local_file)
        {
            (*hs).local_file = url_file_name(u, 0 as *mut i8);
        } else {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Parsed filename from Content-Disposition: %s\n\0" as *const u8
                        as *const i8,
                    local_file,
                );
            }
            (*hs).local_file = url_file_name(u, local_file);
        }
        rpl_free(local_file as *mut libc::c_void);
        local_file = 0 as *mut i8;
    }
    (*hs).temporary = opt.delete_after as i32 != 0 || opt.spider as i32 != 0
        || !acceptable((*hs).local_file);
    if (*hs).temporary {
        let mut tmp: *mut i8 = aprintf(
            b"%s.tmp\0" as *const u8 as *const i8,
            (*hs).local_file,
        );
        rpl_free((*hs).local_file as *mut libc::c_void);
        (*hs).local_file = 0 as *mut i8;
        (*hs).local_file = tmp;
    }
    if !(*hs).existence_checked
        && file_exists_p((*hs).local_file, 0 as *mut file_stats_t) as i32 != 0
    {
        if opt.noclobber as i32 != 0 && (opt.output_document).is_null() {
            return uerr_t::RETRUNNEEDED
        } else if !(opt.noclobber as i32 != 0 || opt.always_rest as i32 != 0
            || opt.timestamping as i32 != 0 || opt.dirstruct as i32 != 0
            || !(opt.output_document).is_null() || opt.backups > 0 as i32)
        {
            let mut unique: *mut i8 = unique_name_passthrough((*hs).local_file);
            if unique != (*hs).local_file {
                rpl_free((*hs).local_file as *mut libc::c_void);
                (*hs).local_file = 0 as *mut i8;
            }
            (*hs).local_file = unique;
        }
    }
    (*hs).existence_checked = 1 as i32 != 0;
    if opt.timestamping as i32 != 0 && !(*hs).timestamp_checked {
        let mut timestamp_err: uerr_t = set_file_timestamp(hs);
        if timestamp_err as u32 != uerr_t::RETROK as i32 as u32 {
            return timestamp_err;
        }
    }
    return uerr_t::RETROK;
}
unsafe extern "C" fn check_auth(
    mut u: *const url,
    mut user: *mut i8,
    mut passwd: *mut i8,
    mut resp: *mut response,
    mut req: *mut request,
    mut ntlm_seen_ref: *mut bool,
    mut retry: *mut bool,
    mut basic_auth_finished_ref: *mut bool,
    mut auth_finished_ref: *mut bool,
) -> uerr_t {
    let mut auth_err: uerr_t = uerr_t::RETROK;
    let mut basic_auth_finished: bool = *basic_auth_finished_ref;
    let mut auth_finished: bool = *auth_finished_ref;
    let mut ntlm_seen: bool = *ntlm_seen_ref;
    let mut buf: [i8; 256] = [0; 256];
    let mut tmp: *mut i8 = 0 as *mut i8;
    *retry = 0 as i32 != 0;
    if !auth_finished && (!user.is_null() && !passwd.is_null()) {
        let mut wapos: i32 = 0;
        let mut www_authenticate: *const i8 = 0 as *const i8;
        let mut wabeg: *const i8 = 0 as *const i8;
        let mut waend: *const i8 = 0 as *const i8;
        let mut digest: *const i8 = 0 as *const i8;
        let mut basic: *const i8 = 0 as *const i8;
        let mut ntlm: *const i8 = 0 as *const i8;
        wapos = 0 as i32;
        while ntlm.is_null()
            && {
                wapos = resp_header_locate(
                    resp,
                    b"WWW-Authenticate\0" as *const u8 as *const i8,
                    wapos,
                    &mut wabeg,
                    &mut waend,
                );
                wapos != -(1 as i32)
            }
        {
            let mut name: param_token = param_token {
                b: 0 as *const i8,
                e: 0 as *const i8,
            };
            let mut value: param_token = param_token {
                b: 0 as *const i8,
                e: 0 as *const i8,
            };
            let mut len: size_t = waend.offset_from(wabeg) as i64 as size_t;
            if tmp != buf.as_mut_ptr() {
                rpl_free(tmp as *mut libc::c_void);
                tmp = 0 as *mut i8;
            }
            if len < ::core::mem::size_of::<[i8; 256]>() as u64 {
                tmp = buf.as_mut_ptr();
            } else {
                tmp = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
            }
            memcpy(tmp as *mut libc::c_void, wabeg as *const libc::c_void, len);
            *tmp.offset(len as isize) = 0 as i32 as i8;
            www_authenticate = tmp;
            while ntlm.is_null() {
                while c_isspace(*www_authenticate as i32) {
                    www_authenticate = www_authenticate.offset(1);
                    www_authenticate;
                }
                name.b = www_authenticate;
                name.e = name.b;
                while *name.e as i32 != 0 && !c_isspace(*name.e as i32) {
                    name.e = (name.e).offset(1);
                    name.e;
                }
                if name.b == name.e {
                    break;
                }
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Auth scheme found '%.*s'\n\0" as *const u8 as *const i8,
                        (name.e).offset_from(name.b) as i64 as i32,
                        name.b,
                    );
                }
                if known_authentication_scheme_p(name.b, name.e) {
                    if c_strncasecmp(
                        name.b,
                        b"NTLM\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 5]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                        && (c_isspace(
                            *(name.b)
                                .offset(
                                    (::core::mem::size_of::<[i8; 5]>() as u64)
                                        .wrapping_sub(1 as i32 as u64) as isize,
                                ) as i32,
                        ) as i32 != 0
                            || *(name.b)
                                .offset(
                                    (::core::mem::size_of::<[i8; 5]>() as u64)
                                        .wrapping_sub(1 as i32 as u64) as isize,
                                ) == 0)
                    {
                        ntlm = name.b;
                        break;
                    } else if digest.is_null()
                        && (c_strncasecmp(
                            name.b,
                            b"Digest\0" as *const u8 as *const i8,
                            (::core::mem::size_of::<[i8; 7]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        ) == 0
                            && (c_isspace(
                                *(name.b)
                                    .offset(
                                        (::core::mem::size_of::<[i8; 7]>() as u64)
                                            .wrapping_sub(1 as i32 as u64) as isize,
                                    ) as i32,
                            ) as i32 != 0
                                || *(name.b)
                                    .offset(
                                        (::core::mem::size_of::<[i8; 7]>() as u64)
                                            .wrapping_sub(1 as i32 as u64) as isize,
                                    ) == 0))
                    {
                        digest = name.b;
                    } else if basic.is_null()
                        && (c_strncasecmp(
                            name.b,
                            b"Basic\0" as *const u8 as *const i8,
                            (::core::mem::size_of::<[i8; 6]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        ) == 0
                            && (c_isspace(
                                *(name.b)
                                    .offset(
                                        (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64) as isize,
                                    ) as i32,
                            ) as i32 != 0
                                || *(name.b)
                                    .offset(
                                        (::core::mem::size_of::<[i8; 6]>() as u64)
                                            .wrapping_sub(1 as i32 as u64) as isize,
                                    ) == 0))
                    {
                        basic = name.b;
                    }
                }
                www_authenticate = name.e;
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Auth param list '%s'\n\0" as *const u8 as *const i8,
                        www_authenticate,
                    );
                }
                while extract_param(
                    &mut www_authenticate,
                    &mut name,
                    &mut value,
                    ',' as i32 as i8,
                    0 as *mut bool,
                ) as i32 != 0 && !(name.b).is_null() && !(value.b).is_null()
                {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"Auth param %.*s=%.*s\n\0" as *const u8 as *const i8,
                            (name.e).offset_from(name.b) as i64 as i32,
                            name.b,
                            (value.e).offset_from(value.b) as i64 as i32,
                            value.b,
                        );
                    }
                }
            }
            wapos += 1;
            wapos;
        }
        if basic.is_null() && digest.is_null() && ntlm.is_null() {
            logputs(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Unknown authentication scheme.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else if !basic_auth_finished || basic.is_null() {
            let mut pth: *mut i8 = url_full_path(u);
            let mut value_0: *const i8 = 0 as *const i8;
            let mut auth_stat: *mut uerr_t = 0 as *mut uerr_t;
            auth_stat = xmalloc(::core::mem::size_of::<uerr_t>() as u64) as *mut uerr_t;
            *auth_stat = uerr_t::RETROK;
            if !ntlm.is_null() {
                www_authenticate = ntlm;
            } else if !digest.is_null() {
                www_authenticate = digest;
            } else {
                www_authenticate = basic;
            }
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Authentication selected: %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                www_authenticate,
            );
            value_0 = create_authorization_line(
                www_authenticate,
                user,
                passwd,
                request_method(req),
                pth,
                &mut auth_finished,
                auth_stat,
            );
            auth_err = *auth_stat;
            rpl_free(auth_stat as *mut libc::c_void);
            auth_stat = 0 as *mut uerr_t;
            rpl_free(pth as *mut libc::c_void);
            pth = 0 as *mut i8;
            if auth_err as u32 == uerr_t::RETROK as i32 as u32 {
                request_set_header(
                    req,
                    b"Authorization\0" as *const u8 as *const i8,
                    value_0,
                    rp::rel_value,
                );
                if c_strncasecmp(
                    www_authenticate,
                    b"NTLM\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                ) == 0
                    && (c_isspace(
                        *www_authenticate
                            .offset(
                                (::core::mem::size_of::<[i8; 5]>() as u64)
                                    .wrapping_sub(1 as i32 as u64) as isize,
                            ) as i32,
                    ) as i32 != 0
                        || *www_authenticate
                            .offset(
                                (::core::mem::size_of::<[i8; 5]>() as u64)
                                    .wrapping_sub(1 as i32 as u64) as isize,
                            ) == 0)
                {
                    ntlm_seen = 1 as i32 != 0;
                } else if ((*u).user).is_null()
                    && (c_strncasecmp(
                        www_authenticate,
                        b"Basic\0" as *const u8 as *const i8,
                        (::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                    ) == 0
                        && (c_isspace(
                            *www_authenticate
                                .offset(
                                    (::core::mem::size_of::<[i8; 6]>() as u64)
                                        .wrapping_sub(1 as i32 as u64) as isize,
                                ) as i32,
                        ) as i32 != 0
                            || *www_authenticate
                                .offset(
                                    (::core::mem::size_of::<[i8; 6]>() as u64)
                                        .wrapping_sub(1 as i32 as u64) as isize,
                                ) == 0))
                {
                    register_basic_auth_host((*u).host);
                }
                *retry = 1 as i32 != 0;
            } else {
                rpl_free(value_0 as *mut libc::c_void);
                value_0 = 0 as *const i8;
            }
        }
    }
    if tmp != buf.as_mut_ptr() {
        rpl_free(tmp as *mut libc::c_void);
        tmp = 0 as *mut i8;
    }
    *ntlm_seen_ref = ntlm_seen;
    *basic_auth_finished_ref = basic_auth_finished;
    *auth_finished_ref = auth_finished;
    return auth_err;
}
unsafe extern "C" fn open_output_stream(
    mut hs: *mut http_stat,
    mut count: i32,
    mut fp: *mut *mut FILE,
) -> uerr_t {
    if output_stream.is_null() {
        mkalldirs((*hs).local_file);
        if opt.backups != 0 {
            rotate_backups((*hs).local_file);
        }
        if (*hs).restval != 0 {
            *fp = rpl_fopen((*hs).local_file, b"ab\0" as *const u8 as *const i8);
        } else if opt.noclobber as i32 != 0 || opt.always_rest as i32 != 0
            || opt.timestamping as i32 != 0 || opt.dirstruct as i32 != 0
            || !(opt.output_document).is_null() || opt.backups > 0 as i32
            || count > 0 as i32
        {
            if opt.unlink_requested as i32 != 0
                && file_exists_p((*hs).local_file, 0 as *mut file_stats_t) as i32 != 0
            {
                if unlink((*hs).local_file) < 0 as i32 {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        b"%s: %s\n\0" as *const u8 as *const i8,
                        (*hs).local_file,
                        strerror(*__errno_location()),
                    );
                    return uerr_t::UNLINKERR;
                }
            }
            if (*hs).temporary {
                *fp = fdopen(
                    open(
                        (*hs).local_file,
                        0 as i32 | 0o100 as i32 | 0o1000 as i32 | 0o1 as i32,
                        0o400 as i32 | 0o200 as i32,
                    ),
                    b"wb\0" as *const u8 as *const i8,
                );
            } else {
                *fp = rpl_fopen((*hs).local_file, b"wb\0" as *const u8 as *const i8);
            }
        } else {
            *fp = fopen_excl((*hs).local_file, 1 as i32);
            if (*fp).is_null() && *__errno_location() == 17 as i32 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"%s has sprung into existence.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*hs).local_file,
                );
                return uerr_t::FOPEN_EXCL_ERR;
            }
        }
        if (*fp).is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s: %s\n\0" as *const u8 as *const i8,
                (*hs).local_file,
                strerror(*__errno_location()),
            );
            return uerr_t::FOPENERR;
        }
    } else {
        *fp = output_stream;
    }
    logprintf(
        log_options::LOG_VERBOSE,
        dcgettext(
            0 as *const i8,
            b"Saving to: %s\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        if *(*hs).local_file as i32 == '-' as i32
            && *((*hs).local_file).offset(1 as i32 as isize) == 0
        {
            quote(b"STDOUT\0" as *const u8 as *const i8)
        } else {
            quote((*hs).local_file)
        },
    );
    return uerr_t::RETROK;
}
unsafe extern "C" fn set_content_type(mut dt: *mut i32, mut type_0: *const i8) {
    if type_0.is_null()
        || 0 as i32 == c_strcasecmp(type_0, b"text/html\0" as *const u8 as *const i8)
        || 0 as i32
            == c_strcasecmp(type_0, b"application/xhtml+xml\0" as *const u8 as *const i8)
    {
        *dt |= C2RustUnnamed_4::TEXTHTML as i32;
    } else {
        *dt &= !(C2RustUnnamed_4::TEXTHTML as i32);
    }
    if !type_0.is_null()
        && 0 as i32 == c_strcasecmp(type_0, b"text/css\0" as *const u8 as *const i8)
    {
        *dt |= C2RustUnnamed_4::TEXTCSS as i32;
    } else {
        *dt &= !(C2RustUnnamed_4::TEXTCSS as i32);
    };
}
unsafe extern "C" fn gethttp(
    mut u: *const url,
    mut original_url: *mut url,
    mut hs: *mut http_stat,
    mut dt: *mut i32,
    mut proxy: *mut url,
    mut iri: *mut iri,
    mut count: i32,
) -> uerr_t {
    let mut ret_0: uerr_t = uerr_t::NOCONERROR;
    let mut current_block: u64;
    let mut req: *mut request = 0 as *mut request;
    let mut type_0: *mut i8 = 0 as *mut i8;
    let mut user: *mut i8 = 0 as *mut i8;
    let mut passwd: *mut i8 = 0 as *mut i8;
    let mut proxyauth: *mut i8 = 0 as *mut i8;
    let mut statcode: i32 = 0;
    let mut write_error: i32 = 0;
    let mut contlen: wgint = 0;
    let mut contrange: wgint = 0;
    let mut conn: *const url = 0 as *const url;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut err: i32 = 0;
    let mut retval: uerr_t = uerr_t::NOCONERROR;
    extern "C" {
        static mut hsts_store: hsts_store_t;
    }
    let mut sock: i32 = -(1 as i32);
    let mut auth_finished: bool = 0 as i32 != 0;
    let mut basic_auth_finished: bool = 0 as i32 != 0;
    let mut ntlm_seen: bool = 0 as i32 != 0;
    let mut using_ssl: bool = 0 as i32 != 0;
    let mut head_only: bool = *dt & C2RustUnnamed_4::HEAD_ONLY as i32 != 0;
    let mut cond_get: bool = *dt & C2RustUnnamed_4::IF_MODIFIED_SINCE as i32 != 0;
    let mut head: *mut i8 = 0 as *mut i8;
    let mut resp: *mut response = 0 as *mut response;
    let mut hdrval: [i8; 512] = [0; 512];
    let mut message: *mut i8 = 0 as *mut i8;
    let mut warc_enabled: bool = !(opt.warc_filename).is_null();
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut warc_timestamp_str: [i8; 21] = [0; 21];
    let mut warc_request_uuid: [i8; 48] = [0; 48];
    let mut warc_ip_buf: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_6 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut warc_ip: *mut ip_address = 0 as *mut ip_address;
    let mut warc_payload_offset: off_t = -(1 as i32) as off_t;
    let mut keep_alive: bool = false;
    let mut chunked_transfer_encoding: bool = 0 as i32 != 0;
    let mut inhibit_keep_alive: bool = !opt.http_keep_alive
        || opt.ignore_length as i32 != 0;
    let mut body_data_size: wgint = 0 as i32 as wgint;
    if (*u).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32 {
        if !ssl_init() {
            scheme_disable(url_scheme::SCHEME_HTTPS);
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Disabling SSL due to encountered errors.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            retval = uerr_t::SSLINITFAILED;
            current_block = 16779068821653568252;
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    match current_block {
        17833034027772472439 => {
            (*hs).len = 0 as i32 as wgint;
            (*hs).contlen = -(1 as i32) as wgint;
            (*hs).res = -(1 as i32);
            rpl_free((*hs).rderrmsg as *mut libc::c_void);
            (*hs).rderrmsg = 0 as *mut i8;
            rpl_free((*hs).newloc as *mut libc::c_void);
            (*hs).newloc = 0 as *mut i8;
            rpl_free((*hs).remote_time as *mut libc::c_void);
            (*hs).remote_time = 0 as *mut i8;
            rpl_free((*hs).error as *mut libc::c_void);
            (*hs).error = 0 as *mut i8;
            rpl_free((*hs).message as *mut libc::c_void);
            (*hs).message = 0 as *mut i8;
            (*hs).local_encoding = ENC_NONE;
            (*hs).remote_encoding = ENC_NONE;
            conn = u;
            let mut ret: uerr_t = uerr_t::NOCONERROR;
            req = initialize_request(
                u,
                hs,
                dt,
                proxy,
                inhibit_keep_alive,
                &mut basic_auth_finished,
                &mut body_data_size,
                &mut user,
                &mut passwd,
                &mut ret,
            );
            if req.is_null() {
                retval = ret;
            } else {
                '_retry_with_auth: loop {
                    if opt.cookies {
                        request_set_header(
                            req,
                            b"Cookie\0" as *const u8 as *const i8,
                            cookie_header(
                                wget_cookie_jar,
                                (*u).host,
                                (*u).port,
                                (*u).path,
                                (*u).scheme as u32 == url_scheme::SCHEME_HTTPS as i32 as u32,
                            ),
                            rp::rel_value,
                        );
                    }
                    if !(opt.user_headers).is_null() {
                        let mut i: i32 = 0;
                        i = 0 as i32;
                        while !(*(opt.user_headers).offset(i as isize)).is_null() {
                            request_set_user_header(
                                req,
                                *(opt.user_headers).offset(i as isize),
                            );
                            i += 1;
                            i;
                        }
                    }
                    proxyauth = 0 as *mut i8;
                    if !proxy.is_null() {
                        conn = proxy;
                        initialize_proxy_configuration(u, req, proxy, &mut proxyauth);
                    }
                    keep_alive = 1 as i32 != 0;
                    if inhibit_keep_alive {
                        keep_alive = 0 as i32 != 0;
                    }
                    let mut conn_err: uerr_t = establish_connection(
                        u,
                        &mut conn,
                        hs,
                        proxy,
                        &mut proxyauth,
                        &mut req,
                        &mut using_ssl,
                        inhibit_keep_alive,
                        &mut sock,
                    );
                    if conn_err as u32 != uerr_t::RETROK as i32 as u32 {
                        retval = conn_err;
                        current_block = 16779068821653568252;
                        break;
                    } else {
                        if warc_enabled {
                            warc_tmp = warc_tempfile();
                            if warc_tmp.is_null() {
                                if pconn_active as i32 != 0 && sock == pconn.socket {
                                    invalidate_persistent();
                                } else {
                                    fd_close(sock);
                                }
                                sock = -(1 as i32);
                                retval = uerr_t::WARC_TMP_FOPENERR;
                                current_block = 16779068821653568252;
                                break;
                            } else if proxy.is_null() {
                                warc_ip = &mut warc_ip_buf;
                                socket_ip_address(
                                    sock,
                                    warc_ip,
                                    C2RustUnnamed_12::ENDPOINT_PEER as i32,
                                );
                            }
                        }
                        write_error = request_send(req, sock, warc_tmp);
                        if write_error >= 0 as i32 {
                            if !(opt.body_data).is_null() {
                                if opt.debug as i64 != 0 {
                                    debug_logprintf(
                                        b"[BODY data: %s]\n\0" as *const u8 as *const i8,
                                        opt.body_data,
                                    );
                                }
                                write_error = fd_write(
                                    sock,
                                    opt.body_data,
                                    body_data_size as i32,
                                    -(1 as i32) as libc::c_double,
                                );
                                if write_error >= 0 as i32 && !warc_tmp.is_null() {
                                    let mut warc_tmp_written: i32 = 0;
                                    warc_payload_offset = ftello(warc_tmp);
                                    warc_tmp_written = fwrite(
                                        opt.body_data as *const libc::c_void,
                                        1 as i32 as size_t,
                                        body_data_size as size_t,
                                        warc_tmp,
                                    ) as i32;
                                    if warc_tmp_written as i64 != body_data_size {
                                        write_error = -(2 as i32);
                                    }
                                }
                            } else if !(opt.body_file).is_null()
                                && body_data_size != 0 as i32 as i64
                            {
                                if !warc_tmp.is_null() {
                                    warc_payload_offset = ftello(warc_tmp);
                                }
                                write_error = body_file_send(
                                    sock,
                                    opt.body_file,
                                    body_data_size,
                                    warc_tmp,
                                );
                            }
                        }
                        if write_error < 0 as i32 {
                            if pconn_active as i32 != 0 && sock == pconn.socket {
                                invalidate_persistent();
                            } else {
                                fd_close(sock);
                            }
                            sock = -(1 as i32);
                            if !warc_tmp.is_null() {
                                fclose(warc_tmp);
                            }
                            if write_error == -(2 as i32) {
                                retval = uerr_t::WARC_TMP_FWRITEERR;
                            } else {
                                retval = uerr_t::WRITEFAILED;
                            }
                            current_block = 16779068821653568252;
                            break;
                        } else {
                            logprintf(
                                log_options::LOG_VERBOSE,
                                dcgettext(
                                    0 as *const i8,
                                    b"%s request sent, awaiting response... \0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                if !proxy.is_null() {
                                    b"Proxy\0" as *const u8 as *const i8
                                } else {
                                    b"HTTP\0" as *const u8 as *const i8
                                },
                            );
                            contlen = -(1 as i32) as wgint;
                            contrange = 0 as i32 as wgint;
                            *dt &= !(C2RustUnnamed_4::RETROKF as i32);
                            if warc_enabled {
                                let mut warc_result: bool = false;
                                warc_timestamp(
                                    warc_timestamp_str.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 21]>() as u64,
                                );
                                warc_uuid_str(
                                    warc_request_uuid.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 48]>() as u64,
                                );
                                warc_result = warc_write_request_record(
                                    (*u).url,
                                    warc_timestamp_str.as_mut_ptr(),
                                    warc_request_uuid.as_mut_ptr(),
                                    warc_ip,
                                    warc_tmp,
                                    warc_payload_offset,
                                );
                                if !warc_result {
                                    if pconn_active as i32 != 0 && sock == pconn.socket {
                                        invalidate_persistent();
                                    } else {
                                        fd_close(sock);
                                    }
                                    sock = -(1 as i32);
                                    retval = uerr_t::WARC_ERR;
                                    current_block = 16779068821653568252;
                                    break;
                                }
                            }
                            let mut _repeat: bool = false;
                            loop {
                                head = read_http_response_head(sock);
                                if head.is_null() {
                                    if *__errno_location() == 0 as i32 {
                                        logputs(
                                            log_options::LOG_NOTQUIET,
                                            dcgettext(
                                                0 as *const i8,
                                                b"No data received.\n\0" as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                            invalidate_persistent();
                                        } else {
                                            fd_close(sock);
                                        }
                                        sock = -(1 as i32);
                                        retval = uerr_t::HEOF;
                                    } else {
                                        logprintf(
                                            log_options::LOG_NOTQUIET,
                                            dcgettext(
                                                0 as *const i8,
                                                b"Read error (%s) in headers.\n\0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            fd_errstr(sock),
                                        );
                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                            invalidate_persistent();
                                        } else {
                                            fd_close(sock);
                                        }
                                        sock = -(1 as i32);
                                        retval = uerr_t::HERR;
                                    }
                                    current_block = 16779068821653568252;
                                    break '_retry_with_auth;
                                } else {
                                    if opt.debug as i64 != 0 {
                                        debug_logprintf(
                                            b"\n---response begin---\n%s---response end---\n\0"
                                                as *const u8 as *const i8,
                                            head,
                                        );
                                    }
                                    resp = resp_new(head);
                                    rpl_free(message as *mut libc::c_void);
                                    message = 0 as *mut i8;
                                    statcode = resp_status(resp, &mut message);
                                    if statcode < 0 as i32 {
                                        let mut tms: *mut i8 = datetime_str(time(0 as *mut time_t));
                                        logprintf(
                                            log_options::LOG_VERBOSE,
                                            b"%d\n\0" as *const u8 as *const i8,
                                            statcode,
                                        );
                                        logprintf(
                                            log_options::LOG_NOTQUIET,
                                            dcgettext(
                                                0 as *const i8,
                                                b"%s ERROR %d: %s.\n\0" as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                            tms,
                                            statcode,
                                            quotearg_style(
                                                quoting_style::escape_quoting_style,
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"Malformed status line\0" as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                            ),
                                        );
                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                            invalidate_persistent();
                                        } else {
                                            fd_close(sock);
                                        }
                                        sock = -(1 as i32);
                                        retval = uerr_t::HERR;
                                        current_block = 16779068821653568252;
                                        break '_retry_with_auth;
                                    } else {
                                        if statcode >= 100 as i32 && statcode < 200 as i32 {
                                            rpl_free(head as *mut libc::c_void);
                                            head = 0 as *mut i8;
                                            resp_free(&mut resp);
                                            _repeat = 1 as i32 != 0;
                                            if opt.debug as i64 != 0 {
                                                debug_logprintf(
                                                    b"Ignoring response\n\0" as *const u8 as *const i8,
                                                );
                                            }
                                        } else {
                                            _repeat = 0 as i32 != 0;
                                        }
                                        if !_repeat {
                                            break;
                                        }
                                    }
                                }
                            }
                            rpl_free((*hs).message as *mut libc::c_void);
                            (*hs).message = 0 as *mut i8;
                            (*hs).message = xstrdup(message);
                            if !opt.server_response {
                                logprintf(
                                    log_options::LOG_VERBOSE,
                                    b"%2d %s\n\0" as *const u8 as *const i8,
                                    statcode,
                                    if !message.is_null() {
                                        quotearg_style(quoting_style::escape_quoting_style, message)
                                    } else {
                                        b"\0" as *const u8 as *const i8
                                    },
                                );
                            } else {
                                logprintf(
                                    log_options::LOG_VERBOSE,
                                    b"\n\0" as *const u8 as *const i8,
                                );
                                print_server_response(
                                    resp,
                                    b"  \0" as *const u8 as *const i8,
                                );
                            }
                            if !opt.ignore_length
                                && resp_header_copy(
                                    resp,
                                    b"Content-Length\0" as *const u8 as *const i8,
                                    hdrval.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 512]>() as u64 as i32,
                                ) as i32 != 0
                            {
                                let mut parsed: wgint = 0;
                                *__errno_location() = 0 as i32;
                                parsed = rpl_strtoll(
                                    hdrval.as_mut_ptr(),
                                    0 as *mut *mut i8,
                                    10 as i32,
                                ) as wgint;
                                if parsed == 9223372036854775807 as i64
                                    && *__errno_location() == 34 as i32
                                {
                                    contlen = -(1 as i32) as wgint;
                                } else if parsed < 0 as i32 as i64 {
                                    contlen = -(1 as i32) as wgint;
                                } else {
                                    contlen = parsed;
                                }
                            }
                            if !inhibit_keep_alive {
                                if resp_header_copy(
                                    resp,
                                    b"Connection\0" as *const u8 as *const i8,
                                    hdrval.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 512]>() as u64 as i32,
                                ) {
                                    if 0 as i32
                                        == c_strcasecmp(
                                            hdrval.as_mut_ptr(),
                                            b"Close\0" as *const u8 as *const i8,
                                        )
                                    {
                                        keep_alive = 0 as i32 != 0;
                                    }
                                }
                            }
                            chunked_transfer_encoding = 0 as i32 != 0;
                            if resp_header_copy(
                                resp,
                                b"Transfer-Encoding\0" as *const u8 as *const i8,
                                hdrval.as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 512]>() as u64 as i32,
                            ) as i32 != 0
                                && 0 as i32
                                    == c_strcasecmp(
                                        hdrval.as_mut_ptr(),
                                        b"chunked\0" as *const u8 as *const i8,
                                    )
                            {
                                chunked_transfer_encoding = 1 as i32 != 0;
                            }
                            if opt.cookies {
                                let mut scpos: i32 = 0;
                                let mut scbeg: *const i8 = 0 as *const i8;
                                let mut scend: *const i8 = 0 as *const i8;
                                scpos = 0 as i32;
                                loop {
                                    scpos = resp_header_locate(
                                        resp,
                                        b"Set-Cookie\0" as *const u8 as *const i8,
                                        scpos,
                                        &mut scbeg,
                                        &mut scend,
                                    );
                                    if !(scpos != -(1 as i32)) {
                                        break;
                                    }
                                    let mut buf: [i8; 1024] = [0; 1024];
                                    let mut set_cookie: *mut i8 = 0 as *mut i8;
                                    let mut len: size_t = scend.offset_from(scbeg) as i64
                                        as size_t;
                                    if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
                                        set_cookie = buf.as_mut_ptr();
                                    } else {
                                        set_cookie = xmalloc(len.wrapping_add(1 as i32 as u64))
                                            as *mut i8;
                                    }
                                    memcpy(
                                        set_cookie as *mut libc::c_void,
                                        scbeg as *const libc::c_void,
                                        len,
                                    );
                                    *set_cookie.offset(len as isize) = 0 as i32 as i8;
                                    cookie_handle_set_cookie(
                                        wget_cookie_jar,
                                        (*u).host,
                                        (*u).port,
                                        (*u).path,
                                        set_cookie,
                                    );
                                    if set_cookie != buf.as_mut_ptr() {
                                        rpl_free(set_cookie as *mut libc::c_void);
                                        set_cookie = 0 as *mut i8;
                                    }
                                    scpos += 1;
                                    scpos;
                                }
                            }
                            if keep_alive {
                                register_persistent(
                                    (*conn).host,
                                    (*conn).port,
                                    sock,
                                    using_ssl,
                                );
                            }
                            if statcode == 401 as i32 {
                                let mut auth_err: uerr_t = uerr_t::RETROK;
                                let mut retry: bool = false;
                                if warc_enabled {
                                    let mut _err: i32 = 0;
                                    type_0 = resp_header_strdup(
                                        resp,
                                        b"Content-Type\0" as *const u8 as *const i8,
                                    );
                                    _err = read_response_body(
                                        hs,
                                        sock,
                                        0 as *mut FILE,
                                        contlen,
                                        0 as i32 as wgint,
                                        chunked_transfer_encoding,
                                        (*u).url,
                                        warc_timestamp_str.as_mut_ptr(),
                                        warc_request_uuid.as_mut_ptr(),
                                        warc_ip,
                                        type_0,
                                        statcode,
                                        head,
                                    );
                                    rpl_free(type_0 as *mut libc::c_void);
                                    type_0 = 0 as *mut i8;
                                    if _err != uerr_t::RETRFINISHED as i32
                                        || (*hs).res < 0 as i32
                                    {
                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                            invalidate_persistent();
                                        } else {
                                            fd_close(sock);
                                        }
                                        sock = -(1 as i32);
                                        retval = uerr_t::from_libc_c_uint(_err as u32);
                                        current_block = 16779068821653568252;
                                        break;
                                    } else if !keep_alive {
                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                            invalidate_persistent();
                                        } else {
                                            fd_close(sock);
                                        }
                                        sock = -(1 as i32);
                                    }
                                } else if keep_alive as i32 != 0 && !head_only
                                    && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                        as i32 != 0
                                {
                                    if !keep_alive {
                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                            invalidate_persistent();
                                        } else {
                                            fd_close(sock);
                                        }
                                        sock = -(1 as i32);
                                    }
                                } else {
                                    if pconn_active as i32 != 0 && sock == pconn.socket {
                                        invalidate_persistent();
                                    } else {
                                        fd_close(sock);
                                    }
                                    sock = -(1 as i32);
                                }
                                pconn.authorized = 0 as i32 != 0;
                                auth_err = check_auth(
                                    u,
                                    user,
                                    passwd,
                                    resp,
                                    req,
                                    &mut ntlm_seen,
                                    &mut retry,
                                    &mut basic_auth_finished,
                                    &mut auth_finished,
                                );
                                if auth_err as u32 == uerr_t::RETROK as i32 as u32
                                    && retry as i32 != 0
                                {
                                    resp_free(&mut resp);
                                    rpl_free(message as *mut libc::c_void);
                                    message = 0 as *mut i8;
                                    rpl_free(head as *mut libc::c_void);
                                    head = 0 as *mut i8;
                                } else {
                                    if auth_err as u32 == uerr_t::RETROK as i32 as u32 {
                                        retval = uerr_t::AUTHFAILED;
                                    } else {
                                        retval = auth_err;
                                    }
                                    current_block = 16779068821653568252;
                                    break;
                                }
                            } else {
                                if ntlm_seen {
                                    pconn.authorized = 1 as i32 != 0;
                                }
                                ret_0 = check_file_output(
                                    u,
                                    hs,
                                    resp,
                                    hdrval.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 512]>() as u64,
                                );
                                if ret_0 as u32 != uerr_t::RETROK as i32 as u32 {
                                    current_block = 11591941514213818729;
                                    break;
                                } else {
                                    current_block = 2493083811365744214;
                                    break;
                                }
                            }
                        }
                    }
                }
                match current_block {
                    16779068821653568252 => {}
                    _ => {
                        match current_block {
                            11591941514213818729 => {
                                retval = ret_0;
                            }
                            _ => {
                                (*hs).statcode = statcode;
                                rpl_free((*hs).error as *mut libc::c_void);
                                (*hs).error = 0 as *mut i8;
                                if statcode == -(1 as i32) {
                                    (*hs).error = xstrdup(
                                        dcgettext(
                                            0 as *const i8,
                                            b"Malformed status line\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                } else if message.is_null() || *message == 0 {
                                    (*hs).error = xstrdup(
                                        dcgettext(
                                            0 as *const i8,
                                            b"(no description)\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                } else {
                                    (*hs).error = xstrdup(message);
                                }
                                if opt.hsts as i32 != 0 && !hsts_store.is_null() {
                                    let mut max_age: int64_t = 0;
                                    let mut hsts_params: *const i8 = resp_header_strdup(
                                        resp,
                                        b"Strict-Transport-Security\0" as *const u8 as *const i8,
                                    );
                                    let mut include_subdomains: bool = false;
                                    if parse_strict_transport_security(
                                        hsts_params,
                                        &mut max_age,
                                        &mut include_subdomains,
                                    ) {
                                        if hsts_store_entry(
                                            hsts_store,
                                            (*u).scheme,
                                            (*u).host,
                                            (*u).port,
                                            max_age,
                                            include_subdomains,
                                        ) {
                                            if opt.debug as i64 != 0 {
                                                debug_logprintf(
                                                    b"Added new HSTS host: %s:%u (max-age: %ld, includeSubdomains: %s)\n\0"
                                                        as *const u8 as *const i8,
                                                    (*u).host,
                                                    (*u).port as uint32_t,
                                                    max_age,
                                                    if include_subdomains as i32 != 0 {
                                                        b"true\0" as *const u8 as *const i8
                                                    } else {
                                                        b"false\0" as *const u8 as *const i8
                                                    },
                                                );
                                            }
                                        } else if opt.debug as i64 != 0 {
                                            debug_logprintf(
                                                b"Updated HSTS host: %s:%u (max-age: %ld, includeSubdomains: %s)\n\0"
                                                    as *const u8 as *const i8,
                                                (*u).host,
                                                (*u).port as uint32_t,
                                                max_age,
                                                if include_subdomains as i32 != 0 {
                                                    b"true\0" as *const u8 as *const i8
                                                } else {
                                                    b"false\0" as *const u8 as *const i8
                                                },
                                            );
                                        }
                                    }
                                    rpl_free(hsts_params as *mut libc::c_void);
                                    hsts_params = 0 as *const i8;
                                }
                                type_0 = resp_header_strdup(
                                    resp,
                                    b"Content-Type\0" as *const u8 as *const i8,
                                );
                                if !type_0.is_null() {
                                    let mut tmp: *mut i8 = strchr(type_0, ';' as i32);
                                    if !tmp.is_null() {
                                        let mut tmp2: *mut i8 = tmp.offset(1 as i32 as isize);
                                        while tmp > type_0
                                            && c_isspace(*tmp.offset(-(1 as i32) as isize) as i32)
                                                as i32 != 0
                                        {
                                            tmp = tmp.offset(-1);
                                            tmp;
                                        }
                                        *tmp = '\0' as i32 as i8;
                                        if opt.enable_iri as i32 != 0
                                            && (opt.encoding_remote).is_null()
                                        {
                                            tmp = parse_charset(tmp2);
                                            if !tmp.is_null() {
                                                set_content_encoding(iri, tmp);
                                            }
                                            rpl_free(tmp as *mut libc::c_void);
                                            tmp = 0 as *mut i8;
                                        }
                                    }
                                }
                                rpl_free((*hs).newloc as *mut libc::c_void);
                                (*hs).newloc = 0 as *mut i8;
                                (*hs).newloc = resp_header_strdup(
                                    resp,
                                    b"Location\0" as *const u8 as *const i8,
                                );
                                rpl_free((*hs).remote_time as *mut libc::c_void);
                                (*hs).remote_time = 0 as *mut i8;
                                (*hs).remote_time = resp_header_strdup(
                                    resp,
                                    b"Last-Modified\0" as *const u8 as *const i8,
                                );
                                if ((*hs).remote_time).is_null() {
                                    (*hs).remote_time = resp_header_strdup(
                                        resp,
                                        b"X-Archive-Orig-last-modified\0" as *const u8 as *const i8,
                                    );
                                }
                                if resp_header_copy(
                                    resp,
                                    b"Content-Range\0" as *const u8 as *const i8,
                                    hdrval.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 512]>() as u64 as i32,
                                ) {
                                    let mut first_byte_pos: wgint = 0;
                                    let mut last_byte_pos: wgint = 0;
                                    let mut entity_length: wgint = 0;
                                    if parse_content_range(
                                        hdrval.as_mut_ptr(),
                                        &mut first_byte_pos,
                                        &mut last_byte_pos,
                                        &mut entity_length,
                                    ) {
                                        contrange = first_byte_pos;
                                        contlen = last_byte_pos - first_byte_pos + 1 as i32 as i64;
                                    }
                                }
                                if resp_header_copy(
                                    resp,
                                    b"Content-Encoding\0" as *const u8 as *const i8,
                                    hdrval.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 512]>() as u64 as i32,
                                ) {
                                    (*hs).local_encoding = ENC_INVALID;
                                    match hdrval[0 as i32 as usize] as i32 {
                                        98 | 66 => {
                                            if 0 as i32
                                                == c_strcasecmp(
                                                    hdrval.as_mut_ptr(),
                                                    b"br\0" as *const u8 as *const i8,
                                                )
                                            {
                                                (*hs).local_encoding = ENC_BROTLI;
                                            }
                                        }
                                        99 | 67 => {
                                            if 0 as i32
                                                == c_strcasecmp(
                                                    hdrval.as_mut_ptr(),
                                                    b"compress\0" as *const u8 as *const i8,
                                                )
                                            {
                                                (*hs).local_encoding = ENC_COMPRESS;
                                            }
                                        }
                                        100 | 68 => {
                                            if 0 as i32
                                                == c_strcasecmp(
                                                    hdrval.as_mut_ptr(),
                                                    b"deflate\0" as *const u8 as *const i8,
                                                )
                                            {
                                                (*hs).local_encoding = ENC_DEFLATE;
                                            }
                                        }
                                        103 | 71 => {
                                            if 0 as i32
                                                == c_strcasecmp(
                                                    hdrval.as_mut_ptr(),
                                                    b"gzip\0" as *const u8 as *const i8,
                                                )
                                            {
                                                (*hs).local_encoding = ENC_GZIP;
                                            }
                                        }
                                        105 | 73 => {
                                            if 0 as i32
                                                == c_strcasecmp(
                                                    hdrval.as_mut_ptr(),
                                                    b"identity\0" as *const u8 as *const i8,
                                                )
                                            {
                                                (*hs).local_encoding = ENC_NONE;
                                            }
                                        }
                                        120 | 88 => {
                                            if 0 as i32
                                                == c_strcasecmp(
                                                    hdrval.as_mut_ptr(),
                                                    b"x-compress\0" as *const u8 as *const i8,
                                                )
                                            {
                                                (*hs).local_encoding = ENC_COMPRESS;
                                            } else if 0 as i32
                                                == c_strcasecmp(
                                                    hdrval.as_mut_ptr(),
                                                    b"x-gzip\0" as *const u8 as *const i8,
                                                )
                                            {
                                                (*hs).local_encoding = ENC_GZIP;
                                            }
                                        }
                                        0 => {
                                            (*hs).local_encoding = ENC_NONE;
                                        }
                                        _ => {}
                                    }
                                    if (*hs).local_encoding as i32 == ENC_INVALID as i32 {
                                        if opt.debug as i64 != 0 {
                                            debug_logprintf(
                                                b"Unrecognized Content-Encoding: %s\n\0" as *const u8
                                                    as *const i8,
                                                hdrval.as_mut_ptr(),
                                            );
                                        }
                                        (*hs).local_encoding = ENC_NONE;
                                    } else if (*hs).local_encoding as i32 == ENC_GZIP as i32
                                        && opt.compression as u32
                                            != compression_options::compression_none as i32 as u32
                                    {
                                        let mut p: *const i8 = 0 as *const i8;
                                        if !type_0.is_null() {
                                            p = strchr(type_0, '/' as i32);
                                            if p.is_null() {
                                                (*hs).remote_encoding = ENC_GZIP;
                                                (*hs).local_encoding = ENC_NONE;
                                            } else {
                                                p = p.offset(1);
                                                p;
                                                if c_tolower(*p.offset(0 as i32 as isize) as i32)
                                                    == 'x' as i32
                                                    && *p.offset(1 as i32 as isize) as i32 == '-' as i32
                                                {
                                                    p = p.offset(2 as i32 as isize);
                                                }
                                                if 0 as i32
                                                    != c_strcasecmp(p, b"gzip\0" as *const u8 as *const i8)
                                                {
                                                    (*hs).remote_encoding = ENC_GZIP;
                                                    (*hs).local_encoding = ENC_NONE;
                                                }
                                            }
                                        } else {
                                            (*hs).remote_encoding = ENC_GZIP;
                                            (*hs).local_encoding = ENC_NONE;
                                        }
                                        if (*hs).remote_encoding as i32 == ENC_GZIP as i32
                                            && {
                                                p = strrchr((*u).file, '.' as i32);
                                                !p.is_null()
                                            }
                                            && (c_strcasecmp(p, b".gz\0" as *const u8 as *const i8)
                                                == 0 as i32
                                                || c_strcasecmp(p, b".tgz\0" as *const u8 as *const i8)
                                                    == 0 as i32)
                                        {
                                            if opt.debug as i64 != 0 {
                                                debug_logprintf(
                                                    b"Enabling broken server workaround. Will not decompress this GZip file.\n\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                            (*hs).remote_encoding = ENC_NONE;
                                        }
                                    }
                                }
                                if statcode >= 200 as i32 && statcode < 300 as i32 {
                                    *dt |= C2RustUnnamed_4::RETROKF as i32;
                                }
                                if statcode == 204 as i32 {
                                    (*hs).len = 0 as i32 as wgint;
                                    (*hs).res = 0 as i32;
                                    (*hs).restval = 0 as i32 as wgint;
                                    if !keep_alive {
                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                            invalidate_persistent();
                                        } else {
                                            fd_close(sock);
                                        }
                                        sock = -(1 as i32);
                                    }
                                    retval = uerr_t::RETRFINISHED;
                                } else {
                                    if statcode == 301 as i32 || statcode == 302 as i32
                                        || statcode == 303 as i32 || statcode == 307 as i32
                                        || statcode == 308 as i32 || statcode == 300 as i32
                                    {
                                        if statcode == 300 as i32 && ((*hs).newloc).is_null() {
                                            *dt |= C2RustUnnamed_4::RETROKF as i32;
                                            current_block = 17222258012332649691;
                                        } else {
                                            logprintf(
                                                log_options::LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"Location: %s%s\n\0" as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                                if !((*hs).newloc).is_null() {
                                                    escnonprint_uri((*hs).newloc)
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"unspecified\0" as *const u8 as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                if !((*hs).newloc).is_null() {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b" [following]\0" as *const u8 as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    b"\0" as *const u8 as *const i8
                                                },
                                            );
                                            (*hs).len = 0 as i32 as wgint;
                                            (*hs).res = 0 as i32;
                                            (*hs).restval = 0 as i32 as wgint;
                                            if warc_enabled {
                                                let mut _err_0: i32 = read_response_body(
                                                    hs,
                                                    sock,
                                                    0 as *mut FILE,
                                                    contlen,
                                                    0 as i32 as wgint,
                                                    chunked_transfer_encoding,
                                                    (*u).url,
                                                    warc_timestamp_str.as_mut_ptr(),
                                                    warc_request_uuid.as_mut_ptr(),
                                                    warc_ip,
                                                    type_0,
                                                    statcode,
                                                    head,
                                                );
                                                if _err_0 != uerr_t::RETRFINISHED as i32
                                                    || (*hs).res < 0 as i32
                                                {
                                                    if pconn_active as i32 != 0 && sock == pconn.socket {
                                                        invalidate_persistent();
                                                    } else {
                                                        fd_close(sock);
                                                    }
                                                    sock = -(1 as i32);
                                                    retval = uerr_t::from_libc_c_uint(_err_0 as u32);
                                                    current_block = 16779068821653568252;
                                                } else {
                                                    if !keep_alive {
                                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                                            invalidate_persistent();
                                                        } else {
                                                            fd_close(sock);
                                                        }
                                                        sock = -(1 as i32);
                                                    }
                                                    current_block = 2518606133317875777;
                                                }
                                            } else {
                                                if keep_alive as i32 != 0 && !head_only
                                                    && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                                        as i32 != 0
                                                {
                                                    if !keep_alive {
                                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                                            invalidate_persistent();
                                                        } else {
                                                            fd_close(sock);
                                                        }
                                                        sock = -(1 as i32);
                                                    }
                                                } else {
                                                    if pconn_active as i32 != 0 && sock == pconn.socket {
                                                        invalidate_persistent();
                                                    } else {
                                                        fd_close(sock);
                                                    }
                                                    sock = -(1 as i32);
                                                }
                                                current_block = 2518606133317875777;
                                            }
                                            match current_block {
                                                16779068821653568252 => {}
                                                _ => {
                                                    match statcode {
                                                        307 | 308 => {
                                                            retval = uerr_t::NEWLOCATION_KEEP_POST;
                                                            current_block = 16779068821653568252;
                                                        }
                                                        301 => {
                                                            if !(opt.method).is_null()
                                                                && c_strcasecmp(
                                                                    opt.method,
                                                                    b"post\0" as *const u8 as *const i8,
                                                                ) != 0 as i32
                                                            {
                                                                retval = uerr_t::NEWLOCATION_KEEP_POST;
                                                                current_block = 16779068821653568252;
                                                            } else {
                                                                current_block = 768347334755947778;
                                                            }
                                                        }
                                                        302 => {
                                                            if !(opt.method).is_null()
                                                                && c_strcasecmp(
                                                                    opt.method,
                                                                    b"post\0" as *const u8 as *const i8,
                                                                ) != 0 as i32
                                                            {
                                                                retval = uerr_t::NEWLOCATION_KEEP_POST;
                                                                current_block = 16779068821653568252;
                                                            } else {
                                                                current_block = 768347334755947778;
                                                            }
                                                        }
                                                        _ => {
                                                            current_block = 768347334755947778;
                                                        }
                                                    }
                                                    match current_block {
                                                        16779068821653568252 => {}
                                                        _ => {
                                                            retval = uerr_t::NEWLOCATION;
                                                            current_block = 16779068821653568252;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 17222258012332649691;
                                    }
                                    match current_block {
                                        16779068821653568252 => {}
                                        _ => {
                                            if cond_get {
                                                if statcode == 304 as i32 {
                                                    logprintf(
                                                        log_options::LOG_VERBOSE,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"File %s not modified on server. Omitting download.\n\n\0"
                                                                as *const u8 as *const i8,
                                                            5 as i32,
                                                        ),
                                                        quote((*hs).local_file),
                                                    );
                                                    *dt |= C2RustUnnamed_4::RETROKF as i32;
                                                    if !keep_alive {
                                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                                            invalidate_persistent();
                                                        } else {
                                                            fd_close(sock);
                                                        }
                                                        sock = -(1 as i32);
                                                    }
                                                    retval = uerr_t::RETRUNNEEDED;
                                                    current_block = 16779068821653568252;
                                                } else {
                                                    current_block = 17309701497295823357;
                                                }
                                            } else {
                                                current_block = 17309701497295823357;
                                            }
                                            match current_block {
                                                16779068821653568252 => {}
                                                _ => {
                                                    set_content_type(dt, type_0);
                                                    if opt.adjust_extension {
                                                        let mut encoding_ext: *const i8 = 0 as *const i8;
                                                        match (*hs).local_encoding as i32 {
                                                            -1 | 0 => {}
                                                            4 => {
                                                                encoding_ext = b".br\0" as *const u8 as *const i8;
                                                            }
                                                            3 => {
                                                                encoding_ext = b".Z\0" as *const u8 as *const i8;
                                                            }
                                                            2 => {
                                                                encoding_ext = b".zlib\0" as *const u8 as *const i8;
                                                            }
                                                            1 => {
                                                                encoding_ext = b".gz\0" as *const u8 as *const i8;
                                                            }
                                                            _ => {
                                                                if opt.debug as i64 != 0 {
                                                                    debug_logprintf(
                                                                        b"No extension found for encoding %d\n\0" as *const u8
                                                                            as *const i8,
                                                                        (*hs).local_encoding as i32,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        if !encoding_ext.is_null() {
                                                            let mut file_ext: *mut i8 = strrchr(
                                                                (*hs).local_file,
                                                                '.' as i32,
                                                            );
                                                            if !file_ext.is_null()
                                                                && 0 as i32 == strcasecmp(file_ext, encoding_ext)
                                                            {
                                                                *file_ext = '\0' as i32 as i8;
                                                            }
                                                        }
                                                        if *dt & C2RustUnnamed_4::TEXTHTML as i32 != 0 {
                                                            ensure_extension(
                                                                hs,
                                                                b".html\0" as *const u8 as *const i8,
                                                                dt,
                                                            );
                                                        } else if *dt & C2RustUnnamed_4::TEXTCSS as i32 != 0 {
                                                            ensure_extension(
                                                                hs,
                                                                b".css\0" as *const u8 as *const i8,
                                                                dt,
                                                            );
                                                        }
                                                        if !encoding_ext.is_null() {
                                                            ensure_extension(hs, encoding_ext, dt);
                                                        }
                                                    }
                                                    if cond_get {
                                                        if statcode == 200 as i32 && !((*hs).remote_time).is_null()
                                                        {
                                                            let mut tmr: time_t = http_atotm((*hs).remote_time);
                                                            if tmr != -(1 as i32) as time_t
                                                                && tmr <= (*hs).orig_file_tstamp
                                                                && (contlen == -(1 as i32) as i64
                                                                    || contlen == (*hs).orig_file_size)
                                                            {
                                                                logprintf(
                                                                    log_options::LOG_VERBOSE,
                                                                    dcgettext(
                                                                        0 as *const i8,
                                                                        b"Server ignored If-Modified-Since header for file %s.\nYou might want to add --no-if-modified-since option.\n\n\0"
                                                                            as *const u8 as *const i8,
                                                                        5 as i32,
                                                                    ),
                                                                    quote((*hs).local_file),
                                                                );
                                                                *dt |= C2RustUnnamed_4::RETROKF as i32;
                                                                if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                    invalidate_persistent();
                                                                } else {
                                                                    fd_close(sock);
                                                                }
                                                                sock = -(1 as i32);
                                                                retval = uerr_t::RETRUNNEEDED;
                                                                current_block = 16779068821653568252;
                                                            } else {
                                                                current_block = 7388147277619867847;
                                                            }
                                                        } else {
                                                            current_block = 7388147277619867847;
                                                        }
                                                    } else {
                                                        current_block = 7388147277619867847;
                                                    }
                                                    match current_block {
                                                        16779068821653568252 => {}
                                                        _ => {
                                                            if statcode == 416 as i32
                                                                || !opt.timestamping && (*hs).restval > 0 as i32 as i64
                                                                    && statcode == 200 as i32 && contrange == 0 as i32 as i64
                                                                    && contlen >= 0 as i32 as i64 && (*hs).restval >= contlen
                                                            {
                                                                logputs(
                                                                    log_options::LOG_VERBOSE,
                                                                    dcgettext(
                                                                        0 as *const i8,
                                                                        b"\n    The file is already fully retrieved; nothing to do.\n\n\0"
                                                                            as *const u8 as *const i8,
                                                                        5 as i32,
                                                                    ),
                                                                );
                                                                (*hs).len = contlen;
                                                                (*hs).res = 0 as i32;
                                                                *dt |= C2RustUnnamed_4::RETROKF as i32;
                                                                if keep_alive as i32 != 0
                                                                    && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                                                        as i32 != 0
                                                                {
                                                                    if !keep_alive {
                                                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                            invalidate_persistent();
                                                                        } else {
                                                                            fd_close(sock);
                                                                        }
                                                                        sock = -(1 as i32);
                                                                    }
                                                                } else {
                                                                    if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                        invalidate_persistent();
                                                                    } else {
                                                                        fd_close(sock);
                                                                    }
                                                                    sock = -(1 as i32);
                                                                }
                                                                retval = uerr_t::RETRUNNEEDED;
                                                            } else if contrange != 0 as i32 as i64
                                                                && contrange != (*hs).restval
                                                                || statcode == 206 as i32 && contrange == 0
                                                                    && (*hs).restval != 0
                                                            {
                                                                if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                    invalidate_persistent();
                                                                } else {
                                                                    fd_close(sock);
                                                                }
                                                                sock = -(1 as i32);
                                                                retval = uerr_t::RANGEERR;
                                                            } else {
                                                                if contlen == -(1 as i32) as i64 {
                                                                    (*hs).contlen = -(1 as i32) as wgint;
                                                                } else if (*hs).remote_encoding as i32 == ENC_GZIP as i32 {
                                                                    (*hs).contlen = -(1 as i32) as wgint;
                                                                } else {
                                                                    (*hs).contlen = contlen + contrange;
                                                                }
                                                                if opt.verbose != 0 {
                                                                    if *dt & C2RustUnnamed_4::RETROKF as i32 != 0 {
                                                                        logputs(
                                                                            log_options::LOG_VERBOSE,
                                                                            dcgettext(
                                                                                0 as *const i8,
                                                                                b"Length: \0" as *const u8 as *const i8,
                                                                                5 as i32,
                                                                            ),
                                                                        );
                                                                        if contlen != -(1 as i32) as i64 {
                                                                            logputs(
                                                                                log_options::LOG_VERBOSE,
                                                                                number_to_static_string(contlen + contrange),
                                                                            );
                                                                            if contlen + contrange >= 1024 as i32 as i64 {
                                                                                logprintf(
                                                                                    log_options::LOG_VERBOSE,
                                                                                    b" (%s)\0" as *const u8 as *const i8,
                                                                                    human_readable(contlen + contrange, 10 as i32, 1 as i32),
                                                                                );
                                                                            }
                                                                            if contrange != 0 {
                                                                                if contlen >= 1024 as i32 as i64 {
                                                                                    logprintf(
                                                                                        log_options::LOG_VERBOSE,
                                                                                        dcgettext(
                                                                                            0 as *const i8,
                                                                                            b", %s (%s) remaining\0" as *const u8 as *const i8,
                                                                                            5 as i32,
                                                                                        ),
                                                                                        number_to_static_string(contlen),
                                                                                        human_readable(contlen, 10 as i32, 1 as i32),
                                                                                    );
                                                                                } else {
                                                                                    logprintf(
                                                                                        log_options::LOG_VERBOSE,
                                                                                        dcgettext(
                                                                                            0 as *const i8,
                                                                                            b", %s remaining\0" as *const u8 as *const i8,
                                                                                            5 as i32,
                                                                                        ),
                                                                                        number_to_static_string(contlen),
                                                                                    );
                                                                                }
                                                                            }
                                                                        } else {
                                                                            logputs(
                                                                                log_options::LOG_VERBOSE,
                                                                                if opt.ignore_length as i32 != 0 {
                                                                                    dcgettext(
                                                                                        0 as *const i8,
                                                                                        b"ignored\0" as *const u8 as *const i8,
                                                                                        5 as i32,
                                                                                    )
                                                                                } else {
                                                                                    dcgettext(
                                                                                        0 as *const i8,
                                                                                        b"unspecified\0" as *const u8 as *const i8,
                                                                                        5 as i32,
                                                                                    )
                                                                                },
                                                                            );
                                                                        }
                                                                        if !type_0.is_null() {
                                                                            logprintf(
                                                                                log_options::LOG_VERBOSE,
                                                                                b" [%s]\n\0" as *const u8 as *const i8,
                                                                                quotearg_style(quoting_style::escape_quoting_style, type_0),
                                                                            );
                                                                        } else {
                                                                            logputs(
                                                                                log_options::LOG_VERBOSE,
                                                                                b"\n\0" as *const u8 as *const i8,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                                if *dt & C2RustUnnamed_4::RETROKF as i32 == 0
                                                                    && !opt.content_on_error || head_only as i32 != 0
                                                                    || opt.spider as i32 != 0 && !opt.recursive
                                                                {
                                                                    (*hs).len = 0 as i32 as wgint;
                                                                    (*hs).res = 0 as i32;
                                                                    (*hs).restval = 0 as i32 as wgint;
                                                                    if warc_enabled {
                                                                        let mut _err_1: i32 = read_response_body(
                                                                            hs,
                                                                            sock,
                                                                            0 as *mut FILE,
                                                                            contlen,
                                                                            0 as i32 as wgint,
                                                                            chunked_transfer_encoding,
                                                                            (*u).url,
                                                                            warc_timestamp_str.as_mut_ptr(),
                                                                            warc_request_uuid.as_mut_ptr(),
                                                                            warc_ip,
                                                                            type_0,
                                                                            statcode,
                                                                            head,
                                                                        );
                                                                        if _err_1 != uerr_t::RETRFINISHED as i32
                                                                            || (*hs).res < 0 as i32
                                                                        {
                                                                            if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                invalidate_persistent();
                                                                            } else {
                                                                                fd_close(sock);
                                                                            }
                                                                            sock = -(1 as i32);
                                                                            retval = uerr_t::from_libc_c_uint(_err_1 as u32);
                                                                            current_block = 16779068821653568252;
                                                                        } else {
                                                                            if !keep_alive {
                                                                                if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                    invalidate_persistent();
                                                                                } else {
                                                                                    fd_close(sock);
                                                                                }
                                                                                sock = -(1 as i32);
                                                                            }
                                                                            current_block = 1623552932627830973;
                                                                        }
                                                                    } else {
                                                                        if head_only {
                                                                            if !keep_alive {
                                                                                if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                    invalidate_persistent();
                                                                                } else {
                                                                                    fd_close(sock);
                                                                                }
                                                                                sock = -(1 as i32);
                                                                            }
                                                                        } else if opt.spider as i32 != 0 && !opt.recursive {
                                                                            if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                invalidate_persistent();
                                                                            } else {
                                                                                fd_close(sock);
                                                                            }
                                                                            sock = -(1 as i32);
                                                                        } else if keep_alive as i32 != 0
                                                                            && skip_short_body(sock, contlen, chunked_transfer_encoding)
                                                                                as i32 != 0
                                                                        {
                                                                            if !keep_alive {
                                                                                if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                    invalidate_persistent();
                                                                                } else {
                                                                                    fd_close(sock);
                                                                                }
                                                                                sock = -(1 as i32);
                                                                            }
                                                                        } else {
                                                                            if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                invalidate_persistent();
                                                                            } else {
                                                                                fd_close(sock);
                                                                            }
                                                                            sock = -(1 as i32);
                                                                        }
                                                                        current_block = 1623552932627830973;
                                                                    }
                                                                    match current_block {
                                                                        16779068821653568252 => {}
                                                                        _ => {
                                                                            if statcode == 504 as i32 {
                                                                                retval = uerr_t::GATEWAYTIMEOUT;
                                                                            } else {
                                                                                retval = uerr_t::RETRFINISHED;
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    err = open_output_stream(hs, count, &mut fp) as i32;
                                                                    if err != uerr_t::RETROK as i32 {
                                                                        if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                            invalidate_persistent();
                                                                        } else {
                                                                            fd_close(sock);
                                                                        }
                                                                        sock = -(1 as i32);
                                                                        retval = uerr_t::from_libc_c_uint(err as u32);
                                                                    } else {
                                                                        if opt.enable_xattr {
                                                                            if original_url != u as *mut url {
                                                                                set_file_metadata(u, original_url, fp);
                                                                            } else {
                                                                                set_file_metadata(u, 0 as *const url, fp);
                                                                            }
                                                                        }
                                                                        err = read_response_body(
                                                                            hs,
                                                                            sock,
                                                                            fp,
                                                                            contlen,
                                                                            contrange,
                                                                            chunked_transfer_encoding,
                                                                            (*u).url,
                                                                            warc_timestamp_str.as_mut_ptr(),
                                                                            warc_request_uuid.as_mut_ptr(),
                                                                            warc_ip,
                                                                            type_0,
                                                                            statcode,
                                                                            head,
                                                                        );
                                                                        if (*hs).res >= 0 as i32 {
                                                                            if !keep_alive {
                                                                                if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                    invalidate_persistent();
                                                                                } else {
                                                                                    fd_close(sock);
                                                                                }
                                                                                sock = -(1 as i32);
                                                                            }
                                                                        } else {
                                                                            if pconn_active as i32 != 0 && sock == pconn.socket {
                                                                                invalidate_persistent();
                                                                            } else {
                                                                                fd_close(sock);
                                                                            }
                                                                            sock = -(1 as i32);
                                                                        }
                                                                        if output_stream.is_null() {
                                                                            fclose(fp);
                                                                        }
                                                                        retval = uerr_t::from_libc_c_uint(err as u32);
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
        _ => {}
    }
    rpl_free(head as *mut libc::c_void);
    head = 0 as *mut i8;
    rpl_free(type_0 as *mut libc::c_void);
    type_0 = 0 as *mut i8;
    rpl_free(message as *mut libc::c_void);
    message = 0 as *mut i8;
    resp_free(&mut resp);
    request_free(&mut req);
    return retval;
}
unsafe extern "C" fn check_retry_on_http_error(statcode: i32) -> bool {
    let mut tok: *const i8 = opt.retry_on_http_error;
    while !tok.is_null() && *tok as i32 != 0 {
        if atoi(tok) == statcode {
            return 1 as i32 != 0;
        }
        tok = strchr(tok, ',' as i32);
        if !tok.is_null() {
            tok = tok.offset(1);
            tok;
        }
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn http_loop(
    mut u: *const url,
    mut original_url: *mut url,
    mut newloc: *mut *mut i8,
    mut local_file: *mut *mut i8,
    mut referer: *const i8,
    mut dt: *mut i32,
    mut proxy: *mut url,
    mut iri: *mut iri,
) -> uerr_t {
    let mut current_block: u64;
    let mut count: i32 = 0;
    let mut got_head: bool = 0 as i32 != 0;
    let mut time_came_from_head: bool = 0 as i32 != 0;
    let mut got_name: bool = 0 as i32 != 0;
    let mut tms: *mut i8 = 0 as *mut i8;
    let mut tmrate: *const i8 = 0 as *const i8;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut ret: uerr_t = uerr_t::TRYLIMEXC;
    let mut tmr: time_t = -(1 as i32) as time_t;
    let mut hstat: http_stat = http_stat {
        len: 0,
        contlen: 0,
        restval: 0,
        res: 0,
        rderrmsg: 0 as *mut i8,
        newloc: 0 as *mut i8,
        remote_time: 0 as *mut i8,
        error: 0 as *mut i8,
        statcode: 0,
        message: 0 as *mut i8,
        rd_size: 0,
        dltime: 0.,
        referer: 0 as *const i8,
        local_file: 0 as *mut i8,
        existence_checked: false,
        timestamp_checked: false,
        orig_file_name: 0 as *mut i8,
        orig_file_size: 0,
        orig_file_tstamp: 0,
        local_encoding: ENC_NONE,
        remote_encoding: ENC_NONE,
        temporary: false,
    };
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut send_head_first: bool = 1 as i32 != 0;
    let mut force_full_retrieve: bool = 0 as i32 != 0;
    if !(opt.warc_filename).is_null() {
        force_full_retrieve = 1 as i32 != 0;
    }
    if !local_file.is_null() && !(opt.output_document).is_null() {
        *local_file = if *opt.output_document as i32 == '-' as i32
            && *(opt.output_document).offset(1 as i32 as isize) == 0
        {
            0 as *mut i8
        } else {
            xstrdup(opt.output_document)
        };
    }
    *newloc = 0 as *mut i8;
    if opt.cookies {
        load_cookies();
    }
    if opt.ftp_glob as i32 != 0 && has_wildcards_p((*u).path) as i32 != 0 {
        logputs(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Warning: wildcards not supported in HTTP.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    memset(
        &mut hstat as *mut http_stat as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<http_stat>() as u64,
    );
    hstat.referer = referer;
    if !(opt.output_document).is_null() {
        hstat.local_file = xstrdup(opt.output_document);
        got_name = 1 as i32 != 0;
    } else if !opt.content_disposition {
        hstat.local_file = url_file_name(
            if opt.trustservernames as i32 != 0 { u } else { original_url },
            0 as *mut i8,
        );
        got_name = 1 as i32 != 0;
    }
    if got_name as i32 != 0
        && file_exists_p(hstat.local_file, 0 as *mut file_stats_t) as i32 != 0
        && opt.noclobber as i32 != 0 && (opt.output_document).is_null()
    {
        get_file_flags(hstat.local_file, dt);
        ret = uerr_t::RETROK;
    } else {
        count = 0 as i32;
        *dt = 0 as i32;
        if !opt.spider {
            send_head_first = 0 as i32 != 0;
        }
        if opt.content_disposition as i32 != 0 && opt.always_rest as i32 != 0 {
            send_head_first = 1 as i32 != 0;
        }
        if opt.timestamping {
            if opt.if_modified_since as i32 != 0 && !send_head_first
                && got_name as i32 != 0
                && file_exists_p(hstat.local_file, 0 as *mut file_stats_t) as i32 != 0
            {
                *dt |= C2RustUnnamed_4::IF_MODIFIED_SINCE as i32;
                let mut timestamp_err: uerr_t = set_file_timestamp(&mut hstat);
                if timestamp_err as u32 != uerr_t::RETROK as i32 as u32 {
                    return timestamp_err;
                }
            } else if opt.content_disposition as i32 != 0
                || file_exists_p(hstat.local_file, 0 as *mut file_stats_t) as i32 != 0
            {
                send_head_first = 1 as i32 != 0;
            }
        }
        loop {
            count += 1;
            count;
            sleep_between_retrievals(count);
            tms = datetime_str(time(0 as *mut time_t));
            if opt.spider as i32 != 0 && !got_head {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Spider mode enabled. Check if remote file exists.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            if opt.verbose != 0 {
                let mut hurl: *mut i8 = url_string(
                    u,
                    url_auth_mode::URL_AUTH_HIDE_PASSWD,
                );
                if count > 1 as i32 {
                    let mut tmp: [i8; 256] = [0; 256];
                    sprintf(
                        tmp.as_mut_ptr(),
                        dcgettext(
                            0 as *const i8,
                            b"(try:%2d)\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        count,
                    );
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        b"--%s--  %s  %s\n\0" as *const u8 as *const i8,
                        tms,
                        tmp.as_mut_ptr(),
                        hurl,
                    );
                } else {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        b"--%s--  %s\n\0" as *const u8 as *const i8,
                        tms,
                        hurl,
                    );
                }
                rpl_free(hurl as *mut libc::c_void);
                hurl = 0 as *mut i8;
            }
            if send_head_first as i32 != 0 && !got_head {
                *dt |= C2RustUnnamed_4::HEAD_ONLY as i32;
            } else {
                *dt &= !(C2RustUnnamed_4::HEAD_ONLY as i32);
            }
            if force_full_retrieve {
                hstat.restval = hstat.len;
            } else if opt.start_pos >= 0 as i32 as i64 {
                hstat.restval = opt.start_pos;
            } else if opt.always_rest as i32 != 0 && got_name as i32 != 0
                && stat(hstat.local_file, &mut st) == 0 as i32
                && st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
            {
                hstat.restval = st.st_size;
            } else if count > 1 as i32 {
                if hstat.len < hstat.restval {
                    hstat.restval -= hstat.len;
                } else {
                    hstat.restval = hstat.len;
                }
            } else {
                hstat.restval = 0 as i32 as wgint;
            }
            if !proxy.is_null() && count > 1 as i32 || !opt.allow_cache {
                *dt |= C2RustUnnamed_4::SEND_NOCACHE as i32;
            } else {
                *dt &= !(C2RustUnnamed_4::SEND_NOCACHE as i32);
            }
            err = gethttp(u, original_url, &mut hstat, dt, proxy, iri, count);
            tms = datetime_str(time(0 as *mut time_t));
            if !(hstat.newloc).is_null() {
                *newloc = xstrdup(hstat.newloc);
            }
            match err as u32 {
                24 | 22 | 2 | 3 | 36 | 44 | 39 | 20 | 23 => {
                    printwhat(count, opt.ntry);
                }
                21 | 19 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Cannot write to %s (%s).\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote(hstat.local_file),
                        strerror(*__errno_location()),
                    );
                    ret = err;
                    break;
                }
                1 => {
                    if opt.retry_on_host_error {
                        printwhat(count, opt.ntry);
                    } else {
                        ret = err;
                        break;
                    }
                }
                5 | 41 | 45 | 33 | 46 | 38 | 51 => {
                    ret = err;
                    break;
                }
                50 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Required attribute missing from Header received.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    ret = err;
                    break;
                }
                42 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Username/Password Authentication Failed.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    ret = err;
                    break;
                }
                52 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Cannot write to WARC file.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    ret = err;
                    break;
                }
                53 | 54 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Cannot write to temporary WARC file.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    ret = err;
                    break;
                }
                4 => {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Unable to establish SSL connection.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    ret = err;
                    break;
                }
                47 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Cannot unlink %s (%s).\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote(hstat.local_file),
                        strerror(*__errno_location()),
                    );
                    ret = err;
                    break;
                }
                6 | 48 => {
                    if (*newloc).is_null() {
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"ERROR: Redirection (%d) without location.\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            hstat.statcode,
                        );
                        ret = uerr_t::WRONGCODE;
                    } else {
                        ret = err;
                    }
                    break;
                }
                34 => {
                    ret = uerr_t::RETROK;
                    break;
                }
                35 => {
                    if *dt & C2RustUnnamed_4::RETROKF as i32 == 0 {
                        let mut hurl_0: *mut i8 = 0 as *mut i8;
                        if opt.verbose == 0 {
                            hurl_0 = url_string(u, url_auth_mode::URL_AUTH_HIDE_PASSWD);
                            logprintf(
                                log_options::LOG_NONVERBOSE,
                                b"%s:\n\0" as *const u8 as *const i8,
                                hurl_0,
                            );
                        }
                        if *dt & C2RustUnnamed_4::HEAD_ONLY as i32 != 0
                            && (hstat.statcode == 500 as i32
                                || hstat.statcode == 501 as i32)
                        {
                            got_head = 1 as i32 != 0;
                            rpl_free(hurl_0 as *mut libc::c_void);
                            hurl_0 = 0 as *mut i8;
                        } else {
                            if opt.spider as i32 != 0 && !(*iri).utf8_encode {
                                if hurl_0.is_null() {
                                    hurl_0 = url_string(u, url_auth_mode::URL_AUTH_HIDE_PASSWD);
                                }
                                nonexisting_url(hurl_0);
                                logprintf(
                                    log_options::LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const i8,
                                        b"Remote file does not exist -- broken link!!!\n\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 6988365858197790817;
                            } else if check_retry_on_http_error(hstat.statcode) {
                                printwhat(count, opt.ntry);
                                rpl_free(hurl_0 as *mut libc::c_void);
                                hurl_0 = 0 as *mut i8;
                                current_block = 7828949454673616476;
                            } else {
                                logprintf(
                                    log_options::LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const i8,
                                        b"%s ERROR %d: %s.\n\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    tms,
                                    hstat.statcode,
                                    quotearg_style(
                                        quoting_style::escape_quoting_style,
                                        hstat.error,
                                    ),
                                );
                                current_block = 6988365858197790817;
                            }
                            match current_block {
                                7828949454673616476 => {}
                                _ => {
                                    logputs(
                                        log_options::LOG_VERBOSE,
                                        b"\n\0" as *const u8 as *const i8,
                                    );
                                    ret = uerr_t::WRONGCODE;
                                    rpl_free(hurl_0 as *mut libc::c_void);
                                    hurl_0 = 0 as *mut i8;
                                    break;
                                }
                            }
                        }
                    } else {
                        if !got_head || opt.spider as i32 != 0 && !opt.recursive {
                            got_head = 1 as i32 != 0;
                            if opt.timestamping as i32 != 0
                                && (hstat.remote_time).is_null()
                            {
                                logputs(
                                    log_options::LOG_NOTQUIET,
                                    dcgettext(
                                        0 as *const i8,
                                        b"Last-modified header missing -- time-stamps turned off.\n\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            } else if !(hstat.remote_time).is_null() {
                                tmr = http_atotm(hstat.remote_time);
                                if tmr == -(1 as i32) as time_t {
                                    logputs(
                                        log_options::LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const i8,
                                            b"Last-modified header invalid -- time-stamp ignored.\n\0"
                                                as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                }
                                if *dt & C2RustUnnamed_4::HEAD_ONLY as i32 != 0 {
                                    time_came_from_head = 1 as i32 != 0;
                                }
                            }
                            if send_head_first {
                                if opt.timestamping {
                                    if !(hstat.orig_file_name).is_null() {
                                        if !(hstat.remote_time).is_null()
                                            && tmr != -(1 as i32) as time_t
                                        {
                                            if hstat.orig_file_tstamp >= tmr {
                                                if hstat.contlen == -(1 as i32) as i64
                                                    || hstat.orig_file_size == hstat.contlen
                                                {
                                                    logprintf(
                                                        log_options::LOG_VERBOSE,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"Server file no newer than local file %s -- not retrieving.\n\n\0"
                                                                as *const u8 as *const i8,
                                                            5 as i32,
                                                        ),
                                                        quote(hstat.orig_file_name),
                                                    );
                                                    ret = uerr_t::RETROK;
                                                    break;
                                                } else {
                                                    logprintf(
                                                        log_options::LOG_VERBOSE,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"The sizes do not match (local %s) -- retrieving.\n\0"
                                                                as *const u8 as *const i8,
                                                            5 as i32,
                                                        ),
                                                        number_to_static_string(hstat.orig_file_size),
                                                    );
                                                }
                                            } else {
                                                force_full_retrieve = 1 as i32 != 0;
                                                logputs(
                                                    log_options::LOG_VERBOSE,
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"Remote file is newer, retrieving.\n\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    ),
                                                );
                                            }
                                            logputs(
                                                log_options::LOG_VERBOSE,
                                                b"\n\0" as *const u8 as *const i8,
                                            );
                                        }
                                    }
                                    hstat.timestamp_checked = 1 as i32 != 0;
                                }
                                if opt.spider {
                                    let mut finished: bool = 1 as i32 != 0;
                                    if opt.recursive {
                                        if *dt & C2RustUnnamed_4::TEXTHTML as i32 != 0
                                            || *dt & C2RustUnnamed_4::TEXTCSS as i32 != 0
                                        {
                                            logputs(
                                                log_options::LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"Remote file exists and could contain links to other resources -- retrieving.\n\n\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                            );
                                            finished = 0 as i32 != 0;
                                        } else {
                                            logprintf(
                                                log_options::LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"Remote file exists but does not contain any link -- not retrieving.\n\n\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                            );
                                            ret = uerr_t::RETROK;
                                        }
                                    } else {
                                        if *dt & C2RustUnnamed_4::TEXTHTML as i32 != 0
                                            || *dt & C2RustUnnamed_4::TEXTCSS as i32 != 0
                                        {
                                            logprintf(
                                                log_options::LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"Remote file exists and could contain further links,\nbut recursion is disabled -- not retrieving.\n\n\0"
                                                        as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                            );
                                        } else {
                                            logprintf(
                                                log_options::LOG_VERBOSE,
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"Remote file exists.\n\n\0" as *const u8 as *const i8,
                                                    5 as i32,
                                                ),
                                            );
                                        }
                                        ret = uerr_t::RETROK;
                                    }
                                    if finished {
                                        logprintf(
                                            log_options::LOG_NONVERBOSE,
                                            dcgettext(
                                                0 as *const i8,
                                                b"%s URL: %s %2d %s\n\0" as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                            tms,
                                            (*u).url,
                                            hstat.statcode,
                                            if !(hstat.message).is_null() {
                                                quotearg_style(
                                                    quoting_style::escape_quoting_style,
                                                    hstat.message,
                                                )
                                            } else {
                                                b"\0" as *const u8 as *const i8
                                            },
                                        );
                                        break;
                                    }
                                }
                                got_name = 1 as i32 != 0;
                                *dt &= !(C2RustUnnamed_4::HEAD_ONLY as i32);
                                count = 0 as i32;
                                current_block = 7828949454673616476;
                            } else {
                                current_block = 14698008245370361992;
                            }
                        } else {
                            current_block = 14698008245370361992;
                        }
                        match current_block {
                            7828949454673616476 => {}
                            _ => {
                                if opt.useservertimestamps as i32 != 0
                                    && tmr != -(1 as i32) as time_t
                                    && (hstat.len == hstat.contlen
                                        || hstat.res == 0 as i32
                                            && hstat.contlen == -(1 as i32) as i64)
                                {
                                    let mut fl: *const i8 = 0 as *const i8;
                                    set_local_file(&mut fl, hstat.local_file);
                                    if !fl.is_null() {
                                        let mut newtmr: time_t = -(1 as i32) as time_t;
                                        if time_came_from_head as i32 != 0
                                            && !(hstat.remote_time).is_null()
                                            && *(hstat.remote_time).offset(0 as i32 as isize) as i32
                                                != 0
                                        {
                                            newtmr = http_atotm(hstat.remote_time);
                                            if newtmr != -(1 as i32) as time_t {
                                                tmr = newtmr;
                                            }
                                        }
                                        touch(fl, tmr);
                                    }
                                }
                                tmrate = retr_rate(hstat.rd_size, hstat.dltime);
                                total_download_time += hstat.dltime;
                                if hstat.len == hstat.contlen {
                                    if *dt & C2RustUnnamed_4::RETROKF as i32 != 0
                                        || opt.content_on_error as i32 != 0
                                    {
                                        let mut write_to_stdout: bool = !(opt.output_document)
                                            .is_null()
                                            && (*opt.output_document as i32 == '-' as i32
                                                && *(opt.output_document).offset(1 as i32 as isize) == 0);
                                        logprintf(
                                            log_options::LOG_VERBOSE,
                                            if write_to_stdout as i32 != 0 {
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"%s (%s) - written to stdout %s[%s/%s]\n\n\0" as *const u8
                                                        as *const i8,
                                                    5 as i32,
                                                )
                                            } else {
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"%s (%s) - %s saved [%s/%s]\n\n\0" as *const u8
                                                        as *const i8,
                                                    5 as i32,
                                                )
                                            },
                                            tms,
                                            tmrate,
                                            if write_to_stdout as i32 != 0 {
                                                b"\0" as *const u8 as *const i8
                                            } else {
                                                quote(hstat.local_file)
                                            },
                                            number_to_static_string(hstat.len),
                                            number_to_static_string(hstat.contlen),
                                        );
                                        logprintf(
                                            log_options::LOG_NONVERBOSE,
                                            b"%s URL:%s [%s/%s] -> \"%s\" [%d]\n\0" as *const u8
                                                as *const i8,
                                            tms,
                                            (*u).url,
                                            number_to_static_string(hstat.len),
                                            number_to_static_string(hstat.contlen),
                                            hstat.local_file,
                                            count,
                                        );
                                    }
                                    numurls += 1;
                                    numurls;
                                    total_downloaded_bytes += hstat.rd_size;
                                    if *dt & C2RustUnnamed_4::ADDED_HTML_EXTENSION as i32 != 0 {
                                        downloaded_file(
                                            downloaded_file_t::FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED,
                                            hstat.local_file,
                                        );
                                    } else {
                                        downloaded_file(
                                            downloaded_file_t::FILE_DOWNLOADED_NORMALLY,
                                            hstat.local_file,
                                        );
                                    }
                                    ret = uerr_t::RETROK;
                                    break;
                                } else if hstat.res == 0 as i32 {
                                    if hstat.contlen == -(1 as i32) as i64 {
                                        if *dt & C2RustUnnamed_4::RETROKF as i32 != 0
                                            || opt.content_on_error as i32 != 0
                                        {
                                            let mut write_to_stdout_0: bool = !(opt.output_document)
                                                .is_null()
                                                && (*opt.output_document as i32 == '-' as i32
                                                    && *(opt.output_document).offset(1 as i32 as isize) == 0);
                                            logprintf(
                                                log_options::LOG_VERBOSE,
                                                if write_to_stdout_0 as i32 != 0 {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s (%s) - written to stdout %s[%s]\n\n\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s (%s) - %s saved [%s]\n\n\0" as *const u8 as *const i8,
                                                        5 as i32,
                                                    )
                                                },
                                                tms,
                                                tmrate,
                                                if write_to_stdout_0 as i32 != 0 {
                                                    b"\0" as *const u8 as *const i8
                                                } else {
                                                    quote(hstat.local_file)
                                                },
                                                number_to_static_string(hstat.len),
                                            );
                                            if !(opt.verbose != 0 || opt.quiet as i32 != 0) {
                                                let mut url: *mut i8 = url_string(
                                                    u,
                                                    url_auth_mode::URL_AUTH_HIDE_PASSWD,
                                                );
                                                logprintf(
                                                    log_options::LOG_NONVERBOSE,
                                                    b"%s URL:%s [%s] -> \"%s\" [%d]\n\0" as *const u8
                                                        as *const i8,
                                                    tms,
                                                    url,
                                                    number_to_static_string(hstat.len),
                                                    hstat.local_file,
                                                    count,
                                                );
                                                rpl_free(url as *mut libc::c_void);
                                                url = 0 as *mut i8;
                                            }
                                        }
                                        numurls += 1;
                                        numurls;
                                        total_downloaded_bytes += hstat.rd_size;
                                        if *dt & C2RustUnnamed_4::ADDED_HTML_EXTENSION as i32 != 0 {
                                            downloaded_file(
                                                downloaded_file_t::FILE_DOWNLOADED_AND_HTML_EXTENSION_ADDED,
                                                hstat.local_file,
                                            );
                                        } else {
                                            downloaded_file(
                                                downloaded_file_t::FILE_DOWNLOADED_NORMALLY,
                                                hstat.local_file,
                                            );
                                        }
                                        ret = uerr_t::RETROK;
                                        break;
                                    } else if hstat.len < hstat.contlen {
                                        logprintf(
                                            log_options::LOG_VERBOSE,
                                            dcgettext(
                                                0 as *const i8,
                                                b"%s (%s) - Connection closed at byte %s. \0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            tms,
                                            tmrate,
                                            number_to_static_string(hstat.len),
                                        );
                                        printwhat(count, opt.ntry);
                                    } else if hstat.len != hstat.restval {
                                        abort();
                                    } else {
                                        ret = uerr_t::RETROK;
                                        break;
                                    }
                                } else if hstat.contlen == -(1 as i32) as i64 {
                                    logprintf(
                                        log_options::LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const i8,
                                            b"%s (%s) - Read error at byte %s (%s).\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                        tms,
                                        tmrate,
                                        number_to_static_string(hstat.len),
                                        hstat.rderrmsg,
                                    );
                                    printwhat(count, opt.ntry);
                                } else {
                                    logprintf(
                                        log_options::LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const i8,
                                            b"%s (%s) - Read error at byte %s/%s (%s). \0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                        tms,
                                        tmrate,
                                        number_to_static_string(hstat.len),
                                        number_to_static_string(hstat.contlen),
                                        hstat.rderrmsg,
                                    );
                                    printwhat(count, opt.ntry);
                                }
                            }
                        }
                    }
                }
                _ => {
                    abort();
                }
            }
            if !(opt.ntry == 0 || count < opt.ntry) {
                break;
            }
        }
    }
    if (ret as u32 == uerr_t::RETROK as i32 as u32 || opt.content_on_error as i32 != 0)
        && !local_file.is_null()
    {
        rpl_free(*local_file as *mut libc::c_void);
        *local_file = 0 as *mut i8;
        if !(hstat.local_file).is_null() {
            *local_file = hstat.local_file;
            hstat.local_file = 0 as *mut i8;
        }
    }
    free_hstat(&mut hstat);
    return ret;
}
unsafe extern "C" fn check_end(mut p: *const i8) -> bool {
    if p.is_null() {
        return 0 as i32 != 0;
    }
    while c_isspace(*p as i32) {
        p = p.offset(1);
        p;
    }
    if *p == 0
        || *p.offset(0 as i32 as isize) as i32 == 'G' as i32
            && *p.offset(1 as i32 as isize) as i32 == 'M' as i32
            && *p.offset(2 as i32 as isize) as i32 == 'T' as i32
        || (*p.offset(0 as i32 as isize) as i32 == '+' as i32
            || *p.offset(0 as i32 as isize) as i32 == '-' as i32)
            && c_isdigit(*p.offset(1 as i32 as isize) as i32) as i32 != 0
    {
        return 1 as i32 != 0
    } else {
        return 0 as i32 != 0
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_atotm(mut time_string: *const i8) -> time_t {
    static mut time_formats: [*const i8; 4] = [
        b"%a, %d %b %Y %T\0" as *const u8 as *const i8,
        b"%A, %d-%b-%y %T\0" as *const u8 as *const i8,
        b"%a %b %d %T %Y\0" as *const u8 as *const i8,
        b"%a, %d-%b-%Y %T\0" as *const u8 as *const i8,
    ];
    let mut oldlocale: *const i8 = 0 as *const i8;
    let mut savedlocale: [i8; 256] = [0; 256];
    let mut i: size_t = 0;
    let mut ret: time_t = -(1 as i32) as time_t;
    oldlocale = setlocale(2 as i32, 0 as *const i8);
    if !oldlocale.is_null() {
        let mut l: size_t = (strlen(oldlocale)).wrapping_add(1 as i32 as u64);
        if l >= ::core::mem::size_of::<[i8; 256]>() as u64 {
            savedlocale[0 as i32 as usize] = '\0' as i32 as i8;
        } else {
            memcpy(
                savedlocale.as_mut_ptr() as *mut libc::c_void,
                oldlocale as *const libc::c_void,
                l,
            );
        }
    } else {
        savedlocale[0 as i32 as usize] = '\0' as i32 as i8;
    }
    setlocale(2 as i32, b"C\0" as *const u8 as *const i8);
    i = 0 as i32 as size_t;
    while i
        < (::core::mem::size_of::<[*const i8; 4]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
    {
        let mut t: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const i8,
        };
        memset(
            &mut t as *mut tm as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<tm>() as u64,
        );
        if check_end(strptime(time_string, time_formats[i as usize], &mut t)) {
            ret = rpl_timegm(&mut t);
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if savedlocale[0 as i32 as usize] != 0 {
        setlocale(2 as i32, savedlocale.as_mut_ptr());
    }
    return ret;
}
unsafe extern "C" fn basic_authentication_encode(
    mut user: *const i8,
    mut passwd: *const i8,
) -> *mut i8 {
    let mut buf_t1: [i8; 256] = [0; 256];
    let mut buf_t2: [i8; 256] = [0; 256];
    let mut t1: *mut i8 = 0 as *mut i8;
    let mut t2: *mut i8 = 0 as *mut i8;
    let mut ret: *mut i8 = 0 as *mut i8;
    let mut len1: size_t = (strlen(user))
        .wrapping_add(1 as i32 as u64)
        .wrapping_add(strlen(passwd));
    if len1 < ::core::mem::size_of::<[i8; 256]>() as u64 {
        t1 = buf_t1.as_mut_ptr();
    } else {
        t1 = xmalloc(len1.wrapping_add(1 as i32 as u64)) as *mut i8;
    }
    if (4 as i32 as u64)
        .wrapping_mul(len1.wrapping_add(2 as i32 as u64).wrapping_div(3 as i32 as u64))
        < ::core::mem::size_of::<[i8; 256]>() as u64
    {
        t2 = buf_t2.as_mut_ptr();
    } else {
        t2 = xmalloc(
            (4 as i32 as u64)
                .wrapping_mul(
                    len1.wrapping_add(2 as i32 as u64).wrapping_div(3 as i32 as u64),
                )
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
    }
    sprintf(t1, b"%s:%s\0" as *const u8 as *const i8, user, passwd);
    wget_base64_encode(t1 as *const libc::c_void, len1, t2);
    ret = concat_strings(b"Basic \0" as *const u8 as *const i8, t2, 0 as *mut i8);
    if t2 != buf_t2.as_mut_ptr() {
        rpl_free(t2 as *mut libc::c_void);
        t2 = 0 as *mut i8;
    }
    if t1 != buf_t1.as_mut_ptr() {
        rpl_free(t1 as *mut libc::c_void);
        t1 = 0 as *mut i8;
    }
    return ret;
}
unsafe extern "C" fn dump_hash(mut buf: *mut i8, mut hash: *const u8) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 16 as i32 {
        let fresh14 = buf;
        buf = buf.offset(1);
        *fresh14 = ((*::core::mem::transmute::<
            &[u8; 17],
            &[i8; 17],
        >(b"0123456789abcdef\0"))[(*hash as i32 >> 4 as i32) as usize] as i32 + 0 as i32)
            as i8;
        let fresh15 = buf;
        buf = buf.offset(1);
        *fresh15 = ((*::core::mem::transmute::<
            &[u8; 17],
            &[i8; 17],
        >(b"0123456789abcdef\0"))[(*hash as i32 & 0xf as i32) as usize] as i32
            + 0 as i32) as i8;
        i += 1;
        i;
        hash = hash.offset(1);
        hash;
    }
    *buf = '\0' as i32 as i8;
}
unsafe extern "C" fn digest_authentication_encode(
    mut au: *const i8,
    mut user: *const i8,
    mut passwd: *const i8,
    mut method: *const i8,
    mut path: *const i8,
    mut auth_err: *mut uerr_t,
) -> *mut i8 {
    static mut realm: *mut i8 = 0 as *const i8 as *mut i8;
    static mut opaque: *mut i8 = 0 as *const i8 as *mut i8;
    static mut nonce: *mut i8 = 0 as *const i8 as *mut i8;
    static mut qop: *mut i8 = 0 as *const i8 as *mut i8;
    static mut algorithm: *mut i8 = 0 as *const i8 as *mut i8;
    static mut options: [C2RustUnnamed_9; 5] = unsafe {
        [
            {
                let mut init = C2RustUnnamed_9 {
                    name: b"realm\0" as *const u8 as *const i8,
                    variable: &realm as *const *mut i8 as *mut *mut i8,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_9 {
                    name: b"opaque\0" as *const u8 as *const i8,
                    variable: &opaque as *const *mut i8 as *mut *mut i8,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_9 {
                    name: b"nonce\0" as *const u8 as *const i8,
                    variable: &nonce as *const *mut i8 as *mut *mut i8,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_9 {
                    name: b"qop\0" as *const u8 as *const i8,
                    variable: &qop as *const *mut i8 as *mut *mut i8,
                };
                init
            },
            {
                let mut init = C2RustUnnamed_9 {
                    name: b"algorithm\0" as *const u8 as *const i8,
                    variable: &algorithm as *const *mut i8 as *mut *mut i8,
                };
                init
            },
        ]
    };
    let mut cnonce: [i8; 16] = *::core::mem::transmute::<
        &[u8; 16],
        &mut [i8; 16],
    >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    let mut res: *mut i8 = 0 as *mut i8;
    let mut res_len: i32 = 0;
    let mut res_size: size_t = 0;
    let mut name: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    let mut value: param_token = param_token {
        b: 0 as *const i8,
        e: 0 as *const i8,
    };
    qop = 0 as *mut i8;
    algorithm = qop;
    nonce = algorithm;
    opaque = nonce;
    realm = opaque;
    au = au.offset(6 as i32 as isize);
    while extract_param(
        &mut au,
        &mut name,
        &mut value,
        ',' as i32 as i8,
        0 as *mut bool,
    ) {
        let mut i: size_t = 0;
        let mut namelen: size_t = (name.e).offset_from(name.b) as i64 as size_t;
        i = 0 as i32 as size_t;
        while i
            < (::core::mem::size_of::<[C2RustUnnamed_9; 5]>() as u64)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_9>() as u64)
        {
            if namelen == strlen(options[i as usize].name)
                && 0 as i32 == strncmp(name.b, options[i as usize].name, namelen)
            {
                *options[i as usize].variable = strdupdelim(value.b, value.e);
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    if !qop.is_null() && strcmp(qop, b"auth\0" as *const u8 as *const i8) != 0 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Unsupported quality of protection '%s'.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            qop,
        );
        rpl_free(qop as *mut libc::c_void);
        qop = 0 as *mut i8;
    } else if !algorithm.is_null()
        && strcmp(algorithm, b"MD5\0" as *const u8 as *const i8) != 0
        && strcmp(algorithm, b"MD5-sess\0" as *const u8 as *const i8) != 0
    {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Unsupported algorithm '%s'.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            algorithm,
        );
        rpl_free(algorithm as *mut libc::c_void);
        algorithm = 0 as *mut i8;
    }
    if realm.is_null() || nonce.is_null() || user.is_null() || passwd.is_null()
        || path.is_null() || method.is_null()
    {
        *auth_err = uerr_t::ATTRMISSING;
    } else {
        let mut ctx: md5_ctx = md5_ctx {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            total: [0; 2],
            buflen: 0,
            buffer: [0; 32],
        };
        let mut hash: [u8; 16] = [0; 16];
        let mut a1buf: [i8; 33] = [0; 33];
        let mut a2buf: [i8; 33] = [0; 33];
        let mut response_digest: [i8; 33] = [0; 33];
        md5_init_ctx(&mut ctx);
        md5_process_bytes(
            user as *mut u8 as *const libc::c_void,
            strlen(user),
            &mut ctx,
        );
        md5_process_bytes(
            b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
            1 as i32 as size_t,
            &mut ctx,
        );
        md5_process_bytes(
            realm as *mut u8 as *const libc::c_void,
            strlen(realm),
            &mut ctx,
        );
        md5_process_bytes(
            b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
            1 as i32 as size_t,
            &mut ctx,
        );
        md5_process_bytes(
            passwd as *mut u8 as *const libc::c_void,
            strlen(passwd),
            &mut ctx,
        );
        md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        dump_hash(a1buf.as_mut_ptr(), hash.as_mut_ptr());
        if !algorithm.is_null()
            && strcmp(algorithm, b"MD5-sess\0" as *const u8 as *const i8) == 0
        {
            snprintf(
                cnonce.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 16]>() as u64,
                b"%08x\0" as *const u8 as *const i8,
                random_number(2147483647 as i32) as u32,
            );
            md5_init_ctx(&mut ctx);
            md5_process_bytes(
                a1buf.as_mut_ptr() as *const libc::c_void,
                (16 as i32 * 2 as i32) as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                nonce as *mut u8 as *const libc::c_void,
                strlen(nonce),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                cnonce.as_mut_ptr() as *mut u8 as *const libc::c_void,
                strlen(cnonce.as_mut_ptr()),
                &mut ctx,
            );
            md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
            dump_hash(a1buf.as_mut_ptr(), hash.as_mut_ptr());
        }
        md5_init_ctx(&mut ctx);
        md5_process_bytes(
            method as *mut u8 as *const libc::c_void,
            strlen(method),
            &mut ctx,
        );
        md5_process_bytes(
            b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
            1 as i32 as size_t,
            &mut ctx,
        );
        md5_process_bytes(
            path as *mut u8 as *const libc::c_void,
            strlen(path),
            &mut ctx,
        );
        md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        dump_hash(a2buf.as_mut_ptr(), hash.as_mut_ptr());
        if !qop.is_null() && strcmp(qop, b"auth\0" as *const u8 as *const i8) == 0 {
            if *cnonce.as_mut_ptr() == 0 {
                snprintf(
                    cnonce.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 16]>() as u64,
                    b"%08x\0" as *const u8 as *const i8,
                    random_number(2147483647 as i32) as u32,
                );
            }
            md5_init_ctx(&mut ctx);
            md5_process_bytes(
                a1buf.as_mut_ptr() as *mut u8 as *const libc::c_void,
                (16 as i32 * 2 as i32) as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                nonce as *mut u8 as *const libc::c_void,
                strlen(nonce),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b"00000001\0" as *const u8 as *const i8 as *mut u8
                    as *const libc::c_void,
                8 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                cnonce.as_mut_ptr() as *mut u8 as *const libc::c_void,
                strlen(cnonce.as_mut_ptr()),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                qop as *mut u8 as *const libc::c_void,
                strlen(qop),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                a2buf.as_mut_ptr() as *mut u8 as *const libc::c_void,
                (16 as i32 * 2 as i32) as size_t,
                &mut ctx,
            );
            md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        } else {
            md5_init_ctx(&mut ctx);
            md5_process_bytes(
                a1buf.as_mut_ptr() as *mut u8 as *const libc::c_void,
                (16 as i32 * 2 as i32) as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                nonce as *mut u8 as *const libc::c_void,
                strlen(nonce),
                &mut ctx,
            );
            md5_process_bytes(
                b":\0" as *const u8 as *const i8 as *mut u8 as *const libc::c_void,
                1 as i32 as size_t,
                &mut ctx,
            );
            md5_process_bytes(
                a2buf.as_mut_ptr() as *mut u8 as *const libc::c_void,
                (16 as i32 * 2 as i32) as size_t,
                &mut ctx,
            );
            md5_finish_ctx(&mut ctx, hash.as_mut_ptr() as *mut libc::c_void);
        }
        dump_hash(response_digest.as_mut_ptr(), hash.as_mut_ptr());
        res_size = (strlen(user))
            .wrapping_add(strlen(realm))
            .wrapping_add(strlen(nonce))
            .wrapping_add(strlen(path))
            .wrapping_add((2 as i32 * 16 as i32) as u64)
            .wrapping_add(
                (if !opaque.is_null() { strlen(opaque) } else { 0 as i32 as u64 }),
            )
            .wrapping_add(
                (if !algorithm.is_null() { strlen(algorithm) } else { 0 as i32 as u64 }),
            )
            .wrapping_add((if !qop.is_null() { 128 as i32 } else { 0 as i32 }) as u64)
            .wrapping_add(strlen(cnonce.as_mut_ptr()))
            .wrapping_add(128 as i32 as u64);
        res = xmalloc(res_size) as *mut i8;
        if !qop.is_null() && strcmp(qop, b"auth\0" as *const u8 as *const i8) == 0 {
            res_len = snprintf(
                res,
                res_size,
                b"Digest username=\"%s\", realm=\"%s\", nonce=\"%s\", uri=\"%s\", response=\"%s\", qop=auth, nc=00000001, cnonce=\"%s\"\0"
                    as *const u8 as *const i8,
                user,
                realm,
                nonce,
                path,
                response_digest.as_mut_ptr(),
                cnonce.as_mut_ptr(),
            );
        } else {
            res_len = snprintf(
                res,
                res_size,
                b"Digest username=\"%s\", realm=\"%s\", nonce=\"%s\", uri=\"%s\", response=\"%s\"\0"
                    as *const u8 as *const i8,
                user,
                realm,
                nonce,
                path,
                response_digest.as_mut_ptr(),
            );
        }
        if !opaque.is_null() {
            res_len
                += snprintf(
                    res.offset(res_len as isize),
                    res_size.wrapping_sub(res_len as u64),
                    b", opaque=\"%s\"\0" as *const u8 as *const i8,
                    opaque,
                );
        }
        if !algorithm.is_null() {
            snprintf(
                res.offset(res_len as isize),
                res_size.wrapping_sub(res_len as u64),
                b", algorithm=\"%s\"\0" as *const u8 as *const i8,
                algorithm,
            );
        }
    }
    rpl_free(realm as *mut libc::c_void);
    realm = 0 as *mut i8;
    rpl_free(opaque as *mut libc::c_void);
    opaque = 0 as *mut i8;
    rpl_free(nonce as *mut libc::c_void);
    nonce = 0 as *mut i8;
    rpl_free(qop as *mut libc::c_void);
    qop = 0 as *mut i8;
    rpl_free(algorithm as *mut libc::c_void);
    algorithm = 0 as *mut i8;
    return res;
}
unsafe extern "C" fn known_authentication_scheme_p(
    mut hdrbeg: *const i8,
    mut hdrend: *const i8,
) -> bool {
    return hdrend > hdrbeg
        && hdrend.offset_from(hdrbeg) as i64 as size_t
            >= (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
        && 0 as i32
            == c_strncasecmp(
                hdrbeg,
                b"Basic\0" as *const u8 as *const i8,
                (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
            )
        && (hdrend.offset_from(hdrbeg) as i64 as size_t
            == (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
            || c_isspace(
                *hdrbeg
                    .offset(
                        (::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32,
            ) as i32 != 0)
        || hdrend > hdrbeg
            && hdrend.offset_from(hdrbeg) as i64 as size_t
                >= (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
            && 0 as i32
                == c_strncasecmp(
                    hdrbeg,
                    b"Digest\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 7]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
            && (hdrend.offset_from(hdrbeg) as i64 as size_t
                == (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                || c_isspace(
                    *hdrbeg
                        .offset(
                            (::core::mem::size_of::<[i8; 7]>() as u64)
                                .wrapping_sub(1 as i32 as u64) as isize,
                        ) as i32,
                ) as i32 != 0)
        || hdrend > hdrbeg
            && hdrend.offset_from(hdrbeg) as i64 as size_t
                >= (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
            && 0 as i32
                == c_strncasecmp(
                    hdrbeg,
                    b"NTLM\0" as *const u8 as *const i8,
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
            && (hdrend.offset_from(hdrbeg) as i64 as size_t
                == (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                || c_isspace(
                    *hdrbeg
                        .offset(
                            (::core::mem::size_of::<[i8; 5]>() as u64)
                                .wrapping_sub(1 as i32 as u64) as isize,
                        ) as i32,
                ) as i32 != 0);
}
unsafe extern "C" fn create_authorization_line(
    mut au: *const i8,
    mut user: *const i8,
    mut passwd: *const i8,
    mut method: *const i8,
    mut path: *const i8,
    mut finished: *mut bool,
    mut auth_err: *mut uerr_t,
) -> *mut i8 {
    match c_toupper(*au as i32) {
        66 => {
            *finished = 1 as i32 != 0;
            return basic_authentication_encode(user, passwd);
        }
        68 => {
            *finished = 1 as i32 != 0;
            return digest_authentication_encode(
                au,
                user,
                passwd,
                method,
                path,
                auth_err,
            );
        }
        78 => {
            if !ntlm_input(&mut pconn.ntlm, au) {
                *finished = 1 as i32 != 0;
                return 0 as *mut i8;
            }
            return ntlm_output(&mut pconn.ntlm, user, passwd, finished);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn load_cookies() {
    if wget_cookie_jar.is_null() {
        wget_cookie_jar = cookie_jar_new();
    }
    if !(opt.cookies_input).is_null() && !cookies_loaded_p {
        cookie_jar_load(wget_cookie_jar, opt.cookies_input);
        cookies_loaded_p = 1 as i32 != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn save_cookies() {
    if !wget_cookie_jar.is_null() {
        cookie_jar_save(wget_cookie_jar, opt.cookies_output);
    }
}
unsafe extern "C" fn ensure_extension(
    mut hs: *mut http_stat,
    mut ext: *const i8,
    mut dt: *mut i32,
) {
    let mut last_period_in_local_filename: *mut i8 = strrchr(
        (*hs).local_file,
        '.' as i32,
    );
    let mut shortext: [i8; 8] = [0; 8];
    let mut len: i32 = 0;
    shortext[0 as i32 as usize] = '\0' as i32 as i8;
    len = strlen(ext) as i32;
    if len == 5 as i32 {
        memcpy(
            shortext.as_mut_ptr() as *mut libc::c_void,
            ext as *const libc::c_void,
            (len - 1 as i32) as u64,
        );
        shortext[(len - 1 as i32) as usize] = '\0' as i32 as i8;
    }
    if last_period_in_local_filename.is_null()
        || !(0 as i32 == strcasecmp(last_period_in_local_filename, shortext.as_mut_ptr())
            || 0 as i32 == strcasecmp(last_period_in_local_filename, ext))
    {
        let mut local_filename_len: i32 = strlen((*hs).local_file) as i32;
        (*hs).local_file = xrealloc(
            (*hs).local_file as *mut libc::c_void,
            (local_filename_len + 24 as i32 + len) as size_t,
        ) as *mut i8;
        strcpy(((*hs).local_file).offset(local_filename_len as isize), ext);
        if !(opt.noclobber as i32 != 0 || opt.always_rest as i32 != 0
            || opt.timestamping as i32 != 0 || opt.dirstruct as i32 != 0
            || !(opt.output_document).is_null() || opt.backups > 0 as i32)
            && file_exists_p((*hs).local_file, 0 as *mut file_stats_t) as i32 != 0
        {
            let mut ext_num: i32 = 1 as i32;
            loop {
                let fresh16 = ext_num;
                ext_num = ext_num + 1;
                sprintf(
                    ((*hs).local_file).offset(local_filename_len as isize),
                    b".%d%s\0" as *const u8 as *const i8,
                    fresh16,
                    ext,
                );
                if !file_exists_p((*hs).local_file, 0 as *mut file_stats_t) {
                    break;
                }
            }
        }
        *dt |= C2RustUnnamed_4::ADDED_HTML_EXTENSION as i32;
    }
}