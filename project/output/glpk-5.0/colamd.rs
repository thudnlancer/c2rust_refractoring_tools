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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn glp_printf(fmt: *const i8, _: ...);
}
pub type size_t = u64;
pub type Colamd_Row = Colamd_Row_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colamd_Row_struct {
    pub start: i32,
    pub length: i32,
    pub shared1: C2RustUnnamed_0,
    pub shared2: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub mark: i32,
    pub first_column: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub degree: i32,
    pub p: i32,
}
pub type Colamd_Col = Colamd_Col_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Colamd_Col_struct {
    pub start: i32,
    pub length: i32,
    pub shared1: C2RustUnnamed_4,
    pub shared2: C2RustUnnamed_3,
    pub shared3: C2RustUnnamed_2,
    pub shared4: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub degree_next: i32,
    pub hash_next: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub headhash: i32,
    pub hash: i32,
    pub prev: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub score: i32,
    pub order: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub thickness: i32,
    pub parent: i32,
}
unsafe extern "C" fn t_add(mut a: size_t, mut b: size_t, mut ok: *mut i32) -> size_t {
    *ok = (*ok != 0 && a.wrapping_add(b) >= (if a > b { a } else { b })) as i32;
    return if *ok != 0 { a.wrapping_add(b) } else { 0 as i32 as u64 };
}
unsafe extern "C" fn t_mult(mut a: size_t, mut k: size_t, mut ok: *mut i32) -> size_t {
    let mut i: size_t = 0;
    let mut s: size_t = 0 as i32 as size_t;
    i = 0 as i32 as size_t;
    while i < k {
        s = t_add(s, a, ok);
        i = i.wrapping_add(1);
        i;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_colamd_recommended(
    mut nnz: i32,
    mut n_row: i32,
    mut n_col: i32,
) -> size_t {
    let mut s: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut ok: i32 = 1 as i32;
    if nnz < 0 as i32 || n_row < 0 as i32 || n_col < 0 as i32 {
        return 0 as i32 as size_t;
    }
    s = t_mult(nnz as size_t, 2 as i32 as size_t, &mut ok);
    c = (t_mult(
        t_add(n_col as size_t, 1 as i32 as size_t, &mut ok),
        ::core::mem::size_of::<Colamd_Col>() as u64,
        &mut ok,
    ))
        .wrapping_div(::core::mem::size_of::<i32>() as u64);
    r = (t_mult(
        t_add(n_row as size_t, 1 as i32 as size_t, &mut ok),
        ::core::mem::size_of::<Colamd_Row>() as u64,
        &mut ok,
    ))
        .wrapping_div(::core::mem::size_of::<i32>() as u64);
    s = t_add(s, c, &mut ok);
    s = t_add(s, r, &mut ok);
    s = t_add(s, n_col as size_t, &mut ok);
    s = t_add(s, (nnz / 5 as i32) as size_t, &mut ok);
    ok = (ok != 0 && s < 2147483647 as i32 as u64) as i32;
    return if ok != 0 { s } else { 0 as i32 as u64 };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_colamd_set_defaults(mut knobs: *mut libc::c_double) {
    let mut i: i32 = 0;
    if knobs.is_null() {
        return;
    }
    i = 0 as i32;
    while i < 20 as i32 {
        *knobs.offset(i as isize) = 0 as i32 as libc::c_double;
        i += 1;
        i;
    }
    *knobs.offset(0 as i32 as isize) = 10 as i32 as libc::c_double;
    *knobs.offset(1 as i32 as isize) = 10 as i32 as libc::c_double;
    *knobs.offset(2 as i32 as isize) = 1 as i32 as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_symamd(
    mut n: i32,
    mut A: *mut i32,
    mut p: *mut i32,
    mut perm: *mut i32,
    mut knobs: *mut libc::c_double,
    mut stats: *mut i32,
    mut allocate: Option<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    mut release: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> i32 {
    let mut count: *mut i32 = 0 as *mut i32;
    let mut mark: *mut i32 = 0 as *mut i32;
    let mut M: *mut i32 = 0 as *mut i32;
    let mut Mlen: size_t = 0;
    let mut n_row: i32 = 0;
    let mut nnz: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut mnz: i32 = 0;
    let mut pp: i32 = 0;
    let mut last_row: i32 = 0;
    let mut length: i32 = 0;
    let mut cknobs: [libc::c_double; 20] = [0.; 20];
    let mut default_knobs: [libc::c_double; 20] = [0.; 20];
    if stats.is_null() {
        return 0 as i32;
    }
    i = 0 as i32;
    while i < 20 as i32 {
        *stats.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    *stats.offset(3 as i32 as isize) = 0 as i32;
    *stats.offset(4 as i32 as isize) = -(1 as i32);
    *stats.offset(5 as i32 as isize) = -(1 as i32);
    if A.is_null() {
        *stats.offset(3 as i32 as isize) = -(1 as i32);
        return 0 as i32;
    }
    if p.is_null() {
        *stats.offset(3 as i32 as isize) = -(2 as i32);
        return 0 as i32;
    }
    if n < 0 as i32 {
        *stats.offset(3 as i32 as isize) = -(4 as i32);
        *stats.offset(4 as i32 as isize) = n;
        return 0 as i32;
    }
    nnz = *p.offset(n as isize);
    if nnz < 0 as i32 {
        *stats.offset(3 as i32 as isize) = -(5 as i32);
        *stats.offset(4 as i32 as isize) = nnz;
        return 0 as i32;
    }
    if *p.offset(0 as i32 as isize) != 0 as i32 {
        *stats.offset(3 as i32 as isize) = -(6 as i32);
        *stats.offset(4 as i32 as isize) = *p.offset(0 as i32 as isize);
        return 0 as i32;
    }
    if knobs.is_null() {
        _glp_colamd_set_defaults(default_knobs.as_mut_ptr());
        knobs = default_knobs.as_mut_ptr();
    }
    count = (Some(allocate.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((n + 1 as i32) as size_t, ::core::mem::size_of::<i32>() as u64) as *mut i32;
    if count.is_null() {
        *stats.offset(3 as i32 as isize) = -(10 as i32);
        return 0 as i32;
    }
    mark = (Some(allocate.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((n + 1 as i32) as size_t, ::core::mem::size_of::<i32>() as u64) as *mut i32;
    if mark.is_null() {
        *stats.offset(3 as i32 as isize) = -(10 as i32);
        (Some(release.expect("non-null function pointer")))
            .expect("non-null function pointer")(count as *mut libc::c_void);
        return 0 as i32;
    }
    *stats.offset(6 as i32 as isize) = 0 as i32;
    i = 0 as i32;
    while i < n {
        *mark.offset(i as isize) = -(1 as i32);
        i += 1;
        i;
    }
    j = 0 as i32;
    while j < n {
        last_row = -(1 as i32);
        length = *p.offset((j + 1 as i32) as isize) - *p.offset(j as isize);
        if length < 0 as i32 {
            *stats.offset(3 as i32 as isize) = -(8 as i32);
            *stats.offset(4 as i32 as isize) = j;
            *stats.offset(5 as i32 as isize) = length;
            (Some(release.expect("non-null function pointer")))
                .expect("non-null function pointer")(count as *mut libc::c_void);
            (Some(release.expect("non-null function pointer")))
                .expect("non-null function pointer")(mark as *mut libc::c_void);
            return 0 as i32;
        }
        pp = *p.offset(j as isize);
        while pp < *p.offset((j + 1 as i32) as isize) {
            i = *A.offset(pp as isize);
            if i < 0 as i32 || i >= n {
                *stats.offset(3 as i32 as isize) = -(9 as i32);
                *stats.offset(4 as i32 as isize) = j;
                *stats.offset(5 as i32 as isize) = i;
                *stats.offset(6 as i32 as isize) = n;
                (Some(release.expect("non-null function pointer")))
                    .expect("non-null function pointer")(count as *mut libc::c_void);
                (Some(release.expect("non-null function pointer")))
                    .expect("non-null function pointer")(mark as *mut libc::c_void);
                return 0 as i32;
            }
            if i <= last_row || *mark.offset(i as isize) == j {
                *stats.offset(3 as i32 as isize) = 1 as i32;
                *stats.offset(4 as i32 as isize) = j;
                *stats.offset(5 as i32 as isize) = i;
                let ref mut fresh0 = *stats.offset(6 as i32 as isize);
                *fresh0 += 1;
                *fresh0;
            }
            if i > j && *mark.offset(i as isize) != j {
                let ref mut fresh1 = *count.offset(i as isize);
                *fresh1 += 1;
                *fresh1;
                let ref mut fresh2 = *count.offset(j as isize);
                *fresh2 += 1;
                *fresh2;
            }
            *mark.offset(i as isize) = j;
            last_row = i;
            pp += 1;
            pp;
        }
        j += 1;
        j;
    }
    *perm.offset(0 as i32 as isize) = 0 as i32;
    j = 1 as i32;
    while j <= n {
        *perm.offset(j as isize) = *perm.offset((j - 1 as i32) as isize)
            + *count.offset((j - 1 as i32) as isize);
        j += 1;
        j;
    }
    j = 0 as i32;
    while j < n {
        *count.offset(j as isize) = *perm.offset(j as isize);
        j += 1;
        j;
    }
    mnz = *perm.offset(n as isize);
    n_row = mnz / 2 as i32;
    Mlen = _glp_colamd_recommended(mnz, n_row, n);
    M = (Some(allocate.expect("non-null function pointer")))
        .expect("non-null function pointer")(Mlen, ::core::mem::size_of::<i32>() as u64)
        as *mut i32;
    if M.is_null() {
        *stats.offset(3 as i32 as isize) = -(10 as i32);
        (Some(release.expect("non-null function pointer")))
            .expect("non-null function pointer")(count as *mut libc::c_void);
        (Some(release.expect("non-null function pointer")))
            .expect("non-null function pointer")(mark as *mut libc::c_void);
        return 0 as i32;
    }
    k = 0 as i32;
    if *stats.offset(3 as i32 as isize) == 0 as i32 {
        j = 0 as i32;
        while j < n {
            pp = *p.offset(j as isize);
            while pp < *p.offset((j + 1 as i32) as isize) {
                i = *A.offset(pp as isize);
                if i > j {
                    let ref mut fresh3 = *count.offset(i as isize);
                    let fresh4 = *fresh3;
                    *fresh3 = *fresh3 + 1;
                    *M.offset(fresh4 as isize) = k;
                    let ref mut fresh5 = *count.offset(j as isize);
                    let fresh6 = *fresh5;
                    *fresh5 = *fresh5 + 1;
                    *M.offset(fresh6 as isize) = k;
                    k += 1;
                    k;
                }
                pp += 1;
                pp;
            }
            j += 1;
            j;
        }
    } else {
        i = 0 as i32;
        while i < n {
            *mark.offset(i as isize) = -(1 as i32);
            i += 1;
            i;
        }
        j = 0 as i32;
        while j < n {
            pp = *p.offset(j as isize);
            while pp < *p.offset((j + 1 as i32) as isize) {
                i = *A.offset(pp as isize);
                if i > j && *mark.offset(i as isize) != j {
                    let ref mut fresh7 = *count.offset(i as isize);
                    let fresh8 = *fresh7;
                    *fresh7 = *fresh7 + 1;
                    *M.offset(fresh8 as isize) = k;
                    let ref mut fresh9 = *count.offset(j as isize);
                    let fresh10 = *fresh9;
                    *fresh9 = *fresh9 + 1;
                    *M.offset(fresh10 as isize) = k;
                    k += 1;
                    k;
                    *mark.offset(i as isize) = j;
                }
                pp += 1;
                pp;
            }
            j += 1;
            j;
        }
    }
    (Some(release.expect("non-null function pointer")))
        .expect("non-null function pointer")(count as *mut libc::c_void);
    (Some(release.expect("non-null function pointer")))
        .expect("non-null function pointer")(mark as *mut libc::c_void);
    i = 0 as i32;
    while i < 20 as i32 {
        cknobs[i as usize] = *knobs.offset(i as isize);
        i += 1;
        i;
    }
    cknobs[0 as i32 as usize] = -(1 as i32) as libc::c_double;
    cknobs[1 as i32 as usize] = *knobs.offset(0 as i32 as isize);
    _glp_colamd(n_row, n, Mlen as i32, M, perm, cknobs.as_mut_ptr(), stats);
    *stats.offset(0 as i32 as isize) = *stats.offset(1 as i32 as isize);
    (Some(release.expect("non-null function pointer")))
        .expect("non-null function pointer")(M as *mut libc::c_void);
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_colamd(
    mut n_row: i32,
    mut n_col: i32,
    mut Alen: i32,
    mut A: *mut i32,
    mut p: *mut i32,
    mut knobs: *mut libc::c_double,
    mut stats: *mut i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut nnz: i32 = 0;
    let mut Row_size: size_t = 0;
    let mut Col_size: size_t = 0;
    let mut need: size_t = 0;
    let mut Row: *mut Colamd_Row = 0 as *mut Colamd_Row;
    let mut Col: *mut Colamd_Col = 0 as *mut Colamd_Col;
    let mut n_col2: i32 = 0;
    let mut n_row2: i32 = 0;
    let mut ngarbage: i32 = 0;
    let mut max_deg: i32 = 0;
    let mut default_knobs: [libc::c_double; 20] = [0.; 20];
    let mut aggressive: i32 = 0;
    let mut ok: i32 = 0;
    if stats.is_null() {
        return 0 as i32;
    }
    i = 0 as i32;
    while i < 20 as i32 {
        *stats.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    *stats.offset(3 as i32 as isize) = 0 as i32;
    *stats.offset(4 as i32 as isize) = -(1 as i32);
    *stats.offset(5 as i32 as isize) = -(1 as i32);
    if A.is_null() {
        *stats.offset(3 as i32 as isize) = -(1 as i32);
        return 0 as i32;
    }
    if p.is_null() {
        *stats.offset(3 as i32 as isize) = -(2 as i32);
        return 0 as i32;
    }
    if n_row < 0 as i32 {
        *stats.offset(3 as i32 as isize) = -(3 as i32);
        *stats.offset(4 as i32 as isize) = n_row;
        return 0 as i32;
    }
    if n_col < 0 as i32 {
        *stats.offset(3 as i32 as isize) = -(4 as i32);
        *stats.offset(4 as i32 as isize) = n_col;
        return 0 as i32;
    }
    nnz = *p.offset(n_col as isize);
    if nnz < 0 as i32 {
        *stats.offset(3 as i32 as isize) = -(5 as i32);
        *stats.offset(4 as i32 as isize) = nnz;
        return 0 as i32;
    }
    if *p.offset(0 as i32 as isize) != 0 as i32 {
        *stats.offset(3 as i32 as isize) = -(6 as i32);
        *stats.offset(4 as i32 as isize) = *p.offset(0 as i32 as isize);
        return 0 as i32;
    }
    if knobs.is_null() {
        _glp_colamd_set_defaults(default_knobs.as_mut_ptr());
        knobs = default_knobs.as_mut_ptr();
    }
    aggressive = (*knobs.offset(2 as i32 as isize) != 0 as i32 as libc::c_double) as i32;
    ok = 1 as i32;
    Col_size = (t_mult(
        t_add(n_col as size_t, 1 as i32 as size_t, &mut ok),
        ::core::mem::size_of::<Colamd_Col>() as u64,
        &mut ok,
    ))
        .wrapping_div(::core::mem::size_of::<i32>() as u64);
    Row_size = (t_mult(
        t_add(n_row as size_t, 1 as i32 as size_t, &mut ok),
        ::core::mem::size_of::<Colamd_Row>() as u64,
        &mut ok,
    ))
        .wrapping_div(::core::mem::size_of::<i32>() as u64);
    need = t_mult(nnz as size_t, 2 as i32 as size_t, &mut ok);
    need = t_add(need, n_col as size_t, &mut ok);
    need = t_add(need, Col_size, &mut ok);
    need = t_add(need, Row_size, &mut ok);
    if ok == 0 || need > Alen as size_t || need > 2147483647 as i32 as u64 {
        *stats.offset(3 as i32 as isize) = -(7 as i32);
        *stats.offset(4 as i32 as isize) = need as i32;
        *stats.offset(5 as i32 as isize) = Alen;
        return 0 as i32;
    }
    Alen = (Alen as u64).wrapping_sub(Col_size.wrapping_add(Row_size)) as i32 as i32;
    Col = &mut *A.offset(Alen as isize) as *mut i32 as *mut Colamd_Col;
    Row = &mut *A.offset((Alen as u64).wrapping_add(Col_size) as isize) as *mut i32
        as *mut Colamd_Row;
    if init_rows_cols(n_row, n_col, Row, Col, A, p, stats) == 0 {
        return 0 as i32;
    }
    init_scoring(
        n_row,
        n_col,
        Row,
        Col,
        A,
        p,
        knobs,
        &mut n_row2,
        &mut n_col2,
        &mut max_deg,
    );
    ngarbage = find_ordering(
        n_row,
        n_col,
        Alen,
        Row,
        Col,
        A,
        p,
        n_col2,
        max_deg,
        2 as i32 * nnz,
        aggressive,
    );
    order_children(n_col, Col, p);
    *stats.offset(0 as i32 as isize) = n_row - n_row2;
    *stats.offset(1 as i32 as isize) = n_col - n_col2;
    *stats.offset(2 as i32 as isize) = ngarbage;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_colamd_report(mut stats: *mut i32) {
    print_report(b"colamd\0" as *const u8 as *const i8 as *mut i8, stats);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_symamd_report(mut stats: *mut i32) {
    print_report(b"symamd\0" as *const u8 as *const i8 as *mut i8, stats);
}
unsafe extern "C" fn init_rows_cols(
    mut n_row: i32,
    mut n_col: i32,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut i32,
    mut p: *mut i32,
    mut stats: *mut i32,
) -> i32 {
    let mut col: i32 = 0;
    let mut row: i32 = 0;
    let mut cp: *mut i32 = 0 as *mut i32;
    let mut cp_end: *mut i32 = 0 as *mut i32;
    let mut rp: *mut i32 = 0 as *mut i32;
    let mut rp_end: *mut i32 = 0 as *mut i32;
    let mut last_row: i32 = 0;
    col = 0 as i32;
    while col < n_col {
        (*Col.offset(col as isize)).start = *p.offset(col as isize);
        (*Col.offset(col as isize)).length = *p.offset((col + 1 as i32) as isize)
            - *p.offset(col as isize);
        if (*Col.offset(col as isize)).length < 0 as i32 {
            *stats.offset(3 as i32 as isize) = -(8 as i32);
            *stats.offset(4 as i32 as isize) = col;
            *stats.offset(5 as i32 as isize) = (*Col.offset(col as isize)).length;
            return 0 as i32;
        }
        (*Col.offset(col as isize)).shared1.thickness = 1 as i32;
        (*Col.offset(col as isize)).shared2.score = 0 as i32;
        (*Col.offset(col as isize)).shared3.prev = -(1 as i32);
        (*Col.offset(col as isize)).shared4.degree_next = -(1 as i32);
        col += 1;
        col;
    }
    *stats.offset(6 as i32 as isize) = 0 as i32;
    row = 0 as i32;
    while row < n_row {
        (*Row.offset(row as isize)).length = 0 as i32;
        (*Row.offset(row as isize)).shared2.mark = -(1 as i32);
        row += 1;
        row;
    }
    col = 0 as i32;
    while col < n_col {
        last_row = -(1 as i32);
        cp = &mut *A.offset(*p.offset(col as isize) as isize) as *mut i32;
        cp_end = &mut *A.offset(*p.offset((col + 1 as i32) as isize) as isize)
            as *mut i32;
        while cp < cp_end {
            let fresh11 = cp;
            cp = cp.offset(1);
            row = *fresh11;
            if row < 0 as i32 || row >= n_row {
                *stats.offset(3 as i32 as isize) = -(9 as i32);
                *stats.offset(4 as i32 as isize) = col;
                *stats.offset(5 as i32 as isize) = row;
                *stats.offset(6 as i32 as isize) = n_row;
                return 0 as i32;
            }
            if row <= last_row || (*Row.offset(row as isize)).shared2.mark == col {
                *stats.offset(3 as i32 as isize) = 1 as i32;
                *stats.offset(4 as i32 as isize) = col;
                *stats.offset(5 as i32 as isize) = row;
                let ref mut fresh12 = *stats.offset(6 as i32 as isize);
                *fresh12 += 1;
                *fresh12;
            }
            if (*Row.offset(row as isize)).shared2.mark != col {
                let ref mut fresh13 = (*Row.offset(row as isize)).length;
                *fresh13 += 1;
                *fresh13;
            } else {
                let ref mut fresh14 = (*Col.offset(col as isize)).length;
                *fresh14 -= 1;
                *fresh14;
            }
            (*Row.offset(row as isize)).shared2.mark = col;
            last_row = row;
        }
        col += 1;
        col;
    }
    (*Row.offset(0 as i32 as isize)).start = *p.offset(n_col as isize);
    (*Row.offset(0 as i32 as isize)).shared1.p = (*Row.offset(0 as i32 as isize)).start;
    (*Row.offset(0 as i32 as isize)).shared2.mark = -(1 as i32);
    row = 1 as i32;
    while row < n_row {
        (*Row.offset(row as isize)).start = (*Row.offset((row - 1 as i32) as isize))
            .start + (*Row.offset((row - 1 as i32) as isize)).length;
        (*Row.offset(row as isize)).shared1.p = (*Row.offset(row as isize)).start;
        (*Row.offset(row as isize)).shared2.mark = -(1 as i32);
        row += 1;
        row;
    }
    if *stats.offset(3 as i32 as isize) == 1 as i32 {
        col = 0 as i32;
        while col < n_col {
            cp = &mut *A.offset(*p.offset(col as isize) as isize) as *mut i32;
            cp_end = &mut *A.offset(*p.offset((col + 1 as i32) as isize) as isize)
                as *mut i32;
            while cp < cp_end {
                let fresh15 = cp;
                cp = cp.offset(1);
                row = *fresh15;
                if (*Row.offset(row as isize)).shared2.mark != col {
                    let ref mut fresh16 = (*Row.offset(row as isize)).shared1.p;
                    let fresh17 = *fresh16;
                    *fresh16 = *fresh16 + 1;
                    *A.offset(fresh17 as isize) = col;
                    (*Row.offset(row as isize)).shared2.mark = col;
                }
            }
            col += 1;
            col;
        }
    } else {
        col = 0 as i32;
        while col < n_col {
            cp = &mut *A.offset(*p.offset(col as isize) as isize) as *mut i32;
            cp_end = &mut *A.offset(*p.offset((col + 1 as i32) as isize) as isize)
                as *mut i32;
            while cp < cp_end {
                let fresh18 = cp;
                cp = cp.offset(1);
                let ref mut fresh19 = (*Row.offset(*fresh18 as isize)).shared1.p;
                let fresh20 = *fresh19;
                *fresh19 = *fresh19 + 1;
                *A.offset(fresh20 as isize) = col;
            }
            col += 1;
            col;
        }
    }
    row = 0 as i32;
    while row < n_row {
        (*Row.offset(row as isize)).shared2.mark = 0 as i32;
        (*Row.offset(row as isize)).shared1.degree = (*Row.offset(row as isize)).length;
        row += 1;
        row;
    }
    if *stats.offset(3 as i32 as isize) == 1 as i32 {
        (*Col.offset(0 as i32 as isize)).start = 0 as i32;
        *p.offset(0 as i32 as isize) = (*Col.offset(0 as i32 as isize)).start;
        col = 1 as i32;
        while col < n_col {
            (*Col.offset(col as isize)).start = (*Col.offset((col - 1 as i32) as isize))
                .start + (*Col.offset((col - 1 as i32) as isize)).length;
            *p.offset(col as isize) = (*Col.offset(col as isize)).start;
            col += 1;
            col;
        }
        row = 0 as i32;
        while row < n_row {
            rp = &mut *A.offset((*Row.offset(row as isize)).start as isize) as *mut i32;
            rp_end = rp.offset((*Row.offset(row as isize)).length as isize);
            while rp < rp_end {
                let fresh21 = rp;
                rp = rp.offset(1);
                let ref mut fresh22 = *p.offset(*fresh21 as isize);
                let fresh23 = *fresh22;
                *fresh22 = *fresh22 + 1;
                *A.offset(fresh23 as isize) = row;
            }
            row += 1;
            row;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn init_scoring(
    mut n_row: i32,
    mut n_col: i32,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut i32,
    mut head: *mut i32,
    mut knobs: *mut libc::c_double,
    mut p_n_row2: *mut i32,
    mut p_n_col2: *mut i32,
    mut p_max_deg: *mut i32,
) {
    let mut c: i32 = 0;
    let mut r: i32 = 0;
    let mut row: i32 = 0;
    let mut cp: *mut i32 = 0 as *mut i32;
    let mut deg: i32 = 0;
    let mut cp_end: *mut i32 = 0 as *mut i32;
    let mut new_cp: *mut i32 = 0 as *mut i32;
    let mut col_length: i32 = 0;
    let mut score: i32 = 0;
    let mut n_col2: i32 = 0;
    let mut n_row2: i32 = 0;
    let mut dense_row_count: i32 = 0;
    let mut dense_col_count: i32 = 0;
    let mut min_score: i32 = 0;
    let mut max_deg: i32 = 0;
    let mut next_col: i32 = 0;
    if *knobs.offset(0 as i32 as isize) < 0 as i32 as libc::c_double {
        dense_row_count = n_col - 1 as i32;
    } else {
        dense_row_count = (if 16.0f64
            > *knobs.offset(0 as i32 as isize) * sqrt(n_col as libc::c_double)
        {
            16.0f64
        } else {
            *knobs.offset(0 as i32 as isize) * sqrt(n_col as libc::c_double)
        }) as i32;
    }
    if *knobs.offset(1 as i32 as isize) < 0 as i32 as libc::c_double {
        dense_col_count = n_row - 1 as i32;
    } else {
        dense_col_count = (if 16.0f64
            > *knobs.offset(1 as i32 as isize)
                * sqrt((if n_row < n_col { n_row } else { n_col }) as libc::c_double)
        {
            16.0f64
        } else {
            *knobs.offset(1 as i32 as isize)
                * sqrt((if n_row < n_col { n_row } else { n_col }) as libc::c_double)
        }) as i32;
    }
    max_deg = 0 as i32;
    n_col2 = n_col;
    n_row2 = n_row;
    c = n_col - 1 as i32;
    while c >= 0 as i32 {
        deg = (*Col.offset(c as isize)).length;
        if deg == 0 as i32 {
            n_col2 -= 1;
            (*Col.offset(c as isize)).shared2.order = n_col2;
            (*Col.offset(c as isize)).start = -(1 as i32);
        }
        c -= 1;
        c;
    }
    c = n_col - 1 as i32;
    while c >= 0 as i32 {
        if !((*Col.offset(c as isize)).start < 0 as i32) {
            deg = (*Col.offset(c as isize)).length;
            if deg > dense_col_count {
                n_col2 -= 1;
                (*Col.offset(c as isize)).shared2.order = n_col2;
                cp = &mut *A.offset((*Col.offset(c as isize)).start as isize)
                    as *mut i32;
                cp_end = cp.offset((*Col.offset(c as isize)).length as isize);
                while cp < cp_end {
                    let fresh24 = cp;
                    cp = cp.offset(1);
                    let ref mut fresh25 = (*Row.offset(*fresh24 as isize))
                        .shared1
                        .degree;
                    *fresh25 -= 1;
                    *fresh25;
                }
                (*Col.offset(c as isize)).start = -(1 as i32);
            }
        }
        c -= 1;
        c;
    }
    r = 0 as i32;
    while r < n_row {
        deg = (*Row.offset(r as isize)).shared1.degree;
        if deg > dense_row_count || deg == 0 as i32 {
            (*Row.offset(r as isize)).shared2.mark = -(1 as i32);
            n_row2 -= 1;
            n_row2;
        } else {
            max_deg = if max_deg > deg { max_deg } else { deg };
        }
        r += 1;
        r;
    }
    c = n_col - 1 as i32;
    while c >= 0 as i32 {
        if !((*Col.offset(c as isize)).start < 0 as i32) {
            score = 0 as i32;
            cp = &mut *A.offset((*Col.offset(c as isize)).start as isize) as *mut i32;
            new_cp = cp;
            cp_end = cp.offset((*Col.offset(c as isize)).length as isize);
            while cp < cp_end {
                let fresh26 = cp;
                cp = cp.offset(1);
                row = *fresh26;
                if (*Row.offset(row as isize)).shared2.mark < 0 as i32 {
                    continue;
                }
                let fresh27 = new_cp;
                new_cp = new_cp.offset(1);
                *fresh27 = row;
                score += (*Row.offset(row as isize)).shared1.degree - 1 as i32;
                score = if score < n_col { score } else { n_col };
            }
            col_length = new_cp
                .offset_from(
                    &mut *A.offset((*Col.offset(c as isize)).start as isize) as *mut i32,
                ) as i64 as i32;
            if col_length == 0 as i32 {
                n_col2 -= 1;
                (*Col.offset(c as isize)).shared2.order = n_col2;
                (*Col.offset(c as isize)).start = -(1 as i32);
            } else {
                (*Col.offset(c as isize)).length = col_length;
                (*Col.offset(c as isize)).shared2.score = score;
            }
        }
        c -= 1;
        c;
    }
    c = 0 as i32;
    while c <= n_col {
        *head.offset(c as isize) = -(1 as i32);
        c += 1;
        c;
    }
    min_score = n_col;
    c = n_col - 1 as i32;
    while c >= 0 as i32 {
        if (*Col.offset(c as isize)).start >= 0 as i32 {
            score = (*Col.offset(c as isize)).shared2.score;
            next_col = *head.offset(score as isize);
            (*Col.offset(c as isize)).shared3.prev = -(1 as i32);
            (*Col.offset(c as isize)).shared4.degree_next = next_col;
            if next_col != -(1 as i32) {
                (*Col.offset(next_col as isize)).shared3.prev = c;
            }
            *head.offset(score as isize) = c;
            min_score = if min_score < score { min_score } else { score };
        }
        c -= 1;
        c;
    }
    *p_n_col2 = n_col2;
    *p_n_row2 = n_row2;
    *p_max_deg = max_deg;
}
unsafe extern "C" fn find_ordering(
    mut n_row: i32,
    mut n_col: i32,
    mut Alen: i32,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut i32,
    mut head: *mut i32,
    mut n_col2: i32,
    mut max_deg: i32,
    mut pfree: i32,
    mut aggressive: i32,
) -> i32 {
    let mut k: i32 = 0;
    let mut pivot_col: i32 = 0;
    let mut cp: *mut i32 = 0 as *mut i32;
    let mut rp: *mut i32 = 0 as *mut i32;
    let mut pivot_row: i32 = 0;
    let mut new_cp: *mut i32 = 0 as *mut i32;
    let mut new_rp: *mut i32 = 0 as *mut i32;
    let mut pivot_row_start: i32 = 0;
    let mut pivot_row_degree: i32 = 0;
    let mut pivot_row_length: i32 = 0;
    let mut pivot_col_score: i32 = 0;
    let mut needed_memory: i32 = 0;
    let mut cp_end: *mut i32 = 0 as *mut i32;
    let mut rp_end: *mut i32 = 0 as *mut i32;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut max_score: i32 = 0;
    let mut cur_score: i32 = 0;
    let mut hash: u32 = 0;
    let mut head_column: i32 = 0;
    let mut first_col: i32 = 0;
    let mut tag_mark: i32 = 0;
    let mut row_mark: i32 = 0;
    let mut set_difference: i32 = 0;
    let mut min_score: i32 = 0;
    let mut col_thickness: i32 = 0;
    let mut max_mark: i32 = 0;
    let mut pivot_col_thickness: i32 = 0;
    let mut prev_col: i32 = 0;
    let mut next_col: i32 = 0;
    let mut ngarbage: i32 = 0;
    max_mark = 2147483647 as i32 - n_col;
    tag_mark = clear_mark(0 as i32, max_mark, n_row, Row);
    min_score = 0 as i32;
    ngarbage = 0 as i32;
    k = 0 as i32;
    while k < n_col2 {
        while *head.offset(min_score as isize) == -(1 as i32) && min_score < n_col {
            min_score += 1;
            min_score;
        }
        pivot_col = *head.offset(min_score as isize);
        next_col = (*Col.offset(pivot_col as isize)).shared4.degree_next;
        *head.offset(min_score as isize) = next_col;
        if next_col != -(1 as i32) {
            (*Col.offset(next_col as isize)).shared3.prev = -(1 as i32);
        }
        pivot_col_score = (*Col.offset(pivot_col as isize)).shared2.score;
        (*Col.offset(pivot_col as isize)).shared2.order = k;
        pivot_col_thickness = (*Col.offset(pivot_col as isize)).shared1.thickness;
        k += pivot_col_thickness;
        needed_memory = if pivot_col_score < n_col - k {
            pivot_col_score
        } else {
            n_col - k
        };
        if pfree + needed_memory >= Alen {
            pfree = garbage_collection(
                n_row,
                n_col,
                Row,
                Col,
                A,
                &mut *A.offset(pfree as isize),
            );
            ngarbage += 1;
            ngarbage;
            tag_mark = clear_mark(0 as i32, max_mark, n_row, Row);
        }
        pivot_row_start = pfree;
        pivot_row_degree = 0 as i32;
        (*Col.offset(pivot_col as isize)).shared1.thickness = -pivot_col_thickness;
        cp = &mut *A.offset((*Col.offset(pivot_col as isize)).start as isize)
            as *mut i32;
        cp_end = cp.offset((*Col.offset(pivot_col as isize)).length as isize);
        while cp < cp_end {
            let fresh28 = cp;
            cp = cp.offset(1);
            row = *fresh28;
            if (*Row.offset(row as isize)).shared2.mark >= 0 as i32 {
                rp = &mut *A.offset((*Row.offset(row as isize)).start as isize)
                    as *mut i32;
                rp_end = rp.offset((*Row.offset(row as isize)).length as isize);
                while rp < rp_end {
                    let fresh29 = rp;
                    rp = rp.offset(1);
                    col = *fresh29;
                    col_thickness = (*Col.offset(col as isize)).shared1.thickness;
                    if col_thickness > 0 as i32
                        && (*Col.offset(col as isize)).start >= 0 as i32
                    {
                        (*Col.offset(col as isize)).shared1.thickness = -col_thickness;
                        let fresh30 = pfree;
                        pfree = pfree + 1;
                        *A.offset(fresh30 as isize) = col;
                        pivot_row_degree += col_thickness;
                    }
                }
            }
        }
        (*Col.offset(pivot_col as isize)).shared1.thickness = pivot_col_thickness;
        max_deg = if max_deg > pivot_row_degree { max_deg } else { pivot_row_degree };
        cp = &mut *A.offset((*Col.offset(pivot_col as isize)).start as isize)
            as *mut i32;
        cp_end = cp.offset((*Col.offset(pivot_col as isize)).length as isize);
        while cp < cp_end {
            let fresh31 = cp;
            cp = cp.offset(1);
            row = *fresh31;
            (*Row.offset(row as isize)).shared2.mark = -(1 as i32);
        }
        pivot_row_length = pfree - pivot_row_start;
        if pivot_row_length > 0 as i32 {
            pivot_row = *A.offset((*Col.offset(pivot_col as isize)).start as isize);
        } else {
            pivot_row = -(1 as i32);
        }
        rp = &mut *A.offset(pivot_row_start as isize) as *mut i32;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh32 = rp;
            rp = rp.offset(1);
            col = *fresh32;
            col_thickness = -(*Col.offset(col as isize)).shared1.thickness;
            (*Col.offset(col as isize)).shared1.thickness = col_thickness;
            cur_score = (*Col.offset(col as isize)).shared2.score;
            prev_col = (*Col.offset(col as isize)).shared3.prev;
            next_col = (*Col.offset(col as isize)).shared4.degree_next;
            if prev_col == -(1 as i32) {
                *head.offset(cur_score as isize) = next_col;
            } else {
                (*Col.offset(prev_col as isize)).shared4.degree_next = next_col;
            }
            if next_col != -(1 as i32) {
                (*Col.offset(next_col as isize)).shared3.prev = prev_col;
            }
            cp = &mut *A.offset((*Col.offset(col as isize)).start as isize) as *mut i32;
            cp_end = cp.offset((*Col.offset(col as isize)).length as isize);
            while cp < cp_end {
                let fresh33 = cp;
                cp = cp.offset(1);
                row = *fresh33;
                row_mark = (*Row.offset(row as isize)).shared2.mark;
                if row_mark < 0 as i32 {
                    continue;
                }
                set_difference = row_mark - tag_mark;
                if set_difference < 0 as i32 {
                    set_difference = (*Row.offset(row as isize)).shared1.degree;
                }
                set_difference -= col_thickness;
                if set_difference == 0 as i32 && aggressive != 0 {
                    (*Row.offset(row as isize)).shared2.mark = -(1 as i32);
                } else {
                    (*Row.offset(row as isize)).shared2.mark = set_difference + tag_mark;
                }
            }
        }
        rp = &mut *A.offset(pivot_row_start as isize) as *mut i32;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh34 = rp;
            rp = rp.offset(1);
            col = *fresh34;
            hash = 0 as i32 as u32;
            cur_score = 0 as i32;
            cp = &mut *A.offset((*Col.offset(col as isize)).start as isize) as *mut i32;
            new_cp = cp;
            cp_end = cp.offset((*Col.offset(col as isize)).length as isize);
            while cp < cp_end {
                let fresh35 = cp;
                cp = cp.offset(1);
                row = *fresh35;
                row_mark = (*Row.offset(row as isize)).shared2.mark;
                if row_mark < 0 as i32 {
                    continue;
                }
                let fresh36 = new_cp;
                new_cp = new_cp.offset(1);
                *fresh36 = row;
                hash = hash.wrapping_add(row as u32);
                cur_score += row_mark - tag_mark;
                cur_score = if cur_score < n_col { cur_score } else { n_col };
            }
            (*Col.offset(col as isize)).length = new_cp
                .offset_from(
                    &mut *A.offset((*Col.offset(col as isize)).start as isize)
                        as *mut i32,
                ) as i64 as i32;
            if (*Col.offset(col as isize)).length == 0 as i32 {
                (*Col.offset(col as isize)).start = -(1 as i32);
                pivot_row_degree -= (*Col.offset(col as isize)).shared1.thickness;
                (*Col.offset(col as isize)).shared2.order = k;
                k += (*Col.offset(col as isize)).shared1.thickness;
            } else {
                (*Col.offset(col as isize)).shared2.score = cur_score;
                hash = hash.wrapping_rem((n_col + 1 as i32) as u32);
                head_column = *head.offset(hash as isize);
                if head_column > -(1 as i32) {
                    first_col = (*Col.offset(head_column as isize)).shared3.headhash;
                    (*Col.offset(head_column as isize)).shared3.headhash = col;
                } else {
                    first_col = -(head_column + 2 as i32);
                    *head.offset(hash as isize) = -(col + 2 as i32);
                }
                (*Col.offset(col as isize)).shared4.hash_next = first_col;
                (*Col.offset(col as isize)).shared3.hash = hash as i32;
            }
        }
        detect_super_cols(Col, A, head, pivot_row_start, pivot_row_length);
        (*Col.offset(pivot_col as isize)).start = -(1 as i32);
        tag_mark = clear_mark(tag_mark + max_deg + 1 as i32, max_mark, n_row, Row);
        rp = &mut *A.offset(pivot_row_start as isize) as *mut i32;
        new_rp = rp;
        rp_end = rp.offset(pivot_row_length as isize);
        while rp < rp_end {
            let fresh37 = rp;
            rp = rp.offset(1);
            col = *fresh37;
            if (*Col.offset(col as isize)).start < 0 as i32 {
                continue;
            }
            let fresh38 = new_rp;
            new_rp = new_rp.offset(1);
            *fresh38 = col;
            let ref mut fresh39 = (*Col.offset(col as isize)).length;
            let fresh40 = *fresh39;
            *fresh39 = *fresh39 + 1;
            *A.offset(((*Col.offset(col as isize)).start + fresh40) as isize) = pivot_row;
            cur_score = (*Col.offset(col as isize)).shared2.score + pivot_row_degree;
            max_score = n_col - k - (*Col.offset(col as isize)).shared1.thickness;
            cur_score -= (*Col.offset(col as isize)).shared1.thickness;
            cur_score = if cur_score < max_score { cur_score } else { max_score };
            (*Col.offset(col as isize)).shared2.score = cur_score;
            next_col = *head.offset(cur_score as isize);
            (*Col.offset(col as isize)).shared4.degree_next = next_col;
            (*Col.offset(col as isize)).shared3.prev = -(1 as i32);
            if next_col != -(1 as i32) {
                (*Col.offset(next_col as isize)).shared3.prev = col;
            }
            *head.offset(cur_score as isize) = col;
            min_score = if min_score < cur_score { min_score } else { cur_score };
        }
        if pivot_row_degree > 0 as i32 {
            (*Row.offset(pivot_row as isize)).start = pivot_row_start;
            (*Row.offset(pivot_row as isize)).length = new_rp
                .offset_from(&mut *A.offset(pivot_row_start as isize) as *mut i32) as i64
                as i32;
            (*Row.offset(pivot_row as isize)).shared1.degree = pivot_row_degree;
            (*Row.offset(pivot_row as isize)).shared2.mark = 0 as i32;
        }
    }
    return ngarbage;
}
unsafe extern "C" fn order_children(
    mut n_col: i32,
    mut Col: *mut Colamd_Col,
    mut p: *mut i32,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut parent: i32 = 0;
    let mut order: i32 = 0;
    i = 0 as i32;
    while i < n_col {
        if !((*Col.offset(i as isize)).start == -(1 as i32))
            && (*Col.offset(i as isize)).shared2.order == -(1 as i32)
        {
            parent = i;
            loop {
                parent = (*Col.offset(parent as isize)).shared1.parent;
                if (*Col.offset(parent as isize)).start == -(1 as i32) {
                    break;
                }
            }
            c = i;
            order = (*Col.offset(parent as isize)).shared2.order;
            loop {
                let fresh41 = order;
                order = order + 1;
                (*Col.offset(c as isize)).shared2.order = fresh41;
                (*Col.offset(c as isize)).shared1.parent = parent;
                c = (*Col.offset(c as isize)).shared1.parent;
                if !((*Col.offset(c as isize)).shared2.order == -(1 as i32)) {
                    break;
                }
            }
            (*Col.offset(parent as isize)).shared2.order = order;
        }
        i += 1;
        i;
    }
    c = 0 as i32;
    while c < n_col {
        *p.offset((*Col.offset(c as isize)).shared2.order as isize) = c;
        c += 1;
        c;
    }
}
unsafe extern "C" fn detect_super_cols(
    mut Col: *mut Colamd_Col,
    mut A: *mut i32,
    mut head: *mut i32,
    mut row_start: i32,
    mut row_length: i32,
) {
    let mut hash: i32 = 0;
    let mut rp: *mut i32 = 0 as *mut i32;
    let mut c: i32 = 0;
    let mut super_c: i32 = 0;
    let mut cp1: *mut i32 = 0 as *mut i32;
    let mut cp2: *mut i32 = 0 as *mut i32;
    let mut length: i32 = 0;
    let mut prev_c: i32 = 0;
    let mut i: i32 = 0;
    let mut rp_end: *mut i32 = 0 as *mut i32;
    let mut col: i32 = 0;
    let mut head_column: i32 = 0;
    let mut first_col: i32 = 0;
    rp = &mut *A.offset(row_start as isize) as *mut i32;
    rp_end = rp.offset(row_length as isize);
    while rp < rp_end {
        let fresh42 = rp;
        rp = rp.offset(1);
        col = *fresh42;
        if (*Col.offset(col as isize)).start < 0 as i32 {
            continue;
        }
        hash = (*Col.offset(col as isize)).shared3.hash;
        head_column = *head.offset(hash as isize);
        if head_column > -(1 as i32) {
            first_col = (*Col.offset(head_column as isize)).shared3.headhash;
        } else {
            first_col = -(head_column + 2 as i32);
        }
        super_c = first_col;
        while super_c != -(1 as i32) {
            length = (*Col.offset(super_c as isize)).length;
            prev_c = super_c;
            c = (*Col.offset(super_c as isize)).shared4.hash_next;
            while c != -(1 as i32) {
                if (*Col.offset(c as isize)).length != length
                    || (*Col.offset(c as isize)).shared2.score
                        != (*Col.offset(super_c as isize)).shared2.score
                {
                    prev_c = c;
                } else {
                    cp1 = &mut *A.offset((*Col.offset(super_c as isize)).start as isize)
                        as *mut i32;
                    cp2 = &mut *A.offset((*Col.offset(c as isize)).start as isize)
                        as *mut i32;
                    i = 0 as i32;
                    while i < length {
                        let fresh43 = cp1;
                        cp1 = cp1.offset(1);
                        let fresh44 = cp2;
                        cp2 = cp2.offset(1);
                        if *fresh43 != *fresh44 {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    if i != length {
                        prev_c = c;
                    } else {
                        (*Col.offset(super_c as isize)).shared1.thickness
                            += (*Col.offset(c as isize)).shared1.thickness;
                        (*Col.offset(c as isize)).shared1.parent = super_c;
                        (*Col.offset(c as isize)).start = -(2 as i32);
                        (*Col.offset(c as isize)).shared2.order = -(1 as i32);
                        (*Col.offset(prev_c as isize)).shared4.hash_next = (*Col
                            .offset(c as isize))
                            .shared4
                            .hash_next;
                    }
                }
                c = (*Col.offset(c as isize)).shared4.hash_next;
            }
            super_c = (*Col.offset(super_c as isize)).shared4.hash_next;
        }
        if head_column > -(1 as i32) {
            (*Col.offset(head_column as isize)).shared3.headhash = -(1 as i32);
        } else {
            *head.offset(hash as isize) = -(1 as i32);
        }
    }
}
unsafe extern "C" fn garbage_collection(
    mut n_row: i32,
    mut n_col: i32,
    mut Row: *mut Colamd_Row,
    mut Col: *mut Colamd_Col,
    mut A: *mut i32,
    mut pfree: *mut i32,
) -> i32 {
    let mut psrc: *mut i32 = 0 as *mut i32;
    let mut pdest: *mut i32 = 0 as *mut i32;
    let mut j: i32 = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut length: i32 = 0;
    pdest = &mut *A.offset(0 as i32 as isize) as *mut i32;
    c = 0 as i32;
    while c < n_col {
        if (*Col.offset(c as isize)).start >= 0 as i32 {
            psrc = &mut *A.offset((*Col.offset(c as isize)).start as isize) as *mut i32;
            (*Col.offset(c as isize)).start = pdest
                .offset_from(&mut *A.offset(0 as i32 as isize) as *mut i32) as i64
                as i32;
            length = (*Col.offset(c as isize)).length;
            j = 0 as i32;
            while j < length {
                let fresh45 = psrc;
                psrc = psrc.offset(1);
                r = *fresh45;
                if (*Row.offset(r as isize)).shared2.mark >= 0 as i32 {
                    let fresh46 = pdest;
                    pdest = pdest.offset(1);
                    *fresh46 = r;
                }
                j += 1;
                j;
            }
            (*Col.offset(c as isize)).length = pdest
                .offset_from(
                    &mut *A.offset((*Col.offset(c as isize)).start as isize) as *mut i32,
                ) as i64 as i32;
        }
        c += 1;
        c;
    }
    r = 0 as i32;
    while r < n_row {
        if (*Row.offset(r as isize)).shared2.mark < 0 as i32
            || (*Row.offset(r as isize)).length == 0 as i32
        {
            (*Row.offset(r as isize)).shared2.mark = -(1 as i32);
        } else {
            psrc = &mut *A.offset((*Row.offset(r as isize)).start as isize) as *mut i32;
            (*Row.offset(r as isize)).shared2.first_column = *psrc;
            *psrc = -r - 1 as i32;
        }
        r += 1;
        r;
    }
    psrc = pdest;
    while psrc < pfree {
        let fresh47 = psrc;
        psrc = psrc.offset(1);
        if *fresh47 < 0 as i32 {
            psrc = psrc.offset(-1);
            psrc;
            r = -*psrc - 1 as i32;
            *psrc = (*Row.offset(r as isize)).shared2.first_column;
            (*Row.offset(r as isize)).start = pdest
                .offset_from(&mut *A.offset(0 as i32 as isize) as *mut i32) as i64
                as i32;
            length = (*Row.offset(r as isize)).length;
            j = 0 as i32;
            while j < length {
                let fresh48 = psrc;
                psrc = psrc.offset(1);
                c = *fresh48;
                if (*Col.offset(c as isize)).start >= 0 as i32 {
                    let fresh49 = pdest;
                    pdest = pdest.offset(1);
                    *fresh49 = c;
                }
                j += 1;
                j;
            }
            (*Row.offset(r as isize)).length = pdest
                .offset_from(
                    &mut *A.offset((*Row.offset(r as isize)).start as isize) as *mut i32,
                ) as i64 as i32;
        }
    }
    return pdest.offset_from(&mut *A.offset(0 as i32 as isize) as *mut i32) as i64
        as i32;
}
unsafe extern "C" fn clear_mark(
    mut tag_mark: i32,
    mut max_mark: i32,
    mut n_row: i32,
    mut Row: *mut Colamd_Row,
) -> i32 {
    let mut r: i32 = 0;
    if tag_mark <= 0 as i32 || tag_mark >= max_mark {
        r = 0 as i32;
        while r < n_row {
            if (*Row.offset(r as isize)).shared2.mark >= 0 as i32 {
                (*Row.offset(r as isize)).shared2.mark = 0 as i32;
            }
            r += 1;
            r;
        }
        tag_mark = 1 as i32;
    }
    return tag_mark;
}
unsafe extern "C" fn print_report(mut method: *mut i8, mut stats: *mut i32) {
    let mut i1: i32 = 0;
    let mut i2: i32 = 0;
    let mut i3: i32 = 0;
    if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some() {
        glp_printf(
            b"\n%s version %d.%d, %s: \0" as *const u8 as *const i8,
            method,
            2 as i32,
            7 as i32,
            b"Nov 1, 2007\0" as *const u8 as *const i8,
        );
    }
    if stats.is_null() {
        if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some() {
            glp_printf(b"No statistics available.\n\0" as *const u8 as *const i8);
        }
        return;
    }
    i1 = *stats.offset(4 as i32 as isize);
    i2 = *stats.offset(5 as i32 as isize);
    i3 = *stats.offset(6 as i32 as isize);
    if *stats.offset(3 as i32 as isize) >= 0 as i32 {
        if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some() {
            glp_printf(b"OK.  \0" as *const u8 as *const i8);
        }
    } else if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
    {
        glp_printf(b"ERROR.  \0" as *const u8 as *const i8);
    }
    let mut current_block_100: u64;
    match *stats.offset(3 as i32 as isize) {
        1 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Matrix has unsorted or duplicate row indices.\n\0" as *const u8
                        as *const i8,
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"%s: number of duplicate or out-of-order row indices: %d\n\0"
                        as *const u8 as *const i8,
                    method,
                    i3,
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"%s: last seen duplicate or out-of-order row index:   %d\n\0"
                        as *const u8 as *const i8,
                    method,
                    i2,
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"%s: last seen in column:                             %d\0"
                        as *const u8 as *const i8,
                    method,
                    i1,
                );
            }
            current_block_100 = 10013602513651193801;
        }
        0 => {
            current_block_100 = 10013602513651193801;
        }
        -1 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Array A (row indices of matrix) not present.\n\0" as *const u8
                        as *const i8,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -2 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Array p (column pointers for matrix) not present.\n\0" as *const u8
                        as *const i8,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -3 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Invalid number of rows (%d).\n\0" as *const u8 as *const i8,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -4 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Invalid number of columns (%d).\n\0" as *const u8 as *const i8,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -5 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Invalid number of nonzero entries (%d).\n\0" as *const u8
                        as *const i8,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -6 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Invalid column pointer, p [0] = %d, must be zero.\n\0" as *const u8
                        as *const i8,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -7 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(b"Array A too small.\n\0" as *const u8 as *const i8);
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"        Need Alen >= %d, but given only Alen = %d.\n\0"
                        as *const u8 as *const i8,
                    i1,
                    i2,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -8 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Column %d has a negative number of nonzero entries (%d).\n\0"
                        as *const u8 as *const i8,
                    i1,
                    i2,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -9 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"Row index (row %d) out of bounds (%d to %d) in column %d.\n\0"
                        as *const u8 as *const i8,
                    i2,
                    0 as i32,
                    i3 - 1 as i32,
                    i1,
                );
            }
            current_block_100 = 15237655884915618618;
        }
        -10 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(b"Out of memory.\n\0" as *const u8 as *const i8);
            }
            current_block_100 = 15237655884915618618;
        }
        _ => {
            current_block_100 = 15237655884915618618;
        }
    }
    match current_block_100 {
        10013602513651193801 => {
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(b"\n\0" as *const u8 as *const i8);
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"%s: number of dense or empty rows ignored:           %d\n\0"
                        as *const u8 as *const i8,
                    method,
                    *stats.offset(0 as i32 as isize),
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"%s: number of dense or empty columns ignored:        %d\n\0"
                        as *const u8 as *const i8,
                    method,
                    *stats.offset(1 as i32 as isize),
                );
            }
            if (Some(glp_printf as unsafe extern "C" fn(*const i8, ...) -> ())).is_some()
            {
                glp_printf(
                    b"%s: number of garbage collections performed:         %d\n\0"
                        as *const u8 as *const i8,
                    method,
                    *stats.offset(2 as i32 as isize),
                );
            }
        }
        _ => {}
    };
}