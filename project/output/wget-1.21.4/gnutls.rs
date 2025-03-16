#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __dirstream;
    pub type gnutls_session_int;
    pub type gnutls_pubkey_st;
    pub type gnutls_x509_crt_int;
    pub type gnutls_certificate_credentials_st;
    pub type hash_table;
    pub type ptimer;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn logputs(_: log_options, _: *const libc::c_char);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn abort() -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn gnutls_init(session: *mut gnutls_session_t, flags: libc::c_uint) -> libc::c_int;
    fn gnutls_deinit(session: gnutls_session_t);
    fn gnutls_handshake(session: gnutls_session_t) -> libc::c_int;
    fn gnutls_alert_get(session: gnutls_session_t) -> gnutls_alert_description_t;
    fn gnutls_alert_get_name(alert: gnutls_alert_description_t) -> *const libc::c_char;
    fn gnutls_certificate_type_get(
        session: gnutls_session_t,
    ) -> gnutls_certificate_type_t;
    fn gnutls_error_is_fatal(error: libc::c_int) -> libc::c_int;
    fn gnutls_strerror(error: libc::c_int) -> *const libc::c_char;
    fn gnutls_record_send(
        session: gnutls_session_t,
        data: *const libc::c_void,
        data_size: size_t,
    ) -> ssize_t;
    fn gnutls_record_recv(
        session: gnutls_session_t,
        data: *mut libc::c_void,
        data_size: size_t,
    ) -> ssize_t;
    fn gnutls_session_enable_compatibility_mode(session: gnutls_session_t);
    fn gnutls_record_get_direction(session: gnutls_session_t) -> libc::c_int;
    fn gnutls_record_check_pending(session: gnutls_session_t) -> size_t;
    fn gnutls_server_name_set(
        session: gnutls_session_t,
        type_0: gnutls_server_name_type_t,
        name: *const libc::c_void,
        name_length: size_t,
    ) -> libc::c_int;
    fn gnutls_priority_set_direct(
        session: gnutls_session_t,
        priorities: *const libc::c_char,
        err_pos: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn gnutls_set_default_priority(session: gnutls_session_t) -> libc::c_int;
    fn gnutls_session_set_data(
        session: gnutls_session_t,
        session_data: *const libc::c_void,
        session_data_size: size_t,
    ) -> libc::c_int;
    fn gnutls_session_get_data2(
        session: gnutls_session_t,
        data: *mut gnutls_datum_t,
    ) -> libc::c_int;
    fn gnutls_session_is_resumed(session: gnutls_session_t) -> libc::c_int;
    fn gnutls_credentials_set(
        session: gnutls_session_t,
        type_0: gnutls_credentials_type_t,
        cred: *mut libc::c_void,
    ) -> libc::c_int;
    fn gnutls_certificate_set_verify_flags(
        res: gnutls_certificate_credentials_t,
        flags: libc::c_uint,
    );
    fn gnutls_certificate_set_x509_system_trust(
        cred: gnutls_certificate_credentials_t,
    ) -> libc::c_int;
    fn gnutls_certificate_set_x509_trust_file(
        cred: gnutls_certificate_credentials_t,
        cafile: *const libc::c_char,
        type_0: gnutls_x509_crt_fmt_t,
    ) -> libc::c_int;
    fn gnutls_certificate_set_x509_crl_file(
        res: gnutls_certificate_credentials_t,
        crlfile: *const libc::c_char,
        type_0: gnutls_x509_crt_fmt_t,
    ) -> libc::c_int;
    fn gnutls_certificate_set_x509_key_file(
        res: gnutls_certificate_credentials_t,
        certfile: *const libc::c_char,
        keyfile: *const libc::c_char,
        type_0: gnutls_x509_crt_fmt_t,
    ) -> libc::c_int;
    fn gnutls_global_init() -> libc::c_int;
    fn gnutls_global_deinit();
    static mut gnutls_free: gnutls_free_function;
    fn gnutls_transport_set_ptr(session: gnutls_session_t, ptr: gnutls_transport_ptr_t);
    fn gnutls_certificate_get_peers(
        session: gnutls_session_t,
        list_size: *mut libc::c_uint,
    ) -> *const gnutls_datum_t;
    fn gnutls_certificate_verify_peers2(
        session: gnutls_session_t,
        status: *mut libc::c_uint,
    ) -> libc::c_int;
    fn gnutls_x509_crt_init(cert: *mut gnutls_x509_crt_t) -> libc::c_int;
    fn gnutls_x509_crt_deinit(cert: gnutls_x509_crt_t);
    fn gnutls_x509_crt_import(
        cert: gnutls_x509_crt_t,
        data: *const gnutls_datum_t,
        format: gnutls_x509_crt_fmt_t,
    ) -> libc::c_int;
    fn gnutls_x509_crt_check_hostname(
        cert: gnutls_x509_crt_t,
        hostname: *const libc::c_char,
    ) -> libc::c_uint;
    fn gnutls_x509_crt_get_activation_time(cert: gnutls_x509_crt_t) -> time_t;
    fn gnutls_x509_crt_get_expiration_time(cert: gnutls_x509_crt_t) -> time_t;
    fn gnutls_pubkey_init(key: *mut gnutls_pubkey_t) -> libc::c_int;
    fn gnutls_pubkey_deinit(key: gnutls_pubkey_t);
    fn gnutls_pubkey_import_x509(
        key: gnutls_pubkey_t,
        crt: gnutls_x509_crt_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn gnutls_pubkey_export(
        key: gnutls_pubkey_t,
        format: gnutls_x509_crt_fmt_t,
        output_data: *mut libc::c_void,
        output_data_size: *mut size_t,
    ) -> libc::c_int;
    fn gnutls_certificate_free_credentials(sc: gnutls_certificate_credentials_t);
    fn gnutls_certificate_allocate_credentials(
        res: *mut gnutls_certificate_credentials_t,
    ) -> libc::c_int;
    fn wg_pin_peer_pubkey(
        pinnedpubkey: *const libc::c_char,
        pubkey: *const libc::c_char,
        pubkeylen: size_t,
    ) -> bool;
    fn is_valid_ip_address(name: *const libc::c_char) -> bool;
    fn fd_transport_context(_: libc::c_int) -> *mut libc::c_void;
    fn fd_register_transport(
        _: libc::c_int,
        _: *mut transport_implementation,
        _: *mut libc::c_void,
    );
    fn select_fd(_: libc::c_int, _: libc::c_double, _: libc::c_int) -> libc::c_int;
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn hash_table_new(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong>,
        _: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    ) -> *mut hash_table;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> libc::c_int;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type intptr_t = libc::c_long;
pub type wgint = int64_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum CHECK_CERT_MODES {
    CHECK_CERT_OFF,
    CHECK_CERT_ON,
    CHECK_CERT_QUIET,
}
impl CHECK_CERT_MODES {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            CHECK_CERT_MODES::CHECK_CERT_OFF => 0,
            CHECK_CERT_MODES::CHECK_CERT_ON => 1,
            CHECK_CERT_MODES::CHECK_CERT_QUIET => 2,
        }
    }
}

