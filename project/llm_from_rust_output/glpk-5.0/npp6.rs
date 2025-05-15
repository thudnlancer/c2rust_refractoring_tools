use std::ptr;
use std::mem;
use std::ffi::CString;
use libc::{c_int, c_double, c_char, c_void};

const DBL_MAX: c_double = 1.7976931348623157e+308;

struct DMP;

#[derive(Clone, Copy)]
struct NPP {
    orig_dir: c_int,
    orig_m: c_int,
    orig_n: c_int,
    orig_nnz: c_int,
    pool: *mut DMP,
    name: *mut c_char,
    obj: *mut c_char,
    c0: c_double,
    nrows: c_int,
    ncols: c_int,
    r_head: *mut NPPROW,
    r_tail: *mut NPPROW,
    c_head: *mut NPPCOL,
    c_tail: *mut NPPCOL,
    stack: *mut DMP,
    top: *mut NPPTSE,
    m: c_int,
    n: c_int,
    nnz: c_int,
    row_ref: *mut c_int,
    col_ref: *mut c_int,
    sol: c_int,
    scaling: c_int,
    p_stat: c_int,
    d_stat: c_int,
    t_stat: c_int,
    i_stat: c_int,
    r_stat: *mut c_char,
    c_stat: *mut c_char,
    r_pi: *mut c_double,
    c_value: *mut c_double,
}

#[derive(Clone, Copy)]
struct NPPTSE {
    func: Option<unsafe extern "C" fn(*mut NPP, *mut c_void) -> c_int>,
    info: *mut c_void,
    link: *mut NPPTSE,
}

#[derive(Clone, Copy)]
struct NPPCOL {
    j: c_int,
    name: *mut c_char,
    is_int: c_char,
    lb: c_double,
    ub: c_double,
    coef: c_double,
    ptr: *mut NPPAIJ,
    temp: c_int,
    ll: C2RustUnnamed_0,
    uu: C2RustUnnamed,
    prev: *mut NPPCOL,
    next: *mut NPPCOL,
}

#[derive(Clone, Copy)]
union C2RustUnnamed {
    uu: c_double,
    neg: c_int,
}

#[derive(Clone, Copy)]
union C2RustUnnamed_0 {
    ll: c_double,
    pos: c_int,
}

#[derive(Clone, Copy)]
struct NPPAIJ {
    row: *mut NPPROW,
    col: *mut NPPCOL,
    val: c_double,
    r_prev: *mut NPPAIJ,
    r_next: *mut NPPAIJ,
    c_prev: *mut NPPAIJ,
    c_next: *mut NPPAIJ,
}

#[derive(Clone, Copy)]
struct NPPROW {
    i: c_int,
    name: *mut c_char,
    lb: c_double,
    ub: c_double,
    ptr: *mut NPPAIJ,
    temp: c_int,
    prev: *mut NPPROW,
    next: *mut NPPROW,
}

#[derive(Clone, Copy)]
struct sat_fixed_col {
    q: c_int,
    s: c_int,
}

#[derive(Clone, Copy)]
struct NPPLIT {
    col: *mut NPPCOL,
    neg: c_int,
}

#[derive(Clone, Copy)]
struct NPPLSE {
    lit: NPPLIT,
    next: *mut NPPLSE,
}

#[derive(Clone, Copy)]
struct NPPSED {
    x: NPPLIT,
    y: NPPLIT,
    z: NPPLIT,
    s: *mut NPPCOL,
    c: *mut NPPCOL,
}

fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int) {
    unsafe {
        assert!(!expr.is_null() && !file.is_null());
    }
}

fn _glp_npp_sat_free_row(npp: *mut NPP, p: *mut NPPROW) {
    unsafe {
        assert!(!npp.is_null() && !p.is_null());
        assert!(
            ((*p).lb == -DBL_MAX && (*p).ub == DBL_MAX),
            "p->lb == -DBL_MAX && p->ub == +DBL_MAX"
        );
        _glp_npp_del_row(npp, p);
    }
}

fn _glp_npp_sat_fixed_col(npp: *mut NPP, q: *mut NPPCOL) -> c_int {
    unsafe {
        assert!(!npp.is_null() && !q.is_null());
        assert!((*q).lb == (*q).ub, "q->lb == q->ub");

        let info = _glp_npp_push_tse(
            npp,
            Some(rcv_sat_fixed_col),
            mem::size_of::<sat_fixed_col>() as c_int,
        ) as *mut sat_fixed_col;

        (*info).q = (*q).j;
        (*info).s = (*q).lb as c_int;
        assert!(
            (*info).s as c_double == (*q).lb,
            "(double)info->s == q->lb"
        );

        if (*info).s != 0 {
            let mut aij = (*q).ptr;
            while !aij.is_null() {
                let i = (*aij).row;
                if (*i).lb != -DBL_MAX {
                    (*i).lb -= (*aij).val * (*info).s as c_double;
                    let temp = (*i).lb as c_int;
                    if temp as c_double != (*i).lb {
                        return 1;
                    }
                }
                if (*i).ub != DBL_MAX {
                    (*i).ub -= (*aij).val * (*info).s as c_double;
                    let temp = (*i).ub as c_int;
                    if temp as c_double != (*i).ub {
                        return 2;
                    }
                }
                aij = (*aij).c_next;
            }
        }

        _glp_npp_del_col(npp, q);
        0
    }
}

