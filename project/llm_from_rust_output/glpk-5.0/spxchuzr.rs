use std::cmp::Ordering;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub a_ptr: Vec<i32>,
    pub a_ind: Vec<i32>,
    pub a_val: Vec<f64>,
    pub b: Vec<f64>,
    pub c: Vec<f64>,
    pub l: Vec<f64>,
    pub u: Vec<f64>,
    pub head: Vec<i32>,
    pub flag: Vec<i8>,
    pub valid: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SPXBP {
    pub i: i32,
    pub teta: f64,
    pub dc: f64,
    pub dz: f64,
}

pub fn glp_spx_chuzr_std(
    lp: &mut SPXLP,
    phase: i32,
    beta: &[f64],
    q: i32,
    s: f64,
    tcol: &[f64],
    p_flag: &mut i32,
    tol_piv: f64,
    tol: f64,
    tol1: f64,
) -> i32 {
    assert!(phase == 1 || phase == 2, "phase must be 1 or 2");
    assert!(1 <= q && q <= lp.n - lp.m, "q must be between 1 and n-m");
    assert!(s == 1.0 || s == -1.0, "s must be +1.0 or -1.0");

    let m = lp.m;
    let n = lp.n;
    let k = lp.head[(m + q - 1) as usize];
    
    let (mut p, mut teta_min, mut biga) = if lp.l[k as usize] == f64::MIN || lp.u[k as usize] == f64::MAX {
        (0, f64::MAX, 0.0)
    } else {
        (-1, (lp.l[k as usize] - lp.u[k as usize]).abs(), 1.0)
    };

    *p_flag = 0;

    for i in 1..=m {
        let k = lp.head[i as usize - 1];
        let alfa = s * tcol[i as usize - 1];
        
        if alfa <= -tol_piv {
            if phase == 1 && lp.c[k as usize] < 0.0 {
                continue;
            }
            
            let (lk, i_flag) = if phase == 1 && lp.c[k as usize] > 0.0 {
                assert!(lp.u[k as usize] != f64::MAX, "lk != +DBL_MAX");
                (lp.u[k as usize], 1)
            } else {
                if lp.l[k as usize] == f64::MIN {
                    continue;
                }
                (lp.l[k as usize], 0)
            };
            
            let delta = tol + tol1 * if lk >= 0.0 { lk } else { -lk };
            let teta = if beta[i as usize - 1] <= lk + delta {
                0.0
            } else {
                (lk - beta[i as usize - 1]) / alfa
            };
            
            assert!(teta >= 0.0, "teta must be >= 0.0");
            let abs_alfa = alfa.abs();
            if teta_min > teta || (teta_min == teta && biga < abs_alfa) {
                p = i;
                *p_flag = i_flag;
                teta_min = teta;
                biga = abs_alfa;
            }
        } else if alfa >= tol_piv {
            if phase == 1 && lp.c[k as usize] < 0.0 {
                assert!(lp.l[k as usize] != f64::MIN, "uk != -DBL_MAX");
                let uk = lp.l[k as usize];
                let i_flag = 0;
                
                let delta = tol + tol1 * if uk >= 0.0 { uk } else { -uk };
                let teta = if beta[i as usize - 1] >= uk - delta {
                    0.0
                } else {
                    (uk - beta[i as usize - 1]) / alfa
                };
                
                assert!(teta >= 0.0, "teta must be >= 0.0");
                let abs_alfa = alfa.abs();
                if teta_min > teta || (teta_min == teta && biga < abs_alfa) {
                    p = i;
                    *p_flag = i_flag;
                    teta_min = teta;
                    biga = abs_alfa;
                }
            } else if phase == 1 && lp.c[k as usize] > 0.0 {
                continue;
            } else {
                if lp.u[k as usize] == f64::MAX {
                    continue;
                }
                let uk = lp.u[k as usize];
                let i_flag = 1;
                
                let delta = tol + tol1 * if uk >= 0.0 { uk } else { -uk };
                let teta = if beta[i as usize - 1] >= uk - delta {
                    0.0
                } else {
                    (uk - beta[i as usize - 1]) / alfa
                };
                
                assert!(teta >= 0.0, "teta must be >= 0.0");
                let abs_alfa = alfa.abs();
                if teta_min > teta || (teta_min == teta && biga < abs_alfa) {
                    p = i;
                    *p_flag = i_flag;
                    teta_min = teta;
                    biga = abs_alfa;
                }
            }
        }
    }

    if p > 0 {
        let k = lp.head[p as usize - 1];
        if lp.l[k as usize] == lp.u[k as usize] {
            *p_flag = 0;
        }
    }

    p
}

