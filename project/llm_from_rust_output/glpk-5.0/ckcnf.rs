use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_double, c_uchar};

#[derive(Debug, Clone)]
pub struct GlpProb {
    pub name: Option<String>,
    pub obj: Option<String>,
    pub dir: c_int,
    pub c0: c_double,
    pub m: c_int,
    pub n: c_int,
    pub rows: Vec<GlpRow>,
    pub cols: Vec<GlpCol>,
    pub valid: c_int,
    pub pbs_stat: c_int,
    pub dbs_stat: c_int,
    pub obj_val: c_double,
    pub it_cnt: c_int,
    pub some: c_int,
    pub ipt_stat: c_int,
    pub ipt_obj: c_double,
    pub mip_stat: c_int,
    pub mip_obj: c_double,
}

#[derive(Debug, Clone)]
pub struct GlpCol {
    pub j: c_int,
    pub name: Option<String>,
    pub kind: c_int,
    pub type_: c_int,
    pub lb: c_double,
    pub ub: c_double,
    pub coef: c_double,
    pub aij: Vec<GlpAij>,
    pub sjj: c_double,
    pub stat: c_int,
    pub bind: c_int,
    pub prim: c_double,
    pub dual: c_double,
    pub pval: c_double,
    pub dval: c_double,
    pub mipx: c_double,
}

#[derive(Debug, Clone)]
pub struct GlpAij {
    pub row_idx: c_int,
    pub col_idx: c_int,
    pub val: c_double,
}

#[derive(Debug, Clone)]
pub struct GlpRow {
    pub i: c_int,
    pub name: Option<String>,
    pub level: c_int,
    pub origin: c_uchar,
    pub klass: c_uchar,
    pub type_: c_int,
    pub lb: c_double,
    pub ub: c_double,
    pub aij: Vec<GlpAij>,
    pub rii: c_double,
    pub stat: c_int,
    pub bind: c_int,
    pub prim: c_double,
    pub dual: c_double,
    pub pval: c_double,
    pub dval: c_double,
    pub mipx: c_double,
}

pub fn glp_check_cnfsat(p: &GlpProb) -> c_int {
    // Check columns
    for col in &p.cols {
        if !(col.kind == 2 && col.type_ == 4 && col.lb == 0.0 && col.ub == 1.0) {
            return 1;
        }
        if col.coef != 0.0 {
            return 3;
        }
    }

    if p.c0 != 0.0 {
        return 2;
    }

    // Check rows
    for row in &p.rows {
        if row.type_ != 2 {
            return 4;
        }

        let mut neg = 0;
        for aij in &row.aij {
            if aij.val == -1.0 {
                neg += 1;
            } else if aij.val != 1.0 {
                return 5;
            }
        }

        if row.lb != (1 - neg) as c_double {
            return 6;
        }
    }

    0
}