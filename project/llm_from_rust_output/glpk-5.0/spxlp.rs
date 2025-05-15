use std::f64;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;

struct BFD;

#[derive(Debug, Clone)]
struct FVS {
    n: i32,
    nnz: i32,
    ind: Vec<i32>,
    vec: Vec<f64>,
}

impl FVS {
    fn new(n: i32) -> Self {
        FVS {
            n,
            nnz: 0,
            ind: vec![0; (n + 1) as usize],
            vec: vec![0.0; (n + 1) as usize],
        }
    }

    fn clear_vec(&mut self) {
        self.nnz = 0;
        self.vec.iter_mut().for_each(|x| *x = 0.0);
    }
}

#[derive(Debug, Clone)]
struct SPXLP {
    m: i32,
    n: i32,
    nnz: i32,
    a_ptr: Vec<i32>,
    a_ind: Vec<i32>,
    a_val: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    l: Vec<f64>,
    u: Vec<f64>,
    head: Vec<i32>,
    flag: Vec<i8>,
    valid: i32,
    bfd: Box<BFD>,
}

impl SPXLP {
    fn jth_col(&self, j: i32) -> (Vec<i32>, Vec<f64>) {
        assert!(1 <= j && j <= self.m, "1 <= j && j <= m");
        let k = self.head[j as usize];
        let ptr = self.a_ptr[k as usize];
        let len = self.a_ptr[(k + 1) as usize] - ptr;

        let ind = self.a_ind[ptr as usize..(ptr + len) as usize].to_vec();
        let val = self.a_val[ptr as usize..(ptr + len) as usize].to_vec();

        (ind, val)
    }

    fn factorize(&mut self) -> i32 {
        let ret = unsafe {
            _glp_bfd_factorize(
                &mut *self.bfd,
                self.m,
                Some(Self::jth_col_callback),
                self as *mut _ as *mut std::ffi::c_void,
            )
        };
        self.valid = (ret == 0) as i32;
        ret
    }

    unsafe extern "C" fn jth_col_callback(
        info: *mut std::ffi::c_void,
        j: i32,
        ind: *mut i32,
        val: *mut f64,
    ) -> i32 {
        let lp = &mut *(info as *mut SPXLP);
        let (indices, values) = lp.jth_col(j);
        
        ptr::copy_nonoverlapping(
            indices.as_ptr(),
            ind.offset(1),
            indices.len()
        );
        
        ptr::copy_nonoverlapping(
            values.as_ptr(),
            val.offset(1),
            values.len()
        );
        
        indices.len() as i32
    }

    fn eval_beta(&self, beta: &mut [f64]) {
        let m = self.m;
        let n = self.n;
        
        beta[1..=m as usize].copy_from_slice(&self.b[1..=m as usize]);

        for j in 1..=(n - m) {
            let k = self.head[(m + j) as usize];
            let fj = if self.flag[j as usize] != 0 {
                self.u[k as usize]
            } else {
                self.l[k as usize]
            };

            if fj != 0.0 && fj != f64::MIN {
                let ptr = self.a_ptr[k as usize];
                let end = self.a_ptr[(k + 1) as usize];
                
                for p in ptr..end {
                    let idx = self.a_ind[p as usize];
                    beta[idx as usize] -= self.a_val[p as usize] * fj;
                }
            }
        }

        assert!(self.valid != 0, "lp->valid");
        unsafe {
            _glp_bfd_ftran(&mut *self.bfd, beta.as_mut_ptr());
        }
    }

    fn eval_obj(&self, beta: &[f64]) -> f64 {
        let m = self.m;
        let n = self.n;
        let mut z = self.c[0];

        for i in 1..=m {
            let k = self.head[i as usize];
            z += self.c[k as usize] * beta[i as usize];
        }

        for j in 1..=(n - m) {
            let k = self.head[(m + j) as usize];
            let fj = if self.flag[j as usize] != 0 {
                self.u[k as usize]
            } else {
                self.l[k as usize]
            };

            if fj != 0.0 && fj != f64::MIN {
                z += self.c[k as usize] * fj;
            }
        }

        z
    }

    fn eval_pi(&self, pi: &mut [f64]) {
        let m = self.m;
        
        for i in 1..=m {
            pi[i as usize] = self.c[self.head[i as usize] as usize];
        }

        unsafe {
            _glp_bfd_btran(&mut *self.bfd, pi.as_mut_ptr());
        }
    }

    fn eval_dj(&self, pi: &[f64], j: i32) -> f64 {
        let m = self.m;
        let n = self.n;
        assert!(1 <= j && j <= n - m, "1 <= j && j <= n-m");

        let k = self.head[(m + j) as usize];
        let mut dj = self.c[k as usize];

        let ptr = self.a_ptr[k as usize];
        let end = self.a_ptr[(k + 1) as usize];
        
        for p in ptr..end {
            let idx = self.a_ind[p as usize];
            dj -= self.a_val[p as usize] * pi[idx as usize];
        }

        dj
    }

