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
    pub type BFX;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_mpq_div(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_mpq_abs(z: mpq_t, x: mpq_t);
    fn _glp_mpq_cmp(x: mpq_t, y: mpq_t) -> i32;
    fn _glp_mpq_sgn(x: mpq_t) -> i32;
    fn _glp_bfx_create_binv() -> *mut BFX;
    fn _glp_bfx_factorize(
        binv: *mut BFX,
        m: i32,
        col: Option<
            unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32, *mut mpq_t) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> i32;
    fn _glp_bfx_ftran(binv: *mut BFX, x: *mut mpq_t, save: i32);
    fn _glp_bfx_btran(binv: *mut BFX, x: *mut mpq_t);
    fn _glp_bfx_update(binv: *mut BFX, j: i32) -> i32;
    fn _glp_mpq_sub(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_get_d(x: mpq_t) -> libc::c_double;
    fn _glp_mpq_add(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_set_si(x: mpq_t, p: i32, q: u32);
    fn _glp_mpq_set(z: mpq_t, x: mpq_t);
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_bfx_delete_binv(binv: *mut BFX);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
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
pub type mpq_t = *mut mpq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSX {
    pub m: i32,
    pub n: i32,
    pub type_0: *mut i32,
    pub lb: *mut mpq_t,
    pub ub: *mut mpq_t,
    pub dir: i32,
    pub coef: *mut mpq_t,
    pub A_ptr: *mut i32,
    pub A_ind: *mut i32,
    pub A_val: *mut mpq_t,
    pub stat: *mut i32,
    pub Q_row: *mut i32,
    pub Q_col: *mut i32,
    pub binv: *mut BFX,
    pub bbar: *mut mpq_t,
    pub pi: *mut mpq_t,
    pub cbar: *mut mpq_t,
    pub p: i32,
    pub rho: *mut mpq_t,
    pub ap: *mut mpq_t,
    pub q: i32,
    pub aq: *mut mpq_t,
    pub q_dir: i32,
    pub p_stat: i32,
    pub delta: mpq_t,
    pub msg_lev: i32,
    pub it_lim: i32,
    pub it_cnt: i32,
    pub tm_lim: libc::c_double,
    pub out_frq: libc::c_double,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_create(
    mut m: i32,
    mut n: i32,
    mut nnz: i32,
) -> *mut SSX {
    let mut ssx: *mut SSX = 0 as *mut SSX;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if m < 1 as i32 {
        (glp_error_(b"draft/glpssx01.c\0" as *const u8 as *const i8, 42 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"ssx_create: m = %d; invalid number of rows\n\0" as *const u8 as *const i8,
            m,
        );
    }
    if n < 1 as i32 {
        (glp_error_(b"draft/glpssx01.c\0" as *const u8 as *const i8, 44 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"ssx_create: n = %d; invalid number of columns\n\0" as *const u8
                as *const i8,
            n,
        );
    }
    if nnz < 0 as i32 {
        (glp_error_(b"draft/glpssx01.c\0" as *const u8 as *const i8, 46 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"ssx_create: nnz = %d; invalid number of non-zero constraint coefficients\n\0"
                as *const u8 as *const i8,
            nnz,
        );
    }
    ssx = glp_alloc(1 as i32, ::core::mem::size_of::<SSX>() as u64 as i32) as *mut SSX;
    (*ssx).m = m;
    (*ssx).n = n;
    (*ssx).type_0 = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*ssx).lb = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<mpq_t>() as u64 as i32,
    ) as *mut mpq_t;
    k = 1 as i32;
    while k <= m + n {
        let ref mut fresh0 = *((*ssx).lb).offset(k as isize);
        *fresh0 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx).ub = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<mpq_t>() as u64 as i32,
    ) as *mut mpq_t;
    k = 1 as i32;
    while k <= m + n {
        let ref mut fresh1 = *((*ssx).ub).offset(k as isize);
        *fresh1 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx).coef = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<mpq_t>() as u64 as i32,
    ) as *mut mpq_t;
    k = 0 as i32;
    while k <= m + n {
        let ref mut fresh2 = *((*ssx).coef).offset(k as isize);
        *fresh2 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx).A_ptr = glp_alloc(
        1 as i32 + n + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    *((*ssx).A_ptr).offset((n + 1 as i32) as isize) = nnz + 1 as i32;
    (*ssx).A_ind = glp_alloc(1 as i32 + nnz, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*ssx).A_val = glp_alloc(
        1 as i32 + nnz,
        ::core::mem::size_of::<mpq_t>() as u64 as i32,
    ) as *mut mpq_t;
    k = 1 as i32;
    while k <= nnz {
        let ref mut fresh3 = *((*ssx).A_val).offset(k as isize);
        *fresh3 = _glp_mpq_init();
        k += 1;
        k;
    }
    (*ssx).stat = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*ssx).Q_row = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*ssx).Q_col = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*ssx).binv = _glp_bfx_create_binv();
    (*ssx).bbar = glp_alloc(1 as i32 + m, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    i = 0 as i32;
    while i <= m {
        let ref mut fresh4 = *((*ssx).bbar).offset(i as isize);
        *fresh4 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx).pi = glp_alloc(1 as i32 + m, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    i = 1 as i32;
    while i <= m {
        let ref mut fresh5 = *((*ssx).pi).offset(i as isize);
        *fresh5 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx).cbar = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    j = 1 as i32;
    while j <= n {
        let ref mut fresh6 = *((*ssx).cbar).offset(j as isize);
        *fresh6 = _glp_mpq_init();
        j += 1;
        j;
    }
    (*ssx).rho = glp_alloc(1 as i32 + m, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    i = 1 as i32;
    while i <= m {
        let ref mut fresh7 = *((*ssx).rho).offset(i as isize);
        *fresh7 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx).ap = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    j = 1 as i32;
    while j <= n {
        let ref mut fresh8 = *((*ssx).ap).offset(j as isize);
        *fresh8 = _glp_mpq_init();
        j += 1;
        j;
    }
    (*ssx).aq = glp_alloc(1 as i32 + m, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    i = 1 as i32;
    while i <= m {
        let ref mut fresh9 = *((*ssx).aq).offset(i as isize);
        *fresh9 = _glp_mpq_init();
        i += 1;
        i;
    }
    (*ssx).delta = _glp_mpq_init();
    return ssx;
}
unsafe extern "C" fn basis_col(
    mut info: *mut libc::c_void,
    mut j: i32,
    mut ind: *mut i32,
    mut val: *mut mpq_t,
) -> i32 {
    let mut ssx: *mut SSX = info as *mut SSX;
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut A_ptr: *mut i32 = (*ssx).A_ptr;
    let mut A_ind: *mut i32 = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut ptr: i32 = 0;
    (1 as i32 <= j && j <= m
        || {
            glp_assert_(
                b"1 <= j && j <= m\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                101 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *Q_col.offset(j as isize);
    (1 as i32 <= k && k <= m + n
        || {
            glp_assert_(
                b"1 <= k && k <= m+n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                103 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if k <= m {
        len = 1 as i32;
        *ind.offset(1 as i32 as isize) = k;
        _glp_mpq_set_si(*val.offset(1 as i32 as isize), 1 as i32, 1 as i32 as u32);
    } else {
        len = 0 as i32;
        ptr = *A_ptr.offset((k - m) as isize);
        while ptr < *A_ptr.offset((k - m + 1 as i32) as isize) {
            len += 1;
            len;
            *ind.offset(len as isize) = *A_ind.offset(ptr as isize);
            _glp_mpq_neg(*val.offset(len as isize), *A_val.offset(ptr as isize));
            ptr += 1;
            ptr;
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_factorize(mut ssx: *mut SSX) -> i32 {
    let mut ret: i32 = 0;
    ret = _glp_bfx_factorize(
        (*ssx).binv,
        (*ssx).m,
        Some(
            basis_col
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    i32,
                    *mut i32,
                    *mut mpq_t,
                ) -> i32,
        ),
        ssx as *mut libc::c_void,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_get_xNj(mut ssx: *mut SSX, mut j: i32, mut x: mpq_t) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut lb: *mut mpq_t = (*ssx).lb;
    let mut ub: *mut mpq_t = (*ssx).ub;
    let mut stat: *mut i32 = (*ssx).stat;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut k: i32 = 0;
    (1 as i32 <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                149 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *Q_col.offset((m + j) as isize);
    (1 as i32 <= k && k <= m + n
        || {
            glp_assert_(
                b"1 <= k && k <= m+n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                151 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match *stat.offset(k as isize) {
        1 => {
            _glp_mpq_set(x, *lb.offset(k as isize));
        }
        2 => {
            _glp_mpq_set(x, *ub.offset(k as isize));
        }
        3 => {
            _glp_mpq_set_si(x, 0 as i32, 1 as i32 as u32);
        }
        4 => {
            _glp_mpq_set(x, *lb.offset(k as isize));
        }
        _ => {
            (stat != stat
                || {
                    glp_assert_(
                        b"stat != stat\0" as *const u8 as *const i8,
                        b"draft/glpssx01.c\0" as *const u8 as *const i8,
                        166 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_eval_bbar(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut coef: *mut mpq_t = (*ssx).coef;
    let mut A_ptr: *mut i32 = (*ssx).A_ptr;
    let mut A_ind: *mut i32 = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut x: mpq_t = 0 as *mut mpq;
    let mut temp: mpq_t = 0 as *mut mpq;
    x = _glp_mpq_init();
    temp = _glp_mpq_init();
    i = 1 as i32;
    while i <= m {
        _glp_mpq_set_si(*bbar.offset(i as isize), 0 as i32, 1 as i32 as u32);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        _glp_ssx_get_xNj(ssx, j, x);
        if !(_glp_mpq_sgn(x) == 0 as i32) {
            k = *Q_col.offset((m + j) as isize);
            if k <= m {
                _glp_mpq_sub(*bbar.offset(k as isize), *bbar.offset(k as isize), x);
            } else {
                ptr = *A_ptr.offset((k - m) as isize);
                while ptr < *A_ptr.offset((k - m + 1 as i32) as isize) {
                    _glp_mpq_mul(temp, *A_val.offset(ptr as isize), x);
                    _glp_mpq_add(
                        *bbar.offset(*A_ind.offset(ptr as isize) as isize),
                        *bbar.offset(*A_ind.offset(ptr as isize) as isize),
                        temp,
                    );
                    ptr += 1;
                    ptr;
                }
            }
        }
        j += 1;
        j;
    }
    _glp_bfx_ftran((*ssx).binv, bbar, 0 as i32);
    _glp_mpq_set(*bbar.offset(0 as i32 as isize), *coef.offset(0 as i32 as isize));
    i = 1 as i32;
    while i <= m {
        k = *Q_col.offset(i as isize);
        if !(_glp_mpq_sgn(*coef.offset(k as isize)) == 0 as i32) {
            _glp_mpq_mul(temp, *coef.offset(k as isize), *bbar.offset(i as isize));
            _glp_mpq_add(
                *bbar.offset(0 as i32 as isize),
                *bbar.offset(0 as i32 as isize),
                temp,
            );
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        k = *Q_col.offset((m + j) as isize);
        if !(_glp_mpq_sgn(*coef.offset(k as isize)) == 0 as i32) {
            _glp_ssx_get_xNj(ssx, j, x);
            _glp_mpq_mul(temp, *coef.offset(k as isize), x);
            _glp_mpq_add(
                *bbar.offset(0 as i32 as isize),
                *bbar.offset(0 as i32 as isize),
                temp,
            );
        }
        j += 1;
        j;
    }
    _glp_mpq_clear(x);
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_eval_pi(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut coef: *mut mpq_t = (*ssx).coef;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut pi: *mut mpq_t = (*ssx).pi;
    let mut i: i32 = 0;
    i = 1 as i32;
    while i <= m {
        _glp_mpq_set(
            *pi.offset(i as isize),
            *coef.offset(*Q_col.offset(i as isize) as isize),
        );
        i += 1;
        i;
    }
    _glp_bfx_btran((*ssx).binv, pi);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_eval_dj(mut ssx: *mut SSX, mut j: i32, mut dj: mpq_t) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut coef: *mut mpq_t = (*ssx).coef;
    let mut A_ptr: *mut i32 = (*ssx).A_ptr;
    let mut A_ind: *mut i32 = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut pi: *mut mpq_t = (*ssx).pi;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    (1 as i32 <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                290 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *Q_col.offset((m + j) as isize);
    (1 as i32 <= k && k <= m + n
        || {
            glp_assert_(
                b"1 <= k && k <= m+n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                292 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if k <= m {
        _glp_mpq_sub(dj, *coef.offset(k as isize), *pi.offset(k as isize));
    } else {
        _glp_mpq_set(dj, *coef.offset(k as isize));
        ptr = *A_ptr.offset((k - m) as isize);
        end = *A_ptr.offset((k - m + 1 as i32) as isize);
        while ptr < end {
            _glp_mpq_mul(
                temp,
                *A_val.offset(ptr as isize),
                *pi.offset(*A_ind.offset(ptr as isize) as isize),
            );
            _glp_mpq_add(dj, dj, temp);
            ptr += 1;
            ptr;
        }
    }
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_eval_cbar(mut ssx: *mut SSX) {
    let mut n: i32 = (*ssx).n;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut j: i32 = 0;
    j = 1 as i32;
    while j <= n {
        _glp_ssx_eval_dj(ssx, j, *cbar.offset(j as isize));
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_eval_rho(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut p: i32 = (*ssx).p;
    let mut rho: *mut mpq_t = (*ssx).rho;
    let mut i: i32 = 0;
    (1 as i32 <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                344 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= m {
        _glp_mpq_set_si(*rho.offset(i as isize), 0 as i32, 1 as i32 as u32);
        i += 1;
        i;
    }
    _glp_mpq_set_si(*rho.offset(p as isize), 1 as i32, 1 as i32 as u32);
    _glp_bfx_btran((*ssx).binv, rho);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_eval_row(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut A_ptr: *mut i32 = (*ssx).A_ptr;
    let mut A_ind: *mut i32 = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut rho: *mut mpq_t = (*ssx).rho;
    let mut ap: *mut mpq_t = (*ssx).ap;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    j = 1 as i32;
    while j <= n {
        k = *Q_col.offset((m + j) as isize);
        if k <= m {
            _glp_mpq_neg(*ap.offset(j as isize), *rho.offset(k as isize));
        } else {
            _glp_mpq_set_si(*ap.offset(j as isize), 0 as i32, 1 as i32 as u32);
            ptr = *A_ptr.offset((k - m) as isize);
            while ptr < *A_ptr.offset((k - m + 1 as i32) as isize) {
                _glp_mpq_mul(
                    temp,
                    *A_val.offset(ptr as isize),
                    *rho.offset(*A_ind.offset(ptr as isize) as isize),
                );
                _glp_mpq_add(*ap.offset(j as isize), *ap.offset(j as isize), temp);
                ptr += 1;
                ptr;
            }
        }
        j += 1;
        j;
    }
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_eval_col(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut A_ptr: *mut i32 = (*ssx).A_ptr;
    let mut A_ind: *mut i32 = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut q: i32 = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                415 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= m {
        _glp_mpq_set_si(*aq.offset(i as isize), 0 as i32, 1 as i32 as u32);
        i += 1;
        i;
    }
    k = *Q_col.offset((m + q) as isize);
    if k <= m {
        _glp_mpq_set_si(*aq.offset(k as isize), 1 as i32, 1 as i32 as u32);
    } else {
        ptr = *A_ptr.offset((k - m) as isize);
        while ptr < *A_ptr.offset((k - m + 1 as i32) as isize) {
            _glp_mpq_neg(
                *aq.offset(*A_ind.offset(ptr as isize) as isize),
                *A_val.offset(ptr as isize),
            );
            ptr += 1;
            ptr;
        }
    }
    _glp_bfx_ftran((*ssx).binv, aq, 1 as i32);
    i = 1 as i32;
    while i <= m {
        _glp_mpq_neg(*aq.offset(i as isize), *aq.offset(i as isize));
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_chuzc(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut dir: i32 = if (*ssx).dir == 0 as i32 { 1 as i32 } else { -(1 as i32) };
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut stat: *mut i32 = (*ssx).stat;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut s: i32 = 0;
    let mut q: i32 = 0;
    let mut q_dir: i32 = 0;
    let mut best: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    q = 0 as i32;
    q_dir = 0 as i32;
    best = 0.0f64;
    j = 1 as i32;
    while j <= n {
        k = *Q_col.offset((m + j) as isize);
        s = dir * _glp_mpq_sgn(*cbar.offset(j as isize));
        if (*stat.offset(k as isize) == 3 as i32 || *stat.offset(k as isize) == 1 as i32)
            && s < 0 as i32
            || (*stat.offset(k as isize) == 3 as i32
                || *stat.offset(k as isize) == 2 as i32) && s > 0 as i32
        {
            temp = fabs(_glp_mpq_get_d(*cbar.offset(j as isize)));
            (temp != 0.0f64
                || {
                    glp_assert_(
                        b"temp != 0.0\0" as *const u8 as *const i8,
                        b"draft/glpssx01.c\0" as *const u8 as *const i8,
                        474 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if q == 0 as i32 || best < temp {
                q = j;
                q_dir = -s;
                best = temp;
            }
        }
        j += 1;
        j;
    }
    (*ssx).q = q;
    (*ssx).q_dir = q_dir;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_chuzr(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut type_0: *mut i32 = (*ssx).type_0;
    let mut lb: *mut mpq_t = (*ssx).lb;
    let mut ub: *mut mpq_t = (*ssx).ub;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut q: i32 = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut q_dir: i32 = (*ssx).q_dir;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut s: i32 = 0;
    let mut t: i32 = 0;
    let mut p: i32 = 0;
    let mut p_stat: i32 = 0;
    let mut teta: mpq_t = 0 as *mut mpq;
    let mut temp: mpq_t = 0 as *mut mpq;
    teta = _glp_mpq_init();
    temp = _glp_mpq_init();
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                517 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (q_dir == 1 as i32 || q_dir == -(1 as i32)
        || {
            glp_assert_(
                b"q_dir == +1 || q_dir == -1\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                518 as i32,
            );
            1 as i32 != 0
        }) as i32;
    p = 0 as i32;
    p_stat = 0 as i32;
    i = 1 as i32;
    while i <= m {
        s = q_dir * _glp_mpq_sgn(*aq.offset(i as isize));
        if s < 0 as i32 {
            k = *Q_col.offset(i as isize);
            t = *type_0.offset(k as isize);
            if t == 1 as i32 || t == 3 as i32 || t == 4 as i32 {
                _glp_mpq_sub(temp, *bbar.offset(i as isize), *lb.offset(k as isize));
                _glp_mpq_div(temp, temp, *aq.offset(i as isize));
                _glp_mpq_abs(temp, temp);
                if p == 0 as i32 || _glp_mpq_cmp(teta, temp) > 0 as i32 {
                    p = i;
                    p_stat = if t == 4 as i32 { 4 as i32 } else { 1 as i32 };
                    _glp_mpq_set(teta, temp);
                }
            }
        } else if s > 0 as i32 {
            k = *Q_col.offset(i as isize);
            t = *type_0.offset(k as isize);
            if t == 2 as i32 || t == 3 as i32 || t == 4 as i32 {
                _glp_mpq_sub(temp, *bbar.offset(i as isize), *ub.offset(k as isize));
                _glp_mpq_div(temp, temp, *aq.offset(i as isize));
                _glp_mpq_abs(temp, temp);
                if p == 0 as i32 || _glp_mpq_cmp(teta, temp) > 0 as i32 {
                    p = i;
                    p_stat = if t == 4 as i32 { 4 as i32 } else { 2 as i32 };
                    _glp_mpq_set(teta, temp);
                }
            }
        }
        if p != 0 as i32 && _glp_mpq_sgn(teta) == 0 as i32 {
            break;
        }
        i += 1;
        i;
    }
    k = *Q_col.offset((m + q) as isize);
    if *type_0.offset(k as isize) == 3 as i32 {
        _glp_mpq_sub(temp, *ub.offset(k as isize), *lb.offset(k as isize));
        if p == 0 as i32 || _glp_mpq_cmp(teta, temp) > 0 as i32 {
            p = -(1 as i32);
            p_stat = -(1 as i32);
            _glp_mpq_set(teta, temp);
        }
    }
    (*ssx).p = p;
    (*ssx).p_stat = p_stat;
    if p != 0 as i32 {
        (_glp_mpq_sgn(teta) >= 0 as i32
            || {
                glp_assert_(
                    b"mpq_sgn(teta) >= 0\0" as *const u8 as *const i8,
                    b"draft/glpssx01.c\0" as *const u8 as *const i8,
                    576 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if q_dir > 0 as i32 {
            _glp_mpq_set((*ssx).delta, teta);
        } else {
            _glp_mpq_neg((*ssx).delta, teta);
        }
    }
    _glp_mpq_clear(teta);
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_update_bbar(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut p: i32 = (*ssx).p;
    let mut q: i32 = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut i: i32 = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                624 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !(p < 0 as i32) {
        (1 as i32 <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const i8,
                    b"draft/glpssx01.c\0" as *const u8 as *const i8,
                    632 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_ssx_get_xNj(ssx, q, temp);
        _glp_mpq_add(*bbar.offset(p as isize), temp, (*ssx).delta);
    }
    i = 1 as i32;
    while i <= m {
        if !(i == p) {
            if !(_glp_mpq_sgn(*aq.offset(i as isize)) == 0 as i32) {
                _glp_mpq_mul(temp, *aq.offset(i as isize), (*ssx).delta);
                _glp_mpq_add(*bbar.offset(i as isize), *bbar.offset(i as isize), temp);
            }
        }
        i += 1;
        i;
    }
    _glp_mpq_mul(temp, *cbar.offset(q as isize), (*ssx).delta);
    _glp_mpq_add(*bbar.offset(0 as i32 as isize), *bbar.offset(0 as i32 as isize), temp);
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_update_pi(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut pi: *mut mpq_t = (*ssx).pi;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut p: i32 = (*ssx).p;
    let mut q: i32 = (*ssx).q;
    let mut aq: *mut mpq_t = (*ssx).aq;
    let mut rho: *mut mpq_t = (*ssx).rho;
    let mut i: i32 = 0;
    let mut new_dq: mpq_t = 0 as *mut mpq;
    let mut temp: mpq_t = 0 as *mut mpq;
    new_dq = _glp_mpq_init();
    temp = _glp_mpq_init();
    (1 as i32 <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                673 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                674 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpq_div(new_dq, *cbar.offset(q as isize), *aq.offset(p as isize));
    i = 1 as i32;
    while i <= m {
        if !(_glp_mpq_sgn(*rho.offset(i as isize)) == 0 as i32) {
            _glp_mpq_mul(temp, new_dq, *rho.offset(i as isize));
            _glp_mpq_sub(*pi.offset(i as isize), *pi.offset(i as isize), temp);
        }
        i += 1;
        i;
    }
    _glp_mpq_clear(new_dq);
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_update_cbar(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut p: i32 = (*ssx).p;
    let mut q: i32 = (*ssx).q;
    let mut ap: *mut mpq_t = (*ssx).ap;
    let mut j: i32 = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    (1 as i32 <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                704 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"draft/glpssx01.c\0" as *const u8 as *const i8,
                705 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_mpq_div(
        *cbar.offset(q as isize),
        *cbar.offset(q as isize),
        *ap.offset(q as isize),
    );
    j = 1 as i32;
    while j <= n {
        if !(j == q) {
            if !(_glp_mpq_sgn(*ap.offset(j as isize)) == 0 as i32) {
                _glp_mpq_mul(temp, *ap.offset(j as isize), *cbar.offset(q as isize));
                _glp_mpq_sub(*cbar.offset(j as isize), *cbar.offset(j as isize), temp);
            }
        }
        j += 1;
        j;
    }
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_change_basis(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut type_0: *mut i32 = (*ssx).type_0;
    let mut stat: *mut i32 = (*ssx).stat;
    let mut Q_row: *mut i32 = (*ssx).Q_row;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut p: i32 = (*ssx).p;
    let mut q: i32 = (*ssx).q;
    let mut p_stat: i32 = (*ssx).p_stat;
    let mut k: i32 = 0;
    let mut kp: i32 = 0;
    let mut kq: i32 = 0;
    if p < 0 as i32 {
        (1 as i32 <= q && q <= n
            || {
                glp_assert_(
                    b"1 <= q && q <= n\0" as *const u8 as *const i8,
                    b"draft/glpssx01.c\0" as *const u8 as *const i8,
                    740 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = *Q_col.offset((m + q) as isize);
        (*type_0.offset(k as isize) == 3 as i32
            || {
                glp_assert_(
                    b"type[k] == SSX_DB\0" as *const u8 as *const i8,
                    b"draft/glpssx01.c\0" as *const u8 as *const i8,
                    742 as i32,
                );
                1 as i32 != 0
            }) as i32;
        match *stat.offset(k as isize) {
            1 => {
                *stat.offset(k as isize) = 2 as i32;
            }
            2 => {
                *stat.offset(k as isize) = 1 as i32;
            }
            _ => {
                (stat != stat
                    || {
                        glp_assert_(
                            b"stat != stat\0" as *const u8 as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            751 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
    } else {
        (1 as i32 <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const i8,
                    b"draft/glpssx01.c\0" as *const u8 as *const i8,
                    756 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (1 as i32 <= q && q <= n
            || {
                glp_assert_(
                    b"1 <= q && q <= n\0" as *const u8 as *const i8,
                    b"draft/glpssx01.c\0" as *const u8 as *const i8,
                    757 as i32,
                );
                1 as i32 != 0
            }) as i32;
        kp = *Q_col.offset(p as isize);
        kq = *Q_col.offset((m + q) as isize);
        match *type_0.offset(kp as isize) {
            0 => {
                (p_stat == 3 as i32
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NF\0" as *const u8 as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            763 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            1 => {
                (p_stat == 1 as i32
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NL\0" as *const u8 as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            766 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            2 => {
                (p_stat == 2 as i32
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NU\0" as *const u8 as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            769 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            3 => {
                (p_stat == 1 as i32 || p_stat == 2 as i32
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NL || p_stat == SSX_NU\0" as *const u8
                                as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            772 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            4 => {
                (p_stat == 4 as i32
                    || {
                        glp_assert_(
                            b"p_stat == SSX_NS\0" as *const u8 as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            775 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            778 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        *stat.offset(kp as isize) = p_stat as i8 as i32;
        *stat.offset(kq as isize) = 0 as i32;
        *Q_row.offset(kp as isize) = m + q;
        *Q_row.offset(kq as isize) = p;
        *Q_col.offset(p as isize) = kq;
        *Q_col.offset((m + q) as isize) = kp;
        if _glp_bfx_update((*ssx).binv, p) != 0 {
            if _glp_ssx_factorize(ssx) != 0 {
                (0 as i32 != 0
                    || {
                        glp_assert_(
                            b"(\"Internal error: basis matrix is singular\", 0)\0"
                                as *const u8 as *const i8,
                            b"draft/glpssx01.c\0" as *const u8 as *const i8,
                            787 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_delete(mut ssx: *mut SSX) {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut nnz: i32 = *((*ssx).A_ptr).offset((n + 1 as i32) as isize) - 1 as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    glp_free((*ssx).type_0 as *mut libc::c_void);
    k = 1 as i32;
    while k <= m + n {
        _glp_mpq_clear(*((*ssx).lb).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).lb as *mut libc::c_void);
    k = 1 as i32;
    while k <= m + n {
        _glp_mpq_clear(*((*ssx).ub).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).ub as *mut libc::c_void);
    k = 0 as i32;
    while k <= m + n {
        _glp_mpq_clear(*((*ssx).coef).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).coef as *mut libc::c_void);
    glp_free((*ssx).A_ptr as *mut libc::c_void);
    glp_free((*ssx).A_ind as *mut libc::c_void);
    k = 1 as i32;
    while k <= nnz {
        _glp_mpq_clear(*((*ssx).A_val).offset(k as isize));
        k += 1;
        k;
    }
    glp_free((*ssx).A_val as *mut libc::c_void);
    glp_free((*ssx).stat as *mut libc::c_void);
    glp_free((*ssx).Q_row as *mut libc::c_void);
    glp_free((*ssx).Q_col as *mut libc::c_void);
    _glp_bfx_delete_binv((*ssx).binv);
    i = 0 as i32;
    while i <= m {
        _glp_mpq_clear(*((*ssx).bbar).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).bbar as *mut libc::c_void);
    i = 1 as i32;
    while i <= m {
        _glp_mpq_clear(*((*ssx).pi).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).pi as *mut libc::c_void);
    j = 1 as i32;
    while j <= n {
        _glp_mpq_clear(*((*ssx).cbar).offset(j as isize));
        j += 1;
        j;
    }
    glp_free((*ssx).cbar as *mut libc::c_void);
    i = 1 as i32;
    while i <= m {
        _glp_mpq_clear(*((*ssx).rho).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).rho as *mut libc::c_void);
    j = 1 as i32;
    while j <= n {
        _glp_mpq_clear(*((*ssx).ap).offset(j as isize));
        j += 1;
        j;
    }
    glp_free((*ssx).ap as *mut libc::c_void);
    i = 1 as i32;
    while i <= m {
        _glp_mpq_clear(*((*ssx).aq).offset(i as isize));
        i += 1;
        i;
    }
    glp_free((*ssx).aq as *mut libc::c_void);
    _glp_mpq_clear((*ssx).delta);
    glp_free(ssx as *mut libc::c_void);
}