use std::mem;
use std::ptr;
use std::slice;

// Sparse Vector Area (SVA) structure
struct SVA {
    size: usize,
    m_ptr: usize,
    r_ptr: usize,
    ptr: Vec<usize>,
    len: Vec<usize>,
    cap: Vec<usize>,
    ind: Vec<usize>,
    val: Vec<f64>,
}

impl SVA {
    fn new(size: usize) -> Self {
        SVA {
            size,
            m_ptr: 1,
            r_ptr: size + 1,
            ptr: vec![0; size + 1],
            len: vec![0; size + 1],
            cap: vec![0; size + 1],
            ind: vec![0; size + 1],
            val: vec![0.0; size + 1],
        }
    }

    fn more_space(&mut self, need: usize) {
        let free = self.r_ptr - self.m_ptr;
        if free < need {
            let new_size = self.size + need - free;
            self.ptr.resize(new_size + 1, 0);
            self.len.resize(new_size + 1, 0);
            self.cap.resize(new_size + 1, 0);
            self.ind.resize(new_size + 1, 0);
            self.val.resize(new_size + 1, 0.0);
            self.r_ptr = new_size + 1;
            self.size = new_size;
        }
    }

    fn enlarge_cap(&mut self, k: usize, new_cap: usize, keep: usize) {
        if self.cap[k] >= new_cap {
            return;
        }
        self.more_space(new_cap);
        unsafe {
            let old_ptr = self.ptr[k];
            let new_ptr = self.m_ptr;
            ptr::copy(
                self.ind[old_ptr..old_ptr + keep].as_ptr(),
                self.ind[new_ptr..].as_mut_ptr(),
                keep,
            );
            ptr::copy(
                self.val[old_ptr..old_ptr + keep].as_ptr(),
                self.val[new_ptr..].as_mut_ptr(),
                keep,
            );
            self.ptr[k] = new_ptr;
            self.cap[k] = new_cap;
            self.m_ptr += new_cap;
        }
    }

    fn reserve_cap(&mut self, k: usize, new_cap: usize) {
        if self.cap[k] >= new_cap {
            return;
        }
        self.more_space(new_cap);
        self.ptr[k] = self.m_ptr;
        self.cap[k] = new_cap;
        self.m_ptr += new_cap;
    }
}

// Sparse LU-factorization structure
struct LUF {
    n: usize,
    sva: SVA,
    fr_ref: usize,
    fc_ref: usize,
    vr_ref: usize,
    vc_ref: usize,
    vr_piv: Vec<f64>,
    pp_ind: Vec<usize>,
    pp_inv: Vec<usize>,
    qq_ind: Vec<usize>,
    qq_inv: Vec<usize>,
}

impl LUF {
    fn new(n: usize) -> Self {
        LUF {
            n,
            sva: SVA::new(n * n),
            fr_ref: 1,
            fc_ref: n + 1,
            vr_ref: 2 * n + 1,
            vc_ref: 3 * n + 1,
            vr_piv: vec![0.0; n + 1],
            pp_ind: vec![0; n + 1],
            pp_inv: vec![0; n + 1],
            qq_ind: vec![0; n + 1],
            qq_inv: vec![0; n + 1],
        }
    }

    fn store_v_cols(
        &mut self,
        col: impl Fn(usize, &mut [usize], &mut [f64]) -> usize,
        ind: &mut [usize],
        val: &mut [f64],
    ) -> usize {
        let n = self.n;
        let mut nnz = 0;
        for j in 1..=n {
            let len = col(j, ind, val);
            assert!(len <= n);
            if self.sva.cap[self.vc_ref - 1 + j] < len {
                if self.sva.r_ptr - self.sva.m_ptr < len {
                    self.sva.more_space(len);
                }
                self.sva.enlarge_cap(self.vc_ref - 1 + j, len, 0);
            }
            let ptr = self.sva.ptr[self.vc_ref - 1 + j];
            self.sva.ind[ptr..ptr + len].copy_from_slice(&ind[1..=len]);
            self.sva.val[ptr..ptr + len].copy_from_slice(&val[1..=len]);
            self.sva.len[self.vc_ref - 1 + j] = len;
            nnz += len;
        }
        nnz
    }

