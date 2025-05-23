use std::cmp::Ordering;
use std::env;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::str::FromStr;
use std::time::Duration;

use libc::{c_char, c_double, c_int, c_long, c_ulong, c_void, FILE, size_t, ssize_t};
use nix::unistd::getuid;
use nix::pwd::Passwd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckCertModes {
    Off = 0,
    On = 1,
    Quiet = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionOptions {
    Auto = 0,
    Gzip = 1,
    None = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferFamily {
    Ipv4 = 0,
    Ipv6 = 1,
    None = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictCase {
    NoRestriction = 0,
    Lowercase = 1,
    Uppercase = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictOs {
    Unix = 0,
    Vms = 1,
    Windows = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyfileType {
    Pem = 0,
    Asn1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureProtocol {
    Auto = 0,
    Sslv2 = 1,
    Sslv3 = 2,
    Tlsv1 = 3,
    Tlsv1_1 = 4,
    Tlsv1_2 = 5,
    Tlsv1_3 = 6,
    Pfs = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegexType {
    Pcre = 0,
    Posix = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseLine {
    Ok = 0,
    Empty = 1,
    SyntaxError = 2,
    UnknownCommand = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UerrT {
    NoconError = 0,
    HostErr = 1,
    ConsockErr = 2,
    ConError = 3,
    ConsslErr = 4,
    ConImpossible = 5,
    NewLocation = 6,
    FtpOk = 7,
    FtpLogin = 8,
    FtpLogRefused = 9,
    FtpPortErr = 10,
    FtpsysErr = 11,
    Ftpnsfod = 12,
    FtpUnknownType = 13,
    FtpRerr = 14,
    FtpsrvErr = 15,
    FtpRetrInt = 16,
    FtpRestFail = 17,
    UrlError = 18,
    FopenErr = 19,
    FopenExclErr = 20,
    FwriteErr = 21,
    Heof = 22,
    GatewayTimeout = 23,
    Herr = 24,
    RetrOk = 25,
    RecLevelExc = 26,
    WrongCode = 27,
    FtpInvPasv = 28,
    FtpNoPasv = 29,
    FtpNoPbsz = 30,
    FtpNoProt = 31,
    FtpNoAuth = 32,
    ContNotSupported = 33,
    RetrUnneeded = 34,
    RetrFinished = 35,
    ReadErr = 36,
    TryLimExc = 37,
    FileBadFile = 38,
    RangeErr = 39,
    RetrBadPattern = 40,
    ProxErr = 41,
    AuthFailed = 42,
    QuoteExc = 43,
    WriteFailed = 44,
    SslInitFailed = 45,
    VerifCertErr = 46,
    UnlinkErr = 47,
    NewLocationKeepPost = 48,
    CloseFailed = 49,
    AttrMissing = 50,
    UnknownAttr = 51,
    WarcErr = 52,
    WarcTmpFopenErr = 53,
    WarcTmpFwriteErr = 54,
    TimeconvErr = 55,
    MetalinkParseError = 56,
    MetalinkRetrError = 57,
    MetalinkChksumError = 58,
    MetalinkSigError = 59,
    MetalinkMissingResource = 60,
    RetrWithMetalink = 61,
    MetalinkSizeError = 62,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgetExit {
    Success = 0,
    GenericError = 1,
    ParseError = 2,
    IoFail = 3,
    NetworkFail = 4,
    SslAuthFail = 5,
    ServerAuthFail = 6,
    ProtocolError = 7,
    ServerError = 8,
    Unknown = 9,
}

#[derive(Debug)]
pub struct FileStat {
    access_err: i32,
    st_ino: u64,
    st_dev: u64,
}

#[derive(Debug)]
pub struct DecodeItem {
    name: &'static str,
    code: i32,
}

#[derive(Debug)]
pub struct Command {
    name: &'static str,
    place: *mut c_void,
    action: Option<fn(&str, &str, *mut c_void) -> bool>,
}

#[derive(Debug)]
pub struct Options {
    pub verbose: i32,
    pub quiet: bool,
    pub ntry: i32,
    pub retry_connrefused: bool,
    pub retry_on_host_error: bool,
    pub retry_on_http_error: Option<String>,
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
    pub dir_prefix: Option<String>,
    pub lfilename: Option<String>,
    pub input_filename: Option<String>,
    pub choose_config: Option<String>,
    pub noconfig: bool,
    pub force_html: bool,
    pub default_page: Option<String>,
    pub spider: bool,
    pub accepts: Vec<String>,
    pub rejects: Vec<String>,
    pub excludes: Vec<String>,
    pub includes: Vec<String>,
    pub ignore_case: bool,
    pub acceptregex_s: Option<String>,
    pub rejectregex_s: Option<String>,
    pub acceptregex: Option<*mut c_void>,
    pub rejectregex: Option<*mut c_void>,
    pub regex_type: RegexType,
    pub regex_compile_fun: Option<unsafe extern "C" fn(*const c_char) -> *mut c_void>,
    pub regex_match_fun: Option<unsafe extern "C" fn(*const c_void, *const c_char) -> bool>,
    pub domains: Vec<String>,
    pub exclude_domains: Vec<String>,
    pub dns_cache: bool,
    pub follow_tags: Vec<String>,
    pub ignore_tags: Vec<String>,
    pub follow_ftp: bool,
    pub retr_symlinks: bool,
    pub output_document: Option<String>,
    pub warc_filename: Option<String>,
    pub warc_tempdir: Option<String>,
    pub warc_cdx_dedup_filename: Option<String>,
    pub warc_maxsize: i64,
    pub warc_compression_enabled: bool,
    pub warc_digests_enabled: bool,
    pub warc_cdx_enabled: bool,
    pub warc_keep_log: bool,
    pub warc_user_headers: Vec<String>,
    pub enable_xattr: bool,
    pub user: Option<String>,
    pub passwd: Option<String>,
    pub ask_passwd: bool,
    pub use_askpass: Option<String>,
    pub always_rest: bool,
    pub start_pos: i64,
    pub ftp_user: Option<String>,
    pub ftp_passwd: Option<String>,
    pub netrc: bool,
    pub ftp_glob: bool,
    pub ftp_pasv: bool,
    pub http_user: Option<String>,
    pub http_passwd: Option<String>,
    pub user_headers: Vec<String>,
    pub http_keep_alive: bool,
    pub use_proxy: bool,
    pub allow_cache: bool,
    pub http_proxy: Option<String>,
    pub ftp_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Vec<String>,
    pub base_href: Option<String>,
    pub progress_type: Option<String>,
    pub show_progress: i32,
    pub noscroll: bool,
    pub proxy_user: Option<String>,
    pub proxy_passwd: Option<String>,
    pub read_timeout: f64,
    pub dns_timeout: f64,
    pub connect_timeout: f64,
    pub random_wait: bool,
    pub wait: f64,
    pub waitretry: f64,
    pub use_robots: bool,
    pub limit_rate: i64,
    pub quota: i64,
    pub server_response: bool,
    pub save_headers: bool,
    pub content_on_error: bool,
    pub debug: bool,
    pub timestamping: bool,
    pub if_modified_since: bool,
    pub backup_converted: bool,
    pub backups: i32,
    pub useragent: Option<String>,
    pub referer: Option<String>,
    pub convert_links: bool,
    pub convert_file_only: bool,
    pub remove_listing: bool,
    pub htmlify: bool,
    pub dot_style: Option<String>,
    pub dot_bytes: i64,
    pub dots_in_line: i32,
    pub dot_spacing: i32,
    pub delete_after: bool,
    pub adjust_extension: bool,
    pub page_requisites: bool,
    pub bind_address: Option<String>,
    pub secure_protocol: SecureProtocol,
    pub secure_protocol_name: [c_char; 8],
    pub check_cert: i32,
    pub cert_file: Option<String>,
    pub private_key: Option<String>,
    pub cert_type: KeyfileType,
    pub private_key_type: KeyfileType,
    pub ca_directory: Option<String>,
    pub ca_cert: Option<String>,
    pub crl_file: Option<String>,
    pub pinnedpubkey: Option<String>,
    pub random_file: Option<String>,
    pub egd_file: Option<String>,
    pub https_only: bool,
    pub ftps_resume_ssl: bool,
    pub ftps_fallback_to_ftp: bool,
    pub ftps_implicit: bool,
    pub ftps_clear_data_connection: bool,
    pub tls_ciphers_string: Option<String>,
    pub cookies: bool,
    pub cookies_input: Option<String>,
    pub cookies_output: Option<String>,
    pub keep_badhash: bool,
    pub keep_session_cookies: bool,
    pub post_data: Option<String>,
    pub post_file_name: Option<String>,
    pub method: Option<String>,
    pub body_data: Option<String>,
    pub body_file: Option<String>,
    pub restrict_files_os: RestrictOs,
    pub restrict_files_ctrl: bool,
    pub restrict_files_nonascii: bool,
    pub restrict_files_case: RestrictCase,
    pub strict_comments: bool,
    pub preserve_perm: bool,
    pub ipv4_only: bool,
    pub ipv6_only: bool,
    pub prefer_family: PreferFamily,
    pub content_disposition: bool,
    pub auth_without_challenge: bool,
    pub enable_iri: bool,
    pub encoding_remote: Option<String>,
    pub locale: Option<String>,
    pub trustservernames: bool,
    pub useservertimestamps: bool,
    pub show_all_dns_entries: bool,
    pub report_bps: bool,
    pub compression: CompressionOptions,
    pub rejected_log: Option<String>,
    pub hsts: bool,
    pub hsts_file: Option<String>,
    pub homedir: Option<String>,
    pub wgetrcfile: Option<String>,
}

static mut OPT: Options = Options {
    verbose: -1,
    quiet: false,
    ntry: 20,
    retry_connrefused: false,
    retry_on_host_error: false,
    retry_on_http_error: None,
    background: false,
    ignore_length: false,
    recursive: false,
    spanhost: false,
    max_redirect: 20,
    relative_only: false,
    no_parent: false,
    reclevel: 5,
    dirstruct: false,
    no_dirstruct: false,
    cut_dirs: 0,
    add_hostdir: true,
    protocol_directories: false,
    noclobber: false,
    unlink_requested: false,
    dir_prefix: None,
    lfilename: None,
    input_filename: None,
    choose_config: None,
    noconfig: false,
    force_html: false,
    default_page: None,
    spider: false,
    accepts: Vec::new(),
    rejects: Vec::new(),
    excludes: Vec::new(),
    includes: Vec::new(),
    ignore_case: false,
    acceptregex_s: None,
    rejectregex_s: None,
    acceptregex: None,
    rejectregex: None,
    regex_type: RegexType::Posix,
    regex_compile_fun: None,
    regex_match_fun: None,
    domains: Vec::new(),
    exclude_domains: Vec::new(),
    dns_cache: true,
    follow_tags: Vec::new(),
    ignore_tags: Vec::new(),
    follow_ftp: false,
    retr_symlinks: true,
    output_document: None,
    warc_filename: None,
    warc_tempdir: None,
    warc_cdx_dedup_filename: None,
    warc_maxsize: 0,
    warc_compression_enabled: true,
    warc_digests_enabled: true,
    warc_cdx_enabled: false,
    warc_keep_log: true,
    warc_user_headers: Vec::new(),
    enable_xattr: false,
    user: None,
    passwd: None,
    ask_passwd: false,
    use_askpass: None,
    always_rest: false,
    start_pos: -1,
    ftp_user: None,
    ftp_passwd: None,
    netrc: true,
    ftp_glob: true,
    ftp_pasv: true,
    http_user: None,
    http_passwd: None,
    user_headers: Vec::new(),
    http_keep_alive: true,
    use_proxy: true,
    allow_cache: true,
    http_proxy: None,
    ftp_proxy: None,
    https_proxy: None,
    no_proxy: Vec::new(),
    base_href: None,
    progress_type: None,
    show_progress: -1,
    noscroll: false,
    proxy_user: None,
    proxy_passwd: None,
    read_timeout: 900.0,
    dns_timeout: 900.0,
    connect_timeout: 900.0,
    random_wait: false,
    wait: 0.0,
    waitretry: 10.0,
    use_robots: true,
    limit_rate: 0,
    quota: 0,
    server_response: false,
    save_headers: false,
    content_on_error: false,
    debug: false,
    timestamping: false,
    if_modified_since: true,
    backup_converted: false,
    backups: 0,
    useragent: None,
    referer: None,
    convert_links: false,
    convert_file_only: false,
    remove_listing: true,
    htmlify: true,
    dot_style: None,
    dot_bytes: 1024,
    dots_in_line: 50,
    dot_spacing: 10,
    delete_after: false,
    adjust_extension: false,
    page_requisites: false,
    bind_address: None,
    secure_protocol: SecureProtocol::Auto,
    secure_protocol_name: [0; 8],
    check_cert: CheckCertModes::On as i32,
    cert_file: None,
    private_key: None,
    cert_type: KeyfileType::Pem,
    private_key_type: KeyfileType::Pem,
    ca_directory: None,
    ca_cert: None,
    crl_file: None,
    pinnedpubkey: None,
    random_file: None,
    egd_file: None,
    https_only: false,
    ftps_resume_ssl: true,
    ftps_fallback_to_ftp: false,
    ftps_implicit: false,
    ftps_clear_data_connection: false,
    tls_ciphers_string: None,
    cookies: true,
    cookies_input: None,
    cookies_output: None,
    keep_badhash: false,
    keep_session_cookies: false,
    post_data: None,
    post_file_name: None,
    method: None,
    body_data: None,
    body_file: None,
    restrict_files_os: RestrictOs::Unix,
    restrict_files_ctrl: true,
    restrict_files_nonascii: false,
    restrict_files_case: RestrictCase::NoRestriction,
    strict_comments: false,
    preserve_perm: false,
    ipv4_only: false,
    ipv6_only: false,
    prefer_family: PreferFamily::None,
    content_disposition: false,
    auth_without_challenge: false,
    enable_iri: true,
    encoding_remote: None,
    locale: None,
    trustservernames: false,
   