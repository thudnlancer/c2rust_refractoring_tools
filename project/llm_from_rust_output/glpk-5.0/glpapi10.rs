use std::f64;

#[derive(Debug, Clone, Copy)]
pub enum SolutionType {
    Primal = 1,
    PrimalVal = 2,
    MIP = 3,
}

#[derive(Debug, Clone, Copy)]
pub enum ConditionType {
    Feasibility = 1,
    PrimalFeasibility = 2,
    DualFeasibility = 3,
    ComplementarySlackness = 4,
    Other = 5,
}

#[derive(Debug)]
pub struct GlpProb {
    pub m: i32,
    pub n: i32,
    pub dir: i32,
    pub row: Vec<GlpRow>,
    pub col: Vec<GlpCol>,
}

#[derive(Debug)]
pub struct GlpRow {
    pub i: i32,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub prim: f64,
    pub pval: f64,
    pub dual: f64,
    pub dval: f64,
    pub mipx: f64,
    pub ptr: Vec<GlpAij>,
}

#[derive(Debug)]
pub struct GlpCol {
    pub j: i32,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub coef: f64,
    pub prim: f64,
    pub pval: f64,
    pub dual: f64,
    pub dval: f64,
    pub mipx: f64,
    pub ptr: Vec<GlpAij>,
}

#[derive(Debug)]
pub struct GlpAij {
    pub row: usize,
    pub col: usize,
    pub val: f64,
}

