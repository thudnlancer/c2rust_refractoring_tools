use std::f64;
use std::mem;
use std::ptr;

const DBL_MAX: f64 = f64::MAX;

struct SVA {
    ind: Vec<i32>,
    val: Vec<f64>,
    ptr: Vec<i32>,
    len: Vec<i32>,
    cap: Vec<i32>,
    m_ptr: i32,
    r_ptr: i32,
}

impl SVA {
    fn new() -> Self {
        SVA {
            ind: Vec::new(),
            val: Vec::new(),
            ptr: Vec::new(),
            len: Vec::new(),
            cap: Vec::new(),
            m_ptr: 0,
            r_ptr: 0,
        }
    }

    fn more_space(&mut self, need: i32) {
        let new_size = self.val.len() + need as usize;
        self.ind.resize(new_size, 0);
        self.val.resize(new_size, 0.0);
    }

    fn reserve_cap(&mut self, ref_: i32, len: i32) {
        let idx = (ref_ - 1) as usize;
        self.ptr[idx] = self.m_ptr;
        self.len[idx] = len;
        self.cap[idx] = len;
        self.m_ptr += len;
    }

    fn make_static(&mut self, ref_: i32) {
        let idx = (ref_ - 1) as usize;
        self.ptr[idx] = self.r_ptr;
        self.r_ptr += self.len[idx];
    }

    fn enlarge_cap(&mut self, ref_: i32, need: i32, _: i32) {
        let idx = (ref_ - 1) as usize;
        self.cap[idx] = need;
    }

    fn defrag_area(&mut self) {
        // Simplified defragmentation
        self.m_ptr = 0;
        self.r_ptr = self.val.len() as i32;
    }
}

struct LUF {
    n: i32,
    sva: SVA,
    vr_ref: i32,
    vc_ref: i32,
    fc_ref: i32,
    pp_ind: Vec<i32>,
    pp_inv: Vec<i32>,
    qq_ind: Vec<i32>,
    qq_inv: Vec<i32>,
    vr_piv: Vec<f64>,
}

impl LUF {
    fn new(n: i32) -> Self {
        let size = n as usize;
        LUF {
            n,
            sva: SVA::new(),
            vr_ref: 1,
            vc_ref: 1,
            fc_ref: 1,
            pp_ind: vec![0; size + 1],
            pp_inv: vec![0; size + 1],
            qq_ind: vec![0; size + 1],
            qq_inv: vec![0; size + 1],
            vr_piv: vec![0.0; size + 1],
        }
    }

    fn swap_u_rows(&mut self, k: i32, i: i32) {
        self.pp_ind.swap(self.pp_inv[k] as usize, self.pp_inv[i] as usize);
        self.pp_inv.swap(k as usize, i as usize);
    }

    fn swap_u_cols(&mut self, k: i32, j: i32) {
        self.qq_ind.swap(self.qq_inv[k] as usize, self.qq_inv[j] as usize);
        self.qq_inv.swap(k as usize, j as usize);
    }

    fn build_v_rows(&self, _: &[i32]) {
        // Implementation omitted for brevity
    }

    fn build_f_rows(&self, _: &[i32]) {
        // Implementation omitted for brevity
    }

    fn build_v_cols(&self, _: bool, _: &[i32]) {
        // Implementation omitted for brevity
    }

    fn check_all(&self, _: i32) {
        // Debug check implementation omitted
    }
}

struct SGF {
    luf: LUF,
    rs_head: Vec<i32>,
    rs_prev: Vec<i32>,
    rs_next: Vec<i32>,
    cs_head: Vec<i32>,
    cs_prev: Vec<i32>,
    cs_next: Vec<i32>,
    vr_max: Vec<f64>,
    flag: Vec<bool>,
    work: Vec<f64>,
    updat: bool,
    piv_tol: f64,
    piv_lim: i32,
    suhl: bool,
    eps_tol: f64,
}