pub fn glp_spx_chuzr_harris(
    lp: &mut SPXLP,
    phase: i32,
    beta: &[f64],
    q: i32,
    s: f64,
    tcol: &[f64],
    p_flag: &mut i32,
    tol_piv: f64,
    tol: f64,
    tol1: f64,
) -> i32 {
    assert!(phase == 1 || phase == 2, "phase must be 1 or 2");
    assert!(1 <= q && q <= lp.n - lp.m, "q must be between 1 and n-m");
    assert!(s == 1.0 || s == -1.0, "s must be +1.0 or -1.0");

    let m = lp.m;
    let mut teta_min = f64::MAX;
    
    // First pass to find teta_min
    for i in 1..=m {
        let k = lp.head[i as usize - 1];
        let alfa = s * tcol[i as usize - 1];
        
        if alfa <= -tol_piv {
            if phase == 1 && lp.c[k as usize] < 0.0 {
                continue;
            }
            
            let lk = if phase == 1 && lp.c[k as usize] > 0.0 {
                assert!(lp.u[k as usize] != f64::MAX, "lk != +DBL_MAX");
                lp.u[k as usize]
            } else {
                if lp.l[k as usize] == f64::MIN {
                    continue;
                }
                lp.l[k as usize]
            };
            
            let delta = tol + tol1 * if lk >= 0.0 { lk } else { -lk };
            let teta = if beta[i as usize - 1] < lk {
                -delta / alfa
            } else {
                (lk - delta - beta[i as usize - 1]) / alfa
            };
            
            assert!(teta >= 0.0, "teta must be >= 0.0");
            if teta_min > teta {
                teta_min = teta;
            }
        } else if alfa >= tol_piv {
            if phase == 1 && lp.c[k as usize] < 0.0 {
                assert!(lp.l[k as usize] != f64::MIN, "uk != -DBL_MAX");
                let uk = lp.l[k as usize];
                
                let delta = tol + tol1 * if uk >= 0.0 { uk } else { -uk };
                let teta = if beta[i as usize - 1] > uk {
                    delta / alfa
                } else {
                    (uk + delta - beta[i as usize - 1]) / alfa
                };
                
                assert!(teta >= 0.0, "teta must be >= 0.0");
                if teta_min > teta {
                    teta_min = teta;
                }
            } else if phase == 1 && lp.c[k as usize] > 0.0 {
                continue;
            } else {
                if lp.u[k as usize] == f64::MAX {
                    continue;
                }
                let uk = lp.u[k as usize];
                
                let delta = tol + tol1 * if uk >= 0.0 { uk } else { -uk };
                let teta = if beta[i as usize - 1] > uk {
                    delta / alfa
                } else {
                    (uk + delta - beta[i as usize - 1]) / alfa
                };
                
                assert!(teta >= 0.0, "teta must be >= 0.0");
                if teta_min > teta {
                    teta_min = teta;
                }
            }
        }
    }

    let k = lp.head[(m + q - 1) as usize];
    if lp.l[k as usize] != f64::MIN && lp.u[k as usize] != f64::MAX {
        if (lp.l[k as usize] - lp.u[k as usize]).abs() <= teta_min {
            *p_flag = 0;
            return -1;
        }
    }

    if teta_min == f64::MAX {
        *p_flag = 0;
        return 0;
    }

    let mut p = 0;
    let mut biga = 0.0;
    *p_flag = 0;

    for i in 1..=m {
        let k = lp.head[i as usize - 1];
        let alfa = s * tcol[i as usize - 1];
        
        if alfa <= -tol_piv {
            if phase == 1 && lp.c[k as usize] < 0.0 {
                continue;
            }
            
            let (lk, i_flag) = if phase == 1 && lp.c[k as usize] > 0.0 {
                assert!(lp.u[k as usize] != f64::MAX, "lk != +DBL_MAX");
                (lp.u[k as usize], 1)
            } else {
                if lp.l[k as usize] == f64::MIN {
                    continue;
                }
                (lp.l[k as usize], 0)
            };
            
            let teta = (lk - beta[i as usize - 1]) / alfa;
            let abs_alfa = alfa.abs();
            if teta <= teta_min && biga < abs_alfa {
                p = i;
                *p_flag = i_flag;
                biga = abs_alfa;
            }
        } else if alfa >= tol_piv {
            if phase == 1 && lp.c[k as usize] < 0.0 {
                assert!(lp.l[k as usize] != f64::MIN, "uk != -DBL_MAX");
                let uk = lp.l[k as usize];
                let i_flag = 0;
                
                let teta = (uk - beta[i as usize - 1]) / alfa;
                let abs_alfa = alfa.abs();
                if teta <= teta_min && biga < abs_alfa {
                    p = i;
                    *p_flag = i_flag;
                    biga = abs_alfa;
                }
            } else if phase == 1 && lp.c[k as usize] > 0.0 {
                continue;
            } else {
                if lp.u[k as usize] == f64::MAX {
                    continue;
                }
                let uk = lp.u[k as usize];
                let i_flag = 1;
                
                let teta = (uk - beta[i as usize - 1]) / alfa;
                let abs_alfa = alfa.abs();
                if teta <= teta_min && biga < abs_alfa {
                    p = i;
                    *p_flag = i_flag;
                    biga = abs_alfa;
                }
            }
        }
    }

    assert!(1 <= p && p <= m, "p must be between 1 and m");
    let k = lp.head[p as usize - 1];
    if lp.l[k as usize] == lp.u[k as usize] {
        *p_flag = 0;
    }

    p
}

