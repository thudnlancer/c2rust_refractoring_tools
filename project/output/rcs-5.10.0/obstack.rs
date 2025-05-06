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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut exit_failure: i32;
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 0],
}
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
pub type __off64_t = i64;
pub type _IO_lock_t = ();
pub type __off_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type uintmax_t = __uintmax_t;
pub type __uintmax_t = u64;
unsafe extern "C" fn call_chunkfun(
    mut h: *mut obstack,
    mut size: size_t,
) -> *mut libc::c_void {
    if (*h).use_extra_arg() != 0 {
        return ((*h).chunkfun.extra)
            .expect("non-null function pointer")((*h).extra_arg, size)
    } else {
        return ((*h).chunkfun.plain).expect("non-null function pointer")(size)
    };
}
unsafe extern "C" fn call_freefun(
    mut h: *mut obstack,
    mut old_chunk: *mut libc::c_void,
) {
    if (*h).use_extra_arg() != 0 {
        ((*h).freefun.extra)
            .expect("non-null function pointer")((*h).extra_arg, old_chunk);
    } else {
        ((*h).freefun.plain).expect("non-null function pointer")(old_chunk);
    };
}
unsafe extern "C" fn _obstack_begin_worker(
    mut h: *mut obstack,
    mut size: size_t,
    mut alignment: size_t,
) -> i32 {
    let mut chunk: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    if alignment == 0 as i32 as u64 {
        alignment = if ::core::mem::align_of::<f128::f128>() as u64
            > (if ::core::mem::align_of::<uintmax_t>() as u64
                > ::core::mem::align_of::<*mut libc::c_void>() as u64
            {
                ::core::mem::align_of::<uintmax_t>() as u64
            } else {
                ::core::mem::align_of::<*mut libc::c_void>() as u64
            })
        {
            ::core::mem::align_of::<f128::f128>() as u64
        } else if ::core::mem::align_of::<uintmax_t>() as u64
            > ::core::mem::align_of::<*mut libc::c_void>() as u64
        {
            ::core::mem::align_of::<uintmax_t>() as u64
        } else {
            ::core::mem::align_of::<*mut libc::c_void>() as u64
        };
    }
    if size == 0 as i32 as u64 {
        let mut extra: i32 = (((12 as i32 as u64)
            .wrapping_add(
                (if ::core::mem::size_of::<f128::f128>() as u64
                    > (if ::core::mem::size_of::<uintmax_t>() as u64
                        > ::core::mem::size_of::<*mut libc::c_void>() as u64
                    {
                        ::core::mem::size_of::<uintmax_t>() as u64
                    } else {
                        ::core::mem::size_of::<*mut libc::c_void>() as u64
                    })
                {
                    ::core::mem::size_of::<f128::f128>() as u64
                } else {
                    (if ::core::mem::size_of::<uintmax_t>() as u64
                        > ::core::mem::size_of::<*mut libc::c_void>() as u64
                    {
                        ::core::mem::size_of::<uintmax_t>() as u64
                    } else {
                        ::core::mem::size_of::<*mut libc::c_void>() as u64
                    })
                }),
            )
            .wrapping_sub(1 as i32 as u64)
            & !(if ::core::mem::size_of::<f128::f128>() as u64
                > (if ::core::mem::size_of::<uintmax_t>() as u64
                    > ::core::mem::size_of::<*mut libc::c_void>() as u64
                {
                    ::core::mem::size_of::<uintmax_t>() as u64
                } else {
                    ::core::mem::size_of::<*mut libc::c_void>() as u64
                })
            {
                ::core::mem::size_of::<f128::f128>() as u64
            } else {
                (if ::core::mem::size_of::<uintmax_t>() as u64
                    > ::core::mem::size_of::<*mut libc::c_void>() as u64
                {
                    ::core::mem::size_of::<uintmax_t>() as u64
                } else {
                    ::core::mem::size_of::<*mut libc::c_void>() as u64
                })
            })
                .wrapping_sub(1 as i32 as u64))
            .wrapping_add(4 as i32 as u64)
            .wrapping_add(
                (if ::core::mem::size_of::<f128::f128>() as u64
                    > (if ::core::mem::size_of::<uintmax_t>() as u64
                        > ::core::mem::size_of::<*mut libc::c_void>() as u64
                    {
                        ::core::mem::size_of::<uintmax_t>() as u64
                    } else {
                        ::core::mem::size_of::<*mut libc::c_void>() as u64
                    })
                {
                    ::core::mem::size_of::<f128::f128>() as u64
                } else {
                    (if ::core::mem::size_of::<uintmax_t>() as u64
                        > ::core::mem::size_of::<*mut libc::c_void>() as u64
                    {
                        ::core::mem::size_of::<uintmax_t>() as u64
                    } else {
                        ::core::mem::size_of::<*mut libc::c_void>() as u64
                    })
                }),
            )
            .wrapping_sub(1 as i32 as u64)
            & !(if ::core::mem::size_of::<f128::f128>() as u64
                > (if ::core::mem::size_of::<uintmax_t>() as u64
                    > ::core::mem::size_of::<*mut libc::c_void>() as u64
                {
                    ::core::mem::size_of::<uintmax_t>() as u64
                } else {
                    ::core::mem::size_of::<*mut libc::c_void>() as u64
                })
            {
                ::core::mem::size_of::<f128::f128>() as u64
            } else {
                (if ::core::mem::size_of::<uintmax_t>() as u64
                    > ::core::mem::size_of::<*mut libc::c_void>() as u64
                {
                    ::core::mem::size_of::<uintmax_t>() as u64
                } else {
                    ::core::mem::size_of::<*mut libc::c_void>() as u64
                })
            })
                .wrapping_sub(1 as i32 as u64)) as i32;
        size = (4096 as i32 - extra) as size_t;
    }
    (*h).chunk_size = size;
    (*h).alignment_mask = alignment.wrapping_sub(1 as i32 as u64);
    (*h).chunk = call_chunkfun(h, (*h).chunk_size) as *mut _obstack_chunk;
    chunk = (*h).chunk;
    if chunk.is_null() {
        (Some(obstack_alloc_failed_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")();
    }
    (*h).object_base = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
        < ::core::mem::size_of::<*mut libc::c_void>() as u64
    {
        chunk as *mut i8
    } else {
        0 as *mut i8
    })
        .offset(
            ((((*chunk).contents)
                .as_mut_ptr()
                .offset_from(
                    (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                        < ::core::mem::size_of::<*mut libc::c_void>() as u64
                    {
                        chunk as *mut i8
                    } else {
                        0 as *mut i8
                    }),
                ) as i64 as u64)
                .wrapping_add(alignment.wrapping_sub(1 as i32 as u64))
                & !alignment.wrapping_sub(1 as i32 as u64)) as isize,
        );
    (*h).next_free = (*h).object_base;
    (*chunk).limit = (chunk as *mut i8).offset((*h).chunk_size as isize);
    (*h).chunk_limit = (*chunk).limit;
    (*chunk).prev = 0 as *mut _obstack_chunk;
    (*h).set_maybe_empty_object(0 as i32 as u32);
    (*h).set_alloc_failed(0 as i32 as u32);
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _obstack_begin(
    mut h: *mut obstack,
    mut size: size_t,
    mut alignment: size_t,
    mut chunkfun: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut freefun: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> i32 {
    (*h).chunkfun.plain = chunkfun;
    (*h).freefun.plain = freefun;
    (*h).set_use_extra_arg(0 as i32 as u32);
    return _obstack_begin_worker(h, size, alignment);
}
#[no_mangle]
pub unsafe extern "C" fn _obstack_begin_1(
    mut h: *mut obstack,
    mut size: size_t,
    mut alignment: size_t,
    mut chunkfun: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    mut freefun: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
    mut arg: *mut libc::c_void,
) -> i32 {
    (*h).chunkfun.extra = chunkfun;
    (*h).freefun.extra = freefun;
    (*h).extra_arg = arg;
    (*h).set_use_extra_arg(1 as i32 as u32);
    return _obstack_begin_worker(h, size, alignment);
}
#[no_mangle]
pub unsafe extern "C" fn _obstack_newchunk(mut h: *mut obstack, mut length: size_t) {
    let mut old_chunk: *mut _obstack_chunk = (*h).chunk;
    let mut new_chunk: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut obj_size: size_t = ((*h).next_free).offset_from((*h).object_base) as i64
        as size_t;
    let mut object_base: *mut i8 = 0 as *mut i8;
    let mut sum1: size_t = obj_size.wrapping_add(length);
    let mut sum2: size_t = sum1.wrapping_add((*h).alignment_mask);
    let mut new_size: size_t = sum2
        .wrapping_add(obj_size >> 3 as i32)
        .wrapping_add(100 as i32 as u64);
    if new_size < sum2 {
        new_size = sum2;
    }
    if new_size < (*h).chunk_size {
        new_size = (*h).chunk_size;
    }
    if obj_size <= sum1 && sum1 <= sum2 {
        new_chunk = call_chunkfun(h, new_size) as *mut _obstack_chunk;
    }
    if new_chunk.is_null() {
        (Some(obstack_alloc_failed_handler.expect("non-null function pointer")))
            .expect("non-null function pointer")();
    }
    (*h).chunk = new_chunk;
    (*new_chunk).prev = old_chunk;
    (*h).chunk_limit = (new_chunk as *mut i8).offset(new_size as isize);
    (*new_chunk).limit = (*h).chunk_limit;
    object_base = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
        < ::core::mem::size_of::<*mut libc::c_void>() as u64
    {
        new_chunk as *mut i8
    } else {
        0 as *mut i8
    })
        .offset(
            ((((*new_chunk).contents)
                .as_mut_ptr()
                .offset_from(
                    (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                        < ::core::mem::size_of::<*mut libc::c_void>() as u64
                    {
                        new_chunk as *mut i8
                    } else {
                        0 as *mut i8
                    }),
                ) as i64 as u64)
                .wrapping_add((*h).alignment_mask) & !(*h).alignment_mask) as isize,
        );
    memcpy(
        object_base as *mut libc::c_void,
        (*h).object_base as *const libc::c_void,
        obj_size,
    );
    if (*h).maybe_empty_object() == 0
        && (*h).object_base
            == (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                < ::core::mem::size_of::<*mut libc::c_void>() as u64
            {
                old_chunk as *mut i8
            } else {
                0 as *mut i8
            })
                .offset(
                    ((((*old_chunk).contents)
                        .as_mut_ptr()
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                old_chunk as *mut i8
                            } else {
                                0 as *mut i8
                            }),
                        ) as i64 as u64)
                        .wrapping_add((*h).alignment_mask) & !(*h).alignment_mask)
                        as isize,
                )
    {
        (*new_chunk).prev = (*old_chunk).prev;
        call_freefun(h, old_chunk as *mut libc::c_void);
    }
    (*h).object_base = object_base;
    (*h).next_free = ((*h).object_base).offset(obj_size as isize);
    (*h).set_maybe_empty_object(0 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn _obstack_allocated_p(
    mut h: *mut obstack,
    mut obj: *mut libc::c_void,
) -> i32 {
    let mut lp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut plp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    lp = (*h).chunk;
    while !lp.is_null()
        && (lp as *mut libc::c_void >= obj || ((*lp).limit as *mut libc::c_void) < obj)
    {
        plp = (*lp).prev;
        lp = plp;
    }
    return (lp != 0 as *mut _obstack_chunk) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _obstack_free(mut h: *mut obstack, mut obj: *mut libc::c_void) {
    let mut lp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut plp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    lp = (*h).chunk;
    while !lp.is_null()
        && (lp as *mut libc::c_void >= obj || ((*lp).limit as *mut libc::c_void) < obj)
    {
        plp = (*lp).prev;
        call_freefun(h, lp as *mut libc::c_void);
        lp = plp;
        (*h).set_maybe_empty_object(1 as i32 as u32);
    }
    if !lp.is_null() {
        (*h).next_free = obj as *mut i8;
        (*h).object_base = (*h).next_free;
        (*h).chunk_limit = (*lp).limit;
        (*h).chunk = lp;
    } else if !obj.is_null() {
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn _obstack_memory_used(mut h: *mut obstack) -> size_t {
    let mut lp: *mut _obstack_chunk = 0 as *mut _obstack_chunk;
    let mut nbytes: size_t = 0 as i32 as size_t;
    lp = (*h).chunk;
    while !lp.is_null() {
        nbytes = (nbytes as u64)
            .wrapping_add(((*lp).limit).offset_from(lp as *mut i8) as i64 as u64)
            as size_t as size_t;
        lp = (*lp).prev;
    }
    return nbytes;
}
unsafe extern "C" fn print_and_abort() -> ! {
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const i8,
        b"memory exhausted\0" as *const u8 as *const i8,
    );
    exit(exit_failure);
}
#[no_mangle]
pub static mut obstack_alloc_failed_handler: Option<unsafe extern "C" fn() -> !> = unsafe {
    Some(print_and_abort as unsafe extern "C" fn() -> !)
};