/* spxchuzr.rs */

use std::cmp::Ordering;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub c: Vec<f64>,
    pub l: Vec<f64>,
    pub u: Vec<f64>,
    pub head: Vec<i32>,
}

#[derive(Debug, Clone, Copy)]
pub struct SPXBP {
    pub i: i32,
    pub teta: f64,
    pub dc: f64,
    pub dz: f64,
}

pub fn spx_chuzr_std(
    lp: &SPXLP,
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
    let m = lp.m;
    let n = lp.n;
    let c = &lp.c;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;

    assert!(phase == 1 || phase == 2);
    assert!(1 <= q && q <= n - m);
    assert!(s == 1.0 || s == -1.0);

    let mut p = 0;
    *p_flag = 0;
    let mut teta_min = f64::MAX;
    let mut biga = 0.0;

    let k = head[(m + q - 1) as usize];
    if l[k as usize] == -f64::MAX || u[k as usize] == f64::MAX {
        p = 0;
        *p_flag = 0;
        teta_min = f64::MAX;
        biga = 0.0;
    } else {
        p = -1;
        *p_flag = 0;
        teta_min = (l[k as usize] - u[k as usize]).abs();
        biga = 1.0;
    }

    for i in 1..=m {
        let k = head[(i - 1) as usize];
        let alfa = s * tcol[(i - 1) as usize];

        if alfa <= -tol_piv {
            let (lk, i_flag) = if phase == 1 && c[k as usize] < 0.0 {
                continue;
            } else if phase == 1 && c[k as usize] > 0.0 {
                let lk = u[k as usize];
                assert!(lk != f64::MAX);
                (lk, 1)
            } else {
                let lk = l[k as usize];
                if lk == -f64::MAX {
                    continue;
                }
                (lk, 0)
            };

            let delta = tol + tol1 * if lk >= 0.0 { lk } else { -lk };
            let teta = if beta[(i - 1) as usize] <= lk + delta {
                0.0
            } else {
                (lk - beta[(i - 1) as usize]) / alfa
            };

            let alfa_abs = alfa.abs();
            if teta_min > teta || (teta_min == teta && biga < alfa_abs) {
                p = i;
                *p_flag = i_flag;
                teta_min = teta;
                biga = alfa_abs;
            }
        } else if alfa >= tol_piv {
            let (uk, i_flag) = if phase == 1 && c[k as usize] < 0.0 {
                let uk = l[k as usize];
                assert!(uk != -f64::MAX);
                (uk, 0)
            } else if phase == 1 && c[k as usize] > 0.0 {
                continue;
            } else {
                let uk = u[k as usize];
                if uk == f64::MAX {
                    continue;
                }
                (uk, 1)
            };

            let delta = tol + tol1 * if uk >= 0.0 { uk } else { -uk };
            let teta = if beta[(i - 1) as usize] >= uk - delta {
                0.0
            } else {
                (uk - beta[(i - 1) as usize]) / alfa
            };

            let alfa_abs = alfa.abs();
            if teta_min > teta || (teta_min == teta && biga < alfa_abs) {
                p = i;
                *p_flag = i_flag;
                teta_min = teta;
                biga = alfa_abs;
            }
        }
    }

    if p > 0 {
        let k = head[(p - 1) as usize];
        if l[k as usize] == u[k as usize] {
            *p_flag = 0;
        }
    }

    p
}