pub fn glp_spx_ls_eval_bp(
    lp: &mut SPXLP,
    beta: &[f64],
    q: i32,
    dq: f64,
    tcol: &[f64],
    tol_piv: f64,
    bp: &mut [SPXBP],
) -> i32 {
    assert!(1 <= q && q <= lp.n - lp.m, "q must be between 1 and n-m");
    assert!(dq != 0.0, "dq must not be 0.0");

    let m = lp.m;
    let s = if dq < 0.0 { 1.0 } else { -1.0 };
    let mut nbp = 0;

    let k = lp.head[(m + q - 1) as usize];
    if lp.l[k as usize] != f64::MIN && lp.u[k as usize] != f64::MAX {
        assert!(lp.l[k as usize] < lp.u[k as usize], "l[k] < u[k]");
        nbp += 1;
        bp[nbp as usize - 1].i = 0;
        bp[nbp as usize - 1].teta = lp.u[k as usize] - lp.l[k as usize];
        bp[nbp as usize - 1].dc = s;
    }

    for i in 1..=m {
        let k = lp.head[i as usize - 1];
        assert!(lp.l[k as usize] <= lp.u[k as usize], "l[k] <= u[k]");
        let alfa = s * tcol[i as usize - 1];

        if alfa >= tol_piv {
            if lp.l[k as usize] == lp.u[k as usize] {
                if lp.c[k as usize] <= 0.0 {
                    nbp += 1;
                    bp[nbp as usize - 1].i = i;
                    bp[nbp as usize - 1].teta = (lp.l[k as usize] - beta[i as usize - 1]) / alfa;
                    bp[nbp as usize - 1].dc = 1.0 - lp.c[k as usize];
                }
            } else {
                if lp.l[k as usize] != f64::MIN && lp.c[k as usize] < 0.0 {
                    nbp += 1;
                    bp[nbp as usize - 1].i = i;
                    bp[nbp as usize - 1].teta = (lp.l[k as usize] - beta[i as usize - 1]) / alfa;
                    bp[nbp as usize - 1].dc = 1.0;
                }
                if lp.u[k as usize] != f64::MAX && lp.c[k as usize] <= 0.0 {
                    nbp += 1;
                    bp[nbp as usize - 1].i = -i;
                    bp[nbp as usize - 1].teta = (lp.u[k as usize] - beta[i as usize - 1]) / alfa;
                    bp[nbp as usize - 1].dc = 1.0;
                }
            }
        } else if alfa <= -tol_piv {
            if lp.l[k as usize] == lp.u[k as usize] {
                if lp.c[k as usize] >= 0.0 {
                    nbp += 1;
                    bp[nbp as usize - 1].i = i;
                    bp[nbp as usize - 1].teta = (lp.l[k as usize] - beta[i as usize - 1]) / alfa;
                    bp[nbp as usize - 1].dc = -1.0 - lp.c[k as usize];
                }
            } else {
                if lp.l[k as usize] != f64::MIN && lp.c[k as usize] >= 0.0 {
                    nbp += 1;
                    bp[nbp as usize - 1].i = i;
                    bp[nbp as usize - 1].teta = (lp.l[k as usize] - beta[i as usize - 1]) / alfa;
                    bp[nbp as usize - 1].dc = -1.0;
                }
                if lp.u[k as usize] != f64::MAX && lp.c[k as usize] > 0.0 {
                    nbp += 1;
                    bp[nbp as usize - 1].i = -i;
                    bp[nbp as usize - 1].teta = (lp.u[k as usize] - beta[i as usize - 1]) / alfa;
                    bp[nbp as usize - 1].dc = -1.0;
                }
            }
        }

        if nbp > 0 && bp[nbp as usize - 1].teta < 0.0 {
            bp[nbp as usize - 1].teta = 0.0;
        }
    }

    assert!(nbp <= 2 * m + 1, "nbp must be <= 2*m+1");
    nbp
}

fn fcmp(a: &SPXBP, b: &SPXBP) -> Ordering {
    a.teta.partial_cmp(&b.teta).unwrap_or(Ordering::Equal)
}

pub fn glp_spx_ls_select_bp(
    lp: &mut SPXLP,
    tcol: &[f64],
    nbp: i32,
    bp: &mut [SPXBP],
    num: i32,
    slope: &mut f64,
    teta_lim: f64,
) -> i32