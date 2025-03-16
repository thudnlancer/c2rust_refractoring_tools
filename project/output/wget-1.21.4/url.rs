#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn idn_encode(i: *const iri, host: *const libc::c_char) -> *mut libc::c_char;
    fn remote_to_utf8(
        i: *const iri,
        str: *const libc::c_char,
        newstr: *mut *mut libc::c_char,
    ) -> bool;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn concat_strings(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn file_exists_p(_: *const libc::c_char, _: *mut file_stats_t) -> bool;
    fn file_non_directory_p(_: *const libc::c_char) -> bool;
    fn make_directory(_: *const libc::c_char) -> libc::c_int;
    fn unique_name_passthrough(_: *const libc::c_char) -> *mut libc::c_char;
    fn numdigit(_: wgint) -> libc::c_int;
    fn number_to_string(_: *mut libc::c_char, _: wgint) -> *mut libc::c_char;
    fn get_max_length(
        path: *const libc::c_char,
        length: libc::c_int,
        name: libc::c_int,
    ) -> libc::c_long;
    fn is_valid_ipv6_address(_: *const libc::c_char, _: *const libc::c_char) -> bool;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn iconv_open(
        __tocode: *const libc::c_char,
        __fromcode: *const libc::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
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
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type wgint = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub verbose: libc::c_int,
    pub quiet: bool,
    pub ntry: libc::c_int,
    pub retry_connrefused: bool,
    pub retry_on_host_error: bool,
    pub retry_on_http_error: *mut libc::c_char,
    pub background: bool,
    pub ignore_length: bool,
    pub recursive: bool,
    pub spanhost: bool,
    pub max_redirect: libc::c_int,
    pub relative_only: bool,
    pub no_parent: bool,
    pub reclevel: libc::c_int,
    pub dirstruct: bool,
    pub no_dirstruct: bool,
    pub cut_dirs: libc::c_int,
    pub add_hostdir: bool,
    pub protocol_directories: bool,
    pub noclobber: bool,
    pub unlink_requested: bool,
    pub dir_prefix: *mut libc::c_char,
    pub lfilename: *mut libc::c_char,
    pub input_filename: *mut libc::c_char,
    pub choose_config: *mut libc::c_char,
    pub noconfig: bool,
    pub force_html: bool,
    pub default_page: *mut libc::c_char,
    pub spider: bool,
    pub accepts: *mut *mut libc::c_char,
    pub rejects: *mut *mut libc::c_char,
    pub excludes: *mut *const libc::c_char,
    pub includes: *mut *const libc::c_char,
    pub ignore_case: bool,
    pub acceptregex_s: *mut libc::c_char,
    pub rejectregex_s: *mut libc::c_char,
    pub acceptregex: *mut libc::c_void,
    pub rejectregex: *mut libc::c_void,
    pub regex_type: C2RustUnnamed_3,
    pub regex_compile_fun: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub regex_match_fun: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_char) -> bool,
    >,
    pub domains: *mut *mut libc::c_char,
    pub exclude_domains: *mut *mut libc::c_char,
    pub dns_cache: bool,
    pub follow_tags: *mut *mut libc::c_char,
    pub ignore_tags: *mut *mut libc::c_char,
    pub follow_ftp: bool,
    pub retr_symlinks: bool,
    pub output_document: *mut libc::c_char,
    pub warc_filename: *mut libc::c_char,
    pub warc_tempdir: *mut libc::c_char,
    pub warc_cdx_dedup_filename: *mut libc::c_char,
    pub warc_maxsize: wgint,
    pub warc_compression_enabled: bool,
    pub warc_digests_enabled: bool,
    pub warc_cdx_enabled: bool,
    pub warc_keep_log: bool,
    pub warc_user_headers: *mut *mut libc::c_char,
    pub enable_xattr: bool,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
    pub ask_passwd: bool,
    pub use_askpass: *mut libc::c_char,
    pub always_rest: bool,
    pub start_pos: wgint,
    pub ftp_user: *mut libc::c_char,
    pub ftp_passwd: *mut libc::c_char,
    pub netrc: bool,
    pub ftp_glob: bool,
    pub ftp_pasv: bool,
    pub http_user: *mut libc::c_char,
    pub http_passwd: *mut libc::c_char,
    pub user_headers: *mut *mut libc::c_char,
    pub http_keep_alive: bool,
    pub use_proxy: bool,
    pub allow_cache: bool,
    pub http_proxy: *mut libc::c_char,
    pub ftp_proxy: *mut libc::c_char,
    pub https_proxy: *mut libc::c_char,
    pub no_proxy: *mut *mut libc::c_char,
    pub base_href: *mut libc::c_char,
    pub progress_type: *mut libc::c_char,
    pub show_progress: libc::c_int,
    pub noscroll: bool,
    pub proxy_user: *mut libc::c_char,
    pub proxy_passwd: *mut libc::c_char,
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
    pub backups: libc::c_int,
    pub useragent: *mut libc::c_char,
    pub referer: *mut libc::c_char,
    pub convert_links: bool,
    pub convert_file_only: bool,
    pub remove_listing: bool,
    pub htmlify: bool,
    pub dot_style: *mut libc::c_char,
    pub dot_bytes: wgint,
    pub dots_in_line: libc::c_int,
    pub dot_spacing: libc::c_int,
    pub delete_after: bool,
    pub adjust_extension: bool,
    pub page_requisites: bool,
    pub bind_address: *mut libc::c_char,
    pub secure_protocol: C2RustUnnamed_2,
    pub secure_protocol_name: [libc::c_char; 8],
    pub check_cert: libc::c_int,
    pub cert_file: *mut libc::c_char,
    pub private_key: *mut libc::c_char,
    pub cert_type: keyfile_type,
    pub private_key_type: keyfile_type,
    pub ca_directory: *mut libc::c_char,
    pub ca_cert: *mut libc::c_char,
    pub crl_file: *mut libc::c_char,
    pub pinnedpubkey: *mut libc::c_char,
    pub random_file: *mut libc::c_char,
    pub egd_file: *mut libc::c_char,
    pub https_only: bool,
    pub ftps_resume_ssl: bool,
    pub ftps_fallback_to_ftp: bool,
    pub ftps_implicit: bool,
    pub ftps_clear_data_connection: bool,
    pub tls_ciphers_string: *mut libc::c_char,
    pub cookies: bool,
    pub cookies_input: *mut libc::c_char,
    pub cookies_output: *mut libc::c_char,
    pub keep_badhash: bool,
    pub keep_session_cookies: bool,
    pub post_data: *mut libc::c_char,
    pub post_file_name: *mut libc::c_char,
    pub method: *mut libc::c_char,
    pub body_data: *mut libc::c_char,
    pub body_file: *mut libc::c_char,
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
    pub encoding_remote: *mut libc::c_char,
    pub locale: *const libc::c_char,
    pub trustservernames: bool,
    pub useservertimestamps: bool,
    pub show_all_dns_entries: bool,
    pub report_bps: bool,
    pub compression: compression_options,
    pub rejected_log: *mut libc::c_char,
    pub hsts: bool,
    pub hsts_file: *mut libc::c_char,
    pub homedir: *const libc::c_char,
    pub wgetrcfile: *const libc::c_char,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum compression_options {
    compression_none = 2,
    compression_gzip = 1,
    compression_auto = 0,
}
impl compression_options {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            compression_options::compression_none => 2,
            compression_options::compression_gzip => 1,
            compression_options::compression_auto => 0,
        }
    }
}

pub const compression_none: compression_options = 2;
pub const compression_gzip: compression_options = 1;
pub const compression_auto: compression_options = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    prefer_none = 2,
    prefer_ipv6 = 1,
    prefer_ipv4 = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::prefer_none => 2,
            C2RustUnnamed::prefer_ipv6 => 1,
            C2RustUnnamed::prefer_ipv4 => 0,
        }
    }
}

pub const prefer_none: C2RustUnnamed = 2;
pub const prefer_ipv6: C2RustUnnamed = 1;
pub const prefer_ipv4: C2RustUnnamed = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    restrict_uppercase = 2,
    restrict_lowercase = 1,
    restrict_no_case_restriction = 0,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_0::restrict_uppercase => 2,
            C2RustUnnamed_0::restrict_lowercase => 1,
            C2RustUnnamed_0::restrict_no_case_restriction => 0,
        }
    }
}

pub const restrict_uppercase: C2RustUnnamed_0 = 2;
pub const restrict_lowercase: C2RustUnnamed_0 = 1;
pub const restrict_no_case_restriction: C2RustUnnamed_0 = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    restrict_windows = 2,
    restrict_vms = 1,
    restrict_unix = 0,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_1::restrict_windows => 2,
            C2RustUnnamed_1::restrict_vms => 1,
            C2RustUnnamed_1::restrict_unix => 0,
        }
    }
}

pub const restrict_windows: C2RustUnnamed_1 = 2;
pub const restrict_vms: C2RustUnnamed_1 = 1;
pub const restrict_unix: C2RustUnnamed_1 = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum keyfile_type {
    keyfile_asn1 = 1,
    keyfile_pem = 0,
}
impl keyfile_type {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            keyfile_type::keyfile_asn1 => 1,
            keyfile_type::keyfile_pem => 0,
        }
    }
}

pub const keyfile_asn1: keyfile_type = 1;
pub const keyfile_pem: keyfile_type = 0;
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
    fn to_libc_c_uint(self) -> libc::c_uint {
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
}

pub const secure_protocol_pfs: C2RustUnnamed_2 = 7;
pub const secure_protocol_tlsv1_3: C2RustUnnamed_2 = 6;
pub const secure_protocol_tlsv1_2: C2RustUnnamed_2 = 5;
pub const secure_protocol_tlsv1_1: C2RustUnnamed_2 = 4;
pub const secure_protocol_tlsv1: C2RustUnnamed_2 = 3;
pub const secure_protocol_sslv3: C2RustUnnamed_2 = 2;
pub const secure_protocol_sslv2: C2RustUnnamed_2 = 1;
pub const secure_protocol_auto: C2RustUnnamed_2 = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    regex_type_posix = 1,
    regex_type_pcre = 0,
}
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_3::regex_type_posix => 1,
            C2RustUnnamed_3::regex_type_pcre => 0,
        }
    }
}

pub const regex_type_posix: C2RustUnnamed_3 = 1;
pub const regex_type_pcre: C2RustUnnamed_3 = 0;
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
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            log_options::LOG_VERBOSE => 0,
            log_options::LOG_NOTQUIET => 1,
            log_options::LOG_NONVERBOSE => 2,
            log_options::LOG_ALWAYS => 3,
            log_options::LOG_PROGRESS => 4,
        }
    }
}

pub const LOG_PROGRESS: log_options = 4;
pub const LOG_ALWAYS: log_options = 3;
pub const LOG_NONVERBOSE: log_options = 2;
pub const LOG_NOTQUIET: log_options = 1;
pub const LOG_VERBOSE: log_options = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iri {
    pub uri_encoding: *mut libc::c_char,
    pub content_encoding: *mut libc::c_char,
    pub orig_url: *mut libc::c_char,
    pub utf8_encode: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    _PC_2_SYMLINKS = 20,
    _PC_SYMLINK_MAX = 19,
    _PC_ALLOC_SIZE_MIN = 18,
    _PC_REC_XFER_ALIGN = 17,
    _PC_REC_MIN_XFER_SIZE = 16,
    _PC_REC_MAX_XFER_SIZE = 15,
    _PC_REC_INCR_XFER_SIZE = 14,
    _PC_FILESIZEBITS = 13,
    _PC_SOCK_MAXBUF = 12,
    _PC_PRIO_IO = 11,
    _PC_ASYNC_IO = 10,
    _PC_SYNC_IO = 9,
    _PC_VDISABLE = 8,
    _PC_NO_TRUNC = 7,
    _PC_CHOWN_RESTRICTED = 6,
    _PC_PIPE_BUF = 5,
    _PC_PATH_MAX = 4,
    _PC_NAME_MAX = 3,
    _PC_MAX_INPUT = 2,
    _PC_MAX_CANON = 1,
    _PC_LINK_MAX = 0,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_4::_PC_2_SYMLINKS => 20,
            C2RustUnnamed_4::_PC_SYMLINK_MAX => 19,
            C2RustUnnamed_4::_PC_ALLOC_SIZE_MIN => 18,
            C2RustUnnamed_4::_PC_REC_XFER_ALIGN => 17,
            C2RustUnnamed_4::_PC_REC_MIN_XFER_SIZE => 16,
            C2RustUnnamed_4::_PC_REC_MAX_XFER_SIZE => 15,
            C2RustUnnamed_4::_PC_REC_INCR_XFER_SIZE => 14,
            C2RustUnnamed_4::_PC_FILESIZEBITS => 13,
            C2RustUnnamed_4::_PC_SOCK_MAXBUF => 12,
            C2RustUnnamed_4::_PC_PRIO_IO => 11,
            C2RustUnnamed_4::_PC_ASYNC_IO => 10,
            C2RustUnnamed_4::_PC_SYNC_IO => 9,
            C2RustUnnamed_4::_PC_VDISABLE => 8,
            C2RustUnnamed_4::_PC_NO_TRUNC => 7,
            C2RustUnnamed_4::_PC_CHOWN_RESTRICTED => 6,
            C2RustUnnamed_4::_PC_PIPE_BUF => 5,
            C2RustUnnamed_4::_PC_PATH_MAX => 4,
            C2RustUnnamed_4::_PC_NAME_MAX => 3,
            C2RustUnnamed_4::_PC_MAX_INPUT => 2,
            C2RustUnnamed_4::_PC_MAX_CANON => 1,
            C2RustUnnamed_4::_PC_LINK_MAX => 0,
        }
    }
}

pub const _PC_2_SYMLINKS: C2RustUnnamed_4 = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed_4 = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed_4 = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed_4 = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed_4 = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed_4 = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed_4 = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed_4 = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed_4 = 12;
pub const _PC_PRIO_IO: C2RustUnnamed_4 = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed_4 = 10;
pub const _PC_SYNC_IO: C2RustUnnamed_4 = 9;
pub const _PC_VDISABLE: C2RustUnnamed_4 = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed_4 = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed_4 = 6;
pub const _PC_PIPE_BUF: C2RustUnnamed_4 = 5;
pub const _PC_PATH_MAX: C2RustUnnamed_4 = 4;
pub const _PC_NAME_MAX: C2RustUnnamed_4 = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed_4 = 2;
pub const _PC_MAX_CANON: C2RustUnnamed_4 = 1;
pub const _PC_LINK_MAX: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: libc::c_int,
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
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            url_auth_mode::URL_AUTH_SHOW => 0,
            url_auth_mode::URL_AUTH_HIDE_PASSWD => 1,
            url_auth_mode::URL_AUTH_HIDE => 2,
        }
    }
}

pub const URL_AUTH_HIDE: url_auth_mode = 2;
pub const URL_AUTH_HIDE_PASSWD: url_auth_mode = 1;
pub const URL_AUTH_SHOW: url_auth_mode = 0;
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
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            url_scheme::SCHEME_HTTP => 0,
            url_scheme::SCHEME_HTTPS => 1,
            url_scheme::SCHEME_FTP => 2,
            url_scheme::SCHEME_FTPS => 3,
            url_scheme::SCHEME_INVALID => 4,
        }
    }
}

