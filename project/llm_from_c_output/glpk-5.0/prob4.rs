use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct GlpError {
    message: String,
}

impl Error for GlpError {}

impl fmt::Display for GlpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

struct GlpProb {
    m: usize,
    n: usize,
    valid: bool,
    row: Vec<Row>,
    col: Vec<Col>,
}

struct Row {
    rii: f64,
    ptr: Option<Box<GLPAIJ>>,
}

struct Col {
    sjj: f64,
    stat: GlpStat,
}

struct GLPAIJ {
    col: Box<Col>,
    r_next: Option<Box<GLPAIJ>>,
}

#[derive(PartialEq)]
enum GlpStat {
    BS,
    // Other possible states
}

impl GlpProb {
    fn set_rii(&mut self, i: usize, rii: f64) -> Result<(), GlpError> {
        if !(1 <= i && i <= self.m) {
            return Err(GlpError {
                message: format!("glp_set_rii: i = {}; row number out of range", i),
            });
        }
        if rii <= 0.0 {
            return Err(GlpError {
                message: format!("glp_set_rii: i = {}; rii = {}; invalid scale factor", i, rii),
            });
        }

        let row = &mut self.row[i - 1];
        if self.valid && row.rii != rii {
            let mut aij = row.ptr.as_ref();
            while let Some(a) = aij {
                if a.col.stat == GlpStat::BS {
                    self.valid = false;
                    break;
                }
                aij = a.r_next.as_ref();
            }
        }
        row.rii = rii;
        Ok(())
    }

    fn set_sjj(&mut self, j: usize, sjj: f64) -> Result<(), GlpError> {
        if !(1 <= j && j <= self.n) {
            return Err(GlpError {
                message: format!("glp_set_sjj: j = {}; column number out of range", j),
            });
        }
        if sjj <= 0.0 {
            return Err(GlpError {
                message: format!("glp_set_sjj: j = {}; sjj = {}; invalid scale factor", j, sjj),
            });
        }

        let col = &mut self.col[j - 1];
        if self.valid && col.sjj != sjj && col.stat == GlpStat::BS {
            self.valid = false;
        }
        col.sjj = sjj;
        Ok(())
    }

    fn get_rii(&self, i: usize) -> Result<f64, GlpError> {
        if !(1 <= i && i <= self.m) {
            return Err(GlpError {
                message: format!("glp_get_rii: i = {}; row number out of range", i),
            });
        }
        Ok(self.row[i - 1].rii)
    }

    fn get_sjj(&self, j: usize) -> Result<f64, GlpError> {
        if !(1 <= j && j <= self.n) {
            return Err(GlpError {
                message: format!("glp_get_sjj: j = {}; column number out of range", j),
            });
        }
        Ok(self.col[j - 1].sjj)
    }

    fn unscale_prob(&mut self) -> Result<(), GlpError> {
        for i in 1..=self.m {
            self.set_rii(i, 1.0)?;
        }
        for j in 1..=self.n {
            self.set_sjj(j, 1.0)?;
        }
        Ok(())
    }

    fn get_num_rows(&self) -> usize {
        self.m
    }

    fn get_num_cols(&self) -> usize {
        self.n
    }
}