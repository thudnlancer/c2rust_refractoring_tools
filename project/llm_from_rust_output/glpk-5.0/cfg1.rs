use std::cmp::Ordering;
use std::ptr;

type SizeT = usize;
type CInt = i32;
type CDouble = f64;
type CVoid = std::ffi::c_void;
type CChar = i8;
type CUChar = u8;

struct Term {
    ind: CInt,
    val: CDouble,
}

struct GlpRow {
    i: CInt,
    name: *mut CChar,
    node: *mut CVoid,
    level: CInt,
    origin: CUChar,
    klass: CUChar,
    type_: CInt,
    lb: CDouble,
    ub: CDouble,
    ptr: *mut CVoid,
    rii: CDouble,
    stat: CInt,
    bind: CInt,
    prim: CDouble,
    dual: CDouble,
    pval: CDouble,
    dval: CDouble,
    mipx: CDouble,
}

struct GlpCol {
    j: CInt,
    name: *mut CChar,
    node: *mut CVoid,
    kind: CInt,
    type_: CInt,
    lb: CDouble,
    ub: CDouble,
    coef: CDouble,
    ptr: *mut CVoid,
    sjj: CDouble,
    stat: CInt,
    bind: CInt,
    prim: CDouble,
    dual: CDouble,
    pval: CDouble,
    dval: CDouble,
    mipx: CDouble,
}

struct GlpProb {
    pool: *mut CVoid,
    tree: *mut CVoid,
    name: *mut CChar,
    obj: *mut CChar,
    dir: CInt,
    c0: CDouble,
    m_max: CInt,
    n_max: CInt,
    m: CInt,
    n: CInt,
    nnz: CInt,
    row: *mut *mut GlpRow,
    col: *mut *mut GlpCol,
    r_tree: *mut CVoid,
    c_tree: *mut CVoid,
    valid: CInt,
    head: *mut CInt,
    bfd: *mut CVoid,
    pbs_stat: CInt,
    dbs_stat: CInt,
    obj_val: CDouble,
    it_cnt: CInt,
    some: CInt,
    ipt_stat: CInt,
    ipt_obj: CDouble,
    mip_stat: CInt,
    mip_obj: CDouble,
}

struct Cfg {
    n: CInt,
    pos: *mut CInt,
    neg: *mut CInt,
    pool: *mut CVoid,
    nv_max: CInt,
    nv: CInt,
    ref_: *mut CInt,
    vptr: *mut *mut CVoid,
    cptr: *mut *mut CVoid,
}

struct Csa {
    P: *mut GlpProb,
    G: *mut Cfg,
    ind: *mut CInt,
    nn: CInt,
    vtoi: *mut CInt,
    itov: *mut CInt,
    wgt: *mut CDouble,
}

fn fcmp(t1: &Term, t2: &Term) -> Ordering {
    t1.val.partial_cmp(&t2.val).unwrap_or(Ordering::Equal).reverse()
}

