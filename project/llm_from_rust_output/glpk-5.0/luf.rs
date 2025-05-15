use std::ptr;

#[derive(Debug, Clone)]
pub struct SVA {
    pub n_max: i32,
    pub n: i32,
    pub ptr: Vec<i32>,
    pub len: Vec<i32>,
    pub cap: Vec<i32>,
    pub size: i32,
    pub m_ptr: i32,
    pub r_ptr: i32,
    pub head: i32,
    pub tail: i32,
    pub prev: Vec<i32>,
    pub next: Vec<i32>,
    pub ind: Vec<i32>,
    pub val: Vec<f64>,
    pub talky: i32,
}

#[derive(Debug, Clone)]
pub struct LUF {
    pub n: i32,
    pub sva: Box<SVA>,
    pub fr_ref: i32,
    pub fc_ref: i32,
    pub vr_ref: i32,
    pub vr_piv: Vec<f64>,
    pub vc_ref: i32,
    pub pp_ind: Vec<i32>,
    pub pp_inv: Vec<i32>,
    pub qq_ind: Vec<i32>,
    pub qq_inv: Vec<i32>,
}

impl LUF {
    pub fn store_v_cols(
        &mut self,
        col: impl Fn(&mut dyn std::any::Any, i32, &mut [i32], &mut [f64]) -> i32,
        info: &mut dyn std::any::Any,
        ind: &mut [i32],
        val: &mut [f64],
    ) -> i32 {
        let n = self.n;
        let mut nnz = 0;

        for j in 1..=n {
            let len = col(info, j, ind, val);
            assert!(0 <= len && len <= n, "0 <= len && len <= n");

            if self.sva.cap[j as usize] < len {
                if self.sva.r_ptr - self.sva.m_ptr < len {
                    self.sva.more_space(len);
                }
                self.sva.enlarge_cap(self.vc_ref - 1 + j, len, 0);
            }

            let ptr = self.sva.ptr[(self.vc_ref - 1 + j) as usize];
            let sv_ind = &mut self.sva.ind;
            let sv_val = &mut self.sva.val;

            sv_ind[ptr as usize..(ptr + len) as usize].copy_from_slice(&ind[1..(len + 1) as usize]);
            sv_val[ptr as usize..(ptr + len) as usize].copy_from_slice(&val[1..(len + 1) as usize]);

            self.sva.len[(self.vc_ref - 1 + j) as usize] = len;
            nnz += len;
        }

        nnz
    }