pub const CHECK_CERT_QUIET: CHECK_CERT_MODES = 2;
pub const CHECK_CERT_ON: CHECK_CERT_MODES = 1;
pub const CHECK_CERT_OFF: CHECK_CERT_MODES = 0;
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
    fn to_libc_c_uint(self) -> libc::c_uint {
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
}

pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum gnutls_credentials_type_t {
    GNUTLS_CRD_IA = 5,
    GNUTLS_CRD_PSK = 4,
    GNUTLS_CRD_SRP = 3,
    GNUTLS_CRD_ANON = 2,
    GNUTLS_CRD_CERTIFICATE = 1,
}
impl gnutls_credentials_type_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            gnutls_credentials_type_t::GNUTLS_CRD_IA => 5,
            gnutls_credentials_type_t::GNUTLS_CRD_PSK => 4,
            gnutls_credentials_type_t::GNUTLS_CRD_SRP => 3,
            gnutls_credentials_type_t::GNUTLS_CRD_ANON => 2,
            gnutls_credentials_type_t::GNUTLS_CRD_CERTIFICATE => 1,
        }
    }
}

pub const GNUTLS_CRD_IA: gnutls_credentials_type_t = 5;
pub const GNUTLS_CRD_PSK: gnutls_credentials_type_t = 4;
pub const GNUTLS_CRD_SRP: gnutls_credentials_type_t = 3;
pub const GNUTLS_CRD_ANON: gnutls_credentials_type_t = 2;
pub const GNUTLS_CRD_CERTIFICATE: gnutls_credentials_type_t = 1;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum gnutls_alert_description_t {
    GNUTLS_A_NO_APPLICATION_PROTOCOL = 120,
    GNUTLS_A_UNKNOWN_PSK_IDENTITY = 115,
    GNUTLS_A_UNRECOGNIZED_NAME = 112,
    GNUTLS_A_CERTIFICATE_UNOBTAINABLE = 111,
    GNUTLS_A_UNSUPPORTED_EXTENSION = 110,
    GNUTLS_A_NO_RENEGOTIATION = 100,
    GNUTLS_A_USER_CANCELED = 90,
    GNUTLS_A_INAPPROPRIATE_FALLBACK = 86,
    GNUTLS_A_INTERNAL_ERROR = 80,
    GNUTLS_A_INSUFFICIENT_SECURITY = 71,
    GNUTLS_A_PROTOCOL_VERSION = 70,
    GNUTLS_A_EXPORT_RESTRICTION = 60,
    GNUTLS_A_DECRYPT_ERROR = 51,
    GNUTLS_A_DECODE_ERROR = 50,
    GNUTLS_A_ACCESS_DENIED = 49,
    GNUTLS_A_UNKNOWN_CA = 48,
    GNUTLS_A_ILLEGAL_PARAMETER = 47,
    GNUTLS_A_CERTIFICATE_UNKNOWN = 46,
    GNUTLS_A_CERTIFICATE_EXPIRED = 45,
    GNUTLS_A_CERTIFICATE_REVOKED = 44,
    GNUTLS_A_UNSUPPORTED_CERTIFICATE = 43,
    GNUTLS_A_BAD_CERTIFICATE = 42,
    GNUTLS_A_SSL3_NO_CERTIFICATE = 41,
    GNUTLS_A_HANDSHAKE_FAILURE = 40,
    GNUTLS_A_DECOMPRESSION_FAILURE = 30,
    GNUTLS_A_RECORD_OVERFLOW = 22,
    GNUTLS_A_DECRYPTION_FAILED = 21,
    GNUTLS_A_BAD_RECORD_MAC = 20,
    GNUTLS_A_UNEXPECTED_MESSAGE = 10,
    GNUTLS_A_CLOSE_NOTIFY = 0,
}
impl gnutls_alert_description_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            gnutls_alert_description_t::GNUTLS_A_NO_APPLICATION_PROTOCOL => 120,
            gnutls_alert_description_t::GNUTLS_A_UNKNOWN_PSK_IDENTITY => 115,
            gnutls_alert_description_t::GNUTLS_A_UNRECOGNIZED_NAME => 112,
            gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_UNOBTAINABLE => 111,
            gnutls_alert_description_t::GNUTLS_A_UNSUPPORTED_EXTENSION => 110,
            gnutls_alert_description_t::GNUTLS_A_NO_RENEGOTIATION => 100,
            gnutls_alert_description_t::GNUTLS_A_USER_CANCELED => 90,
            gnutls_alert_description_t::GNUTLS_A_INAPPROPRIATE_FALLBACK => 86,
            gnutls_alert_description_t::GNUTLS_A_INTERNAL_ERROR => 80,
            gnutls_alert_description_t::GNUTLS_A_INSUFFICIENT_SECURITY => 71,
            gnutls_alert_description_t::GNUTLS_A_PROTOCOL_VERSION => 70,
            gnutls_alert_description_t::GNUTLS_A_EXPORT_RESTRICTION => 60,
            gnutls_alert_description_t::GNUTLS_A_DECRYPT_ERROR => 51,
            gnutls_alert_description_t::GNUTLS_A_DECODE_ERROR => 50,
            gnutls_alert_description_t::GNUTLS_A_ACCESS_DENIED => 49,
            gnutls_alert_description_t::GNUTLS_A_UNKNOWN_CA => 48,
            gnutls_alert_description_t::GNUTLS_A_ILLEGAL_PARAMETER => 47,
            gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_UNKNOWN => 46,
            gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_EXPIRED => 45,
            gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_REVOKED => 44,
            gnutls_alert_description_t::GNUTLS_A_UNSUPPORTED_CERTIFICATE => 43,
            gnutls_alert_description_t::GNUTLS_A_BAD_CERTIFICATE => 42,
            gnutls_alert_description_t::GNUTLS_A_SSL3_NO_CERTIFICATE => 41,
            gnutls_alert_description_t::GNUTLS_A_HANDSHAKE_FAILURE => 40,
            gnutls_alert_description_t::GNUTLS_A_DECOMPRESSION_FAILURE => 30,
            gnutls_alert_description_t::GNUTLS_A_RECORD_OVERFLOW => 22,
            gnutls_alert_description_t::GNUTLS_A_DECRYPTION_FAILED => 21,
            gnutls_alert_description_t::GNUTLS_A_BAD_RECORD_MAC => 20,
            gnutls_alert_description_t::GNUTLS_A_UNEXPECTED_MESSAGE => 10,
            gnutls_alert_description_t::GNUTLS_A_CLOSE_NOTIFY => 0,
        }
    }
}

