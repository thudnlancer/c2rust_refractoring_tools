use std::f64;

pub enum SolutionType {
    Sol,
    Ipt,
    Mip,
}

pub enum ConditionType {
    Pe,
    Pb,
    De,
    Db,
    Cs,
}

pub enum RowType {
    Fr,
    Lo,
    Up,
    Db,
    Fx,
}

pub enum ColType {
    Fr,
    Lo,
    Up,
    Db,
    Fx,
}

pub enum ProblemDir {
    Min,
    Max,
}

pub struct GlpRow {
    pub prim: f64,
    pub pval: f64,
    pub mipx: f64,
    pub dual: f64,
    pub dval: f64,
    pub lb: f64,
    pub ub: f64,
    pub type_: RowType,
    pub ptr: Option<Box<GlpAij>>,
}

pub struct GlpCol {
    pub prim: f64,
    pub pval: f64,
    pub mipx: f64,
    pub dual: f64,
    pub dval: f64,
    pub coef: f64,
    pub lb: f64,
    pub ub: f64,
    pub type_: ColType,
    pub ptr: Option<Box<GlpAij>>,
}

pub struct GlpAij {
    pub val: f64,
    pub row: Box<GlpRow>,
    pub col: Box<GlpCol>,
    pub r_next: Option<Box<GlpAij>>,
    pub c_next: Option<Box<GlpAij>>,
}

pub struct GlpProb {
    pub m: i32,
    pub n: i32,
    pub dir: ProblemDir,
    pub row: Vec<GlpRow>,
    pub col: Vec<GlpCol>,
}

