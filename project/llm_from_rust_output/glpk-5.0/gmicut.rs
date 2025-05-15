use std::f64::consts::EPSILON;

#[derive(Debug, Clone)]
pub struct GlpProb {
    pub m: i32,
    pub n: i32,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub valid: i32,
    pub col: Vec<GlpCol>,
    pub row: Vec<GlpRow>,
}

#[derive(Debug, Clone)]
pub struct GlpCol {
    pub j: i32,
    pub kind: i32,
    pub type_: i32,
    pub stat: i32,
    pub lb: f64,
    pub ub: f64,
    pub prim: f64,
    pub ptr: Option<Box<GlpAij>>,
}

#[derive(Debug, Clone)]
pub struct GlpRow {
    pub i: i32,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub ptr: Option<Box<GlpAij>>,
}

#[derive(Debug, Clone)]
pub struct GlpAij {
    pub row: Option<Box<GlpRow>>,
    pub col: Option<Box<GlpCol>>,
    pub val: f64,
    pub r_next: Option<Box<GlpAij>>,
}

pub fn glp_gmi_cut(
    p: &mut GlpProb,
    j: i32,
    ind: &mut [i32],
    val: &mut [f64],
    phi: &mut [f64],
) -> i32 {
    if p.m == 0 || p.valid == 0 {
        return -1;
    }
    if !(p.pbs_stat == 2 && p.dbs_stat == 2) {
        return -2;
    }
    if !(1 <= j && j <= p.n) {
        return -3;
    }

    let col = &p.col[(j - 1) as usize];
    if col.kind != 2 {
        return -4;
    }
    if col.type_ == 5 || col.stat != 1 {
        return -5;
    }
    if (col.prim - (col.prim + 0.5).floor()).abs() < 0.001 {
        return -6;
    }

    // Simulate glp_eval_tab_row (simplified for example)
    let len = p.n; // Simplified assumption
    let beta = col.prim;

    phi.iter_mut().for_each(|x| *x = 0.0);
    let mut rhs = beta - beta.floor();

    for jj in 1..=len {
        let k = ind[(jj - 1) as usize];
        assert!(1 <= k && k <= p.m + p.n, "1 <= k && k <= m+n");

        let (kind, lb, ub, stat) = if k <= p.m {
            let row = &p.row[(k - 1) as usize];
            (1, row.lb, row.ub, row.type_)
        } else {
            let col = &p.col[(k - p.m - 1) as usize];
            (col.kind, col.lb, col.ub, col.stat)
        };

        assert_ne!(stat, 1, "stat != GLP_BS");

        let ksi = val[(jj - 1) as usize];
        if ksi.abs() > 1e5 {
            return -7;
        }

        if ksi.abs() >= 1e-10 {
            if stat == 4 {
                return -8;
            }

            let alfa = match stat {
                2 => -ksi,
                3 => ksi,
                5 => continue,
                _ => panic!("Invalid stat"),
            };

            if kind == 2 {
                if (alfa - (alfa + 0.5).floor()).abs() < 1e-10 {
                    continue;
                }

                let phi1 = if alfa - alfa.floor() <= beta - beta.floor() {
                    alfa - alfa.floor()
                } else {
                    (beta - beta.floor()) / (1.0 - (beta - beta.floor())) * (1.0 - (alfa - alfa.floor()))
                };

                match stat {
                    2 => {
                        phi[(k - 1) as usize] = phi1;
                        rhs += phi1 * lb;
                    }
                    3 => {
                        phi[(k - 1) as usize] = -phi1;
                        rhs -= phi1 * ub;
                    }
                    _ => panic!("Invalid stat"),
                }
            } else if kind == 1 {
                let phi1 = if alfa >= 0.0 {
                    alfa
                } else {
                    (beta - beta.floor()) / (1.0 - (beta - beta.floor())) * -alfa
                };

                match stat {
                    2 => {
                        phi[(k - 1) as usize] = phi1;
                        rhs += phi1 * lb;
                    }
                    3 => {
                        phi[(k - 1) as usize] = -phi1;
                        rhs -= phi1 * ub;
                    }
                    _ => panic!("Invalid stat"),
                }
            } else {
                panic!("Invalid kind");
            }
        }
    }

    for i in 1..=p.m {
        if phi[(i - 1) as usize].abs() >= 1e-10 {
            let row = &p.row[(i - 1) as usize];
            assert_ne!(row.type_, 5, "row->type != GLP_FX");

            let mut aij = &row.ptr;
            while let Some(a) = aij {
                phi[(p.m + a.col.as_ref().unwrap().j - 1) as usize] +=
                    phi[(i - 1) as usize] * a.val;
                aij = &a.r_next;
            }
        }
    }

    let mut new_len = 0;
    for jj in 1..=p.n {
        if phi[(p.m + jj - 1) as usize].abs() >= 1e-10 {
            let col = &p.col[(jj - 1) as usize];
            if col.type_ == 5 {
                rhs -= phi[(p.m + jj - 1) as usize] * col.lb;
            } else {
                new_len += 1;
                ind[new_len as usize] = jj;
                val[new_len as usize] = phi[(p.m + jj - 1) as usize];
            }
        }
    }

    if rhs.abs() < 1e-12 {
        rhs = 0.0;
    }

    ind[0] = 0;
    val[0] = rhs;
    new_len
}