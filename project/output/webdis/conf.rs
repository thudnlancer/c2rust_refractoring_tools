#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, core_intrinsics)]
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(__s: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn inet_addr(__cp: *const i8) -> in_addr_t;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn getgrnam(__name: *const i8) -> *mut group;
    fn __errno_location() -> *mut i32;
    fn json_delete(json: *mut json_t);
    fn json_object_get(object: *const json_t, key: *const i8) -> *mut json_t;
    fn json_object_iter(object: *mut json_t) -> *mut libc::c_void;
    fn json_object_iter_next(
        object: *mut json_t,
        iter: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn json_object_iter_key(iter: *mut libc::c_void) -> *const i8;
    fn json_object_iter_value(iter: *mut libc::c_void) -> *mut json_t;
    fn json_array_size(array: *const json_t) -> size_t;
    fn json_array_get(array: *const json_t, index: size_t) -> *mut json_t;
    fn json_string_value(string: *const json_t) -> *const i8;
    fn json_integer_value(integer: *const json_t) -> json_int_t;
    fn json_load_file(
        path: *const i8,
        flags: size_t,
        error: *mut json_error_t,
    ) -> *mut json_t;
    fn base64_init_encodestate(state_in: *mut base64_encodestate);
    fn base64_encode_block(
        plaintext_in: *const i8,
        length_in: i32,
        code_out: *mut i8,
        state_in: *mut base64_encodestate,
    ) -> i32;
    fn base64_encode_blockend(
        code_out: *mut i8,
        state_in: *mut base64_encodestate,
    ) -> i32;
}
pub type size_t = u64;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type uint32_t = __uint32_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut i8,
    pub gr_passwd: *mut i8,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum json_type {
    JSON_OBJECT,
    JSON_ARRAY,
    JSON_STRING,
    JSON_INTEGER,
    JSON_REAL,
    JSON_TRUE,
    JSON_FALSE,
    JSON_NULL,
}
impl json_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            json_type::JSON_OBJECT => 0,
            json_type::JSON_ARRAY => 1,
            json_type::JSON_STRING => 2,
            json_type::JSON_INTEGER => 3,
            json_type::JSON_REAL => 4,
            json_type::JSON_TRUE => 5,
            json_type::JSON_FALSE => 6,
            json_type::JSON_NULL => 7,
        }
    }
    fn from_libc_c_uint(value: u32) -> json_type {
        match value {
            0 => json_type::JSON_OBJECT,
            1 => json_type::JSON_ARRAY,
            2 => json_type::JSON_STRING,
            3 => json_type::JSON_INTEGER,
            4 => json_type::JSON_REAL,
            5 => json_type::JSON_TRUE,
            6 => json_type::JSON_FALSE,
            7 => json_type::JSON_NULL,
            _ => panic!("Invalid value for json_type: {}", value),
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_t {
    pub type_0: json_type,
    pub refcount: size_t,
}
pub type json_int_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_error_t {
    pub line: i32,
    pub column: i32,
    pub position: i32,
    pub source: [i8; 80],
    pub text: [i8; 160],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum base64_encodestep {
    step_A,
    step_B,
    step_C,
}
impl base64_encodestep {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            base64_encodestep::step_A => 0,
            base64_encodestep::step_B => 1,
            base64_encodestep::step_C => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> base64_encodestep {
        match value {
            0 => base64_encodestep::step_A,
            1 => base64_encodestep::step_B,
            2 => base64_encodestep::step_C,
            _ => panic!("Invalid value for base64_encodestep: {}", value),
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encodestate {
    pub step: base64_encodestep,
    pub result: i8,
    pub stepcount: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_level {
    WEBDIS_ERROR = 0,
    WEBDIS_WARNING,
    WEBDIS_NOTICE,
    WEBDIS_INFO,
    WEBDIS_DEBUG,
    WEBDIS_TRACE = 8,
}
impl log_level {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_level::WEBDIS_ERROR => 0,
            log_level::WEBDIS_WARNING => 1,
            log_level::WEBDIS_NOTICE => 2,
            log_level::WEBDIS_INFO => 3,
            log_level::WEBDIS_DEBUG => 4,
            log_level::WEBDIS_TRACE => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_level {
        match value {
            0 => log_level::WEBDIS_ERROR,
            1 => log_level::WEBDIS_WARNING,
            2 => log_level::WEBDIS_NOTICE,
            3 => log_level::WEBDIS_INFO,
            4 => log_level::WEBDIS_DEBUG,
            8 => log_level::WEBDIS_TRACE,
            _ => panic!("Invalid value for log_level: {}", value),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_fsync_mode {
    LOG_FSYNC_AUTO = 0,
    LOG_FSYNC_MILLIS,
    LOG_FSYNC_ALL,
}
impl log_fsync_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_fsync_mode::LOG_FSYNC_AUTO => 0,
            log_fsync_mode::LOG_FSYNC_MILLIS => 1,
            log_fsync_mode::LOG_FSYNC_ALL => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_fsync_mode {
        match value {
            0 => log_fsync_mode::LOG_FSYNC_AUTO,
            1 => log_fsync_mode::LOG_FSYNC_MILLIS,
            2 => log_fsync_mode::LOG_FSYNC_ALL,
            _ => panic!("Invalid value for log_fsync_mode: {}", value),
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: i32,
    pub username: *mut i8,
    pub password: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf {
    pub redis_host: *mut i8,
    pub redis_port: i32,
    pub redis_auth: *mut auth,
    pub http_host: *mut i8,
    pub http_port: i32,
    pub http_threads: i32,
    pub http_max_request_size: size_t,
    pub pool_size_per_thread: i32,
    pub daemonize: i32,
    pub pidfile: *mut i8,
    pub websockets: i32,
    pub database: i32,
    pub perms: *mut acl,
    pub user: uid_t,
    pub group: gid_t,
    pub logfile: *mut i8,
    pub verbosity: log_level,
    pub log_fsync: C2RustUnnamed_0,
    pub hiredis_opts: C2RustUnnamed,
    pub default_root: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_alive_sec: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub mode: log_fsync_mode,
    pub period_millis: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl {
    pub cidr: C2RustUnnamed_1,
    pub http_basic_auth: *mut i8,
    pub enabled: acl_commands,
    pub disabled: acl_commands,
    pub next: *mut acl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl_commands {
    pub count: u32,
    pub commands: *mut *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub enabled: i32,
    pub subnet: in_addr_t,
    pub mask: in_addr_t,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn json_decref(mut json: *mut json_t) {
    if !json.is_null() && (*json).refcount != -(1 as i32) as size_t
        && {
            let fresh0 = &mut (*json).refcount as *mut size_t;
            let fresh1 = 1 as i32 as size_t;
            ::core::intrinsics::atomic_xsub_release(fresh0, fresh1) - fresh1
                == 0 as i32 as u64
        }
    {
        json_delete(json);
    }
}
#[no_mangle]
pub unsafe extern "C" fn conf_str_allcaps(mut s: *const i8, sz: size_t) -> i32 {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < sz {
        if *s.offset(i as isize) as i32
            != ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *s.offset(i as isize) as i32;
                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(*s.offset(i as isize) as i32);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*s.offset(i as isize) as i32 as isize);
                }
                __res
            })
        {
            return 0 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn conf_string_or_envvar(mut val: *const i8) -> *mut i8 {
    if val.is_null() {
        return strdup(b"\0" as *const u8 as *const i8);
    }
    let mut val_len: size_t = strlen(val);
    if val_len >= 2 as i32 as u64 && *val.offset(0 as i32 as isize) as i32 == '$' as i32
        && conf_str_allcaps(
            val.offset(1 as i32 as isize),
            val_len.wrapping_sub(1 as i32 as u64),
        ) != 0
    {
        let mut env_val: *mut i8 = getenv(val.offset(1 as i32 as isize));
        if !env_val.is_null() {
            return strdup(env_val)
        } else {
            fprintf(
                stderr,
                b"No value found for env var %s\n\0" as *const u8 as *const i8,
                val.offset(1 as i32 as isize),
            );
        }
    }
    return strdup(val);
}
#[no_mangle]
pub unsafe extern "C" fn atoi_free(mut s: *mut i8) -> i32 {
    let mut val: i32 = atoi(s);
    free(s as *mut libc::c_void);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn is_true_free(mut s: *mut i8) -> i32 {
    let mut val: i32 = if strcasecmp(s, b"true\0" as *const u8 as *const i8) == 0 as i32
    {
        1 as i32
    } else {
        0 as i32
    };
    free(s as *mut libc::c_void);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn conf_read(mut filename: *const i8) -> *mut conf {
    let mut j: *mut json_t = 0 as *mut json_t;
    let mut error: json_error_t = json_error_t {
        line: 0,
        column: 0,
        position: 0,
        source: [0; 80],
        text: [0; 160],
    };
    let mut conf: *mut conf = 0 as *mut conf;
    let mut kv: *mut libc::c_void = 0 as *mut libc::c_void;
    conf = calloc(1 as i32 as u64, ::core::mem::size_of::<conf>() as u64) as *mut conf;
    (*conf).redis_host = strdup(b"127.0.0.1\0" as *const u8 as *const i8);
    (*conf).redis_port = 6379 as i32;
    (*conf).http_host = strdup(b"0.0.0.0\0" as *const u8 as *const i8);
    (*conf).http_port = 7379 as i32;
    (*conf).http_max_request_size = (128 as i32 * 1024 as i32 * 1024 as i32) as size_t;
    (*conf).http_threads = 4 as i32;
    (*conf).user = getuid();
    (*conf).group = getgid();
    (*conf).logfile = b"webdis.log\0" as *const u8 as *const i8 as *mut i8;
    (*conf).log_fsync.mode = log_fsync_mode::LOG_FSYNC_AUTO;
    (*conf).verbosity = log_level::WEBDIS_NOTICE;
    (*conf).daemonize = 0 as i32;
    (*conf).pidfile = b"webdis.pid\0" as *const u8 as *const i8 as *mut i8;
    (*conf).database = 0 as i32;
    (*conf).pool_size_per_thread = 2 as i32;
    j = json_load_file(filename, 0 as i32 as size_t, &mut error);
    if j.is_null() {
        fprintf(
            stderr,
            b"Error: %s (line %d)\n\0" as *const u8 as *const i8,
            (error.text).as_mut_ptr(),
            error.line,
        );
        return conf;
    }
    kv = json_object_iter(j);
    while !kv.is_null() {
        let mut jtmp: *mut json_t = json_object_iter_value(kv);
        if strcmp(json_object_iter_key(kv), b"redis_host\0" as *const u8 as *const i8)
            == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            free((*conf).redis_host as *mut libc::c_void);
            (*conf).redis_host = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"redis_port\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            (*conf).redis_port = json_integer_value(jtmp) as i32;
        } else if strcmp(
            json_object_iter_key(kv),
            b"redis_port\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).redis_port = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"redis_auth\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            if (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32 {
                (*conf).redis_auth = conf_auth_legacy(
                    conf_string_or_envvar(json_string_value(jtmp)),
                );
            } else if (*jtmp).type_0 as u32 == json_type::JSON_ARRAY as i32 as u32 {
                (*conf).redis_auth = conf_auth_username_password(jtmp);
            } else if (*jtmp).type_0 as u32 != json_type::JSON_NULL as i32 as u32 {
                fprintf(
                    stderr,
                    b"Config error with 'redis_auth': expected a string or an array of two strings. Starting with auth disabled.\n\0"
                        as *const u8 as *const i8,
                );
            }
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_host\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            free((*conf).http_host as *mut libc::c_void);
            (*conf).http_host = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_port\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            (*conf).http_port = json_integer_value(jtmp) as i32;
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_port\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).http_port = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_max_request_size\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            (*conf).http_max_request_size = json_integer_value(jtmp) as size_t;
        } else if strcmp(
            json_object_iter_key(kv),
            b"http_max_request_size\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).http_max_request_size = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            ) as size_t;
        } else if strcmp(
            json_object_iter_key(kv),
            b"threads\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            (*conf).http_threads = json_integer_value(jtmp) as i32;
        } else if strcmp(
            json_object_iter_key(kv),
            b"threads\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).http_threads = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(json_object_iter_key(kv), b"acl\0" as *const u8 as *const i8)
            == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_ARRAY as i32 as u32
        {
            (*conf).perms = conf_parse_acls(jtmp);
        } else if strcmp(json_object_iter_key(kv), b"user\0" as *const u8 as *const i8)
            == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            let mut u: *mut passwd = 0 as *mut passwd;
            let mut username: *mut i8 = conf_string_or_envvar(json_string_value(jtmp));
            *__errno_location() = 0 as i32;
            u = getpwnam(username);
            if !u.is_null() {
                (*conf).user = (*u).pw_uid;
            } else if *__errno_location() != 0 {
                fprintf(
                    stderr,
                    b"Could not find user ID for user '%s', an error occurred: %s\n\0"
                        as *const u8 as *const i8,
                    username,
                    strerror(*__errno_location()),
                );
            } else {
                fprintf(
                    stderr,
                    b"Could not find user ID for unknown user '%s'\n\0" as *const u8
                        as *const i8,
                    username,
                );
            }
            free(username as *mut libc::c_void);
        } else if strcmp(json_object_iter_key(kv), b"group\0" as *const u8 as *const i8)
            == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            let mut g: *mut group = 0 as *mut group;
            let mut groupname: *mut i8 = conf_string_or_envvar(json_string_value(jtmp));
            *__errno_location() = 0 as i32;
            g = getgrnam(groupname);
            if !g.is_null() {
                (*conf).group = (*g).gr_gid;
            } else if *__errno_location() != 0 {
                fprintf(
                    stderr,
                    b"Could not find group ID for group '%s', an error occurred: %s\n\0"
                        as *const u8 as *const i8,
                    groupname,
                    strerror(*__errno_location()),
                );
            } else {
                fprintf(
                    stderr,
                    b"Could not find group ID for unknown group '%s'\n\0" as *const u8
                        as *const i8,
                    groupname,
                );
            }
            free(groupname as *mut libc::c_void);
        } else if strcmp(
            json_object_iter_key(kv),
            b"logfile\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).logfile = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"log_fsync\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            if (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
                && strcmp(json_string_value(jtmp), b"auto\0" as *const u8 as *const i8)
                    == 0 as i32
            {
                (*conf).log_fsync.mode = log_fsync_mode::LOG_FSYNC_AUTO;
            } else if (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
                && strcmp(json_string_value(jtmp), b"all\0" as *const u8 as *const i8)
                    == 0 as i32
            {
                (*conf).log_fsync.mode = log_fsync_mode::LOG_FSYNC_ALL;
            } else if (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
                && json_integer_value(jtmp) > 0 as i32 as libc::c_longlong
            {
                (*conf).log_fsync.mode = log_fsync_mode::LOG_FSYNC_MILLIS;
                (*conf).log_fsync.period_millis = json_integer_value(jtmp) as i32;
            } else if (*jtmp).type_0 as u32 != json_type::JSON_NULL as i32 as u32 {
                fprintf(
                    stderr,
                    b"Unexpected value for \"log_fsync\", defaulting to \"auto\"\n\0"
                        as *const u8 as *const i8,
                );
            }
        } else if strcmp(
            json_object_iter_key(kv),
            b"verbosity\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            let mut tmp: i32 = json_integer_value(jtmp) as i32;
            if tmp < 0 as i32 || tmp > log_level::WEBDIS_TRACE as i32 {
                fprintf(
                    stderr,
                    b"Invalid log verbosity: %d. Acceptable range: [%d .. %d]\n\0"
                        as *const u8 as *const i8,
                    tmp,
                    log_level::WEBDIS_ERROR as i32,
                    log_level::WEBDIS_TRACE as i32,
                );
            }
            (*conf).verbosity = (if tmp < 0 as i32 {
                log_level::WEBDIS_ERROR as i32 as u32
            } else if tmp > log_level::WEBDIS_TRACE as i32 {
                log_level::WEBDIS_TRACE as i32 as u32
            } else {
                tmp as log_level as u32
            }) as log_level;
        } else if strcmp(
            json_object_iter_key(kv),
            b"daemonize\0" as *const u8 as *const i8,
        ) == 0 as i32
            && ((*jtmp).type_0 as u32 == json_type::JSON_TRUE as i32 as u32
                || (*jtmp).type_0 as u32 == json_type::JSON_FALSE as i32 as u32)
        {
            (*conf).daemonize = if (*jtmp).type_0 as u32
                == json_type::JSON_TRUE as i32 as u32
            {
                1 as i32
            } else {
                0 as i32
            };
        } else if strcmp(
            json_object_iter_key(kv),
            b"daemonize\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).daemonize = is_true_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"pidfile\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).pidfile = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"websockets\0" as *const u8 as *const i8,
        ) == 0 as i32
            && ((*jtmp).type_0 as u32 == json_type::JSON_TRUE as i32 as u32
                || (*jtmp).type_0 as u32 == json_type::JSON_FALSE as i32 as u32)
        {
            (*conf).websockets = if (*jtmp).type_0 as u32
                == json_type::JSON_TRUE as i32 as u32
            {
                1 as i32
            } else {
                0 as i32
            };
        } else if strcmp(
            json_object_iter_key(kv),
            b"websockets\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).websockets = is_true_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"database\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            (*conf).database = json_integer_value(jtmp) as i32;
        } else if strcmp(
            json_object_iter_key(kv),
            b"database\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).database = atoi_free(conf_string_or_envvar(json_string_value(jtmp)));
        } else if strcmp(
            json_object_iter_key(kv),
            b"pool_size\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            (*conf).pool_size_per_thread = json_integer_value(jtmp) as i32;
        } else if strcmp(
            json_object_iter_key(kv),
            b"pool_size\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).pool_size_per_thread = atoi_free(
                conf_string_or_envvar(json_string_value(jtmp)),
            );
        } else if strcmp(
            json_object_iter_key(kv),
            b"default_root\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_STRING as i32 as u32
        {
            (*conf).default_root = conf_string_or_envvar(json_string_value(jtmp));
        } else if strcmp(
            json_object_iter_key(kv),
            b"hiredis\0" as *const u8 as *const i8,
        ) == 0 as i32 && (*jtmp).type_0 as u32 == json_type::JSON_OBJECT as i32 as u32
        {
            conf_parse_hiredis(conf, jtmp);
        } else {
            fprintf(
                stderr,
                b"Warning! Unexpected key or incorrect value in %s: '%s'\n\0"
                    as *const u8 as *const i8,
                filename,
                json_object_iter_key(kv),
            );
        }
        kv = json_object_iter_next(j, kv);
    }
    json_decref(j);
    return conf;
}
#[no_mangle]
pub unsafe extern "C" fn acl_read_commands(
    mut jlist: *mut json_t,
    mut ac: *mut acl_commands,
) {
    let mut i: u32 = 0;
    let mut n: u32 = 0;
    let mut cur: u32 = 0;
    i = 0 as i32 as u32;
    n = 0 as i32 as u32;
    while (i as u64) < json_array_size(jlist) {
        let mut jelem: *mut json_t = json_array_get(jlist, i as size_t);
        if (*jelem).type_0 as u32 == json_type::JSON_STRING as i32 as u32 {
            n = n.wrapping_add(1);
            n;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*ac).commands = calloc(n as size_t, ::core::mem::size_of::<*mut i8>() as u64)
        as *mut *mut i8;
    (*ac).count = n;
    i = 0 as i32 as u32;
    cur = 0 as i32 as u32;
    while (i as u64) < json_array_size(jlist) {
        let mut jelem_0: *mut json_t = json_array_get(jlist, i as size_t);
        if (*jelem_0).type_0 as u32 == json_type::JSON_STRING as i32 as u32 {
            let ref mut fresh2 = *((*ac).commands).offset(cur as isize);
            *fresh2 = conf_string_or_envvar(json_string_value(jelem_0));
            cur = cur.wrapping_add(1);
            cur;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn conf_parse_acl(mut j: *mut json_t) -> *mut acl {
    let mut jcidr: *mut json_t = 0 as *mut json_t;
    let mut jbasic: *mut json_t = 0 as *mut json_t;
    let mut jlist: *mut json_t = 0 as *mut json_t;
    let mut mask_bits: libc::c_ushort = 0 as i32 as libc::c_ushort;
    let mut a: *mut acl = calloc(1 as i32 as u64, ::core::mem::size_of::<acl>() as u64)
        as *mut acl;
    jcidr = json_object_get(j, b"ip\0" as *const u8 as *const i8);
    if !jcidr.is_null() && (*jcidr).type_0 as u32 == json_type::JSON_STRING as i32 as u32
    {
        let mut s: *const i8 = 0 as *const i8;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut ip: *mut i8 = 0 as *mut i8;
        s = conf_string_or_envvar(json_string_value(jcidr));
        p = strchr(s, '/' as i32);
        if p.is_null() {
            ip = strdup(s);
        } else {
            ip = calloc(
                (p.offset_from(s) as i64 + 1 as i32 as i64) as size_t,
                1 as i32 as u64,
            ) as *mut i8;
            memcpy(
                ip as *mut libc::c_void,
                s as *const libc::c_void,
                p.offset_from(s) as i64 as size_t,
            );
            mask_bits = atoi(p.offset(1 as i32 as isize)) as libc::c_ushort;
        }
        (*a).cidr.enabled = 1 as i32;
        (*a).cidr.mask = if mask_bits as i32 == 0 as i32 {
            0xffffffff as u32
        } else {
            (0xffffffff as u32) << 32 as i32 - mask_bits as i32
        };
        (*a).cidr.subnet = ({
            let mut __v: u32 = 0;
            let mut __x: u32 = inet_addr(ip);
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh3 = &mut __v;
                let fresh4;
                let fresh5 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
            }
            __v
        }) & (*a).cidr.mask;
        free(ip as *mut libc::c_void);
    }
    jbasic = json_object_get(j, b"http_basic_auth\0" as *const u8 as *const i8);
    if !jbasic.is_null()
        && (*jbasic).type_0 as u32 == json_type::JSON_STRING as i32 as u32
    {
        let mut b64: base64_encodestate = base64_encodestate {
            step: base64_encodestep::step_A,
            result: 0,
            stepcount: 0,
        };
        let mut pos: i32 = 0;
        let mut p_0: *mut i8 = 0 as *mut i8;
        let mut plain: *mut i8 = conf_string_or_envvar(json_string_value(jbasic));
        let mut len: size_t = 0;
        let mut plain_len: size_t = (strlen(plain)).wrapping_add(0 as i32 as u64);
        len = plain_len
            .wrapping_add(8 as i32 as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(6 as i32 as u64);
        (*a).http_basic_auth = calloc(len, 1 as i32 as u64) as *mut i8;
        base64_init_encodestate(&mut b64);
        pos = base64_encode_block(
            plain,
            plain_len as i32,
            (*a).http_basic_auth,
            &mut b64,
        );
        free(plain as *mut libc::c_void);
        if pos == 0 {
            fprintf(
                stderr,
                b"Error: could not encode credentials as HTTP basic auth header\n\0"
                    as *const u8 as *const i8,
            );
            exit(1 as i32);
        }
        base64_encode_blockend(((*a).http_basic_auth).offset(pos as isize), &mut b64);
        p_0 = strchr(((*a).http_basic_auth).offset(pos as isize), '\n' as i32);
        if !p_0.is_null() {
            *p_0 = 0 as i32 as i8;
        }
    }
    jlist = json_object_get(j, b"enabled\0" as *const u8 as *const i8);
    if !jlist.is_null() && (*jlist).type_0 as u32 == json_type::JSON_ARRAY as i32 as u32
    {
        acl_read_commands(jlist, &mut (*a).enabled);
    }
    jlist = json_object_get(j, b"disabled\0" as *const u8 as *const i8);
    if !jlist.is_null() && (*jlist).type_0 as u32 == json_type::JSON_ARRAY as i32 as u32
    {
        acl_read_commands(jlist, &mut (*a).disabled);
    }
    return a;
}
unsafe extern "C" fn conf_parse_acls(mut jtab: *mut json_t) -> *mut acl {
    let mut head: *mut acl = 0 as *mut acl;
    let mut tail: *mut acl = 0 as *mut acl;
    let mut tmp: *mut acl = 0 as *mut acl;
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64) < json_array_size(jtab) {
        let mut val: *mut json_t = json_array_get(jtab, i as size_t);
        tmp = conf_parse_acl(val);
        if head.is_null() && tail.is_null() {
            tail = tmp;
            head = tail;
        } else {
            (*tail).next = tmp;
            tail = tmp;
        }
        i = i.wrapping_add(1);
        i;
    }
    return head;
}
#[no_mangle]
pub unsafe extern "C" fn conf_free(mut conf: *mut conf) {
    free((*conf).redis_host as *mut libc::c_void);
    if !((*conf).redis_auth).is_null() {
        free((*(*conf).redis_auth).username as *mut libc::c_void);
        free((*(*conf).redis_auth).password as *mut libc::c_void);
    }
    free((*conf).redis_auth as *mut libc::c_void);
    free((*conf).http_host as *mut libc::c_void);
    free(conf as *mut libc::c_void);
}
unsafe extern "C" fn conf_auth_legacy(mut password: *mut i8) -> *mut auth {
    let mut ret: *mut auth = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<auth>() as u64,
    ) as *mut auth;
    if ret.is_null() {
        free(password as *mut libc::c_void);
        fprintf(
            stderr,
            b"Config error with 'redis_auth': failed to allocate memory for credentials. Starting with auth disabled.\n\0"
                as *const u8 as *const i8,
        );
        return 0 as *mut auth;
    }
    (*ret).use_legacy_auth = 1 as i32;
    (*ret).password = password;
    return ret;
}
unsafe extern "C" fn conf_auth_username_password(mut jarray: *mut json_t) -> *mut auth {
    let mut array_size: size_t = json_array_size(jarray);
    if array_size != 2 as i32 as u64 {
        fprintf(
            stderr,
            b"Config error with 'redis_auth': expected two values, found %zu. Starting with auth disabled.\n\0"
                as *const u8 as *const i8,
            array_size,
        );
        return 0 as *mut auth;
    }
    let mut jusername: *mut json_t = json_array_get(jarray, 0 as i32 as size_t);
    let mut jpassword: *mut json_t = json_array_get(jarray, 1 as i32 as size_t);
    if (*jusername).type_0 as u32 != json_type::JSON_STRING as i32 as u32
        || (*jpassword).type_0 as u32 != json_type::JSON_STRING as i32 as u32
    {
        fprintf(
            stderr,
            b"Config error with 'redis_auth': both values need to be strings. Starting with auth disabled.\n\0"
                as *const u8 as *const i8,
        );
        return 0 as *mut auth;
    }
    let mut username: *mut i8 = conf_string_or_envvar(json_string_value(jusername));
    let mut password: *mut i8 = conf_string_or_envvar(json_string_value(jpassword));
    let mut ret: *mut auth = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<auth>() as u64,
    ) as *mut auth;
    if username.is_null() || password.is_null() || ret.is_null() {
        free(username as *mut libc::c_void);
        free(password as *mut libc::c_void);
        free(ret as *mut libc::c_void);
        fprintf(
            stderr,
            b"Config error with 'redis_auth': failed to allocate memory for credentials. Starting with auth disabled.\n\0"
                as *const u8 as *const i8,
        );
        return 0 as *mut auth;
    }
    (*ret).use_legacy_auth = 0 as i32;
    (*ret).username = username;
    (*ret).password = password;
    return ret;
}
unsafe extern "C" fn conf_parse_hiredis(mut conf: *mut conf, mut jhiredis: *mut json_t) {
    let mut kv: *mut libc::c_void = json_object_iter(jhiredis);
    while !kv.is_null() {
        let mut jtmp: *mut json_t = json_object_iter_value(kv);
        let mut key: *const i8 = json_object_iter_key(kv);
        if strcmp(key, b"keep_alive_sec\0" as *const u8 as *const i8) == 0 as i32
            && (*jtmp).type_0 as u32 == json_type::JSON_INTEGER as i32 as u32
        {
            (*conf).hiredis_opts.keep_alive_sec = json_integer_value(jtmp) as i32;
        } else {
            fprintf(
                stderr,
                b"Config error under 'hiredis': unknown key '%s'.\0" as *const u8
                    as *const i8,
                key,
            );
        }
        kv = json_object_iter_next(jhiredis, kv);
    }
}