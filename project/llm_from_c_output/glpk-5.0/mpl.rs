use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::f64;

// Assuming these are defined in external crates or modules
mod glpk {
    pub type Prob;
    pub type Tran;
    
    pub enum SolType {
        Sol,
        Ipt,
        Mip,
    }
    
    pub enum BoundType {
        Fr,
        Lo,
        Up,
        Db,
        Fx,
    }
    
    pub enum ObjDir {
        Min,
        Max,
    }
    
    pub enum RowKind {
        Min,
        Max,
        Num,
        Int,
        Bin,
    }
    
    // Function prototypes
    pub fn mpl_initialize() -> *mut Tran;
    pub fn rng_init_rand(rand: *mut (), seed: i32);
    pub fn mpl_read_model(tran: *mut Tran, fname: *const c_char, skip: i32) -> i32;
    pub fn mpl_read_data(tran: *mut Tran, fname: *const c_char) -> i32;
    pub fn mpl_generate(tran: *mut Tran, fname: *const c_char) -> i32;
    pub fn mpl_get_prob_name(tran: *mut Tran) -> *const c_char;
    pub fn mpl_get_num_rows(tran: *mut Tran) -> i32;
    pub fn mpl_get_row_name(tran: *mut Tran, i: i32) -> *const c_char;
    pub fn mpl_get_row_bnds(tran: *mut Tran, i: i32, lb: *mut f64, ub: *mut f64) -> i32;
    pub fn mpl_get_row_c0(tran: *mut Tran, i: i32) -> f64;
    pub fn mpl_get_num_cols(tran: *mut Tran) -> i32;
    pub fn mpl_get_col_name(tran: *mut Tran, j: i32) -> *const c_char;
    pub fn mpl_get_col_kind(tran: *mut Tran, j: i32) -> i32;
    pub fn mpl_get_col_bnds(tran: *mut Tran, j: i32, lb: *mut f64, ub: *mut f64) -> i32;
    pub fn mpl_get_mat_row(tran: *mut Tran, i: i32, ind: *mut i32, val: *mut f64) -> i32;
    pub fn mpl_get_row_kind(tran: *mut Tran, i: i32) -> i32;
    pub fn mpl_has_solve_stmt(tran: *mut Tran) -> bool;
    pub fn mpl_put_row_soln(tran: *mut Tran, i: i32, stat: i32, prim: f64, dual: f64);
    pub fn mpl_put_col_soln(tran: *mut Tran, j: i32, stat: i32, prim: f64, dual: f64);
    pub fn mpl_postsolve(tran: *mut Tran) -> i32;
    pub fn mpl_terminate(tran: *mut Tran);
    
    // GLPK functions
    pub fn glp_erase_prob(prob: *mut Prob);
    pub fn glp_set_prob_name(prob: *mut Prob, name: *const c_char);
    pub fn glp_add_rows(prob: *mut Prob, n: i32);
    pub fn glp_set_row_name(prob: *mut Prob, i: i32, name: *const c_char);
    pub fn glp_set_row_bnds(prob: *mut Prob, i: i32, typ: i32, lb: f64, ub: f64);
    pub fn glp_add_cols(prob: *mut Prob, n: i32);
    pub fn glp_set_col_name(prob: *mut Prob, j: i32, name: *const c_char);
    pub fn glp_set_col_kind(prob: *mut Prob, j: i32, kind: i32);
    pub fn glp_set_col_bnds(prob: *mut Prob, j: i32, typ: i32, lb: f64, ub: f64);
    pub fn glp_set_mat_row(prob: *mut Prob, i: i32, len: i32, ind: *const i32, val: *const f64);
    pub fn glp_set_obj_name(prob: *mut Prob, name: *const c_char);
    pub fn glp_set_obj_dir(prob: *mut Prob, dir: i32);
    pub fn glp_set_obj_coef(prob: *mut Prob, j: i32, coef: f64);
    pub fn glp_get_num_rows(prob: *mut Prob) -> i32;
    pub fn glp_get_num_cols(prob: *mut Prob) -> i32;
    pub fn glp_get_row_stat(prob: *mut Prob, i: i32) -> i32;
    pub fn glp_get_row_prim(prob: *mut Prob, i: i32) -> f64;
    pub fn glp_get_row_dual(prob: *mut Prob, i: i32) -> f64;
    pub fn glp_get_col_stat(prob: *mut Prob, j: i32) -> i32;
    pub fn glp_get_col_prim(prob: *mut Prob, j: i32) -> f64;
    pub fn glp_get_col_dual(prob: *mut Prob, j: i32) -> f64;
    pub fn glp_ipt_row_prim(prob: *mut Prob, i: i32) -> f64;
    pub fn glp_ipt_row_dual(prob: *mut Prob, i: i32) -> f64;
    pub fn glp_ipt_col_prim(prob: *mut Prob, j: i32) -> f64;
    pub fn glp_ipt_col_dual(prob: *mut Prob, j: i32) -> f64;
    pub fn glp_mip_row_val(prob: *mut Prob, i: i32) -> f64;
    pub fn glp_mip_col_val(prob: *mut Prob, j: i32) -> f64;
}

