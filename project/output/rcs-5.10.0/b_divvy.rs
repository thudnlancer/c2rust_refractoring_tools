#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const i8,
        __args: ::core::ffi::VaList,
    ) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xalloc_die();
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    static mut obstack_alloc_failed_handler: Option<unsafe extern "C" fn() -> !>;
    fn strlen(_: *const i8) -> u64;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = i64;
pub type size_t = u64;
pub type va_list = __builtin_va_list;
pub type off_t = __off_t;
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
pub type ptrdiff_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const i8,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
}
#[no_mangle]
pub static mut plexus: *mut divvy = 0 as *const divvy as *mut divvy;
#[no_mangle]
pub static mut single: *mut divvy = 0 as *const divvy as *mut divvy;
#[no_mangle]
pub unsafe extern "C" fn make_space(mut name: *const i8) -> *mut divvy {
    let mut divvy: *mut divvy = xmalloc(::core::mem::size_of::<divvy>() as u64)
        as *mut divvy;
    (*divvy).name = name;
    obstack_alloc_failed_handler = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        Option<unsafe extern "C" fn() -> !>,
    >(Some(xalloc_die as unsafe extern "C" fn() -> ()));
    _obstack_begin(
        &mut (*divvy).space,
        0 as i32 as size_t,
        0 as i32 as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    let mut widest: size_t = if ::core::mem::size_of::<*mut libc::c_void>() as u64
        > ::core::mem::size_of::<off_t>() as u64
    {
        ::core::mem::size_of::<*mut libc::c_void>() as u64
    } else {
        ::core::mem::size_of::<off_t>() as u64
    };
    (*divvy).space.alignment_mask = widest.wrapping_sub(1 as i32 as u64);
    (*divvy).first = (*divvy).space.next_free as *mut libc::c_void;
    (*divvy).count = 0 as i32 as size_t;
    return divvy;
}
#[no_mangle]
pub unsafe extern "C" fn alloc(
    mut divvy: *mut divvy,
    mut len: size_t,
) -> *mut libc::c_void {
    (*divvy).count = ((*divvy).count).wrapping_add(1);
    (*divvy).count;
    return ({
        let mut __h: *mut obstack = &mut (*divvy).space;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = len;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        ({
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut i8 {
                (*__o1).set_maybe_empty_object(1 as i32 as u32);
            }
            (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                < ::core::mem::size_of::<*mut libc::c_void>() as u64
            {
                (*__o1).object_base
            } else {
                0 as *mut i8
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut i8
                            }),
                        ) as i64 as u64)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        })
    });
}
#[no_mangle]
pub unsafe extern "C" fn zlloc(
    mut divvy: *mut divvy,
    mut len: size_t,
) -> *mut libc::c_void {
    return memset(alloc(divvy, len), 0 as i32, len);
}
#[no_mangle]
pub unsafe extern "C" fn intern(
    mut divvy: *mut divvy,
    mut s: *const i8,
    mut len: size_t,
) -> *mut i8 {
    (*divvy).count = ((*divvy).count).wrapping_add(1);
    (*divvy).count;
    return ({
        let mut __h: *mut obstack = &mut (*divvy).space as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = len;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len.wrapping_add(1 as i32 as u64)
        {
            _obstack_newchunk(__o, __len.wrapping_add(1 as i32 as u64));
        }
        memcpy((*__o).next_free as *mut libc::c_void, s as *const libc::c_void, __len);
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        let fresh0 = (*__o).next_free;
        (*__o).next_free = ((*__o).next_free).offset(1);
        *fresh0 = 0 as i32 as i8;
        ({
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut i8 {
                (*__o1).set_maybe_empty_object(1 as i32 as u32);
            }
            (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                < ::core::mem::size_of::<*mut libc::c_void>() as u64
            {
                (*__o1).object_base
            } else {
                0 as *mut i8
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut i8
                            }),
                        ) as i64 as u64)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        })
    }) as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn brush_off(mut divvy: *mut divvy, mut ptr: *mut libc::c_void) {
    (*divvy).count = ((*divvy).count).wrapping_sub(1);
    (*divvy).count;
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __obj: *mut libc::c_void = ptr;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut i8;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    };
}
#[no_mangle]
pub unsafe extern "C" fn forget(mut divvy: *mut divvy) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __obj: *mut libc::c_void = (*divvy).first;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut i8;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    (*divvy).count = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn accf(mut divvy: *mut divvy, mut fmt: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    obstack_vprintf(&mut (*divvy).space, fmt, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn accumulate_nbytes(
    mut divvy: *mut divvy,
    mut start: *const i8,
    mut count: size_t,
) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __len: size_t = count;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, start as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
}
#[no_mangle]
pub unsafe extern "C" fn accumulate_byte(mut divvy: *mut divvy, mut c: i32) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
    }) < 1 as i32 as u64
    {
        _obstack_newchunk(__o, 1 as i32 as size_t);
    }
    let fresh1 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh1 = c as i8;
}
#[no_mangle]
pub unsafe extern "C" fn accumulate_range(
    mut divvy: *mut divvy,
    mut beg: *const i8,
    mut end: *const i8,
) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __len: size_t = end.offset_from(beg) as i64 as size_t;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, beg as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
}
#[no_mangle]
pub unsafe extern "C" fn accs(mut divvy: *mut divvy, mut string: *const i8) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __len: size_t = strlen(string);
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, string as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
}
#[no_mangle]
pub unsafe extern "C" fn finish_string(
    mut divvy: *mut divvy,
    mut result_len: *mut size_t,
) -> *mut i8 {
    let mut o: *mut obstack = &mut (*divvy).space;
    let mut rv: *mut i8 = 0 as *mut i8;
    *result_len = ({
        let mut __o: *const obstack = o;
        ((*__o).next_free).offset_from((*__o).object_base) as i64 as size_t
    });
    let mut __o: *mut obstack = o;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
    }) < 1 as i32 as u64
    {
        _obstack_newchunk(__o, 1 as i32 as size_t);
    }
    let fresh2 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh2 = '\0' as i32 as i8;
    rv = ({
        let mut __o1: *mut obstack = o;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut i8 {
            (*__o1).set_maybe_empty_object(1 as i32 as u32);
        }
        (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
            < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            (*__o1).object_base
        } else {
            0 as *mut i8
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut i8
                        }),
                    ) as i64 as u64)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut i8;
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn pointer_array(
    mut divvy: *mut divvy,
    mut count: size_t,
) -> *mut libc::c_void {
    let mut o: *mut obstack = &mut (*divvy).space;
    loop {
        let fresh3 = count;
        count = count.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        let mut __o: *mut obstack = o;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            _obstack_newchunk(__o, ::core::mem::size_of::<*mut libc::c_void>() as u64);
        }
        let mut __o1: *mut obstack = __o;
        let mut __p1: *mut libc::c_void = (*__o1).next_free as *mut libc::c_void;
        let ref mut fresh4 = *(__p1 as *mut *const libc::c_void);
        *fresh4 = 0 as *const libc::c_void;
        (*__o1).next_free = ((*__o1).next_free)
            .offset(::core::mem::size_of::<*const libc::c_void>() as u64 as isize);
    }
    return ({
        let mut __o1_0: *mut obstack = o;
        let mut __value: *mut libc::c_void = (*__o1_0).object_base as *mut libc::c_void;
        if (*__o1_0).next_free == __value as *mut i8 {
            (*__o1_0).set_maybe_empty_object(1 as i32 as u32);
        }
        (*__o1_0).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
            < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            (*__o1_0).object_base
        } else {
            0 as *mut i8
        })
            .offset(
                ((((*__o1_0).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1_0).object_base
                        } else {
                            0 as *mut i8
                        }),
                    ) as i64 as u64)
                    .wrapping_add((*__o1_0).alignment_mask) & !(*__o1_0).alignment_mask)
                    as isize,
            );
        if ((*__o1_0).next_free).offset_from((*__o1_0).chunk as *mut i8) as i64 as size_t
            > ((*__o1_0).chunk_limit).offset_from((*__o1_0).chunk as *mut i8) as i64
                as size_t
        {
            (*__o1_0).next_free = (*__o1_0).chunk_limit;
        }
        (*__o1_0).object_base = (*__o1_0).next_free;
        __value
    });
}
#[no_mangle]
pub unsafe extern "C" fn close_space(mut divvy: *mut divvy) {
    let mut __o: *mut obstack = &mut (*divvy).space;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut i8;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    (*divvy).count = 0 as i32 as size_t;
    (*divvy).first = 0 as *mut libc::c_void;
    free(divvy as *mut libc::c_void);
}