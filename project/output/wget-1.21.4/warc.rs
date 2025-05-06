#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type hash_table;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn rewind(__stream: *mut FILE);
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn feof(__stream: *mut FILE) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn dcngettext(
        __domainname: *const i8,
        __msgid1: *const i8,
        __msgid2: *const i8,
        __n: u64,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strtok_r(__s: *mut i8, __delim: *const i8, __save_ptr: *mut *mut i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    static mut opt: options;
    fn exit(_: i32) -> !;
    fn mkostemp(__template: *mut i8, __flags: i32) -> i32;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn rpl_fopen(filename: *const i8, mode: *const i8) -> *mut FILE;
    fn rpl_fseek(fp: *mut FILE, offset: i64, whence: i32) -> i32;
    fn rpl_fseeko(fp: *mut FILE, offset: off_t, whence: i32) -> i32;
    fn log_set_warc_log_fp(_: *mut FILE);
    fn logprintf(_: log_options, _: *const i8, _: ...);
    fn quote(arg: *const i8) -> *const i8;
    static mut program_argstring: *const i8;
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
    fn hash_table_count(_: *const hash_table) -> i32;
    fn aprintf(_: *const i8, _: ...) -> *mut i8;
    fn number_to_string(_: *mut i8, _: wgint) -> *mut i8;
    fn random_number(_: i32) -> i32;
    static mut version_string: *const i8;
    fn base_name(file: *const i8) -> *mut i8;
    fn url_escape(_: *const i8) -> *mut i8;
    fn path_search(
        tmpl: *mut i8,
        tmpl_len: size_t,
        dir: *const i8,
        pfx: *const i8,
        try_tmpdir: bool,
    ) -> i32;
    fn sha1_init_ctx(ctx: *mut sha1_ctx);
    fn sha1_process_block(buffer: *const libc::c_void, len: size_t, ctx: *mut sha1_ctx);
    fn sha1_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut sha1_ctx);
    fn sha1_finish_ctx(
        ctx: *mut sha1_ctx,
        resbuf: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sha1_stream(stream: *mut FILE, resblock: *mut libc::c_void) -> i32;
    fn base32_encode(in_0: *const i8, inlen: idx_t, out: *mut i8, outlen: idx_t);
    fn base32_decode_alloc_ctx(
        ctx: *mut base32_decode_context,
        in_0: *const i8,
        inlen: idx_t,
        out: *mut *mut i8,
        outlen: *mut idx_t,
    ) -> bool;
    fn close(__fd: i32) -> i32;
    fn dup(__fd: i32) -> i32;
    fn unlink(__name: *const i8) -> i32;
    fn ftruncate(__fd: i32, __length: __off_t) -> i32;
    fn gzdopen(fd: i32, mode: *const i8) -> gzFile;
    fn gzwrite(file: gzFile, buf: voidpc, len: u32) -> i32;
    fn gzclose(file: gzFile) -> i32;
    fn print_address(_: *const ip_address) -> *const i8;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
pub type ptrdiff_t = i64;
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
pub type idx_t = ptrdiff_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub E: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base32_decode_context {
    pub i: i32,
    pub buf: [i8; 8],
}
pub type voidpc = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: u32,
    pub next: *mut u8,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub data: C2RustUnnamed_5,
    pub ipv6_scope: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub d4: in_addr,
    pub d6: in6_addr,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_6 {
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
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_6::WGET_EXIT_SUCCESS => 0,
            C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR => 1,
            C2RustUnnamed_6::WGET_EXIT_PARSE_ERROR => 2,
            C2RustUnnamed_6::WGET_EXIT_IO_FAIL => 3,
            C2RustUnnamed_6::WGET_EXIT_NETWORK_FAIL => 4,
            C2RustUnnamed_6::WGET_EXIT_SSL_AUTH_FAIL => 5,
            C2RustUnnamed_6::WGET_EXIT_SERVER_AUTH_FAIL => 6,
            C2RustUnnamed_6::WGET_EXIT_PROTOCOL_ERROR => 7,
            C2RustUnnamed_6::WGET_EXIT_SERVER_ERROR => 8,
            C2RustUnnamed_6::WGET_EXIT_UNKNOWN => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_6 {
        match value {
            0 => C2RustUnnamed_6::WGET_EXIT_SUCCESS,
            1 => C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR,
            2 => C2RustUnnamed_6::WGET_EXIT_PARSE_ERROR,
            3 => C2RustUnnamed_6::WGET_EXIT_IO_FAIL,
            4 => C2RustUnnamed_6::WGET_EXIT_NETWORK_FAIL,
            5 => C2RustUnnamed_6::WGET_EXIT_SSL_AUTH_FAIL,
            6 => C2RustUnnamed_6::WGET_EXIT_SERVER_AUTH_FAIL,
            7 => C2RustUnnamed_6::WGET_EXIT_PROTOCOL_ERROR,
            8 => C2RustUnnamed_6::WGET_EXIT_SERVER_ERROR,
            9 => C2RustUnnamed_6::WGET_EXIT_UNKNOWN,
            _ => panic!("Invalid value for C2RustUnnamed_6: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_6 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_6 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_6 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_6 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_6 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn add(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn sub(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn mul(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn div(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn rem(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warc_cdx_record {
    pub url: *mut i8,
    pub uuid: *mut i8,
    pub digest: [i8; 20],
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
static mut warc_log_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_manifest_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_current_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_current_gzfile: gzFile = 0 as *const gzFile_s as *mut gzFile_s;
static mut warc_current_gzfile_offset: off_t = 0;
static mut warc_current_gzfile_uncompressed_size: off_t = 0;
static mut warc_write_ok: bool = false;
static mut warc_current_cdx_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut warc_current_warcinfo_uuid_str: [i8; 48] = [0; 48];
static mut warc_current_filename: *mut i8 = 0 as *const i8 as *mut i8;
static mut warc_current_file_number: i32 = 0;
static mut warc_cdx_dedup_table: *mut hash_table = 0 as *const hash_table
    as *mut hash_table;
unsafe extern "C" fn warc_hash_sha1_digest(mut key: *const libc::c_void) -> u64 {
    let mut v: u64 = 0 as i32 as u64;
    memcpy(
        &mut v as *mut u64 as *mut libc::c_void,
        key,
        ::core::mem::size_of::<u64>() as u64,
    );
    return v;
}
unsafe extern "C" fn warc_cmp_sha1_digest(
    mut digest1: *const libc::c_void,
    mut digest2: *const libc::c_void,
) -> i32 {
    return (memcmp(digest1, digest2, 20 as i32 as u64) == 0) as i32;
}
unsafe extern "C" fn warc_write_buffer(
    mut buffer: *const i8,
    mut size: size_t,
) -> size_t {
    if !warc_current_gzfile.is_null() {
        warc_current_gzfile_uncompressed_size = (warc_current_gzfile_uncompressed_size
            as u64)
            .wrapping_add(size) as off_t as off_t;
        return gzwrite(warc_current_gzfile, buffer as voidpc, size as u32) as size_t;
    } else {
        return fwrite(
            buffer as *const libc::c_void,
            1 as i32 as size_t,
            size,
            warc_current_file,
        )
    };
}
unsafe extern "C" fn warc_write_string(mut str: *const i8) -> bool {
    let mut n: size_t = 0;
    if !warc_write_ok {
        return 0 as i32 != 0;
    }
    n = strlen(str);
    if n != warc_write_buffer(str, n) {
        warc_write_ok = 0 as i32 != 0;
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_start_record() -> bool {
    if !warc_write_ok {
        return 0 as i32 != 0;
    }
    rpl_fflush(warc_current_file);
    if opt.warc_maxsize > 0 as i32 as i64
        && ftello(warc_current_file) >= opt.warc_maxsize
    {
        warc_start_new_file(0 as i32 != 0);
    }
    if opt.warc_compression_enabled {
        let mut dup_fd: i32 = 0;
        warc_current_gzfile_offset = ftello(warc_current_file);
        if rpl_fseek(warc_current_file, 14 as i32 as i64, 1 as i32) < 0 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Error setting WARC file position.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            warc_write_ok = 0 as i32 != 0;
            return 0 as i32 != 0;
        }
        if rpl_fflush(warc_current_file) != 0 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Error flushing WARC file to disk.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            warc_write_ok = 0 as i32 != 0;
            return 0 as i32 != 0;
        }
        dup_fd = dup(fileno(warc_current_file));
        if dup_fd < 0 as i32 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Error duplicating WARC file file descriptor.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            warc_write_ok = 0 as i32 != 0;
            return 0 as i32 != 0;
        }
        warc_current_gzfile = gzdopen(dup_fd, b"wb9\0" as *const u8 as *const i8);
        warc_current_gzfile_uncompressed_size = 0 as i32 as off_t;
        if warc_current_gzfile.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Error opening GZIP stream to WARC file.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            close(dup_fd);
            warc_write_ok = 0 as i32 != 0;
            return 0 as i32 != 0;
        }
    }
    warc_write_string(b"WARC/1.0\r\n\0" as *const u8 as *const i8);
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_header(
    mut name: *const i8,
    mut value: *const i8,
) -> bool {
    if !value.is_null() {
        warc_write_string(name);
        warc_write_string(b": \0" as *const u8 as *const i8);
        warc_write_string(value);
        warc_write_string(b"\r\n\0" as *const u8 as *const i8);
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_header_uri(
    mut name: *const i8,
    mut value: *const i8,
) -> bool {
    if !value.is_null() {
        warc_write_string(name);
        warc_write_string(b": <\0" as *const u8 as *const i8);
        warc_write_string(value);
        warc_write_string(b">\r\n\0" as *const u8 as *const i8);
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_block_from_file(mut data_in: *mut FILE) -> bool {
    let mut content_length: [i8; 21] = [0; 21];
    let mut buffer: [i8; 8192] = [0; 8192];
    let mut s: size_t = 0;
    rpl_fseeko(data_in, 0 as i64, 2 as i32);
    number_to_string(content_length.as_mut_ptr(), ftello(data_in));
    warc_write_header(
        b"Content-Length\0" as *const u8 as *const i8,
        content_length.as_mut_ptr(),
    );
    warc_write_string(b"\r\n\0" as *const u8 as *const i8);
    if rpl_fseeko(data_in, 0 as i64, 0 as i32) != 0 as i32 {
        warc_write_ok = 0 as i32 != 0;
    }
    while warc_write_ok as i32 != 0
        && {
            s = fread(
                buffer.as_mut_ptr() as *mut libc::c_void,
                1 as i32 as size_t,
                8192 as i32 as size_t,
                data_in,
            );
            s > 0 as i32 as u64
        }
    {
        if warc_write_buffer(buffer.as_mut_ptr(), s) < s {
            warc_write_ok = 0 as i32 != 0;
        }
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_end_record() -> bool {
    if warc_write_buffer(b"\r\n\r\n\0" as *const u8 as *const i8, 4 as i32 as size_t)
        != 4 as i32 as u64
    {
        warc_write_ok = 0 as i32 != 0;
        return 0 as i32 != 0;
    }
    if warc_write_ok as i32 != 0 && !warc_current_gzfile.is_null() {
        let mut extra_header: [i8; 14] = [0; 14];
        let mut static_header: [i8; 10] = [0; 10];
        let mut current_offset: off_t = 0;
        let mut uncompressed_size: off_t = 0;
        let mut compressed_size: off_t = 0;
        let mut result: size_t = 0;
        if gzclose(warc_current_gzfile) != 0 as i32 {
            warc_write_ok = 0 as i32 != 0;
            return 0 as i32 != 0;
        }
        rpl_fflush(warc_current_file);
        rpl_fseeko(warc_current_file, 0 as i32 as off_t, 2 as i32);
        current_offset = ftello(warc_current_file);
        uncompressed_size = current_offset - warc_current_gzfile_offset;
        compressed_size = warc_current_gzfile_uncompressed_size;
        result = rpl_fseeko(
            warc_current_file,
            warc_current_gzfile_offset + 14 as i32 as i64,
            0 as i32,
        ) as size_t;
        if result != 0 as i32 as u64 {
            warc_write_ok = 0 as i32 != 0;
            return 0 as i32 != 0;
        }
        result = fread(
            static_header.as_mut_ptr() as *mut libc::c_void,
            1 as i32 as size_t,
            10 as i32 as size_t,
            warc_current_file,
        );
        if result != 10 as i32 as u64 {
            warc_write_ok = 0 as i32 != 0;
            return 0 as i32 != 0;
        }
        static_header[3 as i32 as usize] = (static_header[3 as i32 as usize] as i32
            | 0x4 as i32) as i8;
        rpl_fseeko(warc_current_file, warc_current_gzfile_offset, 0 as i32);
        fwrite(
            static_header.as_mut_ptr() as *const libc::c_void,
            1 as i32 as size_t,
            10 as i32 as size_t,
            warc_current_file,
        );
        extra_header[0 as i32 as usize] = (14 as i32 - 2 as i32 & 255 as i32) as i8;
        extra_header[1 as i32 as usize] = (14 as i32 - 2 as i32 >> 8 as i32 & 255 as i32)
            as i8;
        extra_header[2 as i32 as usize] = 's' as i32 as i8;
        extra_header[3 as i32 as usize] = 'l' as i32 as i8;
        extra_header[4 as i32 as usize] = (8 as i32 & 255 as i32) as i8;
        extra_header[5 as i32 as usize] = (8 as i32 >> 8 as i32 & 255 as i32) as i8;
        extra_header[6 as i32 as usize] = (uncompressed_size & 255 as i32 as i64) as i8;
        extra_header[7 as i32 as usize] = (uncompressed_size >> 8 as i32
            & 255 as i32 as i64) as i8;
        extra_header[8 as i32 as usize] = (uncompressed_size >> 16 as i32
            & 255 as i32 as i64) as i8;
        extra_header[9 as i32 as usize] = (uncompressed_size >> 24 as i32
            & 255 as i32 as i64) as i8;
        extra_header[10 as i32 as usize] = (compressed_size & 255 as i32 as i64) as i8;
        extra_header[11 as i32 as usize] = (compressed_size >> 8 as i32
            & 255 as i32 as i64) as i8;
        extra_header[12 as i32 as usize] = (compressed_size >> 16 as i32
            & 255 as i32 as i64) as i8;
        extra_header[13 as i32 as usize] = (compressed_size >> 24 as i32
            & 255 as i32 as i64) as i8;
        rpl_fseeko(
            warc_current_file,
            warc_current_gzfile_offset + 10 as i32 as i64,
            0 as i32,
        );
        fwrite(
            extra_header.as_mut_ptr() as *const libc::c_void,
            1 as i32 as size_t,
            14 as i32 as size_t,
            warc_current_file,
        );
        rpl_fflush(warc_current_file);
        rpl_fseeko(warc_current_file, 0 as i32 as off_t, 2 as i32);
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_date_header(mut timestamp: *const i8) -> bool {
    let mut current_timestamp: [i8; 21] = [0; 21];
    return warc_write_header(
        b"WARC-Date\0" as *const u8 as *const i8,
        if !timestamp.is_null() {
            timestamp
        } else {
            warc_timestamp(
                current_timestamp.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 21]>() as u64,
            )
        },
    );
}
unsafe extern "C" fn warc_write_ip_header(mut ip: *const ip_address) -> bool {
    if !ip.is_null() {
        return warc_write_header(
            b"WARC-IP-Address\0" as *const u8 as *const i8,
            print_address(ip),
        )
    } else {
        return warc_write_ok
    };
}
unsafe extern "C" fn warc_sha1_stream_with_payload(
    mut stream: *mut FILE,
    mut res_block: *mut libc::c_void,
    mut res_payload: *mut libc::c_void,
    mut payload_offset: off_t,
) -> i32 {
    let mut ctx_block: sha1_ctx = sha1_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        E: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut ctx_payload: sha1_ctx = sha1_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        E: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut pos: off_t = 0;
    let mut sum: off_t = 0;
    let mut buffer: *mut i8 = xmalloc((32768 as i32 + 72 as i32) as size_t) as *mut i8;
    sha1_init_ctx(&mut ctx_block);
    if payload_offset >= 0 as i32 as i64 {
        sha1_init_ctx(&mut ctx_payload);
    }
    pos = 0 as i32 as off_t;
    's_28: loop {
        let mut n: off_t = 0;
        sum = 0 as i32 as off_t;
        loop {
            n = fread(
                buffer.offset(sum as isize) as *mut libc::c_void,
                1 as i32 as size_t,
                (32768 as i32 as i64 - sum) as size_t,
                stream,
            ) as off_t;
            sum += n;
            pos += n;
            if sum == 32768 as i32 as i64 {
                break;
            }
            if n == 0 as i32 as i64 {
                if ferror(stream) != 0 {
                    rpl_free(buffer as *mut libc::c_void);
                    buffer = 0 as *mut i8;
                    return 1 as i32;
                }
                break 's_28;
            } else if feof(stream) != 0 {
                break 's_28;
            }
        }
        sha1_process_block(
            buffer as *const libc::c_void,
            32768 as i32 as size_t,
            &mut ctx_block,
        );
        if payload_offset >= 0 as i32 as i64 && payload_offset < pos {
            let mut start_of_payload: off_t = payload_offset
                - (pos - 32768 as i32 as i64);
            if start_of_payload <= 0 as i32 as i64 {
                start_of_payload = 0 as i32 as off_t;
            }
            sha1_process_bytes(
                buffer.offset(start_of_payload as isize) as *const libc::c_void,
                (32768 as i32 as i64 - start_of_payload) as size_t,
                &mut ctx_payload,
            );
        }
    }
    if sum > 0 as i32 as i64 {
        sha1_process_bytes(buffer as *const libc::c_void, sum as size_t, &mut ctx_block);
        if payload_offset >= 0 as i32 as i64 && payload_offset < pos {
            let mut start_of_payload_0: off_t = payload_offset - (pos - sum);
            if start_of_payload_0 <= 0 as i32 as i64 {
                start_of_payload_0 = 0 as i32 as off_t;
            }
            sha1_process_bytes(
                buffer.offset(start_of_payload_0 as isize) as *const libc::c_void,
                (sum - start_of_payload_0) as size_t,
                &mut ctx_payload,
            );
        }
    }
    sha1_finish_ctx(&mut ctx_block, res_block);
    if payload_offset >= 0 as i32 as i64 {
        sha1_finish_ctx(&mut ctx_payload, res_payload);
    }
    rpl_free(buffer as *mut libc::c_void);
    buffer = 0 as *mut i8;
    return 0 as i32;
}
unsafe extern "C" fn warc_base32_sha1_digest(
    mut sha1_digest: *const i8,
    mut sha1_base32: *mut i8,
    mut sha1_base32_size: size_t,
) -> *mut i8 {
    if sha1_base32_size
        >= ((20 as i32 + 4 as i32) / 5 as i32 * 8 as i32 + 5 as i32 + 1 as i32) as u64
    {
        memcpy(
            sha1_base32 as *mut libc::c_void,
            b"sha1:\0" as *const u8 as *const i8 as *const libc::c_void,
            5 as i32 as u64,
        );
        base32_encode(
            sha1_digest,
            20 as i32 as idx_t,
            sha1_base32.offset(5 as i32 as isize),
            sha1_base32_size.wrapping_sub(5 as i32 as u64) as idx_t,
        );
    } else {
        *sha1_base32 = 0 as i32 as i8;
    }
    return sha1_base32;
}
unsafe extern "C" fn warc_write_digest_headers(
    mut file: *mut FILE,
    mut payload_offset: i64,
) {
    if opt.warc_digests_enabled {
        let mut sha1_res_block: [i8; 20] = [0; 20];
        let mut sha1_res_payload: [i8; 20] = [0; 20];
        rewind(file);
        if warc_sha1_stream_with_payload(
            file,
            sha1_res_block.as_mut_ptr() as *mut libc::c_void,
            sha1_res_payload.as_mut_ptr() as *mut libc::c_void,
            payload_offset,
        ) == 0 as i32
        {
            let mut digest: [i8; 38] = [0; 38];
            warc_write_header(
                b"WARC-Block-Digest\0" as *const u8 as *const i8,
                warc_base32_sha1_digest(
                    sha1_res_block.as_mut_ptr(),
                    digest.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 38]>() as u64,
                ),
            );
            if payload_offset >= 0 as i32 as i64 {
                warc_write_header(
                    b"WARC-Payload-Digest\0" as *const u8 as *const i8,
                    warc_base32_sha1_digest(
                        sha1_res_payload.as_mut_ptr(),
                        digest.as_mut_ptr(),
                        ::core::mem::size_of::<[i8; 38]>() as u64,
                    ),
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn warc_timestamp(
    mut timestamp: *mut i8,
    mut timestamp_size: size_t,
) -> *mut i8 {
    let mut rawtime: time_t = time(0 as *mut time_t);
    let mut timeinfo: *mut tm = gmtime(&mut rawtime);
    if strftime(
        timestamp,
        timestamp_size,
        b"%Y-%m-%dT%H:%M:%SZ\0" as *const u8 as *const i8,
        timeinfo,
    ) == 0 as i32 as u64 && timestamp_size > 0 as i32 as u64
    {
        *timestamp = 0 as i32 as i8;
    }
    return timestamp;
}
#[no_mangle]
pub unsafe extern "C" fn warc_uuid_str(mut urn_str: *mut i8, mut urn_size: size_t) {
    let mut uuid_data: [u8; 16] = [0; 16];
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 16 as i32 {
        uuid_data[i as usize] = random_number(255 as i32) as u8;
        i += 1;
        i;
    }
    uuid_data[6 as i32 as usize] = (uuid_data[6 as i32 as usize] as i32 & 0xf as i32
        | 0x40 as i32) as u8;
    uuid_data[8 as i32 as usize] = (uuid_data[8 as i32 as usize] as i32 & 0xbf as i32
        | 0x80 as i32) as u8;
    snprintf(
        urn_str,
        urn_size,
        b"<urn:uuid:%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x>\0"
            as *const u8 as *const i8,
        uuid_data[0 as i32 as usize] as i32,
        uuid_data[1 as i32 as usize] as i32,
        uuid_data[2 as i32 as usize] as i32,
        uuid_data[3 as i32 as usize] as i32,
        uuid_data[4 as i32 as usize] as i32,
        uuid_data[5 as i32 as usize] as i32,
        uuid_data[6 as i32 as usize] as i32,
        uuid_data[7 as i32 as usize] as i32,
        uuid_data[8 as i32 as usize] as i32,
        uuid_data[9 as i32 as usize] as i32,
        uuid_data[10 as i32 as usize] as i32,
        uuid_data[11 as i32 as usize] as i32,
        uuid_data[12 as i32 as usize] as i32,
        uuid_data[13 as i32 as usize] as i32,
        uuid_data[14 as i32 as usize] as i32,
        uuid_data[15 as i32 as usize] as i32,
    );
}
unsafe extern "C" fn warc_write_warcinfo_record(mut filename: *const i8) -> bool {
    let mut warc_tmp: *mut FILE = 0 as *mut FILE;
    let mut timestamp: [i8; 22] = [0; 22];
    let mut filename_basename: *mut i8 = 0 as *mut i8;
    warc_uuid_str(
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 48]>() as u64,
    );
    warc_timestamp(timestamp.as_mut_ptr(), ::core::mem::size_of::<[i8; 22]>() as u64);
    filename_basename = base_name(filename);
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const i8,
        b"warcinfo\0" as *const u8 as *const i8,
    );
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const i8,
        b"application/warc-fields\0" as *const u8 as *const i8,
    );
    warc_write_header(b"WARC-Date\0" as *const u8 as *const i8, timestamp.as_mut_ptr());
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const i8,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(b"WARC-Filename\0" as *const u8 as *const i8, filename_basename);
    rpl_free(filename_basename as *mut libc::c_void);
    filename_basename = 0 as *mut i8;
    warc_tmp = warc_tempfile();
    if warc_tmp.is_null() {
        return 0 as i32 != 0;
    }
    fprintf(
        warc_tmp,
        b"software: Wget/%s (%s)\r\n\0" as *const u8 as *const i8,
        version_string,
        b"linux-gnu\0" as *const u8 as *const i8,
    );
    fprintf(warc_tmp, b"format: WARC File Format 1.0\r\n\0" as *const u8 as *const i8);
    fprintf(
        warc_tmp,
        b"conformsTo: http://bibnum.bnf.fr/WARC/WARC_ISO_28500_version1_latestdraft.pdf\r\n\0"
            as *const u8 as *const i8,
    );
    fprintf(
        warc_tmp,
        b"robots: %s\r\n\0" as *const u8 as *const i8,
        if opt.use_robots as i32 != 0 {
            b"classic\0" as *const u8 as *const i8
        } else {
            b"off\0" as *const u8 as *const i8
        },
    );
    fprintf(
        warc_tmp,
        b"wget-arguments: %s\r\n\0" as *const u8 as *const i8,
        program_argstring,
    );
    if !(opt.warc_user_headers).is_null() {
        let mut i: i32 = 0;
        i = 0 as i32;
        while !(*(opt.warc_user_headers).offset(i as isize)).is_null() {
            fprintf(
                warc_tmp,
                b"%s\r\n\0" as *const u8 as *const i8,
                *(opt.warc_user_headers).offset(i as isize),
            );
            i += 1;
            i;
        }
    }
    fprintf(warc_tmp, b"\r\n\0" as *const u8 as *const i8);
    warc_write_digest_headers(warc_tmp, -(1 as i32) as i64);
    warc_write_block_from_file(warc_tmp);
    warc_write_end_record();
    if !warc_write_ok {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Error writing warcinfo record to WARC file.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    fclose(warc_tmp);
    return warc_write_ok;
}
unsafe extern "C" fn warc_start_new_file(mut meta: bool) -> bool {
    let mut extension: *const i8 = if opt.warc_compression_enabled as i32 != 0 {
        b"warc.gz\0" as *const u8 as *const i8
    } else {
        b"warc\0" as *const u8 as *const i8
    };
    let mut base_filename_length: i32 = 0;
    let mut new_filename: *mut i8 = 0 as *mut i8;
    if (opt.warc_filename).is_null() {
        return 0 as i32 != 0;
    }
    if !warc_current_file.is_null() {
        fclose(warc_current_file);
    }
    *warc_current_warcinfo_uuid_str.as_mut_ptr() = 0 as i32 as i8;
    rpl_free(warc_current_filename as *mut libc::c_void);
    warc_current_filename = 0 as *mut i8;
    warc_current_file_number += 1;
    warc_current_file_number;
    base_filename_length = strlen(opt.warc_filename) as i32;
    new_filename = xmalloc(
        (base_filename_length + 1 as i32 + 5 as i32 + 8 as i32 + 1 as i32) as size_t,
    ) as *mut i8;
    warc_current_filename = new_filename;
    if meta {
        sprintf(
            new_filename,
            b"%s-meta.%s\0" as *const u8 as *const i8,
            opt.warc_filename,
            extension,
        );
    } else if opt.warc_maxsize > 0 as i32 as i64 {
        sprintf(
            new_filename,
            b"%s-%05d.%s\0" as *const u8 as *const i8,
            opt.warc_filename,
            warc_current_file_number,
            extension,
        );
    } else {
        sprintf(
            new_filename,
            b"%s.%s\0" as *const u8 as *const i8,
            opt.warc_filename,
            extension,
        );
    }
    logprintf(
        log_options::LOG_VERBOSE,
        dcgettext(
            0 as *const i8,
            b"Opening WARC file %s.\n\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        quote(new_filename),
    );
    warc_current_file = rpl_fopen(new_filename, b"wb+\0" as *const u8 as *const i8);
    if warc_current_file.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Error opening WARC file %s.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(new_filename),
        );
        return 0 as i32 != 0;
    }
    if !warc_write_warcinfo_record(new_filename) {
        return 0 as i32 != 0;
    }
    if !warc_manifest_fp.is_null() {
        fprintf(
            warc_manifest_fp,
            b"%s\n\0" as *const u8 as *const i8,
            warc_current_warcinfo_uuid_str.as_mut_ptr(),
        );
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn warc_start_cdx_file() -> bool {
    let mut cdx_filename: *mut i8 = aprintf(
        b"%s.cdx\0" as *const u8 as *const i8,
        opt.warc_filename,
    );
    warc_current_cdx_file = rpl_fopen(cdx_filename, b"a+\0" as *const u8 as *const i8);
    rpl_free(cdx_filename as *mut libc::c_void);
    if warc_current_cdx_file.is_null() {
        return 0 as i32 != 0;
    }
    fprintf(
        warc_current_cdx_file,
        b" CDX a b a m s k r M V g u\n\0" as *const u8 as *const i8,
    );
    rpl_fflush(warc_current_cdx_file);
    return 1 as i32 != 0;
}
unsafe extern "C" fn warc_parse_cdx_header(
    mut lineptr: *mut i8,
    mut field_num_original_url: *mut i32,
    mut field_num_checksum: *mut i32,
    mut field_num_record_id: *mut i32,
) -> bool {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut save_ptr: *mut i8 = 0 as *mut i8;
    *field_num_original_url = -(1 as i32);
    *field_num_checksum = -(1 as i32);
    *field_num_record_id = -(1 as i32);
    token = strtok_r(lineptr, b" \t\r\n\0" as *const u8 as *const i8, &mut save_ptr);
    if !token.is_null() && strcmp(token, b"CDX\0" as *const u8 as *const i8) == 0 as i32
    {
        let mut field_num: i32 = 0 as i32;
        while !token.is_null() {
            token = strtok_r(
                0 as *mut i8,
                b" \t\r\n\0" as *const u8 as *const i8,
                &mut save_ptr,
            );
            if !token.is_null() {
                match *token.offset(0 as i32 as isize) as i32 {
                    97 => {
                        *field_num_original_url = field_num;
                    }
                    107 => {
                        *field_num_checksum = field_num;
                    }
                    117 => {
                        *field_num_record_id = field_num;
                    }
                    _ => {}
                }
            }
            field_num += 1;
            field_num;
        }
    }
    return *field_num_original_url != -(1 as i32) && *field_num_checksum != -(1 as i32)
        && *field_num_record_id != -(1 as i32);
}
unsafe extern "C" fn warc_process_cdx_line(
    mut lineptr: *mut i8,
    mut field_num_original_url: i32,
    mut field_num_checksum: i32,
    mut field_num_record_id: i32,
) {
    let mut original_url: *mut i8 = 0 as *mut i8;
    let mut checksum: *mut i8 = 0 as *mut i8;
    let mut record_id: *mut i8 = 0 as *mut i8;
    let mut token: *mut i8 = 0 as *mut i8;
    let mut save_ptr: *mut i8 = 0 as *mut i8;
    let mut field_num: i32 = 0 as i32;
    token = strtok_r(lineptr, b" \t\r\n\0" as *const u8 as *const i8, &mut save_ptr);
    while !token.is_null() {
        let mut val: *mut *mut i8 = 0 as *mut *mut i8;
        if field_num == field_num_original_url {
            val = &mut original_url;
        } else if field_num == field_num_checksum {
            val = &mut checksum;
        } else if field_num == field_num_record_id {
            val = &mut record_id;
        } else {
            val = 0 as *mut *mut i8;
        }
        if !val.is_null() {
            *val = strdup(token);
        }
        token = strtok_r(
            0 as *mut i8,
            b" \t\r\n\0" as *const u8 as *const i8,
            &mut save_ptr,
        );
        field_num += 1;
        field_num;
    }
    if !original_url.is_null() && !checksum.is_null() && !record_id.is_null() {
        let mut checksum_l: idx_t = 0;
        let mut checksum_v: *mut i8 = 0 as *mut i8;
        base32_decode_alloc_ctx(
            0 as *mut base32_decode_context,
            checksum,
            strlen(checksum) as idx_t,
            &mut checksum_v,
            &mut checksum_l,
        );
        rpl_free(checksum as *mut libc::c_void);
        checksum = 0 as *mut i8;
        if !checksum_v.is_null() && checksum_l == 20 as i32 as i64 {
            let mut rec: *mut warc_cdx_record = 0 as *mut warc_cdx_record;
            rec = xmalloc(::core::mem::size_of::<warc_cdx_record>() as u64)
                as *mut warc_cdx_record;
            (*rec).url = original_url;
            (*rec).uuid = record_id;
            memcpy(
                ((*rec).digest).as_mut_ptr() as *mut libc::c_void,
                checksum_v as *const libc::c_void,
                20 as i32 as u64,
            );
            hash_table_put(
                warc_cdx_dedup_table,
                ((*rec).digest).as_mut_ptr() as *const libc::c_void,
                rec as *const libc::c_void,
            );
            rpl_free(checksum_v as *mut libc::c_void);
            checksum_v = 0 as *mut i8;
        } else {
            rpl_free(original_url as *mut libc::c_void);
            original_url = 0 as *mut i8;
            rpl_free(checksum_v as *mut libc::c_void);
            checksum_v = 0 as *mut i8;
            rpl_free(record_id as *mut libc::c_void);
            record_id = 0 as *mut i8;
        }
    } else {
        rpl_free(checksum as *mut libc::c_void);
        checksum = 0 as *mut i8;
        rpl_free(original_url as *mut libc::c_void);
        original_url = 0 as *mut i8;
        rpl_free(record_id as *mut libc::c_void);
        record_id = 0 as *mut i8;
    };
}
unsafe extern "C" fn warc_load_cdx_dedup_file() -> bool {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut lineptr: *mut i8 = 0 as *mut i8;
    let mut n: size_t = 0 as i32 as size_t;
    let mut line_length: ssize_t = 0;
    let mut field_num_original_url: i32 = -(1 as i32);
    let mut field_num_checksum: i32 = -(1 as i32);
    let mut field_num_record_id: i32 = -(1 as i32);
    f = rpl_fopen(opt.warc_cdx_dedup_filename, b"r\0" as *const u8 as *const i8);
    if f.is_null() {
        return 0 as i32 != 0;
    }
    line_length = getline(&mut lineptr, &mut n, f);
    if line_length != -(1 as i32) as i64 {
        warc_parse_cdx_header(
            lineptr,
            &mut field_num_original_url,
            &mut field_num_checksum,
            &mut field_num_record_id,
        );
    }
    if field_num_original_url == -(1 as i32) || field_num_checksum == -(1 as i32)
        || field_num_record_id == -(1 as i32)
    {
        if field_num_original_url == -(1 as i32) {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"CDX file does not list original urls. (Missing column 'a'.)\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if field_num_checksum == -(1 as i32) {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"CDX file does not list checksums. (Missing column 'k'.)\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if field_num_record_id == -(1 as i32) {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"CDX file does not list record ids. (Missing column 'u'.)\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    } else {
        let mut nrecords: i32 = 0;
        warc_cdx_dedup_table = hash_table_new(
            1000 as i32,
            Some(
                warc_hash_sha1_digest as unsafe extern "C" fn(*const libc::c_void) -> u64,
            ),
            Some(
                warc_cmp_sha1_digest
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
        loop {
            line_length = getline(&mut lineptr, &mut n, f);
            if line_length != -(1 as i32) as i64 {
                warc_process_cdx_line(
                    lineptr,
                    field_num_original_url,
                    field_num_checksum,
                    field_num_record_id,
                );
            }
            if !(line_length != -(1 as i32) as i64) {
                break;
            }
        }
        nrecords = hash_table_count(warc_cdx_dedup_table);
        logprintf(
            log_options::LOG_VERBOSE,
            dcngettext(
                0 as *const i8,
                b"Loaded %d record from CDX.\n\n\0" as *const u8 as *const i8,
                b"Loaded %d records from CDX.\n\n\0" as *const u8 as *const i8,
                nrecords as u64,
                5 as i32,
            ),
            nrecords,
        );
    }
    rpl_free(lineptr as *mut libc::c_void);
    lineptr = 0 as *mut i8;
    fclose(f);
    return 1 as i32 != 0;
}
unsafe extern "C" fn warc_find_duplicate_cdx_record(
    mut url: *const i8,
    mut sha1_digest_payload: *mut i8,
) -> *mut warc_cdx_record {
    let mut rec_existing: *mut warc_cdx_record = 0 as *mut warc_cdx_record;
    if warc_cdx_dedup_table.is_null() {
        return 0 as *mut warc_cdx_record;
    }
    rec_existing = hash_table_get(
        warc_cdx_dedup_table,
        sha1_digest_payload as *const libc::c_void,
    ) as *mut warc_cdx_record;
    if !rec_existing.is_null() && strcmp((*rec_existing).url, url) == 0 as i32 {
        return rec_existing
    } else {
        return 0 as *mut warc_cdx_record
    };
}
#[no_mangle]
pub unsafe extern "C" fn warc_init() {
    warc_write_ok = 1 as i32 != 0;
    if !(opt.warc_filename).is_null() {
        if !(opt.warc_cdx_dedup_filename).is_null() {
            if !warc_load_cdx_dedup_file() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Could not read CDX file %s for deduplication.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    quote(opt.warc_cdx_dedup_filename),
                );
                exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
            }
        }
        warc_manifest_fp = warc_tempfile();
        if warc_manifest_fp.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Could not open temporary WARC manifest file.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
        }
        if opt.warc_keep_log {
            warc_log_fp = warc_tempfile();
            if warc_log_fp.is_null() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Could not open temporary WARC log file.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
            }
            log_set_warc_log_fp(warc_log_fp);
        }
        warc_current_file_number = -(1 as i32);
        if !warc_start_new_file(0 as i32 != 0) {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    0 as *const i8,
                    b"Could not open WARC file.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
        }
        if opt.warc_cdx_enabled {
            if !warc_start_cdx_file() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    dcgettext(
                        0 as *const i8,
                        b"Could not open CDX file for output.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
            }
        }
    }
}
unsafe extern "C" fn warc_write_metadata() {
    let mut manifest_uuid: [i8; 48] = [0; 48];
    let mut warc_tmp_fp: *mut FILE = 0 as *mut FILE;
    if opt.warc_maxsize > 0 as i32 as i64 {
        warc_start_new_file(1 as i32 != 0);
    }
    warc_uuid_str(manifest_uuid.as_mut_ptr(), ::core::mem::size_of::<[i8; 48]>() as u64);
    rpl_fflush(warc_manifest_fp);
    warc_write_metadata_record(
        manifest_uuid.as_mut_ptr(),
        b"metadata://gnu.org/software/wget/warc/MANIFEST.txt\0" as *const u8
            as *const i8,
        0 as *const i8,
        0 as *const i8,
        0 as *mut ip_address,
        b"text/plain\0" as *const u8 as *const i8,
        warc_manifest_fp,
        -(1 as i32) as off_t,
    );
    warc_tmp_fp = warc_tempfile();
    if warc_tmp_fp.is_null() {
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                0 as *const i8,
                b"Could not open temporary WARC file.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        exit(C2RustUnnamed_6::WGET_EXIT_GENERIC_ERROR as i32);
    }
    rpl_fflush(warc_tmp_fp);
    fprintf(warc_tmp_fp, b"%s\n\0" as *const u8 as *const i8, program_argstring);
    warc_write_resource_record(
        0 as *const i8,
        b"metadata://gnu.org/software/wget/warc/wget_arguments.txt\0" as *const u8
            as *const i8,
        0 as *const i8,
        manifest_uuid.as_mut_ptr(),
        0 as *const ip_address,
        b"text/plain\0" as *const u8 as *const i8,
        warc_tmp_fp,
        -(1 as i32) as off_t,
    );
    if !warc_log_fp.is_null() {
        warc_write_resource_record(
            0 as *const i8,
            b"metadata://gnu.org/software/wget/warc/wget.log\0" as *const u8
                as *const i8,
            0 as *const i8,
            manifest_uuid.as_mut_ptr(),
            0 as *const ip_address,
            b"text/plain\0" as *const u8 as *const i8,
            warc_log_fp,
            -(1 as i32) as off_t,
        );
        warc_log_fp = 0 as *mut FILE;
        log_set_warc_log_fp(0 as *mut FILE);
    }
}
#[no_mangle]
pub unsafe extern "C" fn warc_close() {
    if !warc_current_file.is_null() {
        warc_write_metadata();
        *warc_current_warcinfo_uuid_str.as_mut_ptr() = 0 as i32 as i8;
        fclose(warc_current_file);
        warc_current_file = 0 as *mut FILE;
    }
    if !warc_current_cdx_file.is_null() {
        fclose(warc_current_cdx_file);
        warc_current_cdx_file = 0 as *mut FILE;
    }
    if !warc_log_fp.is_null() {
        fclose(warc_log_fp);
        log_set_warc_log_fp(0 as *mut FILE);
    }
}
#[no_mangle]
pub unsafe extern "C" fn warc_tempfile() -> *mut FILE {
    let mut filename: [i8; 100] = [0; 100];
    let mut fd: i32 = 0;
    if path_search(
        filename.as_mut_ptr(),
        100 as i32 as size_t,
        opt.warc_tempdir,
        b"wget\0" as *const u8 as *const i8,
        1 as i32 != 0,
    ) == -(1 as i32)
    {
        return 0 as *mut FILE;
    }
    fd = mkostemp(filename.as_mut_ptr(), 0 as i32);
    if fd < 0 as i32 {
        return 0 as *mut FILE;
    }
    if unlink(filename.as_mut_ptr()) < 0 as i32 {
        close(fd);
        return 0 as *mut FILE;
    }
    return fdopen(fd, b"wb+\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn warc_write_request_record(
    mut url: *const i8,
    mut timestamp_str: *const i8,
    mut record_uuid: *const i8,
    mut ip: *const ip_address,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const i8,
        b"request\0" as *const u8 as *const i8,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const i8, url);
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const i8,
        b"application/http;msgtype=request\0" as *const u8 as *const i8,
    );
    warc_write_date_header(timestamp_str);
    warc_write_header(b"WARC-Record-ID\0" as *const u8 as *const i8, record_uuid);
    warc_write_ip_header(ip);
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const i8,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_digest_headers(body, payload_offset);
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_cdx_record(
    mut url: *const i8,
    mut timestamp_str: *const i8,
    mut mime_type: *const i8,
    mut response_code: i32,
    mut payload_digest: *const i8,
    mut redirect_location: *const i8,
    mut offset: off_t,
    mut warc_filename: *const i8,
    mut response_uuid: *const i8,
) -> bool {
    let mut timestamp_str_cdx: [i8; 15] = [0; 15];
    let mut offset_string: [i8; 21] = [0; 21];
    let mut checksum: *const i8 = 0 as *const i8;
    let mut tmp_location: *mut i8 = 0 as *mut i8;
    memcpy(
        timestamp_str_cdx.as_mut_ptr() as *mut libc::c_void,
        timestamp_str as *const libc::c_void,
        4 as i32 as u64,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(4 as i32 as isize) as *mut libc::c_void,
        timestamp_str.offset(5 as i32 as isize) as *const libc::c_void,
        2 as i32 as u64,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(6 as i32 as isize) as *mut libc::c_void,
        timestamp_str.offset(8 as i32 as isize) as *const libc::c_void,
        2 as i32 as u64,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(8 as i32 as isize) as *mut libc::c_void,
        timestamp_str.offset(11 as i32 as isize) as *const libc::c_void,
        2 as i32 as u64,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(10 as i32 as isize) as *mut libc::c_void,
        timestamp_str.offset(14 as i32 as isize) as *const libc::c_void,
        2 as i32 as u64,
    );
    memcpy(
        timestamp_str_cdx.as_mut_ptr().offset(12 as i32 as isize) as *mut libc::c_void,
        timestamp_str.offset(17 as i32 as isize) as *const libc::c_void,
        2 as i32 as u64,
    );
    timestamp_str_cdx[14 as i32 as usize] = '\0' as i32 as i8;
    if !payload_digest.is_null() {
        checksum = payload_digest.offset(5 as i32 as isize);
    } else {
        checksum = b"-\0" as *const u8 as *const i8;
    }
    if mime_type.is_null() || strlen(mime_type) == 0 as i32 as u64 {
        mime_type = b"-\0" as *const u8 as *const i8;
    }
    if redirect_location.is_null() || strlen(redirect_location) == 0 as i32 as u64 {
        tmp_location = strdup(b"-\0" as *const u8 as *const i8);
    } else {
        tmp_location = url_escape(redirect_location);
    }
    number_to_string(offset_string.as_mut_ptr(), offset);
    fprintf(
        warc_current_cdx_file,
        b"%s %s %s %s %d %s %s - %s %s %s\n\0" as *const u8 as *const i8,
        url,
        timestamp_str_cdx.as_mut_ptr(),
        url,
        mime_type,
        response_code,
        checksum,
        tmp_location,
        offset_string.as_mut_ptr(),
        warc_current_filename,
        response_uuid,
    );
    rpl_fflush(warc_current_cdx_file);
    rpl_free(tmp_location as *mut libc::c_void);
    return 1 as i32 != 0;
}
unsafe extern "C" fn warc_write_revisit_record(
    mut url: *const i8,
    mut timestamp_str: *const i8,
    mut concurrent_to_uuid: *const i8,
    mut payload_digest: *const i8,
    mut refers_to: *const i8,
    mut ip: *const ip_address,
    mut body: *mut FILE,
) -> bool {
    let mut revisit_uuid: [i8; 48] = [0; 48];
    let mut block_digest: [i8; 38] = [0; 38];
    let mut sha1_res_block: [i8; 20] = [0; 20];
    warc_uuid_str(revisit_uuid.as_mut_ptr(), ::core::mem::size_of::<[i8; 48]>() as u64);
    sha1_stream(body, sha1_res_block.as_mut_ptr() as *mut libc::c_void);
    warc_base32_sha1_digest(
        sha1_res_block.as_mut_ptr(),
        block_digest.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 38]>() as u64,
    );
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const i8,
        b"revisit\0" as *const u8 as *const i8,
    );
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const i8,
        revisit_uuid.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const i8,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Concurrent-To\0" as *const u8 as *const i8,
        concurrent_to_uuid,
    );
    warc_write_header(b"WARC-Refers-To\0" as *const u8 as *const i8, refers_to);
    warc_write_header(
        b"WARC-Profile\0" as *const u8 as *const i8,
        b"http://netpreserve.org/warc/1.0/revisit/identical-payload-digest\0"
            as *const u8 as *const i8,
    );
    warc_write_header(
        b"WARC-Truncated\0" as *const u8 as *const i8,
        b"length\0" as *const u8 as *const i8,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const i8, url);
    warc_write_date_header(timestamp_str);
    warc_write_ip_header(ip);
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const i8,
        b"application/http;msgtype=response\0" as *const u8 as *const i8,
    );
    warc_write_header(
        b"WARC-Block-Digest\0" as *const u8 as *const i8,
        block_digest.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Payload-Digest\0" as *const u8 as *const i8,
        payload_digest,
    );
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    return warc_write_ok;
}
#[no_mangle]
pub unsafe extern "C" fn warc_write_response_record(
    mut url: *const i8,
    mut timestamp_str: *const i8,
    mut concurrent_to_uuid: *const i8,
    mut ip: *const ip_address,
    mut body: *mut FILE,
    mut payload_offset: off_t,
    mut mime_type: *const i8,
    mut response_code: i32,
    mut redirect_location: *const i8,
) -> bool {
    let mut block_digest: [i8; 38] = [0; 38];
    let mut payload_digest: [i8; 38] = [0; 38];
    let mut sha1_res_block: [i8; 20] = [0; 20];
    let mut sha1_res_payload: [i8; 20] = [0; 20];
    let mut response_uuid: [i8; 48] = [0; 48];
    let mut offset: off_t = 0;
    if opt.warc_digests_enabled {
        rewind(body);
        if warc_sha1_stream_with_payload(
            body,
            sha1_res_block.as_mut_ptr() as *mut libc::c_void,
            sha1_res_payload.as_mut_ptr() as *mut libc::c_void,
            payload_offset,
        ) == 0 as i32
        {
            let mut rec_existing: *mut warc_cdx_record = 0 as *mut warc_cdx_record;
            rec_existing = warc_find_duplicate_cdx_record(
                url,
                sha1_res_payload.as_mut_ptr(),
            );
            if !rec_existing.is_null() {
                let mut result: bool = false;
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        0 as *const i8,
                        b"Found exact match in CDX file. Saving revisit record to WARC.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if payload_offset > 0 as i32 as i64 {
                    if ftruncate(fileno(body), payload_offset) == -(1 as i32) {
                        return 0 as i32 != 0;
                    }
                }
                warc_base32_sha1_digest(
                    sha1_res_payload.as_mut_ptr(),
                    payload_digest.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 38]>() as u64,
                );
                result = warc_write_revisit_record(
                    url,
                    timestamp_str,
                    concurrent_to_uuid,
                    payload_digest.as_mut_ptr(),
                    (*rec_existing).uuid,
                    ip,
                    body,
                );
                return result;
            }
            warc_base32_sha1_digest(
                sha1_res_block.as_mut_ptr(),
                block_digest.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 38]>() as u64,
            );
            warc_base32_sha1_digest(
                sha1_res_payload.as_mut_ptr(),
                payload_digest.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 38]>() as u64,
            );
        }
    }
    warc_uuid_str(response_uuid.as_mut_ptr(), ::core::mem::size_of::<[i8; 48]>() as u64);
    rpl_fseeko(warc_current_file, 0 as i64, 2 as i32);
    offset = ftello(warc_current_file);
    warc_write_start_record();
    warc_write_header(
        b"WARC-Type\0" as *const u8 as *const i8,
        b"response\0" as *const u8 as *const i8,
    );
    warc_write_header(
        b"WARC-Record-ID\0" as *const u8 as *const i8,
        response_uuid.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const i8,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Concurrent-To\0" as *const u8 as *const i8,
        concurrent_to_uuid,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const i8, url);
    warc_write_date_header(timestamp_str);
    warc_write_ip_header(ip);
    warc_write_header(
        b"WARC-Block-Digest\0" as *const u8 as *const i8,
        block_digest.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Payload-Digest\0" as *const u8 as *const i8,
        payload_digest.as_mut_ptr(),
    );
    warc_write_header(
        b"Content-Type\0" as *const u8 as *const i8,
        b"application/http;msgtype=response\0" as *const u8 as *const i8,
    );
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    if warc_write_ok as i32 != 0 && opt.warc_cdx_enabled as i32 != 0 {
        warc_write_cdx_record(
            url,
            timestamp_str,
            mime_type,
            response_code,
            payload_digest.as_mut_ptr(),
            redirect_location,
            offset,
            warc_current_filename,
            response_uuid.as_mut_ptr(),
        );
    }
    return warc_write_ok;
}
unsafe extern "C" fn warc_write_record(
    mut record_type: *const i8,
    mut resource_uuid: *const i8,
    mut url: *const i8,
    mut timestamp_str: *const i8,
    mut concurrent_to_uuid: *const i8,
    mut ip: *const ip_address,
    mut content_type: *const i8,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    let mut uuid_buf: [i8; 48] = [0; 48];
    if resource_uuid.is_null() {
        warc_uuid_str(uuid_buf.as_mut_ptr(), ::core::mem::size_of::<[i8; 48]>() as u64);
        resource_uuid = uuid_buf.as_mut_ptr();
    }
    if content_type.is_null() {
        content_type = b"application/octet-stream\0" as *const u8 as *const i8;
    }
    warc_write_start_record();
    warc_write_header(b"WARC-Type\0" as *const u8 as *const i8, record_type);
    warc_write_header(b"WARC-Record-ID\0" as *const u8 as *const i8, resource_uuid);
    warc_write_header(
        b"WARC-Warcinfo-ID\0" as *const u8 as *const i8,
        warc_current_warcinfo_uuid_str.as_mut_ptr(),
    );
    warc_write_header(
        b"WARC-Concurrent-To\0" as *const u8 as *const i8,
        concurrent_to_uuid,
    );
    warc_write_header_uri(b"WARC-Target-URI\0" as *const u8 as *const i8, url);
    warc_write_date_header(timestamp_str);
    warc_write_ip_header(ip);
    warc_write_digest_headers(body, payload_offset);
    warc_write_header(b"Content-Type\0" as *const u8 as *const i8, content_type);
    warc_write_block_from_file(body);
    warc_write_end_record();
    fclose(body);
    return warc_write_ok;
}
#[no_mangle]
pub unsafe extern "C" fn warc_write_resource_record(
    mut resource_uuid: *const i8,
    mut url: *const i8,
    mut timestamp_str: *const i8,
    mut concurrent_to_uuid: *const i8,
    mut ip: *const ip_address,
    mut content_type: *const i8,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    return warc_write_record(
        b"resource\0" as *const u8 as *const i8,
        resource_uuid,
        url,
        timestamp_str,
        concurrent_to_uuid,
        ip,
        content_type,
        body,
        payload_offset,
    );
}
#[no_mangle]
pub unsafe extern "C" fn warc_write_metadata_record(
    mut record_uuid: *const i8,
    mut url: *const i8,
    mut timestamp_str: *const i8,
    mut concurrent_to_uuid: *const i8,
    mut ip: *mut ip_address,
    mut content_type: *const i8,
    mut body: *mut FILE,
    mut payload_offset: off_t,
) -> bool {
    return warc_write_record(
        b"metadata\0" as *const u8 as *const i8,
        record_uuid,
        url,
        timestamp_str,
        concurrent_to_uuid,
        ip,
        content_type,
        body,
        payload_offset,
    );
}