    pub fn check_all(&self, k: i32) {
        let n = self.n;
        assert!(n > 0, "n > 0");
        assert!(1 <= k && k <= n + 1, "1 <= k && k <= n+1");

        for i in 1..=n {
            let ii = self.pp_ind[i as usize];
            assert!(1 <= ii && ii <= n, "1 <= ii && ii <= n");
            assert_eq!(self.pp_inv[ii as usize], i, "pp_inv[ii] == i");
        }

        for j in 1..=n {
            let jj = self.qq_inv[j as usize];
            assert!(1 <= jj && jj <= n, "1 <= jj && jj <= n");
            assert_eq!(self.qq_ind[jj as usize], j, "qq_ind[jj] == j");
        }

        for i in 1..=n {
            assert_eq!(self.sva.len[(self.fr_ref - 1 + i) as usize], 0, "fr_len[i] == 0");
        }

        for j in 1..=n {
            let jj = self.pp_ind[j as usize];
            if jj < k {
                let mut j_ptr = self.sva.ptr[(self.fc_ref - 1 + j) as usize];
                let j_end = j_ptr + self.sva.len[(self.fc_ref - 1 + j) as usize];

                while j_ptr < j_end {
                    let i = self.sva.ind[j_ptr as usize];
                    assert!(1 <= i && i <= n, "1 <= i && i <= n");
                    let ii = self.pp_ind[i as usize];
                    assert!(ii > jj, "ii > jj");
                    assert_ne!(self.sva.val[j_ptr as usize], 0.0, "sv_val[j_ptr] != 0.0");
                    j_ptr += 1;
                }
            } else {
                assert_eq!(self.sva.len[(self.fc_ref - 1 + j) as usize], 0, "fc_len[j] == 0");
            }
        }

        for i in 1..=n {
            let ii = self.pp_ind[i as usize];
            let mut i_ptr = self.sva.ptr[(self.vr_ref - 1 + i) as usize];
            let i_end = i_ptr + self.sva.len[(self.vr_ref - 1 + i) as usize];

            while i_ptr < i_end {
                let j = self.sva.ind[i_ptr as usize];
                assert!(1 <= j && j <= n, "1 <= j && j <= n");
                let jj = self.qq_inv[j as usize];

                if ii < k {
                    assert!(jj > ii, "jj > ii");
                } else {
                    assert!(jj >= k, "jj >= k");
                    let mut j_ptr = self.sva.ptr[(self.vc_ref - 1 + j) as usize];
                    let j_end = j_ptr + self.sva.len[(self.vc_ref - 1 + j) as usize];

                    while self.sva.ind[j_ptr as usize] != i {
                        j_ptr += 1;
                        assert!(j_ptr < j_end, "j_ptr < j_end");
                    }
                }

                assert_ne!(self.sva.val[i_ptr as usize], 0.0, "sv_val[i_ptr] != 0.0");
                i_ptr += 1;
            }
        }

        for j in 1..=n {
            let jj = self.qq_inv[j as usize];
            if jj < k {
                assert_eq!(self.sva.len[(self.vc_ref - 1 + j) as usize], 0, "vc_len[j] == 0");
            } else {
                let mut j_ptr = self.sva.ptr[(self.vc_ref - 1 + j) as usize];
                let j_end = j_ptr + self.sva.len[(self.vc_ref - 1 + j) as usize];

                while j_ptr < j_end {
                    let i = self.sva.ind[j_ptr as usize];
                    let ii = self.pp_ind[i as usize];
                    assert!(ii >= k, "ii >= k");

                    let mut i_ptr = self.sva.ptr[(self.vr_ref - 1 + i) as usize];
                    let i_end = i_ptr + self.sva.len[(self.vr_ref - 1 + i) as usize];

                    while self.sva.ind[i_ptr as usize] != j {
                        i_ptr += 1;
                        assert!(i_ptr < i_end, "i_ptr < i_end");
                    }

                    j_ptr += 1;
                }
            }
        }
    }

    pub fn build_v_rows(&mut self, len: &mut [i32]) {
        let n = self.n;
        let mut nnz = 0;

        for i in 1..=n {
            len[i as usize] = 0;
        }

        for j in 1..=n {
            nnz += self.sva.len[(self.vc_ref - 1 + j) as usize];
            let ptr = self.sva.ptr[(self.vc_ref - 1 + j) as usize];
            let end = ptr + self.sva.len[(self.vc_ref - 1 + j) as usize];

            for p in ptr..end {
                len[self.sva.ind[p as usize] as usize] += 1;
            }
        }

        if self.sva.r_ptr - self.sva.m_ptr < nnz {
            self.sva.more_space(nnz);
        }

        for i in 1..=n {
            if len[i as usize] > 0 {
                self.sva.enlarge_cap(self.vr_ref - 1 + i, len[i as usize], 0);
            }
            self.sva.len[(self.vr_ref - 1 + i) as usize] = len[i as usize];
        }

        for j in 1..=n {
            let ptr = self.sva.ptr[(self.vc_ref - 1 + j) as usize];
            let end = ptr + self.sva.len[(self.vc_ref - 1 + j) as usize];

            for p in ptr..end {
                let i = self.sva.ind[p as usize];
                len[i as usize] -= 1;
                let ptr1 = self.sva.ptr[(self.vr_ref - 1 + i) as usize] + len[i as usize];
                self.sva.ind[ptr1 as usize] = j;
                self.sva.val[ptr1 as usize] = self.sva.val[p as usize];
            }
        }
    }