impl SGF {
    fn new(luf: LUF) -> Self {
        let n = luf.n as usize;
        SGF {
            luf,
            rs_head: vec![0; n + 1],
            rs_prev: vec![0; n + 1],
            rs_next: vec![0; n + 1],
            cs_head: vec![0; n + 1],
            cs_prev: vec![0; n + 1],
            cs_next: vec![0; n + 1],
            vr_max: vec![-1.0; n + 1],
            flag: vec![false; n + 1],
            work: vec![0.0; n + 1],
            updat: false,
            piv_tol: 0.1,
            piv_lim: 10,
            suhl: true,
            eps_tol: 1e-10,
        }
    }

    fn activate_row(&mut self, i: i32) {
        let len = self.luf.sva.len[(self.luf.vr_ref - 1 + i) as usize];
        self.rs_prev[i as usize] = 0;
        self.rs_next[i as usize] = self.rs_head[len as usize];
        if self.rs_next[i as usize] != 0 {
            self.rs_prev[self.rs_next[i as usize] as usize] = i;
        }
        self.rs_head[len as usize] = i;
    }

    fn deactivate_row(&mut self, i: i32) {
        let len = self.luf.sva.len[(self.luf.vr_ref - 1 + i) as usize];
        if self.rs_prev[i as usize] == 0 {
            self.rs_head[len as usize] = self.rs_next[i as usize];
        } else {
            self.rs_next[self.rs_prev[i as usize] as usize] = self.rs_next[i as usize];
        }
        if self.rs_next[i as usize] != 0 {
            self.rs_prev[self.rs_next[i as usize] as usize] = self.rs_prev[i as usize];
        }
        self.rs_prev[i as usize] = -1;
        self.rs_next[i as usize] = -1;
    }

    fn activate_col(&mut self, j: i32) {
        let len = self.luf.sva.len[(self.luf.vc_ref - 1 + j) as usize];
        self.cs_prev[j as usize] = 0;
        self.cs_next[j as usize] = self.cs_head[len as usize];
        if self.cs_next[j as usize] != 0 {
            self.cs_prev[self.cs_next[j as usize] as usize] = j;
        }
        self.cs_head[len as usize] = j;
    }

    fn deactivate_col(&mut self, j: i32) {
        let len = self.luf.sva.len[(self.luf.vc_ref - 1 + j) as usize];
        if self.cs_prev[j as usize] == 0 {
            self.cs_head[len as usize] = self.cs_next[j as usize];
        } else {
            self.cs_next[self.cs_prev[j as usize] as usize] = self.cs_next[j as usize];
        }
        if self.cs_next[j as usize] != 0 {
            self.cs_prev[self.cs_next[j as usize] as usize] = self.cs_prev[j as usize];
        }
        self.cs_prev[j as usize] = -1;
        self.cs_next[j as usize] = -1;
    }

    fn reduce_nuc(&mut self, k1_: &mut i32, k2_: &mut i32, cnt: &mut [i32], list: &mut [i32]) -> i32 {
        // Implementation omitted for brevity
        0
    }

    fn singl_phase(&mut self, k1: i32, k2: i32, updat: bool, ind: &mut [i32], val: &mut [f64]) -> i32 {
        // Implementation omitted for brevity
        k2
    }

    fn choose_pivot(&mut self, p: &mut i32, q: &mut i32) -> i32 {
        // Implementation omitted for brevity
        0
    }

    fn eliminate(&mut self, p: i32, q: i32) -> i32 {
        // Implementation omitted for brevity
        0
    }

    fn dense_lu(n: i32, a: &mut [f64], r: &mut [i32], c: &mut [i32], eps: f64) -> i32 {
        // Implementation omitted for brevity
        0
    }

    fn dense_phase(&mut self, k: i32) -> i32 {
        // Implementation omitted for brevity
        0
    }

    fn factorize(&mut self, singl: bool) -> i32 {
        // Implementation omitted for brevity
        0
    }
}

// Helper functions and remaining implementations would go here