unsafe extern "C" fn rcv_sat_fixed_col(npp: *mut NPP, info_: *mut c_void) -> c_int {
    let info = info_ as *mut sat_fixed_col;
    *(*npp).c_value.offset((*info).q as isize) = (*info).s as c_double;
    0
}

fn _glp_npp_sat_is_bin_comb(npp: *mut NPP, row: *mut NPPROW) -> c_int {
    unsafe {
        assert!(!npp.is_null() && !row.is_null());
        let mut aij = (*row).ptr;
        while !aij.is_null() {
            if (*aij).val != 1.0 && (*aij).val != -1.0 {
                return 0;
            }
            let col = (*aij).col;
            if (*col).is_int == 0 || (*col).lb != 0.0 || (*col).ub != 1.0 {
                return 0;
            }
            aij = (*aij).r_next;
        }
        1
    }
}

// ... (其他函数类似转换)

fn _glp_npp_sat_encode_prob(npp: *mut NPP) -> c_int {
    unsafe {
        assert!(!npp.is_null());
        let mut row = (*npp).r_head;
        while !row.is_null() {
            let next_row = (*row).next;
            if (*row).lb == -DBL_MAX && (*row).ub == DBL_MAX {
                _glp_npp_sat_free_row(npp, row);
            }
            row = next_row;
        }

        let mut col = (*npp).c_head;
        while !col.is_null() {
            let next_col = (*col).next;
            if (*col).lb == (*col).ub {
                assert!(
                    _glp_npp_sat_fixed_col(npp, col) == 0,
                    "npp_sat_fixed_col(npp, col) == 0"
                );
            }
            col = next_col;
        }

        col = (*npp).c_head;
        while !col.is_null() {
            assert!(
                (*col).is_int != 0 && (*col).lb == 0.0 && (*col).ub == 1.0,
                "col->is_int && col->lb == 0.0 && col->ub == 1.0"
            );
            col = (*col).next;
        }

        let mut ret = 0;
        let mut cover = 0;
        let mut pack = 0;
        let mut partn = 0;
        let mut row = (*npp).r_tail;
        while !row.is_null() {
            let prev_row = (*row).prev;
            let mut current_ret = _glp_npp_sat_is_cover_ineq(npp, row);
            if current_ret != 0 {
                cover += 1;
                if current_ret == 2 {
                    assert!(
                        _glp_npp_sat_reverse_row(npp, row) == 0,
                        "npp_sat_reverse_row(npp, row) == 0"
                    );
                    current_ret = _glp_npp_sat_is_cover_ineq(npp, row);
                }
                assert!(current_ret == 1, "ret == 1");
            } else {
                current_ret = _glp_npp_sat_is_partn_eq(npp, row);
                if current_ret != 0 {
                    partn += 1;
                    if current_ret == 2 {
                        assert!(
                            _glp_npp_sat_reverse_row(npp, row) == 0,
                            "npp_sat_reverse_row(npp, row) == 0"
                        );
                        current_ret = _glp_npp_sat_is_partn_eq(npp, row);
                    }
                    assert!(current_ret == 1, "ret == 1");
                    // ... (处理partitioning equalities)
                }
                current_ret = _glp_npp_sat_is_pack_ineq(npp, row);
                if current_ret != 0 {
                    pack += 1;
                    if current_ret == 2 {
                        assert!(
                            _glp_npp_sat_reverse_row(npp, row) == 0,
                            "npp_sat_reverse_row(npp, row) == 0"
                        );
                        current_ret = _glp_npp_sat_is_pack_ineq(npp, row);
                    }
                    assert!(current_ret == 1, "ret == 1");
                    // ... (处理packing inequalities)
                } else {
                    current_ret = _glp_npp_sat_encode_row(npp, row);
                    if current_ret != 0 {
                        if current_ret == 1 {
                            ret = 0xa;
                        } else if current_ret == 2 {
                            ret = 0x13;
                        } else {
                            panic!("Unexpected return value");
                        }
                        return ret;
                    }
                }
            }
            row = prev_row;
        }

        if cover != 0 {
            println!("{} covering inequalities", cover);
        }
        if pack != 0 {
            println!("{} packing inequalities", pack);
        }
        if partn != 0 {
            println!("{} partitioning equalities", partn);
        }
        ret
    }
}