pub fn glp_check_kkt(
    p: &GlpProb,
    sol: SolutionType,
    cond: ConditionType,
) -> (f64, i32, f64, i32) {
    let mut ae_max = 0.0;
    let mut ae_ind = 0;
    let mut re_max = 0.0;
    let mut re_ind = 0;

    match cond {
        ConditionType::Feasibility => {
            for (i, row) in p.row.iter().enumerate() {
                let i = i + 1;
                let t = match sol {
                    SolutionType::Primal => row.prim,
                    SolutionType::PrimalVal => row.pval,
                    SolutionType::MIP => row.mipx,
                };

                let mut sp = if t >= 0.0 { t } else { 0.0 };
                let mut sn = if t < 0.0 { -t } else { 0.0 };

                for aij in &row.ptr {
                    let col = &p.col[aij.col - 1];
                    let t = match sol {
                        SolutionType::Primal => -aij.val * col.prim,
                        SolutionType::PrimalVal => -aij.val * col.pval,
                        SolutionType::MIP => -aij.val * col.mipx,
                    };

                    if t >= 0.0 {
                        sp += t;
                    } else {
                        sn -= t;
                    }
                }

                let e = (sp - sn).abs();
                if ae_max < e {
                    ae_max = e;
                    ae_ind = i as i32;
                }

                let e = e / (1.0 + sp + sn);
                if re_max < e {
                    re_max = e;
                    re_ind = i as i32;
                }
            }
        }
        ConditionType::PrimalFeasibility => {
            for (i, row) in p.row.iter().enumerate() {
                let i = i + 1;
                let t = match sol {
                    SolutionType::Primal => row.prim,
                    SolutionType::PrimalVal => row.pval,
                    SolutionType::MIP => row.mipx,
                };

                if row.type_ == 2 || row.type_ == 4 || row.type_ == 5 {
                    if t < row.lb {
                        let e = row.lb - t;
                        if ae_max < e {
                            ae_max = e;
                            ae_ind = i as i32;
                        }
                        let e = e / (1.0 + row.lb.abs());
                        if re_max < e {
                            re_max = e;
                            re_ind = i as i32;
                        }
                    }
                }

                if row.type_ == 3 || row.type_ == 4 || row.type_ == 5 {
                    if t > row.ub {
                        let e = t - row.ub;
                        if ae_max < e {
                            ae_max = e;
                            ae_ind = i as i32;
                        }
                        let e = e / (1.0 + row.ub.abs());
                        if re_max < e {
                            re_max = e;
                            re_ind = i as i32;
                        }
                    }
                }
            }

            for (j, col) in p.col.iter().enumerate() {
                let j = j + 1;
                let t = match sol {
                    SolutionType::Primal => col.prim,
                    SolutionType::PrimalVal => col.pval,
                    SolutionType::MIP => col.mipx,
                };

                if col.type_ == 2 || col.type_ == 4 || col.type_ == 5 {
                    if t < col.lb {
                        let e = col.lb - t;
                        if ae_max < e {
                            ae_max = e;
                            ae_ind = (p.m + j) as i32;
                        }
                        let e = e / (1.0 + col.lb.abs());
                        if re_max < e {
                            re_max = e;
                            re_ind = (p.m + j) as i32;
                        }
                    }
                }

                if col.type_ == 3 || col.type_ == 4 || col.type_ == 5 {
                    if t > col.ub {
                        let e = t - col.ub;
                        if ae_max < e {
                            ae_max = e;
                            ae_ind = (p.m + j) as i32;
                        }
                        let e = e / (1.0 + col.ub.abs());
                        if re_max < e {
                            re_max = e;
                            re_ind = (p.m + j) as i32;
                        }
                    }
                }
            }
        }
        ConditionType::DualFeasibility => {
            for (j, col) in p.col.iter().enumerate() {
                let j = j + 1;
                let t = match sol {
                    SolutionType::Primal => col.dual - col.coef,
                    SolutionType::PrimalVal => col.dval - col.coef,
                    SolutionType::MIP => 0.0,
                };

                let mut sp = if t >= 0.0 { t } else { 0.0 };
                let mut sn = if t < 0.0 { -t } else { 0.0 };

                for aij in &col.ptr {
                    let row = &p.row[aij.row - 1];
                    let t = match sol {
                        SolutionType::Primal => aij.val * row.dual,
                        SolutionType::PrimalVal => aij.val * row.dval,
                        SolutionType::MIP => 0.0,
                    };

                    if t >= 0.0 {
                        sp += t;
                    } else {
                        sn -= t;
                    }
                }

                let e = (sp - sn).abs();
                if ae_max < e {
                    ae_max = e;
                    ae_ind = (p.m + j) as i32;
                }

                let e = e / (1.0 + sp + sn);
                if re_max < e {
                    re_max = e;
                    re_ind = (p.m + j) as i32;
                }
            }
        }
        ConditionType::ComplementarySlackness => {
            for (i, row) in p.row.iter().enumerate() {
                let i = i + 1;
                let mut t = match sol {
                    SolutionType::Primal => row.dual,
                    SolutionType::PrimalVal => row.dval,
                    SolutionType::MIP => 0.0,
                };

                t = match p.dir {
                    1 => t,
                    2 => -t,
                    _ => t,
                };

                if row.type_ == 1 || row.type_ == 2 {
                    if t < 0.0 {
                        let e = -t;
                        if ae_max < e {
                            re_max = e;
                            ae_max = re_max;
                            re_ind = i as i32;
                            ae_ind = re_ind;
                        }
                    }
                }

                if row.type_ == 1 || row.type_ == 3 {
                    if t > 0.0 {
                        let e = t;
                        if ae_max < e {
                            re_max = e;
                            ae_max = re_max;
                            re_ind = i as i32;
                            ae_ind = re_ind;
                        }
                    }
                }
            }

            for (j, col) in p.col.iter().enumerate() {
                let j = j + 1;
                let mut t = match sol {
                    SolutionType::Primal => col.dual,
                    SolutionType::PrimalVal => col.dval,
                    SolutionType::MIP => 0.0,
                };

                t = match p.dir {
                    1 => t,
                    2 => -t,
                    _ => t,
                };

                if col.type_ == 1 || col.type_ == 2 {
                    if t < 0.0 {
                        let e = -t;
                        if ae_max < e {
                            re_max = e;
                            ae_max = re_max;
                            re_ind = (p.m + j) as i32;
                            ae_ind = re_ind;
                        }
                    }
                }

                if col.type_ == 1 || col.type_ == 3 {
                    if t > 0.0 {
                        let e = t;
                        if ae_max < e {
                            re_max = e;
                            ae_max = re_max;
                            re_ind = (p.m + j) as i32;
                            ae_ind = re_ind;
                        }
                    }
                }
            }
        }
        ConditionType::Other => {}
    }

    (ae_max, ae_ind, re_max, re_ind)
}