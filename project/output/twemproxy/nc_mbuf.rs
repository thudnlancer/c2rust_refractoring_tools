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
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn _nc_alloc(size: size_t, name: *const i8, line: i32) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const i8, line: i32);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __pid_t = i32;
pub type pid_t = __pid_t;
pub type int64_t = __int64_t;
pub type pthread_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: i32,
    pub timeout: i32,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: i32,
    pub event: *mut epoll_event,
    pub nevent: i32,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option<unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: i32,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: i32,
    pub service_str: string,
    pub service: string,
    pub source_str: string,
    pub source: string,
    pub version_str: string,
    pub version: string,
    pub uptime_str: string,
    pub timestamp_str: string,
    pub ntotal_conn_str: string,
    pub ncurr_conn_str: string,
    pub aggregate: i32,
    pub updated: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mhdr {
    pub stqh_first: *mut mbuf,
    pub stqh_last: *mut *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf {
    pub magic: uint32_t,
    pub next: C2RustUnnamed,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub stqe_next: *mut mbuf,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct instance {
    pub ctx: *mut context,
    pub log_level: i32,
    pub log_filename: *const i8,
    pub conf_filename: *const i8,
    pub stats_port: uint16_t,
    pub stats_interval: i32,
    pub stats_addr: *const i8,
    pub hostname: [i8; 256],
    pub mbuf_chunk_size: size_t,
    pub pid: pid_t,
    pub pid_filename: *const i8,
    #[bitfield(name = "pidfile", ty = "libc::c_uint", bits = "0..=0")]
    pub pidfile: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type mbuf_copy_t = Option<unsafe extern "C" fn(*mut mbuf, *mut libc::c_void) -> ()>;
static mut nfree_mbufq: uint32_t = 0;
static mut free_mbufq: mhdr = mhdr {
    stqh_first: 0 as *const mbuf as *mut mbuf,
    stqh_last: 0 as *const *mut mbuf as *mut *mut mbuf,
};
static mut mbuf_chunk_size: size_t = 0;
static mut mbuf_offset: size_t = 0;
unsafe extern "C" fn _mbuf_get() -> *mut mbuf {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    if !(free_mbufq.stqh_first).is_null() {
        mbuf = free_mbufq.stqh_first;
        nfree_mbufq = nfree_mbufq.wrapping_sub(1);
        nfree_mbufq;
        let mut oldnext: *mut *mut libc::c_void = &mut (*free_mbufq.stqh_first)
            .next
            .stqe_next as *mut *mut mbuf as *mut libc::c_void as *mut *mut libc::c_void;
        free_mbufq.stqh_first = (*free_mbufq.stqh_first).next.stqe_next;
        if (free_mbufq.stqh_first).is_null() {
            free_mbufq.stqh_last = &mut free_mbufq.stqh_first;
        }
        *oldnext = 0 as *mut libc::c_void;
    } else {
        buf = _nc_alloc(
            mbuf_chunk_size,
            b"nc_mbuf.c\0" as *const u8 as *const i8,
            46 as i32,
        ) as *mut uint8_t;
        if buf.is_null() {
            return 0 as *mut mbuf;
        }
        mbuf = buf.offset(mbuf_offset as isize) as *mut mbuf;
        (*mbuf).magic = 0xdeadbeef as u32;
    }
    (*mbuf).next.stqe_next = 0 as *mut mbuf;
    return mbuf;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_get() -> *mut mbuf {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    mbuf = _mbuf_get();
    if mbuf.is_null() {
        return 0 as *mut mbuf;
    }
    buf = (mbuf as *mut uint8_t).offset(-(mbuf_offset as isize));
    (*mbuf).start = buf;
    (*mbuf).end = buf.offset(mbuf_offset as isize);
    (*mbuf).pos = (*mbuf).start;
    (*mbuf).last = (*mbuf).start;
    return mbuf;
}
unsafe extern "C" fn mbuf_free(mut mbuf: *mut mbuf) {
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    buf = (mbuf as *mut uint8_t).offset(-(mbuf_offset as isize));
    _nc_free(
        buf as *mut libc::c_void,
        b"nc_mbuf.c\0" as *const u8 as *const i8,
        115 as i32,
    );
    buf = 0 as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_put(mut mbuf: *mut mbuf) {
    nfree_mbufq = nfree_mbufq.wrapping_add(1);
    nfree_mbufq;
    (*mbuf).next.stqe_next = free_mbufq.stqh_first;
    if ((*mbuf).next.stqe_next).is_null() {
        free_mbufq.stqh_last = &mut (*mbuf).next.stqe_next;
    }
    free_mbufq.stqh_first = mbuf;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_rewind(mut mbuf: *mut mbuf) {
    (*mbuf).pos = (*mbuf).start;
    (*mbuf).last = (*mbuf).start;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_length(mut mbuf: *const mbuf) -> uint32_t {
    return ((*mbuf).last).offset_from((*mbuf).pos) as i64 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_size(mut mbuf: *const mbuf) -> uint32_t {
    return ((*mbuf).end).offset_from((*mbuf).last) as i64 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_data_size() -> size_t {
    return mbuf_offset;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_insert(mut mhdr: *mut mhdr, mut mbuf: *mut mbuf) {
    (*mbuf).next.stqe_next = 0 as *mut mbuf;
    *(*mhdr).stqh_last = mbuf;
    (*mhdr).stqh_last = &mut (*mbuf).next.stqe_next;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_remove(mut mhdr: *mut mhdr, mut mbuf: *mut mbuf) {
    if (*mhdr).stqh_first == mbuf {
        let mut oldnext: *mut *mut libc::c_void = &mut (*(*mhdr).stqh_first)
            .next
            .stqe_next as *mut *mut mbuf as *mut libc::c_void as *mut *mut libc::c_void;
        (*mhdr).stqh_first = (*(*mhdr).stqh_first).next.stqe_next;
        if ((*mhdr).stqh_first).is_null() {
            (*mhdr).stqh_last = &mut (*mhdr).stqh_first;
        }
        *oldnext = 0 as *mut libc::c_void;
    } else {
        let mut curelm: *mut mbuf = (*mhdr).stqh_first;
        while (*curelm).next.stqe_next != mbuf {
            curelm = (*curelm).next.stqe_next;
        }
        let mut oldnext_0: *mut *mut libc::c_void = &mut (*(*curelm).next.stqe_next)
            .next
            .stqe_next as *mut *mut mbuf as *mut libc::c_void as *mut *mut libc::c_void;
        (*curelm).next.stqe_next = (*(*curelm).next.stqe_next).next.stqe_next;
        if ((*curelm).next.stqe_next).is_null() {
            (*mhdr).stqh_last = &mut (*curelm).next.stqe_next;
        }
        *oldnext_0 = 0 as *mut libc::c_void;
    }
    (*mbuf).next.stqe_next = 0 as *mut mbuf;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_copy(
    mut mbuf: *mut mbuf,
    mut pos: *const uint8_t,
    mut n: size_t,
) {
    if n == 0 as i32 as u64 {
        return;
    }
    memcpy((*mbuf).last as *mut libc::c_void, pos as *const libc::c_void, n);
    (*mbuf).last = ((*mbuf).last).offset(n as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_split(
    mut h: *mut mhdr,
    mut pos: *mut uint8_t,
    mut cb: mbuf_copy_t,
    mut cbarg: *mut libc::c_void,
) -> *mut mbuf {
    let mut mbuf: *mut mbuf = 0 as *mut mbuf;
    let mut nbuf: *mut mbuf = 0 as *mut mbuf;
    let mut size: size_t = 0;
    mbuf = if ((*h).stqh_first).is_null() {
        0 as *mut mbuf
    } else {
        ((*h).stqh_last as *mut i8)
            .offset(
                -(&mut (*(0 as *mut libc::c_void as *mut mbuf)).next
                    as *mut C2RustUnnamed as size_t as isize),
            ) as *mut libc::c_void as *mut mbuf
    };
    nbuf = mbuf_get();
    if nbuf.is_null() {
        return 0 as *mut mbuf;
    }
    if cb.is_some() {
        cb.expect("non-null function pointer")(nbuf, cbarg);
    }
    size = ((*mbuf).last).offset_from(pos) as i64 as size_t;
    mbuf_copy(nbuf, pos, size);
    (*mbuf).last = pos;
    return nbuf;
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_init(mut nci: *const instance) {
    nfree_mbufq = 0 as i32 as uint32_t;
    free_mbufq.stqh_first = 0 as *mut mbuf;
    free_mbufq.stqh_last = &mut free_mbufq.stqh_first;
    mbuf_chunk_size = (*nci).mbuf_chunk_size;
    mbuf_offset = mbuf_chunk_size.wrapping_sub(::core::mem::size_of::<mbuf>() as u64);
}
#[no_mangle]
pub unsafe extern "C" fn mbuf_deinit() {
    while !(free_mbufq.stqh_first).is_null() {
        let mut mbuf: *mut mbuf = free_mbufq.stqh_first;
        mbuf_remove(&mut free_mbufq, mbuf);
        mbuf_free(mbuf);
        nfree_mbufq = nfree_mbufq.wrapping_sub(1);
        nfree_mbufq;
    }
}