use std::f64::{INFINITY, NEG_INFINITY};

/// Structure to hold row information for implied bounds calculation
#[derive(Debug, Clone)]
struct FInfo {
    j_min: usize,
    j_max: usize,
    f_min: f64,
    f_max: f64,
}

/// Prepares row information to determine implied bounds
fn prepare_row_info(n: usize, a: &[f64], l: &[f64], u: &[f64]) -> FInfo {
    assert!(n > 0);
    assert_eq!(a.len(), n + 1);
    assert_eq!(l.len(), n + 1);
    assert_eq!(u.len(), n + 1);

    // Determine f_min and j_min
    let mut f_min = 0.0;
    let mut j_min = 0;
    for j in 1..=n {
        if a[j] > 0.0 {
            if l[j] == NEG_INFINITY {
                if j_min == 0 {
                    j_min = j;
                } else {
                    f_min = NEG_INFINITY;
                    j_min = 0;
                    break;
                }
            } else {
                f_min += a[j] * l[j];
            }
        } else if a[j] < 0.0 {
            if u[j] == INFINITY {
                if j_min == 0 {
                    j_min = j;
                } else {
                    f_min = NEG_INFINITY;
                    j_min = 0;
                    break;
                }
            } else {
                f_min += a[j] * u[j];
            }
        } else {
            panic!("Zero coefficient in row");
        }
    }

    // Determine f_max and j_max
    let mut f_max = 0.0;
    let mut j_max = 0;
    for j in 1..=n {
        if a[j] > 0.0 {
            if u[j] == INFINITY {
                if j_max == 0 {
                    j_max = j;
                } else {
                    f_max = INFINITY;
                    j_max = 0;
                    break;
                }
            } else {
                f_max += a[j] * u[j];
            }
        } else if a[j] < 0.0 {
            if l[j] == NEG_INFINITY {
                if j_max == 0 {
                    j_max = j;
                } else {
                    f_max = INFINITY;
                    j_max = 0;
                    break;
                }
            } else {
                f_max += a[j] * l[j];
            }
        } else {
            panic!("Zero coefficient in row");
        }
    }

    FInfo {
        j_min,
        j_max,
        f_min,
        f_max,
    }
}

/// Determines row implied bounds
fn row_implied_bounds(f: &FInfo) -> (f64, f64) {
    let ll = if f.j_min == 0 { f.f_min } else { NEG_INFINITY };
    let uu = if f.j_max == 0 { f.f_max } else { INFINITY };
    (ll, uu)
}

/// Determines column implied bounds
fn col_implied_bounds(
    f: &FInfo,
    n: usize,
    a: &[f64],
    l: f64,
    u: f64,
    l_bounds: &[f64],
    u_bounds: &[f64],
    k: usize,
) -> (f64, f64) {
    assert!(n > 0);
    assert!(k >= 1 && k <= n);
    assert_eq!(a.len(), n + 1);
    assert_eq!(l_bounds.len(), n + 1);
    assert_eq!(u_bounds.len(), n + 1);

    // Determine implied lower bound of term a[k] * x[k]
    let ilb = if l == NEG_INFINITY || f.f_max == INFINITY {
        NEG_INFINITY
    } else if f.j_max == 0 {
        if a[k] > 0.0 {
            assert!(u_bounds[k] != INFINITY);
            l - (f.f_max - a[k] * u_bounds[k])
        } else if a[k] < 0.0 {
            assert!(l_bounds[k] != NEG_INFINITY);
            l - (f.f_max - a[k] * l_bounds[k])
        } else {
            panic!("Zero coefficient");
        }
    } else if f.j_max == k {
        l - f.f_max
    } else {
        NEG_INFINITY
    };

    // Determine implied upper bound of term a[k] * x[k]
    let iub = if u == INFINITY || f.f_min == NEG_INFINITY {
        INFINITY
    } else if f.j_min == 0 {
        if a[k] > 0.0 {
            assert!(l_bounds[k] != NEG_INFINITY);
            u - (f.f_min - a[k] * l_bounds[k])
        } else if a[k] < 0.0 {
            assert!(u_bounds[k] != INFINITY);
            u - (f.f_min - a[k] * u_bounds[k])
        } else {
            panic!("Zero coefficient");
        }
    } else if f.j_min == k {
        u - f.f_min
    } else {
        INFINITY
    };

    // Determine implied bounds of x[k]
    if a[k].abs() < 1e-6 {
        (NEG_INFINITY, INFINITY)
    } else if a[k] > 0.0 {
        let ll = if ilb == NEG_INFINITY {
            NEG_INFINITY
        } else {
            ilb / a[k]
        };
        let uu = if iub == INFINITY {
            INFINITY
        } else {
            iub / a[k]
        };
        (ll, uu)
    } else if a[k] < 0.0 {
        let ll = if iub == INFINITY {
            NEG_INFINITY
        } else {
            iub / a[k]
        };
        let uu = if ilb == NEG_INFINITY {
            INFINITY
        } else {
            ilb / a[k]
        };
        (ll, uu)
    } else {
        panic!("Zero coefficient");
    }
}

