use glp_sys::*;
use std::ffi::c_int;
use std::ptr;

const MAXTRY: usize = 1000;

fn cover2(
    n: usize,
    a: &[f64],
    b: f64,
    u: f64,
    x: &[f64],
    y: f64,
    cov: &mut [usize],
    alfa: &mut f64,
    beta: &mut f64,
) -> bool {
    let mut try_count = 0;
    let mut ret = false;
    let eps = 0.001 * (1.0 + b.abs());
    let mut rmax = 0.001;

    for i in 1..=n {
        for j in i + 1..=n {
            try_count += 1;
            if try_count > MAXTRY {
                return ret;
            }
            if a[i] + a[j] + y > b + eps {
                let temp = a[i] + a[j] - b;
                let local_alfa = 1.0 / (temp + u);
                let local_beta = 2.0 - local_alfa * temp;
                let violation = x[i] + x[j] + local_alfa * y - local_beta;
                if rmax < violation {
                    rmax = violation;
                    cov[1] = i;
                    cov[2] = j;
                    *alfa = local_alfa;
                    *beta = local_beta;
                    ret = true;
                }
            }
        }
    }
    ret
}

fn cover3(
    n: usize,
    a: &[f64],
    b: f64,
    u: f64,
    x: &[f64],
    y: f64,
    cov: &mut [usize],
    alfa: &mut f64,
    beta: &mut f64,
) -> bool {
    let mut try_count = 0;
    let mut ret = false;
    let eps = 0.001 * (1.0 + b.abs());
    let mut rmax = 0.001;

    for i in 1..=n {
        for j in i + 1..=n {
            for k in j + 1..=n {
                try_count += 1;
                if try_count > MAXTRY {
                    return ret;
                }
                if a[i] + a[j] + a[k] + y > b + eps {
                    let temp = a[i] + a[j] + a[k] - b;
                    let local_alfa = 1.0 / (temp + u);
                    let local_beta = 3.0 - local_alfa * temp;
                    let violation = x[i] + x[j] + x[k] + local_alfa * y - local_beta;
                    if rmax < violation {
                        rmax = violation;
                        cov[1] = i;
                        cov[2] = j;
                        cov[3] = k;
                        *alfa = local_alfa;
                        *beta = local_beta;
                        ret = true;
                    }
                }
            }
        }
    }
    ret
}

fn cover4(
    n: usize,
    a: &[f64],
    b: f64,
    u: f64,
    x: &[f64],
    y: f64,
    cov: &mut [usize],
    alfa: &mut f64,
    beta: &mut f64,
) -> bool {
    let mut try_count = 0;
    let mut ret = false;
    let eps = 0.001 * (1.0 + b.abs());
    let mut rmax = 0.001;

    for i in 1..=n {
        for j in i + 1..=n {
            for k in j + 1..=n {
                for l in k + 1..=n {
                    try_count += 1;
                    if try_count > MAXTRY {
                        return ret;
                    }
                    if a[i] + a[j] + a[k] + a[l] + y > b + eps {
                        let temp = a[i] + a[j] + a[k] + a[l] - b;
                        let local_alfa = 1.0 / (temp + u);
                        let local_beta = 4.0 - local_alfa * temp;
                        let violation = x[i] + x[j] + x[k] + x[l] + local_alfa * y - local_beta;
                        if rmax < violation {
                            rmax = violation;
                            cov[1] = i;
                            cov[2] = j;
                            cov[3] = k;
                            cov[4] = l;
                            *alfa = local_alfa;
                            *beta = local_beta;
                            ret = true;
                        }
                    }
                }
            }
        }
    }
    ret
}

fn cover(
    n: usize,
    a: &[f64],
    b: f64,
    u: f64,
    x: &[f64],
    y: f64,
    cov: &mut [usize],
    alfa: &mut f64,
    beta: &mut f64,
) -> usize {
    assert!(n >= 2);
    for j in 1..=n {
        assert!(a[j] > 0.0);
    }
    assert!(b > -1e-5);
    assert!(u >= 0.0);
    for j in 1..=n {
        assert!((0.0..=1.0).contains(&x[j]));
    }
    assert!((0.0..=u).contains(&y));

    if cover2(n, a, b, u, x, y, cov, alfa, beta) {
        return 2;
    }
    if cover3(n, a, b, u, x, y, cov, alfa, beta) {
        return 3;
    }
    if cover4(n, a, b, u, x, y, cov, alfa, beta) {
        return 4;
    }
    0
}