pub struct MplWorkspace {
    tran: *mut glpk::Tran,
}

impl MplWorkspace {
    pub fn new() -> Result<Self, &'static str> {
        let tran = unsafe { glpk::mpl_initialize() };
        if tran.is_null() {
            Err("Failed to initialize MPL workspace")
        } else {
            Ok(MplWorkspace { tran })
        }
    }

    pub fn init_rand(&mut self, seed: i32) -> Result<(), &'static str> {
        unsafe {
            if (*self.tran).phase != 0 {
                return Err("glp_mpl_init_rand: invalid call sequence");
            }
            glpk::rng_init_rand((*self.tran).rand, seed);
        }
        Ok(())
    }

    pub fn read_model(&mut self, fname: &str, skip: bool) -> Result<i32, &'static str> {
        let c_fname = CString::new(fname).unwrap();
        let skip = if skip { 1 } else { 0 };
        
        unsafe {
            if (*self.tran).phase != 0 {
                return Err("glp_mpl_read_model: invalid call sequence");
            }
            
            let ret = glpk::mpl_read_model(self.tran, c_fname.as_ptr(), skip);
            match ret {
                1 | 2 => Ok(0),
                4 => Ok(1),
                _ => Err("Unexpected return value from mpl_read_model"),
            }
        }
    }

    pub fn read_data(&mut self, fname: &str) -> Result<i32, &'static str> {
        let c_fname = CString::new(fname).unwrap();
        
        unsafe {
            if (*self.tran).phase != 1 && (*self.tran).phase != 2 {
                return Err("glp_mpl_read_data: invalid call sequence");
            }
            
            let ret = glpk::mpl_read_data(self.tran, c_fname.as_ptr());
            match ret {
                2 => Ok(0),
                4 => Ok(1),
                _ => Err("Unexpected return value from mpl_read_data"),
            }
        }
    }

    pub fn generate(&mut self, fname: &str) -> Result<i32, &'static str> {
        let c_fname = CString::new(fname).unwrap();
        
        unsafe {
            if (*self.tran).phase != 1 && (*self.tran).phase != 2 {
                return Err("glp_mpl_generate: invalid call sequence");
            }
            
            let ret = glpk::mpl_generate(self.tran, c_fname.as_ptr());
            match ret {
                3 => Ok(0),
                4 => Ok(1),
                _ => Err("Unexpected return value from mpl_generate"),
            }
        }
    }

    pub fn build_prob(&mut self, prob: *mut glpk::Prob) -> Result<(), &'static str> {
        unsafe {
            if (*self.tran).phase != 3 {
                return Err("glp_mpl_build_prob: invalid call sequence");
            }
            
            // Erase the problem object
            glpk::glp_erase_prob(prob);
            
            // Set problem name
            let prob_name = glpk::mpl_get_prob_name(self.tran);
            glpk::glp_set_prob_name(prob, prob_name);
            
            // Build rows (constraints)
            let m = glpk::mpl_get_num_rows(self.tran);
            if m > 0 {
                glpk::glp_add_rows(prob, m);
            }
            
            for i in 1..=m {
                // Set row name
                let row_name = glpk::mpl_get_row_name(self.tran, i);
                glpk::glp_set_row_name(prob, i, row_name);
                
                // Set row bounds
                let mut lb = 0.0;
                let mut ub = 0.0;
                let type_ = glpk::mpl_get_row_bnds(self.tran, i, &mut lb, &mut ub);
                let typ = match type_ {
                    0 => glpk::BoundType::Fr,
                    1 => glpk::BoundType::Lo,
                    2 => glpk::BoundType::Up,
                    3 => glpk::BoundType::Db,
                    4 => glpk::BoundType::Fx,
                    _ => return Err("Invalid bound type"),
                };
                
                if let glpk::BoundType::Db = typ {
                    if f64::abs(lb - ub) < 1e-9 * (1.0 + f64::abs(lb)) {
                        if f64::abs(lb) <= f64::abs(ub) {
                            ub = lb;
                        } else {
                            lb = ub;
                        }
                    }
                }
                
                glpk::glp_set_row_bnds(prob, i, typ as i32, lb, ub);
                
                // Warn about non-zero constant term
                let c0 = glpk::mpl_get_row_c0(self.tran, i);
                if c0 != 0.0 {
                    println!("glp_mpl_build_prob: row {}; constant term {} ignored",
                        CString::from_raw(glpk::mpl_get_row_name(self.tran, i) as *mut c_char)
                            .into_string().unwrap(),
                        c0);
                }
            }
            
            // Build columns (variables)
            let n = glpk::mpl_get_num_cols(self.tran);
            if n > 0 {
                glpk::glp_add_cols(prob, n);
            }
            
            for j in 1..=n {
                // Set column name
                let col_name = glpk::mpl_get_col_name(self.tran, j);
                glpk::glp_set_col_name(prob, j, col_name);
                
                // Set column kind
                let kind = glpk::mpl_get_col_kind(self.tran, j);
                match kind {
                    0 => (), // MPL_NUM
                    1 | 2 => glpk::glp_set_col_kind(prob, j, 1), // GLP_IV
                    _ => return Err("Invalid column kind"),
                }
                
                // Set column bounds
                let mut lb = 0.0;
                let mut ub = 0.0;
                let type_ = glpk::mpl_get_col_bnds(self.tran, j, &mut lb, &mut ub);
                let typ = match type_ {
                    0 => glpk::BoundType::Fr,
                    1 => glpk::BoundType::Lo,
                    2 => glpk::BoundType::Up,
                    3 => glpk::BoundType::Db,
                    4 => glpk::BoundType::Fx,
                    _ => return Err("Invalid bound type"),
                };
                
                if kind == 2 { // MPL_BIN
                    if matches!(typ, glpk::BoundType::Fr | glpk::BoundType::Up) || lb < 0.0 {
                        lb = 0.0;
                    }
                    if matches!(typ, glpk::BoundType::Fr | glpk::BoundType::Lo) || ub > 1.0 {
                        ub = 1.0;
                    }
                }
                
                if let glpk::BoundType::Db = typ {
                    if f64::abs(lb - ub) < 1e-9 * (1.0 + f64::abs(lb)) {
                        if f64::abs(lb) <= f64::abs(ub) {
                            ub = lb;
                        } else {
                            lb = ub;
                        }
                    }
                }
                
                glpk::glp_set_col_bnds(prob, j, typ as i32, lb, ub);
            }
            
            // Load the constraint matrix
            let mut ind: Vec<i32> = vec![0; (n + 1) as usize];
            let mut val: Vec<f64> = vec![0.0; (n + 1) as usize];
            
            for i in 1..=m {
                let len = glpk::mpl_get_mat_row(self.tran, i, ind.as_mut_ptr(), val.as_mut_ptr());
                glpk::glp_set_mat_row(prob, i, len, ind.as_ptr(), val.as_ptr());
            }
            
            // Build objective function (the first objective is used)
            for i in 1..=m {
                let kind = glpk::mpl_get_row_kind(self.tran, i);
                if kind == 0 || kind == 1 { // MPL_MIN or MPL_MAX
                    // Set objective name
                    glpk::glp_set_obj_name(prob, glpk::mpl_get_row_name(self.tran, i));
                    
                    // Set optimization direction
                    let dir = if kind == 0 { glpk::ObjDir::Min } else { glpk::ObjDir::Max };
                    glpk::glp_set_obj_dir(prob, dir as i32);
                    
                    // Set constant term
                    glpk::glp_set_obj_coef(prob, 0, glpk::mpl_get_row_c0(self.tran, i));
                    
                    // Set objective coefficients
                    let len = glpk::mpl_get_mat_row(self.tran, i, ind.as_mut_ptr(), val.as_mut_ptr());
                    for t in 1..=len {
                        glpk::glp_set_obj_coef(prob, ind[t as usize], val[t as usize]);
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn postsolve(&mut self, prob: *mut glpk::Prob, sol: glpk::SolType) -> Result<i32, &'static str> {
        unsafe {
            if !((*self.tran).phase == 3 && !(*self.tran).flag_p) {
                return Err("glp_mpl_postsolve: invalid call sequence");
            }
            
            let m = glpk::mpl_get_num_rows(self.tran);
            let n = glpk::mpl_get_num_cols(self.tran);
            
            if !(m == glpk::glp_get_num_rows(prob) && n == glpk::glp_get_num_cols(prob)) {
                return Err("glp_mpl_postsolve: wrong problem object");
            }
            
            if !glpk::mpl_has_solve_stmt(self.tran) {
                return Ok(0);
            }
            
            for i in 1..=m {
                let (stat, prim, dual) = match sol {
                    glpk::SolType::Sol => (
                        glpk::glp_get_row_stat(prob, i),
                        glpk::glp_get_row_prim(prob, i),
                        glpk::glp_get_row_dual(prob, i),
                    ),
                    glpk::SolType::Ipt => (
                        0,
                        glpk::glp_ipt_row_prim(prob, i),
                        glpk::glp_ipt_row_dual(prob, i),
                    ),
                    glpk::SolType::Mip => (
                        0,
                        glpk::glp_mip_row_val(prob, i),
                        0.0,
                    ),
                };
                
                let prim = if f64::abs(prim) < 1e-9 { 0.0 } else { prim };
                let dual = if f64::abs(dual) < 1e-9 { 0.0 } else { dual };
                
                glpk::mpl_put_row_soln(self.tran, i, stat, prim, dual);
            }
            
            for j in 1..=n {
                let (stat, prim, dual) = match sol {
                    glpk::SolType::Sol => (
                        glpk::glp_get_col_stat(prob, j),
                        glpk::glp_get_col_prim(prob, j),
                        glpk::glp_get_col_dual(prob, j),
                    ),
                    glpk::SolType::Ipt => (
                        0,
                        glpk::glp_ipt_col_prim(prob, j),
                        glpk::glp_ipt_col_dual(prob, j),
                    ),
                    glpk::SolType::Mip => (
                        0,
                        glpk::glp_mip_col_val(prob, j),
                        0.0,
                    ),
               