pub const GNUTLS_A_NO_APPLICATION_PROTOCOL: gnutls_alert_description_t = 120;
pub const GNUTLS_A_UNKNOWN_PSK_IDENTITY: gnutls_alert_description_t = 115;
pub const GNUTLS_A_UNRECOGNIZED_NAME: gnutls_alert_description_t = 112;
pub const GNUTLS_A_CERTIFICATE_UNOBTAINABLE: gnutls_alert_description_t = 111;
pub const GNUTLS_A_UNSUPPORTED_EXTENSION: gnutls_alert_description_t = 110;
pub const GNUTLS_A_NO_RENEGOTIATION: gnutls_alert_description_t = 100;
pub const GNUTLS_A_USER_CANCELED: gnutls_alert_description_t = 90;
pub const GNUTLS_A_INAPPROPRIATE_FALLBACK: gnutls_alert_description_t = 86;
pub const GNUTLS_A_INTERNAL_ERROR: gnutls_alert_description_t = 80;
pub const GNUTLS_A_INSUFFICIENT_SECURITY: gnutls_alert_description_t = 71;
pub const GNUTLS_A_PROTOCOL_VERSION: gnutls_alert_description_t = 70;
pub const GNUTLS_A_EXPORT_RESTRICTION: gnutls_alert_description_t = 60;
pub const GNUTLS_A_DECRYPT_ERROR: gnutls_alert_description_t = 51;
pub const GNUTLS_A_DECODE_ERROR: gnutls_alert_description_t = 50;
pub const GNUTLS_A_ACCESS_DENIED: gnutls_alert_description_t = 49;
pub const GNUTLS_A_UNKNOWN_CA: gnutls_alert_description_t = 48;
pub const GNUTLS_A_ILLEGAL_PARAMETER: gnutls_alert_description_t = 47;
pub const GNUTLS_A_CERTIFICATE_UNKNOWN: gnutls_alert_description_t = 46;
pub const GNUTLS_A_CERTIFICATE_EXPIRED: gnutls_alert_description_t = 45;
pub const GNUTLS_A_CERTIFICATE_REVOKED: gnutls_alert_description_t = 44;
pub const GNUTLS_A_UNSUPPORTED_CERTIFICATE: gnutls_alert_description_t = 43;
pub const GNUTLS_A_BAD_CERTIFICATE: gnutls_alert_description_t = 42;
pub const GNUTLS_A_SSL3_NO_CERTIFICATE: gnutls_alert_description_t = 41;
pub const GNUTLS_A_HANDSHAKE_FAILURE: gnutls_alert_description_t = 40;
pub const GNUTLS_A_DECOMPRESSION_FAILURE: gnutls_alert_description_t = 30;
pub const GNUTLS_A_RECORD_OVERFLOW: gnutls_alert_description_t = 22;
pub const GNUTLS_A_DECRYPTION_FAILED: gnutls_alert_description_t = 21;
pub const GNUTLS_A_BAD_RECORD_MAC: gnutls_alert_description_t = 20;
pub const GNUTLS_A_UNEXPECTED_MESSAGE: gnutls_alert_description_t = 10;
pub const GNUTLS_A_CLOSE_NOTIFY: gnutls_alert_description_t = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    GNUTLS_CERT_INVALID_OCSP_STATUS = 1048576,
    GNUTLS_CERT_MISSING_OCSP_STATUS = 524288,
    GNUTLS_CERT_PURPOSE_MISMATCH = 262144,
    GNUTLS_CERT_MISMATCH = 131072,
    GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE = 65536,
    GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE = 32768,
    GNUTLS_CERT_UNEXPECTED_OWNER = 16384,
    GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED = 4096,
    GNUTLS_CERT_SIGNATURE_FAILURE = 2048,
    GNUTLS_CERT_EXPIRED = 1024,
    GNUTLS_CERT_NOT_ACTIVATED = 512,
    GNUTLS_CERT_INSECURE_ALGORITHM = 256,
    GNUTLS_CERT_SIGNER_NOT_CA = 128,
    GNUTLS_CERT_SIGNER_NOT_FOUND = 64,
    GNUTLS_CERT_REVOKED = 32,
    GNUTLS_CERT_INVALID = 2,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_4::GNUTLS_CERT_INVALID_OCSP_STATUS => 1048576,
            C2RustUnnamed_4::GNUTLS_CERT_MISSING_OCSP_STATUS => 524288,
            C2RustUnnamed_4::GNUTLS_CERT_PURPOSE_MISMATCH => 262144,
            C2RustUnnamed_4::GNUTLS_CERT_MISMATCH => 131072,
            C2RustUnnamed_4::GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE => 65536,
            C2RustUnnamed_4::GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE => 32768,
            C2RustUnnamed_4::GNUTLS_CERT_UNEXPECTED_OWNER => 16384,
            C2RustUnnamed_4::GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED => 4096,
            C2RustUnnamed_4::GNUTLS_CERT_SIGNATURE_FAILURE => 2048,
            C2RustUnnamed_4::GNUTLS_CERT_EXPIRED => 1024,
            C2RustUnnamed_4::GNUTLS_CERT_NOT_ACTIVATED => 512,
            C2RustUnnamed_4::GNUTLS_CERT_INSECURE_ALGORITHM => 256,
            C2RustUnnamed_4::GNUTLS_CERT_SIGNER_NOT_CA => 128,
            C2RustUnnamed_4::GNUTLS_CERT_SIGNER_NOT_FOUND => 64,
            C2RustUnnamed_4::GNUTLS_CERT_REVOKED => 32,
            C2RustUnnamed_4::GNUTLS_CERT_INVALID => 2,
        }
    }
}

pub const GNUTLS_CERT_INVALID_OCSP_STATUS: C2RustUnnamed_4 = 1048576;
pub const GNUTLS_CERT_MISSING_OCSP_STATUS: C2RustUnnamed_4 = 524288;
pub const GNUTLS_CERT_PURPOSE_MISMATCH: C2RustUnnamed_4 = 262144;
pub const GNUTLS_CERT_MISMATCH: C2RustUnnamed_4 = 131072;
pub const GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE: C2RustUnnamed_4 = 65536;
pub const GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE: C2RustUnnamed_4 = 32768;
pub const GNUTLS_CERT_UNEXPECTED_OWNER: C2RustUnnamed_4 = 16384;
pub const GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED: C2RustUnnamed_4 = 4096;
pub const GNUTLS_CERT_SIGNATURE_FAILURE: C2RustUnnamed_4 = 2048;
pub const GNUTLS_CERT_EXPIRED: C2RustUnnamed_4 = 1024;
pub const GNUTLS_CERT_NOT_ACTIVATED: C2RustUnnamed_4 = 512;
pub const GNUTLS_CERT_INSECURE_ALGORITHM: C2RustUnnamed_4 = 256;
pub const GNUTLS_CERT_SIGNER_NOT_CA: C2RustUnnamed_4 = 128;
pub const GNUTLS_CERT_SIGNER_NOT_FOUND: C2RustUnnamed_4 = 64;
pub const GNUTLS_CERT_REVOKED: C2RustUnnamed_4 = 32;
pub const GNUTLS_CERT_INVALID: C2RustUnnamed_4 = 2;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum gnutls_certificate_type_t {
    GNUTLS_CRT_RAW = 3,
    GNUTLS_CRT_OPENPGP = 2,
    GNUTLS_CRT_X509 = 1,
    GNUTLS_CRT_UNKNOWN = 0,
}
impl gnutls_certificate_type_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            gnutls_certificate_type_t::GNUTLS_CRT_RAW => 3,
            gnutls_certificate_type_t::GNUTLS_CRT_OPENPGP => 2,
            gnutls_certificate_type_t::GNUTLS_CRT_X509 => 1,
            gnutls_certificate_type_t::GNUTLS_CRT_UNKNOWN => 0,
        }
    }
}

