#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
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
    fn idn2_lookup_u8(
        src: *const uint8_t,
        lookupname: *mut *mut uint8_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn idn2_strerror(rc: libc::c_int) -> *const libc::c_char;
    fn idn2_free(ptr: *mut libc::c_void);
    fn strdupdelim(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn url_unescape_except_reserved(_: *mut libc::c_char);
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn c_strcasestr(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xstrndup(string: *const libc::c_char, n: size_t) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
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
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    prefer_none = 2,
    prefer_ipv6 = 1,
    prefer_ipv4 = 0,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    restrict_uppercase = 2,
    restrict_lowercase = 1,
    restrict_no_case_restriction = 0,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    restrict_windows = 2,
    restrict_vms = 1,
    restrict_unix = 0,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum keyfile_type {
    keyfile_asn1 = 1,
    keyfile_pem = 0,
}  // end of enum

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
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    regex_type_posix = 1,
    regex_type_pcre = 0,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_options {
    LOG_VERBOSE,
    LOG_NOTQUIET,
    LOG_NONVERBOSE,
    LOG_ALWAYS,
    LOG_PROGRESS,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct iri {
    pub uri_encoding: *mut libc::c_char,
    pub content_encoding: *mut libc::c_char,
    pub orig_url: *mut libc::c_char,
    pub utf8_encode: bool,
}
pub const CODESET: C2RustUnnamed_4 = 14;
pub type nl_item = libc::c_int;
pub type iconv_t = *mut libc::c_void;
pub const IDN2_OK: C2RustUnnamed_6 = 0;
pub const IDN2_TRANSITIONAL: C2RustUnnamed_5 = 4;
pub const IDN2_NONTRANSITIONAL: C2RustUnnamed_5 = 8;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
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
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    IDN2_TRANSITIONAL = 4,
    IDN2_NONTRANSITIONAL = 8,
    IDN2_NO_ALABEL_ROUNDTRIP = 128,
    IDN2_NO_TR46 = 64,
    IDN2_USE_STD3_ASCII_RULES = 32,
    IDN2_ALLOW_UNASSIGNED = 16,
    IDN2_ALABEL_ROUNDTRIP = 2,
    IDN2_NFC_INPUT = 1,
}  // end of enum

pub type C2RustUnnamed_6 = libc::c_int;
pub const IDN2_ALABEL_ROUNDTRIP_FAILED: C2RustUnnamed_6 = -314;
pub const IDN2_INVALID_NONTRANSITIONAL: C2RustUnnamed_6 = -313;
pub const IDN2_INVALID_TRANSITIONAL: C2RustUnnamed_6 = -312;
pub const IDN2_DOT_IN_LABEL: C2RustUnnamed_6 = -311;
pub const IDN2_BIDI: C2RustUnnamed_6 = -310;
pub const IDN2_UNASSIGNED: C2RustUnnamed_6 = -309;
pub const IDN2_CONTEXTO_NO_RULE: C2RustUnnamed_6 = -308;
pub const IDN2_CONTEXTO: C2RustUnnamed_6 = -307;
pub const IDN2_CONTEXTJ_NO_RULE: C2RustUnnamed_6 = -306;
pub const IDN2_CONTEXTJ: C2RustUnnamed_6 = -305;
pub const IDN2_DISALLOWED: C2RustUnnamed_6 = -304;
pub const IDN2_LEADING_COMBINING: C2RustUnnamed_6 = -303;
pub const IDN2_HYPHEN_STARTEND: C2RustUnnamed_6 = -302;
pub const IDN2_2HYPHEN: C2RustUnnamed_6 = -301;
pub const IDN2_NOT_NFC: C2RustUnnamed_6 = -300;
pub const IDN2_INVALID_FLAGS: C2RustUnnamed_6 = -209;
pub const IDN2_UALABEL_MISMATCH: C2RustUnnamed_6 = -208;
pub const IDN2_INVALID_ALABEL: C2RustUnnamed_6 = -207;
pub const IDN2_TOO_BIG_LABEL: C2RustUnnamed_6 = -206;
pub const IDN2_TOO_BIG_DOMAIN: C2RustUnnamed_6 = -205;
pub const IDN2_PUNYCODE_OVERFLOW: C2RustUnnamed_6 = -204;
pub const IDN2_PUNYCODE_BIG_OUTPUT: C2RustUnnamed_6 = -203;
pub const IDN2_PUNYCODE_BAD_INPUT: C2RustUnnamed_6 = -202;
pub const IDN2_NFC: C2RustUnnamed_6 = -201;
pub const IDN2_ENCODING_ERROR: C2RustUnnamed_6 = -200;
pub const IDN2_ICONV_FAIL: C2RustUnnamed_6 = -102;
pub const IDN2_NO_CODESET: C2RustUnnamed_6 = -101;
pub const IDN2_MALLOC: C2RustUnnamed_6 = -100;
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
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_charset(
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut charset: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() || *str == 0 {
        return 0 as *mut libc::c_char;
    }
    str = c_strcasestr(str, b"charset=\0" as *const u8 as *const libc::c_char);
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    str = str.offset(8 as libc::c_int as isize);
    end = str;
    while *end as libc::c_int != 0 && !c_isspace(*end as libc::c_int) {
        end = end.offset(1);
        end;
    }
    charset = strdupdelim(str, end);
    if !check_encoding_name(charset) {
        rpl_free(charset as *mut libc::c_void);
        charset = 0 as *mut libc::c_char;
        return 0 as *mut libc::c_char;
    }
    return charset;
}
#[no_mangle]
pub unsafe extern "C" fn find_locale() -> *const libc::c_char {
    let mut encoding: *const libc::c_char = nl_langinfo(CODESET as libc::c_int);
    if encoding.is_null() || *encoding == 0 {
        return xstrdup(b"ASCII\0" as *const u8 as *const libc::c_char);
    }
    return xstrdup(encoding);
}
#[no_mangle]
pub unsafe extern "C" fn check_encoding_name(mut encoding: *const libc::c_char) -> bool {
    let mut s: *const libc::c_char = encoding;
    while *s != 0 {
        if !c_isascii(*s as libc::c_int)
            || c_isspace(*s as libc::c_int) as libc::c_int != 0
        {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Encoding %s isn't valid\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(encoding),
            );
            return 0 as libc::c_int != 0;
        }
        s = s.offset(1);
        s;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn do_conversion(
    mut tocode: *const libc::c_char,
    mut fromcode: *const libc::c_char,
    mut in_org: *const libc::c_char,
    mut inlen: size_t,
    mut out: *mut *mut libc::c_char,
) -> bool {
    let mut cd: iconv_t = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut done: size_t = 0;
    let mut outlen: size_t = 0;
    let mut invalid: libc::c_int = 0 as libc::c_int;
    let mut tooshort: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_save: *mut libc::c_char = 0 as *mut libc::c_char;
    cd = iconv_open(tocode, fromcode);
    if cd == -(1 as libc::c_int) as iconv_t {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"Conversion from %s to %s isn't supported\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(0 as libc::c_int, fromcode),
            quote_n(1 as libc::c_int, tocode),
        );
        *out = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    }
    in_0 = xstrndup(in_org, inlen);
    in_save = in_0;
    url_unescape_except_reserved(in_0);
    inlen = strlen(in_0);
    outlen = inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    len = outlen;
    s = xmalloc(outlen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    *out = s;
    done = 0 as libc::c_int as size_t;
    loop {
        if iconv(cd, &mut in_0 as *mut *mut libc::c_char, &mut inlen, out, &mut outlen)
            != -(1 as libc::c_int) as size_t
            && iconv(cd, 0 as *mut *mut libc::c_char, 0 as *mut size_t, out, &mut outlen)
                != -(1 as libc::c_int) as size_t
        {
            *out = s;
            *s
                .offset(len as isize)
                .offset(-(outlen as isize))
                .offset(-(done as isize)) = '\0' as i32 as libc::c_char;
            rpl_free(in_save as *mut libc::c_void);
            in_save = 0 as *mut libc::c_char;
            iconv_close(cd);
            if opt.debug as libc::c_long != 0 {
                if (strchr(in_org, '@' as i32)).is_null()
                    && (strchr(*out, '@' as i32)).is_null()
                {
                    debug_logprintf(
                        b"converted '%s' (%s) -> '%s' (%s)\n\0" as *const u8
                            as *const libc::c_char,
                        in_org,
                        fromcode,
                        *out,
                        tocode,
                    );
                } else {
                    debug_logprintf(
                        b"logging suppressed, strings may contain password\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
            return 1 as libc::c_int != 0;
        }
        if *__errno_location() == 22 as libc::c_int
            || *__errno_location() == 84 as libc::c_int
        {
            if invalid == 0 {
                logprintf(
                    LOG_VERBOSE,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Incomplete or invalid multibyte sequence encountered\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            invalid += 1;
            invalid;
            **out = *in_0;
            in_0 = in_0.offset(1);
            in_0;
            inlen = inlen.wrapping_sub(1);
            inlen;
            *out = (*out).offset(1);
            *out;
            outlen = outlen.wrapping_sub(1);
            outlen;
        } else if *__errno_location() == 7 as libc::c_int {
            tooshort += 1;
            tooshort;
            done = len;
            len = done
                .wrapping_add(inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong));
            s = xrealloc(
                s as *mut libc::c_void,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            *out = s.offset(done as isize).offset(-(outlen as isize));
            outlen = (outlen as libc::c_ulong)
                .wrapping_add(inlen.wrapping_mul(2 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
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
            break;
        }
    }
    rpl_free(in_save as *mut libc::c_void);
    in_save = 0 as *mut libc::c_char;
    iconv_close(cd);
    if opt.debug as libc::c_long != 0 {
        if (strchr(in_org, '@' as i32)).is_null() && (strchr(*out, '@' as i32)).is_null()
        {
            debug_logprintf(
                b"converted '%s' (%s) -> '%s' (%s)\n\0" as *const u8
                    as *const libc::c_char,
                in_org,
                fromcode,
                *out,
                tocode,
            );
        } else {
            debug_logprintf(
                b"logging suppressed, strings may contain password\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn locale_to_utf8(
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    if (opt.locale).is_null() {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"locale_to_utf8: locale is unset\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        opt.locale = find_locale();
    }
    if (opt.locale).is_null()
        || c_strcasecmp(opt.locale, b"utf-8\0" as *const u8 as *const libc::c_char) == 0
    {
        return str;
    }
    if do_conversion(
        b"UTF-8\0" as *const u8 as *const libc::c_char,
        opt.locale,
        str as *mut libc::c_char,
        strlen(str as *mut libc::c_char),
        &mut new,
    ) {
        return new as *const libc::c_char;
    }
    rpl_free(new as *mut libc::c_void);
    new = 0 as *mut libc::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn idn_encode(
    mut i: *const iri,
    mut host: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ret: libc::c_int = 0;
    let mut ascii_encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut utf8_encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    if !(*i).utf8_encode {
        if !remote_to_utf8(i, host, &mut utf8_encoded) {
            return 0 as *mut libc::c_char;
        }
        src = utf8_encoded;
    } else {
        src = host;
    }
    ret = idn2_lookup_u8(
        src as *mut uint8_t,
        &mut ascii_encoded as *mut *mut libc::c_char as *mut *mut uint8_t,
        IDN2_NONTRANSITIONAL as libc::c_int,
    );
    if ret != IDN2_OK as libc::c_int {
        ret = idn2_lookup_u8(
            src as *mut uint8_t,
            &mut ascii_encoded as *mut *mut libc::c_char as *mut *mut uint8_t,
            IDN2_TRANSITIONAL as libc::c_int,
        );
    }
    if ret != IDN2_OK as libc::c_int {
        logprintf(
            LOG_VERBOSE,
            dcgettext(
                0 as *const libc::c_char,
                b"idn_encode failed (%d): %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ret,
            quote(idn2_strerror(ret)),
        );
    }
    rpl_free(utf8_encoded as *mut libc::c_void);
    utf8_encoded = 0 as *mut libc::c_char;
    if ret == IDN2_OK as libc::c_int && !ascii_encoded.is_null() {
        let mut tmp: *mut libc::c_char = xstrdup(ascii_encoded);
        idn2_free(ascii_encoded as *mut libc::c_void);
        ascii_encoded = tmp;
    }
    return if ret == IDN2_OK as libc::c_int {
        ascii_encoded
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn idn_decode(mut host: *const libc::c_char) -> *mut libc::c_char {
    return xstrdup(host);
}
#[no_mangle]
pub unsafe extern "C" fn remote_to_utf8(
    mut iri: *const iri,
    mut str: *const libc::c_char,
    mut new: *mut *mut libc::c_char,
) -> bool {
    let mut ret: bool = 0 as libc::c_int != 0;
    if ((*iri).uri_encoding).is_null() {
        return 0 as libc::c_int != 0;
    }
    if c_strcasecmp((*iri).uri_encoding, b"UTF-8\0" as *const u8 as *const libc::c_char)
        == 0
    {
        let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
        p = str as *mut libc::c_uchar;
        while *p != 0 {
            if *p as libc::c_int > 127 as libc::c_int {
                *new = strdup(str);
                return 1 as libc::c_int != 0;
            }
            p = p.offset(1);
            p;
        }
        return 0 as libc::c_int != 0;
    }
    if do_conversion(
        b"UTF-8\0" as *const u8 as *const libc::c_char,
        (*iri).uri_encoding,
        str,
        strlen(str),
        new,
    ) {
        ret = 1 as libc::c_int != 0;
    }
    if !(*new).is_null() && strcmp(str, *new) == 0 {
        rpl_free(*new as *mut libc::c_void);
        *new = 0 as *mut libc::c_char;
        return 0 as libc::c_int != 0;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn iri_new() -> *mut iri {
    let mut i: *mut iri = xmalloc(::core::mem::size_of::<iri>() as libc::c_ulong)
        as *mut iri;
    (*i)
        .uri_encoding = if !(opt.encoding_remote).is_null() {
        xstrdup(opt.encoding_remote)
    } else {
        0 as *mut libc::c_char
    };
    (*i).content_encoding = 0 as *mut libc::c_char;
    (*i).orig_url = 0 as *mut libc::c_char;
    (*i).utf8_encode = opt.enable_iri;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn iri_dup(mut src: *const iri) -> *mut iri {
    let mut i: *mut iri = xmalloc(::core::mem::size_of::<iri>() as libc::c_ulong)
        as *mut iri;
    (*i)
        .uri_encoding = if !((*src).uri_encoding).is_null() {
        xstrdup((*src).uri_encoding)
    } else {
        0 as *mut libc::c_char
    };
    (*i)
        .content_encoding = if !((*src).content_encoding).is_null() {
        xstrdup((*src).content_encoding)
    } else {
        0 as *mut libc::c_char
    };
    (*i)
        .orig_url = if !((*src).orig_url).is_null() {
        xstrdup((*src).orig_url)
    } else {
        0 as *mut libc::c_char
    };
    (*i).utf8_encode = (*src).utf8_encode;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn iri_free(mut i: *mut iri) {
    if !i.is_null() {
        rpl_free((*i).uri_encoding as *mut libc::c_void);
        (*i).uri_encoding = 0 as *mut libc::c_char;
        rpl_free((*i).content_encoding as *mut libc::c_void);
        (*i).content_encoding = 0 as *mut libc::c_char;
        rpl_free((*i).orig_url as *mut libc::c_void);
        (*i).orig_url = 0 as *mut libc::c_char;
        rpl_free(i as *mut libc::c_void);
        i = 0 as *mut iri;
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_uri_encoding(
    mut i: *mut iri,
    mut charset: *const libc::c_char,
    mut force: bool,
) {
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"URI encoding = %s\n\0" as *const u8 as *const libc::c_char,
            if !charset.is_null() {
                quote(charset)
            } else {
                b"None\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !force && !(opt.encoding_remote).is_null() {
        return;
    }
    if !((*i).uri_encoding).is_null() {
        if !charset.is_null() && c_strcasecmp((*i).uri_encoding, charset) == 0 {
            return;
        }
        rpl_free((*i).uri_encoding as *mut libc::c_void);
        (*i).uri_encoding = 0 as *mut libc::c_char;
    }
    (*i)
        .uri_encoding = if !charset.is_null() {
        xstrdup(charset)
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_content_encoding(
    mut i: *mut iri,
    mut charset: *const libc::c_char,
) {
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"URI content encoding = %s\n\0" as *const u8 as *const libc::c_char,
            if !charset.is_null() {
                quote(charset)
            } else {
                b"None\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !(opt.encoding_remote).is_null() {
        return;
    }
    if !((*i).content_encoding).is_null() {
        if !charset.is_null() && c_strcasecmp((*i).content_encoding, charset) == 0 {
            return;
        }
        rpl_free((*i).content_encoding as *mut libc::c_void);
        (*i).content_encoding = 0 as *mut libc::c_char;
    }
    (*i)
        .content_encoding = if !charset.is_null() {
        xstrdup(charset)
    } else {
        0 as *mut libc::c_char
    };
}