    pub fn build_f_rows(&mut self, len: &mut [i32]) {
        let n = self.n;
        let mut nnz = 0;

        for i in 1..=n {
            len[i as usize] = 0;
        }

        for j in 1..=n {
            nnz += self.sva.len[(self.fc_ref - 1 + j) as usize];
            let ptr = self.sva.ptr[(self.fc_ref - 1 + j) as usize];
            let end = ptr + self.sva.len[(self.fc_ref - 1 + j) as usize];

            for p in ptr..end {
                len[self.sva.ind[p as usize] as usize] += 1;
            }
        }

        if self.sva.r_ptr - self.sva.m_ptr < nnz {
            self.sva.more_space(nnz);
        }

        for i in 1..=n {
            if len[i as usize] > 0 {
                self.sva.reserve_cap(self.fr_ref - 1 + i, len[i as usize]);
            }
            self.sva.len[(self.fr_ref - 1 + i) as usize] = len[i as usize];
        }

        for j in 1..=n {
            let ptr = self.sva.ptr[(self.fc_ref - 1 + j) as usize];
            let end = ptr + self.sva.len[(self.fc_ref - 1 + j) as usize];

            for p in ptr..end {
                let i = self.sva.ind[p as usize];
                len[i as usize] -= 1;
                let ptr1 = self.sva.ptr[(self.fr_ref - 1 + i) as usize] + len[i as usize];
                self.sva.ind[ptr1 as usize] = j;
                self.sva.val[ptr1 as usize] = self.sva.val[p as usize];
            }
        }
    }

    pub fn build_v_cols(&mut self, updat: bool, len: &mut [i32]) {
        let n = self.n;
        let mut nnz = 0;

        for j in 1..=n {
            len[j as usize] = 0;
        }

        for i in 1..=n {
            nnz += self.sva.len[(self.vr_ref - 1 + i) as usize];
            let ptr = self.sva.ptr[(self.vr_ref - 1 + i) as usize];
            let end = ptr + self.sva.len[(self.vr_ref - 1 + i) as usize];

            for p in ptr..end {
                len[self.sva.ind[p as usize] as usize] += 1;
            }
        }

        if self.sva.r_ptr - self.sva.m_ptr < nnz {
            self.sva.more_space(nnz);
        }

        for j in 1..=n {
            if len[j as usize] > 0 {
                if updat {
                    self.sva.enlarge_cap(self.vc_ref - 1 + j, len[j as usize], 0);
                } else {
                    self.sva.reserve_cap(self.vc_ref - 1 + j, len[j as usize]);
                }
            }
            self.sva.len[(self.vc_ref - 1 + j) as usize] = len[j as usize];
        }

        for i in 1..=n {
            let ptr = self.sva.ptr[(self.vr_ref - 1 + i) as usize];
            let end = ptr + self.sva.len[(self.vr_ref - 1 + i) as usize];

            for p in ptr..end {
                let j = self.sva.ind[p as usize];
                len[j as usize] -= 1;
                let ptr1 = self.sva.ptr[(self.vc_ref - 1 + j) as usize] + len[j as usize];
                self.sva.ind[ptr1 as usize] = i;
                self.sva.val[ptr1 as usize] = self.sva.val[p as usize];
            }
        }
    }

