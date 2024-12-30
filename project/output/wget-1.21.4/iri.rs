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
    compression_none,
    compression_gzip,
    compression_auto,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    prefer_none,
    prefer_ipv6,
    prefer_ipv4,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    restrict_uppercase,
    restrict_lowercase,
    restrict_no_case_restriction,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    restrict_windows,
    restrict_vms,
    restrict_unix,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum keyfile_type {
    keyfile_asn1,
    keyfile_pem,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    secure_protocol_pfs,
    secure_protocol_tlsv1_3,
    secure_protocol_tlsv1_2,
    secure_protocol_tlsv1_1,
    secure_protocol_tlsv1,
    secure_protocol_sslv3,
    secure_protocol_sslv2,
    secure_protocol_auto,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    regex_type_posix,
    regex_type_pcre,
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
    CODESET,
    _NL_NUM,
    _NL_NUM_LC_IDENTIFICATION,
    _NL_IDENTIFICATION_CODESET,
    _NL_IDENTIFICATION_CATEGORY,
    _NL_IDENTIFICATION_DATE,
    _NL_IDENTIFICATION_REVISION,
    _NL_IDENTIFICATION_ABBREVIATION,
    _NL_IDENTIFICATION_APPLICATION,
    _NL_IDENTIFICATION_AUDIENCE,
    _NL_IDENTIFICATION_TERRITORY,
    _NL_IDENTIFICATION_LANGUAGE,
    _NL_IDENTIFICATION_FAX,
    _NL_IDENTIFICATION_TEL,
    _NL_IDENTIFICATION_EMAIL,
    _NL_IDENTIFICATION_CONTACT,
    _NL_IDENTIFICATION_ADDRESS,
    _NL_IDENTIFICATION_SOURCE,
    _NL_IDENTIFICATION_TITLE,
    _NL_NUM_LC_MEASUREMENT,
    _NL_MEASUREMENT_CODESET,
    _NL_MEASUREMENT_MEASUREMENT,
    _NL_NUM_LC_TELEPHONE,
    _NL_TELEPHONE_CODESET,
    _NL_TELEPHONE_INT_PREFIX,
    _NL_TELEPHONE_INT_SELECT,
    _NL_TELEPHONE_TEL_DOM_FMT,
    _NL_TELEPHONE_TEL_INT_FMT,
    _NL_NUM_LC_ADDRESS,
    _NL_ADDRESS_CODESET,
    _NL_ADDRESS_LANG_LIB,
    _NL_ADDRESS_LANG_TERM,
    _NL_ADDRESS_LANG_AB,
    _NL_ADDRESS_LANG_NAME,
    _NL_ADDRESS_COUNTRY_ISBN,
    _NL_ADDRESS_COUNTRY_NUM,
    _NL_ADDRESS_COUNTRY_CAR,
    _NL_ADDRESS_COUNTRY_AB3,
    _NL_ADDRESS_COUNTRY_AB2,
    _NL_ADDRESS_COUNTRY_POST,
    _NL_ADDRESS_COUNTRY_NAME,
    _NL_ADDRESS_POSTAL_FMT,
    _NL_NUM_LC_NAME,
    _NL_NAME_CODESET,
    _NL_NAME_NAME_MS,
    _NL_NAME_NAME_MISS,
    _NL_NAME_NAME_MRS,
    _NL_NAME_NAME_MR,
    _NL_NAME_NAME_GEN,
    _NL_NAME_NAME_FMT,
    _NL_NUM_LC_PAPER,
    _NL_PAPER_CODESET,
    _NL_PAPER_WIDTH,
    _NL_PAPER_HEIGHT,
    _NL_NUM_LC_MESSAGES,
    _NL_MESSAGES_CODESET,
    __NOSTR,
    __YESSTR,
    __NOEXPR,
    __YESEXPR,
    _NL_NUM_LC_NUMERIC,
    _NL_NUMERIC_CODESET,
    _NL_NUMERIC_THOUSANDS_SEP_WC,
    _NL_NUMERIC_DECIMAL_POINT_WC,
    __GROUPING,
    THOUSEP,
    __THOUSANDS_SEP,
    RADIXCHAR,
    __DECIMAL_POINT,
    _NL_NUM_LC_MONETARY,
    _NL_MONETARY_CODESET,
    _NL_MONETARY_THOUSANDS_SEP_WC,
    _NL_MONETARY_DECIMAL_POINT_WC,
    _NL_MONETARY_CONVERSION_RATE,
    _NL_MONETARY_DUO_VALID_TO,
    _NL_MONETARY_DUO_VALID_FROM,
    _NL_MONETARY_UNO_VALID_TO,
    _NL_MONETARY_UNO_VALID_FROM,
    _NL_MONETARY_DUO_INT_N_SIGN_POSN,
    _NL_MONETARY_DUO_INT_P_SIGN_POSN,
    _NL_MONETARY_DUO_N_SIGN_POSN,
    _NL_MONETARY_DUO_P_SIGN_POSN,
    _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE,
    _NL_MONETARY_DUO_INT_N_CS_PRECEDES,
    _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE,
    _NL_MONETARY_DUO_INT_P_CS_PRECEDES,
    _NL_MONETARY_DUO_N_SEP_BY_SPACE,
    _NL_MONETARY_DUO_N_CS_PRECEDES,
    _NL_MONETARY_DUO_P_SEP_BY_SPACE,
    _NL_MONETARY_DUO_P_CS_PRECEDES,
    _NL_MONETARY_DUO_FRAC_DIGITS,
    _NL_MONETARY_DUO_INT_FRAC_DIGITS,
    _NL_MONETARY_DUO_CURRENCY_SYMBOL,
    _NL_MONETARY_DUO_INT_CURR_SYMBOL,
    __INT_N_SIGN_POSN,
    __INT_P_SIGN_POSN,
    __INT_N_SEP_BY_SPACE,
    __INT_N_CS_PRECEDES,
    __INT_P_SEP_BY_SPACE,
    __INT_P_CS_PRECEDES,
    _NL_MONETARY_CRNCYSTR,
    __N_SIGN_POSN,
    __P_SIGN_POSN,
    __N_SEP_BY_SPACE,
    __N_CS_PRECEDES,
    __P_SEP_BY_SPACE,
    __P_CS_PRECEDES,
    __FRAC_DIGITS,
    __INT_FRAC_DIGITS,
    __NEGATIVE_SIGN,
    __POSITIVE_SIGN,
    __MON_GROUPING,
    __MON_THOUSANDS_SEP,
    __MON_DECIMAL_POINT,
    __CURRENCY_SYMBOL,
    __INT_CURR_SYMBOL,
    _NL_NUM_LC_CTYPE,
    _NL_CTYPE_EXTRA_MAP_14,
    _NL_CTYPE_EXTRA_MAP_13,
    _NL_CTYPE_EXTRA_MAP_12,
    _NL_CTYPE_EXTRA_MAP_11,
    _NL_CTYPE_EXTRA_MAP_10,
    _NL_CTYPE_EXTRA_MAP_9,
    _NL_CTYPE_EXTRA_MAP_8,
    _NL_CTYPE_EXTRA_MAP_7,
    _NL_CTYPE_EXTRA_MAP_6,
    _NL_CTYPE_EXTRA_MAP_5,
    _NL_CTYPE_EXTRA_MAP_4,
    _NL_CTYPE_EXTRA_MAP_3,
    _NL_CTYPE_EXTRA_MAP_2,
    _NL_CTYPE_EXTRA_MAP_1,
    _NL_CTYPE_NONASCII_CASE,
    _NL_CTYPE_MAP_TO_NONASCII,
    _NL_CTYPE_TRANSLIT_IGNORE,
    _NL_CTYPE_TRANSLIT_IGNORE_LEN,
    _NL_CTYPE_TRANSLIT_DEFAULT_MISSING,
    _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN,
    _NL_CTYPE_TRANSLIT_TO_TBL,
    _NL_CTYPE_TRANSLIT_TO_IDX,
    _NL_CTYPE_TRANSLIT_FROM_TBL,
    _NL_CTYPE_TRANSLIT_FROM_IDX,
    _NL_CTYPE_TRANSLIT_TAB_SIZE,
    _NL_CTYPE_OUTDIGIT9_WC,
    _NL_CTYPE_OUTDIGIT8_WC,
    _NL_CTYPE_OUTDIGIT7_WC,
    _NL_CTYPE_OUTDIGIT6_WC,
    _NL_CTYPE_OUTDIGIT5_WC,
    _NL_CTYPE_OUTDIGIT4_WC,
    _NL_CTYPE_OUTDIGIT3_WC,
    _NL_CTYPE_OUTDIGIT2_WC,
    _NL_CTYPE_OUTDIGIT1_WC,
    _NL_CTYPE_OUTDIGIT0_WC,
    _NL_CTYPE_OUTDIGIT9_MB,
    _NL_CTYPE_OUTDIGIT8_MB,
    _NL_CTYPE_OUTDIGIT7_MB,
    _NL_CTYPE_OUTDIGIT6_MB,
    _NL_CTYPE_OUTDIGIT5_MB,
    _NL_CTYPE_OUTDIGIT4_MB,
    _NL_CTYPE_OUTDIGIT3_MB,
    _NL_CTYPE_OUTDIGIT2_MB,
    _NL_CTYPE_OUTDIGIT1_MB,
    _NL_CTYPE_OUTDIGIT0_MB,
    _NL_CTYPE_INDIGITS9_WC,
    _NL_CTYPE_INDIGITS8_WC,
    _NL_CTYPE_INDIGITS7_WC,
    _NL_CTYPE_INDIGITS6_WC,
    _NL_CTYPE_INDIGITS5_WC,
    _NL_CTYPE_INDIGITS4_WC,
    _NL_CTYPE_INDIGITS3_WC,
    _NL_CTYPE_INDIGITS2_WC,
    _NL_CTYPE_INDIGITS1_WC,
    _NL_CTYPE_INDIGITS0_WC,
    _NL_CTYPE_INDIGITS_WC_LEN,
    _NL_CTYPE_INDIGITS9_MB,
    _NL_CTYPE_INDIGITS8_MB,
    _NL_CTYPE_INDIGITS7_MB,
    _NL_CTYPE_INDIGITS6_MB,
    _NL_CTYPE_INDIGITS5_MB,
    _NL_CTYPE_INDIGITS4_MB,
    _NL_CTYPE_INDIGITS3_MB,
    _NL_CTYPE_INDIGITS2_MB,
    _NL_CTYPE_INDIGITS1_MB,
    _NL_CTYPE_INDIGITS0_MB,
    _NL_CTYPE_INDIGITS_MB_LEN,
    _NL_CTYPE_MAP_OFFSET,
    _NL_CTYPE_CLASS_OFFSET,
    _NL_CTYPE_TOLOWER32,
    _NL_CTYPE_TOUPPER32,
    _NL_CTYPE_CODESET_NAME,
    _NL_CTYPE_MB_CUR_MAX,
    _NL_CTYPE_WIDTH,
    _NL_CTYPE_MAP_NAMES,
    _NL_CTYPE_CLASS_NAMES,
    _NL_CTYPE_GAP6,
    _NL_CTYPE_GAP5,
    _NL_CTYPE_GAP4,
    _NL_CTYPE_GAP3,
    _NL_CTYPE_CLASS32,
    _NL_CTYPE_GAP2,
    _NL_CTYPE_TOLOWER,
    _NL_CTYPE_GAP1,
    _NL_CTYPE_TOUPPER,
    _NL_CTYPE_CLASS,
    _NL_NUM_LC_COLLATE,
    _NL_COLLATE_CODESET,
    _NL_COLLATE_COLLSEQWC,
    _NL_COLLATE_COLLSEQMB,
    _NL_COLLATE_SYMB_EXTRAMB,
    _NL_COLLATE_SYMB_TABLEMB,
    _NL_COLLATE_SYMB_HASH_SIZEMB,
    _NL_COLLATE_INDIRECTWC,
    _NL_COLLATE_EXTRAWC,
    _NL_COLLATE_WEIGHTWC,
    _NL_COLLATE_TABLEWC,
    _NL_COLLATE_GAP3,
    _NL_COLLATE_GAP2,
    _NL_COLLATE_GAP1,
    _NL_COLLATE_INDIRECTMB,
    _NL_COLLATE_EXTRAMB,
    _NL_COLLATE_WEIGHTMB,
    _NL_COLLATE_TABLEMB,
    _NL_COLLATE_RULESETS,
    _NL_COLLATE_NRULES,
    _NL_NUM_LC_TIME,
    _NL_WABALTMON_12,
    _NL_WABALTMON_11,
    _NL_WABALTMON_10,
    _NL_WABALTMON_9,
    _NL_WABALTMON_8,
    _NL_WABALTMON_7,
    _NL_WABALTMON_6,
    _NL_WABALTMON_5,
    _NL_WABALTMON_4,
    _NL_WABALTMON_3,
    _NL_WABALTMON_2,
    _NL_WABALTMON_1,
    _NL_ABALTMON_12,
    _NL_ABALTMON_11,
    _NL_ABALTMON_10,
    _NL_ABALTMON_9,
    _NL_ABALTMON_8,
    _NL_ABALTMON_7,
    _NL_ABALTMON_6,
    _NL_ABALTMON_5,
    _NL_ABALTMON_4,
    _NL_ABALTMON_3,
    _NL_ABALTMON_2,
    _NL_ABALTMON_1,
    _NL_WALTMON_12,
    _NL_WALTMON_11,
    _NL_WALTMON_10,
    _NL_WALTMON_9,
    _NL_WALTMON_8,
    _NL_WALTMON_7,
    _NL_WALTMON_6,
    _NL_WALTMON_5,
    _NL_WALTMON_4,
    _NL_WALTMON_3,
    _NL_WALTMON_2,
    _NL_WALTMON_1,
    __ALTMON_12,
    __ALTMON_11,
    __ALTMON_10,
    __ALTMON_9,
    __ALTMON_8,
    __ALTMON_7,
    __ALTMON_6,
    __ALTMON_5,
    __ALTMON_4,
    __ALTMON_3,
    __ALTMON_2,
    __ALTMON_1,
    _NL_TIME_CODESET,
    _NL_W_DATE_FMT,
    _DATE_FMT,
    _NL_TIME_TIMEZONE,
    _NL_TIME_CAL_DIRECTION,
    _NL_TIME_FIRST_WORKDAY,
    _NL_TIME_FIRST_WEEKDAY,
    _NL_TIME_WEEK_1STWEEK,
    _NL_TIME_WEEK_1STDAY,
    _NL_TIME_WEEK_NDAYS,
    _NL_WERA_T_FMT,
    _NL_WERA_D_T_FMT,
    _NL_WALT_DIGITS,
    _NL_WERA_D_FMT,
    _NL_WERA_YEAR,
    _NL_WT_FMT_AMPM,
    _NL_WT_FMT,
    _NL_WD_FMT,
    _NL_WD_T_FMT,
    _NL_WPM_STR,
    _NL_WAM_STR,
    _NL_WMON_12,
    _NL_WMON_11,
    _NL_WMON_10,
    _NL_WMON_9,
    _NL_WMON_8,
    _NL_WMON_7,
    _NL_WMON_6,
    _NL_WMON_5,
    _NL_WMON_4,
    _NL_WMON_3,
    _NL_WMON_2,
    _NL_WMON_1,
    _NL_WABMON_12,
    _NL_WABMON_11,
    _NL_WABMON_10,
    _NL_WABMON_9,
    _NL_WABMON_8,
    _NL_WABMON_7,
    _NL_WABMON_6,
    _NL_WABMON_5,
    _NL_WABMON_4,
    _NL_WABMON_3,
    _NL_WABMON_2,
    _NL_WABMON_1,
    _NL_WDAY_7,
    _NL_WDAY_6,
    _NL_WDAY_5,
    _NL_WDAY_4,
    _NL_WDAY_3,
    _NL_WDAY_2,
    _NL_WDAY_1,
    _NL_WABDAY_7,
    _NL_WABDAY_6,
    _NL_WABDAY_5,
    _NL_WABDAY_4,
    _NL_WABDAY_3,
    _NL_WABDAY_2,
    _NL_WABDAY_1,
    _NL_TIME_ERA_ENTRIES,
    _NL_TIME_ERA_NUM_ENTRIES,
    ERA_T_FMT,
    ERA_D_T_FMT,
    ALT_DIGITS,
    ERA_D_FMT,
    __ERA_YEAR,
    ERA,
    T_FMT_AMPM,
    T_FMT,
    D_FMT,
    D_T_FMT,
    PM_STR,
    AM_STR,
    MON_12,
    MON_11,
    MON_10,
    MON_9,
    MON_8,
    MON_7,
    MON_6,
    MON_5,
    MON_4,
    MON_3,
    MON_2,
    MON_1,
    ABMON_12,
    ABMON_11,
    ABMON_10,
    ABMON_9,
    ABMON_8,
    ABMON_7,
    ABMON_6,
    ABMON_5,
    ABMON_4,
    ABMON_3,
    ABMON_2,
    ABMON_1,
    DAY_7,
    DAY_6,
    DAY_5,
    DAY_4,
    DAY_3,
    DAY_2,
    DAY_1,
    ABDAY_7,
    ABDAY_6,
    ABDAY_5,
    ABDAY_4,
    ABDAY_3,
    ABDAY_2,
    ABDAY_1,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    IDN2_TRANSITIONAL,
    IDN2_NONTRANSITIONAL,
    IDN2_NO_ALABEL_ROUNDTRIP,
    IDN2_NO_TR46,
    IDN2_USE_STD3_ASCII_RULES,
    IDN2_ALLOW_UNASSIGNED,
    IDN2_ALABEL_ROUNDTRIP,
    IDN2_NFC_INPUT,
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