pub fn glp_check_kkt(
    p: &GlpProb,
    sol: SolutionType,
    cond: ConditionType,
    ae_max: &mut f64,
    ae_ind: &mut i32,
    re_max: &mut f64,
    re_ind: &mut i32,
) {
    let m = p.m;
    let n = p.n;
    let mut local_ae_max = 0.0;
    let mut local_ae_ind = 0;
    let mut local_re_max = 0.0;
    let mut local_re_ind = 0;

    match cond {
        ConditionType::Pe => {
            for i in 1..=m {
                let row = &p.row[i as usize - 1];
                let mut sp = 0.0;
                let mut sn = 0.0;
                
                let t = match sol {
                    SolutionType::Sol => row.prim,
                    SolutionType::Ipt => row.pval,
                    SolutionType::Mip => row.mipx,
                };
                
                if t >= 0.0 { sp += t; } else { sn -= t; }
                
                let mut aij = &row.ptr;
                while let Some(a) = aij {
                    let col = &a.col;
                    let t = match sol {
                        SolutionType::Sol => -a.val * col.prim,
                        SolutionType::Ipt => -a.val * col.pval,
                        SolutionType::Mip => -a.val * col.mipx,
                    };
                    
                    if t >= 0.0 { sp += t; } else { sn -= t; }
                    aij = &a.r_next;
                }
                
                let e = (sp - sn).abs();
                if local_ae_max < e {
                    local_ae_max = e;
                    local_ae_ind = i;
                }
                
                let e = e / (1.0 + sp + sn);
                if local_re_max < e {
                    local_re_max = e;
                    local_re_ind = i;
                }
            }
        }
        ConditionType::Pb => {
            for i in 1..=m {
                let row = &p.row[i as usize - 1];
                let t = match sol {
                    SolutionType::Sol => row.prim,
                    SolutionType::Ipt => row.pval,
                    SolutionType::Mip => row.mipx,
                };
                
                match row.type_ {
                    RowType::Lo | RowType::Db | RowType::Fx => {
                        if t < row.lb {
                            let e = row.lb - t;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = i;
                            }
                            let e = e / (1.0 + row.lb.abs());
                            if local_re_max < e {
                                local_re_max = e;
                                local_re_ind = i;
                            }
                        }
                    }
                    _ => {}
                }
                
                match row.type_ {
                    RowType::Up | RowType::Db | RowType::Fx => {
                        if t > row.ub {
                            let e = t - row.ub;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = i;
                            }
                            let e = e / (1.0 + row.ub.abs());
                            if local_re_max < e {
                                local_re_max = e;
                                local_re_ind = i;
                            }
                        }
                    }
                    _ => {}
                }
            }
            
            for j in 1..=n {
                let col = &p.col[j as usize - 1];
                let t = match sol {
                    SolutionType::Sol => col.prim,
                    SolutionType::Ipt => col.pval,
                    SolutionType::Mip => col.mipx,
                };
                
                match col.type_ {
                    ColType::Lo | ColType::Db | ColType::Fx => {
                        if t < col.lb {
                            let e = col.lb - t;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = m + j;
                            }
                            let e = e / (1.0 + col.lb.abs());
                            if local_re_max < e {
                                local_re_max = e;
                                local_re_ind = m + j;
                            }
                        }
                    }
                    _ => {}
                }
                
                match col.type_ {
                    ColType::Up | ColType::Db | ColType::Fx => {
                        if t > col.ub {
                            let e = t - col.ub;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = m + j;
                            }
                            let e = e / (1.0 + col.ub.abs());
                            if local_re_max < e {
                                local_re_max = e;
                                local_re_ind = m + j;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        ConditionType::De => {
            for j in 1..=n {
                let col = &p.col[j as usize - 1];
                let mut sp = 0.0;
                let mut sn = 0.0;
                
                let t = match sol {
                    SolutionType::Sol => col.dual - col.coef,
                    SolutionType::Ipt => col.dval - col.coef,
                    _ => panic!("Invalid solution type for DE condition"),
                };
                
                if t >= 0.0 { sp += t; } else { sn -= t; }
                
                let mut aij = &col.ptr;
                while let Some(a) = aij {
                    let row = &a.row;
                    let t = match sol {
                        SolutionType::Sol => a.val * row.dual,
                        SolutionType::Ipt => a.val * row.dval,
                        _ => panic!("Invalid solution type for DE condition"),
                    };
                    
                    if t >= 0.0 { sp += t; } else { sn -= t; }
                    aij = &a.c_next;
                }
                
                let e = (sp - sn).abs();
                if local_ae_max < e {
                    local_ae_max = e;
                    local_ae_ind = m + j;
                }
                
                let e = e / (1.0 + sp + sn);
                if local_re_max < e {
                    local_re_max = e;
                    local_re_ind = m + j;
                }
            }
        }
        ConditionType::Db => {
            for i in 1..=m {
                let row = &p.row[i as usize - 1];
                let mut t = match sol {
                    SolutionType::Sol => row.dual,
                    SolutionType::Ipt => row.dval,
                    _ => panic!("Invalid solution type for DB condition"),
                };
                
                t = match p.dir {
                    ProblemDir::Min => t,
                    ProblemDir::Max => -t,
                };
                
                match row.type_ {
                    RowType::Fr | RowType::Lo => {
                        if t < 0.0 {
                            let e = -t;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = i;
                                local_re_max = e;
                                local_re_ind = i;
                            }
                        }
                    }
                    _ => {}
                }
                
                match row.type_ {
                    RowType::Fr | RowType::Up => {
                        if t > 0.0 {
                            let e = t;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = i;
                                local_re_max = e;
                                local_re_ind = i;
                            }
                        }
                    }
                    _ => {}
                }
            }
            
            for j in 1..=n {
                let col = &p.col[j as usize - 1];
                let mut t = match sol {
                    SolutionType::Sol => col.dual,
                    SolutionType::Ipt => col.dval,
                    _ => panic!("Invalid solution type for DB condition"),
                };
                
                t = match p.dir {
                    ProblemDir::Min => t,
                    ProblemDir::Max => -t,
                };
                
                match col.type_ {
                    ColType::Fr | ColType::Lo => {
                        if t < 0.0 {
                            let e = -t;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = m + j;
                                local_re_max = e;
                                local_re_ind = m + j;
                            }
                        }
                    }
                    _ => {}
                }
                
                match col.type_ {
                    ColType::Fr | ColType::Up => {
                        if t > 0.0 {
                            let e = t;
                            if local_ae_max < e {
                                local_ae_max = e;
                                local_ae_ind = m + j;
                                local_re_max = e;
                                local_re_ind = m + j;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        ConditionType::Cs => {
            // Complementary slackness condition not implemented in original C code
        }
    }

    *ae_max = local_ae_max;
    *ae_ind = local_ae_ind;
    *re_max = local_re_max;
    *re_ind = local_re_ind;
}