    fn eval_tcol(&self, j: i32, tcol: &mut [f64]) {
        let m = self.m;
        let n = self.n;
        assert!(1 <= j && j <= n - m, "1 <= j && j <= n-m");

        tcol[1..=m as usize].fill(0.0);

        let k = self.head[(m + j) as usize];
        let ptr = self.a_ptr[k as usize];
        let end = self.a_ptr[(k + 1) as usize];
        
        for p in ptr..end {
            let idx = self.a_ind[p as usize];
            tcol[idx as usize] = -self.a_val[p as usize];
        }

        unsafe {
            _glp_bfd_ftran(&mut *self.bfd, tcol.as_mut_ptr());
        }
    }

    fn eval_rho(&self, i: i32, rho: &mut [f64]) {
        let m = self.m;
        assert!(1 <= i && i <= m, "1 <= i && i <= m");

        rho[1..=m as usize].fill(0.0);
        rho[i as usize] = 1.0;

        unsafe {
            _glp_bfd_btran(&mut *self.bfd, rho.as_mut_ptr());
        }
    }

    fn eval_rho_s(&self, i: i32, rho: &mut FVS) {
        let m = self.m;
        assert!(1 <= i && i <= m, "1 <= i && i <= m");
        assert!(rho.n == m, "rho->n == m");

        rho.clear_vec();
        rho.nnz = 1;
        rho.ind[1] = i;
        rho.vec[i as usize] = 1.0;

        unsafe {
            _glp_bfd_btran_s(&mut *self.bfd, rho);
        }
    }

    fn eval_tij(&self, rho: &[f64], j: i32) -> f64 {
        let m = self.m;
        let n = self.n;
        assert!(1 <= j && j <= n - m, "1 <= j && j <= n-m");

        let k = self.head[(m + j) as usize];
        let mut tij = 0.0;

        let ptr = self.a_ptr[k as usize];
        let end = self.a_ptr[(k + 1) as usize];
        
        for p in ptr..end {
            let idx = self.a_ind[p as usize];
            tij -= self.a_val[p as usize] * rho[idx as usize];
        }

        tij
    }

    fn eval_trow(&self, rho: &[f64], trow: &mut [f64]) {
        let n = self.n;
        let m = self.m;
        
        for j in 1..=(n - m) {
            trow[j as usize] = self.eval_tij(rho, j);
        }
    }

    fn update_beta(
        &mut self,
        beta: &mut [f64],
        p: i32,
        p_flag: i32,
        q: i32,
        tcol: &[f64],
    ) {
        let m = self.m;
        let n = self.n;
        let mut delta_q;

        if p < 0 {
            assert!(1 <= q && q <= n - m, "1 <= q && q <= n-m");
            let k = self.head[(m + q) as usize];
            assert!(
                self.l[k as usize] != f64::MIN && 
                self.u[k as usize] != f64::MAX && 
                self.l[k as usize] != self.u[k as usize],
                "l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]"
            );

            delta_q = if self.flag[q as usize] != 0 {
                self.l[k as usize] - self.u[k as usize]
            } else {
                self.u[k as usize] - self.l[k as usize]
            };
        } else {
            assert!(1 <= p && p <= m, "1 <= p && p <= m");
            assert!(1 <= q && q <= n - m, "1 <= q && q <= n-m");

            let k = self.head[p as usize];
            let delta_p = if p_flag != 0 {
                assert!(
                    self.l[k as usize] != self.u[k as usize] && 
                    self.u[k as usize] != f64::MAX,
                    "l[k] != u[k] && u[k] != +DBL_MAX"
                );
                self.u[k as usize] - beta[p as usize]
            } else if self.l[k as usize] == f64::MIN {
                assert!(self.u[k as usize] == f64::MAX, "u[k] == +DBL_MAX");
                -beta[p as usize]
            } else {
                self.l[k as usize] - beta[p as usize]
            };

            delta_q = delta_p / tcol[p as usize];

            let k = self.head[(m + q) as usize];
            if self.flag[q as usize] != 0 {
                assert!(
                    self.l[k as usize] != self.u[k as usize] && 
                    self.u[k as usize] != f64::MAX,
                    "l[k] != u[k] && u[k] != +DBL_MAX"
                );
                beta[p as usize] = self.u[k as usize] + delta_q;
            } else if self.l[k as usize] == f64::MIN {
                assert!(self.u[k as usize] == f64::MAX, "u[k] == +DBL_MAX");
                beta[p as usize] = 0.0 + delta_q;
            } else {
                beta[p as usize] = self.l[k as usize] + delta_q;
            }
        }

        for i in 1..=m {
            if i != p {
                beta[i as usize] += tcol[i as usize] * delta_q;
            }
        }
    }

