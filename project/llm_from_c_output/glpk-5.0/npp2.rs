use std::f64::{MAX, MIN};
use std::collections::LinkedList;

const GLP_BS: i32 = 0;
const GLP_NL: i32 = 1;
const GLP_NU: i32 = 2;
const GLP_NS: i32 = 3;
const GLP_NF: i32 = 4;
const GLP_SOL: i32 = 1;
const GLP_MIP: i32 = 2;

struct NPP {
    sol: i32,
    c_stat: Vec<i32>,
    r_stat: Vec<i32>,
    c_value: Vec<f64>,
    r_pi: Vec<f64>,
    c0: f64,
    stack: Vec<Box<dyn Fn(&mut NPP) -> i32>>,
}

struct NPPROW {
    i: usize,
    lb: f64,
    ub: f64,
}

struct NPPCOL {
    j: usize,
    lb: f64,
    ub: f64,
    is_int: bool,
    coef: f64,
    ptr: Option<Box<NPPAIJ>>,
}

struct NPPAIJ {
    row: Box<NPPROW>,
    val: f64,
    c_next: Option<Box<NPPAIJ>>,
}

struct NPPLFE {
    ref_: usize,
    val: f64,
    next: Option<Box<NPPLFE>>,
}

struct FreeRow {
    p: usize,
}

struct IneqRow {
    p: usize,
    s: usize,
}

struct FreeCol {
    q: usize,
    s: usize,
}

struct BndCol {
    q: usize,
    bnd: f64,
}

struct DbndCol {
    q: usize,
    s: usize,
}

struct FixedCol {
    q: usize,
    s: f64,
    ptr: Option<Box<NPPLFE>>,
}

struct MakeEquality {
    p: usize,
}

struct MakeFixed {
    q: usize,
    c: f64,
    ptr: Option<Box<NPPLFE>>,
}

impl NPP {
    fn new() -> Self {
        NPP {
            sol: 0,
            c_stat: Vec::new(),
            r_stat: Vec::new(),
            c_value: Vec::new(),
            r_pi: Vec::new(),
            c0: 0.0,
            stack: Vec::new(),
        }
    }

    fn push_tse<T: 'static + Fn(&mut NPP) -> i32>(&mut self, func: T) {
        self.stack.push(Box::new(func));
    }

    fn del_row(&mut self, row: NPPROW) {
        // Implementation depends on your data structure
    }

    fn add_col(&mut self) -> NPPCOL {
        NPPCOL {
            j: 0,
            lb: 0.0,
            ub: 0.0,
            is_int: false,
            coef: 0.0,
            ptr: None,
        }
    }

    fn add_aij(&mut self, row: &mut NPPROW, col: &mut NPPCOL, val: f64) {
        let aij = NPPAIJ {
            row: Box::new(NPPROW { i: row.i, lb: row.lb, ub: row.ub }),
            val,
            c_next: col.ptr.take(),
        };
        col.ptr = Some(Box::new(aij));
    }

    fn add_row(&mut self) -> NPPROW {
        NPPROW {
            i: 0,
            lb: 0.0,
            ub: 0.0,
        }
    }

    fn del_col(&mut self, col: NPPCOL) {
        // Implementation depends on your data structure
    }

    fn error(&self) {
        // Handle error
    }
}

fn npp_free_row(npp: &mut NPP, p: &mut NPPROW) {
    assert!(p.lb == MIN && p.ub == MAX);
    let info = FreeRow { p: p.i };
    npp.push_tse(rcv_free_row);
    npp.del_row(NPPROW { i: p.i, lb: p.lb, ub: p.ub });
}

fn rcv_free_row(npp: &mut NPP) -> i32 {
    let info = FreeRow { p: 0 }; // Placeholder
    if npp.sol == GLP_SOL {
        npp.r_stat[info.p] = GLP_BS;
    }
    if npp.sol != GLP_MIP {
        npp.r_pi[info.p] = 0.0;
    }
    0
}

fn npp_geq_row(npp: &mut NPP, p: &mut NPPROW) {
    assert!(p.lb != MIN);
    assert!(p.lb < p.ub);
    let mut s = npp.add_col();
    s.lb = 0.0;
    s.ub = if p.ub == MAX { MAX } else { p.ub - p.lb };
    npp.add_aij(p, &mut s, -1.0);
    let info = IneqRow { p: p.i, s: s.j };
    npp.push_tse(rcv_geq_row);
    p.ub = p.lb;
}

fn rcv_geq_row(npp: &mut NPP) -> i32 {
    let info = IneqRow { p: 0, s: 0 }; // Placeholder
    if npp.sol == GLP_SOL {
        match npp.r_stat[info.p] {
            GLP_BS => match npp.c_stat[info.s] {
                GLP_BS => {
                    npp.error();
                    return 1;
                }
                GLP_NL | GLP_NU => npp.r_stat[info.p] = GLP_BS,
                _ => {
                    npp.error();
                    return 1;
                }
            },
            GLP_NS => match npp.c_stat[info.s] {
                GLP_BS => npp.r_stat[info.p] = GLP_BS,
                GLP_NL => npp.r_stat[info.p] = GLP_NL,
                GLP_NU => npp.r_stat[info.p] = GLP_NU,
                _ => {
                    npp.error();
                    return 1;
                }
            },
            _ => {
                npp.error();
                return 1;
            }
        }
    }
    0
}