/// Checks and relaxes original row bounds
fn check_row_bounds(f: &FInfo, l: &mut f64, u: &mut f64) -> bool {
    let (ll, uu) = row_implied_bounds(f);

    // Check if the original lower bound is infeasible
    if *l != NEG_INFINITY {
        let eps = 1e-3 * (1.0 + l.abs());
        if uu < *l - eps {
            return true;
        }
    }

    // Check if the original upper bound is infeasible
    if *u != INFINITY {
        let eps = 1e-3 * (1.0 + u.abs());
        if ll > *u + eps {
            return true;
        }
    }

    // Check if the original lower bound is redundant
    if *l != NEG_INFINITY {
        let eps = 1e-12 * (1.0 + l.abs());
        if ll > *l - eps {
            *l = NEG_INFINITY;
        }
    }

    // Check if the original upper bound is redundant
    if *u != INFINITY {
        let eps = 1e-12 * (1.0 + u.abs());
        if uu < *u + eps {
            *u = INFINITY;
        }
    }

    false
}

/// Checks and tightens original column bounds
fn check_col_bounds(
    f: &FInfo,
    n: usize,
    a: &[f64],
    l: f64,
    u: f64,
    l_bounds: &[f64],
    u_bounds: &[f64],
    is_integer: bool,
    j: usize,
    lj: &mut f64,
    uj: &mut f64,
) -> bool {
    assert!(n > 0);
    assert!(j >= 1 && j <= n);
    assert_eq!(a.len(), n + 1);
    assert_eq!(l_bounds.len(), n + 1);
    assert_eq!(u_bounds.len(), n + 1);

    let (mut ll, mut uu) = col_implied_bounds(f, n, a, l, u, l_bounds, u_bounds, j);

    // If x[j] is integral, round its implied bounds
    if is_integer {
        if ll != NEG_INFINITY {
            ll = if ll - ll.floor() < 1e-3 {
                ll.floor()
            } else {
                ll.ceil()
            };
        }
        if uu != INFINITY {
            uu = if uu.ceil() - uu < 1e-3 {
                uu.ceil()
            } else {
                uu.floor()
            };
        }
    }

    // Check if the original lower bound is infeasible
    if *lj != NEG_INFINITY {
        let eps = 1e-3 * (1.0 + lj.abs());
        if uu < *lj - eps {
            return true;
        }
    }

    // Check if the original upper bound is infeasible
    if *uj != INFINITY {
        let eps = 1e-3 * (1.0 + uj.abs());
        if ll > *uj + eps {
            return true;
        }
    }

    // Check if the original lower bound is redundant
    if ll != NEG_INFINITY {
        let eps = 1e-3 * (1.0 + ll.abs());
        if *lj < ll - eps {
            *lj = ll;
        }
    }

    // Check if the original upper bound is redundant
    if uu != INFINITY {
        let eps = 1e-3 * (1.0 + uu.abs());
        if *uj > uu + eps {
            *uj = uu;
        }
    }

    // Adjust bounds to ensure lj <= uj
    if !(*lj == NEG_INFINITY || *uj == INFINITY) {
        let t1 = lj.abs();
        let t2 = uj.abs();
        let eps = 1e-10 * (1.0 + if t1 <= t2 { t1 } else { t2 });
        if *lj > *uj - eps {
            if *lj == l_bounds[j] {
                *uj = *lj;
            } else if *uj == u_bounds[j] {
                *lj = *uj;
            } else if t1 <= t2 {
                *uj = *lj;
            } else {
                *lj = *uj;
            }
        }
    }

    false
}

/// Checks if change in column bounds is efficient
fn check_efficiency(is_integer: bool, l: f64, u: f64, ll: f64, uu: f64) -> bool {
    let mut eff = false;

    // Check efficiency for lower bound
    if l < ll {
        if is_integer || l == NEG_INFINITY {
            eff = true;
        } else {
            let r = if u == INFINITY {
                1.0 + l.abs()
            } else {
                1.0 + (u - l)
            };
            if ll - l >= 0.25 * r {
                eff = true;
            }
        }
    }

    // Check efficiency for upper bound
    if u > uu {
        if is_integer || u == INFINITY {
            eff = true;
        } else {
            let r = if l == NEG_INFINITY {
                1.0 + u.abs()
            } else {
                1.0 + (u - l)
            };
            if u - uu >= 0.25 * r {
                eff = true;
            }
        }
    }

    eff
}