    fn check_all(&self, k: usize) {
        let n = self.n;
        assert!(1 <= k && k <= n + 1);

        // Check permutation matrix P
        for i in 1..=n {
            let ii = self.pp_ind[i];
            assert!(1 <= ii && ii <= n);
            assert!(self.pp_inv[ii] == i);
        }

        // Check permutation matrix Q
        for j in 1..=n {
            let jj = self.qq_inv[j];
            assert!(1 <= jj && jj <= n);
            assert!(self.qq_ind[jj] == j);
        }

        // Check row-wise representation of matrix F
        for i in 1..=n {
            assert!(self.sva.len[self.fr_ref - 1 + i] == 0);
        }

        // Check column-wise representation of matrix F
        for j in 1..=n {
            let jj = self.pp_ind[j];
            if jj < k {
                let ptr = self.sva.ptr[self.fc_ref - 1 + j];
                let end = ptr + self.sva.len[self.fc_ref - 1 + j];
                for p in ptr..end {
                    let i = self.sva.ind[p];
                    assert!(1 <= i && i <= n);
                    let ii = self.pp_ind[i];
                    assert!(ii > jj);
                    assert!(self.sva.val[p] != 0.0);
                }
            } else {
                assert!(self.sva.len[self.fc_ref - 1 + j] == 0);
            }
        }

        // Check row-wise representation of matrix V
        for i in 1..=n {
            let ii = self.pp_ind[i];
            let ptr = self.sva.ptr[self.vr_ref - 1 + i];
            let end = ptr + self.sva.len[self.vr_ref - 1 + i];
            for p in ptr..end {
                let j = self.sva.ind[p];
                assert!(1 <= j && j <= n);
                let jj = self.qq_inv[j];
                if ii < k {
                    assert!(jj > ii);
                } else {
                    assert!(jj >= k);
                    let j_ptr = self.sva.ptr[self.vc_ref - 1 + j];
                    let j_end = j_ptr + self.sva.len[self.vc_ref - 1 + j];
                    let mut found = false;
                    for q in j_ptr..j_end {
                        if self.sva.ind[q] == i {
                            found = true;
                            break;
                        }
                    }
                    assert!(found);
                }
                assert!(self.sva.val[p] != 0.0);
            }
        }

        // Check column-wise representation of matrix V
        for j in 1..=n {
            let jj = self.qq_inv[j];
            if jj < k {
                assert!(self.sva.len[self.vc_ref - 1 + j] == 0);
            } else {
                let ptr = self.sva.ptr[self.vc_ref - 1 + j];
                let end = ptr + self.sva.len[self.vc_ref - 1 + j];
                for p in ptr..end {
                    let i = self.sva.ind[p];
                    let ii = self.pp_ind[i];
                    assert!(ii >= k);
                    let i_ptr = self.sva.ptr[self.vr_ref - 1 + i];
                    let i_end = i_ptr + self.sva.len[self.vr_ref - 1 + i];
                    let mut found = false;
                    for q in i_ptr..i_end {
                        if self.sva.ind[q] == j {
                            found = true;
                            break;
                        }
                    }
                    assert!(found);
                }
            }
        }
    }

    fn build_v_rows(&mut self, len: &mut [usize]) {
        let n = self.n;
        let mut nnz = 0;

        // Calculate number of non-zeros in each row and total
        for i in 1..=n {
            len[i] = 0;
        }
        for j in 1..=n {
            nnz += self.sva.len[self.vc_ref - 1 + j];
            let ptr = self.sva.ptr[self.vc_ref - 1 + j];
            let end = ptr + self.sva.len[self.vc_ref - 1 + j];
            for p in ptr..end {
                len[self.sva.ind[p]] += 1;
            }
        }

        // Ensure enough space
        if self.sva.r_ptr - self.sva.m_ptr < nnz {
            self.sva.more_space(nnz);
        }

        // Reserve locations for rows
        for i in 1..=n {
            if len[i] > 0 {
                self.sva.enlarge_cap(self.vr_ref - 1 + i, len[i], 0);
            }
            self.sva.len[self.vr_ref - 1 + i] = len[i];
        }

        // Build rows from columns
        for j in 1..=n {
            let ptr = self.sva.ptr[self.vc_ref - 1 + j];
            let end = ptr + self.sva.len[self.vc_ref - 1 + j];
            for p in ptr..end {
                let i = self.sva.ind[p];
                let new_ptr = self.sva.ptr[self.vr_ref - 1 + i] + (len[i] - 1);
                len[i] -= 1;
                self.sva.ind[new_ptr] = j;
                self.sva.val[new_ptr] = self.sva.val[p];
            }
        }
    }

    fn build_f_rows(&mut self, len: &mut [usize]) {
        let n = self.n;
        let mut nnz = 0;

        // Calculate number of non-zeros in each row and total
        for i in 1..=n {
            len[i] = 0;
        }
        for j in 1..=n {
            nnz += self.sva.len[self.fc_ref - 1 + j];
            let ptr = self.sva.ptr[self.fc_ref - 1 + j];
            let end = ptr + self.sva.len[self.fc_ref - 1 + j];
            for p in ptr..end {
                len[self.sva.ind[p]] += 1;
            }
        }

        // Ensure enough space
        if self.sva.r_ptr - self.sva.m_ptr < nnz {
            self.sva.more_space(nnz);
        }

        // Reserve locations for rows
        for i in 1..=n {
            if len[i] > 0 {
                self.sva.reserve_cap(self.fr_ref - 1 + i, len[i]);
            }
            self.sva.len[self.fr_ref - 1 + i] = len[i];
        }

        // Build rows from columns
        for j in 1..=n {
            let ptr = self.sva.ptr[self.fc_ref - 1 + j];
            let end = ptr + self.sva.len[self.fc_ref - 1 + j];
            for p in ptr..end {
                let i = self.sva.ind[p];
                let new_ptr = self.sva.ptr[self.fr_ref - 1 + i] + (len[i] - 1);
                len[i] -= 1;
                self.sva.ind[new_ptr] = j;
                self.sva.val[new_ptr] = self.sva.val[p];
            }
        }
    }