fn npp_leq_row(npp: &mut NPP, p: &mut NPPROW) {
    assert!(p.ub != MAX);
    assert!(p.lb < p.ub);
    let mut s = npp.add_col();
    s.lb = 0.0;
    s.ub = if p.lb == MIN { MAX } else { p.ub - p.lb };
    npp.add_aij(p, &mut s, 1.0);
    let info = IneqRow { p: p.i, s: s.j };
    npp.push_tse(rcv_leq_row);
    p.lb = p.ub;
}

fn rcv_leq_row(npp: &mut NPP) -> i32 {
    let info = IneqRow { p: 0, s: 0 }; // Placeholder
    if npp.sol == GLP_SOL {
        match npp.r_stat[info.p] {
            GLP_BS => match npp.c_stat[info.s] {
                GLP_BS => {
                    npp.error();
                    return 1;
                }
                GLP_NL | GLP_NU => npp.r_stat[info.p] = GLP_BS,
                _ => {
                    npp.error();
                    return 1;
                }
            },
            GLP_NS => match npp.c_stat[info.s] {
                GLP_BS => npp.r_stat[info.p] = GLP_BS,
                GLP_NL => npp.r_stat[info.p] = GLP_NU,
                GLP_NU => npp.r_stat[info.p] = GLP_NL,
                _ => {
                    npp.error();
                    return 1;
                }
            },
            _ => {
                npp.error();
                return 1;
            }
        }
    }
    0
}

fn npp_free_col(npp: &mut NPP, q: &mut NPPCOL) {
    assert!(q.lb == MIN && q.ub == MAX);
    q.lb = 0.0;
    q.ub = MAX;
    let mut s = npp.add_col();
    s.is_int = q.is_int;
    s.lb = 0.0;
    s.ub = MAX;
    s.coef = -q.coef;
    let mut aij = q.ptr.take();
    while let Some(mut aij_node) = aij {
        npp.add_aij(&mut aij_node.row, &mut s, -aij_node.val);
        aij = aij_node.c_next;
    }
    let info = FreeCol { q: q.j, s: s.j };
    npp.push_tse(rcv_free_col);
}

fn rcv_free_col(npp: &mut NPP) -> i32 {
    let info = FreeCol { q: 0, s: 0 }; // Placeholder
    if npp.sol == GLP_SOL {
        match npp.c_stat[info.q] {
            GLP_BS => match npp.c_stat[info.s] {
                GLP_BS => {
                    npp.error();
                    return 1;
                }
                GLP_NL => npp.c_stat[info.q] = GLP_BS,
                _ => {
                    npp.error();
                    return -1;
                }
            },
            GLP_NL => match npp.c_stat[info.s] {
                GLP_BS => npp.c_stat[info.q] = GLP_BS,
                GLP_NL => npp.c_stat[info.q] = GLP_NF,
                _ => {
                    npp.error();
                    return -1;
                }
            },
            _ => {
                npp.error();
                return -1;
            }
        }
    }
    npp.c_value[info.q] -= npp.c_value[info.s];
    0
}

fn npp_lbnd_col(npp: &mut NPP, q: &mut NPPCOL) {
    assert!(q.lb != 0.0);
    assert!(q.lb != MIN);
    assert!(q.lb < q.ub);
    let info = BndCol { q: q.j, bnd: q.lb };
    npp.push_tse(rcv_lbnd_col);
    npp.c0 += q.coef * q.lb;
    let mut aij = q.ptr.take();
    while let Some(mut aij_node) = aij {
        let row = &mut aij_node.row;
        if row.lb == row.ub {
            row.ub = row.lb - aij_node.val * q.lb;
        } else {
            if row.lb != MIN {
                row.lb -= aij_node.val * q.lb;
            }
            if row.ub != MAX {
                row.ub -= aij_node.val * q.lb;
            }
        }
        aij = aij_node.c_next;
    }
    if q.ub != MAX {
        q.ub -= q.lb;
    }
    q.lb = 0.0;
}

fn rcv_lbnd_col(npp: &mut NPP) -> i32 {
    let info = BndCol { q: 0, bnd: 0.0 }; // Placeholder
    if npp.sol == GLP_SOL {
        match npp.c_stat[info.q] {
            GLP_BS | GLP_NL | GLP_NU => (),
            _ => {
                npp.error();
                return 1;
            }
        }
    }
    npp.c_value[info.q] = info.bnd + npp.c_value[info.q];
    0
}