pub fn spx_chuzr_harris(
    lp: &SPXLP,
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
    let m = lp.m;
    let n = lp.n;
    let c = &lp.c;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;

    assert!(phase == 1 || phase == 2);
    assert!(1 <= q && q <= n - m);
    assert!(s == 1.0 || s == -1.0);

    let mut teta_min = f64::MAX;

    for i in 1..=m {
        let k = head[(i - 1) as usize];
        let alfa = s * tcol[(i - 1) as usize];

        if alfa <= -tol_piv {
            let lk = if phase == 1 && c[k as usize] < 0.0 {
                continue;
            } else if phase == 1 && c[k as usize] > 0.0 {
                u[k as usize]
            } else {
                l[k as usize]
            };

            if lk == -f64::MAX {
                continue;
            }

            let delta = tol + tol1 * if lk >= 0.0 { lk } else { -lk };
            let teta = if beta[(i - 1) as usize] < lk {
                -delta / alfa
            } else {
                ((lk - delta) - beta[(i - 1) as usize]) / alfa
            };

            assert!(teta >= 0.0);
            if teta_min > teta {
                teta_min = teta;
            }
        } else if alfa >= tol_piv {
            let uk = if phase == 1 && c[k as usize] < 0.0 {
                l[k as usize]
            } else if phase == 1 && c[k as usize] > 0.0 {
                continue;
            } else {
                u[k as usize]
            };

            if uk == f64::MAX {
                continue;
            }

            let delta = tol + tol1 * if uk >= 0.0 { uk } else { -uk };
            let teta = if beta[(i - 1) as usize] > uk {
                delta / alfa
            } else {
                ((uk + delta) - beta[(i - 1) as usize]) / alfa
            };

            assert!(teta >= 0.0);
            if teta_min > teta {
                teta_min = teta;
            }
        }
    }

    let k = head[(m + q - 1) as usize];
    if l[k as usize] != -f64::MAX && u[k as usize] != f64::MAX {
        if (l[k as usize] - u[k as usize]).abs() <= teta_min {
            *p_flag = 0;
            return -1;
        }
    }

    if teta_min == f64::MAX {
        *p_flag = 0;
        return 0;
    }

    let mut p = 0;
    *p_flag = 0;
    let mut biga = 0.0;

    for i in 1..=m {
        let k = head[(i - 1) as usize];
        let alfa = s * tcol[(i - 1) as usize];

        if alfa <= -tol_piv {
            let (lk, i_flag) = if phase == 1 && c[k as usize] < 0.0 {
                continue;
            } else if phase == 1 && c[k as usize] > 0.0 {
                (u[k as usize], 1)
            } else {
                let lk = l[k as usize];
                if lk == -f64::MAX {
                    continue;
                }
                (lk, 0)
            };

            let teta = (lk - beta[(i - 1) as usize]) / alfa;
            let alfa_abs = alfa.abs();
            if teta <= teta_min && biga < alfa_abs {
                p = i;
                *p_flag = i_flag;
                biga = alfa_abs;
            }
        } else if alfa >= tol_piv {
            let (uk, i_flag) = if phase == 1 && c[k as usize] < 0.0 {
                (l[k as usize], 0)
            } else if phase == 1 && c[k as usize] > 0.0 {
                continue;
            } else {
                let uk = u[k as usize];
                if uk == f64::MAX {
                    continue;
                }
                (uk, 1)
            };

            let teta = (uk - beta[(i - 1) as usize]) / alfa;
            let alfa_abs = alfa.abs();
            if teta <= teta_min && biga < alfa_abs {
                p = i;
                *p_flag = i_flag;
                biga = alfa_abs;
            }
        }
    }

    assert!(1 <= p && p <= m);
    let k = head[(p - 1) as usize];
    if l[k as usize] == u[k as usize] {
        *p_flag = 0;
    }

    p
}

