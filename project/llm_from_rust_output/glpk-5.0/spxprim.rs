use std::f64;
use std::ptr;
use std::mem;

struct FVS {
    n: i32,
    nnz: i32,
    ind: *mut i32,
    vec: *mut f64,
}

struct GLPCOL {
    j: i32,
    name: *mut i8,
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
    name: *mut i8,
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

struct glp_smcp {
    msg_lev: i32,
    meth: i32,
    pricing: i32,
    r_test: i32,
    tol_bnd: f64,
    tol_dj: f64,
    tol_piv: f64,
    obj_ll: f64,
    obj_ul: f64,
    it_lim: i32,
    tm_lim: i32,
    out_frq: i32,
    out_dly: i32,
    presolve: i32,
    excl: i32,
    shift: i32,
    aorn: i32,
    foo_bar: [f64; 33],
}

struct csa {
    lp: *mut SPXLP,
    dir: i32,
    fz: f64,
    orig_c: *mut f64,
    orig_l: *mut f64,
    orig_u: *mut f64,
    at: *mut SPXAT,
    nt: *mut SPXNT,
    phase: i32,
    beta: *mut f64,
    beta_st: i32,
    d: *mut f64,
    d_st: i32,
    se: *mut SPXSE,
    num: i32,
    list: *mut i32,
    q: i32,
    tcol: FVS,
    bp: *mut SPXBP,
    p: i32,
    p_flag: i32,
    trow: FVS,
    work: FVS,
    p_stat: i32,
    d_stat: i32,
    msg_lev: i32,
    r_test: i32,
    tol_bnd: f64,
    tol_bnd1: f64,
    tol_dj: f64,
    tol_dj1: f64,
    tol_piv: f64,
    it_lim: i32,
    tm_lim: i32,
    out_frq: i32,
    out_dly: i32,
    tm_beg: f64,
    it_beg: i32,
    it_cnt: i32,
    it_dpy: i32,
    tm_dpy: f64,
    inv_cnt: i32,
    degen: i32,
    ns_cnt: i32,
    ls_cnt: i32,
}

struct SPXBP {
    i: i32,
    teta: f64,
    dc: f64,
    dz: f64,
}

struct SPXSE {
    valid: i32,
    refsp: *mut i8,
    gamma: *mut f64,
    work: *mut f64,
}

struct SPXNT {
    ptr: *mut i32,
    len: *mut i32,
    ind: *mut i32,
    val: *mut f64,
}

struct SPXAT {
    ptr: *mut i32,
    ind: *mut i32,
    val: *mut f64,
    work: *mut f64,
}

struct SPXLP {
    m: i32,
    n: i32,
    nnz: i32,
    A_ptr: *mut i32,
    A_ind: *mut i32,
    A_val: *mut f64,
    b: *mut f64,
    c: *mut f64,
    l: *mut f64,
    u: *mut f64,
    head: *mut i32,
    flag: *mut i8,
    valid: i32,
    bfd: *mut BFD,
}

fn set_penalty(csa: &mut csa, tol: f64, tol1: f64) -> i32 {
    let lp = unsafe { &mut *csa.lp };
    let m = lp.m;
    let n = lp.n;
    let c = unsafe { &mut *lp.c };
    let l = unsafe { &mut *lp.l };
    let u = unsafe { &mut *lp.u };
    let head = unsafe { &mut *lp.head };
    let beta = unsafe { &mut *csa.beta };
    
    let mut count = 0;
    
    for k in 0..=n {
        unsafe { *c.offset(k as isize) = 0.0; }
    }
    
    for i in 1..=m {
        let k = unsafe { *head.offset(i as isize) };
        let t = unsafe { *l.offset(k as isize) };
        
        if t != f64::MIN {
            let eps = tol + tol1 * t.abs();
            if unsafe { *beta.offset(i as isize) } < t - eps {
                unsafe { *c.offset(k as isize) = -1.0; }
                count += 1;
            }
        }
        
        let t = unsafe { *u.offset(k as isize) };
        if t != f64::MAX {
            let eps = tol + tol1 * t.abs();
            if unsafe { *beta.offset(i as isize) } > t + eps {
                unsafe { *c.offset(k as isize) = 1.0; }
                count += 1;
            }
        }
    }
    
    count
}

fn check_feas(csa: &mut csa, phase: i32, tol: f64, tol1: f64) -> i32 {
    let lp = unsafe { &mut *csa.lp };
    let m = lp.m;
    let c = unsafe { &mut *lp.c };
    let l = unsafe { &mut *lp.l };
    let u = unsafe { &mut *lp.u };
    let head = unsafe { &mut *lp.head };
    let beta = unsafe { &mut *csa.beta };
    
    let mut ret = 0;
    
    for i in 1..=m {
        let k = unsafe { *head.offset(i as isize) };
        let (lk, uk, orig) = if phase == 1 && unsafe { *c.offset(k as isize) } < 0.0 {
            (-f64::MAX, unsafe { *l.offset(k as isize) }, 0)
        } else if phase == 1 && unsafe { *c.offset(k as isize) } > 0.0 {
            (unsafe { *u.offset(k as isize) }, f64::MAX, 0)
        } else {
            (unsafe { *l.offset(k as isize) }, unsafe { *u.offset(k as isize) }, 1)
        };
        
        if lk != -f64::MAX {
            let eps = tol + tol1 * lk.abs();
            if unsafe { *beta.offset(i as isize) } < lk - eps {
                if orig != 0 {
                    ret = 2;
                    break;
                } else {
                    ret = 1;
                }
            }
        }
        
        if uk != f64::MAX {
            let eps = tol + tol1 * uk.abs();
            if unsafe { *beta.offset(i as isize) } > uk + eps {
                if orig != 0 {
                    ret = 2;
                    break;
                } else {
                    ret = 1;
                }
            }
        }
    }
    
    ret
}

fn adjust_penalty(csa: &mut csa, num: i32, ind: *const i32, tol: f64, tol1: f64) -> i32 {
    let lp = unsafe { &mut *csa.lp };
    let m = lp.m;
    let c = unsafe { &mut *lp.c };
    let l = unsafe { &mut *lp.l };
    let u = unsafe { &mut *lp.u };
    let head = unsafe { &mut *lp.head };
    let beta = unsafe { &mut *csa.beta };
    
    let mut cnt = 0;
    
    for t in 1..=num {
        let i = unsafe { *ind.offset(t as isize) };
        let k = unsafe { *head.offset(i as isize) };
        
        if unsafe { *c.offset(k as isize) } < 0.0 {
            let lk = unsafe { *l.offset(k as isize) };
            let eps = tol + tol1 * lk.abs();
            if unsafe { *beta.offset(i as isize) } >= lk - eps {
                unsafe { *c.offset(k as isize) = 0.0; }
                cnt += 1;
            }
        } else if unsafe { *c.offset(k as isize) } > 0.0 {
            let uk = unsafe { *u.offset(k as isize) };
            let eps = tol + tol1 * uk.abs();
            if unsafe { *beta.offset(i as isize) } <= uk + eps {
                unsafe { *c.offset(k as isize) = 0.0; }
                cnt += 1;
            }
        }
    }
    
    cnt
}

fn choose_pivot(csa: &mut csa) -> i32 {
    let lp = unsafe { &mut *csa.lp };
    let m = lp.m;
    let n = lp.n;
    let beta = unsafe { &mut *csa.beta };
    let d = unsafe { &mut *csa.d };
    let se = unsafe { &mut *csa.se };
    let list = unsafe { &mut *csa.list };
    let tcol = unsafe { &mut *csa.work.vec };
    let tol_piv = csa.tol_piv;
    
    let mut ret = 0;
    
    loop {
        let nnn = csa.num;
        csa.q = 0;
        let mut best_ratio = 0.0;
        let mut try_ = ret;
        
        loop {
            try_ += 1;
            
            let q = if se.is_null() {
                unsafe { _glp_spx_chuzc_std(lp, d, nnn, list) }
            } else {
                unsafe { _glp_spx_chuzc_pse(lp, se, d, nnn, list) }
            };
            
            unsafe {
                _glp_spx_eval_tcol(lp, q, tcol);
            }
            
            let mut big = 1.0;
            
            if csa.phase == 1 && csa.r_test == 0x33 && try_ <= 2 {
                let mut nbp = unsafe {
                    _glp_spx_ls_eval_bp(lp, beta, q, unsafe { *d.offset(q as isize) }, tcol, tol_piv, csa.bp)
                };
                
                if nbp >= 2 {
                    let mut slope = -unsafe { *d.offset(q as isize) }.abs();
                    let mut teta_lim = f64::MAX;
                    
                    for t_ in 1..=nbp {
                        teta_lim = teta_lim.min(unsafe { (*csa.bp.offset(t_ as isize)).teta });
                    }
                    
                    teta_lim = teta_lim.max(1e-3);
                    
                    let mut t_best = 0;
                    let mut dz_best = 0.0;
                    let mut num = 0;
                    
                    while num < nbp {
                        let num1 = unsafe {
                            _glp_spx_ls_select_bp(lp, tcol, nbp, csa.bp, num, &mut slope, teta_lim)
                        };
                        
                        for t_ in (num + 1)..=num1 {
                            let i = unsafe { (*csa.bp.offset(t_ as isize)).i }.abs();
                            
                            if i == 0 || unsafe { (*tcol.offset(i as isize)).abs() / big >= 0.0001 {
                                if dz_best > unsafe { (*csa.bp.offset(t_ as isize)).dz } {
                                    t_best = t_;
                                    dz_best = unsafe { (*csa.bp.offset(t_ as isize)).dz };
                                }
                            }
                        }
                        
                        if slope > 0.0 {
                            break;
                        }
                        
                        num = num1;
                        teta_lim += teta_lim;
                    }
                    
                    if dz_best != 0.0 {
                        unsafe {
                            ptr::copy_nonoverlapping(
                                tcol.offset(1),
                                (*csa.tcol.vec).offset(1),
                                m as usize
                            );
                            
                            _glp_fvs_gather_vec(&mut csa.tcol, 2.2204460492503131e-16);
                            
                            if unsafe { (*csa.bp.offset(t_best as isize)).i } == 0 {
                                csa.p = -1;
                                csa.p_flag = 0;
                                best_ratio = 1.0;
                            } else if unsafe { (*csa.bp.offset(t_best as isize)).i } > 0 {
                                csa.p = unsafe { (*csa.bp.offset(t_best as isize)).i };
                                csa.p_flag = 0;
                                best_ratio = unsafe { (*tcol.offset(csa.p as isize)).abs() } / big;
                            } else {
                                csa.p = -unsafe { (*csa.bp.offset(t_best as isize)).i };
                                csa.p_flag = 1;
                                best_ratio = unsafe { (*tcol.offset(csa.p as isize)).abs() } / big;
                            }
                            
                            ret = 1;
                            break;
                        }
                    }
                }
            }
            
            let (p, p_flag) = if csa.r_test == 0x11 {
                unsafe {
                    let mut p_flag = 0;
                    let p = _glp_spx_chuzr_std(
                        lp,
                        csa.phase,
                        beta,
                        q,
                        if unsafe { *d.offset(q as isize) } < 0.0 { 1.0 } else { -1.0 },
                        tcol,
                        &mut p_flag,
                        tol_piv,
                        0.30 * csa.tol_bnd,
                        0.30 * csa.tol_bnd1,
                    );
                    (p, p_flag)
                }
            } else {
                unsafe {
                    let mut p_flag = 0;
                    let p = _glp_spx_chuzr_harris(
                        lp,
                        csa.phase,
                        beta,
                        q,
                        if unsafe { *d.offset(q as isize) } < 0.0 { 1.0 } else { -1.0 },
                        tcol,
                        &mut p_flag,
                        tol_piv,
                        0.50 * csa.tol_bnd,
                        0.50 * csa.tol_bnd1,
                    );
                    (p, p_flag)
                }
            };
            
            if p <= 0 {
                unsafe {
                    ptr::copy_nonoverlapping(
                        tcol.offset(1),
                        (*csa.tcol.vec).offset(1),
                        m as usize
                    );
                    
                    _glp_fvs_gather_vec(&mut csa.tcol, 2.2204460492503131e-16);
                    
                    csa.q = q;
                    csa.p = p;
                    csa.p_flag = p_flag;
                    best_ratio = 1.0;
                    break;
                }
            } else {
                if best_ratio < unsafe { (*tcol.offset(p as isize)).abs() } / big {
                    unsafe {
                        ptr::copy_nonoverlapping(
                            tcol.offset(1),
                            (*csa.tcol.vec).offset(1),
                            m as usize
                        );
                        
                        _glp_fvs_gather_vec(&mut csa.tcol, 2.2204460492503131e-16);
                        
                        csa.q = q;
                        csa.p = p;
                        csa.p_flag = p_flag;
                        best_ratio = unsafe { (*tcol.offset(p as isize)).abs() } / big;
                    }
                }
                
                if best_ratio >= 0.0001 * 0.0001 || nnn == 1 || try_ == 5 {
                    break;
                }
                
                let mut t = 1;
                while t <= nnn {
                    if unsafe { *list.offset(t as isize) } == q {
                        break;
                    }
                    t += 1;
                }
                
                unsafe {
                    *list.offset(t as isize) = *list.offset(nnn as isize);
                    *list.offset(nnn as isize) = q;
                }
                
                nnn -= 1;
            }
        }
        
        if best_ratio >= 0.001 * 0.0001 {
            break;
        }
        
        if unsafe { _glp_bfd_get_count(lp.bfd) } > 0 {
            return -1;
        }
        
        if tol_piv != csa.tol_piv {
           