fn analyze_ineq(
    P: &GlpProb,
    G: &mut Cfg,
    len: CInt,
    ind: &mut [CInt],
    val: &mut [CDouble],
    rhs: CDouble,
    t: &mut [Term],
) {
    let mut new_len = 0;
    let mut rhs = rhs;

    for k in 1..=len as usize {
        let j = ind[k] as usize;
        unsafe {
            let col = *(*P.col).add(j);
            if col.kind == 2 && col.type_ == 4 && col.lb == 0.0 && col.ub == 1.0 {
                new_len += 1;
                ind[new_len] = ind[k];
                val[new_len] = val[k];
            } else if val[k] > 0.0 {
                let type_ = col.type_;
                if type_ == 1 || type_ == 3 {
                    return;
                } else {
                    rhs -= val[k] * col.lb;
                }
            } else {
                let type_ = col.type_;
                if type_ == 1 || type_ == 2 {
                    return;
                } else {
                    rhs -= val[k] * col.ub;
                }
            }
        }
    }

    let len = new_len;
    if len <= 1 {
        return;
    }

    for k in 1..=len as usize {
        if val[k] < 0.0 {
            ind[k] = -ind[k];
            val[k] = -val[k];
            rhs += val[k];
        }
    }

    rhs += 0.001 * (1.0 + rhs.abs());

    let p = (1..=len as usize).max_by(|&a, &b| val[a].partial_cmp(&val[b]).unwrap()).unwrap();
    let q = (1..=len as usize)
        .filter(|&k| k != p)
        .max_by(|&a, &b| val[a].partial_cmp(&val[b]).unwrap())
        .unwrap();

    if val[p] + val[q] > rhs {
        unsafe {
            _glp_cfg_add_clique(G, len, ind.as_ptr());
        }
    } else {
        let p = (1..=len as usize).min_by(|&a, &b| val[a].partial_cmp(&val[b]).unwrap()).unwrap();
        let q = (1..=len as usize)
            .filter(|&k| k != p)
            .min_by(|&a, &b| val[a].partial_cmp(&val[b]).unwrap())
            .unwrap();

        if val[p] + val[q] <= rhs {
            if len < 3 {
                return;
            }

            for k in 1..=len as usize {
                t[k].ind = ind[k];
                t[k].val = val[k];
            }

            t[1..=len as usize].sort_by(fcmp);

            for k in 1..=len as usize {
                ind[k] = t[k].ind;
                val[k] = t[k].val;
            }

            let p = (2..len as usize)
                .find(|&p| val[p] + val[p + 1] <= rhs)
                .unwrap();

            unsafe {
                _glp_cfg_add_clique(G, p as CInt, ind.as_ptr());
            }

            for k in 1..=p {
                for kk in p..=len as usize {
                    if k != kk && val[k] + val[kk] > rhs {
                        let iii = [ind[k], ind[kk]];
                        unsafe {
                            _glp_cfg_add_clique(G, 2, iii.as_ptr());
                        }
                    }
                }
            }
        }
    }
}

fn _glp_cfg_build_graph(P: &mut GlpProb) -> *mut Cfg {
    let m = P.m;
    let n = P.n;
    let G = unsafe { _glp_cfg_create_graph(n, 2 * glp_get_num_bin(P)) };

    let mut ind = vec![0; (n + 1) as usize];
    let mut val = vec![0.0; (n + 1) as usize];
    let mut t = vec![Term { ind: 0, val: 0.0 }; (n + 1) as usize];

    for i in 1..=m {
        unsafe {
            let row = *(*P.row).add(i as usize);
            let type_ = row.type_;

            if type_ == 2 || type_ == 4 || type_ == 5 {
                let len = glp_get_mat_row(P, i, ind.as_mut_ptr(), val.as_mut_ptr());
                for k in 1..=len as usize {
                    val[k] = -val[k];
                }
                analyze_ineq(
                    P,
                    &mut *G,
                    len,
                    ind.as_mut_slice(),
                    val.as_mut_slice(),
                    -row.lb,
                    t.as_mut_slice(),
                );
            }

            if type_ == 3 || type_ == 4 || type_ == 5 {
                let len = glp_get_mat_row(P, i, ind.as_mut_ptr(), val.as_mut_ptr());
                analyze_ineq(
                    P,
                    &mut *G,
                    len,
                    ind.as_mut_slice(),
                    val.as_mut_slice(),
                    row.ub,
                    t.as_mut_slice(),
                );
            }
        }
    }

    G
}

// Placeholder for external functions
extern "C" {
    fn _glp_cfg_create_graph(n: CInt, nv_max: CInt) -> *mut Cfg;
    fn _glp_cfg_add_clique(G: *mut Cfg, size: CInt, ind: *const CInt);
    fn glp_get_num_bin(P: *const GlpProb) -> CInt;
    fn glp_get_mat_row(
        P: *mut GlpProb,
        i: CInt,
        ind: *mut CInt,
        val: *mut CDouble,
    ) -> CInt;
    fn glp_alloc(n: CInt, size: CInt) -> *mut CVoid;
    fn glp_free(ptr: *mut CVoid);
}