pub fn spx_ls_eval_bp(
    lp: &SPXLP,
    beta: &[f64],
    q: i32,
    dq: f64,
    tcol: &[f64],
    tol_piv: f64,
    bp: &mut [SPXBP],
) -> i32 {
    let m = lp.m;
    let n = lp.n;
    let c = &lp.c;
    let l = &lp.l;
    let u = &lp.u;
    let head = &lp.head;

    assert!(1 <= q && q <= n - m);
    assert!(dq != 0.0);

    let s = if dq < 0.0 { 1.0 } else { -1.0 };
    let mut nbp = 0;

    let k = head[(m + q - 1) as usize];
    if l[k as usize] != -f64::MAX && u[k as usize] != f64::MAX {
        nbp += 1;
        bp[nbp as usize].i = 0;
        assert!(l[k as usize] < u[k as usize]);
        bp[nbp as usize].teta = u[k as usize] - l[k as usize];
        bp[nbp as usize].dc = s;
    }

    for i in 1..=m {
        let k = head[(i - 1) as usize];
        assert!(l[k as usize] <= u[k as usize]);
        let alfa = s * tcol[(i - 1) as usize];

        if alfa >= tol_piv {
            if l[k as usize] == u[k as usize] {
                if c[k as usize] <= 0.0 {
                    nbp += 1;
                    bp[nbp as usize].i = i;
                    bp[nbp as usize].teta = (l[k as usize] - beta[(i - 1) as usize]) / alfa;
                    bp[nbp as usize].dc = 1.0 - c[k as usize];
                }
            } else {
                if l[k as usize] != -f64::MAX && c[k as usize] < 0.0 {
                    nbp += 1;
                    bp[nbp as usize].i = i;
                    bp[nbp as usize].teta = (l[k as usize] - beta[(i - 1) as usize]) / alfa;
                    bp[nbp as usize].dc = 1.0;
                }
                if u[k as usize] != f64::MAX && c[k as usize] <= 0.0 {
                    nbp += 1;
                    bp[nbp as usize].i = -i;
                    bp[nbp as usize].teta = (u[k as usize] - beta[(i - 1) as usize]) / alfa;
                    bp[nbp as usize].dc = 1.0;
                }
            }
        } else if alfa <= -tol_piv {
            if l[k as usize] == u[k as usize] {
                if c[k as usize] >= 0.0 {
                    nbp += 1;
                    bp[nbp as usize].i = i;
                    bp[nbp as usize].teta = (l[k as usize] - beta[(i - 1) as usize]) / alfa;
                    bp[nbp as usize].dc = -1.0 - c[k as usize];
                }
            } else {
                if l[k as usize] != -f64::MAX && c[k as usize] >= 0.0 {
                    nbp += 1;
                    bp[nbp as usize].i = i;
                    bp[nbp as usize].teta = (l[k as usize] - beta[(i - 1) as usize]) / alfa;
                    bp[nbp as usize].dc = -1.0;
                }
                if u[k as usize] != f64::MAX && c[k as usize] > 0.0 {
                    nbp += 1;
                    bp[nbp as usize].i = -i;
                    bp[nbp as usize].teta = (u[k as usize] - beta[(i - 1) as usize]) / alfa;
                    bp[nbp as usize].dc = -1.0;
                }
            }
        }

        if nbp > 0 && bp[nbp as usize].teta < 0.0 {
            bp[nbp as usize].teta = 0.0;
        }
    }

    assert!(nbp <= 2 * m + 1);
    nbp
}

pub fn spx_ls_select_bp(
    lp: &SPXLP,
    tcol: &[f64],
    nbp: i32,
    bp: &mut [SPXBP],
    num: i32,
    slope: &mut f64,
    teta_lim: f64,
) -> i32 {
    let m = lp.m;
    assert!(0 <= num && num <= nbp && nbp <= m + m + 1);

    let mut num1 = num;
    for t in (num + 1)..=nbp {
        if bp[t as usize].teta <= teta_lim {
            num1 += 1;
            bp.swap(num1 as usize, t as usize);
        }
    }

    if num1 - num > 1 {
        bp[(num + 1) as usize..=(num1) as usize].sort_by(|a, b| {
            a.teta.partial_cmp(&b.teta).unwrap_or(Ordering::Equal)
        });
    }

    for t in (num + 1)..=num1 {
        let dz = *slope * (bp[t as usize].teta - if t == 1 { 0.0 } else { bp[(t - 1) as usize].teta });
        bp[t as usize].dz = if t == 1 { 0.0 } else { bp[(t - 1) as usize].dz } + dz;
        
        let i = if bp[t as usize].i >= 0 {
            bp[t as usize].i
        } else {
            -bp[t as usize].i
        };
        assert!(0 <= i && i <= m);
        
        if i == 0 {
            *slope += bp[t as usize].dc.abs();
        } else {
            *slope += tcol[(i - 1) as usize].abs() * bp[t as usize].dc.abs();
        }
    }

    num1
}