pub const SCHEME_INVALID: url_scheme = 4;
pub const SCHEME_FTPS: url_scheme = 3;
pub const SCHEME_FTP: url_scheme = 2;
pub const SCHEME_HTTPS: url_scheme = 1;
pub const SCHEME_HTTP: url_scheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url {
    pub url: *mut libc::c_char,
    pub scheme: url_scheme,
    pub host: *mut libc::c_char,
    pub port: libc::c_int,
    pub path: *mut libc::c_char,
    pub params: *mut libc::c_char,
    pub query: *mut libc::c_char,
    pub fragment: *mut libc::c_char,
    pub dir: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_7 {
    urlchr_reserved = 1,
    urlchr_unsafe = 2,
}
impl C2RustUnnamed_7 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_7::urlchr_reserved => 1,
            C2RustUnnamed_7::urlchr_unsafe => 2,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct scheme_data {
    pub name: *const libc::c_char,
    pub leading_string: *const libc::c_char,
    pub default_port: libc::c_int,
    pub flags: libc::c_int,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_6 {
    scm_disabled = 1,
    scm_has_params = 2,
    scm_has_query = 4,
    scm_has_fragment = 8,
}
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_6::scm_disabled => 1,
            C2RustUnnamed_6::scm_has_params => 2,
            C2RustUnnamed_6::scm_has_query => 4,
            C2RustUnnamed_6::scm_has_fragment => 8,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_8 {
    PE_NO_ERROR = 0,
    PE_UNSUPPORTED_SCHEME,
    PE_UNSUPPORTED_SCHEME_HTTPS,
    PE_UNSUPPORTED_SCHEME_FTPS,
    PE_MISSING_SCHEME,
    PE_INVALID_HOST_NAME,
    PE_BAD_PORT_NUMBER,
    PE_INVALID_USER_NAME,
    PE_UNTERMINATED_IPV6_ADDRESS,
    PE_IPV6_NOT_SUPPORTED,
    PE_INVALID_IPV6_ADDRESS,
}
impl C2RustUnnamed_8 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_8::PE_NO_ERROR => 0,
            C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME => 1,
            C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_HTTPS => 2,
            C2RustUnnamed_8::PE_UNSUPPORTED_SCHEME_FTPS => 3,
            C2RustUnnamed_8::PE_MISSING_SCHEME => 4,
            C2RustUnnamed_8::PE_INVALID_HOST_NAME => 5,
            C2RustUnnamed_8::PE_BAD_PORT_NUMBER => 6,
            C2RustUnnamed_8::PE_INVALID_USER_NAME => 7,
            C2RustUnnamed_8::PE_UNTERMINATED_IPV6_ADDRESS => 8,
            C2RustUnnamed_8::PE_IPV6_NOT_SUPPORTED => 9,
            C2RustUnnamed_8::PE_INVALID_IPV6_ADDRESS => 10,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct growable {
    pub base: *mut libc::c_char,
    pub size: libc::c_int,
    pub tail: libc::c_int,
}
pub type iconv_t = *mut libc::c_void;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    CODESET = 14,
    _NL_NUM = 786449,
    _NL_NUM_LC_IDENTIFICATION = 786448,
    _NL_IDENTIFICATION_CODESET = 786447,
    _NL_IDENTIFICATION_CATEGORY = 786446,
    _NL_IDENTIFICATION_DATE = 786445,
    _NL_IDENTIFICATION_REVISION = 786444,
    _NL_IDENTIFICATION_ABBREVIATION = 786443,
    _NL_IDENTIFICATION_APPLICATION = 786442,
    _NL_IDENTIFICATION_AUDIENCE = 786441,
    _NL_IDENTIFICATION_TERRITORY = 786440,
    _NL_IDENTIFICATION_LANGUAGE = 786439,
    _NL_IDENTIFICATION_FAX = 786438,
    _NL_IDENTIFICATION_TEL = 786437,
    _NL_IDENTIFICATION_EMAIL = 786436,
    _NL_IDENTIFICATION_CONTACT = 786435,
    _NL_IDENTIFICATION_ADDRESS = 786434,
    _NL_IDENTIFICATION_SOURCE = 786433,
    _NL_IDENTIFICATION_TITLE = 786432,
    _NL_NUM_LC_MEASUREMENT = 720898,
    _NL_MEASUREMENT_CODESET = 720897,
    _NL_MEASUREMENT_MEASUREMENT = 720896,
    _NL_NUM_LC_TELEPHONE = 655365,
    _NL_TELEPHONE_CODESET = 655364,
    _NL_TELEPHONE_INT_PREFIX = 655363,
    _NL_TELEPHONE_INT_SELECT = 655362,
    _NL_TELEPHONE_TEL_DOM_FMT = 655361,
    _NL_TELEPHONE_TEL_INT_FMT = 655360,
    _NL_NUM_LC_ADDRESS = 589837,
    _NL_ADDRESS_CODESET = 589836,
    _NL_ADDRESS_LANG_LIB = 589835,
    _NL_ADDRESS_LANG_TERM = 589834,
    _NL_ADDRESS_LANG_AB = 589833,
    _NL_ADDRESS_LANG_NAME = 589832,
    _NL_ADDRESS_COUNTRY_ISBN = 589831,
    _NL_ADDRESS_COUNTRY_NUM = 589830,
    _NL_ADDRESS_COUNTRY_CAR = 589829,
    _NL_ADDRESS_COUNTRY_AB3 = 589828,
    _NL_ADDRESS_COUNTRY_AB2 = 589827,
    _NL_ADDRESS_COUNTRY_POST = 589826,
    _NL_ADDRESS_COUNTRY_NAME = 589825,
    _NL_ADDRESS_POSTAL_FMT = 589824,
    _NL_NUM_LC_NAME = 524295,
    _NL_NAME_CODESET = 524294,
    _NL_NAME_NAME_MS = 524293,
    _NL_NAME_NAME_MISS = 524292,
    _NL_NAME_NAME_MRS = 524291,
    _NL_NAME_NAME_MR = 524290,
    _NL_NAME_NAME_GEN = 524289,
    _NL_NAME_NAME_FMT = 524288,
    _NL_NUM_LC_PAPER = 458755,
    _NL_PAPER_CODESET = 458754,
    _NL_PAPER_WIDTH = 458753,
    _NL_PAPER_HEIGHT = 458752,
    _NL_NUM_LC_MESSAGES = 327685,
    _NL_MESSAGES_CODESET = 327684,
    __NOSTR = 327683,
    __YESSTR = 327682,
    __NOEXPR = 327681,
    __YESEXPR = 327680,
    _NL_NUM_LC_NUMERIC = 65542,
    _NL_NUMERIC_CODESET = 65541,
    _NL_NUMERIC_THOUSANDS_SEP_WC = 65540,
    _NL_NUMERIC_DECIMAL_POINT_WC = 65539,
    __GROUPING = 65538,
    THOUSEP = 65537,
    __THOUSANDS_SEP = 65537,
    RADIXCHAR = 65536,
    __DECIMAL_POINT = 65536,
    _NL_NUM_LC_MONETARY = 262190,
    _NL_MONETARY_CODESET = 262189,
    _NL_MONETARY_THOUSANDS_SEP_WC = 262188,
    _NL_MONETARY_DECIMAL_POINT_WC = 262187,
    _NL_MONETARY_CONVERSION_RATE = 262186,
    _NL_MONETARY_DUO_VALID_TO = 262185,
    _NL_MONETARY_DUO_VALID_FROM = 262184,
    _NL_MONETARY_UNO_VALID_TO = 262183,
    _NL_MONETARY_UNO_VALID_FROM = 262182,
    _NL_MONETARY_DUO_INT_N_SIGN_POSN = 262181,
    _NL_MONETARY_DUO_INT_P_SIGN_POSN = 262180,
    _NL_MONETARY_DUO_N_SIGN_POSN = 262179,
    _NL_MONETARY_DUO_P_SIGN_POSN = 262178,
    _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE = 262177,
    _NL_MONETARY_DUO_INT_N_CS_PRECEDES = 262176,
    _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE = 262175,
    _NL_MONETARY_DUO_INT_P_CS_PRECEDES = 262174,
    _NL_MONETARY_DUO_N_SEP_BY_SPACE = 262173,
    _NL_MONETARY_DUO_N_CS_PRECEDES = 262172,
    _NL_MONETARY_DUO_P_SEP_BY_SPACE = 262171,
    _NL_MONETARY_DUO_P_CS_PRECEDES = 262170,
    _NL_MONETARY_DUO_FRAC_DIGITS = 262169,
    _NL_MONETARY_DUO_INT_FRAC_DIGITS = 262168,
    _NL_MONETARY_DUO_CURRENCY_SYMBOL = 262167,
    _NL_MONETARY_DUO_INT_CURR_SYMBOL = 262166,
    __INT_N_SIGN_POSN = 262165,
    __INT_P_SIGN_POSN = 262164,
    __INT_N_SEP_BY_SPACE = 262163,
    __INT_N_CS_PRECEDES = 262162,
    __INT_P_SEP_BY_SPACE = 262161,
    __INT_P_CS_PRECEDES = 262160,
    _NL_MONETARY_CRNCYSTR = 262159,
    __N_SIGN_POSN = 262158,
    __P_SIGN_POSN = 262157,
    __N_SEP_BY_SPACE = 262156,
    __N_CS_PRECEDES = 262155,
    __P_SEP_BY_SPACE = 262154,
    __P_CS_PRECEDES = 262153,
    __FRAC_DIGITS = 262152,
    __INT_FRAC_DIGITS = 262151,
    __NEGATIVE_SIGN = 262150,
    __POSITIVE_SIGN = 262149,
    __MON_GROUPING = 262148,
    __MON_THOUSANDS_SEP = 262147,
    __MON_DECIMAL_POINT = 262146,
    __CURRENCY_SYMBOL = 262145,
    __INT_CURR_SYMBOL = 262144,
    _NL_NUM_LC_CTYPE = 86,
    _NL_CTYPE_EXTRA_MAP_14 = 85,
    _NL_CTYPE_EXTRA_MAP_13 = 84,
    _NL_CTYPE_EXTRA_MAP_12 = 83,
    _NL_CTYPE_EXTRA_MAP_11 = 82,
    _NL_CTYPE_EXTRA_MAP_10 = 81,
    _NL_CTYPE_EXTRA_MAP_9 = 80,
    _NL_CTYPE_EXTRA_MAP_8 = 79,
    _NL_CTYPE_EXTRA_MAP_7 = 78,
    _NL_CTYPE_EXTRA_MAP_6 = 77,
    _NL_CTYPE_EXTRA_MAP_5 = 76,
    _NL_CTYPE_EXTRA_MAP_4 = 75,
    _NL_CTYPE_EXTRA_MAP_3 = 74,
    _NL_CTYPE_EXTRA_MAP_2 = 73,
    _NL_CTYPE_EXTRA_MAP_1 = 72,
    _NL_CTYPE_NONASCII_CASE = 71,
    _NL_CTYPE_MAP_TO_NONASCII = 70,
    _NL_CTYPE_TRANSLIT_IGNORE = 69,
    _NL_CTYPE_TRANSLIT_IGNORE_LEN = 68,
    _NL_CTYPE_TRANSLIT_DEFAULT_MISSING = 67,
    _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN = 66,
    _NL_CTYPE_TRANSLIT_TO_TBL = 65,
    _NL_CTYPE_TRANSLIT_TO_IDX = 64,
    _NL_CTYPE_TRANSLIT_FROM_TBL = 63,
    _NL_CTYPE_TRANSLIT_FROM_IDX = 62,
    _NL_CTYPE_TRANSLIT_TAB_SIZE = 61,
    _NL_CTYPE_OUTDIGIT9_WC = 60,
    _NL_CTYPE_OUTDIGIT8_WC = 59,
    _NL_CTYPE_OUTDIGIT7_WC = 58,
    _NL_CTYPE_OUTDIGIT6_WC = 57,
    _NL_CTYPE_OUTDIGIT5_WC = 56,
    _NL_CTYPE_OUTDIGIT4_WC = 55,
    _NL_CTYPE_OUTDIGIT3_WC = 54,
    _NL_CTYPE_OUTDIGIT2_WC = 53,
    _NL_CTYPE_OUTDIGIT1_WC = 52,
    _NL_CTYPE_OUTDIGIT0_WC = 51,
    _NL_CTYPE_OUTDIGIT9_MB = 50,
    _NL_CTYPE_OUTDIGIT8_MB = 49,
    _NL_CTYPE_OUTDIGIT7_MB = 48,
    _NL_CTYPE_OUTDIGIT6_MB = 47,
    _NL_CTYPE_OUTDIGIT5_MB = 46,
    _NL_CTYPE_OUTDIGIT4_MB = 45,
    _NL_CTYPE_OUTDIGIT3_MB = 44,
    _NL_CTYPE_OUTDIGIT2_MB = 43,
    _NL_CTYPE_OUTDIGIT1_MB = 42,
    _NL_CTYPE_OUTDIGIT0_MB = 41,
    _NL_CTYPE_INDIGITS9_WC = 40,
    _NL_CTYPE_INDIGITS8_WC = 39,
    _NL_CTYPE_INDIGITS7_WC = 38,
    _NL_CTYPE_INDIGITS6_WC = 37,
    _NL_CTYPE_INDIGITS5_WC = 36,
    _NL_CTYPE_INDIGITS4_WC = 35,
    _NL_CTYPE_INDIGITS3_WC = 34,
    _NL_CTYPE_INDIGITS2_WC = 33,
    _NL_CTYPE_INDIGITS1_WC = 32,
    _NL_CTYPE_INDIGITS0_WC = 31,
    _NL_CTYPE_INDIGITS_WC_LEN = 30,
    _NL_CTYPE_INDIGITS9_MB = 29,
    _NL_CTYPE_INDIGITS8_MB = 28,
    _NL_CTYPE_INDIGITS7_MB = 27,
    _NL_CTYPE_INDIGITS6_MB = 26,
    _NL_CTYPE_INDIGITS5_MB = 25,
    _NL_CTYPE_INDIGITS4_MB = 24,
    _NL_CTYPE_INDIGITS3_MB = 23,
    _NL_CTYPE_INDIGITS2_MB = 22,
    _NL_CTYPE_INDIGITS1_MB = 21,
    _NL_CTYPE_INDIGITS0_MB = 20,
    _NL_CTYPE_INDIGITS_MB_LEN = 19,
    _NL_CTYPE_MAP_OFFSET = 18,
    _NL_CTYPE_CLASS_OFFSET = 17,
    _NL_CTYPE_TOLOWER32 = 16,
    _NL_CTYPE_TOUPPER32 = 15,
    _NL_CTYPE_CODESET_NAME = 14,
    _NL_CTYPE_MB_CUR_MAX = 13,
    _NL_CTYPE_WIDTH = 12,
    _NL_CTYPE_MAP_NAMES = 11,
    _NL_CTYPE_CLASS_NAMES = 10,
    _NL_CTYPE_GAP6 = 9,
    _NL_CTYPE_GAP5 = 8,
    _NL_CTYPE_GAP4 = 7,
    _NL_CTYPE_GAP3 = 6,
    _NL_CTYPE_CLASS32 = 5,
    _NL_CTYPE_GAP2 = 4,
    _NL_CTYPE_TOLOWER = 3,
    _NL_CTYPE_GAP1 = 2,
    _NL_CTYPE_TOUPPER = 1,
    _NL_CTYPE_CLASS = 0,
    _NL_NUM_LC_COLLATE = 196627,
    _NL_COLLATE_CODESET = 196626,
    _NL_COLLATE_COLLSEQWC = 196625,
    _NL_COLLATE_COLLSEQMB = 196624,
    _NL_COLLATE_SYMB_EXTRAMB = 196623,
    _NL_COLLATE_SYMB_TABLEMB = 196622,
    _NL_COLLATE_SYMB_HASH_SIZEMB = 196621,
    _NL_COLLATE_INDIRECTWC = 196620,
    _NL_COLLATE_EXTRAWC = 196619,
    _NL_COLLATE_WEIGHTWC = 196618,
    _NL_COLLATE_TABLEWC = 196617,
    _NL_COLLATE_GAP3 = 196616,
    _NL_COLLATE_GAP2 = 196615,
    _NL_COLLATE_GAP1 = 196614,
    _NL_COLLATE_INDIRECTMB = 196613,
    _NL_COLLATE_EXTRAMB = 196612,
    _NL_COLLATE_WEIGHTMB = 196611,
    _NL_COLLATE_TABLEMB = 196610,
    _NL_COLLATE_RULESETS = 196609,
    _NL_COLLATE_NRULES = 196608,
    _NL_NUM_LC_TIME = 131231,
    _NL_WABALTMON_12 = 131230,
    _NL_WABALTMON_11 = 131229,
    _NL_WABALTMON_10 = 131228,
    _NL_WABALTMON_9 = 131227,
    _NL_WABALTMON_8 = 131226,
    _NL_WABALTMON_7 = 131225,
    _NL_WABALTMON_6 = 131224,
    _NL_WABALTMON_5 = 131223,
    _NL_WABALTMON_4 = 131222,
    _NL_WABALTMON_3 = 131221,
    _NL_WABALTMON_2 = 131220,
    _NL_WABALTMON_1 = 131219,
    _NL_ABALTMON_12 = 131218,
    _NL_ABALTMON_11 = 131217,
    _NL_ABALTMON_10 = 131216,
    _NL_ABALTMON_9 = 131215,
    _NL_ABALTMON_8 = 131214,
    _NL_ABALTMON_7 = 131213,
    _NL_ABALTMON_6 = 131212,
    _NL_ABALTMON_5 = 131211,
    _NL_ABALTMON_4 = 131210,
    _NL_ABALTMON_3 = 131209,
    _NL_ABALTMON_2 = 131208,
    _NL_ABALTMON_1 = 131207,
    _NL_WALTMON_12 = 131206,
    _NL_WALTMON_11 = 131205,
    _NL_WALTMON_10 = 131204,
    _NL_WALTMON_9 = 131203,
    _NL_WALTMON_8 = 131202,
    _NL_WALTMON_7 = 131201,
    _NL_WALTMON_6 = 131200,
    _NL_WALTMON_5 = 131199,
    _NL_WALTMON_4 = 131198,
    _NL_WALTMON_3 = 131197,
    _NL_WALTMON_2 = 131196,
    _NL_WALTMON_1 = 131195,
    __ALTMON_12 = 131194,
    __ALTMON_11 = 131193,
    __ALTMON_10 = 131192,
    __ALTMON_9 = 131191,
    __ALTMON_8 = 131190,
    __ALTMON_7 = 131189,
    __ALTMON_6 = 131188,
    __ALTMON_5 = 131187,
    __ALTMON_4 = 131186,
    __ALTMON_3 = 131185,
    __ALTMON_2 = 131184,
    __ALTMON_1 = 131183,
    _NL_TIME_CODESET = 131182,
    _NL_W_DATE_FMT = 131181,
    _DATE_FMT = 131180,
    _NL_TIME_TIMEZONE = 131179,
    _NL_TIME_CAL_DIRECTION = 131178,
    _NL_TIME_FIRST_WORKDAY = 131177,
    _NL_TIME_FIRST_WEEKDAY = 131176,
    _NL_TIME_WEEK_1STWEEK = 131175,
    _NL_TIME_WEEK_1STDAY = 131174,
    _NL_TIME_WEEK_NDAYS = 131173,
    _NL_WERA_T_FMT = 131172,
    _NL_WERA_D_T_FMT = 131171,
    _NL_WALT_DIGITS = 131170,
    _NL_WERA_D_FMT = 131169,
    _NL_WERA_YEAR = 131168,
    _NL_WT_FMT_AMPM = 131167,
    _NL_WT_FMT = 131166,
    _NL_WD_FMT = 131165,
    _NL_WD_T_FMT = 131164,
    _NL_WPM_STR = 131163,
    _NL_WAM_STR = 131162,
    _NL_WMON_12 = 131161,
    _NL_WMON_11 = 131160,
    _NL_WMON_10 = 131159,
    _NL_WMON_9 = 131158,
    _NL_WMON_8 = 131157,
    _NL_WMON_7 = 131156,
    _NL_WMON_6 = 131155,
    _NL_WMON_5 = 131154,
    _NL_WMON_4 = 131153,
    _NL_WMON_3 = 131152,
    _NL_WMON_2 = 131151,
    _NL_WMON_1 = 131150,
    _NL_WABMON_12 = 131149,
    _NL_WABMON_11 = 131148,
    _NL_WABMON_10 = 131147,
    _NL_WABMON_9 = 131146,
    _NL_WABMON_8 = 131145,
    _NL_WABMON_7 = 131144,
    _NL_WABMON_6 = 131143,
    _NL_WABMON_5 = 131142,
    _NL_WABMON_4 = 131141,
    _NL_WABMON_3 = 131140,
    _NL_WABMON_2 = 131139,
    _NL_WABMON_1 = 131138,
    _NL_WDAY_7 = 131137,
    _NL_WDAY_6 = 131136,
    _NL_WDAY_5 = 131135,
    _NL_WDAY_4 = 131134,
    _NL_WDAY_3 = 131133,
    _NL_WDAY_2 = 131132,
    _NL_WDAY_1 = 131131,
    _NL_WABDAY_7 = 131130,
    _NL_WABDAY_6 = 131129,
    _NL_WABDAY_5 = 131128,
    _NL_WABDAY_4 = 131127,
    _NL_WABDAY_3 = 131126,
    _NL_WABDAY_2 = 131125,
    _NL_WABDAY_1 = 131124,
    _NL_TIME_ERA_ENTRIES = 131123,
    _NL_TIME_ERA_NUM_ENTRIES = 131122,
    ERA_T_FMT = 131121,
    ERA_D_T_FMT = 131120,
    ALT_DIGITS = 131119,
    ERA_D_FMT = 131118,
    __ERA_YEAR = 131117,
    ERA = 131116,
    T_FMT_AMPM = 131115,
    T_FMT = 131114,
    D_FMT = 131113,
    D_T_FMT = 131112,
    PM_STR = 131111,
    AM_STR = 131110,
    MON_12 = 131109,
    MON_11 = 131108,
    MON_10 = 131107,
    MON_9 = 131106,
    MON_8 = 131105,
    MON_7 = 131104,
    MON_6 = 131103,
    MON_5 = 131102,
    MON_4 = 131101,
    MON_3 = 131100,
    MON_2 = 131099,
    MON_1 = 131098,
    ABMON_12 = 131097,
    ABMON_11 = 131096,
    ABMON_10 = 131095,
    ABMON_9 = 131094,
    ABMON_8 = 131093,
    ABMON_7 = 131092,
    ABMON_6 = 131091,
    ABMON_5 = 131090,
    ABMON_4 = 131089,
    ABMON_3 = 131088,
    ABMON_2 = 131087,
    ABMON_1 = 131086,
    DAY_7 = 131085,
    DAY_6 = 131084,
    DAY_5 = 131083,
    DAY_4 = 131082,
    DAY_3 = 131081,
    DAY_2 = 131080,
    DAY_1 = 131079,
    ABDAY_7 = 131078,
    ABDAY_6 = 131077,
    ABDAY_5 = 131076,
    ABDAY_4 = 131075,
    ABDAY_3 = 131074,
    ABDAY_2 = 131073,
    ABDAY_1 = 131072,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_5::CODESET => 14,
            C2RustUnnamed_5::_NL_NUM => 786449,
            C2RustUnnamed_5::_NL_NUM_LC_IDENTIFICATION => 786448,
            C2RustUnnamed_5::_NL_IDENTIFICATION_CODESET => 786447,
            C2RustUnnamed_5::_NL_IDENTIFICATION_CATEGORY => 786446,
            C2RustUnnamed_5::_NL_IDENTIFICATION_DATE => 786445,
            C2RustUnnamed_5::_NL_IDENTIFICATION_REVISION => 786444,
            C2RustUnnamed_5::_NL_IDENTIFICATION_ABBREVIATION => 786443,
            C2RustUnnamed_5::_NL_IDENTIFICATION_APPLICATION => 786442,
            C2RustUnnamed_5::_NL_IDENTIFICATION_AUDIENCE => 786441,
            C2RustUnnamed_5::_NL_IDENTIFICATION_TERRITORY => 786440,
            C2RustUnnamed_5::_NL_IDENTIFICATION_LANGUAGE => 786439,
            C2RustUnnamed_5::_NL_IDENTIFICATION_FAX => 786438,
            C2RustUnnamed_5::_NL_IDENTIFICATION_TEL => 786437,
            C2RustUnnamed_5::_NL_IDENTIFICATION_EMAIL => 786436,
            C2RustUnnamed_5::_NL_IDENTIFICATION_CONTACT => 786435,
            C2RustUnnamed_5::_NL_IDENTIFICATION_ADDRESS => 786434,
            C2RustUnnamed_5::_NL_IDENTIFICATION_SOURCE => 786433,
            C2RustUnnamed_5::_NL_IDENTIFICATION_TITLE => 786432,
            C2RustUnnamed_5::_NL_NUM_LC_MEASUREMENT => 720898,
            C2RustUnnamed_5::_NL_MEASUREMENT_CODESET => 720897,
            C2RustUnnamed_5::_NL_MEASUREMENT_MEASUREMENT => 720896,
            C2RustUnnamed_5::_NL_NUM_LC_TELEPHONE => 655365,
            C2RustUnnamed_5::_NL_TELEPHONE_CODESET => 655364,
            C2RustUnnamed_5::_NL_TELEPHONE_INT_PREFIX => 655363,
            C2RustUnnamed_5::_NL_TELEPHONE_INT_SELECT => 655362,
            C2RustUnnamed_5::_NL_TELEPHONE_TEL_DOM_FMT => 655361,
            C2RustUnnamed_5::_NL_TELEPHONE_TEL_INT_FMT => 655360,
            C2RustUnnamed_5::_NL_NUM_LC_ADDRESS => 589837,
            C2RustUnnamed_5::_NL_ADDRESS_CODESET => 589836,
            C2RustUnnamed_5::_NL_ADDRESS_LANG_LIB => 589835,
            C2RustUnnamed_5::_NL_ADDRESS_LANG_TERM => 589834,
            C2RustUnnamed_5::_NL_ADDRESS_LANG_AB => 589833,
            C2RustUnnamed_5::_NL_ADDRESS_LANG_NAME => 589832,
            C2RustUnnamed_5::_NL_ADDRESS_COUNTRY_ISBN => 589831,
            C2RustUnnamed_5::_NL_ADDRESS_COUNTRY_NUM => 589830,
            C2RustUnnamed_5::_NL_ADDRESS_COUNTRY_CAR => 589829,
            C2RustUnnamed_5::_NL_ADDRESS_COUNTRY_AB3 => 589828,
            C2RustUnnamed_5::_NL_ADDRESS_COUNTRY_AB2 => 589827,
            C2RustUnnamed_5::_NL_ADDRESS_COUNTRY_POST => 589826,
            C2RustUnnamed_5::_NL_ADDRESS_COUNTRY_NAME => 589825,
            C2RustUnnamed_5::_NL_ADDRESS_POSTAL_FMT => 589824,
            C2RustUnnamed_5::_NL_NUM_LC_NAME => 524295,
            C2RustUnnamed_5::_NL_NAME_CODESET => 524294,
            C2RustUnnamed_5::_NL_NAME_NAME_MS => 524293,
            C2RustUnnamed_5::_NL_NAME_NAME_MISS => 524292,
            C2RustUnnamed_5::_NL_NAME_NAME_MRS => 524291,
            C2RustUnnamed_5::_NL_NAME_NAME_MR => 524290,
            C2RustUnnamed_5::_NL_NAME_NAME_GEN => 524289,
            C2RustUnnamed_5::_NL_NAME_NAME_FMT => 524288,
            C2RustUnnamed_5::_NL_NUM_LC_PAPER => 458755,
            C2RustUnnamed_5::_NL_PAPER_CODESET => 458754,
            C2RustUnnamed_5::_NL_PAPER_WIDTH => 458753,
            C2RustUnnamed_5::_NL_PAPER_HEIGHT => 458752,
            C2RustUnnamed_5::_NL_NUM_LC_MESSAGES => 327685,
            C2RustUnnamed_5::_NL_MESSAGES_CODESET => 327684,
            C2RustUnnamed_5::__NOSTR => 327683,
            C2RustUnnamed_5::__YESSTR => 327682,
            C2RustUnnamed_5::__NOEXPR => 327681,
            C2RustUnnamed_5::__YESEXPR => 327680,
            C2RustUnnamed_5::_NL_NUM_LC_NUMERIC => 65542,
            C2RustUnnamed_5::_NL_NUMERIC_CODESET => 65541,
            C2RustUnnamed_5::_NL_NUMERIC_THOUSANDS_SEP_WC => 65540,
            C2RustUnnamed_5::_NL_NUMERIC_DECIMAL_POINT_WC => 65539,
            C2RustUnnamed_5::__GROUPING => 65538,
            C2RustUnnamed_5::THOUSEP => 65537,
            C2RustUnnamed_5::__THOUSANDS_SEP => 65537,
            C2RustUnnamed_5::RADIXCHAR => 65536,
            C2RustUnnamed_5::__DECIMAL_POINT => 65536,
            C2RustUnnamed_5::_NL_NUM_LC_MONETARY => 262190,
            C2RustUnnamed_5::_NL_MONETARY_CODESET => 262189,
            C2RustUnnamed_5::_NL_MONETARY_THOUSANDS_SEP_WC => 262188,
            C2RustUnnamed_5::_NL_MONETARY_DECIMAL_POINT_WC => 262187,
            C2RustUnnamed_5::_NL_MONETARY_CONVERSION_RATE => 262186,
            C2RustUnnamed_5::_NL_MONETARY_DUO_VALID_TO => 262185,
            C2RustUnnamed_5::_NL_MONETARY_DUO_VALID_FROM => 262184,
            C2RustUnnamed_5::_NL_MONETARY_UNO_VALID_TO => 262183,
            C2RustUnnamed_5::_NL_MONETARY_UNO_VALID_FROM => 262182,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_N_SIGN_POSN => 262181,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_P_SIGN_POSN => 262180,
            C2RustUnnamed_5::_NL_MONETARY_DUO_N_SIGN_POSN => 262179,
            C2RustUnnamed_5::_NL_MONETARY_DUO_P_SIGN_POSN => 262178,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_N_SEP_BY_SPACE => 262177,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_N_CS_PRECEDES => 262176,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_P_SEP_BY_SPACE => 262175,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_P_CS_PRECEDES => 262174,
            C2RustUnnamed_5::_NL_MONETARY_DUO_N_SEP_BY_SPACE => 262173,
            C2RustUnnamed_5::_NL_MONETARY_DUO_N_CS_PRECEDES => 262172,
            C2RustUnnamed_5::_NL_MONETARY_DUO_P_SEP_BY_SPACE => 262171,
            C2RustUnnamed_5::_NL_MONETARY_DUO_P_CS_PRECEDES => 262170,
            C2RustUnnamed_5::_NL_MONETARY_DUO_FRAC_DIGITS => 262169,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_FRAC_DIGITS => 262168,
            C2RustUnnamed_5::_NL_MONETARY_DUO_CURRENCY_SYMBOL => 262167,
            C2RustUnnamed_5::_NL_MONETARY_DUO_INT_CURR_SYMBOL => 262166,
            C2RustUnnamed_5::__INT_N_SIGN_POSN => 262165,
            C2RustUnnamed_5::__INT_P_SIGN_POSN => 262164,
            C2RustUnnamed_5::__INT_N_SEP_BY_SPACE => 262163,
            C2RustUnnamed_5::__INT_N_CS_PRECEDES => 262162,
            C2RustUnnamed_5::__INT_P_SEP_BY_SPACE => 262161,
            C2RustUnnamed_5::__INT_P_CS_PRECEDES => 262160,
            C2RustUnnamed_5::_NL_MONETARY_CRNCYSTR => 262159,
            C2RustUnnamed_5::__N_SIGN_POSN => 262158,
            C2RustUnnamed_5::__P_SIGN_POSN => 262157,
            C2RustUnnamed_5::__N_SEP_BY_SPACE => 262156,
            C2RustUnnamed_5::__N_CS_PRECEDES => 262155,
            C2RustUnnamed_5::__P_SEP_BY_SPACE => 262154,
            C2RustUnnamed_5::__P_CS_PRECEDES => 262153,
            C2RustUnnamed_5::__FRAC_DIGITS => 262152,
            C2RustUnnamed_5::__INT_FRAC_DIGITS => 262151,
            C2RustUnnamed_5::__NEGATIVE_SIGN => 262150,
            C2RustUnnamed_5::__POSITIVE_SIGN => 262149,
            C2RustUnnamed_5::__MON_GROUPING => 262148,
            C2RustUnnamed_5::__MON_THOUSANDS_SEP => 262147,
            C2RustUnnamed_5::__MON_DECIMAL_POINT => 262146,
            C2RustUnnamed_5::__CURRENCY_SYMBOL => 262145,
            C2RustUnnamed_5::__INT_CURR_SYMBOL => 262144,
            C2RustUnnamed_5::_NL_NUM_LC_CTYPE => 86,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_14 => 85,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_13 => 84,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_12 => 83,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_11 => 82,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_10 => 81,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_9 => 80,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_8 => 79,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_7 => 78,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_6 => 77,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_5 => 76,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_4 => 75,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_3 => 74,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_2 => 73,
            C2RustUnnamed_5::_NL_CTYPE_EXTRA_MAP_1 => 72,
            C2RustUnnamed_5::_NL_CTYPE_NONASCII_CASE => 71,
            C2RustUnnamed_5::_NL_CTYPE_MAP_TO_NONASCII => 70,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_IGNORE => 69,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_IGNORE_LEN => 68,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_DEFAULT_MISSING => 67,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN => 66,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_TO_TBL => 65,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_TO_IDX => 64,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_FROM_TBL => 63,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_FROM_IDX => 62,
            C2RustUnnamed_5::_NL_CTYPE_TRANSLIT_TAB_SIZE => 61,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT9_WC => 60,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT8_WC => 59,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT7_WC => 58,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT6_WC => 57,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT5_WC => 56,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT4_WC => 55,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT3_WC => 54,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT2_WC => 53,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT1_WC => 52,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT0_WC => 51,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT9_MB => 50,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT8_MB => 49,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT7_MB => 48,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT6_MB => 47,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT5_MB => 46,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT4_MB => 45,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT3_MB => 44,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT2_MB => 43,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT1_MB => 42,
            C2RustUnnamed_5::_NL_CTYPE_OUTDIGIT0_MB => 41,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS9_WC => 40,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS8_WC => 39,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS7_WC => 38,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS6_WC => 37,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS5_WC => 36,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS4_WC => 35,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS3_WC => 34,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS2_WC => 33,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS1_WC => 32,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS0_WC => 31,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS_WC_LEN => 30,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS9_MB => 29,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS8_MB => 28,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS7_MB => 27,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS6_MB => 26,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS5_MB => 25,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS4_MB => 24,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS3_MB => 23,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS2_MB => 22,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS1_MB => 21,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS0_MB => 20,
            C2RustUnnamed_5::_NL_CTYPE_INDIGITS_MB_LEN => 19,
            C2RustUnnamed_5::_NL_CTYPE_MAP_OFFSET => 18,
            C2RustUnnamed_5::_NL_CTYPE_CLASS_OFFSET => 17,
            C2RustUnnamed_5::_NL_CTYPE_TOLOWER32 => 16,
            C2RustUnnamed_5::_NL_CTYPE_TOUPPER32 => 15,
            C2RustUnnamed_5::_NL_CTYPE_CODESET_NAME => 14,
            C2RustUnnamed_5::_NL_CTYPE_MB_CUR_MAX => 13,
            C2RustUnnamed_5::_NL_CTYPE_WIDTH => 12,
            C2RustUnnamed_5::_NL_CTYPE_MAP_NAMES => 11,
            C2RustUnnamed_5::_NL_CTYPE_CLASS_NAMES => 10,
            C2RustUnnamed_5::_NL_CTYPE_GAP6 => 9,
            C2RustUnnamed_5::_NL_CTYPE_GAP5 => 8,
            C2RustUnnamed_5::_NL_CTYPE_GAP4 => 7,
            C2RustUnnamed_5::_NL_CTYPE_GAP3 => 6,
            C2RustUnnamed_5::_NL_CTYPE_CLASS32 => 5,
            C2RustUnnamed_5::_NL_CTYPE_GAP2 => 4,
            C2RustUnnamed_5::_NL_CTYPE_TOLOWER => 3,
            C2RustUnnamed_5::_NL_CTYPE_GAP1 => 2,
            C2RustUnnamed_5::_NL_CTYPE_TOUPPER => 1,
            C2RustUnnamed_5::_NL_CTYPE_CLASS => 0,
            C2RustUnnamed_5::_NL_NUM_LC_COLLATE => 196627,
            C2RustUnnamed_5::_NL_COLLATE_CODESET => 196626,
            C2RustUnnamed_5::_NL_COLLATE_COLLSEQWC => 196625,
            C2RustUnnamed_5::_NL_COLLATE_COLLSEQMB => 196624,
            C2RustUnnamed_5::_NL_COLLATE_SYMB_EXTRAMB => 196623,
            C2RustUnnamed_5::_NL_COLLATE_SYMB_TABLEMB => 196622,
            C2RustUnnamed_5::_NL_COLLATE_SYMB_HASH_SIZEMB => 196621,
            C2RustUnnamed_5::_NL_COLLATE_INDIRECTWC => 196620,
            C2RustUnnamed_5::_NL_COLLATE_EXTRAWC => 196619,
            C2RustUnnamed_5::_NL_COLLATE_WEIGHTWC => 196618,
            C2RustUnnamed_5::_NL_COLLATE_TABLEWC => 196617,
            C2RustUnnamed_5::_NL_COLLATE_GAP3 => 196616,
            C2RustUnnamed_5::_NL_COLLATE_GAP2 => 196615,
            C2RustUnnamed_5::_NL_COLLATE_GAP1 => 196614,
            C2RustUnnamed_5::_NL_COLLATE_INDIRECTMB => 196613,
            C2RustUnnamed_5::_NL_COLLATE_EXTRAMB => 196612,
            C2RustUnnamed_5::_NL_COLLATE_WEIGHTMB => 196611,
            C2RustUnnamed_5::_NL_COLLATE_TABLEMB => 196610,
            C2RustUnnamed_5::_NL_COLLATE_RULESETS => 196609,
            C2RustUnnamed_5::_NL_COLLATE_NRULES => 196608,
            C2RustUnnamed_5::_NL_NUM_LC_TIME => 131231,
            C2RustUnnamed_5::_NL_WABALTMON_12 => 131230,
            C2RustUnnamed_5::_NL_WABALTMON_11 => 131229,
            C2RustUnnamed_5::_NL_WABALTMON_10 => 131228,
            C2RustUnnamed_5::_NL_WABALTMON_9 => 131227,
            C2RustUnnamed_5::_NL_WABALTMON_8 => 131226,
            C2RustUnnamed_5::_NL_WABALTMON_7 => 131225,
            C2RustUnnamed_5::_NL_WABALTMON_6 => 131224,
            C2RustUnnamed_5::_NL_WABALTMON_5 => 131223,
            C2RustUnnamed_5::_NL_WABALTMON_4 => 131222,
            C2RustUnnamed_5::_NL_WABALTMON_3 => 131221,
            C2RustUnnamed_5::_NL_WABALTMON_2 => 131220,
            C2RustUnnamed_5::_NL_WABALTMON_1 => 131219,
            C2RustUnnamed_5::_NL_ABALTMON_12 => 131218,
            C2RustUnnamed_5::_NL_ABALTMON_11 => 131217,
            C2RustUnnamed_5::_NL_ABALTMON_10 => 131216,
            C2RustUnnamed_5::_NL_ABALTMON_9 => 131215,
            C2RustUnnamed_5::_NL_ABALTMON_8 => 131214,
            C2RustUnnamed_5::_NL_ABALTMON_7 => 131213,
            C2RustUnnamed_5::_NL_ABALTMON_6 => 131212,
            C2RustUnnamed_5::_NL_ABALTMON_5 => 131211,
            C2RustUnnamed_5::_NL_ABALTMON_4 => 131210,
            C2RustUnnamed_5::_NL_ABALTMON_3 => 131209,
            C2RustUnnamed_5::_NL_ABALTMON_2 => 131208,
            C2RustUnnamed_5::_NL_ABALTMON_1 => 131207,
            C2RustUnnamed_5::_NL_WALTMON_12 => 131206,
            C2RustUnnamed_5::_NL_WALTMON_11 => 131205,
            C2RustUnnamed_5::_NL_WALTMON_10 => 131204,
            C2RustUnnamed_5::_NL_WALTMON_9 => 131203,
            C2RustUnnamed_5::_NL_WALTMON_8 => 131202,
            C2RustUnnamed_5::_NL_WALTMON_7 => 131201,
            C2RustUnnamed_5::_NL_WALTMON_6 => 131200,
            C2RustUnnamed_5::_NL_WALTMON_5 => 131199,
            C2RustUnnamed_5::_NL_WALTMON_4 => 131198,
            C2RustUnnamed_5::_NL_WALTMON_3 => 131197,
            C2RustUnnamed_5::_NL_WALTMON_2 => 131196,
            C2RustUnnamed_5::_NL_WALTMON_1 => 131195,
            C2RustUnnamed_5::__ALTMON_12 => 131194,
            C2RustUnnamed_5::__ALTMON_11 => 131193,
            C2RustUnnamed_5::__ALTMON_10 => 131192,
            C2RustUnnamed_5::__ALTMON_9 => 131191,
            C2RustUnnamed_5::__ALTMON_8 => 131190,
            C2RustUnnamed_5::__ALTMON_7 => 131189,
            C2RustUnnamed_5::__ALTMON_6 => 131188,
            C2RustUnnamed_5::__ALTMON_5 => 131187,
            C2RustUnnamed_5::__ALTMON_4 => 131186,
            C2RustUnnamed_5::__ALTMON_3 => 131185,
            C2RustUnnamed_5::__ALTMON_2 => 131184,
            C2RustUnnamed_5::__ALTMON_1 => 131183,
            C2RustUnnamed_5::_NL_TIME_CODESET => 131182,
            C2RustUnnamed_5::_NL_W_DATE_FMT => 131181,
            C2RustUnnamed_5::_DATE_FMT => 131180,
            C2RustUnnamed_5::_NL_TIME_TIMEZONE => 131179,
            C2RustUnnamed_5::_NL_TIME_CAL_DIRECTION => 131178,
            C2RustUnnamed_5::_NL_TIME_FIRST_WORKDAY => 131177,
            C2RustUnnamed_5::_NL_TIME_FIRST_WEEKDAY => 131176,
            C2RustUnnamed_5::_NL_TIME_WEEK_1STWEEK => 131175,
            C2RustUnnamed_5::_NL_TIME_WEEK_1STDAY => 131174,
            C2RustUnnamed_5::_NL_TIME_WEEK_NDAYS => 131173,
            C2RustUnnamed_5::_NL_WERA_T_FMT => 131172,
            C2RustUnnamed_5::_NL_WERA_D_T_FMT => 131171,
            C2RustUnnamed_5::_NL_WALT_DIGITS => 131170,
            C2RustUnnamed_5::_NL_WERA_D_FMT => 131169,
            C2RustUnnamed_5::_NL_WERA_YEAR => 131168,
            C2RustUnnamed_5::_NL_WT_FMT_AMPM => 131167,
            C2RustUnnamed_5::_NL_WT_FMT => 131166,
            C2RustUnnamed_5::_NL_WD_FMT => 131165,
            C2RustUnnamed_5::_NL_WD_T_FMT => 131164,
            C2RustUnnamed_5::_NL_WPM_STR => 131163,
            C2RustUnnamed_5::_NL_WAM_STR => 131162,
            C2RustUnnamed_5::_NL_WMON_12 => 131161,
            C2RustUnnamed_5::_NL_WMON_11 => 131160,
            C2RustUnnamed_5::_NL_WMON_10 => 131159,
            C2RustUnnamed_5::_NL_WMON_9 => 131158,
            C2RustUnnamed_5::_NL_WMON_8 => 131157,
            C2RustUnnamed_5::_NL_WMON_7 => 131156,
            C2RustUnnamed_5::_NL_WMON_6 => 131155,
            C2RustUnnamed_5::_NL_WMON_5 => 131154,
            C2RustUnnamed_5::_NL_WMON_4 => 131153,
            C2RustUnnamed_5::_NL_WMON_3 => 131152,
            C2RustUnnamed_5::_NL_WMON_2 => 131151,
            C2RustUnnamed_5::_NL_WMON_1 => 131150,
            C2RustUnnamed_5::_NL_WABMON_12 => 131149,
            C2RustUnnamed_5::_NL_WABMON_11 => 131148,
            C2RustUnnamed_5::_NL_WABMON_10 => 131147,
            C2RustUnnamed_5::_NL_WABMON_9 => 131146,
            C2RustUnnamed_5::_NL_WABMON_8 => 131145,
            C2RustUnnamed_5::_NL_WABMON_7 => 131144,
            C2RustUnnamed_5::_NL_WABMON_6 => 131143,
            C2RustUnnamed_5::_NL_WABMON_5 => 131142,
            C2RustUnnamed_5::_NL_WABMON_4 => 131141,
            C2RustUnnamed_5::_NL_WABMON_3 => 131140,
            C2RustUnnamed_5::_NL_WABMON_2 => 131139,
            C2RustUnnamed_5::_NL_WABMON_1 => 131138,
            C2RustUnnamed_5::_NL_WDAY_7 => 131137,
            C2RustUnnamed_5::_NL_WDAY_6 => 131136,
            C2RustUnnamed_5::_NL_WDAY_5 => 131135,
            C2RustUnnamed_5::_NL_WDAY_4 => 131134,
            C2RustUnnamed_5::_NL_WDAY_3 => 131133,
            C2RustUnnamed_5::_NL_WDAY_2 => 131132,
            C2RustUnnamed_5::_NL_WDAY_1 => 131131,
            C2RustUnnamed_5::_NL_WABDAY_7 => 131130,
            C2RustUnnamed_5::_NL_WABDAY_6 => 131129,
            C2RustUnnamed_5::_NL_WABDAY_5 => 131128,
            C2RustUnnamed_5::_NL_WABDAY_4 => 131127,
            C2RustUnnamed_5::_NL_WABDAY_3 => 131126,
            C2RustUnnamed_5::_NL_WABDAY_2 => 131125,
            C2RustUnnamed_5::_NL_WABDAY_1 => 131124,
            C2RustUnnamed_5::_NL_TIME_ERA_ENTRIES => 131123,
            C2RustUnnamed_5::_NL_TIME_ERA_NUM_ENTRIES => 131122,
            C2RustUnnamed_5::ERA_T_FMT => 131121,
            C2RustUnnamed_5::ERA_D_T_FMT => 131120,
            C2RustUnnamed_5::ALT_DIGITS => 131119,
            C2RustUnnamed_5::ERA_D_FMT => 131118,
            C2RustUnnamed_5::__ERA_YEAR => 131117,
            C2RustUnnamed_5::ERA => 131116,
            C2RustUnnamed_5::T_FMT_AMPM => 131115,
            C2RustUnnamed_5::T_FMT => 131114,
            C2RustUnnamed_5::D_FMT => 131113,
            C2RustUnnamed_5::D_T_FMT => 131112,
            C2RustUnnamed_5::PM_STR => 131111,
            C2RustUnnamed_5::AM_STR => 131110,
            C2RustUnnamed_5::MON_12 => 131109,
            C2RustUnnamed_5::MON_11 => 131108,
            C2RustUnnamed_5::MON_10 => 131107,
            C2RustUnnamed_5::MON_9 => 131106,
            C2RustUnnamed_5::MON_8 => 131105,
            C2RustUnnamed_5::MON_7 => 131104,
            C2RustUnnamed_5::MON_6 => 131103,
            C2RustUnnamed_5::MON_5 => 131102,
            C2RustUnnamed_5::MON_4 => 131101,
            C2RustUnnamed_5::MON_3 => 131100,
            C2RustUnnamed_5::MON_2 => 131099,
            C2RustUnnamed_5::MON_1 => 131098,
            C2RustUnnamed_5::ABMON_12 => 131097,
            C2RustUnnamed_5::ABMON_11 => 131096,
            C2RustUnnamed_5::ABMON_10 => 131095,
            C2RustUnnamed_5::ABMON_9 => 131094,
            C2RustUnnamed_5::ABMON_8 => 131093,
            C2RustUnnamed_5::ABMON_7 => 131092,
            C2RustUnnamed_5::ABMON_6 => 131091,
            C2RustUnnamed_5::ABMON_5 => 131090,
            C2RustUnnamed_5::ABMON_4 => 131089,
            C2RustUnnamed_5::ABMON_3 => 131088,
            C2RustUnnamed_5::ABMON_2 => 131087,
            C2RustUnnamed_5::ABMON_1 => 131086,
            C2RustUnnamed_5::DAY_7 => 131085,
            C2RustUnnamed_5::DAY_6 => 131084,
            C2RustUnnamed_5::DAY_5 => 131083,
            C2RustUnnamed_5::DAY_4 => 131082,
            C2RustUnnamed_5::DAY_3 => 131081,
            C2RustUnnamed_5::DAY_2 => 131080,
            C2RustUnnamed_5::DAY_1 => 131079,
            C2RustUnnamed_5::ABDAY_7 => 131078,
            C2RustUnnamed_5::ABDAY_6 => 131077,
            C2RustUnnamed_5::ABDAY_5 => 131076,
            C2RustUnnamed_5::ABDAY_4 => 131075,
            C2RustUnnamed_5::ABDAY_3 => 131074,
            C2RustUnnamed_5::ABDAY_2 => 131073,
            C2RustUnnamed_5::ABDAY_1 => 131072,
        }
    }
}