fn npp_ubnd_col(npp: &mut NPP, q: &mut NPPCOL) {
    assert!(q.ub != MAX);
    assert!(q.lb < q.ub);
    let info = BndCol { q: q.j, bnd: q.ub };
    npp.push_tse(rcv_ubnd_col);
    npp.c0 += q.coef * q.ub;
    q.coef = -q.coef;
    let mut aij = q.ptr.take();
    while let Some(mut aij_node) = aij {
        let row = &mut aij_node.row;
        if row.lb == row.ub {
            row.ub = row.lb - aij_node.val * q.ub;
        } else {
            if row.lb != MIN {
                row.lb -= aij_node.val * q.ub;
            }
            if row.ub != MAX {
                row.ub -= aij_node.val * q.ub;
            }
        }
        aij_node.val = -aij_node.val;
        aij = aij_node.c_next;
    }
    if q.lb != MIN {
        q.ub -= q.lb;
    } else {
        q.ub = MAX;
    }
    q.lb = 0.0;
}

fn rcv_ubnd_col(npp: &mut NPP) -> i32 {
    let info = BndCol { q: 0, bnd: 0.0 }; // Placeholder
    if npp.sol == GLP_BS {
        match npp.c_stat[info.q] {
            GLP_BS => npp.c_stat[info.q] = GLP_BS,
            GLP_NL => npp.c_stat[info.q] = GLP_NU,
            GLP_NU => npp.c_stat[info.q] = GLP_NL,
            _ => {
                npp.error();
                return 1;
            }
        }
    }
    npp.c_value[info.q] = info.bnd - npp.c_value[info.q];
    0
}

fn npp_dbnd_col(npp: &mut NPP, q: &mut NPPCOL) {
    assert!(q.lb == 0.0);
    assert!(q.ub > 0.0);
    assert!(q.ub != MAX);
    let mut s = npp.add_col();
    s.is_int = q.is_int;
    s.lb = 0.0;
    s.ub = MAX;
    let mut p = npp.add_row();
    p.lb = q.ub;
    p.ub = q.ub;
    npp.add_aij(&mut p, q, 1.0);
    npp.add_aij(&mut p, &mut s, 1.0);
    let info = DbndCol { q: q.j, s: s.j };
    npp.push_tse(rcv_dbnd_col);
    q.ub = MAX;
}

fn rcv_dbnd_col(npp: &mut NPP) -> i32 {
    let info = DbndCol { q: 0, s: 0 }; // Placeholder
    if npp.sol == GLP_BS {
        match npp.c_stat[info.q] {
            GLP_BS => match npp.c_stat[info.s] {
                GLP_BS => npp.c_stat[info.q] = GLP_BS,
                GLP_NL => npp.c_stat[info.q] = GLP_NU,
                _ => {
                    npp.error();
                    return 1;
                }
            },
            GLP_NL => match npp.c_stat[info.s] {
                GLP_BS | GLP_NL => npp.c_stat[info.q] = GLP_NL,
                _ => {
                    npp.error();
                    return 1;
                }
            },
            _ => {
                npp.error();
                return 1;
            }
        }
    }
    0
}

fn npp_fixed_col(npp: &mut NPP, q: &mut NPPCOL) {
    assert!(q.lb == q.ub);
    let info = FixedCol {
        q: q.j,
        s: q.lb,
        ptr: None,
    };
    npp.push_tse(rcv_fixed_col);
    npp.c0 += q.coef * q.lb;
    let mut aij = q.ptr.take();
    while let Some(mut aij_node) = aij {
        let row = &mut aij_node.row;
        if row.lb == row.ub {
            row.ub = row.lb - aij_node.val * q.lb;
        } else {
            if row.lb != MIN {
                row.lb -= aij_node.val * q.lb;
            }
            if row.ub != MAX {
                row.ub -= aij_node.val * q.lb;
            }
        }
        aij = aij_node.c_next;
    }
    npp.del_col(NPPCOL {
        j: q.j,
        lb: q.lb,
        ub: q.ub,
        is_int: q.is_int,
        coef: q.coef,
        ptr: None,
    });
}

fn rcv_fixed_col(npp: &mut NPP) -> i32 {
    let info = FixedCol {
        q: 0,
        s: 0.0,
        ptr: None,
    }; // Placeholder
    if npp.sol == GLP_SOL {
        npp.c_stat[info.q] = GLP_NS;
    }
    npp.c_value[info.q] = info.s;
    0
}

fn npp_make_equality(npp: &mut NPP, p: &mut NPPROW) -> i32 {
    assert!(p.lb != MIN);
    assert!(p.ub != MAX);
    assert!(p.lb < p.ub);
    let eps = 1e-9 + 1e-12 * p.lb.abs();
    if p.ub - p.lb > eps {
        return 0;
    }
    let info = MakeEquality { p: p.i };
    npp.push_tse(rcv_make_equality);
    let b = 0.5 * (p.ub + p.lb);
    let nint = (b + 0.5).floor();
    if (b - nint).abs() <= eps {
        p.lb = nint;
