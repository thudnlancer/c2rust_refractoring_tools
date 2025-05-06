#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
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
        __nfds: i32,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut exec_name: *const i8;
    fn idn_decode(host: *const i8) -> *mut i8;
    fn quote(arg: *const i8) -> *const i8;
    fn escnonprint_uri(_: *const i8) -> *const i8;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn debug_logprintf(_: *const i8, _: ...);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strerror(_: i32) -> *mut i8;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
    fn bind(__fd: i32, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> i32;
    fn getsockname(__fd: i32, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> i32;
    fn connect(__fd: i32, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> i32;
    fn getpeername(__fd: i32, __addr: __SOCKADDR_ARG, __len: *mut socklen_t) -> i32;
    fn recv(__fd: i32, __buf: *mut libc::c_void, __n: size_t, __flags: i32) -> ssize_t;
    fn setsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> i32;
    fn listen(__fd: i32, __n: i32) -> i32;
    fn accept(__fd: i32, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn run_with_timeout(
        _: libc::c_double,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> bool;
    fn lookup_host(_: *const i8, _: i32) -> *mut address_list;
    fn address_list_get_bounds(_: *const address_list, _: *mut i32, _: *mut i32);
    fn address_list_address_at(_: *const address_list, _: i32) -> *const ip_address;
    fn address_list_set_faulty(_: *mut address_list, _: i32);
    fn address_list_set_connected(_: *mut address_list);
    fn address_list_connected_p(_: *const address_list) -> bool;
    fn address_list_release(_: *mut address_list);
    fn print_address(_: *const ip_address) -> *const i8;
    fn hash_table_new(
        _: i32,
        _: Option<unsafe extern "C" fn(*const libc::c_void) -> u64>,
        _: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>,
    ) -> *mut hash_table;
    fn hash_table_get(_: *const hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_table_put(
        _: *mut hash_table,
        _: *const libc::c_void,
        _: *const libc::c_void,
    );
    fn hash_table_remove(_: *mut hash_table, _: *const libc::c_void) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type intptr_t = i64;
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
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::WGET_EXIT_SUCCESS => 0,
            C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR => 1,
            C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR => 2,
            C2RustUnnamed_4::WGET_EXIT_IO_FAIL => 3,
            C2RustUnnamed_4::WGET_EXIT_NETWORK_FAIL => 4,
            C2RustUnnamed_4::WGET_EXIT_SSL_AUTH_FAIL => 5,
            C2RustUnnamed_4::WGET_EXIT_SERVER_AUTH_FAIL => 6,
            C2RustUnnamed_4::WGET_EXIT_PROTOCOL_ERROR => 7,
            C2RustUnnamed_4::WGET_EXIT_SERVER_ERROR => 8,
            C2RustUnnamed_4::WGET_EXIT_UNKNOWN => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            0 => C2RustUnnamed_4::WGET_EXIT_SUCCESS,
            1 => C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR,
            2 => C2RustUnnamed_4::WGET_EXIT_PARSE_ERROR,
            3 => C2RustUnnamed_4::WGET_EXIT_IO_FAIL,
            4 => C2RustUnnamed_4::WGET_EXIT_NETWORK_FAIL,
            5 => C2RustUnnamed_4::WGET_EXIT_SSL_AUTH_FAIL,
            6 => C2RustUnnamed_4::WGET_EXIT_SERVER_AUTH_FAIL,
            7 => C2RustUnnamed_4::WGET_EXIT_PROTOCOL_ERROR,
            8 => C2RustUnnamed_4::WGET_EXIT_SERVER_ERROR,
            9 => C2RustUnnamed_4::WGET_EXIT_UNKNOWN,
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
}
impl __socket_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __socket_type::SOCK_NONBLOCK => 2048,
            __socket_type::SOCK_CLOEXEC => 524288,
            __socket_type::SOCK_PACKET => 10,
            __socket_type::SOCK_DCCP => 6,
            __socket_type::SOCK_SEQPACKET => 5,
            __socket_type::SOCK_RDM => 4,
            __socket_type::SOCK_RAW => 3,
            __socket_type::SOCK_DGRAM => 2,
            __socket_type::SOCK_STREAM => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> __socket_type {
        match value {
            2048 => __socket_type::SOCK_NONBLOCK,
            524288 => __socket_type::SOCK_CLOEXEC,
            10 => __socket_type::SOCK_PACKET,
            6 => __socket_type::SOCK_DCCP,
            5 => __socket_type::SOCK_SEQPACKET,
            4 => __socket_type::SOCK_RDM,
            3 => __socket_type::SOCK_RAW,
            2 => __socket_type::SOCK_DGRAM,
            1 => __socket_type::SOCK_STREAM,
            _ => panic!("Invalid value for __socket_type: {}", value),
        }
    }
}
impl AddAssign<u32> for __socket_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for __socket_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for __socket_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for __socket_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for __socket_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for __socket_type {
    type Output = __socket_type;
    fn add(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for __socket_type {
    type Output = __socket_type;
    fn sub(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for __socket_type {
    type Output = __socket_type;
    fn mul(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for __socket_type {
    type Output = __socket_type;
    fn div(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for __socket_type {
    type Output = __socket_type;
    fn rem(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [i8; 118],
    pub __ss_align: u64,
}
pub type C2RustUnnamed_5 = u32;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_5 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_5 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_5 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_5 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_5 = 65536;
pub const MSG_MORE: C2RustUnnamed_5 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_5 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_5 = 8192;
pub const MSG_RST: C2RustUnnamed_5 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_5 = 2048;
pub const MSG_SYN: C2RustUnnamed_5 = 1024;
pub const MSG_FIN: C2RustUnnamed_5 = 512;
pub const MSG_WAITALL: C2RustUnnamed_5 = 256;
pub const MSG_EOR: C2RustUnnamed_5 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_5 = 64;
pub const MSG_TRUNC: C2RustUnnamed_5 = 32;
pub const MSG_PROXY: C2RustUnnamed_5 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_5 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_5 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_5 = 4;
pub const MSG_PEEK: C2RustUnnamed_5 = 2;
pub const MSG_OOB: C2RustUnnamed_5 = 1;
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
    pub sin_zero: [u8; 8],
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
}
impl C2RustUnnamed_7 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_7::IPPROTO_MAX => 256,
            C2RustUnnamed_7::IPPROTO_RAW => 255,
            C2RustUnnamed_7::IPPROTO_MPLS => 137,
            C2RustUnnamed_7::IPPROTO_UDPLITE => 136,
            C2RustUnnamed_7::IPPROTO_SCTP => 132,
            C2RustUnnamed_7::IPPROTO_COMP => 108,
            C2RustUnnamed_7::IPPROTO_PIM => 103,
            C2RustUnnamed_7::IPPROTO_ENCAP => 98,
            C2RustUnnamed_7::IPPROTO_BEETPH => 94,
            C2RustUnnamed_7::IPPROTO_MTP => 92,
            C2RustUnnamed_7::IPPROTO_AH => 51,
            C2RustUnnamed_7::IPPROTO_ESP => 50,
            C2RustUnnamed_7::IPPROTO_GRE => 47,
            C2RustUnnamed_7::IPPROTO_RSVP => 46,
            C2RustUnnamed_7::IPPROTO_IPV6 => 41,
            C2RustUnnamed_7::IPPROTO_DCCP => 33,
            C2RustUnnamed_7::IPPROTO_TP => 29,
            C2RustUnnamed_7::IPPROTO_IDP => 22,
            C2RustUnnamed_7::IPPROTO_UDP => 17,
            C2RustUnnamed_7::IPPROTO_PUP => 12,
            C2RustUnnamed_7::IPPROTO_EGP => 8,
            C2RustUnnamed_7::IPPROTO_TCP => 6,
            C2RustUnnamed_7::IPPROTO_IPIP => 4,
            C2RustUnnamed_7::IPPROTO_IGMP => 2,
            C2RustUnnamed_7::IPPROTO_ICMP => 1,
            C2RustUnnamed_7::IPPROTO_IP => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_7 {
        match value {
            256 => C2RustUnnamed_7::IPPROTO_MAX,
            255 => C2RustUnnamed_7::IPPROTO_RAW,
            137 => C2RustUnnamed_7::IPPROTO_MPLS,
            136 => C2RustUnnamed_7::IPPROTO_UDPLITE,
            132 => C2RustUnnamed_7::IPPROTO_SCTP,
            108 => C2RustUnnamed_7::IPPROTO_COMP,
            103 => C2RustUnnamed_7::IPPROTO_PIM,
            98 => C2RustUnnamed_7::IPPROTO_ENCAP,
            94 => C2RustUnnamed_7::IPPROTO_BEETPH,
            92 => C2RustUnnamed_7::IPPROTO_MTP,
            51 => C2RustUnnamed_7::IPPROTO_AH,
            50 => C2RustUnnamed_7::IPPROTO_ESP,
            47 => C2RustUnnamed_7::IPPROTO_GRE,
            46 => C2RustUnnamed_7::IPPROTO_RSVP,
            41 => C2RustUnnamed_7::IPPROTO_IPV6,
            33 => C2RustUnnamed_7::IPPROTO_DCCP,
            29 => C2RustUnnamed_7::IPPROTO_TP,
            22 => C2RustUnnamed_7::IPPROTO_IDP,
            17 => C2RustUnnamed_7::IPPROTO_UDP,
            12 => C2RustUnnamed_7::IPPROTO_PUP,
            8 => C2RustUnnamed_7::IPPROTO_EGP,
            6 => C2RustUnnamed_7::IPPROTO_TCP,
            4 => C2RustUnnamed_7::IPPROTO_IPIP,
            2 => C2RustUnnamed_7::IPPROTO_IGMP,
            1 => C2RustUnnamed_7::IPPROTO_ICMP,
            0 => C2RustUnnamed_7::IPPROTO_IP,
            _ => panic!("Invalid value for C2RustUnnamed_7: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_7 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_7 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_7 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_7 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_7 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn add(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn sub(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn mul(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn div(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn rem(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_address {
    pub family: i32,
    pub data: C2RustUnnamed_8,
    pub ipv6_scope: i32,
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
}
impl C2RustUnnamed_9 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_9::LH_SILENT => 1,
            C2RustUnnamed_9::LH_BIND => 2,
            C2RustUnnamed_9::LH_REFRESH => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_9 {
        match value {
            1 => C2RustUnnamed_9::LH_SILENT,
            2 => C2RustUnnamed_9::LH_BIND,
            4 => C2RustUnnamed_9::LH_REFRESH,
            _ => panic!("Invalid value for C2RustUnnamed_9: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_9 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_9 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_9 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_9 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_9 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn add(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn sub(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn mul(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn div(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn rem(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type C2RustUnnamed_10 = i32;
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
pub struct cwt_context {
    pub fd: i32,
    pub addr: *const sockaddr,
    pub addrlen: socklen_t,
    pub result: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_12 {
    WAIT_FOR_READ = 1,
    WAIT_FOR_WRITE = 2,
}
impl C2RustUnnamed_12 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_12::WAIT_FOR_READ => 1,
            C2RustUnnamed_12::WAIT_FOR_WRITE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_12 {
        match value {
            1 => C2RustUnnamed_12::WAIT_FOR_READ,
            2 => C2RustUnnamed_12::WAIT_FOR_WRITE,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_11 {
    ENDPOINT_LOCAL,
    ENDPOINT_PEER,
}
impl C2RustUnnamed_11 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_11::ENDPOINT_LOCAL => 0,
            C2RustUnnamed_11::ENDPOINT_PEER => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_11 {
        match value {
            0 => C2RustUnnamed_11::ENDPOINT_LOCAL,
            1 => C2RustUnnamed_11::ENDPOINT_PEER,
            _ => panic!("Invalid value for C2RustUnnamed_11: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_11 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_11 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_11 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_11 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_11 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn add(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn sub(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn mul(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn div(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_11 {
    type Output = C2RustUnnamed_11;
    fn rem(self, rhs: u32) -> C2RustUnnamed_11 {
        C2RustUnnamed_11::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
unsafe extern "C" fn sockaddr_set_data(
    mut sa: *mut sockaddr,
    mut ip: *const ip_address,
    mut port: i32,
) {
    match (*ip).family {
        2 => {
            let mut sin: *mut sockaddr_in = sa as *mut sockaddr_in;
            memset(
                sin as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<sockaddr_in>() as u64,
            );
            (*sin).sin_family = 2 as i32 as sa_family_t;
            (*sin).sin_port = ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = port as libc::c_ushort;
                if 0 != 0 {
                    __v = (__x as i32 >> 8 as i32 & 0xff as i32
                        | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
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
                ::core::mem::size_of::<sockaddr_in6>() as u64,
            );
            (*sin6).sin6_family = 10 as i32 as sa_family_t;
            (*sin6).sin6_port = ({
                let mut __v: libc::c_ushort = 0;
                let mut __x: libc::c_ushort = port as libc::c_ushort;
                if 0 != 0 {
                    __v = (__x as i32 >> 8 as i32 & 0xff as i32
                        | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
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
    mut port: *mut i32,
) {
    match (*sa).sa_family as i32 {
        2 => {
            let mut sin: *mut sockaddr_in = sa as *mut sockaddr_in;
            if !ip.is_null() {
                (*ip).family = 2 as i32;
                (*ip).data.d4 = (*sin).sin_addr;
            }
            if !port.is_null() {
                *port = ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = (*sin).sin_port;
                    if 0 != 0 {
                        __v = (__x as i32 >> 8 as i32 & 0xff as i32
                            | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
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
                }) as i32;
            }
        }
        10 => {
            let mut sin6: *mut sockaddr_in6 = sa as *mut sockaddr_in6;
            if !ip.is_null() {
                (*ip).family = 10 as i32;
                (*ip).data.d6 = (*sin6).sin6_addr;
                (*ip).ipv6_scope = (*sin6).sin6_scope_id as i32;
            }
            if !port.is_null() {
                *port = ({
                    let mut __v: libc::c_ushort = 0;
                    let mut __x: libc::c_ushort = (*sin6).sin6_port;
                    if 0 != 0 {
                        __v = (__x as i32 >> 8 as i32 & 0xff as i32
                            | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
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
                }) as i32;
            }
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn sockaddr_size(mut sa: *const sockaddr) -> socklen_t {
    match (*sa).sa_family as i32 {
        2 => return ::core::mem::size_of::<sockaddr_in>() as u64 as socklen_t,
        10 => return ::core::mem::size_of::<sockaddr_in6>() as u64 as socklen_t,
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
            sockaddr_set_data(sa, &mut ip, 0 as i32);
        }
        return should_bind;
    }
    called = 1 as i32 != 0;
    al = lookup_host(
        opt.bind_address,
        C2RustUnnamed_9::LH_BIND as i32 | C2RustUnnamed_9::LH_SILENT as i32,
    );
    if al.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"%s: unable to resolve bind address %s; disabling bind.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            exec_name,
            quote(opt.bind_address),
        );
        should_bind = 0 as i32 != 0;
        return 0 as i32 != 0;
    }
    ip = *address_list_address_at(al, 0 as i32);
    address_list_release(al);
    sockaddr_set_data(sa, &mut ip, 0 as i32);
    should_bind = 1 as i32 != 0;
    return 1 as i32 != 0;
}
unsafe extern "C" fn connect_with_timeout_callback(mut arg: *mut libc::c_void) {
    let mut ctx: *mut cwt_context = arg as *mut cwt_context;
    (*ctx).result = connect(
        (*ctx).fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: (*ctx).addr,
        },
        (*ctx).addrlen,
    );
}
unsafe extern "C" fn connect_with_timeout(
    mut fd: i32,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
    mut timeout: libc::c_double,
) -> i32 {
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
        *__errno_location() = 110 as i32;
        return -(1 as i32);
    }
    if ctx.result == -(1 as i32) && *__errno_location() == 4 as i32 {
        *__errno_location() = 110 as i32;
    }
    return ctx.result;
}
#[no_mangle]
pub unsafe extern "C" fn connect_to_ip(
    mut ip: *const ip_address,
    mut port: i32,
    mut print: *const i8,
) -> i32 {
    let mut current_block: u64;
    let mut ss: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa: *mut sockaddr = &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut sock: i32 = 0;
    if !print.is_null() {
        let mut txt_addr: *const i8 = print_address(ip);
        if 0 as i32 != strcmp(print, txt_addr) {
            let mut str: *mut i8 = 0 as *mut i8;
            let mut name: *mut i8 = 0 as *mut i8;
            if opt.enable_iri as i32 != 0
                && {
                    name = idn_decode(print as *mut i8);
                    !name.is_null()
                }
            {
                str = aprintf(b"%s (%s)\0" as *const u8 as *const i8, name, print);
                rpl_free(name as *mut libc::c_void);
                name = 0 as *mut i8;
            }
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Connecting to %s|%s|:%d... \0" as *const u8 as *const i8,
                    5 as i32,
                ),
                if !str.is_null() { str } else { escnonprint_uri(print) },
                txt_addr,
                port,
            );
            rpl_free(str as *mut libc::c_void);
            str = 0 as *mut i8;
        } else if (*ip).family == 2 as i32 {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Connecting to %s:%d... \0" as *const u8 as *const i8,
                    5 as i32,
                ),
                txt_addr,
                port,
            );
        } else if (*ip).family == 10 as i32 {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    0 as *const i8,
                    b"Connecting to [%s]:%d... \0" as *const u8 as *const i8,
                    5 as i32,
                ),
                txt_addr,
                port,
            );
        }
    }
    sockaddr_set_data(sa, ip, port);
    sock = socket((*sa).sa_family as i32, __socket_type::SOCK_STREAM as i32, 0 as i32);
    if !(sock < 0 as i32) {
        if opt.ipv6_only {
            let mut on: i32 = 1 as i32;
            let mut err: i32 = setsockopt(
                sock,
                C2RustUnnamed_7::IPPROTO_IPV6 as i32,
                26 as i32,
                &mut on as *mut i32 as *const libc::c_void,
                ::core::mem::size_of::<i32>() as u64 as socklen_t,
            );
            if opt.debug as i64 != 0 {
                if err < 0 as i32 {
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"Failed setting IPV6_V6ONLY: %s\0" as *const u8
                                as *const i8,
                            strerror(*__errno_location()),
                        );
                    }
                }
            }
        }
        if opt.limit_rate != 0 && opt.limit_rate < 8192 as i32 as i64 {
            let mut bufsize: i32 = opt.limit_rate as i32;
            if bufsize < 512 as i32 {
                bufsize = 512 as i32;
            }
            if setsockopt(
                sock,
                1 as i32,
                8 as i32,
                &mut bufsize as *mut i32 as *mut libc::c_void,
                ::core::mem::size_of::<i32>() as u64 as socklen_t,
            ) != 0
            {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"setsockopt SO_RCVBUF failed: %s\n\0" as *const u8 as *const i8,
                        5 as i32,
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
                ) < 0 as i32
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
                ) < 0 as i32)
                {
                    if !print.is_null() {
                        logprintf(
                            log_options::LOG_VERBOSE,
                            dcgettext(
                                0 as *const i8,
                                b"connected.\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    if opt.debug as i64 != 0 {
                        debug_logprintf(
                            b"Created socket %d.\n\0" as *const u8 as *const i8,
                            sock,
                        );
                    }
                    return sock;
                }
            }
        }
    }
    let mut save_errno: i32 = *__errno_location();
    if sock >= 0 as i32 {
        fd_close(sock);
    }
    if !print.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"failed: %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
    }
    *__errno_location() = save_errno;
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn connect_to_host(mut host: *const i8, mut port: i32) -> i32 {
    let mut i: i32 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let mut sock: i32 = 0;
    let mut al: *mut address_list = lookup_host(host, 0 as i32);
    loop {
        if al.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"%s: unable to resolve host address %s\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                exec_name,
                quote(host),
            );
            return E_HOST as i32;
        }
        address_list_get_bounds(al, &mut start, &mut end);
        i = start;
        while i < end {
            let mut ip: *const ip_address = address_list_address_at(al, i);
            sock = connect_to_ip(ip, port, host);
            if sock >= 0 as i32 {
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
        al = lookup_host(host, C2RustUnnamed_9::LH_REFRESH as i32);
    }
    address_list_release(al);
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn bind_local(
    mut bind_address: *const ip_address,
    mut port: *mut i32,
) -> i32 {
    let mut sock: i32 = 0;
    let mut ss: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa: *mut sockaddr = &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut setopt_val: i32 = 1 as i32;
    let mut setopt_ptr: *mut libc::c_void = &mut setopt_val as *mut i32
        as *mut libc::c_void;
    let mut setopt_size: socklen_t = ::core::mem::size_of::<i32>() as u64 as socklen_t;
    sock = socket((*bind_address).family, __socket_type::SOCK_STREAM as i32, 0 as i32);
    if sock < 0 as i32 {
        return -(1 as i32);
    }
    if setsockopt(sock, 1 as i32, 2 as i32, setopt_ptr, setopt_size) != 0 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"setsockopt SO_REUSEADDR failed: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
    }
    memset(
        &mut ss as *mut sockaddr_storage as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<sockaddr_storage>() as u64,
    );
    sockaddr_set_data(sa, bind_address, *port);
    if bind(
        sock,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: sa,
        },
        sockaddr_size(sa),
    ) < 0 as i32
    {
        fd_close(sock);
        return -(1 as i32);
    }
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Local socket fd %d bound.\n\0" as *const u8 as *const i8,
            sock,
        );
    }
    if *port == 0 as i32 {
        let mut addrlen: socklen_t = sockaddr_size(sa);
        if getsockname(sock, __SOCKADDR_ARG { __sockaddr__: sa }, &mut addrlen)
            < 0 as i32
        {
            fd_close(sock);
            return -(1 as i32);
        }
        sockaddr_get_data(sa, 0 as *mut ip_address, port);
        if opt.debug as i64 != 0 {
            debug_logprintf(
                b"binding to address %s using port %i.\n\0" as *const u8 as *const i8,
                print_address(bind_address),
                *port,
            );
        }
    }
    if listen(sock, 1 as i32) < 0 as i32 {
        fd_close(sock);
        return -(1 as i32);
    }
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn accept_connection(mut local_sock: i32) -> i32 {
    let mut sock: i32 = 0;
    let mut ss: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa: *mut sockaddr = &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as u64
        as socklen_t;
    if opt.connect_timeout != 0. {
        let mut test: i32 = select_fd(
            local_sock,
            opt.connect_timeout,
            C2RustUnnamed_12::WAIT_FOR_READ as i32,
        );
        if test == 0 as i32 {
            *__errno_location() = 110 as i32;
        }
        if test <= 0 as i32 {
            return -(1 as i32);
        }
    }
    sock = accept(local_sock, __SOCKADDR_ARG { __sockaddr__: sa }, &mut addrlen);
    if opt.debug as i64 != 0 {
        debug_logprintf(
            b"Accepted client at socket %d.\n\0" as *const u8 as *const i8,
            sock,
        );
    }
    return sock;
}
#[no_mangle]
pub unsafe extern "C" fn socket_ip_address(
    mut sock: i32,
    mut ip: *mut ip_address,
    mut endpoint: i32,
) -> bool {
    let mut storage: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sockaddr: *mut sockaddr = &mut storage as *mut sockaddr_storage
        as *mut sockaddr;
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as u64
        as socklen_t;
    let mut ret: i32 = 0;
    memset(sockaddr as *mut libc::c_void, 0 as i32, addrlen as u64);
    if endpoint == C2RustUnnamed_11::ENDPOINT_LOCAL as i32 {
        ret = getsockname(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: sockaddr,
            },
            &mut addrlen,
        );
    } else if endpoint == C2RustUnnamed_11::ENDPOINT_PEER as i32 {
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
    if ret < 0 as i32 {
        return 0 as i32 != 0;
    }
    memset(
        ip as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<ip_address>() as u64,
    );
    (*ip).family = (*sockaddr).sa_family as i32;
    match (*sockaddr).sa_family as i32 {
        10 => {
            let mut sa6: *mut sockaddr_in6 = &mut storage as *mut sockaddr_storage
                as *mut sockaddr_in6;
            (*ip).data.d6 = (*sa6).sin6_addr;
            (*ip).ipv6_scope = (*sa6).sin6_scope_id as i32;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"conaddr is: %s\n\0" as *const u8 as *const i8,
                    print_address(ip),
                );
            }
            return 1 as i32 != 0;
        }
        2 => {
            let mut sa: *mut sockaddr_in = &mut storage as *mut sockaddr_storage
                as *mut sockaddr_in;
            (*ip).data.d4 = (*sa).sin_addr;
            if opt.debug as i64 != 0 {
                debug_logprintf(
                    b"conaddr is: %s\n\0" as *const u8 as *const i8,
                    print_address(ip),
                );
            }
            return 1 as i32 != 0;
        }
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn socket_family(mut sock: i32, mut endpoint: i32) -> i32 {
    let mut storage: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sockaddr: *mut sockaddr = &mut storage as *mut sockaddr_storage
        as *mut sockaddr;
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as u64
        as socklen_t;
    let mut ret: i32 = 0;
    memset(sockaddr as *mut libc::c_void, 0 as i32, addrlen as u64);
    if endpoint == C2RustUnnamed_11::ENDPOINT_LOCAL as i32 {
        ret = getsockname(
            sock,
            __SOCKADDR_ARG {
                __sockaddr__: sockaddr,
            },
            &mut addrlen,
        );
    } else if endpoint == C2RustUnnamed_11::ENDPOINT_PEER as i32 {
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
    if ret < 0 as i32 {
        return -(1 as i32);
    }
    return (*sockaddr).sa_family as i32;
}
#[no_mangle]
pub unsafe extern "C" fn retryable_socket_connect_error(mut err: i32) -> bool {
    if 0 as i32 != 0 || err == 97 as i32 || err == 96 as i32 || err == 94 as i32
        || err == 93 as i32 || err == 92 as i32 || err == 22 as i32
    {
        return 0 as i32 != 0;
    }
    if !opt.retry_connrefused {
        if err == 111 as i32 || err == 101 as i32 || err == 113 as i32 {
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn select_fd_internal(
    mut fd: i32,
    mut maxtime: libc::c_double,
    mut wait_for: i32,
    mut convert_back: bool,
) -> i32 {
    let mut fdset: fd_set = fd_set { fds_bits: [0; 16] };
    let mut rd: *mut fd_set = 0 as *mut fd_set;
    let mut wr: *mut fd_set = 0 as *mut fd_set;
    let mut tmout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut result: i32 = 0;
    if fd < 0 as i32 {
        return -(1 as i32);
    }
    if fd >= 1024 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Too many fds open.  Cannot use select on a fd >= %d\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            1024 as i32,
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    let mut __d0: i32 = 0;
    let mut __d1: i32 = 0;
    let fresh12 = &mut __d0;
    let fresh13;
    let fresh14 = (::core::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
    let fresh15 = &mut __d1;
    let fresh16;
    let fresh17 = &mut *(fdset.fds_bits).as_mut_ptr().offset(0 as i32 as isize)
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
        .fds_bits[(fd / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
        as usize]
        |= ((1 as u64)
            << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
            as __fd_mask;
    if wait_for & C2RustUnnamed_12::WAIT_FOR_READ as i32 != 0 {
        rd = &mut fdset;
    }
    if wait_for & C2RustUnnamed_12::WAIT_FOR_WRITE as i32 != 0 {
        wr = &mut fdset;
    }
    tmout.tv_sec = maxtime as i64;
    tmout.tv_usec = (1000000 as i32 as libc::c_double
        * (maxtime - maxtime as i64 as libc::c_double)) as __suseconds_t;
    loop {
        result = select(fd + 1 as i32, rd, wr, 0 as *mut fd_set, &mut tmout);
        if !(result < 0 as i32 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn select_fd(
    mut fd: i32,
    mut maxtime: libc::c_double,
    mut wait_for: i32,
) -> i32 {
    return select_fd_internal(fd, maxtime, wait_for, 1 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn test_socket_open(mut sock: i32) -> bool {
    let mut check_set: fd_set = fd_set { fds_bits: [0; 16] };
    let mut to: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ret: i32 = 0 as i32;
    if sock >= 1024 as i32 {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Too many fds open.  Cannot use select on a fd >= %d\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            1024 as i32,
        );
        exit(C2RustUnnamed_4::WGET_EXIT_GENERIC_ERROR as i32);
    }
    let mut __d0: i32 = 0;
    let mut __d1: i32 = 0;
    let fresh18 = &mut __d0;
    let fresh19;
    let fresh20 = (::core::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
    let fresh21 = &mut __d1;
    let fresh22;
    let fresh23 = &mut *(check_set.fds_bits).as_mut_ptr().offset(0 as i32 as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh18,
        fresh20) => fresh19, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh21,
        fresh23) => fresh22, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
    c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
    check_set
        .fds_bits[(sock / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
        as usize]
        |= ((1 as u64)
            << sock % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
            as __fd_mask;
    to.tv_sec = 0 as i32 as __time_t;
    to.tv_usec = 1 as i32 as __suseconds_t;
    ret = select(
        sock + 1 as i32,
        &mut check_set,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut to,
    );
    if ret == 0 { return 1 as i32 != 0 } else { return 0 as i32 != 0 };
}
unsafe extern "C" fn sock_read(mut fd: i32, mut buf: *mut i8, mut bufsize: i32) -> i32 {
    let mut res: i32 = 0;
    loop {
        res = read(fd, buf as *mut libc::c_void, bufsize as size_t) as i32;
        if !(res == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn sock_write(mut fd: i32, mut buf: *mut i8, mut bufsize: i32) -> i32 {
    let mut res: i32 = 0;
    loop {
        res = write(fd, buf as *const libc::c_void, bufsize as size_t) as i32;
        if !(res == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn sock_poll(
    mut fd: i32,
    mut timeout: libc::c_double,
    mut wait_for: i32,
) -> i32 {
    return select_fd(fd, timeout, wait_for);
}
unsafe extern "C" fn sock_peek(mut fd: i32, mut buf: *mut i8, mut bufsize: i32) -> i32 {
    let mut res: i32 = 0;
    loop {
        res = recv(fd, buf as *mut libc::c_void, bufsize as size_t, MSG_PEEK as i32)
            as i32;
        if !(res == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn sock_close(mut fd: i32) {
    close(fd);
    if opt.debug as i64 != 0 {
        debug_logprintf(b"Closed fd %d\n\0" as *const u8 as *const i8, fd);
    }
}
static mut transport_map: *mut hash_table = 0 as *const hash_table as *mut hash_table;
static mut transport_map_modified_tick: u32 = 0;
#[no_mangle]
pub unsafe extern "C" fn fd_register_transport(
    mut fd: i32,
    mut imp: *mut transport_implementation,
    mut ctx: *mut libc::c_void,
) {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    info = xmalloc(::core::mem::size_of::<transport_info>() as u64)
        as *mut transport_info;
    (*info).imp = imp;
    (*info).ctx = ctx;
    if transport_map.is_null() {
        transport_map = hash_table_new(0 as i32, None, None);
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
pub unsafe extern "C" fn fd_transport_context(mut fd: i32) -> *mut libc::c_void {
    let mut info: *mut transport_info = hash_table_get(
        transport_map,
        fd as intptr_t as *mut libc::c_void,
    ) as *mut transport_info;
    return if !info.is_null() { (*info).ctx } else { 0 as *mut libc::c_void };
}
unsafe extern "C" fn poll_internal(
    mut fd: i32,
    mut info: *mut transport_info,
    mut wf: i32,
    mut timeout: libc::c_double,
) -> bool {
    if timeout == -(1 as i32) as libc::c_double {
        timeout = opt.read_timeout;
    }
    if timeout != 0. {
        let mut test: i32 = 0;
        if !info.is_null() && ((*(*info).imp).poller).is_some() {
            test = ((*(*info).imp).poller)
                .expect("non-null function pointer")(fd, timeout, wf, (*info).ctx);
        } else {
            test = sock_poll(fd, timeout, wf);
        }
        if test == 0 as i32 {
            *__errno_location() = 110 as i32;
        }
        if test <= 0 as i32 {
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fd_read(
    mut fd: i32,
    mut buf: *mut i8,
    mut bufsize: i32,
    mut timeout: libc::c_double,
) -> i32 {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    static mut last_info: *mut transport_info = 0 as *const transport_info
        as *mut transport_info;
    static mut last_fd: i32 = -(1 as i32);
    static mut last_tick: u32 = 0;
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
    if !poll_internal(fd, info, C2RustUnnamed_12::WAIT_FOR_READ as i32, timeout) {
        return -(1 as i32);
    }
    return sock_read(fd, buf, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn fd_peek(
    mut fd: i32,
    mut buf: *mut i8,
    mut bufsize: i32,
    mut timeout: libc::c_double,
) -> i32 {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    static mut last_info: *mut transport_info = 0 as *const transport_info
        as *mut transport_info;
    static mut last_fd: i32 = -(1 as i32);
    static mut last_tick: u32 = 0;
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
    if !poll_internal(fd, info, C2RustUnnamed_12::WAIT_FOR_READ as i32, timeout) {
        return -(1 as i32);
    }
    return sock_peek(fd, buf, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn fd_write(
    mut fd: i32,
    mut buf: *mut i8,
    mut bufsize: i32,
    mut timeout: libc::c_double,
) -> i32 {
    let mut res: i32 = 0;
    let mut info: *mut transport_info = 0 as *mut transport_info;
    static mut last_info: *mut transport_info = 0 as *const transport_info
        as *mut transport_info;
    static mut last_fd: i32 = -(1 as i32);
    static mut last_tick: u32 = 0;
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
    res = 0 as i32;
    while bufsize > 0 as i32 {
        if !poll_internal(fd, info, C2RustUnnamed_12::WAIT_FOR_WRITE as i32, timeout) {
            return -(1 as i32);
        }
        if !info.is_null() && ((*(*info).imp).writer).is_some() {
            res = ((*(*info).imp).writer)
                .expect("non-null function pointer")(fd, buf, bufsize, (*info).ctx);
        } else {
            res = sock_write(fd, buf, bufsize);
        }
        if res <= 0 as i32 {
            break;
        }
        buf = buf.offset(res as isize);
        bufsize -= res;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn fd_errstr(mut fd: i32) -> *const i8 {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    if !transport_map.is_null() {
        info = hash_table_get(transport_map, fd as intptr_t as *mut libc::c_void)
            as *mut transport_info;
    }
    if !info.is_null() && ((*(*info).imp).errstr).is_some() {
        let mut err: *const i8 = ((*(*info).imp).errstr)
            .expect("non-null function pointer")(fd, (*info).ctx);
        if !err.is_null() {
            return err;
        }
    }
    return strerror(*__errno_location());
}
#[no_mangle]
pub unsafe extern "C" fn fd_close(mut fd: i32) {
    let mut info: *mut transport_info = 0 as *mut transport_info;
    if fd < 0 as i32 {
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