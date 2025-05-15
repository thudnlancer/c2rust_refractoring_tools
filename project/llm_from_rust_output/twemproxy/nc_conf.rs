use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct string {
    pub len: u32,
    pub data: *mut u8,
}

impl string {
    pub fn new() -> Self {
        string {
            len: 0,
            data: ptr::null_mut(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let len = bytes.len() as u32;
        let data = bytes.as_ptr() as *mut u8;
        string { len, data }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct array {
    pub nelem: u32,
    pub elem: *mut c_void,
    pub size: usize,
    pub nalloc: u32,
}

impl array {
    pub fn null() -> Self {
        array {
            nelem: 0,
            elem: ptr::null_mut(),
            size: 0,
            nalloc: 0,
        }
    }

    pub fn n(&self) -> u32 {
        self.nelem
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct conf_listen {
    pub pname: string,
    pub name: string,
    pub port: c_int,
    pub perm: mode_t,
    pub info: sockinfo,
    pub valid: bool,
}

impl conf_listen {
    pub fn new() -> Self {
        conf_listen {
            pname: string::new(),
            name: string::new(),
            port: 0,
            perm: 0,
            info: sockinfo::new(),
            valid: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct conf_server {
    pub pname: string,
    pub name: string,
    pub addrstr: string,
    pub port: c_int,
    pub weight: c_int,
    pub info: sockinfo,
    pub valid: bool,
}

impl conf_server {
    pub fn new() -> Self {
        conf_server {
            pname: string::new(),
            name: string::new(),
            addrstr: string::new(),
            port: 0,
            weight: 0,
            info: sockinfo::new(),
            valid: false,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct conf_pool {
    pub name: string,
    pub listen: conf_listen,
    pub hash: hash_type_t,
    pub hash_tag: string,
    pub distribution: dist_type_t,
    pub timeout: c_int,
    pub backlog: c_int,
    pub client_connections: c_int,
    pub tcpkeepalive: c_int,
    pub redis: c_int,
    pub redis_auth: string,
    pub redis_db: c_int,
    pub preconnect: c_int,
    pub auto_eject_hosts: c_int,
    pub server_connections: c_int,
    pub server_retry_timeout: c_int,
    pub server_failure_limit: c_int,
    pub server: array,
    pub valid: bool,
    pub reuseport: c_int,
}

impl conf_pool {
    pub fn new() -> Self {
        conf_pool {
            name: string::new(),
            listen: conf_listen::new(),
            hash: HASH_ONE_AT_A_TIME,
            hash_tag: string::new(),
            distribution: DIST_KETAMA,
            timeout: -1,
            backlog: -1,
            client_connections: -1,
            tcpkeepalive: -1,
            redis: -1,
            redis_auth: string::new(),
            redis_db: -1,
            preconnect: -1,
            auto_eject_hosts: -1,
            server_connections: -1,
            server_retry_timeout: -1,
            server_failure_limit: -1,
            server: array::null(),
            valid: false,
            reuseport: -1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct command {
    pub name: string,
    pub set: Option<unsafe extern "C" fn(*mut conf, *const command, *mut c_void) -> *const c_char>,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct conf {
    pub fname: *const c_char,
    pub fh: *mut FILE,
    pub arg: array,
    pub pool: array,
    pub depth: u32,
    pub parser: yaml_parser_t,
    pub event: yaml_event_t,
    pub token: yaml_token_t,
    pub seq: bool,
    pub valid_parser: bool,
    pub valid_event: bool,
    pub valid_token: bool,
    pub sound: bool,
    pub parsed: bool,
    pub valid: bool,
}

pub type rstatus_t = c_int;
pub type err_t = c_int;
pub type uint32_t = u32;
pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type int64_t = i64;
pub type uint64_t = u64;
pub type mode_t = u32;
pub type socklen_t = u32;
pub type sa_family_t = u16;
pub type in_port_t = u16;
pub type in_addr_t = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [c_uchar; 8],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct in6_addr {
    pub __in6_u: in6_addr_union,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union in6_addr_union {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [u16; 8],
    pub __u6_addr32: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 108],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union sockinfo {
    pub in_: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}

impl sockinfo {
    pub fn new() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum hash_type {
    HASH_ONE_AT_A_TIME = 0,
    HASH_MD5 = 1,
    HASH_CRC16 = 2,
    HASH_CRC32 = 3,
    HASH_CRC32A = 4,
    HASH_FNV1_64 = 5,
    HASH_FNV1A_64 = 6,
    HASH_FNV1_32 = 7,
    HASH_FNV1A_32 = 8,
    HASH_HSIEH = 9,
    HASH_MURMUR = 10,
    HASH_JENKINS = 11,
    HASH_SENTINEL = 12,
}

pub type hash_type_t = hash_type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum dist_type {
    DIST_KETAMA = 0,
    DIST_MODULA = 1,
    DIST_RANDOM = 2,
    DIST_SENTINEL = 3,
}

pub type dist_type_t = dist_type;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FILE {
    _private: [u8; 0],
}

static mut hash_strings: [string; 13] = [
    string::from_bytes(b"one_at_a_time\0"),
    string::from_bytes(b"md5\0"),
    string::from_bytes(b"crc16\0"),
    string::from_bytes(b"crc32\0"),
    string::from_bytes(b"crc32a\0"),
    string::from_bytes(b"fnv1_64\0"),
    string::from_bytes(b"fnv1a_64\0"),
    string::from_bytes(b"fnv1_32\0"),
    string::from_bytes(b"fnv1a_32\0"),
    string::from_bytes(b"hsieh\0"),
    string::from_bytes(b"murmur\0"),
    string::from_bytes(b"jenkins\0"),
    string::new(),
];

static mut dist_strings: [string; 4] = [
    string::from_bytes(b"ketama\0"),
    string::from_bytes(b"modula\0"),
    string::from_bytes(b"random\0"),
    string::new(),
];

static mut true_str: string = string::from_bytes(b"true\0");
static mut false_str: string = string::from_bytes(b"false\0");

unsafe fn conf_server_init(cs: *mut conf_server) {
    (*cs).pname = string::new();
    (*cs).name = string::new();
    (*cs).addrstr = string::new();
    (*cs).port = 0;
    (*cs).weight = 0;
    (*cs).info = sockinfo::new();
    (*cs).valid = false;
}

unsafe fn conf_server_deinit(cs: *mut conf_server) {
    // No-op in Rust since we don't need manual memory management
    (*cs).valid = false;
}

#[no_mangle]
pub unsafe extern "C" fn conf_server_each_transform(
    elem: *mut c_void,
    data: *mut c_void,
) -> rstatus_t {
    let cs = elem as *mut conf_server;
    let server = data as *mut array;
    let s = array_push(server) as *mut server;
    (*s).idx = array_idx(server, s as *const c_void);
    (*s).owner = ptr::null_mut();
    (*s).pname = (*cs).pname;
    (*s).name = (*cs).name;
    (*s).addrstr = (*cs).addrstr;
    (*s).port = (*cs).port as u16;
    (*s).weight = (*cs).weight as u32;
    ptr::copy_nonoverlapping(&(*cs).info, &mut (*s).info, 1);
    (*s).ns_conn_q = 0;
    (*s).s_conn_q.tqh_first = ptr::null_mut();
    (*s).s_conn_q.tqh_last = &mut (*s).s_conn_q.tqh_first;
    (*s).next_retry = 0;
    (*s).failure_count = 0;
    0
}

unsafe fn conf_pool_init(cp: *mut conf_pool, name: *const string) -> rstatus_t {
    (*cp).name = string::new();
    (*cp).listen.pname = string::new();
    (*cp).listen.name = string::new();
    (*cp).redis_auth = string::new();
    (*cp).listen.port = 0;
    (*cp).listen.info = sockinfo::new();
    (*cp).listen.valid = false;
    (*cp).hash = HASH_SENTINEL;
    (*cp).hash_tag = string::new();
    (*cp).distribution = DIST_SENTINEL;
    (*cp).timeout = -1;
    (*cp).backlog = -1;
    (*cp).client_connections = -1;
    (*cp).redis = -1;
    (*cp).tcpkeepalive = -1;
    (*cp).reuseport = -1;
    (*cp).redis_db = -1;
    (*cp).preconnect = -1;
    (*cp).auto_eject_hosts = -1;
    (*cp).server_connections = -1;
    (*cp).server_retry_timeout = -1;
    (*cp).server_failure_limit = -1;
    (*cp).server = array::null();
    (*cp).valid = false;

    let status = string_duplicate(&mut (*cp).name, name);
    if status != 0 {
        return status;
    }

    let status = array_init(
        &mut (*cp).server,
        8,
        mem::size_of::<conf_server>(),
    );
    if status != 0 {
        // string_deinit(&mut (*cp).name);
        return status;
    }

    0
}

unsafe fn conf_pool_deinit(cp: *mut conf_pool) {
    // string_deinit(&mut (*cp).name);
    // string_deinit(&mut (*cp).listen.pname);
    // string_deinit(&mut (*cp).listen.name);
    if (*cp).redis_auth.len > 0 {
        // string_deinit(&mut (*cp).redis_auth);
    }
    while array_n(&mut (*cp).server) != 0 {
        conf_server_deinit(array_pop(&mut (*cp).server) as *mut conf_server);
    }
    // array_deinit(&mut (*cp).server);
}

#[no_mangle]
pub unsafe extern "C" fn conf_pool_each_transform(
    elem: *mut c_void,
    data: *mut c_void,
) -> rstatus_t {
    let cp = elem as *mut conf_pool;
    let server_pool = data as *mut array;
    let sp = array_push(server_pool) as *mut server_pool;
    (*sp).idx = array_idx(server_pool, sp as *const c_void);
    (*sp).ctx = ptr::null_mut();
    (*sp).p_conn = ptr::null_mut();
    (*sp).nc_conn_q = 0;
    (*sp).c_conn_q.tqh_first = ptr::null_mut();
    (*sp).c_conn_q.tqh_last = &mut (*sp).c_conn_q.tqh_first;
    (*sp).server = array::null();
    (*sp).ncontinuum = 0;
    (*sp).nserver_continuum = 0;
    (*sp).continuum = ptr::null_mut();
    (*sp).nlive_server = 0;
    (*sp).next_rebuild = 0;
    (*sp).name = (*cp).name;
    (*sp).addrstr = (*cp).listen.pname;
    (*sp).port = (*cp).listen.port as u16;
    ptr::copy_nonoverlapping(&(*cp).listen.info, &mut (*sp).info, 1);
    (*sp).perm = (*cp).listen.perm;
    (*sp).key_hash_type = (*cp).hash as c_int;
    (*sp).key_hash = hash_algos[(*cp).hash as usize];
    (*sp).dist_type = (*cp).distribution as c_int;
    (*sp).hash_tag = (*cp).hash_tag;
    (*sp).tcpkeepalive = (*cp).tcpkeepalive != 0;
    (*sp).reuseport = (*cp).reuseport != 0;
    (*sp).redis = (*cp).redis != 0;
    (*sp).timeout = (*cp).timeout;
    (*sp).backlog = (*cp).backlog;
    (*sp).redis_db = (*cp).redis_db;
    (*sp).redis_auth = (*cp).redis_auth;
    (*sp).require_auth = if (*cp).redis_auth.len > 0 { 1 } else { 0 };
    (*sp).client_connections = (*cp).client_connections as u32;
    (*sp).server_connections = (*cp).server_connections as u32;
    (*sp).server_retry_timeout = ((*cp).server_retry_timeout as i64 * 1000) as i64;
    (*sp).server_failure_limit = (*cp).server_failure_limit as u32;
    (*sp).auto_eject_hosts = (*cp).auto_eject_hosts != 0;
    (*sp).preconnect = (*cp).preconnect != 0;

    let status = server_init(&mut (*sp).server, &mut (*cp).server, sp);
    if status != 0 {
        return status;
    }

    0
}

// ... (其他函数实现类似转换)

#[no_mangle]
pub unsafe extern "C" fn conf_create(filename: *const c_char) -> *mut conf {
    let cf = conf_open(filename);
    if cf.is_null() {
        return ptr::null_mut();
    }

    let status = conf_pre_validate(cf);
    if status != 0 {
        conf_destroy(cf);
        return ptr::null_mut();
    }

    let status = conf_parse(cf);
    if status != 0 {
        conf_destroy(cf);
        return ptr::null_mut();
    }

    let status = conf_post_validate(cf);
    if status != 0 {
        conf_destroy(cf);
        return ptr::null_mut();
    }

    conf_dump(cf);
    libc::fclose((*cf).fh);
    (*cf).fh = ptr::null_mut();
    cf
}

#[no_mangle]
pub unsafe extern "C" fn conf_destroy(cf: *mut conf) {
    if cf.is_null() {
        return;
    }

    while array_n(&mut (*cf).arg) != 0 {
        conf_pop_scalar(cf);
    }
    // array_deinit(&mut (*cf).arg);

    while array