pub const GNUTLS_CRT_RAW: gnutls_certificate_type_t = 3;
pub const GNUTLS_CRT_OPENPGP: gnutls_certificate_type_t = 2;
pub const GNUTLS_CRT_X509: gnutls_certificate_type_t = 1;
pub const GNUTLS_CRT_UNKNOWN: gnutls_certificate_type_t = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum gnutls_x509_crt_fmt_t {
    GNUTLS_X509_FMT_PEM = 1,
    GNUTLS_X509_FMT_DER = 0,
}
impl gnutls_x509_crt_fmt_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_PEM => 1,
            gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_DER => 0,
        }
    }
}

pub const GNUTLS_X509_FMT_PEM: gnutls_x509_crt_fmt_t = 1;
pub const GNUTLS_X509_FMT_DER: gnutls_x509_crt_fmt_t = 0;
pub type gnutls_transport_ptr_t = *mut libc::c_void;
pub type gnutls_session_t = *mut gnutls_session_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gnutls_datum_t {
    pub data: *mut libc::c_uchar,
    pub size: libc::c_uint,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum gnutls_server_name_type_t {
    GNUTLS_NAME_DNS = 1,
}
impl gnutls_server_name_type_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            gnutls_server_name_type_t::GNUTLS_NAME_DNS => 1,
        }
    }
}

pub const GNUTLS_NAME_DNS: gnutls_server_name_type_t = 1;
pub type gnutls_pubkey_t = *mut gnutls_pubkey_st;
pub type gnutls_x509_crt_t = *mut gnutls_x509_crt_int;
pub type gnutls_certificate_credentials_t = *mut gnutls_certificate_credentials_st;
pub type gnutls_free_function = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    WAIT_FOR_READ = 1,
    WAIT_FOR_WRITE = 2,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_5::WAIT_FOR_READ => 1,
            C2RustUnnamed_5::WAIT_FOR_WRITE => 2,
        }
    }
}

