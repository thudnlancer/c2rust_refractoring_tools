use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn chmod(__file: *const i8, __mode: __mode_t) -> i32;
    fn fnmatch(__pattern: *const i8, __name: *const i8, __flags: i32) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn rpl_strtoll(
        string: *const i8,
        endptr: *mut *mut i8,
        base: i32,
    ) -> libc::c_longlong;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn fclose(__stream: *mut FILE) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logputs(_: log_options, _: *const i8);
    fn quote_n(n: i32, arg: *const i8) -> *const i8;
    fn quote(arg: *const i8) -> *const i8;
    fn quotearg_style(s: quoting_style, arg: *const i8) -> *mut i8;
    fn symlink(__from: *const i8, __to: *const i8) -> i32;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
    fn unlink(__name: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn datetime_str(_: time_t) -> *mut i8;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn concat_strings(_: *const i8, _: ...) -> *mut i8;
    fn touch(_: *const i8, _: time_t);
    fn remove_link(_: *const i8) -> i32;
    fn file_exists_p(_: *const i8, _: *mut file_stats_t) -> bool;
    fn fopen_excl(_: *const i8, _: i32) -> *mut FILE;
    fn file_merge(_: *const i8, _: *const i8) -> *mut i8;
    fn fnmatch_nocase(_: *const i8, _: *const i8, _: i32) -> i32;
    fn acceptable(_: *const i8) -> bool;
    fn accept_url(_: *const i8) -> bool;
    fn accdir(s: *const i8) -> bool;
    fn has_wildcards_p(_: *const i8) -> bool;
    fn human_readable(_: wgint, _: i32, _: i32) -> *mut i8;
    fn number_to_static_string(_: wgint) -> *mut i8;
    fn url_set_dir(_: *mut url, _: *const i8);
    fn url_set_file(_: *mut url, _: *const i8);
    fn scheme_disable(_: url_scheme);
    fn url_string(_: *const url, _: url_auth_mode) -> *mut i8;
    fn url_file_name(_: *const url, _: *mut i8) -> *mut i8;
    fn mkalldirs(_: *const i8) -> i32;
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
    fn retr_rate(_: wgint, _: libc::c_double) -> *const i8;
    fn printwhat(_: i32, _: i32);
    fn sleep_between_retrievals(_: i32);
    fn rotate_backups(_: *const i8);
    fn set_local_file(_: *mut *const i8, _: *const i8);
    fn input_file_url(_: *const i8) -> bool;
    fn ftp_prot(_: i32, _: prot_level) -> uerr_t;
    fn print_address(_: *const ip_address) -> *const i8;
    fn ftp_response(_: i32, _: *mut *mut i8) -> uerr_t;
    fn ftp_greeting(_: i32) -> uerr_t;
    fn ftp_login(_: i32, _: *const i8, _: *const i8) -> uerr_t;
    fn ftp_port(_: i32, _: *mut i32) -> uerr_t;
    fn ftp_pasv(_: i32, _: *mut ip_address, _: *mut i32) -> uerr_t;
    fn ftp_auth(_: i32, _: url_scheme) -> uerr_t;
    fn ftp_pbsz(_: i32, _: i32) -> uerr_t;
    fn ftp_pwd(_: i32, _: *mut *mut i8) -> uerr_t;
    fn ftp_list(_: i32, _: *const i8, _: bool, _: bool, _: *mut bool) -> uerr_t;
    fn ftp_size(_: i32, _: *const i8, _: *mut wgint) -> uerr_t;
    fn ftp_rest(_: i32, _: wgint) -> uerr_t;
    fn ftp_retr(_: i32, _: *const i8) -> uerr_t;
    fn ftp_syst(_: i32, _: *mut stype, _: *mut ustype) -> uerr_t;
    fn ftp_cwd(_: i32, _: *const i8) -> uerr_t;
    fn ftp_type(_: i32, _: i32) -> uerr_t;
    fn ftp_epsv(_: i32, _: *mut ip_address, _: *mut i32) -> uerr_t;
    fn ftp_eprt(_: i32, _: *mut i32) -> uerr_t;
    fn ftp_lprt(_: i32, _: *mut i32) -> uerr_t;
    fn ftp_lpsv(_: i32, _: *mut ip_address, _: *mut i32) -> uerr_t;
    fn ftp_parse_ls(_: *const i8, _: stype) -> *mut fileinfo;
    fn ftp_index(_: *const i8, _: *mut url, _: *mut fileinfo) -> uerr_t;
    fn ftp_process_type(_: *const i8) -> i8;
    fn ssl_init() -> bool;
    fn ssl_connect_wget(_: i32, _: *const i8, _: *mut i32) -> bool;
    fn ssl_check_certificate(_: i32, _: *const i8) -> bool;
    fn connect_to_host(_: *const i8, _: i32) -> i32;
    fn connect_to_ip(_: *const ip_address, _: i32, _: *const i8) -> i32;
    fn accept_connection(_: i32) -> i32;
    fn socket_ip_address(_: i32, _: *mut ip_address, _: i32) -> bool;
    fn retryable_socket_connect_error(_: i32) -> bool;
    fn fd_errstr(_: i32) -> *const i8;
    fn fd_close(_: i32);
    fn search_netrc(
        _: *const i8,
        _: *mut *const i8,
        _: *mut *const i8,
        _: i32,
        _: *mut FILE,
    );
    fn downloaded_file(_: downloaded_file_t, _: *const i8) -> downloaded_file_t;
    fn warc_tempfile() -> *mut FILE;
    fn warc_write_resource_record(
        resource_uuid: *const i8,
        url: *const i8,
        timestamp_str: *const i8,
        concurrent_to_uuid: *const i8,
        ip: *const ip_address,
        content_type: *const i8,
        body: *mut FILE,
        payload_offset: off_t,
    ) -> bool;
    fn c_strncasecmp(s1: *const i8, s2: *const i8, n: size_t) -> i32;
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
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    rb_read_exactly = 1,
    rb_skip_startpos = 2,
    rb_chunked_transfer_encoding = 4,
    rb_compressed_gzip = 8,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_5::rb_read_exactly => 1,
            C2RustUnnamed_5::rb_skip_startpos => 2,
            C2RustUnnamed_5::rb_chunked_transfer_encoding => 4,
            C2RustUnnamed_5::rb_compressed_gzip => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_5 {
        match value {
            1 => C2RustUnnamed_5::rb_read_exactly,
            2 => C2RustUnnamed_5::rb_skip_startpos,
            4 => C2RustUnnamed_5::rb_chunked_transfer_encoding,
            8 => C2RustUnnamed_5::rb_compressed_gzip,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_address {
    pub family: i32,
    pub data: C2RustUnnamed_7,
    pub ipv6_scope: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stype {
    ST_UNIX,
    ST_VMS,
    ST_WINNT,
    ST_MACOS,
    ST_OS400,
    ST_OTHER,
}
impl stype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            stype::ST_UNIX => 0,
            stype::ST_VMS => 1,
            stype::ST_WINNT => 2,
            stype::ST_MACOS => 3,
            stype::ST_OS400 => 4,
            stype::ST_OTHER => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> stype {
        match value {
            0 => stype::ST_UNIX,
            1 => stype::ST_VMS,
            2 => stype::ST_WINNT,
            3 => stype::ST_MACOS,
            4 => stype::ST_OS400,
            5 => stype::ST_OTHER,
            _ => panic!("Invalid value for stype: {}", value),
        }
    }
}
impl AddAssign<u32> for stype {
    fn add_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for stype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for stype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for stype {
    fn div_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for stype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = stype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for stype {
    type Output = stype;
    fn add(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for stype {
    type Output = stype;
    fn sub(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for stype {
    type Output = stype;
    fn mul(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for stype {
    type Output = stype;
    fn div(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for stype {
    type Output = stype;
    fn rem(self, rhs: u32) -> stype {
        stype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ustype {
    UST_TYPE_L8,
    UST_MULTINET,
    UST_OTHER,
}
impl ustype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ustype::UST_TYPE_L8 => 0,
            ustype::UST_MULTINET => 1,
            ustype::UST_OTHER => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> ustype {
        match value {
            0 => ustype::UST_TYPE_L8,
            1 => ustype::UST_MULTINET,
            2 => ustype::UST_OTHER,
            _ => panic!("Invalid value for ustype: {}", value),
        }
    }
}
impl AddAssign<u32> for ustype {
    fn add_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ustype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ustype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ustype {
    fn div_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ustype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ustype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ustype {
    type Output = ustype;
    fn add(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ustype {
    type Output = ustype;
    fn sub(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ustype {
    type Output = ustype;
    fn mul(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ustype {
    type Output = ustype;
    fn div(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ustype {
    type Output = ustype;
    fn rem(self, rhs: u32) -> ustype {
        ustype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum prot_level {
    PROT_CLEAR,
    PROT_SAFE,
    PROT_CONFIDENTIAL,
    PROT_PRIVATE,
}
impl prot_level {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            prot_level::PROT_CLEAR => 67,
            prot_level::PROT_SAFE => 83,
            prot_level::PROT_CONFIDENTIAL => 69,
            prot_level::PROT_PRIVATE => 80,
        }
    }
    fn from_libc_c_uint(value: u32) -> prot_level {
        match value {
            67 => prot_level::PROT_CLEAR,
            83 => prot_level::PROT_SAFE,
            69 => prot_level::PROT_CONFIDENTIAL,
            80 => prot_level::PROT_PRIVATE,
            _ => panic!("Invalid value for prot_level: {}", value),
        }
    }
}
impl AddAssign<u32> for prot_level {
    fn add_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for prot_level {
    fn sub_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for prot_level {
    fn mul_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for prot_level {
    fn div_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for prot_level {
    fn rem_assign(&mut self, rhs: u32) {
        *self = prot_level::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for prot_level {
    type Output = prot_level;
    fn add(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for prot_level {
    type Output = prot_level;
    fn sub(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for prot_level {
    type Output = prot_level;
    fn mul(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for prot_level {
    type Output = prot_level;
    fn div(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for prot_level {
    type Output = prot_level;
    fn rem(self, rhs: u32) -> prot_level {
        prot_level::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ftype {
    FT_PLAINFILE,
    FT_DIRECTORY,
    FT_SYMLINK,
    FT_UNKNOWN,
}
impl ftype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ftype::FT_PLAINFILE => 0,
            ftype::FT_DIRECTORY => 1,
            ftype::FT_SYMLINK => 2,
            ftype::FT_UNKNOWN => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> ftype {
        match value {
            0 => ftype::FT_PLAINFILE,
            1 => ftype::FT_DIRECTORY,
            2 => ftype::FT_SYMLINK,
            3 => ftype::FT_UNKNOWN,
            _ => panic!("Invalid value for ftype: {}", value),
        }
    }
}
impl AddAssign<u32> for ftype {
    fn add_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ftype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ftype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ftype {
    fn div_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ftype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ftype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ftype {
    type Output = ftype;
    fn add(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ftype {
    type Output = ftype;
    fn sub(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ftype {
    type Output = ftype;
    fn mul(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ftype {
    type Output = ftype;
    fn div(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ftype {
    type Output = ftype;
    fn rem(self, rhs: u32) -> ftype {
        ftype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_8 {
    GLOB_GLOBALL,
    GLOB_GETALL,
    GLOB_GETONE,
}
impl C2RustUnnamed_8 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_8::GLOB_GLOBALL => 0,
            C2RustUnnamed_8::GLOB_GETALL => 1,
            C2RustUnnamed_8::GLOB_GETONE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_8 {
        match value {
            0 => C2RustUnnamed_8::GLOB_GLOBALL,
            1 => C2RustUnnamed_8::GLOB_GETALL,
            2 => C2RustUnnamed_8::GLOB_GETONE,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum parsetype {
    TT_HOUR_MIN,
    TT_DAY,
}
impl parsetype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            parsetype::TT_HOUR_MIN => 0,
            parsetype::TT_DAY => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> parsetype {
        match value {
            0 => parsetype::TT_HOUR_MIN,
            1 => parsetype::TT_DAY,
            _ => panic!("Invalid value for parsetype: {}", value),
        }
    }
}
impl AddAssign<u32> for parsetype {
    fn add_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for parsetype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for parsetype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for parsetype {
    fn div_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for parsetype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = parsetype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for parsetype {
    type Output = parsetype;
    fn add(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for parsetype {
    type Output = parsetype;
    fn sub(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for parsetype {
    type Output = parsetype;
    fn mul(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for parsetype {
    type Output = parsetype;
    fn div(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for parsetype {
    type Output = parsetype;
    fn rem(self, rhs: u32) -> parsetype {
        parsetype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileinfo {
    pub type_0: ftype,
    pub name: *mut i8,
    pub size: wgint,
    pub tstamp: i64,
    pub ptype: parsetype,
    pub perms: i32,
    pub linkto: *mut i8,
    pub prev: *mut fileinfo,
    pub next: *mut fileinfo,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum wget_ftp_command {
    DO_LOGIN = 0x1,
    DO_CWD = 0x2,
    DO_RETR = 0x4,
    DO_LIST = 0x8,
    LEAVE_PENDING = 0x10,
}
impl wget_ftp_command {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            wget_ftp_command::DO_LOGIN => 0x1,
            wget_ftp_command::DO_CWD => 0x2,
            wget_ftp_command::DO_RETR => 0x4,
            wget_ftp_command::DO_LIST => 0x8,
            wget_ftp_command::LEAVE_PENDING => 0x10,
        }
    }
    fn from_libc_c_uint(value: u32) -> wget_ftp_command {
        match value {
            0x1 => wget_ftp_command::DO_LOGIN,
            0x2 => wget_ftp_command::DO_CWD,
            0x4 => wget_ftp_command::DO_RETR,
            0x8 => wget_ftp_command::DO_LIST,
            0x10 => wget_ftp_command::LEAVE_PENDING,
            _ => panic!("Invalid value for wget_ftp_command: {}", value),
        }
    }
}
impl AddAssign<u32> for wget_ftp_command {
    fn add_assign(&mut self, rhs: u32) {
        *self = wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for wget_ftp_command {
    fn sub_assign(&mut self, rhs: u32) {
        *self = wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for wget_ftp_command {
    fn mul_assign(&mut self, rhs: u32) {
        *self = wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for wget_ftp_command {
    fn div_assign(&mut self, rhs: u32) {
        *self = wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for wget_ftp_command {
    fn rem_assign(&mut self, rhs: u32) {
        *self = wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for wget_ftp_command {
    type Output = wget_ftp_command;
    fn add(self, rhs: u32) -> wget_ftp_command {
        wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for wget_ftp_command {
    type Output = wget_ftp_command;
    fn sub(self, rhs: u32) -> wget_ftp_command {
        wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for wget_ftp_command {
    type Output = wget_ftp_command;
    fn mul(self, rhs: u32) -> wget_ftp_command {
        wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for wget_ftp_command {
    type Output = wget_ftp_command;
    fn div(self, rhs: u32) -> wget_ftp_command {
        wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for wget_ftp_command {
    type Output = wget_ftp_command;
    fn rem(self, rhs: u32) -> wget_ftp_command {
        wget_ftp_command::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum wget_ftp_fstatus {
    NOTHING = 0x0,
    ON_YOUR_OWN = 0x1,
    DONE_CWD = 0x2,
    AVOID_LIST_A = 0x4,
    AVOID_LIST = 0x8,
    LIST_AFTER_LIST_A_CHECK_DONE = 0x10,
    DATA_CHANNEL_SECURITY = 0x20,
}
impl wget_ftp_fstatus {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            wget_ftp_fstatus::NOTHING => 0x0,
            wget_ftp_fstatus::ON_YOUR_OWN => 0x1,
            wget_ftp_fstatus::DONE_CWD => 0x2,
            wget_ftp_fstatus::AVOID_LIST_A => 0x4,
            wget_ftp_fstatus::AVOID_LIST => 0x8,
            wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE => 0x10,
            wget_ftp_fstatus::DATA_CHANNEL_SECURITY => 0x20,
        }
    }
    fn from_libc_c_uint(value: u32) -> wget_ftp_fstatus {
        match value {
            0x0 => wget_ftp_fstatus::NOTHING,
            0x1 => wget_ftp_fstatus::ON_YOUR_OWN,
            0x2 => wget_ftp_fstatus::DONE_CWD,
            0x4 => wget_ftp_fstatus::AVOID_LIST_A,
            0x8 => wget_ftp_fstatus::AVOID_LIST,
            0x10 => wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE,
            0x20 => wget_ftp_fstatus::DATA_CHANNEL_SECURITY,
            _ => panic!("Invalid value for wget_ftp_fstatus: {}", value),
        }
    }
}
impl AddAssign<u32> for wget_ftp_fstatus {
    fn add_assign(&mut self, rhs: u32) {
        *self = wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for wget_ftp_fstatus {
    fn sub_assign(&mut self, rhs: u32) {
        *self = wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for wget_ftp_fstatus {
    fn mul_assign(&mut self, rhs: u32) {
        *self = wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for wget_ftp_fstatus {
    fn div_assign(&mut self, rhs: u32) {
        *self = wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for wget_ftp_fstatus {
    fn rem_assign(&mut self, rhs: u32) {
        *self = wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for wget_ftp_fstatus {
    type Output = wget_ftp_fstatus;
    fn add(self, rhs: u32) -> wget_ftp_fstatus {
        wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for wget_ftp_fstatus {
    type Output = wget_ftp_fstatus;
    fn sub(self, rhs: u32) -> wget_ftp_fstatus {
        wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for wget_ftp_fstatus {
    type Output = wget_ftp_fstatus;
    fn mul(self, rhs: u32) -> wget_ftp_fstatus {
        wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for wget_ftp_fstatus {
    type Output = wget_ftp_fstatus;
    fn div(self, rhs: u32) -> wget_ftp_fstatus {
        wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for wget_ftp_fstatus {
    type Output = wget_ftp_fstatus;
    fn rem(self, rhs: u32) -> wget_ftp_fstatus {
        wget_ftp_fstatus::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccon {
    pub st: i32,
    pub cmd: i32,
    pub csock: i32,
    pub dltime: libc::c_double,
    pub rs: stype,
    pub rsu: ustype,
    pub id: *mut i8,
    pub target: *mut i8,
    pub proxy: *mut url,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_10 {
    ENDPOINT_LOCAL,
    ENDPOINT_PEER,
}
impl C2RustUnnamed_10 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_10::ENDPOINT_LOCAL => 0,
            C2RustUnnamed_10::ENDPOINT_PEER => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_10 {
        match value {
            0 => C2RustUnnamed_10::ENDPOINT_LOCAL,
            1 => C2RustUnnamed_10::ENDPOINT_PEER,
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
pub const E_HOST: C2RustUnnamed_9 = -100;
pub type C2RustUnnamed_9 = i32;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isalpha(mut c: i32) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as i32 != 0,
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
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
unsafe extern "C" fn ftp_expected_bytes(mut s: *const i8) -> wgint {
    let mut res: wgint = 0;
    loop {
        while *s as i32 != 0 && *s as i32 != '(' as i32 {
            s = s.offset(1);
            s;
        }
        if *s == 0 {
            return 0 as i32 as wgint;
        }
        s = s.offset(1);
        s;
        res = rpl_strtoll(s, &mut s as *mut *const i8 as *mut *mut i8, 10 as i32)
            as wgint;
        if *s == 0 {
            return 0 as i32 as wgint;
        }
        while *s as i32 != 0 && c_isspace(*s as i32) as i32 != 0 {
            s = s.offset(1);
            s;
        }
        if *s == 0 {
            return 0 as i32 as wgint;
        }
        if c_tolower(*s as i32) != 'b' as i32 {
            continue;
        }
        if !(c_strncasecmp(s, b"byte\0" as *const u8 as *const i8, 4 as i32 as size_t)
            != 0)
        {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn ftp_do_pasv(
    mut csock: i32,
    mut addr: *mut ip_address,
    mut port: *mut i32,
) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    if !socket_ip_address(csock, addr, C2RustUnnamed_10::ENDPOINT_PEER as i32) {
        abort();
    }
    match (*addr).family {
        2 => {
            if !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    b"==> PASV ... \0" as *const u8 as *const i8,
                );
            }
            err = ftp_pasv(csock, addr, port);
        }
        10 => {
            if !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    b"==> EPSV ... \0" as *const u8 as *const i8,
                );
            }
            err = ftp_epsv(csock, addr, port);
            if err as u32 == uerr_t::FTPNOPASV as i32 as u32 {
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        b"==> LPSV ... \0" as *const u8 as *const i8,
                    );
                }
                err = ftp_lpsv(csock, addr, port);
            }
        }
        _ => {
            abort();
        }
    }
    return err;
}
unsafe extern "C" fn ftp_do_port(mut csock: i32, mut local_sock: *mut i32) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut cip: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_7 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    if !socket_ip_address(csock, &mut cip, C2RustUnnamed_10::ENDPOINT_PEER as i32) {
        abort();
    }
    match cip.family {
        2 => {
            if !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    b"==> PORT ... \0" as *const u8 as *const i8,
                );
            }
            err = ftp_port(csock, local_sock);
        }
        10 => {
            if !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    b"==> EPRT ... \0" as *const u8 as *const i8,
                );
            }
            err = ftp_eprt(csock, local_sock);
            if err as u32 == uerr_t::FTPPORTERR as i32 as u32 {
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        b"==> LPRT ... \0" as *const u8 as *const i8,
                    );
                }
                err = ftp_lprt(csock, local_sock);
            }
        }
        _ => {
            abort();
        }
    }
    return err;
}
unsafe extern "C" fn print_length(
    mut size: wgint,
    mut start: wgint,
    mut authoritative: bool,
) {
    logprintf(
        log_options::LOG_VERBOSE,
        dcgettext(0 as *const i8, b"Length: %s\0" as *const u8 as *const i8, 5 as i32),
        number_to_static_string(size),
    );
    if size >= 1024 as i32 as i64 {
        logprintf(
            log_options::LOG_VERBOSE,
            b" (%s)\0" as *const u8 as *const i8,
            human_readable(size, 10 as i32, 1 as i32),
        );
    }
    if start > 0 as i32 as i64 {
        if size - start >= 1024 as i32 as i64 {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b", %s (%s) remaining\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                number_to_static_string(size - start),
                human_readable(size - start, 10 as i32, 1 as i32),
            );
        } else {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b", %s remaining\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                number_to_static_string(size - start),
            );
        }
    }
    logputs(
        log_options::LOG_VERBOSE,
        if !authoritative {
            dcgettext(
                0 as *const i8,
                b" (unauthoritative)\n\0" as *const u8 as *const i8,
                5 as i32,
            )
        } else {
            b"\n\0" as *const u8 as *const i8
        },
    );
}
unsafe extern "C" fn get_ftp_greeting(mut csock: i32, mut con: *mut ccon) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    err = ftp_greeting(csock);
    if err as u32 != uerr_t::FTPOK as i32 as u32 {
        logputs(
            log_options::LOG_NOTQUIET,
            b"Error in server response. Closing.\n\0" as *const u8 as *const i8,
        );
        fd_close(csock);
        (*con).csock = -(1 as i32);
    }
    return err;
}
unsafe extern "C" fn init_control_ssl_connection(
    mut csock: i32,
    mut u: *mut url,
    mut using_control_security: *mut bool,
) -> uerr_t {
    let mut using_security: bool = 0 as i32 != 0;
    if !opt.ftps_implicit && !opt.server_response {
        logputs(
            log_options::LOG_VERBOSE,
            b"==> AUTH TLS ... \0" as *const u8 as *const i8,
        );
    }
    if opt.ftps_implicit as i32 != 0
        || ftp_auth(csock, url_scheme::SCHEME_FTPS) as u32 == uerr_t::FTPOK as i32 as u32
    {
        if !ssl_connect_wget(csock, (*u).host, 0 as *mut i32) {
            fd_close(csock);
            return uerr_t::CONSSLERR;
        } else if !ssl_check_certificate(csock, (*u).host) {
            fd_close(csock);
            return uerr_t::VERIFCERTERR;
        }
        if !opt.ftps_implicit && !opt.server_response {
            logputs(log_options::LOG_VERBOSE, b" done.\n\0" as *const u8 as *const i8);
        }
        using_security = 1 as i32 != 0;
    } else if opt.ftps_fallback_to_ftp {
        logputs(
            log_options::LOG_NOTQUIET,
            b"Server does not support AUTH TLS. Falling back to FTP.\n\0" as *const u8
                as *const i8,
        );
        using_security = 0 as i32 != 0;
    } else {
        fd_close(csock);
        return uerr_t::FTPNOAUTH;
    }
    *using_control_security = using_security;
    return uerr_t::NOCONERROR;
}
unsafe extern "C" fn getftp(
    mut u: *mut url,
    mut original_url: *mut url,
    mut passed_expected_bytes: wgint,
    mut qtyread: *mut wgint,
    mut restval: wgint,
    mut con: *mut ccon,
    mut count: i32,
    mut last_expected_bytes: *mut wgint,
    mut warc_tmp: *mut FILE,
) -> uerr_t {
    let mut current_block: u64;
    let mut csock: i32 = 0;
    let mut dtsock: i32 = 0;
    let mut local_sock: i32 = 0;
    let mut res: i32 = 0;
    let mut err: uerr_t = uerr_t::RETROK;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut respline: *mut i8 = 0 as *mut i8;
    let mut tms: *mut i8 = 0 as *mut i8;
    let mut user: *const i8 = 0 as *const i8;
    let mut passwd: *const i8 = 0 as *const i8;
    let mut tmrate: *const i8 = 0 as *const i8;
    let mut cmd: i32 = (*con).cmd;
    let mut expected_bytes: wgint = 0 as i32 as wgint;
    let mut got_expected_bytes: bool = 0 as i32 != 0;
    let mut rest_failed: bool = 0 as i32 != 0;
    let mut flags: i32 = 0;
    let mut rd_size: wgint = 0;
    let mut previous_rd_size: wgint = 0 as i32 as wgint;
    let mut type_char: i8 = 0;
    let mut try_again: bool = false;
    let mut list_a_used: bool = 0 as i32 != 0;
    let mut prot: prot_level = prot_level::from_libc_c_uint(
        (if opt.ftps_clear_data_connection as i32 != 0 {
            prot_level::PROT_CLEAR as i32
        } else {
            prot_level::PROT_PRIVATE as i32
        }) as u32,
    );
    let mut using_control_security: bool = 0 as i32 != 0;
    let mut using_data_security: bool = 0 as i32 != 0;
    *qtyread = restval;
    if !((*u).user).is_null() {
        user = (*u).user;
    } else if !(opt.user).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as i32 != 0)
    {
        user = opt.user;
    } else if !(opt.ftp_user).is_null() {
        user = opt.ftp_user;
    } else if !(opt.user).is_null() {
        user = opt.user;
    } else {
        user = 0 as *const i8;
    }
    if !((*u).passwd).is_null() {
        passwd = (*u).passwd;
    } else if !(opt.passwd).is_null()
        && (!(opt.use_askpass).is_null() || opt.ask_passwd as i32 != 0)
    {
        passwd = opt.passwd;
    } else if !(opt.ftp_passwd).is_null() {
        passwd = opt.ftp_passwd;
    } else if !(opt.passwd).is_null() {
        passwd = opt.passwd;
    } else {
        passwd = 0 as *const i8;
    }
    if opt.netrc as i32 != 0 && (user.is_null() || passwd.is_null()) {
        search_netrc(
            (*u).host,
            &mut user as *mut *const i8,
            &mut passwd as *mut *const i8,
            1 as i32,
            0 as *mut FILE,
        );
    }
    if user.is_null() {
        user = b"anonymous\0" as *const u8 as *const i8;
    }
    if passwd.is_null() {
        passwd = b"-wget@\0" as *const u8 as *const i8;
    }
    dtsock = -(1 as i32);
    local_sock = -(1 as i32);
    (*con).dltime = 0 as i32 as libc::c_double;
    if (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32 {
        if !ssl_init() {
            scheme_disable(url_scheme::SCHEME_FTPS);
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Could not initialize SSL. It will be disabled.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            err = uerr_t::SSLINITFAILED;
            return err;
        }
        if opt.ftps_implicit as i32 != 0 && (*u).port == 21 as i32 {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Implicit FTPS was specified. Rewriting default port to %d.\n\0"
                        as *const u8 as *const i8,
                    990 as i32,
                );
            }
            (*u).port = 990 as i32;
        }
    }
    if cmd & wget_ftp_command::DO_LOGIN as i32 == 0 {
        csock = (*con).csock;
        using_data_security = (*con).st & wget_ftp_fstatus::DATA_CHANNEL_SECURITY as i32
            != 0;
    } else {
        let mut host: *mut i8 = if !((*con).proxy).is_null() {
            (*(*con).proxy).host
        } else {
            (*u).host
        };
        let mut port: i32 = if !((*con).proxy).is_null() {
            (*(*con).proxy).port
        } else {
            (*u).port
        };
        csock = connect_to_host(host, port);
        if csock == E_HOST as i32 {
            return uerr_t::HOSTERR
        } else if csock < 0 as i32 {
            return uerr_t::from_libc_c_uint(
                (if retryable_socket_connect_error(*__errno_location()) as i32 != 0 {
                    uerr_t::CONERROR as i32
                } else {
                    uerr_t::CONIMPOSSIBLE as i32
                }) as u32,
            )
        }
        if cmd & wget_ftp_command::LEAVE_PENDING as i32 != 0 {
            (*con).csock = csock;
        } else {
            (*con).csock = -(1 as i32);
        }
        if (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32 {
            if opt.ftps_implicit {
                err = init_control_ssl_connection(csock, u, &mut using_control_security);
                if err as u32 != uerr_t::NOCONERROR as i32 as u32 {
                    return err;
                }
                err = get_ftp_greeting(csock, con);
                if err as u32 != uerr_t::FTPOK as i32 as u32 {
                    return err;
                }
            } else {
                err = get_ftp_greeting(csock, con);
                if err as u32 != uerr_t::FTPOK as i32 as u32 {
                    return err;
                }
                err = init_control_ssl_connection(csock, u, &mut using_control_security);
                if err as u32 != uerr_t::NOCONERROR as i32 as u32 {
                    return err;
                }
            }
        } else {
            err = get_ftp_greeting(csock, con);
            if err as u32 != uerr_t::FTPOK as i32 as u32 {
                return err;
            }
        }
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Logging in as %s ... \0" as *const u8 as *const i8,
                5 as i32,
            ),
            quotearg_style(quoting_style::escape_quoting_style, user),
        );
        if opt.server_response {
            logputs(log_options::LOG_ALWAYS, b"\n\0" as *const u8 as *const i8);
        }
        if !((*con).proxy).is_null() {
            let mut logname: *mut i8 = concat_strings(
                user,
                b"@\0" as *const u8 as *const i8,
                (*u).host,
                0 as *mut i8,
            );
            err = ftp_login(csock, logname, passwd);
            rpl_free(logname as *mut libc::c_void);
            logname = 0 as *mut i8;
        } else {
            err = ftp_login(csock, user, passwd);
        }
        match err as u32 {
            14 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            15 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error in server greeting.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            44 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Write failed, closing control connection.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            9 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"The server refuses login.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return uerr_t::FTPLOGREFUSED;
            }
            8 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Login incorrect.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return uerr_t::FTPLOGINC;
            }
            7 => {
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"Logged in!\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
            _ => {
                abort();
            }
        }
        if using_control_security {
            if (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32 {
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        b"==> PBSZ 0 ... \0" as *const u8 as *const i8,
                    );
                }
                err = ftp_pbsz(csock, 0 as i32);
                if err as u32 == uerr_t::FTPNOPBSZ as i32 as u32 {
                    logputs(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Server did not accept the 'PBSZ 0' command.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    return err;
                }
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        b"done.\0" as *const u8 as *const i8,
                    );
                }
                if !opt.server_response {
                    logprintf(
                        log_options::LOG_VERBOSE,
                        b"  ==> PROT %c ... \0" as *const u8 as *const i8,
                        prot as i32,
                    );
                }
                err = ftp_prot(csock, prot);
                if err as u32 == uerr_t::FTPNOPROT as i32 as u32 {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Server did not accept the 'PROT %c' command.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        prot as i32,
                    );
                    return err;
                }
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        b"done.\n\0" as *const u8 as *const i8,
                    );
                }
                if prot as u32 != prot_level::PROT_CLEAR as i32 as u32 {
                    using_data_security = 1 as i32 != 0;
                    (*con).st |= wget_ftp_fstatus::DATA_CHANNEL_SECURITY as i32;
                }
            }
        }
        if !opt.server_response {
            logprintf(
                log_options::LOG_VERBOSE,
                b"==> SYST ... \0" as *const u8 as *const i8,
            );
        }
        err = ftp_syst(csock, &mut (*con).rs, &mut (*con).rsu);
        match err as u32 {
            14 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            15 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Server error, can't determine system type.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            7 => {}
            _ => {
                abort();
            }
        }
        if !opt.server_response && err as u32 != uerr_t::FTPSRVERR as i32 as u32 {
            logputs(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"done.    \0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        match (*con).rs as u32 {
            1 => {
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"\nVMS: I know it and I will use \"LIST\" as standard list command\n\0"
                            as *const u8 as *const i8,
                    );
                }
                (*con).st |= wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32;
                (*con).st |= wget_ftp_fstatus::AVOID_LIST_A as i32;
            }
            0 => {
                if (*con).rsu as u32 == ustype::UST_MULTINET as i32 as u32 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"\nUNIX MultiNet: I know it and I will use \"LIST\" as standard list command\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    (*con).st |= wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32;
                    (*con).st |= wget_ftp_fstatus::AVOID_LIST_A as i32;
                } else if (*con).rsu as u32 == ustype::UST_TYPE_L8 as i32 as u32 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"\nUNIX TYPE L8: I know it and I will use \"LIST -a\" as standard list command\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    (*con).st |= wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32;
                    (*con).st |= wget_ftp_fstatus::AVOID_LIST as i32;
                }
            }
            _ => {}
        }
        if !opt.server_response {
            logprintf(
                log_options::LOG_VERBOSE,
                b"==> PWD ... \0" as *const u8 as *const i8,
            );
        }
        err = ftp_pwd(csock, &mut (*con).id);
        match err as u32 {
            14 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            15 => {
                rpl_free((*con).id as *mut libc::c_void);
                (*con).id = 0 as *mut i8;
                (*con).id = xstrdup(b"/\0" as *const u8 as *const i8);
            }
            7 => {}
            _ => {
                abort();
            }
        }
        if !opt.server_response {
            logputs(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"done.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        type_char = ftp_process_type((*u).params);
        if !opt.server_response {
            logprintf(
                log_options::LOG_VERBOSE,
                b"==> TYPE %c ... \0" as *const u8 as *const i8,
                type_char as i32,
            );
        }
        err = ftp_type(csock, type_char as i32);
        match err as u32 {
            14 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            44 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Write failed, closing control connection.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            13 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Unknown type `%c', closing control connection.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    type_char as i32,
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            7 => {}
            _ => {
                abort();
            }
        }
        if !opt.server_response {
            logputs(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"done.  \0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    if cmd & wget_ftp_command::DO_CWD as i32 != 0 {
        if *(*u).dir == 0 {
            logputs(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"==> CWD not needed.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else {
            let mut targ: *const i8 = 0 as *const i8;
            let mut target: *mut i8 = (*u).dir;
            let mut targetbuf: [i8; 1024] = [0; 1024];
            let mut cwd_count: i32 = 0;
            let mut cwd_end: i32 = 0;
            let mut cwd_start: i32 = 0;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"changing working directory\n\0" as *const u8 as *const i8,
                );
            }
            if *target.offset(0 as i32 as isize) as i32 != '/' as i32
                && !((*con).rs as u32 != stype::ST_UNIX as i32 as u32
                    && c_isalpha(*target.offset(0 as i32 as isize) as i32) as i32 != 0
                    && *target.offset(1 as i32 as isize) as i32 == ':' as i32)
                && (*con).rs as u32 != stype::ST_OS400 as i32 as u32
                && (*con).rs as u32 != stype::ST_VMS as i32 as u32
            {
                let mut ntarget: *mut i8 = 0 as *mut i8;
                let mut p: *mut i8 = 0 as *mut i8;
                let mut idlen: size_t = strlen((*con).id);
                let mut len: size_t = 0;
                while idlen > 0 as i32 as u64
                    && *((*con).id).offset(idlen.wrapping_sub(1 as i32 as u64) as isize)
                        as i32 == '/' as i32
                {
                    idlen = idlen.wrapping_sub(1);
                    idlen;
                }
                len = idlen.wrapping_add(1 as i32 as u64).wrapping_add(strlen(target));
                if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
                    ntarget = targetbuf.as_mut_ptr();
                    p = ntarget;
                } else {
                    ntarget = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
                    p = ntarget;
                }
                memcpy(p as *mut libc::c_void, (*con).id as *const libc::c_void, idlen);
                p = p.offset(idlen as isize);
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = '/' as i32 as i8;
                strcpy(p, target);
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Prepended initial PWD to relative path:\n\0" as *const u8
                            as *const i8,
                    );
                }
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"   pwd: '%s'\n   old: '%s'\n  new: '%s'\n\0" as *const u8
                            as *const i8,
                        (*con).id,
                        target,
                        ntarget,
                    );
                }
                target = ntarget;
            }
            if (*con).rs as u32 == stype::ST_VMS as i32 as u32
                && *target.offset(0 as i32 as isize) as i32 != '/' as i32
            {
                cwd_start = 0 as i32;
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Using two-step CWD for relative path.\n\0" as *const u8
                            as *const i8,
                    );
                }
            } else {
                cwd_start = 1 as i32;
            }
            if (*con).rs as u32 == stype::ST_VMS as i32 as u32
                && !(strchr(target, '/' as i32)).is_null()
            {
                cwd_end = 3 as i32;
                if opt.debug as i64 != 0 {
                    debug_logprintf(
                        b"Using extra \"CWD []\" step for VMS server.\n\0" as *const u8
                            as *const i8,
                    );
                }
            } else {
                cwd_end = 2 as i32;
            }
            cwd_count = cwd_start;
            while cwd_count < cwd_end {
                match cwd_count {
                    0 => {
                        targ = (*con).id;
                    }
                    1 => {
                        targ = target;
                    }
                    2 => {
                        targ = b"[]\0" as *const u8 as *const i8;
                    }
                    _ => {
                        logprintf(
                            log_options::LOG_ALWAYS,
                            dcgettext(
                                0 as *const i8,
                                b"Logically impossible section reached in getftp()\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        logprintf(
                            log_options::LOG_ALWAYS,
                            dcgettext(
                                0 as *const i8,
                                b"cwd_count: %d\ncwd_start: %d\ncwd_end: %d\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            cwd_count,
                            cwd_start,
                            cwd_end,
                        );
                        abort();
                    }
                }
                if !opt.server_response {
                    logprintf(
                        log_options::LOG_VERBOSE,
                        b"==> CWD (%d) %s ... \0" as *const u8 as *const i8,
                        cwd_count,
                        quotearg_style(quoting_style::escape_quoting_style, target),
                    );
                }
                err = ftp_cwd(csock, targ);
                match err as u32 {
                    14 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Error in server response, closing control connection.\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        return err;
                    }
                    44 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Write failed, closing control connection.\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        return err;
                    }
                    12 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"No such directory %s.\n\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            quote((*u).dir),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        return err;
                    }
                    7 => {}
                    _ => {
                        abort();
                    }
                }
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"done.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                cwd_count += 1;
                cwd_count;
            }
        }
    } else {
        logputs(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"==> CWD not required.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if cmd & wget_ftp_command::DO_RETR as i32 != 0
        && passed_expected_bytes == 0 as i32 as i64
    {
        if opt.verbose != 0 {
            if !opt.server_response {
                logprintf(
                    log_options::LOG_VERBOSE,
                    b"==> SIZE %s ... \0" as *const u8 as *const i8,
                    quotearg_style(quoting_style::escape_quoting_style, (*u).file),
                );
            }
        }
        err = ftp_size(csock, (*u).file, &mut expected_bytes);
        match err as u32 {
            14 | 15 => {
                logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                logputs(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error in server response, closing control connection.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                return err;
            }
            7 => {
                got_expected_bytes = 1 as i32 != 0;
            }
            _ => {
                abort();
            }
        }
        if !opt.server_response {
            logprintf(
                log_options::LOG_VERBOSE,
                b"%s\n\0" as *const u8 as *const i8,
                if expected_bytes != 0 {
                    number_to_static_string(expected_bytes)
                } else {
                    dcgettext(
                        0 as *const i8,
                        b"done.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    )
                },
            );
        }
    }
    if cmd & wget_ftp_command::DO_RETR as i32 != 0 && restval > 0 as i32 as i64
        && restval == expected_bytes
    {
        logputs(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"File has already been retrieved.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fd_close(csock);
        (*con).csock = -(1 as i32);
        return uerr_t::RETRFINISHED;
    }
    loop {
        try_again = 0 as i32 != 0;
        if cmd & (wget_ftp_command::DO_LIST as i32 | wget_ftp_command::DO_RETR as i32)
            != 0
        {
            if opt.ftp_pasv {
                let mut passive_addr: ip_address = ip_address {
                    family: 0,
                    data: C2RustUnnamed_7 {
                        d4: in_addr { s_addr: 0 },
                    },
                    ipv6_scope: 0,
                };
                let mut passive_port: i32 = 0;
                err = ftp_do_pasv(csock, &mut passive_addr, &mut passive_port);
                match err as u32 {
                    14 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Error in server response, closing control connection.\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        return err;
                    }
                    44 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Write failed, closing control connection.\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        return err;
                    }
                    29 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Cannot initiate PASV transfer.\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    28 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Cannot parse PASV response.\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    7 => {}
                    _ => {
                        abort();
                    }
                }
                if err as u32 == uerr_t::FTPOK as i32 as u32 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"trying to connect to %s port %d\n\0" as *const u8
                                as *const i8,
                            print_address(&mut passive_addr),
                            passive_port,
                        );
                    }
                    dtsock = connect_to_ip(
                        &mut passive_addr,
                        passive_port,
                        0 as *const i8,
                    );
                    if dtsock < 0 as i32 {
                        let mut save_errno: i32 = *__errno_location();
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        logprintf(
                            log_options::LOG_VERBOSE,
                            dcgettext(
                                0 as *const i8,
                                b"couldn't connect to %s port %d: %s\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            print_address(&mut passive_addr),
                            passive_port,
                            strerror(save_errno),
                        );
                        return uerr_t::from_libc_c_uint(
                            (if retryable_socket_connect_error(save_errno) as i32 != 0 {
                                uerr_t::CONERROR as i32
                            } else {
                                uerr_t::CONIMPOSSIBLE as i32
                            }) as u32,
                        );
                    }
                    if !opt.server_response {
                        logputs(
                            log_options::LOG_VERBOSE,
                            dcgettext(
                                0 as *const i8,
                                b"done.    \0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                } else {
                    return err
                }
            } else {
                err = ftp_do_port(csock, &mut local_sock);
                match err as u32 {
                    14 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Error in server response, closing control connection.\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    44 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Write failed, closing control connection.\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    2 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            b"socket: %s\n\0" as *const u8 as *const i8,
                            strerror(*__errno_location()),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    11 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Bind error (%s).\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            strerror(*__errno_location()),
                        );
                        fd_close(dtsock);
                        return err;
                    }
                    10 => {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Invalid PORT.\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return err;
                    }
                    7 => {}
                    _ => {
                        abort();
                    }
                }
                if !opt.server_response {
                    logputs(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"done.    \0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
        }
        if restval != 0 && cmd & wget_ftp_command::DO_RETR as i32 != 0 {
            if !opt.server_response {
                logprintf(
                    log_options::LOG_VERBOSE,
                    b"==> REST %s ... \0" as *const u8 as *const i8,
                    number_to_static_string(restval),
                );
            }
            err = ftp_rest(csock, restval);
            match err as u32 {
                14 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logputs(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Error in server response, closing control connection.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as i32);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                44 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logputs(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Write failed, closing control connection.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as i32);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                17 => {
                    logputs(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"\nREST failed, starting from scratch.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    rest_failed = 1 as i32 != 0;
                }
                7 => {}
                _ => {
                    abort();
                }
            }
            if err as u32 != uerr_t::FTPRESTFAIL as i32 as u32 && !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"done.    \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
        }
        if cmd & wget_ftp_command::DO_RETR as i32 != 0 {
            if opt.spider {
                let mut exists: bool = 0 as i32 != 0;
                let mut all_exist: bool = 1 as i32 != 0;
                let mut f: *mut fileinfo = 0 as *mut fileinfo;
                let mut _res: uerr_t = ftp_get_listing(u, original_url, con, &mut f);
                (*con).cmd |= wget_ftp_command::DO_RETR as i32;
                if _res as u32 == uerr_t::RETROK as i32 as u32 {
                    while !f.is_null() {
                        if strcmp((*f).name, (*u).file) == 0 {
                            exists = 1 as i32 != 0;
                            break;
                        } else {
                            all_exist = 0 as i32 != 0;
                            f = (*f).next;
                        }
                    }
                    if exists {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"File %s exists.\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            quote((*u).file),
                        );
                    } else {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"No such file %s.\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            quote((*u).file),
                        );
                    }
                }
                fd_close(csock);
                (*con).csock = -(1 as i32);
                fd_close(dtsock);
                fd_close(local_sock);
                if all_exist {
                    return uerr_t::RETRFINISHED
                } else {
                    return uerr_t::FTPNSFOD
                }
            }
            if opt.verbose != 0 {
                if !opt.server_response {
                    if restval != 0 {
                        logputs(
                            log_options::LOG_VERBOSE,
                            b"\n\0" as *const u8 as *const i8,
                        );
                    }
                    logprintf(
                        log_options::LOG_VERBOSE,
                        b"==> RETR %s ... \0" as *const u8 as *const i8,
                        quotearg_style(quoting_style::escape_quoting_style, (*u).file),
                    );
                }
            }
            err = ftp_retr(csock, (*u).file);
            match err as u32 {
                14 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logputs(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Error in server response, closing control connection.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as i32);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                44 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logputs(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Write failed, closing control connection.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as i32);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                12 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"No such file %s.\n\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote((*u).file),
                    );
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                7 => {}
                _ => {
                    abort();
                }
            }
            if !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"done.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            if !got_expected_bytes {
                expected_bytes = *last_expected_bytes;
            }
        }
        if cmd & wget_ftp_command::DO_LIST as i32 != 0 {
            if !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    b"==> LIST ... \0" as *const u8 as *const i8,
                );
            }
            err = ftp_list(
                csock,
                0 as *const i8,
                (*con).st & wget_ftp_fstatus::AVOID_LIST_A as i32 != 0,
                (*con).st & wget_ftp_fstatus::AVOID_LIST as i32 != 0,
                &mut list_a_used,
            );
            match err as u32 {
                14 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logputs(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Error in server response, closing control connection.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as i32);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                44 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logputs(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Write failed, closing control connection.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as i32);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                12 => {
                    logputs(log_options::LOG_VERBOSE, b"\n\0" as *const u8 as *const i8);
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"No such file or directory %s.\n\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        quote(b".\0" as *const u8 as *const i8),
                    );
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return err;
                }
                7 => {}
                _ => {
                    abort();
                }
            }
            if !opt.server_response {
                logputs(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"done.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            if !got_expected_bytes {
                expected_bytes = *last_expected_bytes;
            }
        }
        if cmd & (wget_ftp_command::DO_LIST as i32 | wget_ftp_command::DO_RETR as i32)
            == 0 || opt.spider as i32 != 0 && cmd & wget_ftp_command::DO_LIST as i32 == 0
        {
            return uerr_t::RETRFINISHED;
        }
        if passed_expected_bytes != 0 && restval != 0 && expected_bytes != 0
            && expected_bytes == passed_expected_bytes - restval
        {
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Lying FTP server found, adjusting.\n\0" as *const u8 as *const i8,
                );
            }
            expected_bytes = passed_expected_bytes;
        }
        if !opt.ftp_pasv {
            dtsock = accept_connection(local_sock);
            if dtsock < 0 as i32 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"accept: %s\n\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
                return uerr_t::CONERROR;
            }
        }
        if output_stream.is_null() || (*con).cmd & wget_ftp_command::DO_LIST as i32 != 0
        {
            mkalldirs((*con).target);
            if opt.backups != 0 {
                rotate_backups((*con).target);
            }
            if restval != 0 && (*con).cmd & wget_ftp_command::DO_LIST as i32 == 0 {
                fp = rpl_fopen((*con).target, b"ab\0" as *const u8 as *const i8);
            } else if opt.noclobber as i32 != 0 || opt.always_rest as i32 != 0
                || opt.timestamping as i32 != 0 || opt.dirstruct as i32 != 0
                || !(opt.output_document).is_null() || count > 0 as i32
            {
                if opt.unlink_requested as i32 != 0
                    && file_exists_p((*con).target, 0 as *mut file_stats_t) as i32 != 0
                {
                    if unlink((*con).target) < 0 as i32 {
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            b"%s: %s\n\0" as *const u8 as *const i8,
                            (*con).target,
                            strerror(*__errno_location()),
                        );
                        fd_close(csock);
                        (*con).csock = -(1 as i32);
                        fd_close(dtsock);
                        fd_close(local_sock);
                        return uerr_t::UNLINKERR;
                    }
                }
                fp = rpl_fopen((*con).target, b"wb\0" as *const u8 as *const i8);
            } else {
                fp = fopen_excl((*con).target, 1 as i32);
                if fp.is_null() && *__errno_location() == 17 as i32 {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"%s has sprung into existence.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*con).target,
                    );
                    fd_close(csock);
                    (*con).csock = -(1 as i32);
                    fd_close(dtsock);
                    fd_close(local_sock);
                    return uerr_t::FOPEN_EXCL_ERR;
                }
            }
            if fp.is_null() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"%s: %s\n\0" as *const u8 as *const i8,
                    (*con).target,
                    strerror(*__errno_location()),
                );
                fd_close(csock);
                (*con).csock = -(1 as i32);
                fd_close(dtsock);
                fd_close(local_sock);
                return uerr_t::FOPENERR;
            }
        } else {
            fp = output_stream;
        }
        if passed_expected_bytes != 0 {
            print_length(passed_expected_bytes, restval, 1 as i32 != 0);
            expected_bytes = passed_expected_bytes;
        } else if expected_bytes != 0 {
            print_length(expected_bytes, restval, 0 as i32 != 0);
        }
        if (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32
            && using_data_security as i32 != 0
        {
            if !opt.ftps_resume_ssl || !ssl_connect_wget(dtsock, (*u).host, &mut csock) {
                if opt.ftps_resume_ssl {
                    logputs(
                        log_options::LOG_NOTQUIET,
                        b"Server does not want to resume the SSL session. Trying with a new one.\n\0"
                            as *const u8 as *const i8,
                    );
                }
                if !ssl_connect_wget(dtsock, (*u).host, 0 as *mut i32) {
                    fd_close(csock);
                    fd_close(dtsock);
                    err = uerr_t::CONERROR;
                    logputs(
                        log_options::LOG_NOTQUIET,
                        b"Could not perform SSL handshake.\n\0" as *const u8 as *const i8,
                    );
                    current_block = 931272115498443513;
                    break;
                }
            } else {
                logputs(
                    log_options::LOG_NOTQUIET,
                    b"Resuming SSL session in data connection.\n\0" as *const u8
                        as *const i8,
                );
            }
            if !ssl_check_certificate(dtsock, (*u).host) {
                fd_close(csock);
                fd_close(dtsock);
                err = uerr_t::CONERROR;
                current_block = 931272115498443513;
                break;
            }
        }
        flags = 0 as i32;
        if restval != 0 && rest_failed as i32 != 0 {
            flags |= C2RustUnnamed_5::rb_skip_startpos as i32;
        }
        rd_size = 0 as i32 as wgint;
        res = fd_read_body(
            (*con).target,
            dtsock,
            fp,
            if expected_bytes != 0 { expected_bytes - restval } else { 0 as i32 as i64 },
            restval,
            &mut rd_size,
            qtyread,
            &mut (*con).dltime,
            flags,
            warc_tmp,
        );
        tms = datetime_str(time(0 as *mut time_t));
        tmrate = retr_rate(rd_size, (*con).dltime);
        total_download_time += (*con).dltime;
        if opt.enable_xattr {
            set_file_metadata(u, 0 as *const url, fp);
        }
        fd_close(local_sock);
        if output_stream.is_null() || (*con).cmd & wget_ftp_command::DO_LIST as i32 != 0
        {
            fclose(fp);
        }
        if res == -(2 as i32) || !warc_tmp.is_null() && res == -(3 as i32) {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: %s, closing control connection.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*con).target,
                strerror(*__errno_location()),
            );
            fd_close(csock);
            (*con).csock = -(1 as i32);
            fd_close(dtsock);
            if res == -(2 as i32) {
                return uerr_t::FWRITEERR
            } else if res == -(3 as i32) {
                return uerr_t::WARC_TMP_FWRITEERR
            }
        } else if res == -(1 as i32) {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s (%s) - Data connection: %s; \0" as *const u8 as *const i8,
                    5 as i32,
                ),
                tms,
                tmrate,
                fd_errstr(dtsock),
            );
            if opt.server_response {
                logputs(log_options::LOG_ALWAYS, b"\n\0" as *const u8 as *const i8);
            }
        }
        fd_close(dtsock);
        err = ftp_response(csock, &mut respline);
        if err as u32 != uerr_t::FTPOK as i32 as u32 {
            if res != -(1 as i32) {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"%s (%s) - \0" as *const u8 as *const i8,
                    tms,
                    tmrate,
                );
            }
            logputs(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Control connection closed.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            fd_close(csock);
            (*con).csock = -(1 as i32);
            return uerr_t::FTPRETRINT;
        }
        *last_expected_bytes = ftp_expected_bytes(respline);
        if *respline as i32 != '2' as i32 {
            if res != -(1 as i32) {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"%s (%s) - \0" as *const u8 as *const i8,
                    tms,
                    tmrate,
                );
            }
            logputs(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Data transfer aborted.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if c_strncasecmp(
                respline,
                b"425\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            ) == 0 && (*u).scheme as u32 == url_scheme::SCHEME_FTPS as i32 as u32
            {
                logputs(
                    log_options::LOG_NOTQUIET,
                    b"FTPS server rejects new SSL sessions in the data connection.\n\0"
                        as *const u8 as *const i8,
                );
                rpl_free(respline as *mut libc::c_void);
                respline = 0 as *mut i8;
                return uerr_t::FTPRESTFAIL;
            }
            rpl_free(respline as *mut libc::c_void);
            respline = 0 as *mut i8;
            return uerr_t::FTPRETRINT;
        }
        rpl_free(respline as *mut libc::c_void);
        respline = 0 as *mut i8;
        if res == -(1 as i32) {
            return uerr_t::FTPRETRINT;
        }
        if cmd & wget_ftp_command::LEAVE_PENDING as i32 == 0 {
            fd_close(csock);
            (*con).csock = -(1 as i32);
        }
        if (*con).cmd & wget_ftp_command::DO_LIST as i32 != 0 {
            if opt.server_response {
                mkalldirs((*con).target);
                fp = rpl_fopen((*con).target, b"r\0" as *const u8 as *const i8);
                if fp.is_null() {
                    logprintf(
                        log_options::LOG_ALWAYS,
                        b"%s: %s\n\0" as *const u8 as *const i8,
                        (*con).target,
                        strerror(*__errno_location()),
                    );
                } else {
                    let mut line: *mut i8 = 0 as *mut i8;
                    let mut bufsize: size_t = 0 as i32 as size_t;
                    let mut len_0: ssize_t = 0;
                    loop {
                        len_0 = getline(&mut line, &mut bufsize, fp);
                        if !(len_0 > 0 as i32 as i64) {
                            break;
                        }
                        while len_0 > 0 as i32 as i64
                            && (*line.offset((len_0 - 1 as i32 as i64) as isize) as i32
                                == '\n' as i32
                                || *line.offset((len_0 - 1 as i32 as i64) as isize) as i32
                                    == '\r' as i32)
                        {
                            len_0 -= 1;
                            *line.offset(len_0 as isize) = '\0' as i32 as i8;
                        }
                        logprintf(
                            log_options::LOG_ALWAYS,
                            b"%s\n\0" as *const u8 as *const i8,
                            quotearg_style(quoting_style::escape_quoting_style, line),
                        );
                    }
                    rpl_free(line as *mut libc::c_void);
                    line = 0 as *mut i8;
                    fclose(fp);
                }
            }
            if (*con).st & wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32 == 0 {
                if (*con).st & wget_ftp_fstatus::AVOID_LIST_A as i32 != 0 {
                    if rd_size > previous_rd_size {
                        (*con).st
                            |= wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32;
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"LIST returned more data than \"LIST -a\": I will use \"LIST\" as standard list command\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                    } else if previous_rd_size > rd_size {
                        (*con).st
                            |= wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32;
                        (*con).st |= wget_ftp_fstatus::AVOID_LIST as i32;
                        (*con).st &= !(wget_ftp_fstatus::AVOID_LIST_A as i32);
                        try_again = 1 as i32 != 0;
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"LIST returned less data than \"LIST -a\": I will use \"LIST -a\" as standard list command\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                    } else if rd_size == 0 as i32 as i64 {
                        (*con).st &= !(wget_ftp_fstatus::AVOID_LIST_A as i32);
                    } else {
                        (*con).st
                            |= wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32;
                        (*con).st |= wget_ftp_fstatus::AVOID_LIST as i32;
                        (*con).st &= !(wget_ftp_fstatus::AVOID_LIST_A as i32);
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"LIST returned the same amount of data of \"LIST -a\": I will use \"LIST -a\" as standard list command\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                    }
                } else if list_a_used {
                    previous_rd_size = rd_size;
                    try_again = 1 as i32 != 0;
                    (*con).st |= wget_ftp_fstatus::AVOID_LIST_A as i32;
                } else {
                    (*con).st |= wget_ftp_fstatus::LIST_AFTER_LIST_A_CHECK_DONE as i32;
                    (*con).st |= wget_ftp_fstatus::AVOID_LIST_A as i32;
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"\"LIST -a\" failed: I will use \"LIST\" as standard list command\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                }
            }
        }
        if !try_again {
            current_block = 9812171323773397338;
            break;
        }
    }
    match current_block {
        9812171323773397338 => return uerr_t::RETRFINISHED,
        _ => {
            if !fp.is_null()
                && (output_stream.is_null()
                    || (*con).cmd & wget_ftp_command::DO_LIST as i32 != 0)
            {
                fclose(fp);
            }
            return err;
        }
    };
}
unsafe extern "C" fn ftp_loop_internal(
    mut u: *mut url,
    mut original_url: *mut url,
    mut f: *mut fileinfo,
    mut con: *mut ccon,
    mut local_file: *mut *mut i8,
    mut force_full_retrieve: bool,
) -> uerr_t {
    let mut count: i32 = 0;
    let mut orig_lp: i32 = 0;
    let mut restval: wgint = 0;
    let mut len: wgint = 0 as i32 as wgint;
    let mut qtyread: wgint = 0 as i32 as wgint;
    let mut tms: *mut i8 = 0 as *mut i8;
    let mut locf: *mut i8 = 0 as *mut i8;
    let mut tmrate: *const i8 = 0 as *const i8;
    let mut err: uerr_t = uerr_t::NOCONERROR;
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
    let mut warc_enabled: bool = !(opt.warc_filename).is_null();
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut warc_ip_buf: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_7 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    let mut warc_ip: *mut ip_address = 0 as *mut ip_address;
    let mut last_expected_bytes: wgint = 0 as i32 as wgint;
    if f.is_null() && !((*con).target).is_null() {
        locf = (*con).target;
    } else {
        rpl_free((*con).target as *mut libc::c_void);
        (*con).target = 0 as *mut i8;
        (*con).target = url_file_name(
            if opt.trustservernames as i32 != 0 || original_url.is_null() {
                u
            } else {
                original_url
            },
            0 as *mut i8,
        );
        if (opt.output_document).is_null() {
            locf = (*con).target;
        } else {
            locf = opt.output_document;
        }
    }
    if opt.noclobber as i32 != 0 && (opt.output_document).is_null()
        && file_exists_p((*con).target, 0 as *mut file_stats_t) as i32 != 0
        && !((*con).cmd & wget_ftp_command::DO_LIST as i32 != 0
            && (*con).cmd & wget_ftp_command::DO_RETR as i32 == 0)
    {
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"File %s already there; not retrieving.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote((*con).target),
        );
        return uerr_t::RETROK;
    }
    remove_link((*con).target);
    count = 0 as i32;
    if (*con).st & wget_ftp_fstatus::ON_YOUR_OWN as i32 != 0 {
        (*con).st = wget_ftp_fstatus::ON_YOUR_OWN as i32;
    }
    orig_lp = if (*con).cmd & wget_ftp_command::LEAVE_PENDING as i32 != 0 {
        1 as i32
    } else {
        0 as i32
    };
    let mut current_block_148: u64;
    loop {
        count += 1;
        count;
        sleep_between_retrievals(count);
        if (*con).st & wget_ftp_fstatus::ON_YOUR_OWN as i32 != 0 {
            (*con).cmd = 0 as i32;
            (*con).cmd
                |= wget_ftp_command::DO_RETR as i32
                    | wget_ftp_command::LEAVE_PENDING as i32;
            if (*con).csock != -(1 as i32) {
                (*con).cmd
                    &= !(wget_ftp_command::DO_LOGIN as i32
                        | wget_ftp_command::DO_CWD as i32);
            } else {
                (*con).cmd
                    |= wget_ftp_command::DO_LOGIN as i32
                        | wget_ftp_command::DO_CWD as i32;
            }
        } else {
            if (*con).csock != -(1 as i32) {
                (*con).cmd &= !(wget_ftp_command::DO_LOGIN as i32);
            } else {
                (*con).cmd |= wget_ftp_command::DO_LOGIN as i32;
            }
            if (*con).st & wget_ftp_fstatus::DONE_CWD as i32 != 0 {
                (*con).cmd &= !(wget_ftp_command::DO_CWD as i32);
            } else {
                (*con).cmd |= wget_ftp_command::DO_CWD as i32;
            }
        }
        if warc_enabled as i32 != 0 && (*con).cmd & wget_ftp_command::DO_RETR as i32 != 0
            && warc_tmp.is_null()
        {
            warc_tmp = warc_tempfile();
            if warc_tmp.is_null() {
                return uerr_t::WARC_TMP_FOPENERR;
            }
            if ((*con).proxy).is_null() && (*con).csock != -(1 as i32) {
                warc_ip = &mut warc_ip_buf;
                socket_ip_address(
                    (*con).csock,
                    warc_ip,
                    C2RustUnnamed_10::ENDPOINT_PEER as i32,
                );
            }
        }
        if (*con).cmd & wget_ftp_command::DO_LIST as i32 != 0 {
            restval = 0 as i32 as wgint;
        } else if force_full_retrieve {
            restval = 0 as i32 as wgint;
        } else if opt.start_pos >= 0 as i32 as i64 {
            restval = opt.start_pos;
        } else if opt.always_rest as i32 != 0 && stat(locf, &mut st) == 0 as i32
            && st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
        {
            restval = st.st_size;
        } else if count > 1 as i32 {
            restval = qtyread;
        } else {
            restval = 0 as i32 as wgint;
        }
        tms = datetime_str(time(0 as *mut time_t));
        if opt.verbose != 0 {
            let mut hurl: *mut i8 = url_string(u, url_auth_mode::URL_AUTH_HIDE_PASSWD);
            let mut tmp: [i8; 256] = [0; 256];
            strcpy(tmp.as_mut_ptr(), b"        \0" as *const u8 as *const i8);
            if count > 1 as i32 {
                sprintf(
                    tmp.as_mut_ptr(),
                    dcgettext(
                        0 as *const i8,
                        b"(try:%2d)\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    count,
                );
            }
            logprintf(
                log_options::LOG_VERBOSE,
                b"--%s--  %s\n  %s => %s\n\0" as *const u8 as *const i8,
                tms,
                hurl,
                tmp.as_mut_ptr(),
                quote(locf),
            );
            rpl_free(hurl as *mut libc::c_void);
            hurl = 0 as *mut i8;
        }
        if !f.is_null() && (*f).type_0 as u32 != ftype::FT_SYMLINK as i32 as u32 {
            len = (*f).size;
        } else {
            len = 0 as i32 as wgint;
        }
        err = getftp(
            u,
            original_url,
            len,
            &mut qtyread,
            restval,
            con,
            count,
            &mut last_expected_bytes,
            warc_tmp,
        );
        if (*con).csock == -(1 as i32) {
            (*con).st &= !(wget_ftp_fstatus::DONE_CWD as i32);
        } else {
            (*con).st |= wget_ftp_fstatus::DONE_CWD as i32;
        }
        match err as u32 {
            1 | 5 | 21 | 19 | 12 | 8 | 29 | 32 | 30 | 31 | 47 | 54 | 4 | 33 | 46 => {
                if err as u32 == uerr_t::FTPNOAUTH as i32 as u32 {
                    logputs(
                        log_options::LOG_NOTQUIET,
                        b"Server does not support AUTH TLS.\n\0" as *const u8
                            as *const i8,
                    );
                }
                if opt.ftps_implicit {
                    logputs(
                        log_options::LOG_NOTQUIET,
                        b"Server does not like implicit FTPS connections.\n\0"
                            as *const u8 as *const i8,
                    );
                }
                if !warc_tmp.is_null() {
                    fclose(warc_tmp);
                    warc_tmp = 0 as *mut FILE;
                }
                return err;
            }
            2 | 3 | 15 | 14 | 44 | 13 | 11 | 10 | 9 | 28 | 20 => {
                printwhat(count, opt.ntry);
                if err as u32 == uerr_t::FOPEN_EXCL_ERR as i32 as u32 {
                    rpl_free((*con).target as *mut libc::c_void);
                    (*con).target = 0 as *mut i8;
                    (*con).target = url_file_name(u, 0 as *mut i8);
                    locf = (*con).target;
                }
                current_block_148 = 2838571290723028321;
            }
            16 => {
                if f.is_null() || qtyread != (*f).size {
                    printwhat(count, opt.ntry);
                    current_block_148 = 2838571290723028321;
                } else {
                    current_block_148 = 7301440000599063274;
                }
            }
            35 => {
                current_block_148 = 7301440000599063274;
            }
            _ => {
                abort();
            }
        }
        match current_block_148 {
            2838571290723028321 => {
                if !(opt.ntry == 0 || count < opt.ntry) {
                    break;
                }
            }
            _ => {
                tms = datetime_str(time(0 as *mut time_t));
                if !opt.spider {
                    tmrate = retr_rate(qtyread - restval, (*con).dltime);
                }
                downloaded_file(downloaded_file_t::FILE_DOWNLOADED_NORMALLY, locf);
                if (*con).st & wget_ftp_fstatus::ON_YOUR_OWN as i32 != 0 {
                    fd_close((*con).csock);
                    (*con).csock = -(1 as i32);
                }
                if !opt.spider {
                    let mut write_to_stdout: bool = !(opt.output_document).is_null()
                        && (*opt.output_document as i32 == '-' as i32
                            && *(opt.output_document).offset(1 as i32 as isize) == 0);
                    logprintf(
                        log_options::LOG_VERBOSE,
                        if write_to_stdout as i32 != 0 {
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
                        if write_to_stdout as i32 != 0 {
                            b"\0" as *const u8 as *const i8
                        } else {
                            quote(locf)
                        },
                        number_to_static_string(qtyread),
                    );
                }
                if opt.verbose == 0 && !opt.quiet {
                    let mut hurl_0: *mut i8 = url_string(
                        u,
                        url_auth_mode::URL_AUTH_HIDE_PASSWD,
                    );
                    logprintf(
                        log_options::LOG_NONVERBOSE,
                        b"%s URL: %s [%s] -> \"%s\" [%d]\n\0" as *const u8 as *const i8,
                        tms,
                        hurl_0,
                        number_to_static_string(qtyread),
                        locf,
                        count,
                    );
                    rpl_free(hurl_0 as *mut libc::c_void);
                    hurl_0 = 0 as *mut i8;
                }
                if warc_enabled as i32 != 0
                    && (*con).cmd & wget_ftp_command::DO_RETR as i32 != 0
                {
                    let mut warc_res: bool = false;
                    warc_res = warc_write_resource_record(
                        0 as *const i8,
                        (*u).url,
                        0 as *const i8,
                        0 as *const i8,
                        warc_ip,
                        0 as *const i8,
                        warc_tmp,
                        -(1 as i32) as off_t,
                    );
                    if !warc_res {
                        return uerr_t::WARC_ERR;
                    }
                    warc_tmp = 0 as *mut FILE;
                }
                if (*con).cmd & wget_ftp_command::DO_LIST as i32 != 0 {
                    if !opt.remove_listing {
                        total_downloaded_bytes += qtyread - restval;
                        numurls += 1;
                        numurls;
                    }
                } else if !opt.spider {
                    total_downloaded_bytes += qtyread - restval;
                    numurls += 1;
                    numurls;
                    if opt.delete_after as i32 != 0
                        && !input_file_url(opt.input_filename)
                    {
                        if opt.debug as i64 != 0 {
                            debug_logprintf(
                                b"Removing file due to --delete-after in ftp_loop_internal():\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        logprintf(
                            log_options::LOG_VERBOSE,
                            dcgettext(
                                0 as *const i8,
                                b"Removing %s.\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            locf,
                        );
                        if unlink(locf) != 0 {
                            logprintf(
                                log_options::LOG_NOTQUIET,
                                b"unlink: %s\n\0" as *const u8 as *const i8,
                                strerror(*__errno_location()),
                            );
                        }
                    }
                }
                if orig_lp != 0 {
                    (*con).cmd |= wget_ftp_command::LEAVE_PENDING as i32;
                } else {
                    (*con).cmd &= !(wget_ftp_command::LEAVE_PENDING as i32);
                }
                if !local_file.is_null() {
                    *local_file = xstrdup(locf);
                }
                if !warc_tmp.is_null() {
                    fclose(warc_tmp);
                    warc_tmp = 0 as *mut FILE;
                }
                return uerr_t::RETROK;
            }
        }
    }
    if (*con).csock != -(1 as i32)
        && (*con).st & wget_ftp_fstatus::ON_YOUR_OWN as i32 != 0
    {
        fd_close((*con).csock);
        (*con).csock = -(1 as i32);
    }
    if !warc_tmp.is_null() {
        fclose(warc_tmp);
    }
    return uerr_t::TRYLIMEXC;
}
unsafe extern "C" fn ftp_get_listing(
    mut u: *mut url,
    mut original_url: *mut url,
    mut con: *mut ccon,
    mut f: *mut *mut fileinfo,
) -> uerr_t {
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut uf: *mut i8 = 0 as *mut i8;
    let mut lf: *mut i8 = 0 as *mut i8;
    let mut old_target: *mut i8 = (*con).target;
    (*con).st &= !(wget_ftp_fstatus::ON_YOUR_OWN as i32);
    (*con).cmd
        |= wget_ftp_command::DO_LIST as i32 | wget_ftp_command::LEAVE_PENDING as i32;
    (*con).cmd &= !(wget_ftp_command::DO_RETR as i32);
    uf = url_file_name(u, 0 as *mut i8);
    lf = file_merge(uf, b".listing\0" as *const u8 as *const i8);
    rpl_free(uf as *mut libc::c_void);
    uf = 0 as *mut i8;
    if opt.debug as i64 != 0 {
        debug_logprintf(
            dcgettext(
                0 as *const i8,
                b"Using %s as listing tmp file.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(lf),
        );
    }
    (*con).target = xstrdup(lf);
    rpl_free(lf as *mut libc::c_void);
    lf = 0 as *mut i8;
    err = ftp_loop_internal(
        u,
        original_url,
        0 as *mut fileinfo,
        con,
        0 as *mut *mut i8,
        0 as i32 != 0,
    );
    lf = xstrdup((*con).target);
    rpl_free((*con).target as *mut libc::c_void);
    (*con).target = 0 as *mut i8;
    (*con).target = old_target;
    if err as u32 == uerr_t::RETROK as i32 as u32 {
        *f = ftp_parse_ls(lf, (*con).rs);
        if opt.remove_listing {
            if unlink(lf) != 0 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"unlink: %s\n\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
            } else {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Removed %s.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quote(lf),
                );
            }
        }
    } else {
        *f = 0 as *mut fileinfo;
    }
    rpl_free(lf as *mut libc::c_void);
    lf = 0 as *mut i8;
    (*con).cmd &= !(wget_ftp_command::DO_LIST as i32);
    return err;
}
unsafe extern "C" fn ftp_retrieve_list(
    mut u: *mut url,
    mut original_url: *mut url,
    mut f: *mut fileinfo,
    mut con: *mut ccon,
) -> uerr_t {
    static mut depth: i32 = 0 as i32;
    let mut err: uerr_t = uerr_t::NOCONERROR;
    let mut orig: *mut fileinfo = 0 as *mut fileinfo;
    let mut local_size: wgint = 0;
    let mut tml: time_t = 0;
    let mut dlthis: bool = false;
    let mut actual_target: *const i8 = 0 as *const i8;
    let mut force_full_retrieve: bool = 0 as i32 != 0;
    depth += 1;
    depth;
    if opt.reclevel != -(1 as i32) && depth > opt.reclevel {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                dcgettext(
                    0 as *const i8,
                    b"Recursion depth %d exceeded max. depth %d.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                depth,
                opt.reclevel,
            );
        }
        depth -= 1;
        depth;
        return uerr_t::RECLEVELEXC;
    }
    orig = f;
    (*con).st &= !(wget_ftp_fstatus::ON_YOUR_OWN as i32);
    if (*con).st & wget_ftp_fstatus::DONE_CWD as i32 == 0 {
        (*con).cmd |= wget_ftp_command::DO_CWD as i32;
    } else {
        (*con).cmd &= !(wget_ftp_command::DO_CWD as i32);
    }
    (*con).cmd
        |= wget_ftp_command::DO_RETR as i32 | wget_ftp_command::LEAVE_PENDING as i32;
    if (*con).csock < 0 as i32 {
        (*con).cmd |= wget_ftp_command::DO_LOGIN as i32;
    } else {
        (*con).cmd &= !(wget_ftp_command::DO_LOGIN as i32);
    }
    err = uerr_t::RETROK;
    while !f.is_null() {
        let mut old_target: *mut i8 = 0 as *mut i8;
        let mut ofile: *mut i8 = 0 as *mut i8;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            depth -= 1;
            depth;
            return uerr_t::QUOTEXC;
        }
        old_target = (*con).target;
        ofile = xstrdup((*u).file);
        url_set_file(u, (*f).name);
        (*con).target = url_file_name(u, 0 as *mut i8);
        err = uerr_t::RETROK;
        dlthis = 1 as i32 != 0;
        if opt.timestamping as i32 != 0
            && (*f).type_0 as u32 == ftype::FT_PLAINFILE as i32 as u32
        {
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
            if stat((*con).target, &mut st) == 0 {
                let mut eq_size: bool = false;
                let mut cor_val: bool = false;
                local_size = st.st_size;
                tml = st.st_mtim.tv_sec;
                cor_val = (*con).rs as u32 == stype::ST_UNIX as i32 as u32
                    || (*con).rs as u32 == stype::ST_WINNT as i32 as u32;
                eq_size = if cor_val as i32 != 0 {
                    (local_size == (*f).size) as i32
                } else {
                    1 as i32
                } != 0;
                if (*f).tstamp <= tml && eq_size as i32 != 0 {
                    logprintf(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"Remote file no newer than local file %s -- not retrieving.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote((*con).target),
                    );
                    dlthis = 0 as i32 != 0;
                } else if (*f).tstamp > tml {
                    force_full_retrieve = 1 as i32 != 0;
                    logprintf(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"Remote file is newer than local file %s -- retrieving.\n\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote((*con).target),
                    );
                } else {
                    logprintf(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"The sizes do not match (local %s) -- retrieving.\n\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        number_to_static_string(local_size),
                    );
                }
            }
        }
        let mut current_block_76: u64;
        match (*f).type_0 as u32 {
            2 => {
                if !opt.retr_symlinks {
                    if ((*f).linkto).is_null() {
                        logputs(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Invalid name of the symlink, skipping.\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                    } else {
                        let mut st_0: stat = stat {
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
                        let mut rc: i32 = lstat((*con).target, &mut st_0);
                        if rc == 0 as i32 {
                            let mut len: size_t = (strlen((*f).linkto))
                                .wrapping_add(1 as i32 as u64);
                            if st_0.st_mode & 0o170000 as i32 as u32
                                == 0o120000 as i32 as u32
                            {
                                let mut buf: [i8; 1024] = [0; 1024];
                                let mut link_target: *mut i8 = 0 as *mut i8;
                                let mut n: size_t = 0;
                                let mut res: bool = false;
                                if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
                                    link_target = buf.as_mut_ptr();
                                } else {
                                    link_target = xmalloc(len) as *mut i8;
                                }
                                n = readlink((*con).target, link_target, len) as size_t;
                                res = n == len.wrapping_sub(1 as i32 as u64)
                                    && memcmp(
                                        link_target as *const libc::c_void,
                                        (*f).linkto as *const libc::c_void,
                                        n,
                                    ) == 0 as i32;
                                if link_target != buf.as_mut_ptr() {
                                    rpl_free(link_target as *mut libc::c_void);
                                    link_target = 0 as *mut i8;
                                }
                                if res {
                                    logprintf(
                                        log_options::LOG_VERBOSE,
                                        dcgettext(
                                            0 as *const i8,
                                            b"Already have correct symlink %s -> %s\n\n\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                        quote_n(0 as i32, (*con).target),
                                        quote_n(1 as i32, (*f).linkto),
                                    );
                                    dlthis = 0 as i32 != 0;
                                    current_block_76 = 919954187481050311;
                                } else {
                                    current_block_76 = 2116367355679836638;
                                }
                            } else {
                                current_block_76 = 2116367355679836638;
                            }
                        } else {
                            current_block_76 = 2116367355679836638;
                        }
                        match current_block_76 {
                            919954187481050311 => {}
                            _ => {
                                logprintf(
                                    log_options::LOG_VERBOSE,
                                    dcgettext(
                                        0 as *const i8,
                                        b"Creating symlink %s -> %s\n\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    quote_n(0 as i32, (*con).target),
                                    quote_n(1 as i32, (*f).linkto),
                                );
                                unlink((*con).target);
                                if symlink((*f).linkto, (*con).target) == -(1 as i32) {
                                    logprintf(
                                        log_options::LOG_NOTQUIET,
                                        b"symlink: %s\n\0" as *const u8 as *const i8,
                                        strerror(*__errno_location()),
                                    );
                                }
                                logputs(
                                    log_options::LOG_VERBOSE,
                                    b"\n\0" as *const u8 as *const i8,
                                );
                            }
                        }
                    }
                } else if dlthis {
                    err = ftp_loop_internal(
                        u,
                        original_url,
                        f,
                        con,
                        0 as *mut *mut i8,
                        force_full_retrieve,
                    );
                }
            }
            1 => {
                if !opt.recursive {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Skipping directory %s.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote((*f).name),
                    );
                }
            }
            0 => {
                if dlthis {
                    err = ftp_loop_internal(
                        u,
                        original_url,
                        f,
                        con,
                        0 as *mut *mut i8,
                        force_full_retrieve,
                    );
                }
            }
            3 | _ => {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"%s: unknown/unsupported file type.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    quote((*f).name),
                );
            }
        }
        set_local_file(&mut actual_target, (*con).target);
        if dlthis as i32 != 0 && !actual_target.is_null()
            && (*f).type_0 as u32 == ftype::FT_PLAINFILE as i32 as u32
            && opt.preserve_perm as i32 != 0
        {
            if (*f).perms != 0 {
                if chmod(actual_target, (*f).perms as __mode_t) != 0 {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"Failed to set permissions for %s.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        actual_target,
                    );
                }
            } else if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Unrecognized permissions for %s.\n\0" as *const u8 as *const i8,
                    actual_target,
                );
            }
        }
        if !actual_target.is_null() {
            if opt.useservertimestamps as i32 != 0
                && !((*f).type_0 as u32 == ftype::FT_SYMLINK as i32 as u32
                    && !opt.retr_symlinks) && (*f).tstamp != -(1 as i32) as i64
                && dlthis as i32 != 0
                && file_exists_p((*con).target, 0 as *mut file_stats_t) as i32 != 0
            {
                touch(actual_target, (*f).tstamp);
            } else if (*f).tstamp == -(1 as i32) as i64 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"%s: corrupt time-stamp.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    actual_target,
                );
            }
        }
        rpl_free((*con).target as *mut libc::c_void);
        (*con).target = 0 as *mut i8;
        (*con).target = old_target;
        url_set_file(u, ofile);
        rpl_free(ofile as *mut libc::c_void);
        ofile = 0 as *mut i8;
        if err as u32 == uerr_t::QUOTEXC as i32 as u32
            || err as u32 == uerr_t::HOSTERR as i32 as u32
            || err as u32 == uerr_t::FWRITEERR as i32 as u32
            || err as u32 == uerr_t::WARC_ERR as i32 as u32
            || err as u32 == uerr_t::WARC_TMP_FOPENERR as i32 as u32
            || err as u32 == uerr_t::WARC_TMP_FWRITEERR as i32 as u32
        {
            break;
        }
        (*con).cmd
            &= !(wget_ftp_command::DO_CWD as i32 | wget_ftp_command::DO_LOGIN as i32);
        f = (*f).next;
    }
    if opt.recursive as i32 != 0
        && !(opt.reclevel != -(1 as i32) && depth >= opt.reclevel)
    {
        err = ftp_retrieve_dirs(u, original_url, orig, con);
    } else if opt.recursive {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                dcgettext(
                    0 as *const i8,
                    b"Will not retrieve dirs since depth is %d (max %d).\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                depth,
                opt.reclevel,
            );
        }
    }
    depth -= 1;
    depth;
    return err;
}
unsafe extern "C" fn ftp_retrieve_dirs(
    mut u: *mut url,
    mut original_url: *mut url,
    mut f: *mut fileinfo,
    mut con: *mut ccon,
) -> uerr_t {
    let mut buf: [i8; 1024] = [0; 1024];
    let mut container: *mut i8 = buf.as_mut_ptr();
    let mut container_size: i32 = ::core::mem::size_of::<[i8; 1024]>() as u64 as i32;
    while !f.is_null() {
        let mut size: i32 = 0;
        let mut odir: *mut i8 = 0 as *mut i8;
        let mut newdir: *mut i8 = 0 as *mut i8;
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            break;
        }
        if !((*f).type_0 as u32 != ftype::FT_DIRECTORY as i32 as u32) {
            size = (strlen((*u).dir))
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(strlen((*f).name))
                .wrapping_add(1 as i32 as u64) as i32;
            if size > container_size {
                if container == buf.as_mut_ptr() {
                    container = xmalloc(size as size_t) as *mut i8;
                } else {
                    container = xrealloc(container as *mut libc::c_void, size as size_t)
                        as *mut i8;
                }
                container_size = size;
            }
            newdir = container;
            odir = (*u).dir;
            if *odir as i32 == '\0' as i32
                || *odir as i32 == '/' as i32
                    && *odir.offset(1 as i32 as isize) as i32 == '\0' as i32
            {
                sprintf(newdir, b"%s%s\0" as *const u8 as *const i8, odir, (*f).name);
            } else {
                sprintf(newdir, b"%s/%s\0" as *const u8 as *const i8, odir, (*f).name);
            }
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"Composing new CWD relative to the initial directory.\n\0"
                        as *const u8 as *const i8,
                );
            }
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"  odir = '%s'\n  f->name = '%s'\n  newdir = '%s'\n\n\0"
                        as *const u8 as *const i8,
                    odir,
                    (*f).name,
                    newdir,
                );
            }
            if !accdir(newdir) {
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Not descending to %s as it is excluded/not-included.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quote(newdir),
                );
            } else {
                (*con).st &= !(wget_ftp_fstatus::DONE_CWD as i32);
                odir = xstrdup((*u).dir);
                url_set_dir(u, newdir);
                ftp_retrieve_glob(
                    u,
                    original_url,
                    con,
                    C2RustUnnamed_8::GLOB_GETALL as i32,
                );
                url_set_dir(u, odir);
                rpl_free(odir as *mut libc::c_void);
                odir = 0 as *mut i8;
            }
        }
        f = (*f).next;
    }
    if container != buf.as_mut_ptr() {
        rpl_free(container as *mut libc::c_void);
        container = 0 as *mut i8;
    }
    if opt.quota != 0 && total_downloaded_bytes > opt.quota {
        return uerr_t::QUOTEXC
    } else {
        return uerr_t::RETROK
    };
}
unsafe extern "C" fn has_insecure_name_p(mut s: *const i8) -> bool {
    if *s as i32 == '/' as i32 {
        return 1 as i32 != 0;
    }
    if !(strstr(s, b"../\0" as *const u8 as *const i8)).is_null() {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn is_invalid_entry(mut f: *mut fileinfo) -> bool {
    let mut cur: *mut fileinfo = f;
    let mut f_name: *mut i8 = (*f).name;
    while !((*cur).next).is_null() {
        cur = (*cur).next;
        if strcmp(f_name, (*cur).name) == 0 as i32 {
            return 1 as i32 != 0;
        }
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn ftp_retrieve_glob(
    mut u: *mut url,
    mut original_url: *mut url,
    mut con: *mut ccon,
    mut action: i32,
) -> uerr_t {
    let mut f: *mut fileinfo = 0 as *mut fileinfo;
    let mut start: *mut fileinfo = 0 as *mut fileinfo;
    let mut res: uerr_t = uerr_t::NOCONERROR;
    (*con).cmd |= wget_ftp_command::LEAVE_PENDING as i32;
    res = ftp_get_listing(u, original_url, con, &mut start);
    if res as u32 != uerr_t::RETROK as i32 as u32 {
        return res;
    }
    let mut matcher: Option<unsafe extern "C" fn(*const i8, *const i8, i32) -> i32> = if opt
        .ignore_case as i32 != 0
    {
        Some(fnmatch_nocase as unsafe extern "C" fn(*const i8, *const i8, i32) -> i32)
    } else {
        Some(fnmatch as unsafe extern "C" fn(*const i8, *const i8, i32) -> i32)
    };
    let mut cmp: Option<unsafe extern "C" fn(*const i8, *const i8) -> i32> = if opt
        .ignore_case as i32 != 0
    {
        Some(strcasecmp as unsafe extern "C" fn(*const i8, *const i8) -> i32)
    } else {
        Some(strcmp as unsafe extern "C" fn(*const i8, *const i8) -> i32)
    };
    f = start;
    while !f.is_null() {
        if (!(opt.accepts).is_null() || !(opt.rejects).is_null())
            && (*f).type_0 as u32 != ftype::FT_DIRECTORY as i32 as u32
            && !acceptable((*f).name)
        {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Rejecting %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote((*f).name),
            );
            f = delelement(&mut f, &mut start);
        } else if has_insecure_name_p((*f).name) as i32 != 0
            || is_invalid_entry(f) as i32 != 0
        {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Rejecting %s (Invalid Entry).\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote((*f).name),
            );
            f = delelement(&mut f, &mut start);
        } else {
            if !(opt.acceptregex).is_null() || !(opt.rejectregex).is_null() {
                let mut buf: [i8; 1024] = [0; 1024];
                let mut url: *mut i8 = buf.as_mut_ptr();
                if snprintf(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 1024]>() as u64,
                    b"%s%s%s\0" as *const u8 as *const i8,
                    (*u).url,
                    (*f).name,
                    (if (*f).type_0 as u32 == ftype::FT_DIRECTORY as i32 as u32 {
                        b"/\0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    }),
                ) as u32 as u64 >= ::core::mem::size_of::<[i8; 1024]>() as u64
                {
                    url = aprintf(
                        b"%s%s%s\0" as *const u8 as *const i8,
                        (*u).url,
                        (*f).name,
                        if (*f).type_0 as u32 == ftype::FT_DIRECTORY as i32 as u32 {
                            b"/\0" as *const u8 as *const i8
                        } else {
                            b"\0" as *const u8 as *const i8
                        },
                    );
                }
                if !accept_url(url) {
                    logprintf(
                        log_options::LOG_VERBOSE,
                        dcgettext(
                            0 as *const i8,
                            b"%s is excluded/not-included through regex.\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        url,
                    );
                    f = delelement(&mut f, &mut start);
                    if url != buf.as_mut_ptr() {
                        rpl_free(url as *mut libc::c_void);
                        url = 0 as *mut i8;
                    }
                    continue;
                } else if url != buf.as_mut_ptr() {
                    rpl_free(url as *mut libc::c_void);
                    url = 0 as *mut i8;
                }
            }
            if *(*u).file != 0 {
                if action == C2RustUnnamed_8::GLOB_GLOBALL as i32 {
                    let mut matchres: i32 = matcher
                        .expect(
                            "non-null function pointer",
                        )((*u).file, (*f).name, 0 as i32);
                    if matchres == -(1 as i32) {
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Error matching %s against %s: %s\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            (*u).file,
                            quotearg_style(
                                quoting_style::escape_quoting_style,
                                (*f).name,
                            ),
                            strerror(*__errno_location()),
                        );
                        freefileinfo(start);
                        return uerr_t::RETRBADPATTERN;
                    }
                    if matchres == 1 as i32 {
                        f = delelement(&mut f, &mut start);
                        continue;
                    }
                } else if action == C2RustUnnamed_8::GLOB_GETONE as i32 {
                    if 0 as i32
                        != cmp.expect("non-null function pointer")((*u).file, (*f).name)
                    {
                        f = delelement(&mut f, &mut start);
                        continue;
                    }
                }
            }
            f = (*f).next;
        }
    }
    if !start.is_null() {
        res = ftp_retrieve_list(u, original_url, start, con);
    } else if action == C2RustUnnamed_8::GLOB_GLOBALL as i32 {
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"No matches on pattern %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote((*u).file),
        );
    } else if action == C2RustUnnamed_8::GLOB_GETONE as i32 {
        (*con).st |= wget_ftp_fstatus::ON_YOUR_OWN as i32;
        res = ftp_loop_internal(
            u,
            original_url,
            0 as *mut fileinfo,
            con,
            0 as *mut *mut i8,
            0 as i32 != 0,
        );
        return res;
    }
    freefileinfo(start);
    if opt.quota != 0 && total_downloaded_bytes > opt.quota {
        return uerr_t::QUOTEXC
    } else {
        return res
    };
}
#[no_mangle]
pub unsafe extern "C" fn ftp_loop(
    mut u: *mut url,
    mut original_url: *mut url,
    mut local_file: *mut *mut i8,
    mut dt: *mut i32,
    mut proxy: *mut url,
    mut recursive: bool,
    mut glob: bool,
) -> uerr_t {
    let mut con: ccon = ccon {
        st: 0,
        cmd: 0,
        csock: 0,
        dltime: 0.,
        rs: stype::ST_UNIX,
        rsu: ustype::UST_TYPE_L8,
        id: 0 as *mut i8,
        target: 0 as *mut i8,
        proxy: 0 as *mut url,
    };
    let mut res: uerr_t = uerr_t::NOCONERROR;
    *dt = 0 as i32;
    memset(
        &mut con as *mut ccon as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<ccon>() as u64,
    );
    con.csock = -(1 as i32);
    con.st = wget_ftp_fstatus::ON_YOUR_OWN as i32;
    con.rs = stype::ST_UNIX;
    con.id = 0 as *mut i8;
    con.proxy = proxy;
    if *(*u).file == 0 && !recursive {
        let mut f: *mut fileinfo = 0 as *mut fileinfo;
        res = ftp_get_listing(u, original_url, &mut con, &mut f);
        if res as u32 == uerr_t::RETROK as i32 as u32 {
            if opt.htmlify as i32 != 0 && !opt.spider {
                let mut url_file: *mut url = if opt.trustservernames as i32 != 0 {
                    u
                } else {
                    original_url
                };
                let mut filename: *mut i8 = if !(opt.output_document).is_null() {
                    xstrdup(opt.output_document)
                } else if !(con.target).is_null() {
                    xstrdup(con.target)
                } else {
                    url_file_name(url_file, 0 as *mut i8)
                };
                res = ftp_index(filename, u, f);
                if res as u32 == uerr_t::FTPOK as i32 as u32 && opt.verbose != 0 {
                    if (opt.output_document).is_null() {
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
                        let mut sz: wgint = 0;
                        if stat(filename, &mut st) == 0 as i32 {
                            sz = st.st_size;
                        } else {
                            sz = -(1 as i32) as wgint;
                        }
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Wrote HTML-ized index to %s [%s].\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            quote(filename),
                            number_to_static_string(sz),
                        );
                    } else {
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Wrote HTML-ized index to %s.\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            quote(filename),
                        );
                    }
                }
                rpl_free(filename as *mut libc::c_void);
                filename = 0 as *mut i8;
            }
            freefileinfo(f);
        }
    } else {
        let mut ispattern: bool = 0 as i32 != 0;
        if glob {
            let mut file_part: *mut i8 = strrchr((*u).path, '/' as i32);
            if file_part.is_null() {
                file_part = (*u).path;
            }
            ispattern = has_wildcards_p(file_part);
        }
        if ispattern as i32 != 0 || recursive as i32 != 0 || opt.timestamping as i32 != 0
            || opt.preserve_perm as i32 != 0
        {
            res = ftp_retrieve_glob(
                u,
                original_url,
                &mut con,
                if ispattern as i32 != 0 {
                    C2RustUnnamed_8::GLOB_GLOBALL as i32
                } else {
                    C2RustUnnamed_8::GLOB_GETONE as i32
                },
            );
        } else {
            res = ftp_loop_internal(
                u,
                original_url,
                0 as *mut fileinfo,
                &mut con,
                local_file,
                0 as i32 != 0,
            );
        }
    }
    if res as u32 == uerr_t::FTPOK as i32 as u32 {
        res = uerr_t::RETROK;
    }
    if res as u32 == uerr_t::RETROK as i32 as u32 {
        *dt |= C2RustUnnamed_4::RETROKF as i32;
    }
    if con.csock != -(1 as i32) {
        fd_close(con.csock);
    }
    rpl_free(con.id as *mut libc::c_void);
    con.id = 0 as *mut i8;
    rpl_free(con.target as *mut libc::c_void);
    con.target = 0 as *mut i8;
    return res;
}
unsafe extern "C" fn delelement(
    mut f: *mut *mut fileinfo,
    mut start: *mut *mut fileinfo,
) -> *mut fileinfo {
    let mut prev: *mut fileinfo = (**f).prev;
    let mut next: *mut fileinfo = (**f).next;
    rpl_free((**f).name as *mut libc::c_void);
    (**f).name = 0 as *mut i8;
    rpl_free((**f).linkto as *mut libc::c_void);
    (**f).linkto = 0 as *mut i8;
    rpl_free(*f as *mut libc::c_void);
    *f = 0 as *mut fileinfo;
    if !next.is_null() {
        (*next).prev = prev;
    }
    if !prev.is_null() {
        (*prev).next = next;
    } else {
        *start = next;
    }
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn freefileinfo(mut f: *mut fileinfo) {
    while !f.is_null() {
        let mut next: *mut fileinfo = (*f).next;
        rpl_free((*f).name as *mut libc::c_void);
        (*f).name = 0 as *mut i8;
        rpl_free((*f).linkto as *mut libc::c_void);
        (*f).linkto = 0 as *mut i8;
        rpl_free(f as *mut libc::c_void);
        f = 0 as *mut fileinfo;
        f = next;
    }
}