pub type nl_item = libc::c_int;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_9 {
    filechr_not_unix = 1,
    filechr_not_vms = 2,
    filechr_not_windows = 4,
    filechr_control = 8,
}
impl C2RustUnnamed_9 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_9::filechr_not_unix => 1,
            C2RustUnnamed_9::filechr_not_vms => 2,
            C2RustUnnamed_9::filechr_not_windows => 4,
            C2RustUnnamed_9::filechr_control => 8,
        }
    }
}

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isascii(mut c: libc::c_int) -> bool {
    match c {
        32 | 7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16
        | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127
        | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_iscntrl(mut c: libc::c_int) -> bool {
    match c {
        7 | 8 | 12 | 10 | 13 | 9 | 11 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 14 | 15 | 16 | 17
        | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 127 => {
            return 1 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isupper(mut c: libc::c_int) -> bool {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn c_toupper(mut c: libc::c_int) -> libc::c_int {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 => {
            return c - 'a' as i32 + 'A' as i32;
        }
        _ => return c,
    };
}
#[inline]
unsafe extern "C" fn _unhex(mut c: libc::c_uchar) -> libc::c_uchar {
    return (if c as libc::c_int <= '9' as i32 {
        c as libc::c_int - '0' as i32
    } else if c as libc::c_int <= 'F' as i32 {
        c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    } else {
        c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }) as libc::c_uchar;
}
static mut supported_schemes: [scheme_data; 5] = [
    {
        let mut init = scheme_data {
            name: b"http\0" as *const u8 as *const libc::c_char,
            leading_string: b"http://\0" as *const u8 as *const libc::c_char,
            default_port: 80 as libc::c_int,
            flags: scm_has_query as libc::c_int | scm_has_fragment as libc::c_int,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: b"https\0" as *const u8 as *const libc::c_char,
            leading_string: b"https://\0" as *const u8 as *const libc::c_char,
            default_port: 443 as libc::c_int,
            flags: scm_has_query as libc::c_int | scm_has_fragment as libc::c_int,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: b"ftp\0" as *const u8 as *const libc::c_char,
            leading_string: b"ftp://\0" as *const u8 as *const libc::c_char,
            default_port: 21 as libc::c_int,
            flags: scm_has_params as libc::c_int | scm_has_fragment as libc::c_int,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: b"ftps\0" as *const u8 as *const libc::c_char,
            leading_string: b"ftps://\0" as *const u8 as *const libc::c_char,
            default_port: 21 as libc::c_int,
            flags: scm_has_params as libc::c_int | scm_has_fragment as libc::c_int,
        };
        init
    },
    {
        let mut init = scheme_data {
            name: 0 as *const libc::c_char,
            leading_string: 0 as *const libc::c_char,
            default_port: -(1 as libc::c_int),
            flags: 0 as libc::c_int,
        };
        init
    },
];
static mut urlchr_table: [libc::c_uchar; 256] = [
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_reserved as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    (urlchr_reserved as libc::c_int | urlchr_unsafe as libc::c_int) as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
    urlchr_unsafe as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn url_unescape_1(mut s: *mut libc::c_char, mut mask: libc::c_uchar) {
    let mut current_block: u64;
    let mut t: *mut libc::c_uchar = s as *mut libc::c_uchar;
    let mut h: *mut libc::c_uchar = s as *mut libc::c_uchar;
    while *h != 0 {
        if *h as libc::c_int != '%' as i32 {
            current_block = 18150253981776513775;
        } else {
            let mut c: libc::c_uchar = 0;
            if *h.offset(1 as libc::c_int as isize) == 0
                || *h.offset(2 as libc::c_int as isize) == 0
                || !(c_isxdigit(*h.offset(1 as libc::c_int as isize) as libc::c_int)
                    as libc::c_int != 0
                    && c_isxdigit(*h.offset(2 as libc::c_int as isize) as libc::c_int)
                        as libc::c_int != 0)
            {
                current_block = 18150253981776513775;
            } else {
                c = (((_unhex(*h.offset(1 as libc::c_int as isize)) as libc::c_int)
                    << 4 as libc::c_int)
                    + _unhex(*h.offset(2 as libc::c_int as isize)) as libc::c_int)
                    as libc::c_uchar;
                if urlchr_table[c as usize] as libc::c_int & mask as libc::c_int != 0 {
                    current_block = 18150253981776513775;
                } else if c as libc::c_int == '\0' as i32 {
                    current_block = 18150253981776513775;
                } else {
                    *t = c;
                    h = h.offset(2 as libc::c_int as isize);
                    current_block = 16668937799742929182;
                }
            }
        }
        match current_block {
            18150253981776513775 => {
                *t = *h;
            }
            _ => {}
        }
        h = h.offset(1);
        h;
        t = t.offset(1);
        t;
    }
    *t = '\0' as i32 as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn url_unescape(mut s: *mut libc::c_char) {
    url_unescape_1(s, 0 as libc::c_int as libc::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn url_unescape_except_reserved(mut s: *mut libc::c_char) {
    url_unescape_1(s, urlchr_reserved as libc::c_int as libc::c_uchar);
}
unsafe extern "C" fn url_escape_1(
    mut s: *const libc::c_char,
    mut mask: libc::c_uchar,
    mut allow_passthrough: bool,
) -> *mut libc::c_char {
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newlen: libc::c_int = 0;
    let mut addition: libc::c_int = 0 as libc::c_int;
    p1 = s;
    while *p1 != 0 {
        if urlchr_table[*p1 as libc::c_uchar as usize] as libc::c_int
            & mask as libc::c_int != 0
        {
            addition += 2 as libc::c_int;
        }
        p1 = p1.offset(1);
        p1;
    }
    if addition == 0 {
        return if allow_passthrough as libc::c_int != 0 {
            s as *mut libc::c_char
        } else {
            xstrdup(s)
        };
    }
    newlen = (p1.offset_from(s) as libc::c_long + addition as libc::c_long)
        as libc::c_int;
    newstr = xmalloc((newlen + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    p1 = s;
    p2 = newstr;
    while *p1 != 0 {
        if urlchr_table[*p1 as libc::c_uchar as usize] as libc::c_int
            & mask as libc::c_int != 0
        {
            let fresh0 = p1;
            p1 = p1.offset(1);
            let mut c: libc::c_uchar = *fresh0 as libc::c_uchar;
            let fresh1 = p2;
            p2 = p2.offset(1);
            *fresh1 = '%' as i32 as libc::c_char;
            let fresh2 = p2;
            p2 = p2.offset(1);
            *fresh2 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int >> 4 as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
            let fresh3 = p2;
            p2 = p2.offset(1);
            *fresh3 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int & 0xf as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
        } else {
            let fresh4 = p1;
            p1 = p1.offset(1);
            let fresh5 = p2;
            p2 = p2.offset(1);
            *fresh5 = *fresh4;
        }
    }
    *p2 = '\0' as i32 as libc::c_char;
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn url_escape(mut s: *const libc::c_char) -> *mut libc::c_char {
    return url_escape_1(
        s,
        urlchr_unsafe as libc::c_int as libc::c_uchar,
        0 as libc::c_int != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn url_escape_unsafe_and_reserved(
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    return url_escape_1(
        s,
        (urlchr_unsafe as libc::c_int | urlchr_reserved as libc::c_int) as libc::c_uchar,
        0 as libc::c_int != 0,
    );
}
unsafe extern "C" fn url_escape_allow_passthrough(
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    return url_escape_1(
        s,
        urlchr_unsafe as libc::c_int as libc::c_uchar,
        1 as libc::c_int != 0,
    );
}
#[inline]
unsafe extern "C" fn char_needs_escaping(mut p: *const libc::c_char) -> bool {
    if *p as libc::c_int == '%' as i32 {
        if c_isxdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_int
            != 0
            && c_isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
        {
            return 0 as libc::c_int != 0
        } else {
            return 1 as libc::c_int != 0
        }
    } else if urlchr_table[*p as libc::c_uchar as usize] as libc::c_int
        & urlchr_unsafe as libc::c_int != 0
        && urlchr_table[*p as libc::c_uchar as usize] as libc::c_int
            & urlchr_reserved as libc::c_int == 0
    {
        return 1 as libc::c_int != 0
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn reencode_escapes(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldlen: libc::c_int = 0;
    let mut newlen: libc::c_int = 0;
    let mut encode_count: libc::c_int = 0 as libc::c_int;
    p1 = s;
    while *p1 != 0 {
        if char_needs_escaping(p1) {
            encode_count += 1;
            encode_count;
        }
        p1 = p1.offset(1);
        p1;
    }
    if encode_count == 0 {
        return s as *mut libc::c_char;
    }
    oldlen = p1.offset_from(s) as libc::c_long as libc::c_int;
    newlen = oldlen + 2 as libc::c_int * encode_count;
    newstr = xmalloc((newlen + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    p1 = s;
    p2 = newstr;
    while *p1 != 0 {
        if char_needs_escaping(p1) {
            let fresh6 = p1;
            p1 = p1.offset(1);
            let mut c: libc::c_uchar = *fresh6 as libc::c_uchar;
            let fresh7 = p2;
            p2 = p2.offset(1);
            *fresh7 = '%' as i32 as libc::c_char;
            let fresh8 = p2;
            p2 = p2.offset(1);
            *fresh8 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int >> 4 as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
            let fresh9 = p2;
            p2 = p2.offset(1);
            *fresh9 = ((*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"0123456789ABCDEF\0"))[(c as libc::c_int & 0xf as libc::c_int) as usize]
                as libc::c_int + 0 as libc::c_int) as libc::c_char;
        } else {
            let fresh10 = p1;
            p1 = p1.offset(1);
            let fresh11 = p2;
            p2 = p2.offset(1);
            *fresh11 = *fresh10;
        }
    }
    *p2 = '\0' as i32 as libc::c_char;
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn url_scheme(mut url: *const libc::c_char) -> url_scheme {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(supported_schemes[i as usize].leading_string).is_null() {
        if 0 as libc::c_int
            == c_strncasecmp(
                url,
                supported_schemes[i as usize].leading_string,
                strlen(supported_schemes[i as usize].leading_string),
            )
        {
            if supported_schemes[i as usize].flags & scm_disabled as libc::c_int == 0 {
                return i as url_scheme
            } else {
                return SCHEME_INVALID
            }
        }
        i += 1;
        i;
    }
    return SCHEME_INVALID;
}
#[no_mangle]
pub unsafe extern "C" fn url_has_scheme(mut url: *const libc::c_char) -> bool {
    let mut p: *const libc::c_char = url;
    if *p == 0
        || !(c_isalnum(*p as libc::c_int) as libc::c_int != 0
            || *p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32)
    {
        return 0 as libc::c_int != 0;
    }
    p = p.offset(1);
    p;
    while *p as libc::c_int != 0
        && (c_isalnum(*p as libc::c_int) as libc::c_int != 0
            || *p as libc::c_int == '-' as i32 || *p as libc::c_int == '+' as i32)
    {
        p = p.offset(1);
        p;
    }
    return *p as libc::c_int == ':' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn url_valid_scheme(mut url: *const libc::c_char) -> bool {
    let mut scheme: url_scheme = url_scheme(url);
    return scheme as libc::c_uint != SCHEME_INVALID as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn scheme_default_port(mut scheme: url_scheme) -> libc::c_int {
    return supported_schemes[scheme as usize].default_port;
}
#[no_mangle]
pub unsafe extern "C" fn scheme_disable(mut scheme: url_scheme) {
    supported_schemes[scheme as usize].flags |= scm_disabled as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scheme_leading_string(
    mut scheme: url_scheme,
) -> *const libc::c_char {
    return supported_schemes[scheme as usize].leading_string;
}
unsafe extern "C" fn url_skip_credentials(
    mut url: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = strpbrk(
        url,
        b"@/?#;\0" as *const u8 as *const libc::c_char,
    ) as *const libc::c_char;
    if p.is_null() || *p as libc::c_int != '@' as i32 {
        return url;
    }
    return p.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn parse_credentials(
    mut beg: *const libc::c_char,
    mut end: *const libc::c_char,
    mut user: *mut *mut libc::c_char,
    mut passwd: *mut *mut libc::c_char,
) -> bool {
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut userend: *const libc::c_char = 0 as *const libc::c_char;
    if beg == end {
        return 0 as libc::c_int != 0;
    }
    colon = memchr(
        beg as *const libc::c_void,
        ':' as i32,
        end.offset_from(beg) as libc::c_long as libc::c_ulong,
    ) as *mut libc::c_char;
    if colon == beg as *mut libc::c_char {
        return 0 as libc::c_int != 0;
    }
    if !colon.is_null() {
        *passwd = strdupdelim(colon.offset(1 as libc::c_int as isize), end);
        userend = colon;
        url_unescape(*passwd);
    } else {
        *passwd = 0 as *mut libc::c_char;
        userend = end;
    }
    *user = strdupdelim(beg, userend);
    url_unescape(*user);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn rewrite_shorthand_url(
    mut url: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if url_scheme(url) as libc::c_uint != SCHEME_INVALID as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_char;
    }
    p = strpbrk(url, b":/\0" as *const u8 as *const libc::c_char);
    if p == url {
        return 0 as *mut libc::c_char;
    }
    if !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int == ':' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        return 0 as *mut libc::c_char;
    }
    let mut current_block_9: u64;
    if !p.is_null() && *p as libc::c_int == ':' as i32 {
        let mut digits: libc::c_int = strspn(
            p.offset(1 as libc::c_int as isize),
            b"0123456789\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        if digits != 0
            && (*p.offset((1 as libc::c_int + digits) as isize) as libc::c_int
                == '/' as i32
                || *p.offset((1 as libc::c_int + digits) as isize) as libc::c_int
                    == '\0' as i32)
        {
            current_block_9 = 9810375517332381765;
        } else {
            ret = aprintf(b"ftp://%s\0" as *const u8 as *const libc::c_char, url);
            if !ret.is_null() {
                *ret
                    .offset(
                        (6 as libc::c_int as libc::c_long
                            + p.offset_from(url) as libc::c_long) as isize,
                    ) = '/' as i32 as libc::c_char;
            }
            current_block_9 = 7746791466490516765;
        }
    } else {
        current_block_9 = 9810375517332381765;
    }
    match current_block_9 {
        9810375517332381765 => {
            ret = aprintf(b"http://%s\0" as *const u8 as *const libc::c_char, url);
        }
        _ => {}
    }
    return ret;
}
#[inline]
unsafe extern "C" fn strpbrk_or_eos(
    mut s: *const libc::c_char,
    mut accept: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = strpbrk(s, accept);
    if p.is_null() {
        p = strchr(s, '\0' as i32);
    }
    return p;
}
unsafe extern "C" fn lowercase_str(mut str: *mut libc::c_char) -> bool {
    let mut changed: bool = 0 as libc::c_int != 0;
    while *str != 0 {
        if c_isupper(*str as libc::c_int) {
            changed = 1 as libc::c_int != 0;
            *str = c_tolower(*str as libc::c_int) as libc::c_char;
        }
        str = str.offset(1);
        str;
    }
    return changed;
}
unsafe extern "C" fn init_seps(mut scheme: url_scheme) -> *const libc::c_char {
    static mut seps: [libc::c_char; 8] = unsafe {
        *::core::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b":/\0\0\0\0\0\0")
    };
    let mut p: *mut libc::c_char = seps.as_mut_ptr().offset(2 as libc::c_int as isize);
    let mut flags: libc::c_int = supported_schemes[scheme as usize].flags;
    if flags & scm_has_params as libc::c_int != 0 {
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = ';' as i32 as libc::c_char;
    }
    if flags & scm_has_query as libc::c_int != 0 {
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = '?' as i32 as libc::c_char;
    }
    if flags & scm_has_fragment as libc::c_int != 0 {
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = '#' as i32 as libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    return seps.as_mut_ptr();
}
static mut parse_errors: [*const libc::c_char; 11] = [
    b"No error\0" as *const u8 as *const libc::c_char,
    b"Unsupported scheme\0" as *const u8 as *const libc::c_char,
    b"HTTPS support not compiled in\0" as *const u8 as *const libc::c_char,
    b"FTPS support not compiled in\0" as *const u8 as *const libc::c_char,
    b"Scheme missing\0" as *const u8 as *const libc::c_char,
    b"Invalid host name\0" as *const u8 as *const libc::c_char,
    b"Bad port number\0" as *const u8 as *const libc::c_char,
    b"Invalid user name\0" as *const u8 as *const libc::c_char,
    b"Unterminated IPv6 numeric address\0" as *const u8 as *const libc::c_char,
    b"IPv6 addresses not supported\0" as *const u8 as *const libc::c_char,
    b"Invalid IPv6 numeric address\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn url_parse(
    mut url: *const libc::c_char,
    mut error: *mut libc::c_int,
    mut iri: *mut iri,
    mut percent_encode: bool,
) -> *mut url {
    let mut current_block: u64;
    let mut u: *mut url = 0 as *mut url;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_modified: bool = false;
    let mut host_modified: bool = false;
    let mut scheme: url_scheme = SCHEME_HTTP;
    let mut seps: *const libc::c_char = 0 as *const libc::c_char;
    let mut uname_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut uname_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut host_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut host_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut path_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut params_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut params_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut query_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut query_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut fragment_b: *const libc::c_char = 0 as *const libc::c_char;
    let mut fragment_e: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: libc::c_int = 0;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url_encoded: *const libc::c_char = 0 as *const libc::c_char;
    let mut error_code: libc::c_int = 0;
    scheme = url_scheme(url);
    if scheme as libc::c_uint == SCHEME_INVALID as libc::c_int as libc::c_uint {
        if !url_has_scheme(url) {
            error_code = PE_MISSING_SCHEME as libc::c_int;
        } else if c_strncasecmp(
            url,
            b"https:\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        ) == 0
        {
            error_code = PE_UNSUPPORTED_SCHEME_HTTPS as libc::c_int;
        } else if c_strncasecmp(
            url,
            b"ftps:\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ) == 0
        {
            error_code = PE_UNSUPPORTED_SCHEME_FTPS as libc::c_int;
        } else {
            error_code = PE_UNSUPPORTED_SCHEME as libc::c_int;
        }
    } else {
        url_encoded = url;
        if !iri.is_null() && (*iri).utf8_encode as libc::c_int != 0 {
            let mut new_url: *mut libc::c_char = 0 as *mut libc::c_char;
            (*iri)
                .utf8_encode = remote_to_utf8(
                iri,
                if !((*iri).orig_url).is_null() { (*iri).orig_url } else { url },
                &mut new_url,
            );
            if !(*iri).utf8_encode {
                new_url = 0 as *mut libc::c_char;
            } else {
                rpl_free((*iri).orig_url as *mut libc::c_void);
                (*iri).orig_url = 0 as *mut libc::c_char;
                (*iri).orig_url = xstrdup(url);
                url_encoded = reencode_escapes(new_url);
                if url_encoded != new_url {
                    rpl_free(new_url as *mut libc::c_void);
                    new_url = 0 as *mut libc::c_char;
                }
                percent_encode = 0 as libc::c_int != 0;
            }
        }
        if percent_encode {
            url_encoded = reencode_escapes(url);
        }
        p = url_encoded;
        p = p.offset(strlen(supported_schemes[scheme as usize].leading_string) as isize);
        uname_b = p;
        p = url_skip_credentials(p);
        uname_e = p;
        path_e = 0 as *const libc::c_char;
        path_b = path_e;
        params_e = 0 as *const libc::c_char;
        params_b = params_e;
        query_e = 0 as *const libc::c_char;
        query_b = query_e;
        fragment_e = 0 as *const libc::c_char;
        fragment_b = fragment_e;
        seps = init_seps(scheme);
        host_b = p;
        if *p as libc::c_int == '[' as i32 {
            host_b = p.offset(1 as libc::c_int as isize);
            host_e = strchr(host_b, ']' as i32);
            if host_e.is_null() {
                error_code = PE_UNTERMINATED_IPV6_ADDRESS as libc::c_int;
                current_block = 11473658817087337075;
            } else if !is_valid_ipv6_address(host_b, host_e) {
                error_code = PE_INVALID_IPV6_ADDRESS as libc::c_int;
                current_block = 11473658817087337075;
            } else {
                p = host_e.offset(1 as libc::c_int as isize);
                if (strchr(seps, *p as libc::c_int)).is_null() {
                    error_code = PE_INVALID_HOST_NAME as libc::c_int;
                    current_block = 11473658817087337075;
                } else {
                    current_block = 1924505913685386279;
                }
            }
        } else {
            p = strpbrk_or_eos(p, seps);
            host_e = p;
            current_block = 1924505913685386279;
        }
        match current_block {
            11473658817087337075 => {}
            _ => {
                seps = seps.offset(1);
                seps;
                if host_b == host_e {
                    error_code = PE_INVALID_HOST_NAME as libc::c_int;
                } else {
                    port = scheme_default_port(scheme);
                    if *p as libc::c_int == ':' as i32 {
                        let mut port_b: *const libc::c_char = 0 as *const libc::c_char;
                        let mut port_e: *const libc::c_char = 0 as *const libc::c_char;
                        let mut pp: *const libc::c_char = 0 as *const libc::c_char;
                        p = p.offset(1);
                        p;
                        port_b = p;
                        p = strpbrk_or_eos(p, seps);
                        port_e = p;
                        if port_b != port_e {
                            port = 0 as libc::c_int;
                            pp = port_b;
                            loop {
                                if !(pp < port_e) {
                                    current_block = 3580086814630675314;
                                    break;
                                }
                                if !c_isdigit(*pp as libc::c_int) {
                                    error_code = PE_BAD_PORT_NUMBER as libc::c_int;
                                    current_block = 11473658817087337075;
                                    break;
                                } else {
                                    port = 10 as libc::c_int * port
                                        + (*pp as libc::c_int - '0' as i32);
                                    if port > 0xffff as libc::c_int {
                                        error_code = PE_BAD_PORT_NUMBER as libc::c_int;
                                        current_block = 11473658817087337075;
                                        break;
                                    } else {
                                        pp = pp.offset(1);
                                        pp;
                                    }
                                }
                            }
                        } else {
                            current_block = 3580086814630675314;
                        }
                    } else {
                        current_block = 3580086814630675314;
                    }
                    match current_block {
                        11473658817087337075 => {}
                        _ => {
                            seps = seps.offset(1);
                            seps;
                            if *p as libc::c_int == '/' as i32 {
                                p = p.offset(1);
                                path_b = p;
                                p = strpbrk_or_eos(p, seps);
                                path_e = p;
                            }
                            seps = seps.offset(1);
                            seps;
                            if supported_schemes[scheme as usize].flags
                                & scm_has_params as libc::c_int != 0
                            {
                                if *p as libc::c_int == ';' as i32 {
                                    p = p.offset(1);
                                    params_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    params_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if supported_schemes[scheme as usize].flags
                                & scm_has_query as libc::c_int != 0
                            {
                                if *p as libc::c_int == '?' as i32 {
                                    p = p.offset(1);
                                    query_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    query_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if supported_schemes[scheme as usize].flags
                                & scm_has_fragment as libc::c_int != 0
                            {
                                if *p as libc::c_int == '#' as i32 {
                                    p = p.offset(1);
                                    fragment_b = p;
                                    p = strpbrk_or_eos(p, seps);
                                    fragment_e = p;
                                }
                                seps = seps.offset(1);
                                seps;
                            }
                            if uname_b != uname_e {
                                if !parse_credentials(
                                    uname_b,
                                    uname_e.offset(-(1 as libc::c_int as isize)),
                                    &mut user,
                                    &mut passwd,
                                ) {
                                    error_code = PE_INVALID_USER_NAME as libc::c_int;
                                    current_block = 11473658817087337075;
                                } else {
                                    current_block = 200744462051969938;
                                }
                            } else {
                                current_block = 200744462051969938;
                            }
                            match current_block {
                                11473658817087337075 => {}
                                _ => {
                                    u = xcalloc(
                                        1 as libc::c_int as size_t,
                                        ::core::mem::size_of::<url>() as libc::c_ulong,
                                    ) as *mut url;
                                    (*u).scheme = scheme;
                                    (*u).host = strdupdelim(host_b, host_e);
                                    (*u).port = port;
                                    (*u).user = user;
                                    (*u).passwd = passwd;
                                    (*u).path = strdupdelim(path_b, path_e);
                                    path_modified = path_simplify(scheme, (*u).path);
                                    split_path((*u).path, &mut (*u).dir, &mut (*u).file);
                                    host_modified = lowercase_str((*u).host);
                                    if !(strchr((*u).host, '%' as i32)).is_null() {
                                        url_unescape((*u).host);
                                        host_modified = 1 as libc::c_int != 0;
                                        p = (*u).host;
                                        loop {
                                            if !(*p != 0) {
                                                current_block = 5700653730392116747;
                                                break;
                                            }
                                            if c_iscntrl(*p as libc::c_int) {
                                                url_free(u);
                                                error_code = PE_INVALID_HOST_NAME as libc::c_int;
                                                current_block = 11473658817087337075;
                                                break;
                                            } else {
                                                p = p.offset(1);
                                                p;
                                            }
                                        }
                                        match current_block {
                                            11473658817087337075 => {}
                                            _ => {
                                                if opt.enable_iri as libc::c_int != 0 && !iri.is_null() {
                                                    let mut new: *mut libc::c_char = idn_encode(iri, (*u).host);
                                                    if !new.is_null() {
                                                        rpl_free((*u).host as *mut libc::c_void);
                                                        (*u).host = 0 as *mut libc::c_char;
                                                        (*u).host = new;
                                                        host_modified = 1 as libc::c_int != 0;
                                                    }
                                                }
                                                current_block = 9775647934248138666;
                                            }
                                        }
                                    } else {
                                        current_block = 9775647934248138666;
                                    }
                                    match current_block {
                                        11473658817087337075 => {}
                                        _ => {
                                            if !params_b.is_null() {
                                                (*u).params = strdupdelim(params_b, params_e);
                                            }
                                            if !query_b.is_null() {
                                                (*u).query = strdupdelim(query_b, query_e);
                                            }
                                            if !fragment_b.is_null() {
                                                (*u).fragment = strdupdelim(fragment_b, fragment_e);
                                            }
                                            if opt.enable_iri as libc::c_int != 0
                                                || path_modified as libc::c_int != 0
                                                || !((*u).fragment).is_null()
                                                || host_modified as libc::c_int != 0 || path_b == path_e
                                            {
                                                (*u).url = url_string(u, URL_AUTH_SHOW);
                                                if url_encoded != url {
                                                    rpl_free(url_encoded as *mut libc::c_void);
                                                    url_encoded = 0 as *const libc::c_char;
                                                }
                                            } else if url_encoded == url {
                                                (*u).url = xstrdup(url);
                                            } else {
                                                (*u).url = url_encoded as *mut libc::c_char;
                                            }
                                            return u;
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
    if !url_encoded.is_null() && url_encoded != url {
        rpl_free(url_encoded as *mut libc::c_void);
        url_encoded = 0 as *const libc::c_char;
    }
    if !error.is_null() {
        *error = error_code;
    }
    return 0 as *mut url;
}
#[no_mangle]
pub unsafe extern "C" fn url_error(mut error_code: libc::c_int) -> *const libc::c_char {
    if error_code >= 0 as libc::c_int
        && error_code
            < (::core::mem::size_of::<[*const libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ) as libc::c_int
    {
        return dcgettext(
            0 as *const libc::c_char,
            parse_errors[error_code as usize],
            5 as libc::c_int,
        );
    }
    return b"\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn split_path(
    mut path: *const libc::c_char,
    mut dir: *mut *mut libc::c_char,
    mut file: *mut *mut libc::c_char,
) {
    let mut last_slash: *mut libc::c_char = strrchr(path, '/' as i32);
    if last_slash.is_null() {
        *dir = xstrdup(b"\0" as *const u8 as *const libc::c_char);
        *file = xstrdup(path);
    } else {
        *dir = strdupdelim(path, last_slash);
        *file = xstrdup(last_slash.offset(1 as libc::c_int as isize));
    }
    url_unescape(*dir);
    url_unescape(*file);
}
unsafe extern "C" fn full_path_length(mut url: *const url) -> libc::c_int {
    let mut len: libc::c_int = 0 as libc::c_int;
    if !((*url).path).is_null() {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen((*url).path)),
            ) as libc::c_int as libc::c_int;
    }
    if !((*url).params).is_null() {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen((*url).params)),
            ) as libc::c_int as libc::c_int;
    }
    if !((*url).query).is_null() {
        len = (len as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen((*url).query)),
            ) as libc::c_int as libc::c_int;
    }
    return len;
}
unsafe extern "C" fn full_path_write(
    mut url: *const url,
    mut where_0: *mut libc::c_char,
) {
    let mut f_el: *mut libc::c_char = (*url).path;
    if !f_el.is_null() {
        let mut l: libc::c_int = strlen(f_el) as libc::c_int;
        let fresh15 = where_0;
        where_0 = where_0.offset(1);
        *fresh15 = '/' as i32 as libc::c_char;
        memcpy(
            where_0 as *mut libc::c_void,
            f_el as *const libc::c_void,
            l as libc::c_ulong,
        );
        where_0 = where_0.offset(l as isize);
    }
    let mut f_el_0: *mut libc::c_char = (*url).params;
    if !f_el_0.is_null() {
        let mut l_0: libc::c_int = strlen(f_el_0) as libc::c_int;
        let fresh16 = where_0;
        where_0 = where_0.offset(1);
        *fresh16 = ';' as i32 as libc::c_char;
        memcpy(
            where_0 as *mut libc::c_void,
            f_el_0 as *const libc::c_void,
            l_0 as libc::c_ulong,
        );
        where_0 = where_0.offset(l_0 as isize);
    }
    let mut f_el_1: *mut libc::c_char = (*url).query;
    if !f_el_1.is_null() {
        let mut l_1: libc::c_int = strlen(f_el_1) as libc::c_int;
        let fresh17 = where_0;
        where_0 = where_0.offset(1);
        *fresh17 = '?' as i32 as libc::c_char;
        memcpy(
            where_0 as *mut libc::c_void,
            f_el_1 as *const libc::c_void,
            l_1 as libc::c_ulong,
        );
        where_0 = where_0.offset(l_1 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn url_full_path(mut url: *const url) -> *mut libc::c_char {
    let mut length: libc::c_int = full_path_length(url);
    let mut full_path: *mut libc::c_char = xmalloc((length + 1 as libc::c_int) as size_t)
        as *mut libc::c_char;
    full_path_write(url, full_path);
    *full_path.offset(length as isize) = '\0' as i32 as libc::c_char;
    return full_path;
}
unsafe extern "C" fn unescape_single_char(
    mut str: *mut libc::c_char,
    mut chr: libc::c_char,
) {
    let c1: libc::c_char = ((*::core::mem::transmute::<
        &[u8; 17],
        &[libc::c_char; 17],
    >(b"0123456789ABCDEF\0"))[(chr as libc::c_int >> 4 as libc::c_int) as usize]
        as libc::c_int + 0 as libc::c_int) as libc::c_char;
    let c2: libc::c_char = ((*::core::mem::transmute::<
        &[u8; 17],
        &[libc::c_char; 17],
    >(b"0123456789ABCDEF\0"))[(chr as libc::c_int & 0xf as libc::c_int) as usize]
        as libc::c_int + 0 as libc::c_int) as libc::c_char;
    let mut h: *mut libc::c_char = str;
    let mut t: *mut libc::c_char = str;
    while *h != 0 {
        if *h.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
            && *h.offset(1 as libc::c_int as isize) as libc::c_int == c1 as libc::c_int
            && *h.offset(2 as libc::c_int as isize) as libc::c_int == c2 as libc::c_int
        {
            *t = chr;
            h = h.offset(2 as libc::c_int as isize);
        } else {
            *t = *h;
        }
        h = h.offset(1);
        h;
        t = t.offset(1);
        t;
    }
    *t = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn url_escape_dir(mut dir: *const libc::c_char) -> *mut libc::c_char {
    let mut newdir: *mut libc::c_char = url_escape_1(
        dir,
        (urlchr_unsafe as libc::c_int | urlchr_reserved as libc::c_int) as libc::c_uchar,
        1 as libc::c_int != 0,
    );
    if newdir == dir as *mut libc::c_char {
        return dir as *mut libc::c_char;
    }
    unescape_single_char(newdir, '/' as i32 as libc::c_char);
    return newdir;
}
unsafe extern "C" fn sync_path(mut u: *mut url) {
    let mut newpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut efile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut edir: *mut libc::c_char = 0 as *mut libc::c_char;
    rpl_free((*u).path as *mut libc::c_void);
    (*u).path = 0 as *mut libc::c_char;
    edir = url_escape_dir((*u).dir);
    efile = url_escape_1(
        (*u).file,
        (urlchr_unsafe as libc::c_int | urlchr_reserved as libc::c_int) as libc::c_uchar,
        1 as libc::c_int != 0,
    );
    if *edir == 0 {
        newpath = xstrdup(efile);
    } else {
        let mut dirlen: libc::c_int = strlen(edir) as libc::c_int;
        let mut filelen: libc::c_int = strlen(efile) as libc::c_int;
        newpath = xmalloc(
            (dirlen + 1 as libc::c_int + filelen + 1 as libc::c_int) as size_t,
        ) as *mut libc::c_char;
        let mut p: *mut libc::c_char = newpath;
        memcpy(
            p as *mut libc::c_void,
            edir as *const libc::c_void,
            dirlen as libc::c_ulong,
        );
        p = p.offset(dirlen as isize);
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = '/' as i32 as libc::c_char;
        memcpy(
            p as *mut libc::c_void,
            efile as *const libc::c_void,
            filelen as libc::c_ulong,
        );
        p = p.offset(filelen as isize);
        *p = '\0' as i32 as libc::c_char;
    }
    (*u).path = newpath;
    if edir != (*u).dir {
        rpl_free(edir as *mut libc::c_void);
        edir = 0 as *mut libc::c_char;
    }
    if efile != (*u).file {
        rpl_free(efile as *mut libc::c_void);
        efile = 0 as *mut libc::c_char;
    }
    rpl_free((*u).url as *mut libc::c_void);
    (*u).url = 0 as *mut libc::c_char;
    (*u).url = url_string(u, URL_AUTH_SHOW);
}
#[no_mangle]
pub unsafe extern "C" fn url_set_dir(
    mut url: *mut url,
    mut newdir: *const libc::c_char,
) {
    rpl_free((*url).dir as *mut libc::c_void);
    (*url).dir = 0 as *mut libc::c_char;
    (*url).dir = xstrdup(newdir);
    sync_path(url);
}
#[no_mangle]
pub unsafe extern "C" fn url_set_file(
    mut url: *mut url,
    mut newfile: *const libc::c_char,
) {
    rpl_free((*url).file as *mut libc::c_void);
    (*url).file = 0 as *mut libc::c_char;
    (*url).file = xstrdup(newfile);
    sync_path(url);
}
#[no_mangle]
pub unsafe extern "C" fn url_free(mut url: *mut url) {
    if !url.is_null() {
        rpl_free((*url).host as *mut libc::c_void);
        (*url).host = 0 as *mut libc::c_char;
        rpl_free((*url).path as *mut libc::c_void);
        (*url).path = 0 as *mut libc::c_char;
        rpl_free((*url).url as *mut libc::c_void);
        (*url).url = 0 as *mut libc::c_char;
        rpl_free((*url).params as *mut libc::c_void);
        (*url).params = 0 as *mut libc::c_char;
        rpl_free((*url).query as *mut libc::c_void);
        (*url).query = 0 as *mut libc::c_char;
        rpl_free((*url).fragment as *mut libc::c_void);
        (*url).fragment = 0 as *mut libc::c_char;
        rpl_free((*url).user as *mut libc::c_void);
        (*url).user = 0 as *mut libc::c_char;
        rpl_free((*url).passwd as *mut libc::c_void);
        (*url).passwd = 0 as *mut libc::c_char;
        rpl_free((*url).dir as *mut libc::c_void);
        (*url).dir = 0 as *mut libc::c_char;
        rpl_free((*url).file as *mut libc::c_void);
        (*url).file = 0 as *mut libc::c_char;
        rpl_free(url as *mut libc::c_void);
        url = 0 as *mut url;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mkalldirs(mut path: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut res: libc::c_int = 0;
    p = strrchr(path, '/' as i32);
    p = if p.is_null() { path } else { p };
    if p == path && *p as libc::c_int != '/' as i32 {
        return 0 as libc::c_int;
    }
    t = strdupdelim(path, p);
    if stat(t, &mut st) == 0 as libc::c_int {
        if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            rpl_free(t as *mut libc::c_void);
            t = 0 as *mut libc::c_char;
            return 0 as libc::c_int;
        } else {
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Removing %s because of directory danger!\n\0" as *const u8
                        as *const libc::c_char,
                    t,
                );
            }
            if unlink(t) != 0 {
                logprintf(
                    LOG_NOTQUIET,
                    b"Failed to unlink %s (%d): %s\n\0" as *const u8
                        as *const libc::c_char,
                    t,
                    *__errno_location(),
                    strerror(*__errno_location()),
                );
            }
        }
    }
    res = make_directory(t);
    if res != 0 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            t,
            strerror(*__errno_location()),
        );
    }
    rpl_free(t as *mut libc::c_void);
    t = 0 as *mut libc::c_char;
    return res;
}
unsafe extern "C" fn append_null(mut dest: *mut growable) {
    let mut G_: *mut growable = dest;
    let mut DR_needed_size: libc::c_long = ((*G_).tail + 1 as libc::c_int)
        as libc::c_long;
    let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
    while ((*G_).size as libc::c_long) < DR_needed_size {
        DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
        if DR_newsize < 16 as libc::c_int as libc::c_long {
            DR_newsize = 16 as libc::c_int as libc::c_long;
        }
        (*G_).size = DR_newsize as libc::c_int;
    }
    if DR_newsize != 0 {
        (*G_)
            .base = xrealloc(
            (*G_).base as *mut libc::c_void,
            (DR_newsize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    *((*dest).base).offset((*dest).tail as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn append_char(mut ch: libc::c_char, mut dest: *mut growable) {
    if ch != 0 {
        let mut G_: *mut growable = dest;
        let mut DR_needed_size: libc::c_long = ((*G_).tail + 1 as libc::c_int)
            as libc::c_long;
        let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
        while ((*G_).size as libc::c_long) < DR_needed_size {
            DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
            if DR_newsize < 16 as libc::c_int as libc::c_long {
                DR_newsize = 16 as libc::c_int as libc::c_long;
            }
            (*G_).size = DR_newsize as libc::c_int;
        }
        if DR_newsize != 0 {
            (*G_)
                .base = xrealloc(
                (*G_).base as *mut libc::c_void,
                (DR_newsize as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
        }
        *((*dest).base).offset((*dest).tail as isize) = ch;
        (*dest).tail += 1 as libc::c_int;
    }
    append_null(dest);
}
unsafe extern "C" fn append_string(
    mut str: *const libc::c_char,
    mut dest: *mut growable,
) {
    let mut l: libc::c_int = strlen(str) as libc::c_int;
    if l != 0 {
        let mut G_: *mut growable = dest;
        let mut DR_needed_size: libc::c_long = ((*G_).tail + l) as libc::c_long;
        let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
        while ((*G_).size as libc::c_long) < DR_needed_size {
            DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
            if DR_newsize < 16 as libc::c_int as libc::c_long {
                DR_newsize = 16 as libc::c_int as libc::c_long;
            }
            (*G_).size = DR_newsize as libc::c_int;
        }
        if DR_newsize != 0 {
            (*G_)
                .base = xrealloc(
                (*G_).base as *mut libc::c_void,
                (DR_newsize as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
        }
        memcpy(
            ((*dest).base).offset((*dest).tail as isize) as *mut libc::c_void,
            str as *const libc::c_void,
            l as libc::c_ulong,
        );
        (*dest).tail += l;
    }
    append_null(dest);
}
static mut filechr_table: [libc::c_uchar; 256] = [
    (filechr_not_unix as libc::c_int | filechr_not_vms as libc::c_int
        | filechr_not_windows as libc::c_int | filechr_control as libc::c_int)
        as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_control as libc::c_int) as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_not_windows as libc::c_int)
        as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    (filechr_not_unix as libc::c_int | filechr_not_windows as libc::c_int)
        as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    (filechr_not_vms as libc::c_int | filechr_not_windows as libc::c_int)
        as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_not_windows as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    filechr_control as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn append_uri_pathel(
    mut b: *const libc::c_char,
    mut e: *const libc::c_char,
    mut escaped: bool,
    mut dest: *mut growable,
) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut unescaped: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted: libc::c_int = 0;
    let mut outlen: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut max_length: libc::c_int = 0;
    if dest.is_null() {
        return;
    }
    if opt.restrict_files_os as libc::c_uint
        == restrict_unix as libc::c_int as libc::c_uint
    {
        mask = filechr_not_unix as libc::c_int;
    } else if opt.restrict_files_os as libc::c_uint
        == restrict_vms as libc::c_int as libc::c_uint
    {
        mask = filechr_not_vms as libc::c_int;
    } else {
        mask = filechr_not_windows as libc::c_int;
    }
    if opt.restrict_files_ctrl {
        mask |= filechr_control as libc::c_int;
    }
    if escaped {
        let mut len: size_t = e.offset_from(b) as libc::c_long as size_t;
        if len < ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
            unescaped = buf.as_mut_ptr();
        } else {
            unescaped = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
        }
        memcpy(unescaped as *mut libc::c_void, b as *const libc::c_void, len);
        *unescaped.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        url_unescape(unescaped);
        b = unescaped;
        e = unescaped.offset(strlen(unescaped) as isize);
    }
    if e.offset_from(b) as libc::c_long == 2 as libc::c_int as libc::c_long
        && *b.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *b.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        b = b"%2E%2E\0" as *const u8 as *const libc::c_char;
        e = b.offset(6 as libc::c_int as isize);
    }
    quoted = 0 as libc::c_int;
    p = b;
    while p < e {
        if opt.restrict_files_nonascii as libc::c_int != 0
            && !c_isascii(*p as libc::c_uchar as libc::c_int)
            || filechr_table[*p as libc::c_uchar as usize] as libc::c_int & mask != 0
        {
            quoted += 1;
            quoted;
        }
        p = p.offset(1);
        p;
    }
    outlen = (e.offset_from(b) as libc::c_long
        + (2 as libc::c_int * quoted) as libc::c_long) as libc::c_int;
    max_length = get_max_length((*dest).base, (*dest).tail, _PC_NAME_MAX as libc::c_int)
        as libc::c_int;
    max_length -= 19 as libc::c_int;
    if max_length > 0 as libc::c_int && outlen > max_length {
        logprintf(
            LOG_NOTQUIET,
            b"The destination name is too long (%d), reducing to %d\n\0" as *const u8
                as *const libc::c_char,
            outlen,
            max_length,
        );
        outlen = max_length;
    }
    let mut G_: *mut growable = dest;
    let mut DR_needed_size: libc::c_long = ((*G_).tail + outlen) as libc::c_long;
    let mut DR_newsize: libc::c_long = 0 as libc::c_int as libc::c_long;
    while ((*G_).size as libc::c_long) < DR_needed_size {
        DR_newsize = ((*G_).size << 1 as libc::c_int) as libc::c_long;
        if DR_newsize < 16 as libc::c_int as libc::c_long {
            DR_newsize = 16 as libc::c_int as libc::c_long;
        }
        (*G_).size = DR_newsize as libc::c_int;
    }
    if DR_newsize != 0 {
        (*G_)
            .base = xrealloc(
            (*G_).base as *mut libc::c_void,
            (DR_newsize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    if ((*dest).base).is_null() {
        return;
    }
    if quoted == 0 {
        memcpy(
            ((*dest).base).offset((*dest).tail as isize) as *mut libc::c_void,
            b as *const libc::c_void,
            outlen as libc::c_ulong,
        );
    } else {
        let mut q: *mut libc::c_char = ((*dest).base).offset((*dest).tail as isize);
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        p = b;
        while p < e {
            if !(opt.restrict_files_nonascii as libc::c_int != 0
                && !c_isascii(*p as libc::c_uchar as libc::c_int)
                || filechr_table[*p as libc::c_uchar as usize] as libc::c_int & mask
                    != 0)
            {
                if i == outlen {
                    break;
                }
                let fresh19 = q;
                q = q.offset(1);
                *fresh19 = *p;
                i += 1;
                i;
            } else {
                if i + 3 as libc::c_int > outlen {
                    break;
                }
                let mut ch: libc::c_uchar = *p as libc::c_uchar;
                let fresh20 = q;
                q = q.offset(1);
                *fresh20 = '%' as i32 as libc::c_char;
                let fresh21 = q;
                q = q.offset(1);
                *fresh21 = ((*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(
                    b"0123456789ABCDEF\0",
                ))[(ch as libc::c_int >> 4 as libc::c_int) as usize] as libc::c_int
                    + 0 as libc::c_int) as libc::c_char;
                let fresh22 = q;
                q = q.offset(1);
                *fresh22 = ((*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(
                    b"0123456789ABCDEF\0",
                ))[(ch as libc::c_int & 0xf as libc::c_int) as usize] as libc::c_int
                    + 0 as libc::c_int) as libc::c_char;
                i += 3 as libc::c_int;
            }
            p = p.offset(1);
            p;
        }
    }
    if opt.restrict_files_case as libc::c_uint
        == restrict_lowercase as libc::c_int as libc::c_uint
        || opt.restrict_files_case as libc::c_uint
            == restrict_uppercase as libc::c_int as libc::c_uint
    {
        let mut q_0: *mut libc::c_char = 0 as *mut libc::c_char;
        q_0 = ((*dest).base).offset((*dest).tail as isize);
        while q_0 < ((*dest).base).offset((*dest).tail as isize).offset(outlen as isize)
        {
            if opt.restrict_files_case as libc::c_uint
                == restrict_lowercase as libc::c_int as libc::c_uint
            {
                *q_0 = c_tolower(*q_0 as libc::c_int) as libc::c_char;
            } else {
                *q_0 = c_toupper(*q_0 as libc::c_int) as libc::c_char;
            }
            q_0 = q_0.offset(1);
            q_0;
        }
    }
    (*dest).tail += outlen;
    append_null(dest);
    if !unescaped.is_null() && unescaped != buf.as_mut_ptr() {
        rpl_free(unescaped as *mut libc::c_void);
    }
}
unsafe extern "C" fn convert_fname(mut fname: *mut libc::c_char) -> *mut libc::c_char {
    let mut converted_fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from_encoding: *const libc::c_char = opt.encoding_remote;
    let mut to_encoding: *const libc::c_char = opt.locale;
    let mut cd: iconv_t = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut done: size_t = 0;
    let mut inlen: size_t = 0;
    let mut outlen: size_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut orig_fname: *const libc::c_char = 0 as *const libc::c_char;
    if from_encoding.is_null() {
        from_encoding = b"UTF-8\0" as *const u8 as *const libc::c_char;
    }
    if to_encoding.is_null() {
        to_encoding = nl_langinfo(CODESET as libc::c_int);
    }
    cd = iconv_open(to_encoding, from_encoding);
    if cd == -(1 as libc::c_int) as iconv_t {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Conversion from %s to %s isn't supported\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(0 as libc::c_int, from_encoding),
            quote_n(1 as libc::c_int, to_encoding),
        );
        return fname;
    }
    orig_fname = fname;
    inlen = strlen(fname);
    outlen = inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    len = outlen;
    s = xmalloc(outlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    converted_fname = s;
    done = 0 as libc::c_int as size_t;
    loop {
        *__errno_location() = 0 as libc::c_int;
        if iconv(
            cd,
            &mut fname as *mut *mut libc::c_char,
            &mut inlen,
            &mut s,
            &mut outlen,
        ) == 0 as libc::c_int as libc::c_ulong
            && iconv(
                cd,
                0 as *mut *mut libc::c_char,
                0 as *mut size_t,
                &mut s,
                &mut outlen,
            ) == 0 as libc::c_int as libc::c_ulong
        {
            *converted_fname
                .offset(len as isize)
                .offset(-(outlen as isize))
                .offset(-(done as isize)) = '\0' as i32 as libc::c_char;
            iconv_close(cd);
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"Converted file name '%s' (%s) -> '%s' (%s)\n\0" as *const u8
                        as *const libc::c_char,
                    orig_fname,
                    from_encoding,
                    converted_fname,
                    to_encoding,
                );
            }
            rpl_free(orig_fname as *mut libc::c_void);
            orig_fname = 0 as *const libc::c_char;
            return converted_fname;
        }
        if *__errno_location() == 22 as libc::c_int
            || *__errno_location() == 84 as libc::c_int
            || *__errno_location() == 0 as libc::c_int
        {
            if *__errno_location() != 0 {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Incomplete or invalid multibyte sequence encountered\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unconvertable multibyte sequence encountered\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            rpl_free(converted_fname as *mut libc::c_void);
            converted_fname = 0 as *mut libc::c_char;
            converted_fname = orig_fname as *mut libc::c_char;
            break;
        } else if *__errno_location() == 7 as libc::c_int {
            done = len;
            outlen = done
                .wrapping_add(inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong));
            len = outlen;
            converted_fname = xrealloc(
                converted_fname as *mut libc::c_void,
                outlen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            s = converted_fname.offset(done as isize);
        } else {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unhandled errno %d\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *__errno_location(),
            );
            rpl_free(converted_fname as *mut libc::c_void);
            converted_fname = 0 as *mut libc::c_char;
            converted_fname = orig_fname as *mut libc::c_char;
            break;
        }
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Failed to convert file name '%s' (%s) -> '?' (%s)\n\0" as *const u8
                as *const libc::c_char,
            orig_fname,
            from_encoding,
            to_encoding,
        );
    }
    iconv_close(cd);
    return converted_fname;
}
unsafe extern "C" fn append_dir_structure(mut u: *const url, mut dest: *mut growable) {
    let mut pathel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cut: libc::c_int = opt.cut_dirs;
    pathel = (*u).path;
    loop {
        next = strchr(pathel, '/' as i32);
        if next.is_null() {
            break;
        }
        let fresh23 = cut;
        cut = cut - 1;
        if !(fresh23 > 0 as libc::c_int) {
            if !(pathel == next) {
                if (*dest).tail != 0 {
                    append_char('/' as i32 as libc::c_char, dest);
                }
                append_uri_pathel(pathel, next, 1 as libc::c_int != 0, dest);
            }
        }
        pathel = next.offset(1 as libc::c_int as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn url_file_name(
    mut u: *const url,
    mut replaced_filename: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut fnres: growable = growable {
        base: 0 as *mut libc::c_char,
        size: 0,
        tail: 0,
    };
    let mut temp_fnres: growable = growable {
        base: 0 as *mut libc::c_char,
        size: 0,
        tail: 0,
    };
    let mut u_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unique: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fname_len_check: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index_filename: *const libc::c_char = b"index.html\0" as *const u8
        as *const libc::c_char;
    fnres.base = 0 as *mut libc::c_char;
    fnres.size = 0 as libc::c_int;
    fnres.tail = 0 as libc::c_int;
    temp_fnres.base = 0 as *mut libc::c_char;
    temp_fnres.size = 0 as libc::c_int;
    temp_fnres.tail = 0 as libc::c_int;
    if !(opt.default_page).is_null() {
        index_filename = opt.default_page;
    }
    if !(opt.dir_prefix).is_null() {
        append_string(opt.dir_prefix, &mut fnres);
    }
    if opt.dirstruct {
        if opt.protocol_directories {
            if temp_fnres.tail != 0 {
                append_char('/' as i32 as libc::c_char, &mut temp_fnres);
            }
            append_string(supported_schemes[(*u).scheme as usize].name, &mut temp_fnres);
        }
        if opt.add_hostdir {
            if temp_fnres.tail != 0 {
                append_char('/' as i32 as libc::c_char, &mut temp_fnres);
            }
            if 0 as libc::c_int
                != strcmp((*u).host, b"..\0" as *const u8 as *const libc::c_char)
            {
                append_string((*u).host, &mut temp_fnres);
            } else {
                append_string(
                    b"%2E%2E\0" as *const u8 as *const libc::c_char,
                    &mut temp_fnres,
                );
            }
            if (*u).port != scheme_default_port((*u).scheme) {
                let mut portstr: [libc::c_char; 24] = [0; 24];
                number_to_string(portstr.as_mut_ptr(), (*u).port as wgint);
                append_char(
                    (if opt.restrict_files_os as libc::c_uint
                        != restrict_windows as libc::c_int as libc::c_uint
                    {
                        ':' as i32
                    } else {
                        '+' as i32
                    }) as libc::c_char,
                    &mut temp_fnres,
                );
                append_string(portstr.as_mut_ptr(), &mut temp_fnres);
            }
        }
        append_dir_structure(u, &mut temp_fnres);
    }
    if replaced_filename.is_null() {
        u_file = if *(*u).file as libc::c_int != 0 { (*u).file } else { index_filename };
        if !((*u).query).is_null() {
            fname_len_check = concat_strings(
                u_file,
                if opt.restrict_files_os as libc::c_uint
                    != restrict_vms as libc::c_int as libc::c_uint
                    && opt.restrict_files_os as libc::c_uint
                        != restrict_windows as libc::c_int as libc::c_uint
                {
                    b"?\0" as *const u8 as *const libc::c_char
                } else {
                    b"@\0" as *const u8 as *const libc::c_char
                },
                (*u).query,
                0 as *mut libc::c_void,
            );
        } else {
            fname_len_check = strdupdelim(
                u_file,
                u_file.offset(strlen(u_file) as isize),
            );
        }
    } else {
        u_file = replaced_filename;
        fname_len_check = strdupdelim(u_file, u_file.offset(strlen(u_file) as isize));
    }
    if temp_fnres.tail != 0 {
        append_char('/' as i32 as libc::c_char, &mut temp_fnres);
    }
    append_uri_pathel(
        fname_len_check,
        fname_len_check.offset(strlen(fname_len_check) as isize),
        1 as libc::c_int != 0,
        &mut temp_fnres,
    );
    append_char('\0' as i32 as libc::c_char, &mut temp_fnres);
    fname = convert_fname(temp_fnres.base);
    temp_fnres.base = 0 as *mut libc::c_char;
    temp_fnres.size = 0 as libc::c_int;
    temp_fnres.tail = 0 as libc::c_int;
    append_string(fname, &mut temp_fnres);
    rpl_free(fname as *mut libc::c_void);
    fname = 0 as *mut libc::c_char;
    rpl_free(fname_len_check as *mut libc::c_void);
    fname_len_check = 0 as *mut libc::c_char;
    if fnres.tail != 0 {
        append_char('/' as i32 as libc::c_char, &mut fnres);
    }
    append_string(temp_fnres.base, &mut fnres);
    fname = fnres.base;
    rpl_free(temp_fnres.base as *mut libc::c_void);
    temp_fnres.base = 0 as *mut libc::c_char;
    if (opt.noclobber as libc::c_int != 0 || opt.always_rest as libc::c_int != 0
        || opt.timestamping as libc::c_int != 0 || opt.dirstruct as libc::c_int != 0
        || !(opt.output_document).is_null() || opt.backups > 0 as libc::c_int)
        && !(file_exists_p(fname, 0 as *mut file_stats_t) as libc::c_int != 0
            && !file_non_directory_p(fname))
    {
        unique = fname;
    } else {
        unique = unique_name_passthrough(fname);
        if unique != fname {
            rpl_free(fname as *mut libc::c_void);
            fname = 0 as *mut libc::c_char;
        }
    }
    return unique;
}
unsafe extern "C" fn path_simplify(
    mut scheme: url_scheme,
    mut path: *mut libc::c_char,
) -> bool {
    let mut h: *mut libc::c_char = path;
    let mut t: *mut libc::c_char = path;
    let mut beg: *mut libc::c_char = path;
    let mut end: *mut libc::c_char = strchr(path, '\0' as i32);
    while h < end {
        if *h.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*h.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *h.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32)
        {
            h = h.offset(2 as libc::c_int as isize);
        } else {
            let mut current_block_18: u64;
            if *h.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *h.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && (*h.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *h.offset(2 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32)
            {
                if t > beg {
                    t = t.offset(-1);
                    t;
                    while t > beg
                        && *t.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != '/' as i32
                    {
                        t = t.offset(-1);
                        t;
                    }
                    current_block_18 = 2968425633554183086;
                } else if scheme as libc::c_uint
                    == SCHEME_FTP as libc::c_int as libc::c_uint
                    || scheme as libc::c_uint
                        == SCHEME_FTPS as libc::c_int as libc::c_uint
                {
                    beg = t.offset(3 as libc::c_int as isize);
                    current_block_18 = 3511509469051899454;
                } else {
                    current_block_18 = 2968425633554183086;
                }
                match current_block_18 {
                    3511509469051899454 => {}
                    _ => {
                        h = h.offset(3 as libc::c_int as isize);
                        current_block_18 = 11307063007268554308;
                    }
                }
            } else {
                current_block_18 = 3511509469051899454;
            }
            match current_block_18 {
                3511509469051899454 => {
                    if t == h {
                        while h < end && *h as libc::c_int != '/' as i32 {
                            t = t.offset(1);
                            t;
                            h = h.offset(1);
                            h;
                        }
                        if h < end {
                            t = t.offset(1);
                            t;
                            h = h.offset(1);
                            h;
                        }
                    } else {
                        while h < end && *h as libc::c_int != '/' as i32 {
                            let fresh24 = h;
                            h = h.offset(1);
                            let fresh25 = t;
                            t = t.offset(1);
                            *fresh25 = *fresh24;
                        }
                        if h < end {
                            let fresh26 = h;
                            h = h.offset(1);
                            let fresh27 = t;
                            t = t.offset(1);
                            *fresh27 = *fresh26;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if t != h {
        *t = '\0' as i32 as libc::c_char;
    }
    return t != h;
}
unsafe extern "C" fn path_end(mut url: *const libc::c_char) -> *const libc::c_char {
    let mut scheme: url_scheme = url_scheme(url);
    let mut seps: *const libc::c_char = 0 as *const libc::c_char;
    if scheme as libc::c_uint == SCHEME_INVALID as libc::c_int as libc::c_uint {
        scheme = SCHEME_HTTP;
    }
    seps = (init_seps(scheme)).offset(2 as libc::c_int as isize);
    return strpbrk_or_eos(url, seps);
}
#[no_mangle]
pub unsafe extern "C" fn uri_merge(
    mut base: *const libc::c_char,
    mut link: *const libc::c_char,
) -> *mut libc::c_char {
    let mut linklength: libc::c_int = 0;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut merge: *mut libc::c_char = 0 as *mut libc::c_char;
    if url_has_scheme(link) {
        return xstrdup(link);
    }
    end = path_end(base);
    linklength = strlen(link) as libc::c_int;
    if *link == 0 {
        return xstrdup(base)
    } else if *link as libc::c_int == '?' as i32 {
        let mut baselength: libc::c_int = end.offset_from(base) as libc::c_long
            as libc::c_int;
        merge = xmalloc((baselength + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        memcpy(
            merge as *mut libc::c_void,
            base as *const libc::c_void,
            baselength as libc::c_ulong,
        );
        memcpy(
            merge.offset(baselength as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((baselength + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else if *link as libc::c_int == '#' as i32 {
        let mut baselength_0: libc::c_int = 0;
        let mut end1: *const libc::c_char = strchr(base, '#' as i32);
        if end1.is_null() {
            end1 = base.offset(strlen(base) as isize);
        }
        baselength_0 = end1.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((baselength_0 + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        memcpy(
            merge as *mut libc::c_void,
            base as *const libc::c_void,
            baselength_0 as libc::c_ulong,
        );
        memcpy(
            merge.offset(baselength_0 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge
            .offset((baselength_0 + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else if *link as libc::c_int == '/' as i32
        && *link.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        let mut span: libc::c_int = 0;
        let mut slash: *const libc::c_char = 0 as *const libc::c_char;
        let mut start_insert: *const libc::c_char = 0 as *const libc::c_char;
        slash = memchr(
            base as *const libc::c_void,
            '/' as i32,
            end.offset_from(base) as libc::c_long as libc::c_ulong,
        ) as *const libc::c_char;
        if !slash.is_null()
            && *slash.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            start_insert = slash;
        } else {
            start_insert = base;
        }
        span = start_insert.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((span + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if span != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span as libc::c_ulong,
            );
        }
        memcpy(
            merge.offset(span as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((span + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else if *link as libc::c_int == '/' as i32 {
        let mut span_0: libc::c_int = 0;
        let mut slash_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut start_insert_0: *const libc::c_char = 0 as *const libc::c_char;
        let mut pos: *const libc::c_char = base;
        let mut seen_slash_slash: bool = 0 as libc::c_int != 0;
        loop {
            slash_0 = memchr(
                pos as *const libc::c_void,
                '/' as i32,
                end.offset_from(pos) as libc::c_long as libc::c_ulong,
            ) as *const libc::c_char;
            if !(!slash_0.is_null() && !seen_slash_slash) {
                break;
            }
            if !(*slash_0.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
            {
                break;
            }
            pos = slash_0.offset(2 as libc::c_int as isize);
            seen_slash_slash = 1 as libc::c_int != 0;
        }
        if slash_0.is_null() && !seen_slash_slash {
            start_insert_0 = base;
        } else if slash_0.is_null() && seen_slash_slash as libc::c_int != 0 {
            start_insert_0 = end;
        } else if !slash_0.is_null() && !seen_slash_slash {
            start_insert_0 = base;
        } else if !slash_0.is_null() && seen_slash_slash as libc::c_int != 0 {
            start_insert_0 = slash_0;
        }
        span_0 = start_insert_0.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((span_0 + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if span_0 != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span_0 as libc::c_ulong,
            );
        }
        memcpy(
            merge.offset(span_0 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((span_0 + linklength) as isize) = '\0' as i32 as libc::c_char;
    } else {
        let mut need_explicit_slash: bool = 0 as libc::c_int != 0;
        let mut span_1: libc::c_int = 0;
        let mut start_insert_1: *const libc::c_char = 0 as *const libc::c_char;
        let mut last_slash: *const libc::c_char = memrchr(
            base as *const libc::c_void,
            '/' as i32,
            end.offset_from(base) as libc::c_long as size_t,
        ) as *const libc::c_char;
        if last_slash.is_null() {
            start_insert_1 = base;
        } else if !last_slash.is_null()
            && last_slash >= base.offset(2 as libc::c_int as isize)
            && *last_slash.offset(-(2 as libc::c_int) as isize) as libc::c_int
                == ':' as i32
            && *last_slash.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == '/' as i32
        {
            start_insert_1 = end.offset(1 as libc::c_int as isize);
            need_explicit_slash = 1 as libc::c_int != 0;
        } else {
            start_insert_1 = last_slash.offset(1 as libc::c_int as isize);
        }
        span_1 = start_insert_1.offset_from(base) as libc::c_long as libc::c_int;
        merge = xmalloc((span_1 + linklength + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        if span_1 != 0 {
            memcpy(
                merge as *mut libc::c_void,
                base as *const libc::c_void,
                span_1 as libc::c_ulong,
            );
        }
        if need_explicit_slash {
            *merge
                .offset(
                    (span_1 - 1 as libc::c_int) as isize,
                ) = '/' as i32 as libc::c_char;
        }
        memcpy(
            merge.offset(span_1 as isize) as *mut libc::c_void,
            link as *const libc::c_void,
            linklength as libc::c_ulong,
        );
        *merge.offset((span_1 + linklength) as isize) = '\0' as i32 as libc::c_char;
    }
    return merge;
}
#[no_mangle]
pub unsafe extern "C" fn url_string(
    mut url: *const url,
    mut auth_mode: url_auth_mode,
) -> *mut libc::c_char {
    let mut size: libc::c_int = 0;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted_host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted_user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quoted_passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scheme_port: libc::c_int = supported_schemes[(*url).scheme as usize]
        .default_port;
    let mut scheme_str: *const libc::c_char = supported_schemes[(*url).scheme as usize]
        .leading_string;
    let mut fplen: libc::c_int = full_path_length(url);
    let mut brackets_around_host: bool = false;
    if !((*url).user).is_null() {
        if auth_mode as libc::c_uint != URL_AUTH_HIDE as libc::c_int as libc::c_uint {
            quoted_user = url_escape_allow_passthrough((*url).user);
            if !((*url).passwd).is_null() {
                if auth_mode as libc::c_uint
                    == URL_AUTH_HIDE_PASSWD as libc::c_int as libc::c_uint
                {
                    quoted_passwd = b"*password*\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                } else {
                    quoted_passwd = url_escape_allow_passthrough((*url).passwd);
                }
            }
        }
    }
    quoted_host = url_escape_allow_passthrough((*url).host);
    if quoted_host != (*url).host {
        unescape_single_char(quoted_host, ':' as i32 as libc::c_char);
    }
    brackets_around_host = !(strchr(quoted_host, ':' as i32)).is_null();
    size = (strlen(scheme_str))
        .wrapping_add(strlen(quoted_host))
        .wrapping_add(
            (if brackets_around_host as libc::c_int != 0 {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_ulong,
        )
        .wrapping_add(fplen as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if (*url).port != scheme_port {
        size += 1 as libc::c_int + numdigit((*url).port as wgint);
    }
    if !quoted_user.is_null() {
        size = (size as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(quoted_user)),
            ) as libc::c_int as libc::c_int;
        if !quoted_passwd.is_null() {
            size = (size as libc::c_ulong)
                .wrapping_add(
                    (1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen(quoted_passwd)),
                ) as libc::c_int as libc::c_int;
        }
    }
    result = xmalloc(size as size_t) as *mut libc::c_char;
    p = result;
    let mut len: libc::c_int = strlen(scheme_str) as libc::c_int;
    memcpy(
        p as *mut libc::c_void,
        scheme_str as *const libc::c_void,
        len as libc::c_ulong,
    );
    p = p.offset(len as isize);
    if !quoted_user.is_null() {
        let mut len_0: libc::c_int = strlen(quoted_user) as libc::c_int;
        memcpy(
            p as *mut libc::c_void,
            quoted_user as *const libc::c_void,
            len_0 as libc::c_ulong,
        );
        p = p.offset(len_0 as isize);
        if !quoted_passwd.is_null() {
            let fresh28 = p;
            p = p.offset(1);
            *fresh28 = ':' as i32 as libc::c_char;
            let mut len_1: libc::c_int = strlen(quoted_passwd) as libc::c_int;
            memcpy(
                p as *mut libc::c_void,
                quoted_passwd as *const libc::c_void,
                len_1 as libc::c_ulong,
            );
            p = p.offset(len_1 as isize);
        }
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = '@' as i32 as libc::c_char;
    }
    if brackets_around_host {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = '[' as i32 as libc::c_char;
    }
    let mut len_2: libc::c_int = strlen(quoted_host) as libc::c_int;
    memcpy(
        p as *mut libc::c_void,
        quoted_host as *const libc::c_void,
        len_2 as libc::c_ulong,
    );
    p = p.offset(len_2 as isize);
    if brackets_around_host {
        let fresh31 = p;
        p = p.offset(1);
        *fresh31 = ']' as i32 as libc::c_char;
    }
    if (*url).port != scheme_port {
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = ':' as i32 as libc::c_char;
        p = number_to_string(p, (*url).port as wgint);
    }
    full_path_write(url, p);
    p = p.offset(fplen as isize);
    let fresh33 = p;
    p = p.offset(1);
    *fresh33 = '\0' as i32 as libc::c_char;
    if !quoted_user.is_null() && quoted_user != (*url).user {
        rpl_free(quoted_user as *mut libc::c_void);
        quoted_user = 0 as *mut libc::c_char;
    }
    if !quoted_passwd.is_null()
        && auth_mode as libc::c_uint == URL_AUTH_SHOW as libc::c_int as libc::c_uint
        && quoted_passwd != (*url).passwd
    {
        rpl_free(quoted_passwd as *mut libc::c_void);
        quoted_passwd = 0 as *mut libc::c_char;
    }
    if quoted_host != (*url).host {
        rpl_free(quoted_host as *mut libc::c_void);
        quoted_host = 0 as *mut libc::c_char;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn schemes_are_similar_p(
    mut a: url_scheme,
    mut b: url_scheme,
) -> bool {
    if a as libc::c_uint == b as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    if a as libc::c_uint == SCHEME_HTTP as libc::c_int as libc::c_uint
        && b as libc::c_uint == SCHEME_HTTPS as libc::c_int as libc::c_uint
        || a as libc::c_uint == SCHEME_HTTPS as libc::c_int as libc::c_uint
            && b as libc::c_uint == SCHEME_HTTP as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn getchar_from_escaped_string(
    mut str: *const libc::c_char,
    mut c: *mut libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = str;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32 {
        if !c_isxdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
            || !c_isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
        {
            *c = '%' as i32 as libc::c_char;
            return 1 as libc::c_int;
        } else {
            if *p.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            *c = (((_unhex(*p.offset(1 as libc::c_int as isize) as libc::c_uchar)
                as libc::c_int) << 4 as libc::c_int)
                + _unhex(*p.offset(2 as libc::c_int as isize) as libc::c_uchar)
                    as libc::c_int) as libc::c_char;
            if urlchr_table[*c as libc::c_uchar as usize] as libc::c_int
                & urlchr_reserved as libc::c_int != 0
            {
                *c = '%' as i32 as libc::c_char;
                return 1 as libc::c_int;
            } else {
                return 3 as libc::c_int
            }
        }
    } else {
        *c = *p.offset(0 as libc::c_int as isize);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn are_urls_equal(
    mut u1: *const libc::c_char,
    mut u2: *const libc::c_char,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut pp: libc::c_int = 0;
    let mut qq: libc::c_int = 0;
    let mut ch1: libc::c_char = 0;
    let mut ch2: libc::c_char = 0;
    p = u1;
    q = u2;
    while *p as libc::c_int != 0 && *q as libc::c_int != 0
        && {
            pp = getchar_from_escaped_string(p, &mut ch1);
            pp != 0
        }
        && {
            qq = getchar_from_escaped_string(q, &mut ch2);
            qq != 0
        } && c_tolower(ch1 as libc::c_int) == c_tolower(ch2 as libc::c_int)
    {
        p = p.offset(pp as isize);
        q = q.offset(qq as isize);
    }
    return if *p as libc::c_int == 0 as libc::c_int
        && *q as libc::c_int == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
