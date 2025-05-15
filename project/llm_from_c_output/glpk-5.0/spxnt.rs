/* spxnt.rs */

use std::ptr;
use std::mem;
use std::f64;

#[derive(Debug)]
pub struct SPXNT {
    ptr: Vec<i32>,    // int ptr[1+m]
    len: Vec<i32>,    // int len[1+m]
    ind: Vec<i32>,    // int ind[1+nnz]
    val: Vec<f64>,    // double val[1+nnz]
}

impl SPXNT {
    pub fn new(m: usize, nnz: usize) -> Self {
        SPXNT {
            ptr: vec![0; 1 + m],
            len: vec![0; 1 + m],
            ind: vec![0; 1 + nnz],
            val: vec![0.0; 1 + nnz],
        }
    }
}

pub fn spx_alloc_nt(lp: &SPXLP, nt: &mut SPXNT) {
    let m = lp.m;
    let nnz = lp.nnz;
    *nt = SPXNT::new(m, nnz);
}

pub fn spx_init_nt(lp: &SPXLP, nt: &mut SPXNT) {
    let m = lp.m;
    let n = lp.n;
    let nnz = lp.nnz;
    let a_ptr = &lp.a_ptr;
    let a_ind = &lp.a_ind;
    
    // Calculate NT_len[i] = number of non-zeros in i-th row of A
    for k in 1..=n {
        let ptr = a_ptr[k] as usize;
        let end = a_ptr[k + 1] as usize;
        for i in ptr..end {
            nt.len[a_ind[i] as usize] += 1;
        }
    }
    
    // Initialize row pointers NT_ptr[i]
    nt.ptr[1] = 1;
    for i in 2..=m {
        nt.ptr[i] = nt.ptr[i - 1] + nt.len[i - 1];
    }
    assert!(nt.ptr[m] + nt.len[m] == (nnz + 1) as i32);
}

pub fn spx_nt_add_col(lp: &SPXLP, nt: &mut SPXNT, j: i32, k: i32) {
    let m = lp.m;
    let n = lp.n;
    let nnz = lp.nnz;
    let a_ptr = &lp.a_ptr;
    let a_ind = &lp.a_ind;
    let a_val = &lp.a_val;
    
    assert!(1 <= j && j <= (n - m) as i32);
    assert!(1 <= k && k <= n as i32);
    
    let ptr = a_ptr[k as usize] as usize;
    let end = a_ptr[k as usize + 1] as usize;
    
    for i in ptr..end {
        let row = a_ind[i] as usize;
        let pos = (nt.ptr[row] + nt.len[row]) as usize;
        
        if row < m {
            assert!(pos < nt.ptr[row + 1] as usize);
        } else {
            assert!(pos <= nnz);
        }
        
        nt.ind[pos] = j;
        nt.val[pos] = a_val[i];
        nt.len[row] += 1;
    }
}

pub fn spx_build_nt(lp: &SPXLP, nt: &mut SPXNT) {
    let m = lp.m;
    let n = lp.n;
    let head = &lp.head;
    
    // Reset N to 0
    for i in 1..=m {
        nt.len[i] = 0;
    }
    
    // Add non-basic columns N[j] = A[k]
    for j in 1..=(n - m) {
        let k = head[m + j]; // x[k] = xN[j]
        spx_nt_add_col(lp, nt, j as i32, k as i32);
    }
}

pub fn spx_nt_del_col(lp: &SPXLP, nt: &mut SPXNT, j: i32, k: i32) {
    let m = lp.m;
    let n = lp.n;
    let a_ptr = &lp.a_ptr;
    let a_ind = &lp.a_ind;
    
    assert!(1 <= j && j <= (n - m) as i32);
    assert!(1 <= k && k <= n as i32);
    
    let ptr = a_ptr[k as usize] as usize;
    let end = a_ptr[k as usize + 1] as usize;
    
    for i in ptr..end {
        let row = a_ind[i] as usize;
        let ptr1 = nt.ptr[row] as usize;
        let end1 = ptr1 + nt.len[row] as usize;
        
        // Find element N[i,j] = A[i,k] in i-th row
        let mut pos = ptr1;
        while pos < end1 && nt.ind[pos] != j {
            pos += 1;
        }
        assert!(pos < end1);
        
        // Remove it from i-th row element list
        nt.len[row] -= 1;
        nt.ind[pos] = nt.ind[end1 - 1];
        nt.val[pos] = nt.val[end1 - 1];
    }
}

pub fn spx_update_nt(lp: &SPXLP, nt: &mut SPXNT, p: i32, q: i32) {
    let m = lp.m;
    let n = lp.n;
    let head = &lp.head;
    
    assert!(1 <= p && p <= m as i32);
    assert!(1 <= q && q <= (n - m) as i32);
    
    // Remove old column N[q] corresponding to variable xN[q]
    spx_nt_del_col(lp, nt, q, head[m + q as usize] as i32);
    
    // Add new column N[q] corresponding to variable xB[p]
    spx_nt_add_col(lp, nt, q, head[p as usize] as i32);
}

pub fn spx_nt_prod(
    lp: &SPXLP,
    nt: &SPXNT,
    y: &mut [f64],
    ign: bool,
    s: f64,
    x: &[f64],
) {
    let m = lp.m;
    let n = lp.n;
    
    if ign {
        // y := 0
        for j in 1..=(n - m) {
            y[j] = 0.0;
        }
    }
    
    for i in 1..=m {
        if x[i] != 0.0 {
            // y := y + s * (i-th row of N) * x[i]
            let t = s * x[i];
            let ptr = nt.ptr[i] as usize;
            let end = ptr + nt.len[i] as usize;
            
            for pos in ptr..end {
                y[nt.ind[pos] as usize] += nt.val[pos] * t;
            }
        }
    }
}

pub fn spx_nt_prod_s(
    lp: &SPXLP,
    nt: &SPXNT,
    y: &mut FVS,
    ign: bool,
    s: f64,
    x: &FVS,
    eps: f64,
) {
    assert!(x.n == lp.m);
    assert!(y.n == lp.n - lp.m);
    
    if ign {
        // y := 0
        fvs_clear_vec(y);
    }
    
    let mut nnz = y.nnz;
    for k in (1..=x.nnz).rev() {
        let i = x.ind[k] as usize;
        // y := y + s * (i-th row of N) * x[i]
        let t = s * x.vec[i];
        let ptr = nt.ptr[i] as usize;
        let end = ptr + nt.len[i] as usize;
        
        for pos in ptr..end {
            let j = nt.ind[pos] as usize;
            if y.vec[j] == 0.0 {
                nnz += 1;
                y.ind[nnz] = j as i32;
            }
            y.vec[j] += nt.val[pos] * t;
            // Handle numeric cancellation
            if y.vec[j] == 0.0 {
                y.vec[j] = f64::MIN_POSITIVE;
            }
        }
    }
    
    y.nnz = nnz;
    fvs_adjust_vec(y, eps);
}

pub fn spx_free_nt(_lp: &SPXLP, nt: &mut SPXNT) {
    // Memory will be automatically freed when SPXNT goes out of scope
    // In Rust we don't need explicit free operations
}