#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(asm, extern_types)]
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type hash_table;
    pub type address_list;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut exec_name: *const libc::c_char;
    fn idn_decode(host: *const libc::c_char) -> *mut libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn escnonprint_uri(_: *const libc::c_char) -> *const libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn logprintf(_: log_options, _: *const libc::c_char, _: ...);
    fn debug_logprintf(_: *const libc::c_char, _: ...);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getpeername(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn aprintf(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn run_with_timeout(
        _: libc::c_double,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> bool;
    fn lookup_host(_: *const libc::c_char, _: libc::c_int) -> *mut address_list;
    fn address_list_get_bounds(
        _: *const address_list,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    );
    fn address_list_address_at(
        _: *const address_list,
        _: libc::c_int,
    ) -> *const ip_address;
    fn address_list_set_faulty(_: *mut address_list, _: libc::c_int);
    fn address_list_set_connected(_: *mut address_list);
    fn address_list_connected_p(_: *const address_list) -> bool;
    fn address_list_release(_: *mut address_list);
    fn print_address(_: *const ip_address) -> *const libc::c_char;
    fn hash_table_new(
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong>,
        _: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    ) -> *mut hash_table;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type intptr_t = libc::c_long;
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    WGET_EXIT_SUCCESS = 0,
    WGET_EXIT_GENERIC_ERROR = 1,
    WGET_EXIT_PARSE_ERROR = 2,
    WGET_EXIT_IO_FAIL = 3,
    WGET_EXIT_NETWORK_FAIL = 4,
    WGET_EXIT_SSL_AUTH_FAIL = 5,
    WGET_EXIT_SERVER_AUTH_FAIL = 6,
    WGET_EXIT_PROTOCOL_ERROR = 7,
    WGET_EXIT_SERVER_ERROR = 8,
    WGET_EXIT_UNKNOWN,
}  // end of enum

pub type socklen_t = __socklen_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_NONBLOCK = 2048,
    SOCK_CLOEXEC = 524288,
    SOCK_PACKET = 10,
    SOCK_DCCP = 6,
    SOCK_SEQPACKET = 5,
    SOCK_RDM = 4,
    SOCK_RAW = 3,
    SOCK_DGRAM = 2,
    SOCK_STREAM = 1,
}  // end of enum

pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_5 {
    MSG_CMSG_CLOEXEC = 1073741824,
    MSG_FASTOPEN = 536870912,
    MSG_ZEROCOPY = 67108864,
    MSG_BATCH = 262144,
    MSG_WAITFORONE = 65536,
    MSG_MORE = 32768,
    MSG_NOSIGNAL = 16384,
    MSG_ERRQUEUE = 8192,
    MSG_RST = 4096,
    MSG_CONFIRM = 2048,
    MSG_SYN = 1024,
    MSG_FIN = 512,
    MSG_WAITALL = 256,
    MSG_EOR = 128,
    MSG_DONTWAIT = 64,
    MSG_TRUNC = 32,
    MSG_PROXY = 16,
    MSG_CTRUNC = 8,
    MSG_TRYHARD = 4,
    MSG_DONTROUTE = 4,
    MSG_PEEK = 2,
    MSG_OOB = 1,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_7 {
    IPPROTO_MAX = 256,
    IPPROTO_RAW = 255,
    IPPROTO_MPLS = 137,
    IPPROTO_UDPLITE = 136,
    IPPROTO_SCTP = 132,
    IPPROTO_COMP = 108,
    IPPROTO_PIM = 103,
    IPPROTO_ENCAP = 98,
    IPPROTO_BEETPH = 94,
    IPPROTO_MTP = 92,
    IPPROTO_AH = 51,
    IPPROTO_ESP = 50,
    IPPROTO_GRE = 47,
    IPPROTO_RSVP = 46,
    IPPROTO_IPV6 = 41,
    IPPROTO_DCCP = 33,
    IPPROTO_TP = 29,
    IPPROTO_IDP = 22,
    IPPROTO_UDP = 17,
    IPPROTO_PUP = 12,
    IPPROTO_EGP = 8,
    IPPROTO_TCP = 6,
    IPPROTO_IPIP = 4,
    IPPROTO_IGMP = 2,
    IPPROTO_ICMP = 1,
    IPPROTO_IP = 0,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_address {
    pub family: libc::c_int,
    pub data: C2RustUnnamed_8,
    pub ipv6_scope: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_9 {
    LH_SILENT = 1,
    LH_BIND = 2,
    LH_REFRESH = 4,
}  // end of enum

pub type C2RustUnnamed_10 = libc::c_int;
pub const E_HOST: C2RustUnnamed_10 = -100;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transport_info {
    pub imp: *mut transport_implementation,
    pub ctx: *mut libc::c_void,
}
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
pub struct cwt_context {
    pub fd: libc::c_int,
    pub addr: *const sockaddr,
    pub addrlen: socklen_t,
    pub result: libc::c_int,
}
pub const WAIT_FOR_READ: C2RustUnnamed_12 = 1;
pub const WAIT_FOR_WRITE: C2RustUnnamed_12 = 2;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_11 {
    ENDPOINT_LOCAL,
    ENDPOINT_PEER,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_12 {
    WAIT_FOR_READ = 1,
    WAIT_FOR_WRITE = 2,
}  // end of enum
q, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_11 {
    ENDPOINT_LOCAL,
    ENDPOINT_PEER,
}  // end of enum

pub type C2RustUnnamed_12 = libc::c_uint;
unsafe extern "C" fn sockaddr_set_data(
    mut sa: *mut sockaddr,
    mut ip: *const ip_address,
    mut port: libc::c_int,
) {
    match (*ip).family {
        2 => {
            let mut sin: *mut sockaddr_in = sa as *mut sockaddr_in;
            memset(
                sin as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong,
            );
            (*sin).sin_family = 2 as libc::c_int as sa_family_t;
            (*sin)
                .sin_port = ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = port as libc::c_ushort;
                if 0 != 0 {
                    __v = (__x as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                        | (__x as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
                        as libc::c_ushort;
                } else {
                    let fresh0 = &mut __v;
                    let fresh1;
                    let fresh2 = __x;
                    asm!(
                        "rorw $8, {0:x}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                        options(pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
                }
                __v
            });
            (*sin).sin_addr = (*ip).data.d4;
        }
        10 => {
            let mut sin6: *mut sockaddr_in6 = sa as *mut sockaddr_in6;
            memset(
                sin6 as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<sockaddr_in6>() as libc::c_ulong,
            );
            (*sin6).sin6_family = 10 as libc::c_int as sa_family_t;
            (*sin6)
                .sin6_port = ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = port as libc::c_ushort;
                if 0 != 0 {
                    __v = (__x as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                        | (__x as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
                        as libc::c_ushort;
                } else {
                    let fresh3 = &mut __v;
                    let fresh4;
                    let fresh5 = __x;
                    asm!(
                        "rorw $8, {0:x}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                        options(pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                }
                __v
            });
            (*sin6).sin6_addr = (*ip).data.d6;
            (*sin6).sin6_scope_id = (*ip).ipv6_scope as uint32_t;
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn sockaddr_get_data(
    mut sa: *const sockaddr,
    mut ip: *mut ip_address,
    mut port: *mut libc::c_int,
) {
    match (*sa).sa_family as libc::c_int {
        2 => {
            let mut sin: *mut sockaddr_in = sa as *mut sockaddr_in;
            if !ip.is_null() {
                (*ip).family = 2 as libc::c_int;
                (*ip).data.d4 = (*sin).sin_addr;
            }
            if !port.is_null() {
                *port = ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = (*sin).sin_port;
                    if 0 != 0 {
                        __v = (__x as libc::c_int >> 8 as libc::c_int
                            & 0xff as libc::c_int
                            | (__x as libc::c_int & 0xff as libc::c_int)
                                << 8 as libc::c_int) as libc::c_ushort;
                    } else {
                        let fresh6 = &mut __v;
                        let fresh7;
                        let fresh8 = __x;
                        asm!(
                            "rorw $8, {0:x}", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                            options(pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
                    }
                    __v
                }) as libc::c_int;
            }
        }
        10 => {
            let mut sin6: *mut sockaddr_in6 = sa as *mut sockaddr_in6;
            if !ip.is_null() {
                (*ip).family = 10 as libc::c_int;
                (*ip).data.d6 = (*sin6).sin6_addr;
                (*ip).ipv6_scope = (*sin6).sin6_scope_id as libc::c_int;
            }
            if !port.is_null() {
                *port = ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = (*sin6).sin6_port;
                    if 0 != 0 {
                        __v = (__x as libc::c_int >> 8 as libc::c_int
                            & 0xff as libc::c_int
                            | (__x as libc::c_int & 0xff as libc::c_int)
                                << 8 as libc::c_int) as libc::c_ushort;
                    } else {
                        let fresh9 = &mut __v;
                        let fresh10;
                        let fresh11 = __x;
                        asm!(
                            "rorw $8, {0:x}", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) =>
                            fresh10, options(pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
                    }
                    __v
                }) as libc::c_int;
            }
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn sockaddr_size(mut sa: *const sockaddr) -> socklen_t {
    match (*sa).sa_family as libc::c_int {
        2 => return ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        10 => return ::core::mem::size_of::<sockaddr_in6>() as libc::c_ulong as socklen_t,
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn resolve_bind_address(mut sa: *mut sockaddr) -> bool {
    let mut al: *mut address_list = 0 as *mut address_list;
    static mut called: bool = false;
    static mut should_bind: bool = false;
    static mut ip: ip_address = ip_address {
        family: 0,
        data: C2RustUnnamed_8 {
            d4: in_addr { s_addr: 0 },
        },
        ipv6_scope: 0,
    };
    if called {
        if should_bind {
            sockaddr_set_data(sa, &mut ip, 0 as libc::c_int);
        }
        return should_bind;
    }
    called = 1 as libc::c_int != 0;
    al = lookup_host(
        opt.bind_address,
        LH_BIND as libc::c_int | LH_SILENT as libc::c_int,
    );
    if al.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: unable to resolve bind address %s; disabling bind.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            exec_name,
            quote(opt.bind_address),
        );
        should_bind = 0 as libc::c_int != 0;
        return 0 as libc::c_int != 0;
    }
    ip = *address_list_address_at(al, 0 as libc::c_int);
    address_list_release(al);
    sockaddr_set_data(sa, &mut ip, 0 as libc::c_int);
    should_bind = 1 as libc::c_int != 0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn connect_with_timeout_callback(mut arg: *mut libc::c_void) {
    let mut ctx: *mut cwt_context = arg as *mut cwt_context;
    (*ctx)
        .result = connect(
        (*ctx).fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*ctx).addr,
        },
        (*ctx).addrlen,
    );
}
unsafe extern "C" fn connect_with_timeout(
    mut fd: libc::c_int,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut ctx: cwt_context = cwt_context {
        fd: 0,
        addr: 0 as *const sockaddr,
        addrlen: 0,
        result: 0,
    };
    ctx.fd = fd;
    ctx.addr = addr;
    ctx.addrlen = addrlen;
    if run_with_timeout(
        timeout,
        Some(
            connect_with_timeout_callback
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
        &mut ctx as *mut cwt_context as *mut libc::c_void,
    ) {
        *__errno_location() = 110 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if ctx.result == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int {
        *__errno_location() = 110 as libc::c_int;
    }
    return ctx.result;
}
#[no_mangle]
pub unsafe extern "C" fn connect_to_ip(
    mut ip: *const ip_address,
    mut port: libc::c_int,
    mut print: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ss: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa: *mut sockaddr = &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut sock: libc::c_int = 0;
    if !print.is_null() {
        let mut txt_addr: *const libc::c_char = print_address(ip);
        if 0 as libc::c_int != strcmp(print, txt_addr) {
            let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            if opt.enable_iri as libc::c_int != 0
                && {
                    name = idn_decode(print as *mut libc::c_char);
                    !name.is_null()
                }
            {
                str = aprintf(
                    b"%s (%s)\0" as *const u8 as *const libc::c_char,
                    name,
                    print,
                );
                rpl_free(name as *mut libc::c_void);
                name = 0 as *mut libc::c_char;
            }
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Connecting to %s|%s|:%d... \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                if !str.is_null() { str } else { escnonprint_uri(print) },
                txt_addr,
                port,
            );
            rpl_free(str as *mut libc::c_void);
            str = 0 as *mut libc::c_char;
        } else if (*ip).family == 2 as libc::c_int {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Connecting to %s:%d... \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                txt_addr,
                port,
            );
        } else if (*ip).family == 10 as libc::c_int {
            logprintf(
                LOG_VERBOSE,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Connecting to [%s]:%d... \0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                txt_addr,
                port,
            );
        }
    }
    sockaddr_set_data(sa, ip, port);
    sock = socket(
        (*sa).sa_family as libc::c_int,
        SOCK_STREAM as libc::c_int,
        0 as libc::c_int,
    );
    if !(sock < 0 as libc::c_int) {
        if opt.ipv6_only {
            let mut on: libc::c_int = 1 as libc::c_int;
            let mut err: libc::c_int = setsockopt(
                sock,
                IPPROTO_IPV6 as libc::c_int,
                26 as libc::c_int,
                &mut on as *mut libc::c_int as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
            if opt.debug as libc::c_long != 0 {
                if err < 0 as libc::c_int {
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"Failed setting IPV6_V6ONLY: %s\0" as *const u8
                                as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                    }
                }
            }
        }
        if opt.limit_rate != 0 && opt.limit_rate < 8192 as libc::c_int as libc::c_long {
            let mut bufsize: libc::c_int = opt.limit_rate as libc::c_int;
            if bufsize < 512 as libc::c_int {
                bufsize = 512 as libc::c_int;
            }
            if setsockopt(
                sock,
                1 as libc::c_int,
                8 as libc::c_int,
                &mut bufsize as *mut libc::c_int as *mut libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            ) != 0
            {
                logprintf(
                    LOG_NOTQUIET,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"setsockopt SO_RCVBUF failed: %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    strerror(*__errno_location()),
                );
            }
        }
        if !(opt.bind_address).is_null() {
            let mut bind_ss: sockaddr_storage = sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            };
            let mut bind_sa: *mut sockaddr = &mut bind_ss as *mut sockaddr_storage
                as *mut sockaddr;
            if resolve_bind_address(bind_sa) {
                if bind(
                    sock,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: bind_sa,
                    },
                    sockaddr_size(bind_sa),
                ) < 0 as libc::c_int
                {
                    current_block = 5725010927128442372;
                } else {
                    current_block = 7226443171521532240;
                }
            } else {
                current_block = 7226443171521532240;
            }
        } else {
            current_block = 7226443171521532240;
        }
        match current_block {
            5725010927128442372 => {}
            _ => {
                if !(connect_with_timeout(
                    sock,
                    sa,
                    sockaddr_size(sa),
                    opt.connect_timeout,
                ) < 0 as libc::c_int)
                {
                    if !print.is_null() {
                        logprintf(
                            LOG_VERBOSE,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"connected.\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    if opt.debug as libc::c_long != 0 {
                        debug_logprintf(
                            b"Created socket %d.\n\0" as *const u8
                                as *const libc::c_char,
                            sock,
                        );
                    }
                    return sock;
                }
            }
        }
    }
    let mut save_errno: libc::c_int = *__errno_location();
    if sock >= 0 as libc::c_int {
        fd_close(sock);
    }
    if !print.is_null() {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"failed: %s.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    }
    *__errno_location() = save_errno;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn connect_to_host(
    mut host: *const libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut al: *mut address_list = lookup_host(host, 0 as libc::c_int);
    loop {
        if al.is_null() {
            logprintf(
                LOG_NOTQUIET,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unable to resolve host address %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                exec_name,
                quote(host),
            );
            return E_HOST as libc::c_int;
        }
        address_list_get_bounds(al, &mut start, &mut end);
        i = start;
        while i < end {
            let mut ip: *const ip_address = address_list_address_at(al, i);
            sock = connect_to_ip(ip, port, host);
            if sock >= 0 as libc::c_int {
                address_list_set_connected(al);
                address_list_release(al);
                return sock;
            }
            address_list_set_faulty(al, i);
            i += 1;
            i;
        }
        if !address_list_connected_p(al) {
            break;
        }
        address_list_release(al);
        al = lookup_host(host, LH_REFRESH as libc::c_int);
    }
    address_list_release(al);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bind_local(
    mut bind_address: *const ip_address,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut ss: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa: *mut sockaddr = &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut setopt_val: libc::c_int = 1 as libc::c_int;
    let mut setopt_ptr: *mut libc::c_void = &mut setopt_val as *mut libc::c_int
        as *mut libc::c_void;
    let mut setopt_size: socklen_t = ::core::mem::size_of::<libc::c_int>()
        as libc::c_ulong as socklen_t;
    sock = socket((*bind_address).family, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if sock < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if setsockopt(sock, 1 as libc::c_int, 2 as libc::c_int, setopt_ptr, setopt_size) != 0
    {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"setsockopt SO_REUSEADDR failed: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            strerror(*__errno_location()),
        );
    }
    memset(
        &mut ss as *mut sockaddr_storage as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<sockaddr_storage>() as libc::c_ulong,
    );
    sockaddr_set_data(sa, bind_address, *port);
    if bind(
        sock,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: sa,
        },
        sockaddr_size(sa),
    ) < 0 as libc::c_int
    {
        fd_close(sock);
        return -(1 as libc::c_int);
    }
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Local socket fd %d bound.\n\0" as *const u8 as *const libc::c_char,
            sock,
        );
    }
    if *port == 0 as libc::c_int {
        let mut addrlen: socklen_t = sockaddr_size(sa);
        if getsockname(sock, __SOCKADDR_ARG { __sockaddr__: sa }, &mut addrlen)
            < 0 as libc::c_int
        {
            fd_close(sock);
            return -(1 as libc::c_int);
        }
        sockaddr_get_data(sa, 0 as *mut ip_address, port);
        if opt.debug as libc::c_long != 0 {
            debug_logprintf(
                b"binding to address %s using port %i.\n\0" as *const u8
                    as *const libc::c_char,
                print_address(bind_address),
                *port,
            );
        }
    }
    if listen(sock, 1 as libc::c_int) < 0 as libc::c_int {
        fd_close(sock);
        return -(1 as libc::c_int);
    }
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn accept_connection(mut local_sock: libc::c_int) -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut ss: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa: *mut sockaddr = &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    if opt.connect_timeout != 0. {
        let mut test: libc::c_int = select_fd(
            local_sock,
            opt.connect_timeout,
            WAIT_FOR_READ as libc::c_int,
        );
        if test == 0 as libc::c_int {
            *__errno_location() = 110 as libc::c_int;
        }
        if test <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    sock = accept(local_sock, __SOCKADDR_ARG { __sockaddr__: sa }, &mut addrlen);
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(
            b"Accepted client at socket %d.\n\0" as *const u8 as *const libc::c_char,
            sock,
        );
    }
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn socket_ip_address(
    mut sock: libc::c_int,
    mut ip: *mut ip_address,
    mut endpoint: libc::c_int,
) -> bool {
    let mut storage: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sockaddr: *mut sockaddr = &mut storage as *mut sockaddr_storage
        as *mut sockaddr;
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    let mut ret: libc::c_int = 0;
    memset(sockaddr as *mut libc::c_void, 0 as libc::c_int, addrlen as libc::c_ulong);
    if endpoint == ENDPOINT_LOCAL as libc::c_int {
        ret = getsockname(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: sockaddr,
            },
            &mut addrlen,
        );
    } else if endpoint == ENDPOINT_PEER as libc::c_int {
        ret = getpeername(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: sockaddr,
            },
            &mut addrlen,
        );
    } else {
        abort();
    }
    if ret < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    memset(
        ip as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ip_address>() as libc::c_ulong,
    );
    (*ip).family = (*sockaddr).sa_family as libc::c_int;
    match (*sockaddr).sa_family as libc::c_int {
        10 => {
            let mut sa6: *mut sockaddr_in6 = &mut storage as *mut sockaddr_storage
                as *mut sockaddr_in6;
            (*ip).data.d6 = (*sa6).sin6_addr;
            (*ip).ipv6_scope = (*sa6).sin6_scope_id as libc::c_int;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"conaddr is: %s\n\0" as *const u8 as *const libc::c_char,
                    print_address(ip),
                );
            }
            return 1 as libc::c_int != 0;
        }
        2 => {
            let mut sa: *mut sockaddr_in = &mut storage as *mut sockaddr_storage
                as *mut sockaddr_in;
            (*ip).data.d4 = (*sa).sin_addr;
            if opt.debug as libc::c_long != 0 {
                debug_logprintf(
                    b"conaddr is: %s\n\0" as *const u8 as *const libc::c_char,
                    print_address(ip),
                );
            }
            return 1 as libc::c_int != 0;
        }
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn socket_family(
    mut sock: libc::c_int,
    mut endpoint: libc::c_int,
) -> libc::c_int {
    let mut storage: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sockaddr: *mut sockaddr = &mut storage as *mut sockaddr_storage
        as *mut sockaddr;
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    let mut ret: libc::c_int = 0;
    memset(sockaddr as *mut libc::c_void, 0 as libc::c_int, addrlen as libc::c_ulong);
    if endpoint == ENDPOINT_LOCAL as libc::c_int {
        ret = getsockname(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: sockaddr,
            },
            &mut addrlen,
        );
    } else if endpoint == ENDPOINT_PEER as libc::c_int {
        ret = getpeername(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: sockaddr,
            },
            &mut addrlen,
        );
    } else {
        abort();
    }
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return (*sockaddr).sa_family as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn retryable_socket_connect_error(mut err: libc::c_int) -> bool {
    if 0 as libc::c_int != 0 || err == 97 as libc::c_int || err == 96 as libc::c_int
        || err == 94 as libc::c_int || err == 93 as libc::c_int
        || err == 92 as libc::c_int || err == 22 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if !opt.retry_connrefused {
        if err == 111 as libc::c_int || err == 101 as libc::c_int
            || err == 113 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn select_fd_internal(
    mut fd: libc::c_int,
    mut maxtime: libc::c_double,
    mut wait_for: libc::c_int,
    mut convert_back: bool,
) -> libc::c_int {
    let mut fdset: fd_set = fd_set { fds_bits: [0; 16] };
    let mut rd: *mut fd_set = 0 as *mut fd_set;
    let mut wr: *mut fd_set = 0 as *mut fd_set;
    let mut tmout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut result: libc::c_int = 0;
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if fd >= 1024 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Too many fds open.  Cannot use select on a fd >= %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            1024 as libc::c_int,
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh12 = &mut __d0;
    let fresh13;
    let fresh14 = (::core::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh15 = &mut __d1;
    let fresh16;
    let fresh17 = &mut *(fdset.fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh12,
        fresh14) => fresh13, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh15,
        fresh17) => fresh16, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
    c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
    fdset
        .fds_bits[(fd
        / (8 as libc::c_int
            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd
                % (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    if wait_for & WAIT_FOR_READ as libc::c_int != 0 {
        rd = &mut fdset;
    }
    if wait_for & WAIT_FOR_WRITE as libc::c_int != 0 {
        wr = &mut fdset;
    }
    tmout.tv_sec = maxtime as libc::c_long;
    tmout
        .tv_usec = (1000000 as libc::c_int as libc::c_double
        * (maxtime - maxtime as libc::c_long as libc::c_double)) as __suseconds_t;
    loop {
        result = select(fd + 1 as libc::c_int, rd, wr, 0 as *mut fd_set, &mut tmout);
        if !(result < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn select_fd(
    mut fd: libc::c_int,
    mut maxtime: libc::c_double,
    mut wait_for: libc::c_int,
) -> libc::c_int {
    return select_fd_internal(fd, maxtime, wait_for, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn test_socket_open(mut sock: libc::c_int) -> bool {
    let mut check_set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut to: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ret: libc::c_int = 0 as libc::c_int;
    if sock >= 1024 as libc::c_int {
        logprintf(
            LOG_NOTQUIET,
            dcgettext(
                0 as *const libc::c_char,
                b"Too many fds open.  Cannot use select on a fd >= %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            1024 as libc::c_int,
        );
        exit(WGET_EXIT_GENERIC_ERROR as libc::c_int);
    }
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh18 = &mut __d0;
    let fresh19;
    let fresh20 = (::core::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh21 = &mut __d1;
    let fresh22;
    let fresh23 = &mut *(check_set.fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh18,
        fresh20) => fresh19, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh21,
        fresh23) => fresh22, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
    c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
    check_set
        .fds_bits[(sock
        / (8 as libc::c_int
            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << sock
                % (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    to.tv_sec = 0 as libc::c_int as __time_t;
    to.tv_usec = 1 as libc::c_int as __suseconds_t;
    ret = select(
        sock + 1 as libc::c_int,
        &mut check_set,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut to,
    );
    if ret == 0 { return 1 as libc::c_int != 0 } else { return 0 as libc::c_int != 0 };
}
unsafe extern "C" fn sock_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    loop {
        res = read(fd, buf as *mut libc::c_void, bufsize as size_t) as libc::c_int;
        if !(res == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn sock_write(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    loop {
        res = write(fd, buf as *const libc::c_void, bufsize as size_t) as libc::c_int;
        if !(res == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn sock_poll(
    mut fd: libc::c_int,
    mut timeout: libc::c_double,
    mut wait_for: libc::c_int,
) -> libc::c_int {
    return select_fd(fd, timeout, wait_for);
}
unsafe extern "C" fn sock_peek(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    loop {
        res = recv(
            fd,
            buf as *mut libc::c_void,
            bufsize as size_t,
            MSG_PEEK as libc::c_int,
        ) as libc::c_int;
        if !(res == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn sock_close(mut fd: libc::c_int) {
    close(fd);
    if opt.debug as libc::c_long != 0 {
        debug_logprintf(b"Closed fd %d\n\0" as *const u8 as *const libc::c_char, fd);
    }
}
static mut transport_map: *mut hash_table = 0 as *const hash_table as *mut hash_table;
static mut transport_map_modified_tick: libc::c_uint = 0;
#[no_mangle]
pub unsafe extern "C" fn fd_register_transport(
    mut fd: libc::c_int,
    mut imp: *mut transport_implementation,
    mut ctx: *mut libc::c_void,
) {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    info = xmalloc(::core::mem::size_of::<transport_info>() as libc::c_ulong)
        as *mut transport_info;
    (*info).imp = imp;
    (*info).ctx = ctx;
    if transport_map.is_null() {
        transport_map = hash_table_new(0 as libc::c_int, None, None);
    }
    hash_table_put(
        transport_map,
        fd as intptr_t as *mut libc::c_void,
        info as *const libc::c_void,
    );
    transport_map_modified_tick = transport_map_modified_tick.wrapping_add(1);
    transport_map_modified_tick;
}
#[no_mangle]
pub unsafe extern "C" fn fd_transport_context(mut fd: libc::c_int) -> *mut libc::c_void {
    let mut info: *mut transport_info = hash_table_get(
        transport_map,
        fd as intptr_t as *mut libc::c_void,
    ) as *mut transport_info;
    return if !info.is_null() { (*info).ctx } else { 0 as *mut libc::c_void };
}
unsafe extern "C" fn poll_internal(
    mut fd: libc::c_int,
    mut info: *mut transport_info,
    mut wf: libc::c_int,
    mut timeout: libc::c_double,
) -> bool {
    if timeout == -(1 as libc::c_int) as libc::c_double {
        timeout = opt.read_timeout;
    }
    if timeout != 0. {
        let mut test: libc::c_int = 0;
        if !info.is_null() && ((*(*info).imp).poller).is_some() {
            test = ((*(*info).imp).poller)
                .expect("non-null function pointer")(fd, timeout, wf, (*info).ctx);
        } else {
            test = sock_poll(fd, timeout, wf);
        }
        if test == 0 as libc::c_int {
            *__errno_location() = 110 as libc::c_int;
        }
        if test <= 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fd_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    static mut last_info: *mut transport_info = 0 as *const transport_info
        as *mut transport_info;
    static mut last_fd: libc::c_int = -(1 as libc::c_int);
    static mut last_tick: libc::c_uint = 0;
    if transport_map.is_null() {
        info = 0 as *mut transport_info;
    } else if last_fd == fd && last_tick == transport_map_modified_tick {
        info = last_info;
    } else {
        info = hash_table_get(transport_map, fd as intptr_t as *mut libc::c_void)
            as *mut transport_info;
        last_fd = fd;
        last_info = info;
        last_tick = transport_map_modified_tick;
    }
    if !info.is_null() && ((*(*info).imp).reader).is_some() {
        return ((*(*info).imp).reader)
            .expect("non-null function pointer")(fd, buf, bufsize, (*info).ctx, timeout);
    }
    if !poll_internal(fd, info, WAIT_FOR_READ as libc::c_int, timeout) {
        return -(1 as libc::c_int);
    }
    return sock_read(fd, buf, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn fd_peek(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    static mut last_info: *mut transport_info = 0 as *const transport_info
        as *mut transport_info;
    static mut last_fd: libc::c_int = -(1 as libc::c_int);
    static mut last_tick: libc::c_uint = 0;
    if transport_map.is_null() {
        info = 0 as *mut transport_info;
    } else if last_fd == fd && last_tick == transport_map_modified_tick {
        info = last_info;
    } else {
        info = hash_table_get(transport_map, fd as intptr_t as *mut libc::c_void)
            as *mut transport_info;
        last_fd = fd;
        last_info = info;
        last_tick = transport_map_modified_tick;
    }
    if !info.is_null() && ((*(*info).imp).peeker).is_some() {
        return ((*(*info).imp).peeker)
            .expect("non-null function pointer")(fd, buf, bufsize, (*info).ctx, timeout);
    }
    if !poll_internal(fd, info, WAIT_FOR_READ as libc::c_int, timeout) {
        return -(1 as libc::c_int);
    }
    return sock_peek(fd, buf, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn fd_write(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut timeout: libc::c_double,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut info: *mut transport_info = 0 as *mut transport_info;
    static mut last_info: *mut transport_info = 0 as *const transport_info
        as *mut transport_info;
    static mut last_fd: libc::c_int = -(1 as libc::c_int);
    static mut last_tick: libc::c_uint = 0;
    if transport_map.is_null() {
        info = 0 as *mut transport_info;
    } else if last_fd == fd && last_tick == transport_map_modified_tick {
        info = last_info;
    } else {
        info = hash_table_get(transport_map, fd as intptr_t as *mut libc::c_void)
            as *mut transport_info;
        last_fd = fd;
        last_info = info;
        last_tick = transport_map_modified_tick;
    }
    res = 0 as libc::c_int;
    while bufsize > 0 as libc::c_int {
        if !poll_internal(fd, info, WAIT_FOR_WRITE as libc::c_int, timeout) {
            return -(1 as libc::c_int);
        }
        if !info.is_null() && ((*(*info).imp).writer).is_some() {
            res = ((*(*info).imp).writer)
                .expect("non-null function pointer")(fd, buf, bufsize, (*info).ctx);
        } else {
            res = sock_write(fd, buf, bufsize);
        }
        if res <= 0 as libc::c_int {
            break;
        }
        buf = buf.offset(res as isize);
        bufsize -= res;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn fd_errstr(mut fd: libc::c_int) -> *const libc::c_char {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    if !transport_map.is_null() {
        info = hash_table_get(transport_map, fd as intptr_t as *mut libc::c_void)
            as *mut transport_info;
    }
    if !info.is_null() && ((*(*info).imp).errstr).is_some() {
        let mut err: *const libc::c_char = ((*(*info).imp).errstr)
            .expect("non-null function pointer")(fd, (*info).ctx);
        if !err.is_null() {
            return err;
        }
    }
    return strerror(*__errno_location());
}
#[no_mangle]
pub unsafe extern "C" fn fd_close(mut fd: libc::c_int) {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    if fd < 0 as libc::c_int {
        return;
    }
    info = 0 as *mut transport_info;
    if !transport_map.is_null() {
        info = hash_table_get(transport_map, fd as intptr_t as *mut libc::c_void)
            as *mut transport_info;
    }
    if !info.is_null() && ((*(*info).imp).closer).is_some() {
        ((*(*info).imp).closer).expect("non-null function pointer")(fd, (*info).ctx);
    } else {
        sock_close(fd);
    }
    if !info.is_null() {
        hash_table_remove(transport_map, fd as intptr_t as *mut libc::c_void);
        rpl_free(info as *mut libc::c_void);
        info = 0 as *mut transport_info;
        transport_map_modified_tick = transport_map_modified_tick.wrapping_add(1);
        transport_map_modified_tick;
    }
}