    fn build_v_cols(&mut self, updat: bool, len: &mut [usize]) {
        let n = self.n;
        let mut nnz = 0;

        // Calculate number of non-zeros in each column and total
        for j in 1..=n {
            len[j] = 0;
        }
        for i in 1..=n {
            nnz += self.sva.len[self.vr_ref - 1 + i];
            let ptr = self.sva.ptr[self.vr_ref - 1 + i];
            let end = ptr + self.sva.len[self.vr_ref - 1 + i];
            for p in ptr..end {
                len[self.sva.ind[p]] += 1;
            }
        }

        // Ensure enough space
        if self.sva.r_ptr - self.sva.m_ptr < nnz {
            self.sva.more_space(nnz);
        }

        // Reserve locations for columns
        for j in 1..=n {
            if len[j] > 0 {
                if updat {
                    self.sva.enlarge_cap(self.vc_ref - 1 + j, len[j], 0);
                } else {
                    self.sva.reserve_cap(self.vc_ref - 1 + j, len[j]);
                }
            }
            self.sva.len[self.vc_ref - 1 + j] = len[j];
        }

        // Build columns from rows
        for i in 1..=n {
            let ptr = self.sva.ptr[self.vr_ref - 1 + i];
            let end = ptr + self.sva.len[self.vr_ref - 1 + i];
            for p in ptr..end {
                let j = self.sva.ind[p];
                let new_ptr = self.sva.ptr[self.vc_ref - 1 + j] + (len[j] - 1);
                len[j] -= 1;
                self.sva.ind[new_ptr] = i;
                self.sva.val[new_ptr] = self.sva.val[p];
            }
        }
    }

    fn check_f_rc(&self) {
        let n = self.n;

        // Walk through rows of matrix F
        for i in 1..=n {
            let ptr = self.sva.ptr[self.fr_ref - 1 + i];
            let end = ptr + self.sva.len[self.fr_ref - 1 + i];
            for p in ptr..end {
                let j = self.sva.ind[p];
                // Find element f[i,j] in j-th column
                let j_ptr = self.sva.ptr[self.fc_ref - 1 + j];
                let j_end = j_ptr + self.sva.len[self.fc_ref - 1 + j];
                let mut found = false;
                for q in j_ptr..j_end {
                    if self.sva.ind[q] == i {
                        assert!(self.sva.val[p] == self.sva.val[q]);
                        // Mark element f[i,j]
                        self.sva.ind[q] = i.wrapping_neg();
                        found = true;
                        break;
                    }
                }
                assert!(found);
            }
        }

        // Walk through columns of matrix F and check marks
        for j in 1..=n {
            let ptr = self.sva.ptr[self.fc_ref - 1 + j];
            let end = ptr + self.sva.len[self.fc_ref - 1 + j];
            for p in ptr..end {
                let i = self.sva.ind[p];
                assert!(i < 0);
                // Unmark element f[i,j]
                self.sva.ind[p] = i.wrapping_neg();
            }
        }
    }

    fn check_v_rc(&self) {
        let n = self.n;

        // Walk through rows of matrix V
        for i in 1..=n {
            let ptr = self.sva.ptr[self.vr_ref - 1 + i];
            let end = ptr + self.sva.len[self.vr_ref - 1 + i];
            for p in ptr..end {
                let j = self.sva.ind[p];
                // Find element v[i,j] in j-th column
                let j_ptr = self.sva.ptr[self.vc_ref - 1 + j];
                let j_end = j_ptr + self.sva.len[self.vc_ref - 1 + j];
                let mut found = false;
                for q in j_ptr..j_end {
                    if self.sva.ind[q] == i {
                        assert!(self.sva.val[p] == self.sva.val[q]);
                        // Mark element v[i,j]
                        self.sva.ind[q] = i.wrapping_neg();
                        found = true;
                        break;
                    }
                }
                assert!(found);
            }
        }

        // Walk through columns of matrix V and check marks
        for j in 1..=n {
            let ptr = self.sva.ptr[self.vc_ref - 1 + j];
            let end = ptr + self.sva.len[self.vc_ref - 1 + j];
            for p in ptr..end {
                let i = self.sva.ind[p];
                assert!(i < 0);
                // Unmark element v[i,j]
                self.sva.ind[p] = i.wrapping_neg();
            }
        }
    }

    fn f_solve(&self, x: &mut [f64]) {
        let n = self.n;
        for k in 1..=n {
            let j = self.pp_inv[k];
            let x_j = x[j];
            if x_j != 0.0 {
                let ptr = self.sva.ptr[self.fc_ref - 1 + j];
                let end = ptr + self.sva.len[self.fc_ref - 1 + j];
                for p in ptr..end {
                    x[self.sva.ind[p]] -= self.sva.val[p] * x_j;
                }
            }
        }
    }

    fn ft_solve(&self, x: &mut [f64]) {
        let n = self.n;
        for k in (1..=n).rev() {
            let i = self.pp_inv[k];
            let