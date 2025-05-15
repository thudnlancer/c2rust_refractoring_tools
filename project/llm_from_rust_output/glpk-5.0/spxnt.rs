use std::ptr;

#[derive(Debug, Clone)]
pub struct FVS {
    pub n: i32,
    pub nnz: i32,
    pub ind: Vec<i32>,
    pub vec: Vec<f64>,
}

impl FVS {
    pub fn new(n: i32) -> Self {
        FVS {
            n,
            nnz: 0,
            ind: vec![0; (n + 1) as usize],
            vec: vec![0.0; (n + 1) as usize],
        }
    }

    pub fn clear_vec(&mut self) {
        self.vec.iter_mut().for_each(|x| *x = 0.0);
        self.nnz = 0;
    }

    pub fn adjust_vec(&mut self, eps: f64) {
        for i in 1..=self.nnz {
            let idx = self.ind[i as usize] as usize;
            if self.vec[idx].abs() < eps {
                self.vec[idx] = 0.0;
            }
        }
        self.nnz = self.vec.iter().filter(|&&x| x != 0.0).count() as i32;
    }
}

#[derive(Debug, Clone)]
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub a_ptr: Vec<i32>,
    pub a_ind: Vec<i32>,
    pub a_val: Vec<f64>,
    pub b: Vec<f64>,
    pub c: Vec<f64>,
    pub l: Vec<f64>,
    pub u: Vec<f64>,
    pub head: Vec<i32>,
    pub flag: Vec<i8>,
    pub valid: i32,
    pub bfd: Option<Box<BFD>>,
}

#[derive(Debug, Clone)]
pub struct BFD;

#[derive(Debug, Clone)]
pub struct SPXNT {
    pub ptr: Vec<i32>,
    pub len: Vec<i32>,
    pub ind: Vec<i32>,
    pub val: Vec<f64>,
}

impl SPXNT {
    pub fn new(m: i32, nnz: i32) -> Self {
        SPXNT {
            ptr: vec![0; (m + 1) as usize],
            len: vec![0; (m + 1) as usize],
            ind: vec![0; (nnz + 1) as usize],
            val: vec![0.0; (nnz + 1) as usize],
        }
    }
}

pub fn spx_alloc_nt(lp: &SPXLP) -> SPXNT {
    SPXNT::new(lp.m, lp.nnz)
}

pub fn spx_init_nt(lp: &mut SPXLP, nt: &mut SPXNT) {
    let m = lp.m;
    let nnz = lp.nnz;

    nt.len[1..=m as usize].fill(0);

    for k in 1..=lp.n {
        let ptr = lp.a_ptr[k as usize];
        let end = lp.a_ptr[(k + 1) as usize];
        for p in ptr..end {
            let i = lp.a_ind[p as usize];
            nt.len[i as usize] += 1;
        }
    }

    nt.ptr[1] = 1;
    for i in 2..=m as usize {
        nt.ptr[i] = nt.ptr[i - 1] + nt.len[i - 1];
    }

    assert_eq!(
        nt.ptr[m as usize] + nt.len[m as usize],
        nnz + 1,
        "NT_ptr[m] + NT_len[m] == nnz+1"
    );
}

pub fn spx_nt_add_col(lp: &SPXLP, nt: &mut SPXNT, j: i32, k: i32) {
    assert!(1 <= j && j <= lp.n - lp.m, "1 <= j && j <= n-m");
    assert!(1 <= k && k <= lp.n, "1 <= k && k <= n");

    let ptr = lp.a_ptr[k as usize];
    let end = lp.a_ptr[(k + 1) as usize];

    for p in ptr..end {
        let i = lp.a_ind[p as usize];
        let pos = nt.ptr[i as usize] + nt.len[i as usize];
        nt.len[i as usize] += 1;

        if i < lp.m {
            assert!(pos < nt.ptr[(i + 1) as usize], "pos < NT_ptr[i+1]");
        } else {
            assert!(pos <= lp.nnz, "pos <= nnz");
        }

        nt.ind[pos as usize] = j;
        nt.val[pos as usize] = lp.a_val[p as usize];
    }
}

pub fn spx_build_nt(lp: &mut SPXLP, nt: &mut SPXNT) {
    let m = lp.m;
    let n = lp.n;

    nt.len[1..=m as usize].fill(0);

    for j in 1..=(n - m) {
        let k = lp.head[(m + j) as usize];
        spx_nt_add_col(lp, nt, j, k);
    }
}

pub fn spx_nt_del_col(lp: &SPXLP, nt: &mut SPXNT, j: i32, k: i32) {
    assert!(1 <= j && j <= lp.n - lp.m, "1 <= j && j <= n-m");
    assert!(1 <= k && k <= lp.n, "1 <= k && k <= n");

    let ptr = lp.a_ptr[k as usize];
    let end = lp.a_ptr[(k + 1) as usize];

    for p in ptr..end {
        let i = lp.a_ind[p as usize];
        let mut ptr1 = nt.ptr[i as usize];
        let end1 = ptr1 + nt.len[i as usize];

        while nt.ind[ptr1 as usize] != j {
            ptr1 += 1;
            assert!(ptr1 < end1, "ptr1 < end1");
        }

        nt.len[i as usize] -= 1;
        nt.ind[ptr1 as usize] = nt.ind[(end1 - 1) as usize];
        nt.val[ptr1 as usize] = nt.val[(end1 - 1) as usize];
    }
}

pub fn spx_update_nt(lp: &mut SPXLP, nt: &mut SPXNT, p: i32, q: i32) {
    assert!(1 <= p && p <= lp.m, "1 <= p && p <= m");
    assert!(1 <= q && q <= lp.n - lp.m, "1 <= q && q <= n-m");

    let m = lp.m;
    spx_nt_del_col(lp, nt, q, lp.head[(m + q) as usize]);
    spx_nt_add_col(lp, nt, q, lp.head[p as usize]);
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

    if ign {
        y[1..=(lp.n - lp.m) as usize].fill(0.0);
    }

    for i in 1..=m as usize {
        if x[i] != 0.0 {
            let t = s * x[i];
            let ptr = nt.ptr[i];
            let end = ptr + nt.len[i];
            for p in ptr..end {
                let j = nt.ind[p as usize] as usize;
                y[j] += nt.val[p as usize] * t;
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
    assert_eq!(x.n, lp.m, "x.n == lp.m");
    assert_eq!(y.n, lp.n - lp.m, "y.n == lp.n-lp.m");

    if ign {
        y.clear_vec();
    }

    let mut nnz = y.nnz;
    for k in (1..=x.nnz).rev() {
        let i = x.ind[k as usize] as usize;
        let t = s * x.vec[i];
        let ptr = nt.ptr[i];
        let end = ptr + nt.len[i];

        for p in ptr..end {
            let j = nt.ind[p as usize] as usize;
            if y.vec[j] == 0.0 {
                nnz += 1;
                y.ind[nnz as usize] = j as i32;
            }
            y.vec[j] += nt.val[p as usize] * t;
            if y.vec[j] == 0.0 {
                y.vec[j] = f64::MIN_POSITIVE;
            }
        }
    }

    y.nnz = nnz;
    y.adjust_vec(eps);
}