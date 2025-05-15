use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type __dirstream;
    pub type gnutls_session_int;
    pub type gnutls_pubkey_st;
    pub type gnutls_x509_crt_int;
    pub type gnutls_certificate_credentials_st;
    pub type hash_table;
    pub type ptimer;
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn quotearg_style(s: quoting_style, arg: *const i8) -> *mut i8;
    fn quote(arg: *const i8) -> *const i8;
    fn logputs(_: log_options, _: *const i8);
    fn debug_logprintf(_: *const i8, _: ...);
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn abort() -> !;
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
    fn opendir(__name: *const i8) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn gnutls_init(session: *mut gnutls_session_t, flags: u32) -> i32;
    fn gnutls_deinit(session: gnutls_session_t);
    fn gnutls_handshake(session: gnutls_session_t) -> i32;
    fn gnutls_alert_get(session: gnutls_session_t) -> gnutls_alert_description_t;
    fn gnutls_alert_get_name(alert: gnutls_alert_description_t) -> *const i8;
    fn gnutls_certificate_type_get(
        session: gnutls_session_t,
    ) -> gnutls_certificate_type_t;
    fn gnutls_error_is_fatal(error: i32) -> i32;
    fn gnutls_strerror(error: i32) -> *const i8;
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
    fn gnutls_record_get_direction(session: gnutls_session_t) -> i32;
    fn gnutls_record_check_pending(session: gnutls_session_t) -> size_t;
    fn gnutls_server_name_set(
        session: gnutls_session_t,
        type_0: gnutls_server_name_type_t,
        name: *const libc::c_void,
        name_length: size_t,
    ) -> i32;
    fn gnutls_priority_set_direct(
        session: gnutls_session_t,
        priorities: *const i8,
        err_pos: *mut *const i8,
    ) -> i32;
    fn gnutls_set_default_priority(session: gnutls_session_t) -> i32;
    fn gnutls_session_set_data(
        session: gnutls_session_t,
        session_data: *const libc::c_void,
        session_data_size: size_t,
    ) -> i32;
    fn gnutls_session_get_data2(
        session: gnutls_session_t,
        data: *mut gnutls_datum_t,
    ) -> i32;
    fn gnutls_session_is_resumed(session: gnutls_session_t) -> i32;
    fn gnutls_credentials_set(
        session: gnutls_session_t,
        type_0: gnutls_credentials_type_t,
        cred: *mut libc::c_void,
    ) -> i32;
    fn gnutls_certificate_set_verify_flags(
        res: gnutls_certificate_credentials_t,
        flags: u32,
    );
    fn gnutls_certificate_set_x509_system_trust(
        cred: gnutls_certificate_credentials_t,
    ) -> i32;
    fn gnutls_certificate_set_x509_trust_file(
        cred: gnutls_certificate_credentials_t,
        cafile: *const i8,
        type_0: gnutls_x509_crt_fmt_t,
    ) -> i32;
    fn gnutls_certificate_set_x509_crl_file(
        res: gnutls_certificate_credentials_t,
        crlfile: *const i8,
        type_0: gnutls_x509_crt_fmt_t,
    ) -> i32;
    fn gnutls_certificate_set_x509_key_file(
        res: gnutls_certificate_credentials_t,
        certfile: *const i8,
        keyfile: *const i8,
        type_0: gnutls_x509_crt_fmt_t,
    ) -> i32;
    fn gnutls_global_init() -> i32;
    fn gnutls_global_deinit();
    static mut gnutls_free: gnutls_free_function;
    fn gnutls_transport_set_ptr(session: gnutls_session_t, ptr: gnutls_transport_ptr_t);
    fn gnutls_certificate_get_peers(
        session: gnutls_session_t,
        list_size: *mut u32,
    ) -> *const gnutls_datum_t;
    fn gnutls_certificate_verify_peers2(
        session: gnutls_session_t,
        status: *mut u32,
    ) -> i32;
    fn gnutls_x509_crt_init(cert: *mut gnutls_x509_crt_t) -> i32;
    fn gnutls_x509_crt_deinit(cert: gnutls_x509_crt_t);
    fn gnutls_x509_crt_import(
        cert: gnutls_x509_crt_t,
        data: *const gnutls_datum_t,
        format: gnutls_x509_crt_fmt_t,
    ) -> i32;
    fn gnutls_x509_crt_check_hostname(
        cert: gnutls_x509_crt_t,
        hostname: *const i8,
    ) -> u32;
    fn gnutls_x509_crt_get_activation_time(cert: gnutls_x509_crt_t) -> time_t;
    fn gnutls_x509_crt_get_expiration_time(cert: gnutls_x509_crt_t) -> time_t;
    fn gnutls_pubkey_init(key: *mut gnutls_pubkey_t) -> i32;
    fn gnutls_pubkey_deinit(key: gnutls_pubkey_t);
    fn gnutls_pubkey_import_x509(
        key: gnutls_pubkey_t,
        crt: gnutls_x509_crt_t,
        flags: u32,
    ) -> i32;
    fn gnutls_pubkey_export(
        key: gnutls_pubkey_t,
        format: gnutls_x509_crt_fmt_t,
        output_data: *mut libc::c_void,
        output_data_size: *mut size_t,
    ) -> i32;
    fn gnutls_certificate_free_credentials(sc: gnutls_certificate_credentials_t);
    fn gnutls_certificate_allocate_credentials(
        res: *mut gnutls_certificate_credentials_t,
    ) -> i32;
    fn wg_pin_peer_pubkey(
        pinnedpubkey: *const i8,
        pubkey: *const i8,
        pubkeylen: size_t,
    ) -> bool;
    fn is_valid_ip_address(name: *const i8) -> bool;
    fn fd_transport_context(_: i32) -> *mut libc::c_void;
    fn fd_register_transport(
        _: i32,
        _: *mut transport_implementation,
        _: *mut libc::c_void,
    );
    fn select_fd(_: i32, _: libc::c_double, _: i32) -> i32;
    fn ptimer_new() -> *mut ptimer;
    fn ptimer_destroy(_: *mut ptimer);
    fn ptimer_measure(_: *mut ptimer) -> libc::c_double;
    fn hash_table_new(
        _: i32,
        _: Option<unsafe extern "C" fn(*const libc::c_void) -> u64>,
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>,
    ) -> *mut hash_table;
    fn hash_table_destroy(_: *mut hash_table);
    fn hash_table_contains(_: *const hash_table, _: *const libc::c_void) -> i32;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
}
pub type __int64_t = i64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
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
pub type intptr_t = i64;
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: u8,
    pub d_name: [i8; 256],
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
    fn to_libc_c_uint(self) -> u32 {
        match self {
            gnutls_credentials_type_t::GNUTLS_CRD_IA => 5,
            gnutls_credentials_type_t::GNUTLS_CRD_PSK => 4,
            gnutls_credentials_type_t::GNUTLS_CRD_SRP => 3,
            gnutls_credentials_type_t::GNUTLS_CRD_ANON => 2,
            gnutls_credentials_type_t::GNUTLS_CRD_CERTIFICATE => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> gnutls_credentials_type_t {
        match value {
            5 => gnutls_credentials_type_t::GNUTLS_CRD_IA,
            4 => gnutls_credentials_type_t::GNUTLS_CRD_PSK,
            3 => gnutls_credentials_type_t::GNUTLS_CRD_SRP,
            2 => gnutls_credentials_type_t::GNUTLS_CRD_ANON,
            1 => gnutls_credentials_type_t::GNUTLS_CRD_CERTIFICATE,
            _ => panic!("Invalid value for gnutls_credentials_type_t: {}", value),
        }
    }
}
impl AddAssign<u32> for gnutls_credentials_type_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for gnutls_credentials_type_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for gnutls_credentials_type_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for gnutls_credentials_type_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for gnutls_credentials_type_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for gnutls_credentials_type_t {
    type Output = gnutls_credentials_type_t;
    fn add(self, rhs: u32) -> gnutls_credentials_type_t {
        gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for gnutls_credentials_type_t {
    type Output = gnutls_credentials_type_t;
    fn sub(self, rhs: u32) -> gnutls_credentials_type_t {
        gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for gnutls_credentials_type_t {
    type Output = gnutls_credentials_type_t;
    fn mul(self, rhs: u32) -> gnutls_credentials_type_t {
        gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for gnutls_credentials_type_t {
    type Output = gnutls_credentials_type_t;
    fn div(self, rhs: u32) -> gnutls_credentials_type_t {
        gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for gnutls_credentials_type_t {
    type Output = gnutls_credentials_type_t;
    fn rem(self, rhs: u32) -> gnutls_credentials_type_t {
        gnutls_credentials_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> gnutls_alert_description_t {
        match value {
            120 => gnutls_alert_description_t::GNUTLS_A_NO_APPLICATION_PROTOCOL,
            115 => gnutls_alert_description_t::GNUTLS_A_UNKNOWN_PSK_IDENTITY,
            112 => gnutls_alert_description_t::GNUTLS_A_UNRECOGNIZED_NAME,
            111 => gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_UNOBTAINABLE,
            110 => gnutls_alert_description_t::GNUTLS_A_UNSUPPORTED_EXTENSION,
            100 => gnutls_alert_description_t::GNUTLS_A_NO_RENEGOTIATION,
            90 => gnutls_alert_description_t::GNUTLS_A_USER_CANCELED,
            86 => gnutls_alert_description_t::GNUTLS_A_INAPPROPRIATE_FALLBACK,
            80 => gnutls_alert_description_t::GNUTLS_A_INTERNAL_ERROR,
            71 => gnutls_alert_description_t::GNUTLS_A_INSUFFICIENT_SECURITY,
            70 => gnutls_alert_description_t::GNUTLS_A_PROTOCOL_VERSION,
            60 => gnutls_alert_description_t::GNUTLS_A_EXPORT_RESTRICTION,
            51 => gnutls_alert_description_t::GNUTLS_A_DECRYPT_ERROR,
            50 => gnutls_alert_description_t::GNUTLS_A_DECODE_ERROR,
            49 => gnutls_alert_description_t::GNUTLS_A_ACCESS_DENIED,
            48 => gnutls_alert_description_t::GNUTLS_A_UNKNOWN_CA,
            47 => gnutls_alert_description_t::GNUTLS_A_ILLEGAL_PARAMETER,
            46 => gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_UNKNOWN,
            45 => gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_EXPIRED,
            44 => gnutls_alert_description_t::GNUTLS_A_CERTIFICATE_REVOKED,
            43 => gnutls_alert_description_t::GNUTLS_A_UNSUPPORTED_CERTIFICATE,
            42 => gnutls_alert_description_t::GNUTLS_A_BAD_CERTIFICATE,
            41 => gnutls_alert_description_t::GNUTLS_A_SSL3_NO_CERTIFICATE,
            40 => gnutls_alert_description_t::GNUTLS_A_HANDSHAKE_FAILURE,
            30 => gnutls_alert_description_t::GNUTLS_A_DECOMPRESSION_FAILURE,
            22 => gnutls_alert_description_t::GNUTLS_A_RECORD_OVERFLOW,
            21 => gnutls_alert_description_t::GNUTLS_A_DECRYPTION_FAILED,
            20 => gnutls_alert_description_t::GNUTLS_A_BAD_RECORD_MAC,
            10 => gnutls_alert_description_t::GNUTLS_A_UNEXPECTED_MESSAGE,
            0 => gnutls_alert_description_t::GNUTLS_A_CLOSE_NOTIFY,
            _ => panic!("Invalid value for gnutls_alert_description_t: {}", value),
        }
    }
}
impl AddAssign<u32> for gnutls_alert_description_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = gnutls_alert_description_t::from_libc_c_uint(
            self.to_libc_c_uint() + rhs,
        );
    }
}
impl SubAssign<u32> for gnutls_alert_description_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = gnutls_alert_description_t::from_libc_c_uint(
            self.to_libc_c_uint() - rhs,
        );
    }
}
impl MulAssign<u32> for gnutls_alert_description_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = gnutls_alert_description_t::from_libc_c_uint(
            self.to_libc_c_uint() * rhs,
        );
    }
}
impl DivAssign<u32> for gnutls_alert_description_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = gnutls_alert_description_t::from_libc_c_uint(
            self.to_libc_c_uint() / rhs,
        );
    }
}
impl RemAssign<u32> for gnutls_alert_description_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = gnutls_alert_description_t::from_libc_c_uint(
            self.to_libc_c_uint() % rhs,
        );
    }
}
impl Add<u32> for gnutls_alert_description_t {
    type Output = gnutls_alert_description_t;
    fn add(self, rhs: u32) -> gnutls_alert_description_t {
        gnutls_alert_description_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for gnutls_alert_description_t {
    type Output = gnutls_alert_description_t;
    fn sub(self, rhs: u32) -> gnutls_alert_description_t {
        gnutls_alert_description_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for gnutls_alert_description_t {
    type Output = gnutls_alert_description_t;
    fn mul(self, rhs: u32) -> gnutls_alert_description_t {
        gnutls_alert_description_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for gnutls_alert_description_t {
    type Output = gnutls_alert_description_t;
    fn div(self, rhs: u32) -> gnutls_alert_description_t {
        gnutls_alert_description_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for gnutls_alert_description_t {
    type Output = gnutls_alert_description_t;
    fn rem(self, rhs: u32) -> gnutls_alert_description_t {
        gnutls_alert_description_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            1048576 => C2RustUnnamed_4::GNUTLS_CERT_INVALID_OCSP_STATUS,
            524288 => C2RustUnnamed_4::GNUTLS_CERT_MISSING_OCSP_STATUS,
            262144 => C2RustUnnamed_4::GNUTLS_CERT_PURPOSE_MISMATCH,
            131072 => C2RustUnnamed_4::GNUTLS_CERT_MISMATCH,
            65536 => C2RustUnnamed_4::GNUTLS_CERT_SIGNER_CONSTRAINTS_FAILURE,
            32768 => C2RustUnnamed_4::GNUTLS_CERT_REVOCATION_DATA_ISSUED_IN_FUTURE,
            16384 => C2RustUnnamed_4::GNUTLS_CERT_UNEXPECTED_OWNER,
            4096 => C2RustUnnamed_4::GNUTLS_CERT_REVOCATION_DATA_SUPERSEDED,
            2048 => C2RustUnnamed_4::GNUTLS_CERT_SIGNATURE_FAILURE,
            1024 => C2RustUnnamed_4::GNUTLS_CERT_EXPIRED,
            512 => C2RustUnnamed_4::GNUTLS_CERT_NOT_ACTIVATED,
            256 => C2RustUnnamed_4::GNUTLS_CERT_INSECURE_ALGORITHM,
            128 => C2RustUnnamed_4::GNUTLS_CERT_SIGNER_NOT_CA,
            64 => C2RustUnnamed_4::GNUTLS_CERT_SIGNER_NOT_FOUND,
            32 => C2RustUnnamed_4::GNUTLS_CERT_REVOKED,
            2 => C2RustUnnamed_4::GNUTLS_CERT_INVALID,
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
pub enum gnutls_certificate_type_t {
    GNUTLS_CRT_RAW = 3,
    GNUTLS_CRT_OPENPGP = 2,
    GNUTLS_CRT_X509 = 1,
    GNUTLS_CRT_UNKNOWN = 0,
}
impl gnutls_certificate_type_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            gnutls_certificate_type_t::GNUTLS_CRT_RAW => 3,
            gnutls_certificate_type_t::GNUTLS_CRT_OPENPGP => 2,
            gnutls_certificate_type_t::GNUTLS_CRT_X509 => 1,
            gnutls_certificate_type_t::GNUTLS_CRT_UNKNOWN => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> gnutls_certificate_type_t {
        match value {
            3 => gnutls_certificate_type_t::GNUTLS_CRT_RAW,
            2 => gnutls_certificate_type_t::GNUTLS_CRT_OPENPGP,
            1 => gnutls_certificate_type_t::GNUTLS_CRT_X509,
            0 => gnutls_certificate_type_t::GNUTLS_CRT_UNKNOWN,
            _ => panic!("Invalid value for gnutls_certificate_type_t: {}", value),
        }
    }
}
impl AddAssign<u32> for gnutls_certificate_type_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for gnutls_certificate_type_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for gnutls_certificate_type_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for gnutls_certificate_type_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for gnutls_certificate_type_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for gnutls_certificate_type_t {
    type Output = gnutls_certificate_type_t;
    fn add(self, rhs: u32) -> gnutls_certificate_type_t {
        gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for gnutls_certificate_type_t {
    type Output = gnutls_certificate_type_t;
    fn sub(self, rhs: u32) -> gnutls_certificate_type_t {
        gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for gnutls_certificate_type_t {
    type Output = gnutls_certificate_type_t;
    fn mul(self, rhs: u32) -> gnutls_certificate_type_t {
        gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for gnutls_certificate_type_t {
    type Output = gnutls_certificate_type_t;
    fn div(self, rhs: u32) -> gnutls_certificate_type_t {
        gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for gnutls_certificate_type_t {
    type Output = gnutls_certificate_type_t;
    fn rem(self, rhs: u32) -> gnutls_certificate_type_t {
        gnutls_certificate_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum gnutls_x509_crt_fmt_t {
    GNUTLS_X509_FMT_PEM = 1,
    GNUTLS_X509_FMT_DER = 0,
}
impl gnutls_x509_crt_fmt_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_PEM => 1,
            gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_DER => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> gnutls_x509_crt_fmt_t {
        match value {
            1 => gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_PEM,
            0 => gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_DER,
            _ => panic!("Invalid value for gnutls_x509_crt_fmt_t: {}", value),
        }
    }
}
impl AddAssign<u32> for gnutls_x509_crt_fmt_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for gnutls_x509_crt_fmt_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for gnutls_x509_crt_fmt_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for gnutls_x509_crt_fmt_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for gnutls_x509_crt_fmt_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for gnutls_x509_crt_fmt_t {
    type Output = gnutls_x509_crt_fmt_t;
    fn add(self, rhs: u32) -> gnutls_x509_crt_fmt_t {
        gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for gnutls_x509_crt_fmt_t {
    type Output = gnutls_x509_crt_fmt_t;
    fn sub(self, rhs: u32) -> gnutls_x509_crt_fmt_t {
        gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for gnutls_x509_crt_fmt_t {
    type Output = gnutls_x509_crt_fmt_t;
    fn mul(self, rhs: u32) -> gnutls_x509_crt_fmt_t {
        gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for gnutls_x509_crt_fmt_t {
    type Output = gnutls_x509_crt_fmt_t;
    fn div(self, rhs: u32) -> gnutls_x509_crt_fmt_t {
        gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for gnutls_x509_crt_fmt_t {
    type Output = gnutls_x509_crt_fmt_t;
    fn rem(self, rhs: u32) -> gnutls_x509_crt_fmt_t {
        gnutls_x509_crt_fmt_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type gnutls_transport_ptr_t = *mut libc::c_void;
pub type gnutls_session_t = *mut gnutls_session_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gnutls_datum_t {
    pub data: *mut u8,
    pub size: u32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum gnutls_server_name_type_t {
    GNUTLS_NAME_DNS = 1,
}
impl gnutls_server_name_type_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            gnutls_server_name_type_t::GNUTLS_NAME_DNS => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> gnutls_server_name_type_t {
        match value {
            1 => gnutls_server_name_type_t::GNUTLS_NAME_DNS,
            _ => panic!("Invalid value for gnutls_server_name_type_t: {}", value),
        }
    }
}
impl AddAssign<u32> for gnutls_server_name_type_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for gnutls_server_name_type_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for gnutls_server_name_type_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for gnutls_server_name_type_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for gnutls_server_name_type_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for gnutls_server_name_type_t {
    type Output = gnutls_server_name_type_t;
    fn add(self, rhs: u32) -> gnutls_server_name_type_t {
        gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for gnutls_server_name_type_t {
    type Output = gnutls_server_name_type_t;
    fn sub(self, rhs: u32) -> gnutls_server_name_type_t {
        gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for gnutls_server_name_type_t {
    type Output = gnutls_server_name_type_t;
    fn mul(self, rhs: u32) -> gnutls_server_name_type_t {
        gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for gnutls_server_name_type_t {
    type Output = gnutls_server_name_type_t;
    fn div(self, rhs: u32) -> gnutls_server_name_type_t {
        gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for gnutls_server_name_type_t {
    type Output = gnutls_server_name_type_t;
    fn rem(self, rhs: u32) -> gnutls_server_name_type_t {
        gnutls_server_name_type_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type gnutls_pubkey_t = *mut gnutls_pubkey_st;
pub type gnutls_x509_crt_t = *mut gnutls_x509_crt_int;
pub type gnutls_certificate_credentials_t = *mut gnutls_certificate_credentials_st;
pub type gnutls_free_function = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    WAIT_FOR_READ = 1,
    WAIT_FOR_WRITE = 2,
}
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_5::WAIT_FOR_READ => 1,
            C2RustUnnamed_5::WAIT_FOR_WRITE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_5 {
        match value {
            1 => C2RustUnnamed_5::WAIT_FOR_READ,
            2 => C2RustUnnamed_5::WAIT_FOR_WRITE,
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
pub struct transport_implementation {
    pub reader: Option<
        unsafe extern "C" fn(i32, *mut i8, i32, *mut libc::c_void, libc::c_double) -> i32,
    >,
    pub writer: Option<
        unsafe extern "C" fn(i32, *mut i8, i32, *mut libc::c_void) -> i32,
    >,
    pub poller: Option<
        unsafe extern "C" fn(i32, libc::c_double, i32, *mut libc::c_void) -> i32,
    >,
    pub peeker: Option<
        unsafe extern "C" fn(i32, *mut i8, i32, *mut libc::c_void, libc::c_double) -> i32,
    >,
    pub errstr: Option<unsafe extern "C" fn(i32, *mut libc::c_void) -> *const i8>,
    pub closer: Option<unsafe extern "C" fn(i32, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wgnutls_transport_context {
    pub session: gnutls_session_t,
    pub session_data: *mut gnutls_datum_t,
    pub last_error: i32,
    pub peekbuf: [i8; 512],
    pub peeklen: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_read_timer {
    pub timeout: libc::c_double,
    pub next_timeout: libc::c_double,
    pub timer: *mut ptimer,
    pub timed_out: i32,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn key_type_to_gnutls_type(mut type_0: keyfile_type) -> i32 {
    match type_0 as u32 {
        0 => return gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_PEM as i32,
        1 => return gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_DER as i32,
        _ => {
            abort();
        }
    };
}
static mut ssl_initialized: bool = 0 as i32 != 0;
static mut credentials: gnutls_certificate_credentials_t = 0
    as *const gnutls_certificate_credentials_st
    as *mut gnutls_certificate_credentials_st;
#[no_mangle]
pub unsafe extern "C" fn ssl_init() -> bool {
    let mut ca_directory: *const i8 = 0 as *const i8;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut ncerts: i32 = -(1 as i32);
    let mut rc: i32 = 0;
    if ssl_initialized {
        return 1 as i32 != 0;
    }
    gnutls_global_init();
    gnutls_certificate_allocate_credentials(&mut credentials);
    gnutls_certificate_set_verify_flags(credentials, 0 as i32 as u32);
    if (opt.ca_directory).is_null() {
        ncerts = gnutls_certificate_set_x509_system_trust(credentials);
    }
    if ncerts <= 0 as i32 {
        ncerts = 0 as i32;
        ca_directory = if !(opt.ca_directory).is_null() {
            opt.ca_directory
        } else {
            b"/etc/ssl/certs\0" as *const u8 as *const i8
        };
        dir = opendir(ca_directory);
        if dir.is_null() {
            if !(opt.ca_directory).is_null() && *opt.ca_directory as i32 != 0 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"ERROR: Cannot open directory %s.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    opt.ca_directory,
                );
            }
        } else {
            let mut inode_map: *mut hash_table = hash_table_new(196 as i32, None, None);
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
                let mut ca_file: [i8; 1024] = [0; 1024];
                if snprintf(
                    ca_file.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 1024]>() as u64,
                    b"%s/%s\0" as *const u8 as *const i8,
                    ca_directory,
                    ((*dent).d_name).as_mut_ptr(),
                ) as u32 as u64 >= ::core::mem::size_of::<[i8; 1024]>() as u64
                {
                    continue;
                }
                if stat(ca_file.as_mut_ptr(), &mut st) != 0 as i32 {
                    continue;
                }
                if !(st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
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
                    gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_PEM,
                );
                if rc <= 0 as i32 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"WARNING: Failed to open cert %s: (%d).\n\0" as *const u8
                                as *const i8,
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
        if ncerts < 0 as i32 {
            ncerts = 0 as i32;
        }
        rc = gnutls_certificate_set_x509_trust_file(
            credentials,
            opt.ca_cert,
            gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_PEM,
        );
        if rc <= 0 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"ERROR: Failed to open cert %s: (%d).\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                opt.ca_cert,
                rc,
            );
        } else {
            ncerts += rc;
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Loaded CA certificate '%s'\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                opt.ca_cert,
            );
        }
    }
    if !(opt.crl_file).is_null() {
        rc = gnutls_certificate_set_x509_crl_file(
            credentials,
            opt.crl_file,
            gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_PEM,
        );
        if rc <= 0 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"ERROR: Failed to load CRL file '%s': (%d)\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                opt.crl_file,
                rc,
            );
            return 0 as i32 != 0;
        }
        logprintf(
            log_options::LOG_VERBOSE,
            dcgettext(
                0 as *const i8,
                b"Loaded CRL file '%s'\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            opt.crl_file,
        );
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Certificates loaded: %d\n\0" as *const u8 as *const i8,
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
        let mut type_0: i32 = 0;
        if opt.private_key_type as u32 != opt.cert_type as u32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"ERROR: GnuTLS requires the key and the cert to be of the same type.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        type_0 = key_type_to_gnutls_type(opt.private_key_type);
        gnutls_certificate_set_x509_key_file(
            credentials,
            opt.cert_file,
            opt.private_key,
            gnutls_x509_crt_fmt_t::from_libc_c_uint(type_0 as u32),
        );
    }
    ssl_initialized = 1 as i32 != 0;
    return 1 as i32 != 0;
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
    ssl_initialized = 0 as i32 != 0;
}
unsafe extern "C" fn wgnutls_read_timeout(
    mut fd: i32,
    mut buf: *mut i8,
    mut bufsize: i32,
    mut arg: *mut libc::c_void,
    mut timeout: libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut flags: i32 = 0 as i32;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    let mut ret: i32 = gnutls_record_check_pending((*ctx).session) as i32;
    let mut read_timer: st_read_timer = {
        let mut init = st_read_timer {
            timeout: if timeout == -(1 as i32) as libc::c_double {
                opt.read_timeout
            } else {
                timeout
            },
            next_timeout: 0 as i32 as libc::c_double,
            timer: 0 as *mut ptimer,
            timed_out: 0 as i32,
        };
        init
    };
    if ret != 0 {
        return gnutls_record_recv(
            (*ctx).session,
            buf as *mut libc::c_void,
            (if ret <= bufsize { ret } else { bufsize }) as size_t,
        ) as i32;
    }
    if read_timer.timeout != 0. {
        flags = rpl_fcntl(fd, 3 as i32, 0 as i32);
        if flags < 0 as i32 {
            return flags;
        }
        if rpl_fcntl(fd, 4 as i32, flags | 0o4000 as i32) != 0 {
            return -(1 as i32);
        }
        read_timer.timer = ptimer_new();
        if (read_timer.timer).is_null() {
            ret = -(1 as i32);
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
                if ret == -(37 as i32) {
                    let mut err: i32 = 0;
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"GnuTLS: *** REHANDSHAKE while reading\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    err = _do_handshake((*ctx).session, fd, &mut read_timer);
                    if err != 0 as i32 {
                        ret = err;
                        break;
                    }
                }
                loop {
                    ret = gnutls_record_recv(
                        (*ctx).session,
                        buf as *mut libc::c_void,
                        bufsize as size_t,
                    ) as i32;
                    if ret == -(28 as i32) && !(read_timer.timer).is_null() {
                        let mut err_0: i32 = select_fd(
                            fd,
                            read_timer.next_timeout,
                            C2RustUnnamed_5::WAIT_FOR_READ as i32,
                        );
                        if err_0 <= 0 as i32 {
                            if err_0 == 0 as i32 {
                                read_timer.timed_out = 1 as i32;
                            }
                            break 's_62;
                        } else {
                            read_timer.next_timeout = read_timer.timeout
                                - ptimer_measure(read_timer.timer);
                            if read_timer.next_timeout <= 0 as i32 as libc::c_double {
                                read_timer.timed_out = 1 as i32;
                                break 's_62;
                            }
                        }
                    }
                    if !(ret == -(28 as i32) || ret == -(52 as i32)) {
                        break;
                    }
                }
                if !(ret == -(37 as i32)) {
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
            if rpl_fcntl(fd, 4 as i32, flags) < 0 as i32 {
                return -(1 as i32);
            }
            if read_timer.timed_out != 0 {
                *__errno_location() = 110 as i32;
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn wgnutls_read(
    mut fd: i32,
    mut buf: *mut i8,
    mut bufsize: i32,
    mut arg: *mut libc::c_void,
    mut timeout: libc::c_double,
) -> i32 {
    let mut ret: i32 = 0;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    if (*ctx).peeklen != 0 {
        let mut copysize: i32 = if bufsize <= (*ctx).peeklen {
            bufsize
        } else {
            (*ctx).peeklen
        };
        memcpy(
            buf as *mut libc::c_void,
            ((*ctx).peekbuf).as_mut_ptr() as *const libc::c_void,
            copysize as u64,
        );
        (*ctx).peeklen -= copysize;
        if (*ctx).peeklen != 0 as i32 {
            memmove(
                ((*ctx).peekbuf).as_mut_ptr() as *mut libc::c_void,
                ((*ctx).peekbuf).as_mut_ptr().offset(copysize as isize)
                    as *const libc::c_void,
                (*ctx).peeklen as u64,
            );
        }
        return copysize;
    }
    ret = wgnutls_read_timeout(fd, buf, bufsize, arg, timeout);
    (*ctx).last_error = ret;
    return ret;
}
unsafe extern "C" fn wgnutls_write(
    mut fd: i32,
    mut buf: *mut i8,
    mut bufsize: i32,
    mut arg: *mut libc::c_void,
) -> i32 {
    let mut current_block: u64;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    let mut ret: i32 = (*ctx).last_error;
    if ret == -(37 as i32) {
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"GnuTLS: *** REHANDSHAKE while writing\n\0" as *const u8 as *const i8,
            );
        }
        ret = _do_handshake((*ctx).session, fd, 0 as *mut st_read_timer);
        if ret != 0 as i32 {
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
                ) as i32;
                if !(ret == -(52 as i32) || ret == -(28 as i32)) {
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
    mut fd: i32,
    mut timeout: libc::c_double,
    mut wait_for: i32,
    mut arg: *mut libc::c_void,
) -> i32 {
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    if wait_for & C2RustUnnamed_5::WAIT_FOR_READ as i32 != 0
        && ((*ctx).peeklen != 0 || gnutls_record_check_pending((*ctx).session) != 0)
    {
        return 1 as i32;
    }
    if timeout == -(1 as i32) as libc::c_double {
        timeout = opt.read_timeout;
    }
    return select_fd(fd, timeout, wait_for);
}
unsafe extern "C" fn wgnutls_peek(
    mut fd: i32,
    mut buf: *mut i8,
    mut bufsize: i32,
    mut arg: *mut libc::c_void,
    mut timeout: libc::c_double,
) -> i32 {
    let mut read: i32 = 0 as i32;
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    let mut offset: i32 = if bufsize <= (*ctx).peeklen {
        bufsize
    } else {
        (*ctx).peeklen
    };
    if (*ctx).peeklen != 0 {
        memcpy(
            buf as *mut libc::c_void,
            ((*ctx).peekbuf).as_mut_ptr() as *const libc::c_void,
            offset as u64,
        );
        return offset;
    }
    if bufsize > ::core::mem::size_of::<[i8; 512]>() as u64 as i32 {
        bufsize = ::core::mem::size_of::<[i8; 512]>() as u64 as i32;
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
        if read < 0 as i32 {
            if offset != 0 {
                read = 0 as i32;
            } else {
                return read
            }
        }
        if read > 0 as i32 {
            memcpy(
                ((*ctx).peekbuf).as_mut_ptr().offset(offset as isize)
                    as *mut libc::c_void,
                buf.offset(offset as isize) as *const libc::c_void,
                read as u64,
            );
            (*ctx).peeklen += read;
        }
    }
    return offset + read;
}
unsafe extern "C" fn wgnutls_errstr(
    mut fd: i32,
    mut arg: *mut libc::c_void,
) -> *const i8 {
    let mut ctx: *mut wgnutls_transport_context = arg as *mut wgnutls_transport_context;
    if (*ctx).last_error > 0 as i32
        || ((*ctx).last_error == -(28 as i32) || (*ctx).last_error == -(37 as i32))
            && *__errno_location() == 110 as i32
    {
        return 0 as *const i8;
    }
    return gnutls_strerror((*ctx).last_error);
}
unsafe extern "C" fn wgnutls_close(mut fd: i32, mut arg: *mut libc::c_void) {
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
                        i32,
                        *mut i8,
                        i32,
                        *mut libc::c_void,
                        libc::c_double,
                    ) -> i32,
            ),
            writer: Some(
                wgnutls_write
                    as unsafe extern "C" fn(i32, *mut i8, i32, *mut libc::c_void) -> i32,
            ),
            poller: Some(
                wgnutls_poll
                    as unsafe extern "C" fn(
                        i32,
                        libc::c_double,
                        i32,
                        *mut libc::c_void,
                    ) -> i32,
            ),
            peeker: Some(
                wgnutls_peek
                    as unsafe extern "C" fn(
                        i32,
                        *mut i8,
                        i32,
                        *mut libc::c_void,
                        libc::c_double,
                    ) -> i32,
            ),
            errstr: Some(
                wgnutls_errstr
                    as unsafe extern "C" fn(i32, *mut libc::c_void) -> *const i8,
            ),
            closer: Some(
                wgnutls_close as unsafe extern "C" fn(i32, *mut libc::c_void) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn _do_handshake(
    mut session: gnutls_session_t,
    mut fd: i32,
    mut read_timer: *mut st_read_timer,
) -> i32 {
    let mut flags: i32 = 0 as i32;
    let mut err: i32 = 0;
    let mut next_timeout: libc::c_double = if !read_timer.is_null() {
        (*read_timer).next_timeout
    } else {
        opt.read_timeout
    };
    if read_timer.is_null() && next_timeout != 0. {
        flags = rpl_fcntl(fd, 3 as i32, 0 as i32);
        if flags < 0 as i32 {
            return flags;
        }
        if rpl_fcntl(fd, 4 as i32, flags | 0o4000 as i32) != 0 {
            return -(1 as i32);
        }
    }
    let mut current_block_25: u64;
    loop {
        err = gnutls_handshake(session);
        if err == -(28 as i32) && next_timeout != 0. {
            let mut sel: i32 = 0;
            if gnutls_record_get_direction(session) != 0 {
                sel = C2RustUnnamed_5::WAIT_FOR_WRITE as i32;
            } else {
                sel = C2RustUnnamed_5::WAIT_FOR_READ as i32;
            }
            sel = select_fd(fd, next_timeout, sel);
            if sel <= 0 as i32 {
                if !(sel == 0 as i32) {
                    break;
                }
                if !read_timer.is_null() {
                    current_block_25 = 8558279550452098021;
                } else {
                    *__errno_location() = 110 as i32;
                    err = -(1 as i32);
                    break;
                }
            } else if !read_timer.is_null() {
                (*read_timer).next_timeout = (*read_timer).timeout
                    - ptimer_measure((*read_timer).timer);
                if (*read_timer).next_timeout <= 0 as i32 as libc::c_double {
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
                    err = -(37 as i32);
                    (*read_timer).timed_out = 1 as i32;
                    break;
                }
            }
        } else if err < 0 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"GnuTLS: %s\n\0" as *const u8 as *const i8,
                gnutls_strerror(err),
            );
            if err == -(16 as i32) || err == -(12 as i32) {
                let mut alert: gnutls_alert_description_t = gnutls_alert_get(session);
                let mut str: *const i8 = gnutls_alert_get_name(alert);
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"GnuTLS: received alert [%u]: %s\n\0" as *const u8 as *const i8,
                    alert as u32,
                    if !str.is_null() {
                        str
                    } else {
                        b"(unknown)\0" as *const u8 as *const i8
                    },
                );
            }
        }
        if !(err != 0 && gnutls_error_is_fatal(err) == 0 as i32) {
            break;
        }
    }
    if read_timer.is_null() && next_timeout != 0. {
        if rpl_fcntl(fd, 4 as i32, flags) < 0 as i32 {
            return -(1 as i32);
        }
    }
    return err;
}
unsafe extern "C" fn _sni_hostname(mut hostname: *const i8) -> *const i8 {
    let mut len: size_t = strlen(hostname);
    let mut sni_hostname: *mut i8 = xmemdup(
        hostname as *const libc::c_void,
        len.wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    while len != 0
        && {
            len = len.wrapping_sub(1);
            *sni_hostname.offset(len as isize) as i32 == '.' as i32
        }
    {
        *sni_hostname.offset(len as isize) = 0 as i32 as i8;
    }
    return sni_hostname;
}
unsafe extern "C" fn set_prio_default(mut session: gnutls_session_t) -> i32 {
    let mut err: i32 = -(1 as i32);
    match opt.secure_protocol as u32 {
        0 => {
            err = gnutls_set_default_priority(session);
            gnutls_session_enable_compatibility_mode(session);
        }
        1 | 2 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-TLS-ALL:+VERS-SSL3.0\0" as *const u8 as *const i8,
                0 as *mut *const i8,
            );
        }
        3 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-SSL3.0\0" as *const u8 as *const i8,
                0 as *mut *const i8,
            );
        }
        4 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-SSL3.0:-VERS-TLS1.0\0" as *const u8 as *const i8,
                0 as *mut *const i8,
            );
        }
        5 => {
            err = gnutls_priority_set_direct(
                session,
                b"NORMAL:-VERS-SSL3.0:-VERS-TLS1.0:-VERS-TLS1.1\0" as *const u8
                    as *const i8,
                0 as *mut *const i8,
            );
        }
        6 => {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Your GnuTLS version is too old to support TLS 1.3\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            return -(1 as i32);
        }
        7 => {
            err = gnutls_priority_set_direct(
                session,
                b"PFS:-VERS-SSL3.0\0" as *const u8 as *const i8,
                0 as *mut *const i8,
            );
            if err != 0 as i32 {
                err = gnutls_priority_set_direct(
                    session,
                    b"NORMAL:-RSA:-VERS-SSL3.0\0" as *const u8 as *const i8,
                    0 as *mut *const i8,
                );
            }
        }
        _ => {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"GnuTLS: unimplemented 'secure-protocol' option value %u\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                opt.secure_protocol as u32,
            );
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Please report this issue to bug-wget@gnu.org\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            abort();
        }
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn ssl_connect_wget(
    mut fd: i32,
    mut hostname: *const i8,
    mut continue_session: *mut i32,
) -> bool {
    let mut ctx: *mut wgnutls_transport_context = 0 as *mut wgnutls_transport_context;
    let mut session: gnutls_session_t = 0 as *mut gnutls_session_int;
    let mut err: i32 = 0;
    gnutls_init(&mut session, ((1 as i32) << 1 as i32) as u32);
    if !is_valid_ip_address(hostname) {
        let mut sni_hostname: *const i8 = _sni_hostname(hostname);
        gnutls_server_name_set(
            session,
            gnutls_server_name_type_t::GNUTLS_NAME_DNS,
            sni_hostname as *const libc::c_void,
            strlen(sni_hostname),
        );
        rpl_free(sni_hostname as *mut libc::c_void);
        sni_hostname = 0 as *const i8;
    }
    gnutls_credentials_set(
        session,
        gnutls_credentials_type_t::GNUTLS_CRD_CERTIFICATE,
        credentials as *mut libc::c_void,
    );
    gnutls_transport_set_ptr(session, fd as intptr_t as gnutls_transport_ptr_t);
    if (opt.tls_ciphers_string).is_null() {
        err = set_prio_default(session);
    } else {
        err = gnutls_priority_set_direct(
            session,
            opt.tls_ciphers_string,
            0 as *mut *const i8,
        );
    }
    if err < 0 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            b"GnuTLS: %s\n\0" as *const u8 as *const i8,
            gnutls_strerror(err),
        );
        gnutls_deinit(session);
        return 0 as i32 != 0;
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
                return 0 as i32 != 0;
            }
        } else {
            logputs(
                log_options::LOG_ALWAYS,
                b"SSL session has already been resumed. Continuing.\n\0" as *const u8
                    as *const i8,
            );
            continue_session = 0 as *mut i32;
        }
    }
    err = _do_handshake(session, fd, 0 as *mut st_read_timer);
    if err < 0 as i32 {
        gnutls_deinit(session);
        return 0 as i32 != 0;
    }
    ctx = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<wgnutls_transport_context>() as u64,
    ) as *mut wgnutls_transport_context;
    (*ctx).session_data = xcalloc(
        1 as i32 as size_t,
        ::core::mem::size_of::<gnutls_datum_t>() as u64,
    ) as *mut gnutls_datum_t;
    (*ctx).session = session;
    if gnutls_session_get_data2(session, (*ctx).session_data) != 0 {
        rpl_free((*ctx).session_data as *mut libc::c_void);
        (*ctx).session_data = 0 as *mut gnutls_datum_t;
        logprintf(
            log_options::LOG_NOTQUIET,
            b"WARNING: Could not save SSL session data for socket %d\n\0" as *const u8
                as *const i8,
            fd,
        );
    }
    fd_register_transport(fd, &mut wgnutls_transport, ctx as *mut libc::c_void);
    return 1 as i32 != 0;
}
unsafe extern "C" fn pkp_pin_peer_pubkey(
    mut cert: gnutls_x509_crt_t,
    mut pinnedpubkey: *const i8,
) -> bool {
    let mut len1: size_t = 0 as i32 as size_t;
    let mut len2: size_t = 0 as i32 as size_t;
    let mut buff1: *mut i8 = 0 as *mut i8;
    let mut key: gnutls_pubkey_t = 0 as gnutls_pubkey_t;
    let mut ret: i32 = 0 as i32;
    let mut result: bool = 0 as i32 != 0;
    if pinnedpubkey.is_null() {
        return 1 as i32 != 0;
    }
    if cert.is_null() {
        return result;
    }
    gnutls_pubkey_init(&mut key);
    ret = gnutls_pubkey_import_x509(key, cert, 0 as i32 as u32);
    if !(ret < 0 as i32) {
        ret = gnutls_pubkey_export(
            key,
            gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_DER,
            0 as *mut libc::c_void,
            &mut len1,
        );
        if !(ret != -(51 as i32) || len1 == 0 as i32 as u64) {
            buff1 = xmalloc(len1) as *mut i8;
            len2 = len1;
            ret = gnutls_pubkey_export(
                key,
                gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_DER,
                buff1 as *mut libc::c_void,
                &mut len2,
            );
            if !(ret < 0 as i32 || len1 != len2) {
                result = wg_pin_peer_pubkey(pinnedpubkey, buff1, len1);
            }
        }
    }
    if !key.is_null() {
        gnutls_pubkey_deinit(key);
    }
    rpl_free(buff1 as *mut libc::c_void);
    buff1 = 0 as *mut i8;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ssl_check_certificate(
    mut fd: i32,
    mut host: *const i8,
) -> bool {
    let mut ctx: *mut wgnutls_transport_context = fd_transport_context(fd)
        as *mut wgnutls_transport_context;
    let mut status: u32 = 0;
    let mut err: i32 = 0;
    let mut severity: *const i8 = if opt.check_cert != 0 {
        dcgettext(0 as *const i8, b"ERROR\0" as *const u8 as *const i8, 5 as i32)
    } else {
        dcgettext(0 as *const i8, b"WARNING\0" as *const u8 as *const i8, 5 as i32)
    };
    let mut success: bool = 1 as i32 != 0;
    let mut pinsuccess: bool = (opt.pinnedpubkey).is_null();
    if opt.check_cert == CHECK_CERT_MODES::CHECK_CERT_QUIET as i32
        && pinsuccess as i32 != 0
    {
        return success;
    }
    err = gnutls_certificate_verify_peers2((*ctx).session, &mut status);
    if err < 0 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"%s: No certificate presented by %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            severity,
            quotearg_style(quoting_style::escape_quoting_style, host),
        );
        success = 0 as i32 != 0;
    } else {
        if status & C2RustUnnamed_4::GNUTLS_CERT_INVALID as i32 as u32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: The certificate of %s is not trusted.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                severity,
                quote(host),
            );
            success = 0 as i32 != 0;
        }
        if status & C2RustUnnamed_4::GNUTLS_CERT_SIGNER_NOT_FOUND as i32 as u32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: The certificate of %s doesn't have a known issuer.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                severity,
                quote(host),
            );
            success = 0 as i32 != 0;
        }
        if status & C2RustUnnamed_4::GNUTLS_CERT_REVOKED as i32 as u32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: The certificate of %s has been revoked.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                severity,
                quote(host),
            );
            success = 0 as i32 != 0;
        }
        if status & C2RustUnnamed_4::GNUTLS_CERT_SIGNER_NOT_CA as i32 as u32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: The certificate signer of %s was not a CA.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                severity,
                quote(host),
            );
            success = 0 as i32 != 0;
        }
        if status & C2RustUnnamed_4::GNUTLS_CERT_INSECURE_ALGORITHM as i32 as u32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: The certificate of %s was signed using an insecure algorithm.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                severity,
                quote(host),
            );
            success = 0 as i32 != 0;
        }
        if status & C2RustUnnamed_4::GNUTLS_CERT_NOT_ACTIVATED as i32 as u32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: The certificate of %s is not yet activated.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                severity,
                quote(host),
            );
            success = 0 as i32 != 0;
        }
        if status & C2RustUnnamed_4::GNUTLS_CERT_EXPIRED as i32 as u32 != 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: The certificate of %s has expired.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                severity,
                quote(host),
            );
            success = 0 as i32 != 0;
        }
        if gnutls_certificate_type_get((*ctx).session) as u32
            == gnutls_certificate_type_t::GNUTLS_CRT_X509 as i32 as u32
        {
            let mut now: time_t = time(0 as *mut time_t);
            let mut cert: gnutls_x509_crt_t = 0 as *mut gnutls_x509_crt_int;
            let mut cert_list: *const gnutls_datum_t = 0 as *const gnutls_datum_t;
            let mut cert_list_size: u32 = 0;
            let mut sni_hostname: *const i8 = 0 as *const i8;
            err = gnutls_x509_crt_init(&mut cert);
            if err < 0 as i32 {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Error initializing X509 certificate: %s\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    gnutls_strerror(err),
                );
                success = 0 as i32 != 0;
            } else {
                cert_list = gnutls_certificate_get_peers(
                    (*ctx).session,
                    &mut cert_list_size,
                );
                if cert_list.is_null() {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        dcgettext(
                            0 as *const i8,
                            b"No certificate found\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    success = 0 as i32 != 0;
                } else {
                    err = gnutls_x509_crt_import(
                        cert,
                        cert_list,
                        gnutls_x509_crt_fmt_t::GNUTLS_X509_FMT_DER,
                    );
                    if err < 0 as i32 {
                        logprintf(
                            log_options::LOG_NOTQUIET,
                            dcgettext(
                                0 as *const i8,
                                b"Error parsing certificate: %s\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            gnutls_strerror(err),
                        );
                        success = 0 as i32 != 0;
                    } else {
                        if now < gnutls_x509_crt_get_activation_time(cert) {
                            logprintf(
                                log_options::LOG_NOTQUIET,
                                dcgettext(
                                    0 as *const i8,
                                    b"The certificate has not yet been activated\n\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            success = 0 as i32 != 0;
                        }
                        if now >= gnutls_x509_crt_get_expiration_time(cert) {
                            logprintf(
                                log_options::LOG_NOTQUIET,
                                dcgettext(
                                    0 as *const i8,
                                    b"The certificate has expired\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                            success = 0 as i32 != 0;
                        }
                        sni_hostname = _sni_hostname(host);
                        if gnutls_x509_crt_check_hostname(cert, sni_hostname) == 0 {
                            logprintf(
                                log_options::LOG_NOTQUIET,
                                dcgettext(
                                    0 as *const i8,
                                    b"The certificate's owner does not match hostname %s\n\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                quote(sni_hostname),
                            );
                            success = 0 as i32 != 0;
                        }
                        rpl_free(sni_hostname as *mut libc::c_void);
                        sni_hostname = 0 as *const i8;
                        pinsuccess = pkp_pin_peer_pubkey(cert, opt.pinnedpubkey);
                        if !pinsuccess {
                            logprintf(
                                log_options::LOG_ALWAYS,
                                dcgettext(
                                    0 as *const i8,
                                    b"The public key does not match pinned public key!\n\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            success = 0 as i32 != 0;
                        }
                    }
                }
                gnutls_x509_crt_deinit(cert);
            }
        } else {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Certificate must be X.509\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            success = 0 as i32 != 0;
        }
    }
    return if !pinsuccess {
        0 as i32
    } else if opt.check_cert == CHECK_CERT_MODES::CHECK_CERT_ON as i32 {
        success as i32
    } else {
        1 as i32
    } != 0;
}