#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type url;
    pub type yy_buffer_state;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn wget_read_file(_: *const libc::c_char) -> *mut file_memory;
    fn wget_read_file_free(_: *mut file_memory);
    fn number_to_static_string(_: wgint) -> *mut libc::c_char;
    fn append_url(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut map_context,
    ) -> *mut urlpos;
    fn xstrndup(string: *const libc::c_char, n: size_t) -> *mut libc::c_char;
    static mut yytext: *mut libc::c_char;
    static mut yyleng: libc::c_int;
    fn yy_scan_bytes(bytes: *const libc::c_char, len: libc::c_int) -> YY_BUFFER_STATE;
    fn yy_delete_buffer(b: YY_BUFFER_STATE);
    fn yylex() -> libc::c_int;
    fn yylex_destroy();
}
pub type __int64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
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
impl compression_options {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            compression_options::compression_none => 2,
            compression_options::compression_gzip => 1,
            compression_options::compression_auto => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    prefer_none = 2,
    prefer_ipv6 = 1,
    prefer_ipv4 = 0,
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::prefer_none => 2,
            C2RustUnnamed::prefer_ipv6 => 1,
            C2RustUnnamed::prefer_ipv4 => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    restrict_uppercase = 2,
    restrict_lowercase = 1,
    restrict_no_case_restriction = 0,
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_0::restrict_uppercase => 2,
            C2RustUnnamed_0::restrict_lowercase => 1,
            C2RustUnnamed_0::restrict_no_case_restriction => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    restrict_windows = 2,
    restrict_vms = 1,
    restrict_unix = 0,
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_1::restrict_windows => 2,
            C2RustUnnamed_1::restrict_vms => 1,
            C2RustUnnamed_1::restrict_unix => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum keyfile_type {
    keyfile_asn1 = 1,
    keyfile_pem = 0,
impl keyfile_type {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            keyfile_type::keyfile_asn1 => 1,
            keyfile_type::keyfile_pem => 0,
        }
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    regex_type_posix = 1,
    regex_type_pcre = 0,
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_3::regex_type_posix => 1,
            C2RustUnnamed_3::regex_type_pcre => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_options {
    LOG_PROGRESS = 4,
    LOG_ALWAYS = 3,
    LOG_NONVERBOSE = 2,
    LOG_NOTQUIET = 1,
    LOG_VERBOSE = 0,
impl log_options {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            log_options::LOG_PROGRESS => 4,
            log_options::LOG_ALWAYS => 3,
            log_options::LOG_NONVERBOSE => 2,
            log_options::LOG_NOTQUIET => 1,
            log_options::LOG_VERBOSE => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_4::_ISalnum => 8,
            C2RustUnnamed_4::_ISpunct => 4,
            C2RustUnnamed_4::_IScntrl => 2,
            C2RustUnnamed_4::_ISblank => 1,
            C2RustUnnamed_4::_ISgraph => 32768,
            C2RustUnnamed_4::_ISprint => 16384,
            C2RustUnnamed_4::_ISspace => 8192,
            C2RustUnnamed_4::_ISxdigit => 4096,
            C2RustUnnamed_4::_ISdigit => 2048,
            C2RustUnnamed_4::_ISalpha => 1024,
            C2RustUnnamed_4::_ISlower => 512,
            C2RustUnnamed_4::_ISupper => 256,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_memory {
    pub content: *mut libc::c_char,
    pub length: libc::c_long,
    pub mmap_p: libc::c_int,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum convert_options {
    CO_NULLIFY_BASE = 4,
    CO_CONVERT_TO_COMPLETE = 3,
    CO_CONVERT_BASENAME_ONLY = 2,
    CO_CONVERT_TO_RELATIVE = 1,
    CO_NOCONVERT = 0,
impl convert_options {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            convert_options::CO_NULLIFY_BASE => 4,
            convert_options::CO_CONVERT_TO_COMPLETE => 3,
            convert_options::CO_CONVERT_BASENAME_ONLY => 2,
            convert_options::CO_CONVERT_TO_RELATIVE => 1,
            convert_options::CO_NOCONVERT => 0,
        }
    }
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct urlpos {
    pub url: *mut url,
    pub local_name: *mut libc::c_char,
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
    pub refresh_timeout: libc::c_int,
    pub convert: convert_options,
    pub pos: libc::c_int,
    pub size: libc::c_int,
    pub next: *mut urlpos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map_context {
    pub text: *mut libc::c_char,
    pub base: *mut libc::c_char,
    pub parent_base: *const libc::c_char,
    pub document_file: *const libc::c_char,
    pub nofollow: bool,
    pub head: *mut urlpos,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    COMMENT = 27,
    FUNCTION = 26,
    BAD_URI = 25,
    URI = 24,
    NUMBER = 23,
    PERCENTAGE = 22,
    DIMENSION = 21,
    FREQ = 20,
    TIME = 19,
    ANGLE = 18,
    LENGTH = 17,
    EXS = 16,
    EMS = 15,
    IMPORTANT_SYM = 14,
    CHARSET_SYM = 13,
    MEDIA_SYM = 12,
    PAGE_SYM = 11,
    IMPORT_SYM = 10,
    HASH = 9,
    IDENT = 8,
    BAD_STRING = 7,
    STRING = 6,
    DASHMATCH = 5,
    INCLUDES = 4,
    CDC = 3,
    CDO = 2,
    S = 1,
    CSSEOF = 0,
impl C2RustUnnamed_5 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_5::COMMENT => 27,
            C2RustUnnamed_5::FUNCTION => 26,
            C2RustUnnamed_5::BAD_URI => 25,
            C2RustUnnamed_5::URI => 24,
            C2RustUnnamed_5::NUMBER => 23,
            C2RustUnnamed_5::PERCENTAGE => 22,
            C2RustUnnamed_5::DIMENSION => 21,
            C2RustUnnamed_5::FREQ => 20,
            C2RustUnnamed_5::TIME => 19,
            C2RustUnnamed_5::ANGLE => 18,
            C2RustUnnamed_5::LENGTH => 17,
            C2RustUnnamed_5::EXS => 16,
            C2RustUnnamed_5::EMS => 15,
            C2RustUnnamed_5::IMPORTANT_SYM => 14,
            C2RustUnnamed_5::CHARSET_SYM => 13,
            C2RustUnnamed_5::MEDIA_SYM => 12,
            C2RustUnnamed_5::PAGE_SYM => 11,
            C2RustUnnamed_5::IMPORT_SYM => 10,
            C2RustUnnamed_5::HASH => 9,
            C2RustUnnamed_5::IDENT => 8,
            C2RustUnnamed_5::BAD_STRING => 7,
            C2RustUnnamed_5::STRING => 6,
            C2RustUnnamed_5::DASHMATCH => 5,
            C2RustUnnamed_5::INCLUDES => 4,
            C2RustUnnamed_5::CDC => 3,
            C2RustUnnamed_5::CDO => 2,
            C2RustUnnamed_5::S => 1,
            C2RustUnnamed_5::CSSEOF => 0,
        }
    }
}

pub type YY_BUFFER_STATE = *mut yy_buffer_state;
unsafe extern "C" fn get_uri_string(
    mut at: *const libc::c_char,
    mut pos: *mut libc::c_int,
    mut length: *mut libc::c_int,
) -> *mut libc::c_char {
    if *length < 4 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if 0 as libc::c_int
        != strncasecmp(
            at.offset(*pos as isize),
            b"url(\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        )
    {
        return 0 as *mut libc::c_char;
    }
    *pos += 4 as libc::c_int;
    *length -= 5 as libc::c_int;
    while *length > 0 as libc::c_int
        && *(*__ctype_b_loc()).offset(*at.offset(*pos as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        *pos += 1;
        *pos;
        *length -= 1;
        if *length == 0 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
    }
    while *length > 0 as libc::c_int
        && *(*__ctype_b_loc())
            .offset(
                *at.offset((*pos + *length - 1 as libc::c_int) as isize) as libc::c_int
                    as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        *length -= 1;
        *length;
    }
    if *length >= 2 as libc::c_int
        && (*at.offset(*pos as isize) as libc::c_int == '\'' as i32
            || *at.offset(*pos as isize) as libc::c_int == '"' as i32)
    {
        *pos += 1;
        *pos;
        *length -= 2 as libc::c_int;
    }
    if *length <= 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    return xstrndup(at.offset(*pos as isize), *length as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn get_urls_css(
    mut ctx: *mut map_context,
    mut offset: libc::c_int,
    mut buf_length: libc::c_int,
) {
    let mut token: libc::c_int = 0;
    let mut buffer_pos: libc::c_int = 0 as libc::c_int;
    let mut pos: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut uri: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = yy_scan_bytes(((*ctx).text).offset(offset as isize), buf_length);
    loop {
        token = yylex();
        if !(token != CSSEOF as libc::c_int) {
            break;
        }
        if token == IMPORT_SYM as libc::c_int {
            loop {
                buffer_pos += yyleng;
                token = yylex();
                if !(token == S as libc::c_int) {
                    break;
                }
            }
            if token == STRING as libc::c_int || token == URI as libc::c_int {
                pos = buffer_pos + offset;
                length = yyleng;
                if token == URI as libc::c_int {
                    uri = get_uri_string((*ctx).text, &mut pos, &mut length);
                } else if length >= 2 as libc::c_int {
                    pos += 1;
                    pos;
                    length -= 2 as libc::c_int;
                    uri = xmalloc((length + 1 as libc::c_int) as size_t)
                        as *mut libc::c_char;
                    memcpy(
                        uri as *mut libc::c_void,
                        yytext.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        length as libc::c_ulong,
                    );
                    *uri.offset(length as isize) = '\0' as i32 as libc::c_char;
                } else {
                    uri = 0 as *mut libc::c_char;
                }
                if !uri.is_null() {
                    let mut up: *mut urlpos = append_url(uri, pos, length, ctx);
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"Found @import: [%s] at %d [%s]\n\0" as *const u8
                                as *const libc::c_char,
                            yytext,
                            buffer_pos,
                            uri,
                        );
                    }
                    if !up.is_null() {
                        (*up).set_link_inline_p(1 as libc::c_int as libc::c_uint);
                        (*up).set_link_css_p(1 as libc::c_int as libc::c_uint);
                        (*up).set_link_expect_css(1 as libc::c_int as libc::c_uint);
                    }
                    rpl_free(uri as *mut libc::c_void);
                    uri = 0 as *mut libc::c_char;
                }
            }
        } else if token == URI as libc::c_int {
            pos = buffer_pos + offset;
            length = yyleng;
            uri = get_uri_string((*ctx).text, &mut pos, &mut length);
            if !uri.is_null() {
                let mut up_0: *mut urlpos = append_url(uri, pos, length, ctx);
                if opt.debug as libc::c_long != 0 {
                    debug_logprintf(
                        b"Found URI: [%s] at %d [%s]\n\0" as *const u8
                            as *const libc::c_char,
                        yytext,
                        buffer_pos,
                        uri,
                    );
                }
                if !up_0.is_null() {
                    (*up_0).set_link_inline_p(1 as libc::c_int as libc::c_uint);
                    (*up_0).set_link_css_p(1 as libc::c_int as libc::c_uint);
                }
                rpl_free(uri as *mut libc::c_void);
                uri = 0 as *mut libc::c_char;
            }
        }
        buffer_pos += yyleng;
    }
    yy_delete_buffer(b);
    yylex_destroy();
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"\n\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_urls_css_file(
    mut file: *const libc::c_char,
    mut url: *const libc::c_char,
) -> *mut urlpos {
    let mut fm: *mut file_memory = 0 as *mut file_memory;
    let mut ctx: map_context = map_context {
        text: 0 as *mut libc::c_char,
        base: 0 as *mut libc::c_char,
        parent_base: 0 as *const libc::c_char,
        document_file: 0 as *const libc::c_char,
        nofollow: false,
        head: 0 as *mut urlpos,
    };
    fm = wget_read_file(file);
    if fm.is_null() {
        logprintf(
            LOG_NOTQUIET,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__errno_location()),
        );
        return 0 as *mut urlpos;
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Loaded %s (size %s).\n\0" as *const u8 as *const libc::c_char,
            file,
            number_to_static_string((*fm).length),
        );
    }
    ctx.text = (*fm).content;
    ctx.head = 0 as *mut urlpos;
    ctx.base = 0 as *mut libc::c_char;
    ctx.parent_base = if !url.is_null() { url } else { opt.base_href };
    ctx.document_file = file;
    ctx.nofollow = 0 as libc::c_int != 0;
    get_urls_css(&mut ctx, 0 as libc::c_int, (*fm).length as libc::c_int);
    wget_read_file_free(fm);
    return ctx.head;
}
