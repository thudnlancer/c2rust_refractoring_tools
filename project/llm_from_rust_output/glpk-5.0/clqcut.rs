use std::ptr;

struct DMP;
struct AVL;
struct AVLNODE;
struct BFD;
struct glp_tree;

struct CFG {
    n: i32,
    pos: Vec<i32>,
    neg: Vec<i32>,
    pool: *mut DMP,
    nv_max: i32,
    nv: i32,
    ref_: Vec<i32>,
    vptr: Vec<*mut CFGVLE>,
    cptr: Vec<*mut CFGCLE>,
}

struct CFGCLE {
    vptr: *mut CFGVLE,
    next: *mut CFGCLE,
}

struct CFGVLE {
    v: i32,
    next: *mut CFGVLE,
}

struct glp_prob {
    pool: *mut DMP,
    tree: *mut glp_tree,
    name: *mut libc::c_char,
    obj: *mut libc::c_char,
    dir: i32,
    c0: f64,
    m_max: i32,
    n_max: i32,
    m: i32,
    n: i32,
    nnz: i32,
    row: Vec<*mut GLPROW>,
    col: Vec<*mut GLPCOL>,
    r_tree: *mut AVL,
    c_tree: *mut AVL,
    valid: i32,
    head: *mut i32,
    bfd: *mut BFD,
    pbs_stat: i32,
    dbs_stat: i32,
    obj_val: f64,
    it_cnt: i32,
    some: i32,
    ipt_stat: i32,
    ipt_obj: f64,
    mip_stat: i32,
    mip_obj: f64,
}

struct GLPCOL {
    j: i32,
    name: *mut libc::c_char,
    node: *mut AVLNODE,
    kind: i32,
    type_: i32,
    lb: f64,
    ub: f64,
    coef: f64,
    ptr: *mut GLPAIJ,
    sjj: f64,
    stat: i32,
    bind: i32,
    prim: f64,
    dual: f64,
    pval: f64,
    dval: f64,
    mipx: f64,
}

struct GLPAIJ {
    row: *mut GLPROW,
    col: *mut GLPCOL,
    val: f64,
    r_prev: *mut GLPAIJ,
    r_next: *mut GLPAIJ,
    c_prev: *mut GLPAIJ,
    c_next: *mut GLPAIJ,
}

struct GLPROW {
    i: i32,
    name: *mut libc::c_char,
    node: *mut AVLNODE,
    level: i32,
    origin: u8,
    klass: u8,
    type_: i32,
    lb: f64,
    ub: f64,
    ptr: *mut GLPAIJ,
    rii: f64,
    stat: i32,
    bind: i32,
    prim: f64,
    dual: f64,
    pval: f64,
    dval: f64,
    mipx: f64,
}

fn glp_clq_cut(P: &mut glp_prob, G: &mut CFG, ind: &mut [i32], val: &mut [f64]) -> i32 {
    assert_eq!(G.n, P.n, "G->n == n");

    let n = P.n;
    let pos = &G.pos;
    let neg = &G.neg;
    let nv = G.nv;
    let ref_ = &G.ref_;

    let mut sum = 0.0;
    let mut len = unsafe { _glp_cfg_find_clique(P as *mut _ as *mut libc::c_void, G, ind.as_mut_ptr(), &mut sum) };
    
    if sum < 1.07 {
        return 0;
    }

    len = unsafe { _glp_cfg_expand_clique(G, len, ind.as_mut_ptr()) };
    let mut rhs = 1.0;

    val[1..=n as usize].fill(0.0);

    for k in 1..=len {
        let v = ind[k as usize];
        assert!((1..=nv).contains(&v), "1 <= v && v <= nv";

        let j = ref_[v as usize];
        assert!((1..=n).contains(&j), "1 <= j && j <= n";

        if pos[j as usize] == v {
            unsafe {
                if (*(*P.col.as_ptr().offset(j as isize)).type_ == 5 {
                    rhs -= (*(*P.col.as_ptr().offset(j as isize))).prim;
                } else {
                    val[j as usize] += 1.0;
                }
            }
        } else if neg[j as usize] == v {
            unsafe {
                if (*(*P.col.as_ptr().offset(j as isize)).type_ == 5 {
                    rhs -= 1.0 - (*(*P.col.as_ptr().offset(j as isize))).prim;
                } else {
                    val[j as usize] -= 1.0;
                    rhs -= 1.0;
                }
            }
        } else {
            panic!("v != v");
        }
    }

    let mut new_len = 0;
    for j in 1..=n {
        if val[j as usize] != 0.0 {
            new_len += 1;
            ind[new_len as usize] = j;
            val[new_len as usize] = val[j as usize];
        }
    }

    ind[0] = 0;
    val[0] = rhs;
    new_len
}

// These would need to be implemented or wrapped safely
extern "C" {
    fn _glp_cfg_find_clique(
        P: *mut libc::c_void,
        G: *mut CFG,
        ind: *mut i32,
        sum: *mut f64,
    ) -> i32;
    fn _glp_cfg_expand_clique(
        G: *mut CFG,
        c_len: i32,
        c_ind: *mut i32,
    ) -> i32;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: i32,
    );
}