#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wlink {
    pub entry: *mut libc::c_void,
    pub next: *mut wlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const i8,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub entry: *const libc::c_void,
    pub next: *mut link,
}
#[no_mangle]
pub unsafe extern "C" fn extend(
    mut tp: *mut link,
    mut x: *const libc::c_void,
    mut to: *mut divvy,
) -> *mut link {
    let mut pair: *mut link = alloc(to, ::core::mem::size_of::<link>() as u64)
        as *mut link;
    (*pair).entry = x;
    (*pair).next = 0 as *mut link;
    (*tp).next = pair;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn wextend(
    mut tp: *mut wlink,
    mut x: *mut libc::c_void,
    mut to: *mut divvy,
) -> *mut wlink {
    let mut pair: *mut wlink = alloc(to, ::core::mem::size_of::<wlink>() as u64)
        as *mut wlink;
    (*pair).entry = x;
    (*pair).next = 0 as *mut wlink;
    (*tp).next = pair;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn prepend(
    mut x: *const libc::c_void,
    mut ls: *mut link,
    mut to: *mut divvy,
) -> *mut link {
    let mut pair: *mut link = alloc(to, ::core::mem::size_of::<link>() as u64)
        as *mut link;
    (*pair).entry = x;
    (*pair).next = ls;
    return pair;
}
#[no_mangle]
pub unsafe extern "C" fn wprepend(
    mut x: *mut libc::c_void,
    mut ls: *mut wlink,
    mut to: *mut divvy,
) -> *mut wlink {
    let mut pair: *mut wlink = alloc(to, ::core::mem::size_of::<wlink>() as u64)
        as *mut wlink;
    (*pair).entry = x;
    (*pair).next = ls;
    return pair;
}