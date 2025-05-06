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
    pub type DMP;
    fn _glp_bigmul(n: i32, m: i32, x: *mut libc::c_ushort, y: *mut libc::c_ushort);
    fn _glp_bigdiv(n: i32, m: i32, x: *mut libc::c_ushort, y: *mut libc::c_ushort);
    fn frexp(_: libc::c_double, _: *mut i32) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: i32) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
    fn _glp_dmp_in_use(pool: *mut DMP) -> size_t;
    static mut stdout: *mut _IO_FILE;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz {
    pub val: i32,
    pub ptr: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz_seg {
    pub d: [libc::c_ushort; 6],
    pub next: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}
pub type mpz_t = *mut mpz;
pub type mpq_t = *mut mpq;
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
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type FILE = _IO_FILE;
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
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[no_mangle]
pub unsafe extern "C" fn _glp_gmp_get_atom(mut size: i32) -> *mut libc::c_void {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if ((*env).gmp_pool).is_null() {
        (*env).gmp_pool = _glp_dmp_create_pool() as *mut libc::c_void;
    }
    return _glp_dmp_get_atom((*env).gmp_pool as *mut DMP, size);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_gmp_free_atom(mut ptr: *mut libc::c_void, mut size: i32) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    (!((*env).gmp_pool).is_null()
        || {
            glp_assert_(
                b"gmp_pool != NULL\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                47 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_dmp_free_atom((*env).gmp_pool as *mut DMP, ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_gmp_pool_count() -> i32 {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if ((*env).gmp_pool).is_null() {
        return 0 as i32
    } else {
        return _glp_dmp_in_use((*env).gmp_pool as *mut DMP) as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_gmp_get_work(mut size: i32) -> *mut libc::c_ushort {
    let mut env: *mut ENV = _glp_get_env_ptr();
    (size > 0 as i32
        || {
            glp_assert_(
                b"size > 0\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                62 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*env).gmp_size < size {
        if (*env).gmp_size == 0 as i32 {
            (((*env).gmp_work).is_null()
                || {
                    glp_assert_(
                        b"gmp_work == NULL\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        65 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*env).gmp_size = 100 as i32;
        } else {
            (!((*env).gmp_work).is_null()
                || {
                    glp_assert_(
                        b"gmp_work != NULL\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        69 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            glp_free((*env).gmp_work as *mut libc::c_void);
        }
        while (*env).gmp_size < size {
            (*env).gmp_size += (*env).gmp_size;
        }
        (*env).gmp_work = glp_alloc(
            (*env).gmp_size,
            ::core::mem::size_of::<libc::c_ushort>() as u64 as i32,
        ) as *mut libc::c_ushort;
    }
    return (*env).gmp_work;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_gmp_free_mem() {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).gmp_pool).is_null() {
        _glp_dmp_delete_pool((*env).gmp_pool as *mut DMP);
    }
    if !((*env).gmp_work).is_null() {
        glp_free((*env).gmp_work as *mut libc::c_void);
    }
    (*env).gmp_pool = 0 as *mut libc::c_void;
    (*env).gmp_size = 0 as i32;
    (*env).gmp_work = 0 as *mut libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_init() -> mpz_t {
    let mut x: mpz_t = 0 as *mut mpz;
    x = _glp_gmp_get_atom(::core::mem::size_of::<mpz>() as u64 as i32) as mpz_t;
    (*x).val = 0 as i32;
    (*x).ptr = 0 as *mut mpz_seg;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_clear(mut x: mpz_t) {
    _glp_mpz_set_si(x, 0 as i32);
    (((*x).ptr).is_null()
        || {
            glp_assert_(
                b"x->ptr == NULL\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                105 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_gmp_free_atom(
        x as *mut libc::c_void,
        ::core::mem::size_of::<mpz>() as u64 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_set(mut z: mpz_t, mut x: mpz_t) {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ee: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    if z != x {
        _glp_mpz_set_si(z, 0 as i32);
        (*z).val = (*x).val;
        (((*z).ptr).is_null()
            || {
                glp_assert_(
                    b"z->ptr == NULL\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    117 as i32,
                );
                1 as i32 != 0
            }) as i32;
        e = (*x).ptr;
        es = 0 as *mut mpz_seg;
        while !e.is_null() {
            ee = _glp_gmp_get_atom(::core::mem::size_of::<mpz_seg>() as u64 as i32)
                as *mut mpz_seg;
            memcpy(
                ((*ee).d).as_mut_ptr() as *mut libc::c_void,
                ((*e).d).as_mut_ptr() as *const libc::c_void,
                12 as i32 as u64,
            );
            (*ee).next = 0 as *mut mpz_seg;
            if ((*z).ptr).is_null() {
                (*z).ptr = ee;
            } else {
                (*es).next = ee;
            }
            es = ee;
            e = (*e).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_set_si(mut x: mpz_t, mut val: i32) {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    while !((*x).ptr).is_null() {
        e = (*x).ptr;
        (*x).ptr = (*e).next;
        _glp_gmp_free_atom(
            e as *mut libc::c_void,
            ::core::mem::size_of::<mpz_seg>() as u64 as i32,
        );
    }
    if val as u32 == 0x80000000 as u32 {
        (*x).val = -(1 as i32);
        e = _glp_gmp_get_atom(::core::mem::size_of::<mpz_seg>() as u64 as i32)
            as *mut mpz_seg;
        (*x).ptr = e;
        memset(((*e).d).as_mut_ptr() as *mut libc::c_void, 0 as i32, 12 as i32 as u64);
        (*e).d[1 as i32 as usize] = 0x8000 as i32 as libc::c_ushort;
        (*e).next = 0 as *mut mpz_seg;
    } else {
        (*x).val = val;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_get_d(mut x: mpz_t) -> libc::c_double {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut j: i32 = 0;
    let mut val: libc::c_double = 0.;
    let mut deg: libc::c_double = 0.;
    if ((*x).ptr).is_null() {
        val = (*x).val as libc::c_double;
    } else {
        ((*x).val != 0 as i32
            || {
                glp_assert_(
                    b"x->val != 0\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    165 as i32,
                );
                1 as i32 != 0
            }) as i32;
        val = 0.0f64;
        deg = 1.0f64;
        e = (*x).ptr;
        while !e.is_null() {
            j = 0 as i32;
            while j <= 5 as i32 {
                val += deg * (*e).d[j as usize] as i32 as libc::c_double;
                deg *= 65536.0f64;
                j += 1;
                j;
            }
            e = (*e).next;
        }
        if (*x).val < 0 as i32 {
            val = -val;
        }
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_get_d_2exp(
    mut exp: *mut i32,
    mut x: mpz_t,
) -> libc::c_double {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut n1: i32 = 0;
    let mut val: libc::c_double = 0.;
    if ((*x).ptr).is_null() {
        val = (*x).val as libc::c_double;
        n = 0 as i32;
    } else {
        ((*x).val != 0 as i32
            || {
                glp_assert_(
                    b"x->val != 0\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    193 as i32,
                );
                1 as i32 != 0
            }) as i32;
        val = 0.0f64;
        n = 0 as i32;
        e = (*x).ptr;
        while !e.is_null() {
            j = 0 as i32;
            while j <= 5 as i32 {
                val += (*e).d[j as usize] as i32 as libc::c_double;
                val /= 65536.0f64;
                n += 16 as i32;
                j += 1;
                j;
            }
            e = (*e).next;
        }
        if (*x).val < 0 as i32 {
            val = -val;
        }
    }
    val = frexp(val, &mut n1);
    *exp = n + n1;
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_swap(mut x: mpz_t, mut y: mpz_t) {
    let mut val: i32 = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    val = (*x).val;
    ptr = (*x).ptr as *mut libc::c_void;
    (*x).val = (*y).val;
    (*x).ptr = (*y).ptr;
    (*y).val = val;
    (*y).ptr = ptr as *mut mpz_seg;
}
unsafe extern "C" fn normalize(mut x: mpz_t) {
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    if ((*x).ptr).is_null() {
        ((*x).val as u32 != 0x80000000 as u32
            || {
                glp_assert_(
                    b"x->val != 0x80000000\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    225 as i32,
                );
                1 as i32 != 0
            }) as i32;
    } else {
        ((*x).val == 1 as i32 || (*x).val == -(1 as i32)
            || {
                glp_assert_(
                    b"x->val == +1 || x->val == -1\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    228 as i32,
                );
                1 as i32 != 0
            }) as i32;
        es = 0 as *mut mpz_seg;
        e = (*x).ptr;
        while !e.is_null() {
            if (*e).d[0 as i32 as usize] as i32 != 0
                || (*e).d[1 as i32 as usize] as i32 != 0
                || (*e).d[2 as i32 as usize] as i32 != 0
                || (*e).d[3 as i32 as usize] as i32 != 0
                || (*e).d[4 as i32 as usize] as i32 != 0
                || (*e).d[5 as i32 as usize] as i32 != 0
            {
                es = e;
            }
            e = (*e).next;
        }
        if es.is_null() {
            _glp_mpz_set_si(x, 0 as i32);
        } else {
            while !((*es).next).is_null() {
                e = (*es).next;
                (*es).next = (*e).next;
                _glp_gmp_free_atom(
                    e as *mut libc::c_void,
                    ::core::mem::size_of::<mpz_seg>() as u64 as i32,
                );
            }
            e = (*x).ptr;
            if ((*e).next).is_null() && (*e).d[1 as i32 as usize] as i32 <= 0x7fff as i32
                && (*e).d[2 as i32 as usize] == 0 && (*e).d[3 as i32 as usize] == 0
                && (*e).d[4 as i32 as usize] == 0 && (*e).d[5 as i32 as usize] == 0
            {
                let mut val: i32 = 0;
                val = (*e).d[0 as i32 as usize] as i32
                    + (((*e).d[1 as i32 as usize] as i32) << 16 as i32);
                if (*x).val < 0 as i32 {
                    val = -val;
                }
                _glp_mpz_set_si(x, val);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_add(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    let mut current_block: u64;
    static mut zero: mpz_seg = {
        let mut init = mpz_seg {
            d: [
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
            ],
            next: 0 as *const mpz_seg as *mut mpz_seg,
        };
        init
    };
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ez: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ee: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut k: i32 = 0;
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut sz: i32 = 0;
    let mut t: u32 = 0;
    if (*x).val == 0 as i32 {
        (((*x).ptr).is_null()
            || {
                glp_assert_(
                    b"x->ptr == NULL\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    268 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_mpz_set(z, y);
    } else if (*y).val == 0 as i32 {
        (((*y).ptr).is_null()
            || {
                glp_assert_(
                    b"y->ptr == NULL\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    274 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_mpz_set(z, x);
    } else {
        if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
            let mut xval: i32 = (*x).val;
            let mut yval: i32 = (*y).val;
            let mut zval: i32 = (*x).val + (*y).val;
            (xval as u32 != 0x80000000 as u32 && yval as u32 != 0x80000000 as u32
                || {
                    glp_assert_(
                        b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                            as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        281 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if !(xval > 0 as i32 && yval > 0 as i32 && zval <= 0 as i32
                || xval < 0 as i32 && yval < 0 as i32 && zval >= 0 as i32)
            {
                _glp_mpz_set_si(z, zval);
                current_block = 5175267383245820967;
            } else {
                current_block = 13536709405535804910;
            }
        } else {
            current_block = 13536709405535804910;
        }
        match current_block {
            5175267383245820967 => {}
            _ => {
                if ((*x).ptr).is_null() {
                    ((*x).val as u32 != 0x80000000 as u32
                        || {
                            glp_assert_(
                                b"x->val != 0x80000000\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                290 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*x).val >= 0 as i32 {
                        sx = 1 as i32;
                        t = (*x).val as u32;
                    } else {
                        sx = -(1 as i32);
                        t = -(*x).val as u32;
                    }
                    ex = &mut dumx;
                    (*ex).d[0 as i32 as usize] = t as libc::c_ushort;
                    (*ex).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
                    (*ex).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                    (*ex).d[4 as i32 as usize] = (*ex).d[5 as i32 as usize];
                    (*ex).d[3 as i32 as usize] = (*ex).d[4 as i32 as usize];
                    (*ex).d[2 as i32 as usize] = (*ex).d[3 as i32 as usize];
                    (*ex).next = 0 as *mut mpz_seg;
                } else {
                    sx = (*x).val;
                    (sx == 1 as i32 || sx == -(1 as i32)
                        || {
                            glp_assert_(
                                b"sx == +1 || sx == -1\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                307 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ex = (*x).ptr;
                }
                if ((*y).ptr).is_null() {
                    ((*y).val as u32 != 0x80000000 as u32
                        || {
                            glp_assert_(
                                b"y->val != 0x80000000\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                312 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*y).val >= 0 as i32 {
                        sy = 1 as i32;
                        t = (*y).val as u32;
                    } else {
                        sy = -(1 as i32);
                        t = -(*y).val as u32;
                    }
                    ey = &mut dumy;
                    (*ey).d[0 as i32 as usize] = t as libc::c_ushort;
                    (*ey).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
                    (*ey).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                    (*ey).d[4 as i32 as usize] = (*ey).d[5 as i32 as usize];
                    (*ey).d[3 as i32 as usize] = (*ey).d[4 as i32 as usize];
                    (*ey).d[2 as i32 as usize] = (*ey).d[3 as i32 as usize];
                    (*ey).next = 0 as *mut mpz_seg;
                } else {
                    sy = (*y).val;
                    (sy == 1 as i32 || sy == -(1 as i32)
                        || {
                            glp_assert_(
                                b"sy == +1 || sy == -1\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                329 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ey = (*y).ptr;
                }
                sz = sx;
                es = 0 as *mut mpz_seg;
                ez = es;
                if sx > 0 as i32 && sy > 0 as i32 || sx < 0 as i32 && sy < 0 as i32 {
                    t = 0 as i32 as u32;
                    while !ex.is_null() || !ey.is_null() {
                        if ex.is_null() {
                            ex = &mut zero;
                        }
                        if ey.is_null() {
                            ey = &mut zero;
                        }
                        ee = _glp_gmp_get_atom(
                            ::core::mem::size_of::<mpz_seg>() as u64 as i32,
                        ) as *mut mpz_seg;
                        k = 0 as i32;
                        while k <= 5 as i32 {
                            t = t.wrapping_add((*ex).d[k as usize] as u32);
                            t = t.wrapping_add((*ey).d[k as usize] as u32);
                            (*ee).d[k as usize] = t as libc::c_ushort;
                            t >>= 16 as i32;
                            k += 1;
                            k;
                        }
                        (*ee).next = 0 as *mut mpz_seg;
                        if ez.is_null() {
                            ez = ee;
                        } else {
                            (*es).next = ee;
                        }
                        es = ee;
                        ex = (*ex).next;
                        ey = (*ey).next;
                    }
                    if t != 0 {
                        ee = _glp_gmp_get_atom(
                            ::core::mem::size_of::<mpz_seg>() as u64 as i32,
                        ) as *mut mpz_seg;
                        (*ee).d[0 as i32 as usize] = 1 as i32 as libc::c_ushort;
                        (*ee).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                        (*ee).d[4 as i32 as usize] = (*ee).d[5 as i32 as usize];
                        (*ee).d[3 as i32 as usize] = (*ee).d[4 as i32 as usize];
                        (*ee).d[2 as i32 as usize] = (*ee).d[3 as i32 as usize];
                        (*ee).d[1 as i32 as usize] = (*ee).d[2 as i32 as usize];
                        (*ee).next = 0 as *mut mpz_seg;
                        (!es.is_null()
                            || {
                                glp_assert_(
                                    b"es != NULL\0" as *const u8 as *const i8,
                                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                                    363 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        (*es).next = ee;
                    }
                } else {
                    t = 1 as i32 as u32;
                    while !ex.is_null() || !ey.is_null() {
                        if ex.is_null() {
                            ex = &mut zero;
                        }
                        if ey.is_null() {
                            ey = &mut zero;
                        }
                        ee = _glp_gmp_get_atom(
                            ::core::mem::size_of::<mpz_seg>() as u64 as i32,
                        ) as *mut mpz_seg;
                        k = 0 as i32;
                        while k <= 5 as i32 {
                            t = t.wrapping_add((*ex).d[k as usize] as u32);
                            t = t
                                .wrapping_add(
                                    (0xffff as i32 as u32)
                                        .wrapping_sub((*ey).d[k as usize] as u32),
                                );
                            (*ee).d[k as usize] = t as libc::c_ushort;
                            t >>= 16 as i32;
                            k += 1;
                            k;
                        }
                        (*ee).next = 0 as *mut mpz_seg;
                        if ez.is_null() {
                            ez = ee;
                        } else {
                            (*es).next = ee;
                        }
                        es = ee;
                        ex = (*ex).next;
                        ey = (*ey).next;
                    }
                    if t == 0 {
                        sz = -sz;
                        t = 1 as i32 as u32;
                        ee = ez;
                        while !ee.is_null() {
                            k = 0 as i32;
                            while k <= 5 as i32 {
                                t = t
                                    .wrapping_add(
                                        (0xffff as i32 as u32)
                                            .wrapping_sub((*ee).d[k as usize] as u32),
                                    );
                                (*ee).d[k as usize] = t as libc::c_ushort;
                                t >>= 16 as i32;
                                k += 1;
                                k;
                            }
                            ee = (*ee).next;
                        }
                    }
                }
                _glp_mpz_set_si(z, 0 as i32);
                (*z).val = sz;
                (*z).ptr = ez;
                normalize(z);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_sub(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    if x == y {
        _glp_mpz_set_si(z, 0 as i32);
    } else {
        (*y).val = -(*y).val;
        _glp_mpz_add(z, x, y);
        if y != z {
            (*y).val = -(*y).val;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_mul(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    let mut current_block: u64;
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut k: i32 = 0;
    let mut nx: i32 = 0;
    let mut ny: i32 = 0;
    let mut n: i32 = 0;
    let mut t: u32 = 0;
    let mut work: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wx: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wy: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    if (*x).val == 0 as i32 {
        (((*x).ptr).is_null()
            || {
                glp_assert_(
                    b"x->ptr == NULL\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    431 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_mpz_set_si(z, 0 as i32);
    } else if (*y).val == 0 as i32 {
        (((*y).ptr).is_null()
            || {
                glp_assert_(
                    b"y->ptr == NULL\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    437 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_mpz_set_si(z, 0 as i32);
    } else {
        if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
            let mut xval: i32 = (*x).val;
            let mut yval: i32 = (*y).val;
            let mut sz: i32 = 1 as i32;
            (xval as u32 != 0x80000000 as u32 && yval as u32 != 0x80000000 as u32
                || {
                    glp_assert_(
                        b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                            as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        444 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if xval < 0 as i32 {
                xval = -xval;
                sz = -sz;
            }
            if yval < 0 as i32 {
                yval = -yval;
                sz = -sz;
            }
            if xval <= 0x7fffffff as i32 / yval {
                _glp_mpz_set_si(z, sz * (xval * yval));
                current_block = 1570142262458892093;
            } else {
                current_block = 12599329904712511516;
            }
        } else {
            current_block = 12599329904712511516;
        }
        match current_block {
            1570142262458892093 => {}
            _ => {
                if ((*x).ptr).is_null() {
                    ((*x).val as u32 != 0x80000000 as u32
                        || {
                            glp_assert_(
                                b"x->val != 0x80000000\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                456 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*x).val >= 0 as i32 {
                        sx = 1 as i32;
                        t = (*x).val as u32;
                    } else {
                        sx = -(1 as i32);
                        t = -(*x).val as u32;
                    }
                    ex = &mut dumx;
                    (*ex).d[0 as i32 as usize] = t as libc::c_ushort;
                    (*ex).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
                    (*ex).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                    (*ex).d[4 as i32 as usize] = (*ex).d[5 as i32 as usize];
                    (*ex).d[3 as i32 as usize] = (*ex).d[4 as i32 as usize];
                    (*ex).d[2 as i32 as usize] = (*ex).d[3 as i32 as usize];
                    (*ex).next = 0 as *mut mpz_seg;
                } else {
                    sx = (*x).val;
                    (sx == 1 as i32 || sx == -(1 as i32)
                        || {
                            glp_assert_(
                                b"sx == +1 || sx == -1\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                473 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ex = (*x).ptr;
                }
                if ((*y).ptr).is_null() {
                    ((*y).val as u32 != 0x80000000 as u32
                        || {
                            glp_assert_(
                                b"y->val != 0x80000000\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                478 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*y).val >= 0 as i32 {
                        sy = 1 as i32;
                        t = (*y).val as u32;
                    } else {
                        sy = -(1 as i32);
                        t = -(*y).val as u32;
                    }
                    ey = &mut dumy;
                    (*ey).d[0 as i32 as usize] = t as libc::c_ushort;
                    (*ey).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
                    (*ey).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                    (*ey).d[4 as i32 as usize] = (*ey).d[5 as i32 as usize];
                    (*ey).d[3 as i32 as usize] = (*ey).d[4 as i32 as usize];
                    (*ey).d[2 as i32 as usize] = (*ey).d[3 as i32 as usize];
                    (*ey).next = 0 as *mut mpz_seg;
                } else {
                    sy = (*y).val;
                    (sy == 1 as i32 || sy == -(1 as i32)
                        || {
                            glp_assert_(
                                b"sy == +1 || sy == -1\0" as *const u8 as *const i8,
                                b"misc/mygmp.c\0" as *const u8 as *const i8,
                                495 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ey = (*y).ptr;
                }
                n = 0 as i32;
                nx = n;
                e = ex;
                while !e.is_null() {
                    k = 0 as i32;
                    while k <= 5 as i32 {
                        n += 1;
                        n;
                        if (*e).d[k as usize] != 0 {
                            nx = n;
                        }
                        k += 1;
                        k;
                    }
                    e = (*e).next;
                }
                (nx > 0 as i32
                    || {
                        glp_assert_(
                            b"nx > 0\0" as *const u8 as *const i8,
                            b"misc/mygmp.c\0" as *const u8 as *const i8,
                            507 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                n = 0 as i32;
                ny = n;
                e = ey;
                while !e.is_null() {
                    k = 0 as i32;
                    while k <= 5 as i32 {
                        n += 1;
                        n;
                        if (*e).d[k as usize] != 0 {
                            ny = n;
                        }
                        k += 1;
                        k;
                    }
                    e = (*e).next;
                }
                (ny > 0 as i32
                    || {
                        glp_assert_(
                            b"ny > 0\0" as *const u8 as *const i8,
                            b"misc/mygmp.c\0" as *const u8 as *const i8,
                            517 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                work = _glp_gmp_get_work(nx + ny + ny);
                wx = &mut *work.offset(0 as i32 as isize) as *mut libc::c_ushort;
                n = 0 as i32;
                while n < nx {
                    *wx.offset((ny + n) as isize) = 0 as i32 as libc::c_ushort;
                    n += 1;
                    n;
                }
                n = 0 as i32;
                e = ex;
                while !e.is_null() {
                    k = 0 as i32;
                    while k <= 5 as i32 {
                        if (*e).d[k as usize] != 0 {
                            *wx.offset((ny + n) as isize) = (*e).d[k as usize];
                        }
                        k += 1;
                        k;
                        n += 1;
                        n;
                    }
                    e = (*e).next;
                }
                wy = &mut *work.offset((nx + ny) as isize) as *mut libc::c_ushort;
                n = 0 as i32;
                while n < ny {
                    *wy.offset(n as isize) = 0 as i32 as libc::c_ushort;
                    n += 1;
                    n;
                }
                n = 0 as i32;
                e = ey;
                while !e.is_null() {
                    k = 0 as i32;
                    while k <= 5 as i32 {
                        if (*e).d[k as usize] != 0 {
                            *wy.offset(n as isize) = (*e).d[k as usize];
                        }
                        k += 1;
                        k;
                        n += 1;
                        n;
                    }
                    e = (*e).next;
                }
                _glp_bigmul(nx, ny, wx, wy);
                _glp_mpz_set_si(z, 0 as i32);
                (*z).val = sx * sy;
                es = 0 as *mut mpz_seg;
                k = 6 as i32;
                n = 0 as i32;
                while n < nx + ny {
                    if k > 5 as i32 {
                        e = _glp_gmp_get_atom(
                            ::core::mem::size_of::<mpz_seg>() as u64 as i32,
                        ) as *mut mpz_seg;
                        (*e).d[2 as i32 as usize] = 0 as i32 as libc::c_ushort;
                        (*e).d[1 as i32 as usize] = (*e).d[2 as i32 as usize];
                        (*e).d[0 as i32 as usize] = (*e).d[1 as i32 as usize];
                        (*e).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                        (*e).d[4 as i32 as usize] = (*e).d[5 as i32 as usize];
                        (*e).d[3 as i32 as usize] = (*e).d[4 as i32 as usize];
                        (*e).next = 0 as *mut mpz_seg;
                        if ((*z).ptr).is_null() {
                            (*z).ptr = e;
                        } else {
                            (*es).next = e;
                        }
                        es = e;
                        k = 0 as i32;
                    }
                    let fresh0 = k;
                    k = k + 1;
                    (*es).d[fresh0 as usize] = *wx.offset(n as isize);
                    n += 1;
                    n;
                }
                normalize(z);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_neg(mut z: mpz_t, mut x: mpz_t) {
    _glp_mpz_set(z, x);
    (*z).val = -(*z).val;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_abs(mut z: mpz_t, mut x: mpz_t) {
    _glp_mpz_set(z, x);
    if (*z).val < 0 as i32 {
        (*z).val = -(*z).val;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_div(
    mut q: mpz_t,
    mut r: mpz_t,
    mut x: mpz_t,
    mut y: mpz_t,
) {
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut k: i32 = 0;
    let mut nx: i32 = 0;
    let mut ny: i32 = 0;
    let mut n: i32 = 0;
    let mut t: u32 = 0;
    let mut work: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wx: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wy: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    if (*y).val == 0 as i32 {
        (((*y).ptr).is_null()
            || {
                glp_assert_(
                    b"y->ptr == NULL\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    596 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (glp_error_(b"misc/mygmp.c\0" as *const u8 as *const i8, 597 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpz_div: divide by zero not allowed\n\0" as *const u8 as *const i8);
    }
    if (*x).val == 0 as i32 {
        (((*x).ptr).is_null()
            || {
                glp_assert_(
                    b"x->ptr == NULL\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    601 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if !q.is_null() {
            _glp_mpz_set_si(q, 0 as i32);
        }
        if !r.is_null() {
            _glp_mpz_set_si(r, 0 as i32);
        }
    } else if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
        let mut xval: i32 = (*x).val;
        let mut yval: i32 = (*y).val;
        (xval as u32 != 0x80000000 as u32 && yval as u32 != 0x80000000 as u32
            || {
                glp_assert_(
                    b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                        as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    611 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if !q.is_null() {
            _glp_mpz_set_si(q, xval / yval);
        }
        if !r.is_null() {
            _glp_mpz_set_si(r, xval % yval);
        }
    } else {
        if ((*x).ptr).is_null() {
            ((*x).val as u32 != 0x80000000 as u32
                || {
                    glp_assert_(
                        b"x->val != 0x80000000\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        621 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*x).val >= 0 as i32 {
                sx = 1 as i32;
                t = (*x).val as u32;
            } else {
                sx = -(1 as i32);
                t = -(*x).val as u32;
            }
            ex = &mut dumx;
            (*ex).d[0 as i32 as usize] = t as libc::c_ushort;
            (*ex).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
            (*ex).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
            (*ex).d[4 as i32 as usize] = (*ex).d[5 as i32 as usize];
            (*ex).d[3 as i32 as usize] = (*ex).d[4 as i32 as usize];
            (*ex).d[2 as i32 as usize] = (*ex).d[3 as i32 as usize];
            (*ex).next = 0 as *mut mpz_seg;
        } else {
            sx = (*x).val;
            (sx == 1 as i32 || sx == -(1 as i32)
                || {
                    glp_assert_(
                        b"sx == +1 || sx == -1\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        638 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ex = (*x).ptr;
        }
        if ((*y).ptr).is_null() {
            ((*y).val as u32 != 0x80000000 as u32
                || {
                    glp_assert_(
                        b"y->val != 0x80000000\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        643 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*y).val >= 0 as i32 {
                sy = 1 as i32;
                t = (*y).val as u32;
            } else {
                sy = -(1 as i32);
                t = -(*y).val as u32;
            }
            ey = &mut dumy;
            (*ey).d[0 as i32 as usize] = t as libc::c_ushort;
            (*ey).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
            (*ey).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
            (*ey).d[4 as i32 as usize] = (*ey).d[5 as i32 as usize];
            (*ey).d[3 as i32 as usize] = (*ey).d[4 as i32 as usize];
            (*ey).d[2 as i32 as usize] = (*ey).d[3 as i32 as usize];
            (*ey).next = 0 as *mut mpz_seg;
        } else {
            sy = (*y).val;
            (sy == 1 as i32 || sy == -(1 as i32)
                || {
                    glp_assert_(
                        b"sy == +1 || sy == -1\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        660 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ey = (*y).ptr;
        }
        n = 0 as i32;
        nx = n;
        e = ex;
        while !e.is_null() {
            k = 0 as i32;
            while k <= 5 as i32 {
                n += 1;
                n;
                if (*e).d[k as usize] != 0 {
                    nx = n;
                }
                k += 1;
                k;
            }
            e = (*e).next;
        }
        (nx > 0 as i32
            || {
                glp_assert_(
                    b"nx > 0\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    672 as i32,
                );
                1 as i32 != 0
            }) as i32;
        n = 0 as i32;
        ny = n;
        e = ey;
        while !e.is_null() {
            k = 0 as i32;
            while k <= 5 as i32 {
                n += 1;
                n;
                if (*e).d[k as usize] != 0 {
                    ny = n;
                }
                k += 1;
                k;
            }
            e = (*e).next;
        }
        (ny > 0 as i32
            || {
                glp_assert_(
                    b"ny > 0\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    682 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if nx < ny {
            if !r.is_null() {
                _glp_mpz_set(r, x);
            }
            if !q.is_null() {
                _glp_mpz_set_si(q, 0 as i32);
            }
        } else {
            work = _glp_gmp_get_work(nx + ny + 1 as i32);
            wx = &mut *work.offset(0 as i32 as isize) as *mut libc::c_ushort;
            n = 0 as i32;
            while n < nx {
                *wx.offset(n as isize) = 0 as i32 as libc::c_ushort;
                n += 1;
                n;
            }
            n = 0 as i32;
            e = ex;
            while !e.is_null() {
                k = 0 as i32;
                while k <= 5 as i32 {
                    if (*e).d[k as usize] != 0 {
                        *wx.offset(n as isize) = (*e).d[k as usize];
                    }
                    k += 1;
                    k;
                    n += 1;
                    n;
                }
                e = (*e).next;
            }
            wy = &mut *work.offset((nx + 1 as i32) as isize) as *mut libc::c_ushort;
            n = 0 as i32;
            while n < ny {
                *wy.offset(n as isize) = 0 as i32 as libc::c_ushort;
                n += 1;
                n;
            }
            n = 0 as i32;
            e = ey;
            while !e.is_null() {
                k = 0 as i32;
                while k <= 5 as i32 {
                    if (*e).d[k as usize] != 0 {
                        *wy.offset(n as isize) = (*e).d[k as usize];
                    }
                    k += 1;
                    k;
                    n += 1;
                    n;
                }
                e = (*e).next;
            }
            (*wy.offset((ny - 1 as i32) as isize) as i32 != 0 as i32
                || {
                    glp_assert_(
                        b"wy[ny-1] != 0\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        710 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            _glp_bigdiv(nx - ny, ny, wx, wy);
            if !q.is_null() {
                _glp_mpz_set_si(q, 0 as i32);
                (*q).val = sx * sy;
                es = 0 as *mut mpz_seg;
                k = 6 as i32;
                n = ny;
                while n <= nx {
                    if k > 5 as i32 {
                        e = _glp_gmp_get_atom(
                            ::core::mem::size_of::<mpz_seg>() as u64 as i32,
                        ) as *mut mpz_seg;
                        (*e).d[2 as i32 as usize] = 0 as i32 as libc::c_ushort;
                        (*e).d[1 as i32 as usize] = (*e).d[2 as i32 as usize];
                        (*e).d[0 as i32 as usize] = (*e).d[1 as i32 as usize];
                        (*e).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                        (*e).d[4 as i32 as usize] = (*e).d[5 as i32 as usize];
                        (*e).d[3 as i32 as usize] = (*e).d[4 as i32 as usize];
                        (*e).next = 0 as *mut mpz_seg;
                        if ((*q).ptr).is_null() {
                            (*q).ptr = e;
                        } else {
                            (*es).next = e;
                        }
                        es = e;
                        k = 0 as i32;
                    }
                    let fresh1 = k;
                    k = k + 1;
                    (*es).d[fresh1 as usize] = *wx.offset(n as isize);
                    n += 1;
                    n;
                }
                normalize(q);
            }
            if !r.is_null() {
                _glp_mpz_set_si(r, 0 as i32);
                (*r).val = sx;
                es = 0 as *mut mpz_seg;
                k = 6 as i32;
                n = 0 as i32;
                while n < ny {
                    if k > 5 as i32 {
                        e = _glp_gmp_get_atom(
                            ::core::mem::size_of::<mpz_seg>() as u64 as i32,
                        ) as *mut mpz_seg;
                        (*e).d[2 as i32 as usize] = 0 as i32 as libc::c_ushort;
                        (*e).d[1 as i32 as usize] = (*e).d[2 as i32 as usize];
                        (*e).d[0 as i32 as usize] = (*e).d[1 as i32 as usize];
                        (*e).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
                        (*e).d[4 as i32 as usize] = (*e).d[5 as i32 as usize];
                        (*e).d[3 as i32 as usize] = (*e).d[4 as i32 as usize];
                        (*e).next = 0 as *mut mpz_seg;
                        if ((*r).ptr).is_null() {
                            (*r).ptr = e;
                        } else {
                            (*es).next = e;
                        }
                        es = e;
                        k = 0 as i32;
                    }
                    let fresh2 = k;
                    k = k + 1;
                    (*es).d[fresh2 as usize] = *wx.offset(n as isize);
                    n += 1;
                    n;
                }
                normalize(r);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_gcd(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    let mut u: mpz_t = 0 as *mut mpz;
    let mut v: mpz_t = 0 as *mut mpz;
    let mut r: mpz_t = 0 as *mut mpz;
    u = _glp_mpz_init();
    v = _glp_mpz_init();
    r = _glp_mpz_init();
    _glp_mpz_abs(u, x);
    _glp_mpz_abs(v, y);
    while _glp_mpz_sgn(v) != 0 {
        _glp_mpz_div(0 as mpz_t, r, u, v);
        _glp_mpz_set(u, v);
        _glp_mpz_set(v, r);
    }
    _glp_mpz_set(z, u);
    _glp_mpz_clear(u);
    _glp_mpz_clear(v);
    _glp_mpz_clear(r);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_cmp(mut x: mpz_t, mut y: mpz_t) -> i32 {
    static mut zero: mpz_seg = {
        let mut init = mpz_seg {
            d: [
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
                0 as i32 as libc::c_ushort,
            ],
            next: 0 as *const mpz_seg as *mut mpz_seg,
        };
        init
    };
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut cc: i32 = 0;
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut k: i32 = 0;
    let mut t: u32 = 0;
    if x == y {
        cc = 0 as i32;
    } else if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
        let mut xval: i32 = (*x).val;
        let mut yval: i32 = (*y).val;
        (xval as u32 != 0x80000000 as u32 && yval as u32 != 0x80000000 as u32
            || {
                glp_assert_(
                    b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                        as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    797 as i32,
                );
                1 as i32 != 0
            }) as i32;
        cc = if xval > yval {
            1 as i32
        } else if xval < yval {
            -(1 as i32)
        } else {
            0 as i32
        };
    } else if (*x).val > 0 as i32 && (*y).val <= 0 as i32
        || (*x).val == 0 as i32 && (*y).val < 0 as i32
    {
        cc = 1 as i32;
    } else if (*x).val < 0 as i32 && (*y).val >= 0 as i32
        || (*x).val == 0 as i32 && (*y).val > 0 as i32
    {
        cc = -(1 as i32);
    } else {
        if ((*x).ptr).is_null() {
            ((*x).val as u32 != 0x80000000 as u32
                || {
                    glp_assert_(
                        b"x->val != 0x80000000\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        812 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*x).val >= 0 as i32 {
                sx = 1 as i32;
                t = (*x).val as u32;
            } else {
                sx = -(1 as i32);
                t = -(*x).val as u32;
            }
            ex = &mut dumx;
            (*ex).d[0 as i32 as usize] = t as libc::c_ushort;
            (*ex).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
            (*ex).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
            (*ex).d[4 as i32 as usize] = (*ex).d[5 as i32 as usize];
            (*ex).d[3 as i32 as usize] = (*ex).d[4 as i32 as usize];
            (*ex).d[2 as i32 as usize] = (*ex).d[3 as i32 as usize];
            (*ex).next = 0 as *mut mpz_seg;
        } else {
            sx = (*x).val;
            (sx == 1 as i32 || sx == -(1 as i32)
                || {
                    glp_assert_(
                        b"sx == +1 || sx == -1\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        829 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ex = (*x).ptr;
        }
        if ((*y).ptr).is_null() {
            ((*y).val as u32 != 0x80000000 as u32
                || {
                    glp_assert_(
                        b"y->val != 0x80000000\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        834 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*y).val >= 0 as i32 {
                sy = 1 as i32;
                t = (*y).val as u32;
            } else {
                sy = -(1 as i32);
                t = -(*y).val as u32;
            }
            ey = &mut dumy;
            (*ey).d[0 as i32 as usize] = t as libc::c_ushort;
            (*ey).d[1 as i32 as usize] = (t >> 16 as i32) as libc::c_ushort;
            (*ey).d[5 as i32 as usize] = 0 as i32 as libc::c_ushort;
            (*ey).d[4 as i32 as usize] = (*ey).d[5 as i32 as usize];
            (*ey).d[3 as i32 as usize] = (*ey).d[4 as i32 as usize];
            (*ey).d[2 as i32 as usize] = (*ey).d[3 as i32 as usize];
            (*ey).next = 0 as *mut mpz_seg;
        } else {
            sy = (*y).val;
            (sy == 1 as i32 || sy == -(1 as i32)
                || {
                    glp_assert_(
                        b"sy == +1 || sy == -1\0" as *const u8 as *const i8,
                        b"misc/mygmp.c\0" as *const u8 as *const i8,
                        851 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ey = (*y).ptr;
        }
        (sx > 0 as i32 && sy > 0 as i32 || sx < 0 as i32 && sy < 0 as i32
            || {
                glp_assert_(
                    b"sx > 0 && sy > 0 || sx < 0 && sy < 0\0" as *const u8 as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    855 as i32,
                );
                1 as i32 != 0
            }) as i32;
        cc = 0 as i32;
        while !ex.is_null() || !ey.is_null() {
            if ex.is_null() {
                ex = &mut zero;
            }
            if ey.is_null() {
                ey = &mut zero;
            }
            k = 0 as i32;
            while k <= 5 as i32 {
                if (*ex).d[k as usize] as i32 > (*ey).d[k as usize] as i32 {
                    cc = 1 as i32;
                }
                if ((*ex).d[k as usize] as i32) < (*ey).d[k as usize] as i32 {
                    cc = -(1 as i32);
                }
                k += 1;
                k;
            }
            ex = (*ex).next;
            ey = (*ey).next;
        }
        if sx < 0 as i32 {
            cc = -cc;
        }
    }
    return cc;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_sgn(mut x: mpz_t) -> i32 {
    let mut s: i32 = 0;
    s = if (*x).val > 0 as i32 {
        1 as i32
    } else if (*x).val < 0 as i32 {
        -(1 as i32)
    } else {
        0 as i32
    };
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpz_out_str(
    mut _fp: *mut libc::c_void,
    mut base: i32,
    mut x: mpz_t,
) -> i32 {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut b: mpz_t = 0 as *mut mpz;
    let mut y: mpz_t = 0 as *mut mpz;
    let mut r: mpz_t = 0 as *mut mpz;
    let mut n: i32 = 0;
    let mut j: i32 = 0;
    let mut nwr: i32 = 0 as i32;
    let mut d: *mut u8 = 0 as *mut u8;
    static mut set: *mut i8 = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8
        as *const i8 as *mut i8;
    if !(2 as i32 <= base && base <= 36 as i32) {
        (glp_error_(b"misc/mygmp.c\0" as *const u8 as *const i8, 891 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpz_out_str: base = %d; invalid base\n\0" as *const u8 as *const i8,
            base,
        );
    }
    b = _glp_mpz_init();
    _glp_mpz_set_si(b, base);
    y = _glp_mpz_init();
    r = _glp_mpz_init();
    _glp_mpz_abs(y, x);
    n = 0 as i32;
    while _glp_mpz_sgn(y) != 0 as i32 {
        _glp_mpz_div(y, 0 as mpz_t, y, b);
        n += 1;
        n;
    }
    if n == 0 as i32 {
        n = 1 as i32;
    }
    d = glp_alloc(1 as i32, n) as *mut u8;
    _glp_mpz_abs(y, x);
    j = 0 as i32;
    while j < n {
        _glp_mpz_div(y, r, y, b);
        (0 as i32 <= (*r).val && (*r).val < base && ((*r).ptr).is_null()
            || {
                glp_assert_(
                    b"0 <= r->val && r->val < base && r->ptr == NULL\0" as *const u8
                        as *const i8,
                    b"misc/mygmp.c\0" as *const u8 as *const i8,
                    906 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *d.offset(j as isize) = (*r).val as u8;
        j += 1;
        j;
    }
    if fp.is_null() {
        fp = stdout;
    }
    if _glp_mpz_sgn(x) < 0 as i32 {
        fputc('-' as i32, fp);
        nwr += 1;
        nwr;
    }
    j = n - 1 as i32;
    while j >= 0 as i32 {
        fputc(*set.offset(*d.offset(j as isize) as isize) as i32, fp);
        nwr += 1;
        nwr;
        j -= 1;
        j;
    }
    if ferror(fp) != 0 {
        nwr = 0 as i32;
    }
    _glp_mpz_clear(b);
    _glp_mpz_clear(y);
    _glp_mpz_clear(r);
    glp_free(d as *mut libc::c_void);
    return nwr;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_init() -> mpq_t {
    let mut x: mpq_t = 0 as *mut mpq;
    x = _glp_gmp_get_atom(::core::mem::size_of::<mpq>() as u64 as i32) as mpq_t;
    (*x).p.val = 0 as i32;
    (*x).p.ptr = 0 as *mut mpz_seg;
    (*x).q.val = 1 as i32;
    (*x).q.ptr = 0 as *mut mpz_seg;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_clear(mut x: mpq_t) {
    _glp_mpz_set_si(&mut (*x).p, 0 as i32);
    (((*x).p.ptr).is_null()
        || {
            glp_assert_(
                b"x->p.ptr == NULL\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                941 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpz_set_si(&mut (*x).q, 0 as i32);
    (((*x).q.ptr).is_null()
        || {
            glp_assert_(
                b"x->q.ptr == NULL\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                943 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_gmp_free_atom(
        x as *mut libc::c_void,
        ::core::mem::size_of::<mpq>() as u64 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_canonicalize(mut x: mpq_t) {
    let mut f: mpz_t = 0 as *mut mpz;
    ((*x).q.val != 0 as i32
        || {
            glp_assert_(
                b"x->q.val != 0\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                953 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*x).q.val < 0 as i32 {
        _glp_mpz_neg(&mut (*x).p, &mut (*x).p);
        _glp_mpz_neg(&mut (*x).q, &mut (*x).q);
    }
    f = _glp_mpz_init();
    _glp_mpz_gcd(f, &mut (*x).p, &mut (*x).q);
    if !((*f).val == 1 as i32 && ((*f).ptr).is_null()) {
        _glp_mpz_div(&mut (*x).p, 0 as mpz_t, &mut (*x).p, f);
        _glp_mpz_div(&mut (*x).q, 0 as mpz_t, &mut (*x).q, f);
    }
    _glp_mpz_clear(f);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_set(mut z: mpq_t, mut x: mpq_t) {
    if z != x {
        _glp_mpz_set(&mut (*z).p, &mut (*x).p);
        _glp_mpz_set(&mut (*z).q, &mut (*x).q);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_set_si(mut x: mpq_t, mut p: i32, mut q: u32) {
    if q == 0 as i32 as u32 {
        (glp_error_(b"misc/mygmp.c\0" as *const u8 as *const i8, 980 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpq_set_si: zero denominator not allowed\n\0" as *const u8 as *const i8);
    }
    _glp_mpz_set_si(&mut (*x).p, p);
    (q <= 0x7fffffff as i32 as u32
        || {
            glp_assert_(
                b"q <= 0x7FFFFFFF\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                982 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpz_set_si(&mut (*x).q, q as i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_get_d(mut x: mpq_t) -> libc::c_double {
    let mut np: i32 = 0;
    let mut nq: i32 = 0;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    p = _glp_mpz_get_d_2exp(&mut np, &mut (*x).p);
    q = _glp_mpz_get_d_2exp(&mut nq, &mut (*x).q);
    return ldexp(p / q, np - nq);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_set_d(mut x: mpq_t, mut val: libc::c_double) {
    let mut current_block: u64;
    let mut s: i32 = 0;
    let mut n: i32 = 0;
    let mut d: i32 = 0;
    let mut j: i32 = 0;
    let mut f: libc::c_double = 0.;
    let mut temp: mpz_t = 0 as *mut mpz;
    (-1.7976931348623157e+308f64 <= val && val <= 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"-DBL_MAX <= val && val <= +DBL_MAX\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                1001 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpq_set_si(x, 0 as i32, 1 as i32 as u32);
    if val > 0.0f64 {
        s = 1 as i32;
        current_block = 15427931788582360902;
    } else if val < 0.0f64 {
        s = -(1 as i32);
        current_block = 15427931788582360902;
    } else {
        current_block = 2055143722991225061;
    }
    match current_block {
        15427931788582360902 => {
            f = frexp(fabs(val), &mut n);
            temp = _glp_mpz_init();
            while f != 0.0f64 {
                f *= 16.0f64;
                n -= 4 as i32;
                d = f as i32;
                (0 as i32 <= d && d <= 15 as i32
                    || {
                        glp_assert_(
                            b"0 <= d && d <= 15\0" as *const u8 as *const i8,
                            b"misc/mygmp.c\0" as *const u8 as *const i8,
                            1015 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                f -= d as libc::c_double;
                _glp_mpz_set_si(temp, 16 as i32);
                _glp_mpz_mul(&mut (*x).p, &mut (*x).p, temp);
                _glp_mpz_set_si(temp, d);
                _glp_mpz_add(&mut (*x).p, &mut (*x).p, temp);
            }
            _glp_mpz_clear(temp);
            if n > 0 as i32 {
                j = 1 as i32;
                while j <= n {
                    _glp_mpz_add(&mut (*x).p, &mut (*x).p, &mut (*x).p);
                    j += 1;
                    j;
                }
            } else if n < 0 as i32 {
                j = 1 as i32;
                while j <= -n {
                    _glp_mpz_add(&mut (*x).q, &mut (*x).q, &mut (*x).q);
                    j += 1;
                    j;
                }
                _glp_mpq_canonicalize(x);
            }
            if s < 0 as i32 {
                _glp_mpq_neg(x, x);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_add(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    let mut p: mpz_t = 0 as *mut mpz;
    let mut q: mpz_t = 0 as *mut mpz;
    p = _glp_mpz_init();
    q = _glp_mpz_init();
    _glp_mpz_mul(p, &mut (*x).p, &mut (*y).q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).p);
    _glp_mpz_add(p, p, q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).q);
    _glp_mpz_set(&mut (*z).p, p);
    _glp_mpz_set(&mut (*z).q, q);
    _glp_mpz_clear(p);
    _glp_mpz_clear(q);
    _glp_mpq_canonicalize(z);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_sub(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    let mut p: mpz_t = 0 as *mut mpz;
    let mut q: mpz_t = 0 as *mut mpz;
    p = _glp_mpz_init();
    q = _glp_mpz_init();
    _glp_mpz_mul(p, &mut (*x).p, &mut (*y).q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).p);
    _glp_mpz_sub(p, p, q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).q);
    _glp_mpz_set(&mut (*z).p, p);
    _glp_mpz_set(&mut (*z).q, q);
    _glp_mpz_clear(p);
    _glp_mpz_clear(q);
    _glp_mpq_canonicalize(z);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_mul(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    _glp_mpz_mul(&mut (*z).p, &mut (*x).p, &mut (*y).p);
    _glp_mpz_mul(&mut (*z).q, &mut (*x).q, &mut (*y).q);
    _glp_mpq_canonicalize(z);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_div(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    let mut p: mpz_t = 0 as *mut mpz;
    let mut q: mpz_t = 0 as *mut mpz;
    if _glp_mpq_sgn(y) == 0 as i32 {
        (glp_error_(b"misc/mygmp.c\0" as *const u8 as *const i8, 1085 as i32))
            .expect(
                "non-null function pointer",
            )(b"mpq_div: zero divisor not allowed\n\0" as *const u8 as *const i8);
    }
    p = _glp_mpz_init();
    q = _glp_mpz_init();
    _glp_mpz_mul(p, &mut (*x).p, &mut (*y).q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).p);
    _glp_mpz_set(&mut (*z).p, p);
    _glp_mpz_set(&mut (*z).q, q);
    _glp_mpz_clear(p);
    _glp_mpz_clear(q);
    _glp_mpq_canonicalize(z);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_neg(mut z: mpq_t, mut x: mpq_t) {
    _glp_mpq_set(z, x);
    _glp_mpz_neg(&mut (*z).p, &mut (*z).p);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_abs(mut z: mpq_t, mut x: mpq_t) {
    _glp_mpq_set(z, x);
    _glp_mpz_abs(&mut (*z).p, &mut (*z).p);
    (_glp_mpz_sgn(&mut (*x).q) > 0 as i32
        || {
            glp_assert_(
                b"mpz_sgn(&x->q) > 0\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                1109 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_cmp(mut x: mpq_t, mut y: mpq_t) -> i32 {
    let mut temp: mpq_t = 0 as *mut mpq;
    let mut s: i32 = 0;
    temp = _glp_mpq_init();
    _glp_mpq_sub(temp, x, y);
    s = _glp_mpq_sgn(temp);
    _glp_mpq_clear(temp);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_sgn(mut x: mpq_t) -> i32 {
    let mut s: i32 = 0;
    s = _glp_mpz_sgn(&mut (*x).p);
    (_glp_mpz_sgn(&mut (*x).q) > 0 as i32
        || {
            glp_assert_(
                b"mpz_sgn(&x->q) > 0\0" as *const u8 as *const i8,
                b"misc/mygmp.c\0" as *const u8 as *const i8,
                1129 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpq_out_str(
    mut _fp: *mut libc::c_void,
    mut base: i32,
    mut x: mpq_t,
) -> i32 {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut nwr: i32 = 0;
    if !(2 as i32 <= base && base <= 36 as i32) {
        (glp_error_(b"misc/mygmp.c\0" as *const u8 as *const i8, 1143 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"mpq_out_str: base = %d; invalid base\n\0" as *const u8 as *const i8,
            base,
        );
    }
    if fp.is_null() {
        fp = stdout;
    }
    nwr = _glp_mpz_out_str(fp as *mut libc::c_void, base, &mut (*x).p);
    if !((*x).q.val == 1 as i32 && ((*x).q.ptr).is_null()) {
        fputc('/' as i32, fp);
        nwr += 1;
        nwr;
        nwr += _glp_mpz_out_str(fp as *mut libc::c_void, base, &mut (*x).q);
    }
    if ferror(fp) != 0 {
        nwr = 0 as i32;
    }
    return nwr;
}