    fn update_beta_s(
        &mut self,
        beta: &mut [f64],
        p: i32,
        p_flag: i32,
        q: i32,
        tcol: &FVS,
    ) {
        let m = self.m;
        let n = self.n;
        assert!(tcol.n == m, "tcol->n == m");
        let mut delta_q;

        if p < 0 {
            assert!(1 <= q && q <= n - m, "1 <= q && q <= n-m");
            let k = self.head[(m + q) as usize];
            assert!(
                self.l[k as usize] != f64::MIN && 
                self.u[k as usize] != f64::MAX && 
                self.l[k as usize] != self.u[k as usize],
                "l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]"
            );

            delta_q = if self.flag[q as usize] != 0 {
                self.l[k as usize] - self.u[k as usize]
            } else {
                self.u[k as usize] - self.l[k as usize]
            };
        } else {
            assert!(1 <= p && p <= m, "1 <= p && p <= m");
            assert!(1 <= q && q <= n - m, "1 <= q && q <= n-m");

            let k = self.head[p as usize];
            let delta_p = if p_flag != 0 {
                assert!(
                    self.l[k as usize] != self.u[k as usize] && 
                    self.u[k as usize] != f64::MAX,
                    "l[k] != u[k] && u[k] != +DBL_MAX"
                );
                self.u[k as usize] - beta[p as usize]
            } else if self.l[k as usize] == f64::MIN {
                assert!(self.u[k as usize] == f64::MAX, "u[k] == +DBL_MAX");
                -beta[p as usize]
            } else {
                self.l[k as usize] - beta[p as usize]
            };

            delta_q = delta_p / tcol.vec[p as usize];

            let k = self.head[(m + q) as usize];
            if self.flag[q as usize] != 0 {
                assert!(
                    self.l[k as usize] != self.u[k as usize] && 
                    self.u[k as usize] != f64::MAX,
                    "l[k] != u[k] && u[k] != +DBL_MAX"
                );
                beta[p as usize] = self.u[k as usize] + delta_q;
            } else if self.l[k as usize] == f64::MIN {
                assert!(self.u[k as usize] == f64::MAX, "u[k] == +DBL_MAX");
                beta[p as usize] = 0.0 + delta_q;
            } else {
                beta[p as usize] = self.l[k as usize] + delta_q;
            }
        }

        for k in 1..=tcol.nnz {
            let i = tcol.ind[k as usize];
            if i != p {
                beta[i as usize] += tcol.vec[i as usize] * delta_q;
            }
        }
    }

    fn update_d(
        &mut self,
        d: &mut [f64],
        p: i32,
        q: i32,
        trow: &[f64],
        tcol: &[f64],
    ) -> f64 {
        let m = self.m;
        let n = self.n;
        assert!(1 <= p && p <= m, "1 <= p && p <= m");
        assert!(1 <= q && q <= n, "1 <= q && q <= n");

        let k = self.head[(m + q) as usize];
        let mut dq = self.c[k as usize];

        for i in 1..=m {
            dq += tcol[i as usize] * self.c[self.head[i as usize] as usize];
        }

        let e = (dq - d[q as usize]).abs() / (1.0 + dq.abs());
        dq /= tcol[p as usize];
        d[q as usize] = dq;

        for j in 1..=(n - m) {
            if j != q {
                d[j as usize] -= trow[j as usize] * dq;
            }
        }

        e
    }

    fn update_d_s(
        &mut self,
        d: &mut [f64],
        p: i32,
        q: i32,
        trow: &FVS,
        tcol: &FVS,
    ) -> f64 {
        let m = self.m;
        let n = self.n;
        assert!(1 <= p && p <= m, "1 <= p && p <= m");
        assert!(1 <= q && q <= n, "1 <= q && q <= n");
        assert!(trow.n == n - m, "trow->n == n-m");
        assert!(tcol.n == m, "tcol->n == m");

        let k = self.head[(m + q) as usize];
        let mut dq = self.c[k as usize];

        for k in 1..=tcol.nnz {
            let i = tcol.ind[k as usize];
            dq += tcol.vec[i as usize] * self.c[self.head[i as usize] as usize];
        }

        let e = (dq - d[q as usize]).abs() / (1.0 + dq.abs());
        dq /= tcol.vec[p as usize];
        d[q as usize] = dq;

        for k in 1..=trow.nnz {
            let j = trow.ind[k as usize];
            if j != q {
                d[j as usize] -= trow.vec[j as usize] * dq;
            }
        }

        e
    }

    fn change_basis(&mut self, p: i32, p_flag: i32, q: i32) {
        let m = self.m;
        let n = self.n;

        if p < 0 {
            assert!(1 <= q && q <= n - m, "1 <= q && q <= n-m");
            let k = self.head[(m + q) as usize];
            assert!(
                self.l[k as usize] != f64::