    pub fn check_f_rc(&mut self) {
        let n = self.n;

        for i in 1..=n {
            let mut i_ptr = self.sva.ptr[(self.fr_ref - 1 + i) as usize];
            let i_end = i_ptr + self.sva.len[(self.fr_ref - 1 + i) as usize];

            while i_ptr < i_end {
                let j = self.sva.ind[i_ptr as usize];
                let mut j_ptr = self.sva.ptr[(self.fc_ref - 1 + j) as usize];
                let j_end = j_ptr + self.sva.len[(self.fc_ref - 1 + j) as usize];

                while self.sva.ind[j_ptr as usize] != i {
                    j_ptr += 1;
                    assert!(j_ptr < j_end, "j_ptr < j_end");
                }

                assert_eq!(
                    self.sva.val[i_ptr as usize], 
                    self.sva.val[j_ptr as usize],
                    "sv_val[i_ptr] == sv_val[j_ptr]"
                );
                self.sva.ind[j_ptr as usize] = -i;
                i_ptr += 1;
            }
        }

        for j in 1..=n {
            let mut j_ptr = self.sva.ptr[(self.fc_ref - 1 + j) as usize];
            let j_end = j_ptr + self.sva.len[(self.fc_ref - 1 + j) as usize];

            while j_ptr < j_end {
                let i = self.sva.ind[j_ptr as usize];
                assert!(i < 0, "(i = sv_ind[j_ptr]) < 0");
                self.sva.ind[j_ptr as usize] = -i;
                j_ptr += 1;
            }
        }
    }

    pub fn check_v_rc(&mut self) {
        let n = self.n;

        for i in 1..=n {
            let mut i_ptr = self.sva.ptr[(self.vr_ref - 1 + i) as usize];
            let i_end = i_ptr + self.sva.len[(self.vr_ref - 1 + i) as usize];

            while i_ptr < i_end {
                let j = self.sva.ind[i_ptr as usize];
                let mut j_ptr = self.sva.ptr[(self.vc_ref - 1 + j) as usize];
                let j_end = j_ptr + self.sva.len[(self.vc_ref - 1 + j) as usize];

                while self.sva.ind[j_ptr as usize] != i {
                    j_ptr += 1;
                    assert!(j_ptr < j_end, "j_ptr < j_end");
                }

                assert_eq!(
                    self.sva.val[i_ptr as usize],
                    self.sva.val[j_ptr as usize],
                    "sv_val[i_ptr] == sv_val[j_ptr]"
                );
                self.sva.ind[j_ptr as usize] = -i;
                i_ptr += 1;
            }
        }

        for j in 1..=n {
            let mut j_ptr = self.sva.ptr[(self.vc_ref - 1 + j) as usize];
            let j_end = j_ptr + self.sva.len[(self.vc_ref - 1 + j) as usize];

            while j_ptr < j_end {
                let i = self.sva.ind[j_ptr as usize];
                assert!(i < 0, "(i = sv_ind[j_ptr]) < 0");
                self.sva.ind[j_ptr as usize] = -i;
                j_ptr += 1;
            }
        }
    }

    pub fn f_solve(&self, x: &mut [f64]) {
        let n = self.n;

        for k in 1..=n {
            let j = self.pp_inv[k as usize];
            let x_j = x[j as usize];
            if x_j != 0.0 {
                let ptr = self.sva.ptr[(self.fc_ref - 1 + j) as usize];
                let end = ptr + self.sva.len[(self.fc_ref - 1 + j) as usize];

                for p in ptr..end {
                    x[self.sva.ind[p as usize] as usize] -= self.sva.val[p as usize] * x_j;
                }
            }
        }
    }

    pub fn ft_solve(&self, x: &mut [f64]) {
        let n = self.n;

        for k in (1..=n).rev() {
            let i = self.pp_inv[k as usize];
            let x_i = x[i as usize];
            if x_i != 0.0 {
                let ptr = self.sva.ptr[(self.fr_ref - 1 + i) as usize];
                let end = ptr + self.sva.len[(self.fr_ref - 1 + i) as usize];

                for p in ptr..end {
                    x[self.sva.ind[p as usize] as usize] -= self.sva.val[p as usize] * x_i;
                }
            }
        }
    }

    pub fn v_solve(&self, b: &mut [f64], x: &mut [f64]) {
        let n = self.n;

        for k in (1..=n).