pub const WAIT_FOR_WRITE: C2RustUnnamed_5 = 2;
pub const WAIT_FOR_READ: C2RustUnnamed_5 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transport_implementation {
    pub reader: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_char,
            libc::c_int,
            *mut libc::c_void,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub writer: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_char,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub poller: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_double,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub peeker: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_char,
            libc::c_int,
            *mut libc::c_void,
            libc::c_double,
        ) -> libc::c_int,
    >,
    pub errstr: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> *const libc::c_char,
    >,
    pub closer: Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wgnutls_transport_context {
    pub session: gnutls_session_t,
    pub session_data: *mut gnutls_datum_t,
    pub last_error: libc::c_int,
    pub peekbuf: [libc::c_char; 512],
    pub peeklen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_read_timer {
    pub timeout: libc::c_double,
    pub next_timeout: libc::c_double,
    pub timer: *mut ptimer,
    pub timed_out: libc::c_int,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn key_type_to_gnutls_type(mut type_0: keyfile_type) -> libc::c_int {
    match type_0 as libc::c_uint {
        0 => return GNUTLS_X509_FMT_PEM as libc::c_int,
        1 => return GNUTLS_X509_FMT_DER as libc::c_int,
        _ => {
            abort();
        }
    };
}
static mut ssl_initialized: bool = 0 as libc::c_int != 0;
static mut credentials: gnutls_certificate_credentials_t = 0
    as *const gnutls_certificate_credentials_st
    as *mut gnutls_certificate_credentials_st;
#[no_mangle]
pub unsafe extern "C" fn ssl_init() -> bool {
    let mut ca_directory: *const libc::c_char = 0 as *const libc::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut ncerts: libc::c_int = -(1 as libc::c_int);
    let mut rc: libc::c_int = 0;
    if ssl_initialized {
        return 1 as libc::c_int != 0;
    }
    gnutls_global_init();
    gnutls_certificate_allocate_credentials(&mut credentials);
    gnutls_certificate_set_verify_flags(credentials, 0 as libc::c_int as libc::c_uint);
    if (opt.ca_directory).is_null() {
        ncerts = gnutls_certificate_set_x509_system_trust(credentials);
    }
    if ncerts <= 0 as libc::c_int {
        ncerts = 0 as libc::c_int;
        ca_directory = if !(opt.ca_directory).is_null() {
            opt.ca_directory
        } else {
            b"/etc/ssl/certs\0" as *const u8 as *const libc::c_char
        };
        dir = opendir(ca_directory);
        if dir.is_null() {
            if !(opt.ca_directory).is_null() && *opt.ca_directory as libc::c_int != 0 {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"ERROR: Cannot open directory %s.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    opt.ca_directory,
                );
            }
        } else {
            let mut inode_map: *mut hash_table = hash_table_new(
                196 as libc::c_int,
                None,
                None,
            );
            let mut dent: *mut dirent = 0 as *mut dirent;
            loop {
                dent = readdir(dir);
                if dent.is_null() {
                    break;
                }
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
                let mut ca_file: [libc::c_char; 1024] = [0; 1024];
                if snprintf(
                    ca_file.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    ca_directory,
                    ((*dent).d_name).as_mut_ptr(),
                ) as libc::c_uint as libc::c_ulong
                    >= ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                {
                    continue;
                }
                if stat(ca_file.as_mut_ptr(), &mut st) != 0 as libc::c_int {
                    continue;
                }
                if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint)
                {
                    continue;
                }
                if hash_table_contains(
                    inode_map,
                    st.st_ino as intptr_t as *mut libc::c_void,
                ) != 0
                {
                    continue;
                }
                hash_table_put(
                    inode_map,
                    st.st_ino as intptr_t as *mut libc::c_void,
                    0 as *const libc::c_void,
                );
                rc = gnutls_certificate_set_x509_trust_file(
                    credentials,
                    ca_file.as_mut_ptr(),
                    GNUTLS_X509_FMT_PEM,
                );
                if rc <= 0 as libc::c_int {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"WARNING: Failed to open cert %s: (%d).\n\0" as *const u8
                                as *const libc::c_char,
                            ca_file.as_mut_ptr(),
                            rc,
                        );
                    }
                } else {
                    ncerts += rc;
                }
            }
            hash_table_destroy(inode_map);
            closedir(dir);
        }
    }
    if !(opt.ca_cert).is_null() {
        if ncerts < 0 as libc::c_int {
            ncerts = 0 as libc::c_int;
        }
        rc = gnutls_certificate_set_x509_trust_file(
            credentials,
            opt.ca_cert,
            GNUTLS_X509_FMT_PEM,
        );
        if rc <= 0 as libc::c_int {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"ERROR: Failed to open cert %s: (%d).\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                opt.ca_cert,
                rc,
            );
        } else {
            ncerts += rc;
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Loaded CA certificate '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                opt.ca_cert,
            );
        }
    }
    if !(opt.crl_file).is_null() {
        rc = gnutls_certificate_set_x509_crl_file(
            credentials,
            opt.crl_file,
            GNUTLS_X509_FMT_PEM,
        );
        if rc <= 0 as libc::c_int {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"ERROR: Failed to load CRL file '%s': (%d)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                opt.crl_file,
                rc,
            );
            return 0 as libc::c_int != 0;
        }
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Loaded CRL file '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            opt.crl_file,
        );
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Certificates loaded: %d\n\0" as *const u8 as *const libc::c_char,
            ncerts,
        );
    }
    if !(opt.cert_file).is_null() && (opt.private_key).is_null() {
        opt.private_key = xstrdup(opt.cert_file);
        opt.private_key_type = opt.cert_type;
    }
    if (opt.cert_file).is_null() && !(opt.private_key).is_null() {
        opt.cert_file = xstrdup(opt.private_key);
        opt.cert_type = opt.private_key_type;
    }
    if !(opt.cert_file).is_null() && !(opt.private_key).is_null() {
        let mut type_0: libc::c_int = 0;
        if opt.private_key_type as libc::c_uint != opt.cert_type as libc::c_uint {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"ERROR: GnuTLS requires the key and the cert to be of the same type.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        type_0 = key_type_to_gnutls_type(opt.private_key_type);
        gnutls_certificate_set_x509_key_file(
            credentials,
            opt.cert_file,
            opt.private_key,
            type_0 as gnutls_x509_crt_fmt_t,
        );
    }
    ssl_initialized = 1 as libc::c_int != 0;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ssl_cleanup() {
    if !ssl_initialized {
        return;
    }
    if !credentials.is_null() {
        gnutls_certificate_free_credentials(credentials);
    }
    gnutls_global_deinit();
    ssl_initialized = 0 as libc::c_int != 0;
}
unsafe extern "C" fn wgnutls_read_timeout(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut arg: *mut libc::c_void,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    let mut ret: libc::c_int = gnutls_record_check_pending((*ctx).session)
        as libc::c_int;
    let mut read_timer: st_read_timer = {
        let mut init = st_read_timer {
            timeout: if timeout == -(1 as libc::c_int) as libc::c_double {
                opt.read_timeout
            } else {
                timeout
            },
            next_timeout: 0 as libc::c_int as libc::c_double,
            timer: 0 as *mut ptimer,
            timed_out: 0 as libc::c_int,
        };
        init
    };
    if ret != 0 {
        return gnutls_record_recv(
            (*ctx).session,
            buf as *mut libc::c_void,
            (if ret <= bufsize { ret } else { bufsize }) as size_t,
        ) as libc::c_int;
    }
    if read_timer.timeout != 0. {
        flags = rpl_fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
        if flags < 0 as libc::c_int {
            return flags;
        }
        if rpl_fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int) != 0 {
            return -(1 as libc::c_int);
        }
        read_timer.timer = ptimer_new();
        if (read_timer.timer).is_null() {
            ret = -(1 as libc::c_int);
            current_block = 3275366147856559585;
        } else {
            read_timer.next_timeout = read_timer.timeout;
            current_block = 11650488183268122163;
        }
    } else {
        current_block = 11650488183268122163;
    }
    match current_block {
        11650488183268122163 => {
            ret = (*ctx).last_error;
            's_62: loop {
                if ret == -(37 as libc::c_int) {
                    let mut err: libc::c_int = 0;
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"GnuTLS: *** REHANDSHAKE while reading\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    err = _do_handshake((*ctx).session, fd, &mut read_timer);
                    if err != 0 as libc::c_int {
                        ret = err;
                        break;
                    }
                }
                loop {
                    ret = gnutls_record_recv(
                        (*ctx).session,
                        buf as *mut libc::c_void,
                        bufsize as size_t,
                    ) as libc::c_int;
                    if ret == -(28 as libc::c_int) && !(read_timer.timer).is_null() {
                        let mut err_0: libc::c_int = select_fd(
                            fd,
                            read_timer.next_timeout,
                            WAIT_FOR_READ as libc::c_int,
                        );
                        if err_0 <= 0 as libc::c_int {
                            if err_0 == 0 as libc::c_int {
                                read_timer.timed_out = 1 as libc::c_int;
                            }
                            break 's_62;
                        } else {
                            read_timer
                                .next_timeout = read_timer.timeout
                                - ptimer_measure(read_timer.timer);
                            if read_timer.next_timeout
                                <= 0 as libc::c_int as libc::c_double
                            {
                                read_timer.timed_out = 1 as libc::c_int;
                                break 's_62;
                            }
                        }
                    }
                    if !(ret == -(28 as libc::c_int) || ret == -(52 as libc::c_int)) {
                        break;
                    }
                }
                if !(ret == -(37 as libc::c_int)) {
                    break;
                }
            }
            if !(read_timer.timer).is_null() {
                ptimer_destroy(read_timer.timer);
                current_block = 3275366147856559585;
            } else {
                current_block = 6450636197030046351;
            }
        }
        _ => {}
    }
    match current_block {
        3275366147856559585 => {
            if rpl_fcntl(fd, 4 as libc::c_int, flags) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if read_timer.timed_out != 0 {
                *__errno_location() = 110 as libc::c_int;
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn wgnutls_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut arg: *mut libc::c_void,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    if (*ctx).peeklen != 0 {
        let mut copysize: libc::c_int = if bufsize <= (*ctx).peeklen {
            bufsize
        } else {
            (*ctx).peeklen
        };
        memcpy(
            buf as *mut libc::c_void,
            ((*ctx).peekbuf).as_mut_ptr() as *const libc::c_void,
            copysize as libc::c_ulong,
        );
        (*ctx).peeklen -= copysize;
        if (*ctx).peeklen != 0 as libc::c_int {
            memmove(
                ((*ctx).peekbuf).as_mut_ptr() as *mut libc::c_void,
                ((*ctx).peekbuf).as_mut_ptr().offset(copysize as isize)
                    as *const libc::c_void,
                (*ctx).peeklen as libc::c_ulong,
            );
        }
        return copysize;
    }
    ret = wgnutls_read_timeout(fd, buf, bufsize, arg, timeout);
    (*ctx).last_error = ret;
    return ret;
}
unsafe extern "C" fn wgnutls_write(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    let mut ret: libc::c_int = (*ctx).last_error;
    if ret == -(37 as libc::c_int) {
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"GnuTLS: *** REHANDSHAKE while writing\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        ret = _do_handshake((*ctx).session, fd, 0 as *mut st_read_timer);
        if ret != 0 as libc::c_int {
            current_block = 7941880718455721252;
        } else {
            current_block = 7351195479953500246;
        }
    } else {
        current_block = 7351195479953500246;
    }
    match current_block {
        7351195479953500246 => {
            loop {
                ret = gnutls_record_send(
                    (*ctx).session,
                    buf as *const libc::c_void,
                    bufsize as size_t,
                ) as libc::c_int;
                if !(ret == -(52 as libc::c_int) || ret == -(28 as libc::c_int)) {
                    break;
                }
            }
        }
        _ => {}
    }
    (*ctx).last_error = ret;
    return ret;
}
unsafe extern "C" fn wgnutls_poll(
    mut fd: libc::c_int,
    mut timeout: libc::c_double,
    mut wait_for: libc::c_int,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    if wait_for & WAIT_FOR_READ as libc::c_int != 0
        && ((*ctx).peeklen != 0 || gnutls_record_check_pending((*ctx).session) != 0)
    {
        return 1 as libc::c_int;
    }
    if timeout == -(1 as libc::c_int) as libc::c_double {
        timeout = opt.read_timeout;
    }
    return select_fd(fd, timeout, wait_for);
}
unsafe extern "C" fn wgnutls_peek(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut arg: *mut libc::c_void,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut read: libc::c_int = 0 as libc::c_int;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    let mut offset: libc::c_int = if bufsize <= (*ctx).peeklen {
        bufsize
    } else {
        (*ctx).peeklen
    };
    if (*ctx).peeklen != 0 {
        memcpy(
            buf as *mut libc::c_void,
            ((*ctx).peekbuf).as_mut_ptr() as *const libc::c_void,
            offset as libc::c_ulong,
        );
        return offset;
    }
    if bufsize
        > ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int
    {
        bufsize = ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
            as libc::c_int;
    }
    if bufsize > offset {
        read = wgnutls_read_timeout(
            fd,
            buf.offset(offset as isize),
            bufsize - offset,
            ctx as *mut libc::c_void,
            timeout,
        );
        (*ctx).last_error = read;
        if read < 0 as libc::c_int {
            if offset != 0 {
                read = 0 as libc::c_int;
            } else {
                return read
            }
        }
        if read > 0 as libc::c_int {
            memcpy(
                ((*ctx).peekbuf).as_mut_ptr().offset(offset as isize)
                    as *mut libc::c_void,
                buf.offset(offset as isize) as *const libc::c_void,
                read as libc::c_ulong,
            );
            (*ctx).peeklen += read;
        }
    }
    return offset + read;
}
unsafe extern "C" fn wgnutls_errstr(
    mut fd: libc::c_int,
    mut arg: *mut libc::c_void,
) -> *const libc::c_char {
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    if (*ctx).last_error > 0 as libc::c_int
        || ((*ctx).last_error == -(28 as libc::c_int)
            || (*ctx).last_error == -(37 as libc::c_int))
            && *__errno_location() == 110 as libc::c_int
    {
        return 0 as *const libc::c_char;
    }
    return gnutls_strerror((*ctx).last_error);
}
unsafe extern "C" fn wgnutls_close(mut fd: libc::c_int, mut arg: *mut libc::c_void) {
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    if !((*ctx).session_data).is_null() {
        gnutls_free
            .expect(
                "non-null function pointer",
            )((*(*ctx).session_data).data as *mut libc::c_void);
        gnutls_free
            .expect(
                "non-null function pointer",
            )((*ctx).session_data as *mut libc::c_void);
    }
    gnutls_deinit((*ctx).session);
    rpl_free(ctx as *mut libc::c_void);
    ctx = 0 as *mut wgnutls_transport_context;
    close(fd);
}
static mut wgnutls_transport: transport_implementation = unsafe {
    {
        let mut init = transport_implementation {
            reader: Some(
                wgnutls_read
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_double,
                    ) -> libc::c_int,
            ),
            writer: Some(
                wgnutls_write
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            poller: Some(
                wgnutls_poll
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_double,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            peeker: Some(
                wgnutls_peek
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_double,
                    ) -> libc::c_int,
            ),
            errstr: Some(
                wgnutls_errstr
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> *const libc::c_char,
            ),
            closer: Some(
                wgnutls_close
                    as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn _do_handshake(
    mut session: gnutls_session_t,
    mut fd: libc::c_int,
    mut read_timer: *mut st_read_timer,
) -> libc::c_int {
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut err: libc::c_int = 0;
    let mut next_timeout: libc::c_double = if !read_timer.is_null() {
        (*read_timer).next_timeout
    } else {
        opt.read_timeout
    };
    if read_timer.is_null() && next_timeout != 0. {
        flags = rpl_fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
        if flags < 0 as libc::c_int {
            return flags;
        }
        if rpl_fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int) != 0 {
            return -(1 as libc::c_int);
        }
    }
    let mut current_block_25: u64;
    loop {
        err = gnutls_handshake(session);
        if err == -(28 as libc::c_int) && next_timeout != 0. {
            let mut sel: libc::c_int = 0;
            if gnutls_record_get_direction(session) != 0 {
                sel = WAIT_FOR_WRITE as libc::c_int;
            } else {
                sel = WAIT_FOR_READ as libc::c_int;
            }
            sel = select_fd(fd, next_timeout, sel);
            if sel <= 0 as libc::c_int {
                if !(sel == 0 as libc::c_int) {
                    break;
                }
                if !read_timer.is_null() {
                    current_block_25 = 8558279550452098021;
                } else {
                    *__errno_location() = 110 as libc::c_int;
                    err = -(1 as libc::c_int);
                    break;
                }
            } else if !read_timer.is_null() {
                (*read_timer)
                    .next_timeout = (*read_timer).timeout
                    - ptimer_measure((*read_timer).timer);
                if (*read_timer).next_timeout <= 0 as libc::c_int as libc::c_double {
                    current_block_25 = 8558279550452098021;
                } else {
                    next_timeout = (*read_timer).next_timeout;
                    current_block_25 = 2473556513754201174;
                }
            } else {
                current_block_25 = 2473556513754201174;
            }
            match current_block_25 {
                2473556513754201174 => {}
                _ => {
                    err = -(37 as libc::c_int);
                    (*read_timer).timed_out = 1 as libc::c_int;
                    break;
                }
            }
        } else if err < 0 as libc::c_int {
            logprintf(
                LOG_NOTQUIET,
                b"GnuTLS: %s\n\0" as *const u8 as *const libc::c_char,
                gnutls_strerror(err),
            );
            if err == -(16 as libc::c_int) || err == -(12 as libc::c_int) {
                let mut alert: gnutls_alert_description_t = gnutls_alert_get(session);
                let mut str: *const libc::c_char = gnutls_alert_get_name(alert);
                logprintf(
                    LOG_NOTQUIET,
                    b"GnuTLS: received alert [%u]: %s\n\0" as *const u8
                        as *const libc::c_char,
                    alert as libc::c_uint,
                    if !str.is_null() {
                        str
                    } else {
                        b"(unknown)\0" as *const u8 as *const libc::c_char
                    },
                );
            }
        }
        if !(err != 0 && gnutls_error_is_fatal(err) == 0 as libc::c_int) {
            break;
        }
    }
    if read_timer.is_null() && next_timeout != 0. {
        if rpl_fcntl(fd, 4 as libc::c_int, flags) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return err;
}
unsafe extern "C" fn _sni_hostname(
    mut hostname: *const libc::c_char,
) -> *const libc::c_char {
    let mut len: size_t = strlen(hostname);
    let mut sni_hostname: *mut libc::c_char = xmemdup(
        hostname as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    while len != 0
        && {
            len = len.wrapping_sub(1);
            *sni_hostname.offset(len as isize) as libc::c_int == '.' as i32
        }
    {
        *sni_hostname.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    }
    return sni_hostname;
}
unsafe extern "C" fn set_prio_default(mut session: gnutls_session_t) -> libc::c_int {
    let mut err: libc::c_int = -(1 as libc::c_int);
    match opt.secure_protocol as libc::c_uint {
        0 => {
            err = gnutls_set_default_priority(session);
            gnutls_session_enable_compatibility_mode(session);
        }
        1 | 2 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-TLS-ALL:+VERS-SSL3.0\0" as *const u8
                    as *const libc::c_char,
                0 as *mut *const libc::c_char,
            );
        }
        3 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-SSL3.0\0" as *const u8 as *const libc::c_char,
                0 as *mut *const libc::c_char,
            );
        }
        4 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-SSL3.0:-VERS-TLS1.0\0" as *const u8
                    as *const libc::c_char,
                0 as *mut *const libc::c_char,
            );
        }
        5 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-SSL3.0:-VERS-TLS1.0:-VERS-TLS1.1\0" as *const u8
                    as *const libc::c_char,
                0 as *mut *const libc::c_char,
            );
        }
        6 => {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Your GnuTLS version is too old to support TLS 1.3\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return -(1 as libc::c_int);
        }
        7 => {
            err = gnutls_priority_set_direct(
                session,
                b"PFS:-VERS-SSL3.0\0" as *const u8 as *const libc::c_char,
                0 as *mut *const libc::c_char,
            );
            if err != 0 as libc::c_int {
                err = gnutls_priority_set_direct(
                    session,
                    b"NORMAL:-RSA:-VERS-SSL3.0\0" as *const u8 as *const libc::c_char,
                    0 as *mut *const libc::c_char,
                );
            }
        }
        _ => {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"GnuTLS: unimplemented 'secure-protocol' option value %u\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                opt.secure_protocol as libc::c_uint,
            );
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Please report this issue to bug-wget@gnu.org\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ssl_connect_wget(
    mut fd: libc::c_int,
    mut hostname: *const libc::c_char,
    mut continue_session: *mut libc::c_int,
) -> bool {
    let mut ctx: *mut wgnutls_transport_context = 0 as *mut wgnutls_transport_context;
    let mut session: gnutls_session_t = 0 as *mut gnutls_session_int;
    let mut err: libc::c_int = 0;
    gnutls_init(&mut session, ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint);
    if !is_valid_ip_address(hostname) {
        let mut sni_hostname: *const libc::c_char = _sni_hostname(hostname);
        gnutls_server_name_set(
            session,
            GNUTLS_NAME_DNS,
            sni_hostname as *const libc::c_void,
            strlen(sni_hostname),
        );
        rpl_free(sni_hostname as *mut libc::c_void);
        sni_hostname = 0 as *const libc::c_char;
    }
    gnutls_credentials_set(
        session,
        GNUTLS_CRD_CERTIFICATE,
        credentials as *mut libc::c_void,
    );
    gnutls_transport_set_ptr(session, fd as intptr_t as gnutls_transport_ptr_t);
    if (opt.tls_ciphers_string).is_null() {
        err = set_prio_default(session);
    } else {
        err = gnutls_priority_set_direct(
            session,
            opt.tls_ciphers_string,
            0 as *mut *const libc::c_char,
        );
    }
    if err < 0 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            b"GnuTLS: %s\n\0" as *const u8 as *const libc::c_char,
            gnutls_strerror(err),
        );
        gnutls_deinit(session);
        return 0 as libc::c_int != 0;
    }
    if !continue_session.is_null() {
        ctx = fd_transport_context(*continue_session) as *mut wgnutls_transport_context;
        if gnutls_session_is_resumed(session) == 0 {
            if ctx.is_null() || ((*ctx).session_data).is_null()
                || gnutls_session_set_data(
                    session,
                    (*(*ctx).session_data).data as *const libc::c_void,
                    (*(*ctx).session_data).size as size_t,
                ) != 0
            {
                if !ctx.is_null() && !((*ctx).session_data).is_null() {
                    if !((*(*ctx).session_data).data).is_null() {
                        gnutls_free
                            .expect(
                                "non-null function pointer",
                            )((*(*ctx).session_data).data as *mut libc::c_void);
                    }
                    gnutls_free
                        .expect(
                            "non-null function pointer",
                        )((*ctx).session_data as *mut libc::c_void);
                }
                gnutls_deinit(session);
                return 0 as libc::c_int != 0;
            }
        } else {
            logputs(
                LOG_ALWAYS,
                b"SSL session has already been resumed. Continuing.\n\0" as *const u8
                    as *const libc::c_char,
            );
            continue_session = 0 as *mut libc::c_int;
        }
    }
    err = _do_handshake(session, fd, 0 as *mut st_read_timer);
    if err < 0 as libc::c_int {
        gnutls_deinit(session);
        return 0 as libc::c_int != 0;
    }
    ctx = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wgnutls_transport_context>() as libc::c_ulong,
    ) as *mut wgnutls_transport_context;
    (*ctx)
        .session_data = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<gnutls_datum_t>() as libc::c_ulong,
    ) as *mut gnutls_datum_t;
    (*ctx).session = session;
    if gnutls_session_get_data2(session, (*ctx).session_data) != 0 {
        rpl_free((*ctx).session_data as *mut libc::c_void);
        (*ctx).session_data = 0 as *mut gnutls_datum_t;
        logprintf(
            LOG_NOTQUIET,
            b"WARNING: Could not save SSL session data for socket %d\n\0" as *const u8
                as *const libc::c_char,
            fd,
        );
    }
    fd_register_transport(fd, &mut wgnutls_transport, ctx as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn pkp_pin_peer_pubkey(
    mut cert: gnutls_x509_crt_t,
    mut pinnedpubkey: *const libc::c_char,
) -> bool {
    let mut len1: size_t = 0 as libc::c_int as size_t;
    let mut len2: size_t = 0 as libc::c_int as size_t;
    let mut buff1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: gnutls_pubkey_t = 0 as gnutls_pubkey_t;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut result: bool = 0 as libc::c_int != 0;
    if pinnedpubkey.is_null() {
        return 1 as libc::c_int != 0;
    }
    if cert.is_null() {
        return result;
    }
    gnutls_pubkey_init(&mut key);
    ret = gnutls_pubkey_import_x509(key, cert, 0 as libc::c_int as libc::c_uint);
    if !(ret < 0 as libc::c_int) {
        ret = gnutls_pubkey_export(
            key,
            GNUTLS_X509_FMT_DER,
            0 as *mut libc::c_void,
            &mut len1,
        );
        if !(ret != -(51 as libc::c_int) || len1 == 0 as libc::c_int as libc::c_ulong) {
            buff1 = xmalloc(len1) as *mut libc::c_char;
            len2 = len1;
            ret = gnutls_pubkey_export(
                key,
                GNUTLS_X509_FMT_DER,
                buff1 as *mut libc::c_void,
                &mut len2,
            );
            if !(ret < 0 as libc::c_int || len1 != len2) {
                result = wg_pin_peer_pubkey(pinnedpubkey, buff1, len1);
            }
        }
    }
    if !key.is_null() {
        gnutls_pubkey_deinit(key);
    }
    rpl_free(buff1 as *mut libc::c_void);
    buff1 = 0 as *mut libc::c_char;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ssl_check_certificate(
    mut fd: libc::c_int,
    mut host: *const libc::c_char,
) -> bool {
    let mut ctx: *mut wgnutls_transport_context = fd_transport_context(fd)
        as *mut wgnutls_transport_context;
    let mut status: libc::c_uint = 0;
    let mut err: libc::c_int = 0;
    let mut severity: *const libc::c_char = if opt.check_cert != 0 {
        dcgettext(
            0 as *const libc::c_char,
            b"ERROR\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        )
    } else {
        dcgettext(
            0 as *const libc::c_char,
            b"WARNING\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        )
    };
    let mut success: bool = 1 as libc::c_int != 0;
    let mut pinsuccess: bool = (opt.pinnedpubkey).is_null();
    if opt.check_cert == CHECK_CERT_QUIET as libc::c_int
        && pinsuccess as libc::c_int != 0
    {
        return success;
    }
    err = gnutls_certificate_verify_peers2((*ctx).session, &mut status);
    if err < 0 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: No certificate presented by %s.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            severity,
            quotearg_style(escape_quoting_style, host),
        );
        success = 0 as libc::c_int != 0;
    } else {
        if status & GNUTLS_CERT_INVALID as libc::c_int as libc::c_uint != 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: The certificate of %s is not trusted.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                severity,
                quote(host),
            );
            success = 0 as libc::c_int != 0;
        }
        if status & GNUTLS_CERT_SIGNER_NOT_FOUND as libc::c_int as libc::c_uint != 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: The certificate of %s doesn't have a known issuer.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                severity,
                quote(host),
            );
            success = 0 as libc::c_int != 0;
        }
        if status & GNUTLS_CERT_REVOKED as libc::c_int as libc::c_uint != 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: The certificate of %s has been revoked.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                severity,
                quote(host),
            );
            success = 0 as libc::c_int != 0;
        }
        if status & GNUTLS_CERT_SIGNER_NOT_CA as libc::c_int as libc::c_uint != 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: The certificate signer of %s was not a CA.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                severity,
                quote(host),
            );
            success = 0 as libc::c_int != 0;
        }
        if status & GNUTLS_CERT_INSECURE_ALGORITHM as libc::c_int as libc::c_uint != 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: The certificate of %s was signed using an insecure algorithm.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                severity,
                quote(host),
            );
            success = 0 as libc::c_int != 0;
        }
        if status & GNUTLS_CERT_NOT_ACTIVATED as libc::c_int as libc::c_uint != 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: The certificate of %s is not yet activated.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                severity,
                quote(host),
            );
            success = 0 as libc::c_int != 0;
        }
        if status & GNUTLS_CERT_EXPIRED as libc::c_int as libc::c_uint != 0 {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: The certificate of %s has expired.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                severity,
                quote(host),
            );
            success = 0 as libc::c_int != 0;
        }
        if gnutls_certificate_type_get((*ctx).session) as libc::c_uint
            == GNUTLS_CRT_X509 as libc::c_int as libc::c_uint
        {
            let mut now: time_t = time(0 as *mut time_t);
            let mut cert: gnutls_x509_crt_t = 0 as *mut gnutls_x509_crt_int;
            let mut cert_list: *const gnutls_datum_t = 0 as *const gnutls_datum_t;
            let mut cert_list_size: libc::c_uint = 0;
            let mut sni_hostname: *const libc::c_char = 0 as *const libc::c_char;
            err = gnutls_x509_crt_init(&mut cert);
            if err < 0 as libc::c_int {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error initializing X509 certificate: %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    gnutls_strerror(err),
                );
                success = 0 as libc::c_int != 0;
            } else {
                cert_list = gnutls_certificate_get_peers(
                    (*ctx).session,
                    &mut cert_list_size,
                );
                if cert_list.is_null() {
                    logprintf(
                        LOG_NOTQUIET,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"No certificate found\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    success = 0 as libc::c_int != 0;
                } else {
                    err = gnutls_x509_crt_import(cert, cert_list, GNUTLS_X509_FMT_DER);
                    if err < 0 as libc::c_int {
                        logprintf(
                            LOG_NOTQUIET,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error parsing certificate: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            gnutls_strerror(err),
                        );
                        success = 0 as libc::c_int != 0;
                    } else {
                        if now < gnutls_x509_crt_get_activation_time(cert) {
                            logprintf(
                                LOG_NOTQUIET,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"The certificate has not yet been activated\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            success = 0 as libc::c_int != 0;
                        }
                        if now >= gnutls_x509_crt_get_expiration_time(cert) {
                            logprintf(
                                LOG_NOTQUIET,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"The certificate has expired\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            success = 0 as libc::c_int != 0;
                        }
                        sni_hostname = _sni_hostname(host);
                        if gnutls_x509_crt_check_hostname(cert, sni_hostname) == 0 {
                            logprintf(
                                LOG_NOTQUIET,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"The certificate's owner does not match hostname %s\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                quote(sni_hostname),
                            );
                            success = 0 as libc::c_int != 0;
                        }
                        rpl_free(sni_hostname as *mut libc::c_void);
                        sni_hostname = 0 as *const libc::c_char;
                        pinsuccess = pkp_pin_peer_pubkey(cert, opt.pinnedpubkey);
                        if !pinsuccess {
                            logprintf(
                                LOG_ALWAYS,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"The public key does not match pinned public key!\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            success = 0 as libc::c_int != 0;
                        }
                    }
                }
                gnutls_x509_crt_deinit(cert);
            }
        } else {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Certificate must be X.509\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            success = 0 as libc::c_int != 0;
        }
    }
    return if !pinsuccess {
        0 as libc::c_int
    } else if opt.check_cert == CHECK_CERT_ON as libc::c_int {
        success as libc::c_int
    } else {
        1 as libc::c_int
    } != 0;
}
