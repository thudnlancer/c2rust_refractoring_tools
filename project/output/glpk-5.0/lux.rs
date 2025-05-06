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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_set(z: mpq_t, x: mpq_t);
    fn _glp_mpq_set_si(x: mpq_t, p: i32, q: u32);
    fn _glp_mpq_sub(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_div(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_mpq_sgn(x: mpq_t) -> i32;
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
pub struct LUX {
    pub n: i32,
    pub pool: *mut DMP,
    pub F_row: *mut *mut LUXELM,
    pub F_col: *mut *mut LUXELM,
    pub V_piv: *mut mpq_t,
    pub V_row: *mut *mut LUXELM,
    pub V_col: *mut *mut LUXELM,
    pub P_row: *mut i32,
    pub P_col: *mut i32,
    pub Q_row: *mut i32,
    pub Q_col: *mut i32,
    pub rank: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUXELM {
    pub i: i32,
    pub j: i32,
    pub val: mpq_t,
    pub r_prev: *mut LUXELM,
    pub r_next: *mut LUXELM,
    pub c_prev: *mut LUXELM,
    pub c_next: *mut LUXELM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUXWKA {
    pub R_len: *mut i32,
    pub R_head: *mut i32,
    pub R_prev: *mut i32,
    pub R_next: *mut i32,
    pub C_len: *mut i32,
    pub C_head: *mut i32,
    pub C_prev: *mut i32,
    pub C_next: *mut i32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lux_create(mut n: i32) -> *mut LUX {
    let mut lux: *mut LUX = 0 as *mut LUX;
    let mut k: i32 = 0;
    if n < 1 as i32 {
        (glp_error_(b"draft/lux.c\0" as *const u8 as *const i8, 51 as i32))
            .expect(
                "non-null function pointer",
            )(b"lux_create: n = %d; invalid parameter\n\0" as *const u8 as *const i8, n);
    }
    lux = glp_alloc(1 as i32, ::core::mem::size_of::<LUX>() as u64 as i32) as *mut LUX;
    (*lux).n = n;
    (*lux).pool = _glp_dmp_create_pool();
    (*lux).F_row = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<*mut LUXELM>() as u64 as i32,
    ) as *mut *mut LUXELM;
    (*lux).F_col = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<*mut LUXELM>() as u64 as i32,
    ) as *mut *mut LUXELM;
    (*lux).V_piv = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    (*lux).V_row = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<*mut LUXELM>() as u64 as i32,
    ) as *mut *mut LUXELM;
    (*lux).V_col = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<*mut LUXELM>() as u64 as i32,
    ) as *mut *mut LUXELM;
    (*lux).P_row = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*lux).P_col = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*lux).Q_row = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*lux).Q_col = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    k = 1 as i32;
    while k <= n {
        let ref mut fresh0 = *((*lux).F_col).offset(k as isize);
        *fresh0 = 0 as *mut LUXELM;
        let ref mut fresh1 = *((*lux).F_row).offset(k as isize);
        *fresh1 = *fresh0;
        let ref mut fresh2 = *((*lux).V_piv).offset(k as isize);
        *fresh2 = _glp_mpq_init();
        _glp_mpq_set_si(*((*lux).V_piv).offset(k as isize), 1 as i32, 1 as i32 as u32);
        let ref mut fresh3 = *((*lux).V_col).offset(k as isize);
        *fresh3 = 0 as *mut LUXELM;
        let ref mut fresh4 = *((*lux).V_row).offset(k as isize);
        *fresh4 = *fresh3;
        let ref mut fresh5 = *((*lux).P_col).offset(k as isize);
        *fresh5 = k;
        *((*lux).P_row).offset(k as isize) = *fresh5;
        let ref mut fresh6 = *((*lux).Q_col).offset(k as isize);
        *fresh6 = k;
        *((*lux).Q_row).offset(k as isize) = *fresh6;
        k += 1;
        k;
    }
    (*lux).rank = n;
    return lux;
}
unsafe extern "C" fn initialize(
    mut lux: *mut LUX,
    mut col: Option<
        unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32, *mut mpq_t) -> i32,
    >,
    mut info: *mut libc::c_void,
    mut wka: *mut LUXWKA,
) {
    let mut n: i32 = (*lux).n;
    let mut pool: *mut DMP = (*lux).pool;
    let mut F_row: *mut *mut LUXELM = (*lux).F_row;
    let mut F_col: *mut *mut LUXELM = (*lux).F_col;
    let mut V_piv: *mut mpq_t = (*lux).V_piv;
    let mut V_row: *mut *mut LUXELM = (*lux).V_row;
    let mut V_col: *mut *mut LUXELM = (*lux).V_col;
    let mut P_row: *mut i32 = (*lux).P_row;
    let mut P_col: *mut i32 = (*lux).P_col;
    let mut Q_row: *mut i32 = (*lux).Q_row;
    let mut Q_col: *mut i32 = (*lux).Q_col;
    let mut R_len: *mut i32 = (*wka).R_len;
    let mut R_head: *mut i32 = (*wka).R_head;
    let mut R_prev: *mut i32 = (*wka).R_prev;
    let mut R_next: *mut i32 = (*wka).R_next;
    let mut C_len: *mut i32 = (*wka).C_len;
    let mut C_head: *mut i32 = (*wka).C_head;
    let mut C_prev: *mut i32 = (*wka).C_prev;
    let mut C_next: *mut i32 = (*wka).C_next;
    let mut fij: *mut LUXELM = 0 as *mut LUXELM;
    let mut vij: *mut LUXELM = 0 as *mut LUXELM;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut val: *mut mpq_t = 0 as *mut mpq_t;
    i = 1 as i32;
    while i <= n {
        while !(*F_row.offset(i as isize)).is_null() {
            fij = *F_row.offset(i as isize);
            let ref mut fresh7 = *F_row.offset(i as isize);
            *fresh7 = (*fij).r_next;
            _glp_mpq_clear((*fij).val);
            _glp_dmp_free_atom(
                pool,
                fij as *mut libc::c_void,
                ::core::mem::size_of::<LUXELM>() as u64 as i32,
            );
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        let ref mut fresh8 = *F_col.offset(j as isize);
        *fresh8 = 0 as *mut LUXELM;
        j += 1;
        j;
    }
    k = 1 as i32;
    while k <= n {
        _glp_mpq_set_si(*V_piv.offset(k as isize), 0 as i32, 1 as i32 as u32);
        k += 1;
        k;
    }
    i = 1 as i32;
    while i <= n {
        while !(*V_row.offset(i as isize)).is_null() {
            vij = *V_row.offset(i as isize);
            let ref mut fresh9 = *V_row.offset(i as isize);
            *fresh9 = (*vij).r_next;
            _glp_mpq_clear((*vij).val);
            _glp_dmp_free_atom(
                pool,
                vij as *mut libc::c_void,
                ::core::mem::size_of::<LUXELM>() as u64 as i32,
            );
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        let ref mut fresh10 = *V_col.offset(j as isize);
        *fresh10 = 0 as *mut LUXELM;
        j += 1;
        j;
    }
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    k = 1 as i32;
    while k <= n {
        let ref mut fresh11 = *val.offset(k as isize);
        *fresh11 = _glp_mpq_init();
        k += 1;
        k;
    }
    j = 1 as i32;
    while j <= n {
        len = col.expect("non-null function pointer")(info, j, ind, val);
        if !(0 as i32 <= len && len <= n) {
            (glp_error_(b"draft/lux.c\0" as *const u8 as *const i8, 135 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"lux_decomp: j = %d: len = %d; invalid column length\n\0" as *const u8
                    as *const i8,
                j,
                len,
            );
        }
        k = 1 as i32;
        while k <= len {
            i = *ind.offset(k as isize);
            if !(1 as i32 <= i && i <= n) {
                (glp_error_(b"draft/lux.c\0" as *const u8 as *const i8, 142 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"lux_decomp: j = %d: i = %d; row index out of range\n\0"
                        as *const u8 as *const i8,
                    j,
                    i,
                );
            }
            if !(*V_row.offset(i as isize)).is_null()
                && (**V_row.offset(i as isize)).j == j
            {
                (glp_error_(b"draft/lux.c\0" as *const u8 as *const i8, 146 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"lux_decomp: j = %d: i = %d; duplicate row indices not allowed\n\0"
                        as *const u8 as *const i8,
                    j,
                    i,
                );
            }
            if _glp_mpq_sgn(*val.offset(k as isize)) == 0 as i32 {
                (glp_error_(b"draft/lux.c\0" as *const u8 as *const i8, 150 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"lux_decomp: j = %d: i = %d; zero elements not allowed\n\0"
                        as *const u8 as *const i8,
                    j,
                    i,
                );
            }
            vij = _glp_dmp_get_atom(pool, ::core::mem::size_of::<LUXELM>() as u64 as i32)
                as *mut LUXELM;
            (*vij).i = i;
            (*vij).j = j;
            (*vij).val = _glp_mpq_init();
            _glp_mpq_set((*vij).val, *val.offset(k as isize));
            (*vij).r_prev = 0 as *mut LUXELM;
            (*vij).r_next = *V_row.offset(i as isize);
            (*vij).c_prev = 0 as *mut LUXELM;
            (*vij).c_next = *V_col.offset(j as isize);
            if !((*vij).r_next).is_null() {
                (*(*vij).r_next).r_prev = vij;
            }
            if !((*vij).c_next).is_null() {
                (*(*vij).c_next).c_prev = vij;
            }
            let ref mut fresh12 = *V_col.offset(j as isize);
            *fresh12 = vij;
            let ref mut fresh13 = *V_row.offset(i as isize);
            *fresh13 = *fresh12;
            k += 1;
            k;
        }
        j += 1;
        j;
    }
    glp_free(ind as *mut libc::c_void);
    k = 1 as i32;
    while k <= n {
        _glp_mpq_clear(*val.offset(k as isize));
        k += 1;
        k;
    }
    glp_free(val as *mut libc::c_void);
    k = 1 as i32;
    while k <= n {
        let ref mut fresh14 = *Q_col.offset(k as isize);
        *fresh14 = k;
        let ref mut fresh15 = *Q_row.offset(k as isize);
        *fresh15 = *fresh14;
        let ref mut fresh16 = *P_col.offset(k as isize);
        *fresh16 = *fresh15;
        *P_row.offset(k as isize) = *fresh16;
        k += 1;
        k;
    }
    (*lux).rank = -(1 as i32);
    i = 1 as i32;
    while i <= n {
        len = 0 as i32;
        vij = *V_row.offset(i as isize);
        while !vij.is_null() {
            len += 1;
            len;
            vij = (*vij).r_next;
        }
        *R_len.offset(i as isize) = len;
        i += 1;
        i;
    }
    len = 0 as i32;
    while len <= n {
        *R_head.offset(len as isize) = 0 as i32;
        len += 1;
        len;
    }
    i = 1 as i32;
    while i <= n {
        len = *R_len.offset(i as isize);
        *R_prev.offset(i as isize) = 0 as i32;
        *R_next.offset(i as isize) = *R_head.offset(len as isize);
        if *R_next.offset(i as isize) != 0 as i32 {
            *R_prev.offset(*R_next.offset(i as isize) as isize) = i;
        }
        *R_head.offset(len as isize) = i;
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        len = 0 as i32;
        vij = *V_col.offset(j as isize);
        while !vij.is_null() {
            len += 1;
            len;
            vij = (*vij).c_next;
        }
        *C_len.offset(j as isize) = len;
        j += 1;
        j;
    }
    len = 0 as i32;
    while len <= n {
        *C_head.offset(len as isize) = 0 as i32;
        len += 1;
        len;
    }
    j = 1 as i32;
    while j <= n {
        len = *C_len.offset(j as isize);
        *C_prev.offset(j as isize) = 0 as i32;
        *C_next.offset(j as isize) = *C_head.offset(len as isize);
        if *C_next.offset(j as isize) != 0 as i32 {
            *C_prev.offset(*C_next.offset(j as isize) as isize) = j;
        }
        *C_head.offset(len as isize) = j;
        j += 1;
        j;
    }
}
unsafe extern "C" fn find_pivot(mut lux: *mut LUX, mut wka: *mut LUXWKA) -> *mut LUXELM {
    let mut n: i32 = (*lux).n;
    let mut V_row: *mut *mut LUXELM = (*lux).V_row;
    let mut V_col: *mut *mut LUXELM = (*lux).V_col;
    let mut R_len: *mut i32 = (*wka).R_len;
    let mut R_head: *mut i32 = (*wka).R_head;
    let mut R_next: *mut i32 = (*wka).R_next;
    let mut C_len: *mut i32 = (*wka).C_len;
    let mut C_head: *mut i32 = (*wka).C_head;
    let mut C_next: *mut i32 = (*wka).C_next;
    let mut piv: *mut LUXELM = 0 as *mut LUXELM;
    let mut some: *mut LUXELM = 0 as *mut LUXELM;
    let mut vij: *mut LUXELM = 0 as *mut LUXELM;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut min_len: i32 = 0;
    let mut ncand: i32 = 0;
    let mut piv_lim: i32 = 5 as i32;
    let mut best: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    piv = 0 as *mut LUXELM;
    best = 1.7976931348623157e+308f64;
    ncand = 0 as i32;
    j = *C_head.offset(1 as i32 as isize);
    if j != 0 as i32 {
        (*C_len.offset(j as isize) == 1 as i32
            || {
                glp_assert_(
                    b"C_len[j] == 1\0" as *const u8 as *const i8,
                    b"draft/lux.c\0" as *const u8 as *const i8,
                    278 as i32,
                );
                1 as i32 != 0
            }) as i32;
        piv = *V_col.offset(j as isize);
        (!piv.is_null() && ((*piv).c_next).is_null()
            || {
                glp_assert_(
                    b"piv != NULL && piv->c_next == NULL\0" as *const u8 as *const i8,
                    b"draft/lux.c\0" as *const u8 as *const i8,
                    280 as i32,
                );
                1 as i32 != 0
            }) as i32;
    } else {
        i = *R_head.offset(1 as i32 as isize);
        if i != 0 as i32 {
            (*R_len.offset(i as isize) == 1 as i32
                || {
                    glp_assert_(
                        b"R_len[i] == 1\0" as *const u8 as *const i8,
                        b"draft/lux.c\0" as *const u8 as *const i8,
                        287 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            piv = *V_row.offset(i as isize);
            (!piv.is_null() && ((*piv).r_next).is_null()
                || {
                    glp_assert_(
                        b"piv != NULL && piv->r_next == NULL\0" as *const u8
                            as *const i8,
                        b"draft/lux.c\0" as *const u8 as *const i8,
                        289 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            len = 2 as i32;
            's_67: while len <= n {
                j = *C_head.offset(len as isize);
                while j != 0 as i32 {
                    some = 0 as *mut LUXELM;
                    min_len = 2147483647 as i32;
                    vij = *V_col.offset(j as isize);
                    while !vij.is_null() {
                        if min_len > *R_len.offset((*vij).i as isize) {
                            some = vij;
                            min_len = *R_len.offset((*vij).i as isize);
                        }
                        if min_len <= len {
                            piv = some;
                            break 's_67;
                        } else {
                            vij = (*vij).c_next;
                        }
                    }
                    (!some.is_null()
                        || {
                            glp_assert_(
                                b"some != NULL\0" as *const u8 as *const i8,
                                b"draft/lux.c\0" as *const u8 as *const i8,
                                313 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ncand += 1;
                    ncand;
                    cost = (min_len - 1 as i32) as libc::c_double
                        * (len - 1 as i32) as libc::c_double;
                    if cost < best {
                        piv = some;
                        best = cost;
                    }
                    if ncand == piv_lim {
                        break 's_67;
                    }
                    j = *C_next.offset(j as isize);
                }
                i = *R_head.offset(len as isize);
                while i != 0 as i32 {
                    some = 0 as *mut LUXELM;
                    min_len = 2147483647 as i32;
                    vij = *V_row.offset(i as isize);
                    while !vij.is_null() {
                        if min_len > *C_len.offset((*vij).j as isize) {
                            some = vij;
                            min_len = *C_len.offset((*vij).j as isize);
                        }
                        if min_len <= len {
                            piv = some;
                            break 's_67;
                        } else {
                            vij = (*vij).r_next;
                        }
                    }
                    (!some.is_null()
                        || {
                            glp_assert_(
                                b"some != NULL\0" as *const u8 as *const i8,
                                b"draft/lux.c\0" as *const u8 as *const i8,
                                342 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ncand += 1;
                    ncand;
                    cost = (len - 1 as i32) as libc::c_double
                        * (min_len - 1 as i32) as libc::c_double;
                    if cost < best {
                        piv = some;
                        best = cost;
                    }
                    if ncand == piv_lim {
                        break 's_67;
                    }
                    i = *R_next.offset(i as isize);
                }
                len += 1;
                len;
            }
        }
    }
    return piv;
}
unsafe extern "C" fn eliminate(
    mut lux: *mut LUX,
    mut wka: *mut LUXWKA,
    mut piv: *mut LUXELM,
    mut flag: *mut i32,
    mut work: *mut mpq_t,
) {
    let mut pool: *mut DMP = (*lux).pool;
    let mut F_row: *mut *mut LUXELM = (*lux).F_row;
    let mut F_col: *mut *mut LUXELM = (*lux).F_col;
    let mut V_piv: *mut mpq_t = (*lux).V_piv;
    let mut V_row: *mut *mut LUXELM = (*lux).V_row;
    let mut V_col: *mut *mut LUXELM = (*lux).V_col;
    let mut R_len: *mut i32 = (*wka).R_len;
    let mut R_head: *mut i32 = (*wka).R_head;
    let mut R_prev: *mut i32 = (*wka).R_prev;
    let mut R_next: *mut i32 = (*wka).R_next;
    let mut C_len: *mut i32 = (*wka).C_len;
    let mut C_head: *mut i32 = (*wka).C_head;
    let mut C_prev: *mut i32 = (*wka).C_prev;
    let mut C_next: *mut i32 = (*wka).C_next;
    let mut fip: *mut LUXELM = 0 as *mut LUXELM;
    let mut vij: *mut LUXELM = 0 as *mut LUXELM;
    let mut vpj: *mut LUXELM = 0 as *mut LUXELM;
    let mut viq: *mut LUXELM = 0 as *mut LUXELM;
    let mut next: *mut LUXELM = 0 as *mut LUXELM;
    let mut temp: mpq_t = 0 as *mut mpq;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    temp = _glp_mpq_init();
    (!piv.is_null()
        || {
            glp_assert_(
                b"piv != NULL\0" as *const u8 as *const i8,
                b"draft/lux.c\0" as *const u8 as *const i8,
                437 as i32,
            );
            1 as i32 != 0
        }) as i32;
    p = (*piv).i;
    q = (*piv).j;
    if *R_prev.offset(p as isize) == 0 as i32 {
        *R_head.offset(*R_len.offset(p as isize) as isize) = *R_next.offset(p as isize);
    } else {
        *R_next.offset(*R_prev.offset(p as isize) as isize) = *R_next.offset(p as isize);
    }
    if !(*R_next.offset(p as isize) == 0 as i32) {
        *R_prev.offset(*R_next.offset(p as isize) as isize) = *R_prev.offset(p as isize);
    }
    if *C_prev.offset(q as isize) == 0 as i32 {
        *C_head.offset(*C_len.offset(q as isize) as isize) = *C_next.offset(q as isize);
    } else {
        *C_next.offset(*C_prev.offset(q as isize) as isize) = *C_next.offset(q as isize);
    }
    if !(*C_next.offset(q as isize) == 0 as i32) {
        *C_prev.offset(*C_next.offset(q as isize) as isize) = *C_prev.offset(q as isize);
    }
    _glp_mpq_set(*V_piv.offset(p as isize), (*piv).val);
    if ((*piv).r_prev).is_null() {
        let ref mut fresh17 = *V_row.offset(p as isize);
        *fresh17 = (*piv).r_next;
    } else {
        (*(*piv).r_prev).r_next = (*piv).r_next;
    }
    if !((*piv).r_next).is_null() {
        (*(*piv).r_next).r_prev = (*piv).r_prev;
    }
    let ref mut fresh18 = *R_len.offset(p as isize);
    *fresh18 -= 1;
    *fresh18;
    if ((*piv).c_prev).is_null() {
        let ref mut fresh19 = *V_col.offset(q as isize);
        *fresh19 = (*piv).c_next;
    } else {
        (*(*piv).c_prev).c_next = (*piv).c_next;
    }
    if !((*piv).c_next).is_null() {
        (*(*piv).c_next).c_prev = (*piv).c_prev;
    }
    let ref mut fresh20 = *C_len.offset(q as isize);
    *fresh20 -= 1;
    *fresh20;
    _glp_mpq_clear((*piv).val);
    _glp_dmp_free_atom(
        pool,
        piv as *mut libc::c_void,
        ::core::mem::size_of::<LUXELM>() as u64 as i32,
    );
    vpj = *V_row.offset(p as isize);
    while !vpj.is_null() {
        j = (*vpj).j;
        *flag.offset(j as isize) = 1 as i32;
        _glp_mpq_set(*work.offset(j as isize), (*vpj).val);
        if *C_prev.offset(j as isize) == 0 as i32 {
            *C_head.offset(*C_len.offset(j as isize) as isize) = *C_next
                .offset(j as isize);
        } else {
            *C_next.offset(*C_prev.offset(j as isize) as isize) = *C_next
                .offset(j as isize);
        }
        if !(*C_next.offset(j as isize) == 0 as i32) {
            *C_prev.offset(*C_next.offset(j as isize) as isize) = *C_prev
                .offset(j as isize);
        }
        if ((*vpj).c_prev).is_null() {
            let ref mut fresh21 = *V_col.offset(j as isize);
            *fresh21 = (*vpj).c_next;
        } else {
            (*(*vpj).c_prev).c_next = (*vpj).c_next;
        }
        if !((*vpj).c_next).is_null() {
            (*(*vpj).c_next).c_prev = (*vpj).c_prev;
        }
        let ref mut fresh22 = *C_len.offset(j as isize);
        *fresh22 -= 1;
        *fresh22;
        vpj = (*vpj).r_next;
    }
    while !(*V_col.offset(q as isize)).is_null() {
        viq = *V_col.offset(q as isize);
        i = (*viq).i;
        if *R_prev.offset(i as isize) == 0 as i32 {
            *R_head.offset(*R_len.offset(i as isize) as isize) = *R_next
                .offset(i as isize);
        } else {
            *R_next.offset(*R_prev.offset(i as isize) as isize) = *R_next
                .offset(i as isize);
        }
        if !(*R_next.offset(i as isize) == 0 as i32) {
            *R_prev.offset(*R_next.offset(i as isize) as isize) = *R_prev
                .offset(i as isize);
        }
        fip = _glp_dmp_get_atom(pool, ::core::mem::size_of::<LUXELM>() as u64 as i32)
            as *mut LUXELM;
        (*fip).i = i;
        (*fip).j = p;
        (*fip).val = _glp_mpq_init();
        _glp_mpq_div((*fip).val, (*viq).val, *V_piv.offset(p as isize));
        (*fip).r_prev = 0 as *mut LUXELM;
        (*fip).r_next = *F_row.offset(i as isize);
        (*fip).c_prev = 0 as *mut LUXELM;
        (*fip).c_next = *F_col.offset(p as isize);
        if !((*fip).r_next).is_null() {
            (*(*fip).r_next).r_prev = fip;
        }
        if !((*fip).c_next).is_null() {
            (*(*fip).c_next).c_prev = fip;
        }
        let ref mut fresh23 = *F_col.offset(p as isize);
        *fresh23 = fip;
        let ref mut fresh24 = *F_row.offset(i as isize);
        *fresh24 = *fresh23;
        if ((*viq).r_prev).is_null() {
            let ref mut fresh25 = *V_row.offset(i as isize);
            *fresh25 = (*viq).r_next;
        } else {
            (*(*viq).r_prev).r_next = (*viq).r_next;
        }
        if !((*viq).r_next).is_null() {
            (*(*viq).r_next).r_prev = (*viq).r_prev;
        }
        let ref mut fresh26 = *R_len.offset(i as isize);
        *fresh26 -= 1;
        *fresh26;
        let ref mut fresh27 = *V_col.offset(q as isize);
        *fresh27 = (*viq).c_next;
        let ref mut fresh28 = *C_len.offset(q as isize);
        *fresh28 -= 1;
        *fresh28;
        _glp_mpq_clear((*viq).val);
        _glp_dmp_free_atom(
            pool,
            viq as *mut libc::c_void,
            ::core::mem::size_of::<LUXELM>() as u64 as i32,
        );
        vij = *V_row.offset(i as isize);
        while !vij.is_null() {
            next = (*vij).r_next;
            j = (*vij).j;
            if *flag.offset(j as isize) != 0 {
                *flag.offset(j as isize) = 0 as i32;
                _glp_mpq_mul(temp, (*fip).val, *work.offset(j as isize));
                _glp_mpq_sub((*vij).val, (*vij).val, temp);
                if _glp_mpq_sgn((*vij).val) == 0 as i32 {
                    if ((*vij).r_prev).is_null() {
                        let ref mut fresh29 = *V_row.offset(i as isize);
                        *fresh29 = (*vij).r_next;
                    } else {
                        (*(*vij).r_prev).r_next = (*vij).r_next;
                    }
                    if !((*vij).r_next).is_null() {
                        (*(*vij).r_next).r_prev = (*vij).r_prev;
                    }
                    let ref mut fresh30 = *R_len.offset(i as isize);
                    *fresh30 -= 1;
                    *fresh30;
                    if ((*vij).c_prev).is_null() {
                        let ref mut fresh31 = *V_col.offset(j as isize);
                        *fresh31 = (*vij).c_next;
                    } else {
                        (*(*vij).c_prev).c_next = (*vij).c_next;
                    }
                    if !((*vij).c_next).is_null() {
                        (*(*vij).c_next).c_prev = (*vij).c_prev;
                    }
                    let ref mut fresh32 = *C_len.offset(j as isize);
                    *fresh32 -= 1;
                    *fresh32;
                    _glp_mpq_clear((*vij).val);
                    _glp_dmp_free_atom(
                        pool,
                        vij as *mut libc::c_void,
                        ::core::mem::size_of::<LUXELM>() as u64 as i32,
                    );
                }
            }
            vij = next;
        }
        vpj = *V_row.offset(p as isize);
        while !vpj.is_null() {
            j = (*vpj).j;
            if *flag.offset(j as isize) != 0 {
                vij = _glp_dmp_get_atom(
                    pool,
                    ::core::mem::size_of::<LUXELM>() as u64 as i32,
                ) as *mut LUXELM;
                (*vij).i = i;
                (*vij).j = j;
                (*vij).val = _glp_mpq_init();
                _glp_mpq_mul((*vij).val, (*fip).val, *work.offset(j as isize));
                _glp_mpq_neg((*vij).val, (*vij).val);
                (*vij).r_prev = 0 as *mut LUXELM;
                (*vij).r_next = *V_row.offset(i as isize);
                (*vij).c_prev = 0 as *mut LUXELM;
                (*vij).c_next = *V_col.offset(j as isize);
                if !((*vij).r_next).is_null() {
                    (*(*vij).r_next).r_prev = vij;
                }
                if !((*vij).c_next).is_null() {
                    (*(*vij).c_next).c_prev = vij;
                }
                let ref mut fresh33 = *V_col.offset(j as isize);
                *fresh33 = vij;
                let ref mut fresh34 = *V_row.offset(i as isize);
                *fresh34 = *fresh33;
                let ref mut fresh35 = *R_len.offset(i as isize);
                *fresh35 += 1;
                *fresh35;
                let ref mut fresh36 = *C_len.offset(j as isize);
                *fresh36 += 1;
                *fresh36;
            } else {
                *flag.offset(j as isize) = 1 as i32;
            }
            vpj = (*vpj).r_next;
        }
        *R_prev.offset(i as isize) = 0 as i32;
        *R_next.offset(i as isize) = *R_head.offset(*R_len.offset(i as isize) as isize);
        if *R_next.offset(i as isize) != 0 as i32 {
            *R_prev.offset(*R_next.offset(i as isize) as isize) = i;
        }
        *R_head.offset(*R_len.offset(i as isize) as isize) = i;
    }
    (*C_len.offset(q as isize) == 0 as i32
        || {
            glp_assert_(
                b"C_len[q] == 0\0" as *const u8 as *const i8,
                b"draft/lux.c\0" as *const u8 as *const i8,
                642 as i32,
            );
            1 as i32 != 0
        }) as i32;
    vpj = *V_row.offset(p as isize);
    while !vpj.is_null() {
        j = (*vpj).j;
        *flag.offset(j as isize) = 0 as i32;
        _glp_mpq_set_si(*work.offset(j as isize), 0 as i32, 1 as i32 as u32);
        *C_prev.offset(j as isize) = 0 as i32;
        *C_next.offset(j as isize) = *C_head.offset(*C_len.offset(j as isize) as isize);
        if *C_next.offset(j as isize) != 0 as i32 {
            *C_prev.offset(*C_next.offset(j as isize) as isize) = j;
        }
        *C_head.offset(*C_len.offset(j as isize) as isize) = j;
        vpj = (*vpj).r_next;
    }
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lux_decomp(
    mut lux: *mut LUX,
    mut col: Option<
        unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32, *mut mpq_t) -> i32,
    >,
    mut info: *mut libc::c_void,
) -> i32 {
    let mut n: i32 = (*lux).n;
    let mut V_row: *mut *mut LUXELM = (*lux).V_row;
    let mut V_col: *mut *mut LUXELM = (*lux).V_col;
    let mut P_row: *mut i32 = (*lux).P_row;
    let mut P_col: *mut i32 = (*lux).P_col;
    let mut Q_row: *mut i32 = (*lux).Q_row;
    let mut Q_col: *mut i32 = (*lux).Q_col;
    let mut piv: *mut LUXELM = 0 as *mut LUXELM;
    let mut vij: *mut LUXELM = 0 as *mut LUXELM;
    let mut wka: *mut LUXWKA = 0 as *mut LUXWKA;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut t: i32 = 0;
    let mut flag: *mut i32 = 0 as *mut i32;
    let mut work: *mut mpq_t = 0 as *mut mpq_t;
    wka = glp_alloc(1 as i32, ::core::mem::size_of::<LUXWKA>() as u64 as i32)
        as *mut LUXWKA;
    (*wka).R_len = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*wka).R_head = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*wka).R_prev = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*wka).R_next = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*wka).C_len = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*wka).C_head = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*wka).C_prev = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*wka).C_next = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    initialize(lux, col, info, wka);
    flag = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    work = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    k = 1 as i32;
    while k <= n {
        *flag.offset(k as isize) = 0 as i32;
        let ref mut fresh37 = *work.offset(k as isize);
        *fresh37 = _glp_mpq_init();
        k += 1;
        k;
    }
    k = 1 as i32;
    while k <= n {
        piv = find_pivot(lux, wka);
        if piv.is_null() {
            break;
        }
        p = (*piv).i;
        q = (*piv).j;
        i = *P_col.offset(p as isize);
        j = *Q_row.offset(q as isize);
        (k <= i && i <= n && k <= j && j <= n
            || {
                glp_assert_(
                    b"k <= i && i <= n && k <= j && j <= n\0" as *const u8 as *const i8,
                    b"draft/lux.c\0" as *const u8 as *const i8,
                    782 as i32,
                );
                1 as i32 != 0
            }) as i32;
        t = *P_row.offset(k as isize);
        *P_row.offset(i as isize) = t;
        *P_col.offset(t as isize) = i;
        *P_row.offset(k as isize) = p;
        *P_col.offset(p as isize) = k;
        t = *Q_col.offset(k as isize);
        *Q_col.offset(j as isize) = t;
        *Q_row.offset(t as isize) = j;
        *Q_col.offset(k as isize) = q;
        *Q_row.offset(q as isize) = k;
        eliminate(lux, wka, piv, flag, work);
        k += 1;
        k;
    }
    (*lux).rank = k - 1 as i32;
    glp_free(flag as *mut libc::c_void);
    k = 1 as i32;
    while k <= n {
        _glp_mpq_clear(*work.offset(k as isize));
        k += 1;
        k;
    }
    glp_free(work as *mut libc::c_void);
    j = 1 as i32;
    while j <= n {
        ((*V_col.offset(j as isize)).is_null()
            || {
                glp_assert_(
                    b"V_col[j] == NULL\0" as *const u8 as *const i8,
                    b"draft/lux.c\0" as *const u8 as *const i8,
                    803 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= n {
        vij = *V_row.offset(i as isize);
        while !vij.is_null() {
            j = (*vij).j;
            (*vij).c_prev = 0 as *mut LUXELM;
            (*vij).c_next = *V_col.offset(j as isize);
            if !((*vij).c_next).is_null() {
                (*(*vij).c_next).c_prev = vij;
            }
            let ref mut fresh38 = *V_col.offset(j as isize);
            *fresh38 = vij;
            vij = (*vij).r_next;
        }
        i += 1;
        i;
    }
    glp_free((*wka).R_len as *mut libc::c_void);
    glp_free((*wka).R_head as *mut libc::c_void);
    glp_free((*wka).R_prev as *mut libc::c_void);
    glp_free((*wka).R_next as *mut libc::c_void);
    glp_free((*wka).C_len as *mut libc::c_void);
    glp_free((*wka).C_head as *mut libc::c_void);
    glp_free((*wka).C_prev as *mut libc::c_void);
    glp_free((*wka).C_next as *mut libc::c_void);
    glp_free(wka as *mut libc::c_void);
    return ((*lux).rank < n) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lux_f_solve(
    mut lux: *mut LUX,
    mut tr: i32,
    mut x: *mut mpq_t,
) {
    let mut n: i32 = (*lux).n;
    let mut F_row: *mut *mut LUXELM = (*lux).F_row;
    let mut F_col: *mut *mut LUXELM = (*lux).F_col;
    let mut P_row: *mut i32 = (*lux).P_row;
    let mut fik: *mut LUXELM = 0 as *mut LUXELM;
    let mut fkj: *mut LUXELM = 0 as *mut LUXELM;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut temp: mpq_t = 0 as *mut mpq;
    temp = _glp_mpq_init();
    if tr == 0 {
        j = 1 as i32;
        while j <= n {
            k = *P_row.offset(j as isize);
            if _glp_mpq_sgn(*x.offset(k as isize)) != 0 as i32 {
                fik = *F_col.offset(k as isize);
                while !fik.is_null() {
                    _glp_mpq_mul(temp, (*fik).val, *x.offset(k as isize));
                    _glp_mpq_sub(
                        *x.offset((*fik).i as isize),
                        *x.offset((*fik).i as isize),
                        temp,
                    );
                    fik = (*fik).c_next;
                }
            }
            j += 1;
            j;
        }
    } else {
        i = n;
        while i >= 1 as i32 {
            k = *P_row.offset(i as isize);
            if _glp_mpq_sgn(*x.offset(k as isize)) != 0 as i32 {
                fkj = *F_row.offset(k as isize);
                while !fkj.is_null() {
                    _glp_mpq_mul(temp, (*fkj).val, *x.offset(k as isize));
                    _glp_mpq_sub(
                        *x.offset((*fkj).j as isize),
                        *x.offset((*fkj).j as isize),
                        temp,
                    );
                    fkj = (*fkj).r_next;
                }
            }
            i -= 1;
            i;
        }
    }
    _glp_mpq_clear(temp);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lux_v_solve(
    mut lux: *mut LUX,
    mut tr: i32,
    mut x: *mut mpq_t,
) {
    let mut n: i32 = (*lux).n;
    let mut V_piv: *mut mpq_t = (*lux).V_piv;
    let mut V_row: *mut *mut LUXELM = (*lux).V_row;
    let mut V_col: *mut *mut LUXELM = (*lux).V_col;
    let mut P_row: *mut i32 = (*lux).P_row;
    let mut Q_col: *mut i32 = (*lux).Q_col;
    let mut vij: *mut LUXELM = 0 as *mut LUXELM;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut b: *mut mpq_t = 0 as *mut mpq_t;
    let mut temp: mpq_t = 0 as *mut mpq;
    b = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    k = 1 as i32;
    while k <= n {
        let ref mut fresh39 = *b.offset(k as isize);
        *fresh39 = _glp_mpq_init();
        _glp_mpq_set(*b.offset(k as isize), *x.offset(k as isize));
        _glp_mpq_set_si(*x.offset(k as isize), 0 as i32, 1 as i32 as u32);
        k += 1;
        k;
    }
    temp = _glp_mpq_init();
    if tr == 0 {
        k = n;
        while k >= 1 as i32 {
            i = *P_row.offset(k as isize);
            j = *Q_col.offset(k as isize);
            if _glp_mpq_sgn(*b.offset(i as isize)) != 0 as i32 {
                _glp_mpq_set(*x.offset(j as isize), *b.offset(i as isize));
                _glp_mpq_div(
                    *x.offset(j as isize),
                    *x.offset(j as isize),
                    *V_piv.offset(i as isize),
                );
                vij = *V_col.offset(j as isize);
                while !vij.is_null() {
                    _glp_mpq_mul(temp, (*vij).val, *x.offset(j as isize));
                    _glp_mpq_sub(
                        *b.offset((*vij).i as isize),
                        *b.offset((*vij).i as isize),
                        temp,
                    );
                    vij = (*vij).c_next;
                }
            }
            k -= 1;
            k;
        }
    } else {
        k = 1 as i32;
        while k <= n {
            i = *P_row.offset(k as isize);
            j = *Q_col.offset(k as isize);
            if _glp_mpq_sgn(*b.offset(j as isize)) != 0 as i32 {
                _glp_mpq_set(*x.offset(i as isize), *b.offset(j as isize));
                _glp_mpq_div(
                    *x.offset(i as isize),
                    *x.offset(i as isize),
                    *V_piv.offset(i as isize),
                );
                vij = *V_row.offset(i as isize);
                while !vij.is_null() {
                    _glp_mpq_mul(temp, (*vij).val, *x.offset(i as isize));
                    _glp_mpq_sub(
                        *b.offset((*vij).j as isize),
                        *b.offset((*vij).j as isize),
                        temp,
                    );
                    vij = (*vij).r_next;
                }
            }
            k += 1;
            k;
        }
    }
    k = 1 as i32;
    while k <= n {
        _glp_mpq_clear(*b.offset(k as isize));
        k += 1;
        k;
    }
    _glp_mpq_clear(temp);
    glp_free(b as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lux_solve(
    mut lux: *mut LUX,
    mut tr: i32,
    mut x: *mut mpq_t,
) {
    if (*lux).rank < (*lux).n {
        (glp_error_(b"draft/lux.c\0" as *const u8 as *const i8, 974 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"lux_solve: LU-factorization has incomplete rank\n\0" as *const u8
                as *const i8,
        );
    }
    if tr == 0 {
        _glp_lux_f_solve(lux, 0 as i32, x);
        _glp_lux_v_solve(lux, 0 as i32, x);
    } else {
        _glp_lux_v_solve(lux, 1 as i32, x);
        _glp_lux_f_solve(lux, 1 as i32, x);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_lux_delete(mut lux: *mut LUX) {
    let mut n: i32 = (*lux).n;
    let mut fij: *mut LUXELM = 0 as *mut LUXELM;
    let mut vij: *mut LUXELM = 0 as *mut LUXELM;
    let mut i: i32 = 0;
    i = 1 as i32;
    while i <= n {
        fij = *((*lux).F_row).offset(i as isize);
        while !fij.is_null() {
            _glp_mpq_clear((*fij).val);
            fij = (*fij).r_next;
        }
        _glp_mpq_clear(*((*lux).V_piv).offset(i as isize));
        vij = *((*lux).V_row).offset(i as isize);
        while !vij.is_null() {
            _glp_mpq_clear((*vij).val);
            vij = (*vij).r_next;
        }
        i += 1;
        i;
    }
    _glp_dmp_delete_pool((*lux).pool);
    glp_free((*lux).F_row as *mut libc::c_void);
    glp_free((*lux).F_col as *mut libc::c_void);
    glp_free((*lux).V_piv as *mut libc::c_void);
    glp_free((*lux).V_row as *mut libc::c_void);
    glp_free((*lux).V_col as *mut libc::c_void);
    glp_free((*lux).P_row as *mut libc::c_void);
    glp_free((*lux).P_col as *mut libc::c_void);
    glp_free((*lux).Q_row as *mut libc::c_void);
    glp_free((*lux).Q_col as *mut libc::c_void);
    glp_free(lux as *mut libc::c_void);
}