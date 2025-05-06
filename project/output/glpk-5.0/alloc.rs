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
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut i8,
    pub term_out: i32,
    pub term_hook: Option<unsafe extern "C" fn(*mut libc::c_void, *const i8) -> i32>,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: i32,
    pub err_file: *const i8,
    pub err_line: i32,
    pub err_hook: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut i8,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: i32,
    pub mem_cpeak: i32,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: i32,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
unsafe extern "C" fn dma(
    mut func: *const i8,
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut mbd: *mut MBD = 0 as *mut MBD;
    if ptr.is_null() {
        mbd = 0 as *mut MBD;
    } else {
        mbd = (ptr as *mut i8)
            .offset(
                -((::core::mem::size_of::<MBD>() as u64)
                    .wrapping_add((16 as i32 - 1 as i32) as u64)
                    .wrapping_div(16 as i32 as u64)
                    .wrapping_mul(16 as i32 as u64) as isize),
            ) as *mut MBD;
        if (*mbd).self_0 != mbd {
            (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 57 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"%s: ptr = %p; invalid pointer\n\0" as *const u8 as *const i8,
                func,
                ptr,
            );
        }
        (*mbd).self_0 = 0 as *mut MBD;
        if ((*mbd).prev).is_null() {
            (*env).mem_ptr = (*mbd).next;
        } else {
            (*(*mbd).prev).next = (*mbd).next;
        }
        if !((*mbd).next).is_null() {
            (*(*mbd).next).prev = (*mbd).prev;
        }
        if !((*env).mem_count >= 1 as i32 && (*env).mem_total >= (*mbd).size) {
            (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 70 as i32))
                .expect(
                    "non-null function pointer",
                )(b"%s: memory allocation error\n\0" as *const u8 as *const i8, func);
        }
        (*env).mem_count -= 1;
        (*env).mem_count;
        (*env).mem_total = ((*env).mem_total as u64).wrapping_sub((*mbd).size) as size_t
            as size_t;
        if size == 0 as i32 as u64 {
            free(mbd as *mut libc::c_void);
            return 0 as *mut libc::c_void;
        }
    }
    if size
        > (!(0 as i32 as size_t))
            .wrapping_sub(
                (::core::mem::size_of::<MBD>() as u64)
                    .wrapping_add((16 as i32 - 1 as i32) as u64)
                    .wrapping_div(16 as i32 as u64)
                    .wrapping_mul(16 as i32 as u64),
            )
    {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 81 as i32))
            .expect(
                "non-null function pointer",
            )(b"%s: block too large\n\0" as *const u8 as *const i8, func);
    }
    size = (size as u64)
        .wrapping_add(
            (::core::mem::size_of::<MBD>() as u64)
                .wrapping_add((16 as i32 - 1 as i32) as u64)
                .wrapping_div(16 as i32 as u64)
                .wrapping_mul(16 as i32 as u64),
        ) as size_t as size_t;
    if size > ((*env).mem_limit).wrapping_sub((*env).mem_total) {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 84 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"%s: memory allocation limit exceeded\n\0" as *const u8 as *const i8,
            func,
        );
    }
    if (*env).mem_count == 2147483647 as i32 {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 86 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"%s: too many memory blocks allocated\n\0" as *const u8 as *const i8,
            func,
        );
    }
    mbd = (if mbd.is_null() {
        malloc(size)
    } else {
        realloc(mbd as *mut libc::c_void, size)
    }) as *mut MBD;
    if mbd.is_null() {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 89 as i32))
            .expect(
                "non-null function pointer",
            )(b"%s: no memory available\n\0" as *const u8 as *const i8, func);
    }
    (*mbd).size = size;
    (*mbd).self_0 = mbd;
    (*mbd).prev = 0 as *mut MBD;
    (*mbd).next = (*env).mem_ptr;
    if !((*mbd).next).is_null() {
        (*(*mbd).next).prev = mbd;
    }
    (*env).mem_ptr = mbd;
    (*env).mem_count += 1;
    (*env).mem_count;
    if (*env).mem_cpeak < (*env).mem_count {
        (*env).mem_cpeak = (*env).mem_count;
    }
    (*env).mem_total = ((*env).mem_total as u64).wrapping_add(size) as size_t as size_t;
    if (*env).mem_tpeak < (*env).mem_total {
        (*env).mem_tpeak = (*env).mem_total;
    }
    return (mbd as *mut i8)
        .offset(
            (::core::mem::size_of::<MBD>() as u64)
                .wrapping_add((16 as i32 - 1 as i32) as u64)
                .wrapping_div(16 as i32 as u64)
                .wrapping_mul(16 as i32 as u64) as isize,
        ) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn glp_alloc(mut n: i32, mut size: i32) -> *mut libc::c_void {
    if n < 1 as i32 {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 133 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_alloc: n = %d; invalid parameter\n\0" as *const u8 as *const i8, n);
    }
    if size < 1 as i32 {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 135 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_alloc: size = %d; invalid parameter\n\0" as *const u8 as *const i8,
            size,
        );
    }
    if n as size_t > (!(0 as i32 as size_t)).wrapping_div(size as size_t) {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 137 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_alloc: n = %d, size = %d; block too large\n\0" as *const u8
                as *const i8,
            n,
            size,
        );
    }
    return dma(
        b"glp_alloc\0" as *const u8 as *const i8,
        0 as *mut libc::c_void,
        (n as size_t).wrapping_mul(size as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn glp_realloc(
    mut ptr: *mut libc::c_void,
    mut n: i32,
    mut size: i32,
) -> *mut libc::c_void {
    if ptr.is_null() {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 147 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_realloc: ptr = %p; invalid pointer\n\0" as *const u8 as *const i8,
            ptr,
        );
    }
    if n < 1 as i32 {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 149 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_realloc: n = %d; invalid parameter\n\0" as *const u8 as *const i8,
            n,
        );
    }
    if size < 1 as i32 {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 151 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_realloc: size = %d; invalid parameter\n\0" as *const u8 as *const i8,
            size,
        );
    }
    if n as size_t > (!(0 as i32 as size_t)).wrapping_div(size as size_t) {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 153 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_realloc: n = %d, size = %d; block too large\n\0" as *const u8
                as *const i8,
            n,
            size,
        );
    }
    return dma(
        b"glp_realloc\0" as *const u8 as *const i8,
        ptr,
        (n as size_t).wrapping_mul(size as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn glp_free(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 175 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_free: ptr = %p; invalid pointer\n\0" as *const u8 as *const i8, ptr);
    }
    dma(b"glp_free\0" as *const u8 as *const i8, ptr, 0 as i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn glp_mem_limit(mut limit: i32) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if limit < 1 as i32 {
        (glp_error_(b"env/alloc.c\0" as *const u8 as *const i8, 197 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mem_limit: limit = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            limit,
        );
    }
    if limit as size_t <= !(0 as i32 as size_t) >> 20 as i32 {
        (*env).mem_limit = (limit as size_t) << 20 as i32;
    } else {
        (*env).mem_limit = !(0 as i32 as size_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_mem_usage(
    mut count: *mut i32,
    mut cpeak: *mut i32,
    mut total: *mut size_t,
    mut tpeak: *mut size_t,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !count.is_null() {
        *count = (*env).mem_count;
    }
    if !cpeak.is_null() {
        *cpeak = (*env).mem_cpeak;
    }
    if !total.is_null() {
        *total = (*env).mem_total;
    }
    if !tpeak.is_null() {
        *tpeak = (*env).mem_tpeak;
    }
}