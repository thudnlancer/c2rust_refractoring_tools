use std::f64::{MAX, MIN};
use std::ptr;

struct FInfo {
    j_min: i32,
    j_max: i32,
    f_min: f64,
    f_max: f64,
}

struct GlpProb {
    // Fields from the original C struct
    // Omitted for brevity, but should be properly defined
}

struct GlpRow {
    // Fields from the original C struct
}

struct GlpCol {
    // Fields from the original C struct
}

struct GlpAij {
    // Fields from the original C struct
}

fn prepare_row_info(
    n: i32,
    a: &[f64],
    l: &[f64],
    u: &[f64],
    f: &mut FInfo,
) {
    assert!(n >= 0, "n >= 0");
    
    let mut f_min = 0.0;
    let mut j_min = 0;
    
    for j in 1..=n {
        let a_j = a[j as usize];
        let l_j = l[j as usize];
        let u_j = u[j as usize];
        
        if a_j > 0.0 {
            if l_j == MIN {
                if j_min == 0 {
                    j_min = j;
                } else {
                    f_min = MIN;
                    j_min = 0;
                    break;
                }
            } else {
                f_min += a_j * l_j;
            }
        } else if a_j < 0.0 {
            if u_j == MAX {
                if j_min == 0 {
                    j_min = j;
                } else {
                    f_min = MIN;
                    j_min = 0;
                    break;
                }
            } else {
                f_min += a_j * u_j;
            }
        } else {
            panic!("a != a");
        }
    }
    
    f.f_min = f_min;
    f.j_min = j_min;
    
    let mut f_max = 0.0;
    let mut j_max = 0;
    
    for j in 1..=n {
        let a_j = a[j as usize];
        let l_j = l[j as usize];
        let u_j = u[j as usize];
        
        if a_j > 0.0 {
            if u_j == MAX {
                if j_max == 0 {
                    j_max = j;
                } else {
                    f_max = MAX;
                    j_max = 0;
                    break;
                }
            } else {
                f_max += a_j * u_j;
            }
        } else if a_j < 0.0 {
            if l_j == MIN {
                if j_max == 0 {
                    j_max = j;
                } else {
                    f_max = MAX;
                    j_max = 0;
                    break;
                }
            } else {
                f_max += a_j * l_j;
            }
        } else {
            panic!("a != a");
        }
    }
    
    f.f_max = f_max;
    f.j_max = j_max;
}

fn row_implied_bounds(f: &FInfo, ll: &mut f64, uu: &mut f64) {
    *ll = if f.j_min == 0 { f.f_min } else { MIN };
    *uu = if f.j_max == 0 { f.f_max } else { MAX };
}

fn check_row_bounds(f: &FInfo, l: &mut f64, u: &mut f64) -> i32 {
    let mut ret = 0;
    let mut ll = 0.0;
    let mut uu = 0.0;
    
    row_implied_bounds(f, &mut ll, &mut uu);
    
    if *l != MIN {
        let eps = 1e-3 * (1.0 + l.abs());
        if uu < *l - eps {
            ret = 1;
            return ret;
        }
    }
    
    if *u != MAX {
        let eps = 1e-3 * (1.0 + u.abs());
        if ll > *u + eps {
            ret = 1;
            return ret;
        }
    }
    
    if *l != MIN {
        let eps = 1e-12 * (1.0 + l.abs());
        if ll > *l - eps {
            *l = MIN;
        }
    }
    
    if *u != MAX {
        let eps = 1e-12 * (1.0 + u.abs());
        if uu < *u + eps {
            *u = MAX;
        }
    }
    
    ret
}

// Additional functions would follow the same pattern of conversion
// The rest of the code would be similarly converted to safe Rust
// with proper error handling and without unsafe blocks