fn lpx_cover_cut(
    lp: *mut glp_prob,
    len: usize,
    ind: &mut [c_int],
    val: &mut [f64],
    work: &mut [f64],
) -> usize {
    let mut cov = [0; 5];
    let mut newlen = 0;

    for k in 1..=len {
        let j = ind[k] as c_int;
        if unsafe { glp_get_col_type(lp, j) } == GLP_FX as c_int {
            val[0] -= val[k] * unsafe { glp_get_col_lb(lp, j) };
        } else {
            newlen += 1;
            ind[newlen] = ind[k];
            val[newlen] = val[k];
        }
    }

    let mut nb = 0;
    for k in 1..=newlen {
        let j = ind[k] as c_int;
        if unsafe { glp_get_col_kind(lp, j) } == GLP_BV as c_int {
            nb += 1;
            ind.swap(nb, k);
            val.swap(nb, k);
        }
    }

    if nb < 2 {
        return 0;
    }

    let mut f_min = 0.0;
    let mut f_max = 0.0;
    for k in nb + 1..=newlen {
        let j = ind[k] as c_int;
        if unsafe { glp_get_col_type(lp, j) } != GLP_DB as c_int {
            return 0;
        }
        if val[k] > 0.0 {
            f_min += val[k] * unsafe { glp_get_col_lb(lp, j) };
            f_max += val[k] * unsafe { glp_get_col_ub(lp, j) };
        } else {
            f_min += val[k] * unsafe { glp_get_col_ub(lp, j) };
            f_max += val[k] * unsafe { glp_get_col_lb(lp, j) };
        }
    }

    let u = f_max - f_min;
    let mut y = 0.0;
    for k in nb + 1..=newlen {
        let j = ind[k] as c_int;
        y += val[k] * unsafe { glp_get_col_prim(lp, j) };
    }
    y -= f_min;
    let y = y.clamp(0.0, u);

    val[0] -= f_min;

    let x = &mut work[1..=nb];
    for k in 1..=nb {
        let j = ind[k] as c_int;
        x[k - 1] = unsafe { glp_get_col_prim(lp, j) }.clamp(0.0, 1.0);
    }

    for k in 1..=nb {
        if val[k] < 0.0 {
            ind[k] = -ind[k];
            val[k] = -val[k];
            val[0] += val[k];
            x[k - 1] = 1.0 - x[k - 1];
        }
    }

    let mut alfa = 0.0;
    let mut beta = 0.0;
    let r = cover(nb, val, val[0], u, x, y, &mut cov, &mut alfa, &mut beta);
    if r == 0 {
        return 0;
    }

    ind[0] = 0;
    val[0] = beta;

    for j in 1..=r {
        cov[j] = ind[cov[j]] as usize;
    }

    for k in 1..=r {
        if cov[k] > 0 {
            ind[k] = cov[k] as c_int;
            val[k] = 1.0;
        } else {
            ind[k] = -cov[k] as c_int;
            val[k] = -1.0;
            val[0] -= 1.0;
        }
    }

    let mut r = r;
    for k in nb + 1..=newlen {
        r += 1;
        ind[r] = ind[k];
        val[r] = alfa * val[k];
    }
    val[0] += alfa * f_min;

    r
}

fn lpx_eval_row(lp: *mut glp_prob, len: usize, ind: &[c_int], val: &[f64]) -> f64 {
    let n = unsafe { glp_get_num_cols(lp) };
    let mut sum = 0.0;

    for k in 1..=len {
        let j = ind[k] as c_int;
        assert!(1 <= j && j <= n as c_int);
        sum += val[k] * unsafe { glp_get_col_prim(lp, j) };
    }

    sum
}

pub fn ios_cov_gen(tree: *mut glp_tree) {
    let prob = unsafe { (*tree).mip };
    let m = unsafe { glp_get_num_rows(prob) } as usize;
    let n = unsafe { glp_get_num_cols(prob) } as usize;
    assert_eq!(unsafe { glp_get_status(prob) }, GLP_OPT as c_int);

    let mut ind = vec![0; n + 1];
    let mut val = vec![0.0; n + 1];
    let mut work = vec![0.0; n + 1];

    for i in 1..=m {
        for kase in 1..=2 {
            let type_ = unsafe { glp_get_row_type(prob, i as c_int) };
            if kase == 1 {
                if !(type_ == GLP_UP as c_int || type_ == GLP_DB as c_int) {
                    continue;
                }
                let len = unsafe { glp_get_mat_row(prob, i as c_int, ind.as_mut_ptr(), val.as_mut_ptr()) } as usize;
                val[0] = unsafe { glp_get_row_ub(prob, i as c_int) };
            } else {
                if !(type_ == GLP_LO as c_int || type_ == GLP_DB as c_int) {
                    continue;
                }
                let len = unsafe { glp_get_mat_row(prob, i as c_int, ind.as_mut_ptr(), val.as_mut_ptr()) } as usize;
                for k in 1..=len {
                    val[k] = -val[k];
                }
                val[0] = -unsafe { glp_get_row_lb(prob, i as c_int) };
            }

            let len = lpx_cover_cut(prob, len, &mut ind, &mut val, &mut work);
            if len == 0 {
                continue;
            }

            let r = lpx_eval_row(prob, len, &ind, &val) - val[0];
            if r < 1e-3 {
                continue;
            }

            unsafe {
                glp_ios_add_row(
                    tree,
                    ptr::null(),
                    GLP_RF_COV as c_int,
                    0,
                    len as c_int,
                    ind.as_ptr(),
                    val.as_ptr(),
                    GLP_UP as c_int,
                    val[0],
                );
            }
        }
    }
}