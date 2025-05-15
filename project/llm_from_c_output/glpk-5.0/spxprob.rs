/* spxprob.rs */

use std::f64;
use std::mem;
use std::ptr;

#[derive(Debug)]
pub struct SPXLP {
    m: i32,
    n: i32,
    nnz: i32,
    A_ptr: Vec<i32>,
    A_ind: Vec<i32>,
    A_val: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
    l: Vec<f64>,
    u: Vec<f64>,
    head: Vec<i32>,
    flag: Vec<u8>,
    valid: i32,
    bfd: Option<Box<dyn std::any::Any>>,
}

impl SPXLP {
    pub fn new() -> Self {
        SPXLP {
            m: 0,
            n: 0,
            nnz: 0,
            A_ptr: Vec::new(),
            A_ind: Vec::new(),
            A_val: Vec::new(),
            b: Vec::new(),
            c: Vec::new(),
            l: Vec::new(),
            u: Vec::new(),
            head: Vec::new(),
            flag: Vec::new(),
            valid: 0,
            bfd: None,
        }
    }

    pub fn init_lp(&mut self, P: &glp_prob, excl: i32) {
        let m = P.m;
        assert!(m > 0);
        let mut n = 0;
        let mut nnz = P.nnz;
        assert!(P.valid != 0);

        for i in 1..=m {
            let row = &P.row[i as usize];
            if excl != 0 && row.stat == GLP_NS {
                // skip non-basic fixed auxiliary variable
            } else {
                n += 1;
                nnz += 1;
            }
        }

        for j in 1..=P.n {
            let col = &P.col[j as usize];
            if excl != 0 && col.stat == GLP_NS {
                let mut aij = col.ptr;
                while !aij.is_null() {
                    nnz -= 1;
                    aij = unsafe { (*aij).c_next };
                }
            } else {
                n += 1;
            }
        }

        self.m = m;
        assert!(n > 0);
        self.n = n;
        self.nnz = nnz;
    }

    pub fn alloc_lp(&mut self) {
        let m = self.m;
        let n = self.n;
        let nnz = self.nnz;

        self.A_ptr = vec![0; (n + 2) as usize];
        self.A_ind = vec![0; (nnz + 1) as usize];
        self.A_val = vec![0.0; (nnz + 1) as usize];
        self.b = vec![0.0; (m + 1) as usize];
        self.c = vec![0.0; (n + 1) as usize];
        self.l = vec![0.0; (n + 1) as usize];
        self.u = vec![0.0; (n + 1) as usize];
        self.head = vec![0; (n + 1) as usize];
        self.flag = vec![0; (n - m + 1) as usize];
    }

    pub fn build_lp(
        &mut self,
        P: &glp_prob,
        excl: i32,
        shift: i32,
        map: &mut [i32],
    ) {
        let m = self.m;
        let n = self.n;
        let nnz = self.nnz;

        let dir = match P.dir {
            GLP_MIN => 1.0,
            GLP_MAX => -1.0,
            _ => panic!("invalid direction"),
        };

        self.c[0] = dir * P.c0;
        let mut k = 0;
        let mut ptr = 1;

        assert_eq!(P.m, m);
        for i in 1..=m {
            let row = &P.row[i as usize];
            if excl != 0 && row.stat == GLP_NS {
                assert_eq!(row.type_, GLP_FX);
                map[i as usize] = 0;
                self.b[i as usize] = -row.lb * row.rii;
            } else {
                k += 1;
                map[i as usize] = k;
                self.A_ptr[k as usize] = ptr;
                self.A_ind[ptr as usize] = i;
                self.A_val[ptr as usize] = 1.0;
                ptr += 1;
                self.b[i as usize] = 0.0;
                self.c[k as usize] = 0.0;

                match row.type_ {
                    GLP_FR => {
                        self.l[k as usize] = -f64::MAX;
                        self.u[k as usize] = f64::MAX;
                    }
                    GLP_LO => {
                        self.l[k as usize] = row.lb * row.rii;
                        self.u[k as usize] = f64::MAX;
                    }
                    GLP_UP => {
                        self.l[k as usize] = -f64::MAX;
                        self.u[k as usize] = row.ub * row.rii;
                    }
                    GLP_DB => {
                        self.l[k as usize] = row.lb * row.rii;
                        self.u[k as usize] = row.ub * row.rii;
                        assert_ne!(self.l[k as usize], self.u[k as usize]);
                    }
                    GLP_FX => {
                        self.l[k as usize] = row.lb * row.rii;
                        self.u[k as usize] = row.lb * row.rii;
                    }
                    _ => panic!("invalid row type"),
                }
            }
        }

        for j in 1..=P.n {
            let col = &P.col[j as usize];
            let mut aij = col.ptr;
            if excl != 0 && col.stat == GLP_NS {
                assert_eq!(col.type_, GLP_FX);
                map[(m + j) as usize] = 0;
                if col.lb != 0.0 {
                    while !aij.is_null() {
                        let aij_ref = unsafe { &*aij };
                        self.b[aij_ref.row.i as usize] +=
                            (aij_ref.row.rii * aij_ref.val) * col.lb;
                        aij = aij_ref.c_next;
                    }
                    self.c[0] += (dir * col.coef) * col.lb;
                }
            } else {
                k += 1;
                map[(m + j) as usize] = k;
                self.A_ptr[k as usize] = ptr;
                while !aij.is_null() {
                    let aij_ref = unsafe { &*aij };
                    self.A_ind[ptr as usize] = aij_ref.row.i;
                    self.A_val[ptr as usize] =
                        -aij_ref.row.rii * aij_ref.val * col.sjj;
                    ptr += 1;
                    aij = aij_ref.c_next;
                }
                self.c[k as usize] = dir * col.coef * col.sjj;

                match col.type_ {
                    GLP_FR => {
                        self.l[k as usize] = -f64::MAX;
                        self.u[k as usize] = f64::MAX;
                    }
                    GLP_LO => {
                        self.l[k as usize] = col.lb / col.sjj;
                        self.u[k as usize] = f64::MAX;
                    }
                    GLP_UP => {
                        self.l[k as usize] = -f64::MAX;
                        self.u[k as usize] = col.ub / col.sjj;
                    }
                    GLP_DB => {
                        self.l[k as usize] = col.lb / col.sjj;
                        self.u[k as usize] = col.ub / col.sjj;
                        assert_ne!(self.l[k as usize], self.u[k as usize]);
                    }
                    GLP_FX => {
                        self.l[k as usize] = col.lb / col.sjj;
                        self.u[k as usize] = col.lb / col.sjj;
                    }
                    _ => panic!("invalid column type"),
                }
            }
        }

        assert_eq!(k, n);
        assert_eq!(ptr, nnz + 1);
        self.A_ptr[(n + 1) as usize] = ptr;

        if shift != 0 {
            for kk in 1..=(m + P.n) {
                let mut k = map[kk as usize];
                if k == 0 {
                    continue;
                }

                let mut delta = 0.0;
                if self.l[k as usize] == -f64::MAX && self.u[k as usize] == f64::MAX {
                    delta = 0.0;
                } else if self.l[k as usize] != -f64::MAX && self.u[k as usize] == f64::MAX {
                    delta = self.l[k as usize];
                    self.l[k as usize] = 0.0;
                } else if self.l[k as usize] == -f64::MAX && self.u[k as usize] != f64::MAX {
                    map[kk as usize] = -k;
                    delta = self.u[k as usize];
                    self.u[k as usize] = 0.0;
                } else if self.l[k as usize] != self.u[k as usize] {
                    if self.l[k as usize].abs() <= self.u[k as usize].abs() {
                        delta = self.l[k as usize];
                        self.l[k as usize] = 0.0;
                        self.u[k as usize] -= delta;
                    } else {
                        map[kk as usize] = -k;
                        delta = self.u[k as usize];
                        self.l[k as usize] -= delta;
                        self.u[k as usize] = 0.0;
                    }
                    assert_ne!(self.l[k as usize], self.u[k as usize]);
                } else {
                    delta = self.l[k as usize];
                    self.l[k as usize] = 0.0;
                    self.u[k as usize] = 0.0;
                }

                if delta != 0.0 {
                    let mut ptr = self.A_ptr[k as usize];
                    let end = self.A_ptr[(k + 1) as usize];
                    while ptr < end {
                        let idx = self.A_ind[ptr as usize] as usize;
                        self.b[idx] -= self.A_val[ptr as usize] * delta;
                        ptr += 1;
                    }
                    self.c[0] += self.c[k as usize] * delta;
                }
            }
        }
    }

    pub fn build_basis(&mut self, P: &mut glp_prob, map: &[i32]) {
        let m = self.m;
        let n = self.n;

        assert_eq!(P.m, m);
        assert_ne!(P.valid, 0);

        self.head[1..=(m as usize)].fill(0);
        let mut jj = 0;

        for i in 1..=m {
            let row = &P.row[i as usize];
            let mut k = map[i as usize];
            if k < 0 {
                k = -k;
            }
            if k == 0 {
                continue;
            }
            assert!(1 <= k && k <= n);

            if row.stat == GLP_BS {
                let ii = row.bind;
                assert!(1 <= ii && ii <= m);
                assert_eq!(self.head[ii as usize], 0);
                self.head[ii as usize] = k;
            } else {
                jj += 1;
                self.head[(m + jj) as usize] = k;
                self.flag[jj as usize] = if row.stat == GLP_NU { 1 } else { 0 };
            }
        }

        for j in 1..=P.n {
            let col = &P.col[j as usize];
            let mut k = map[(m + j) as usize];
            if k < 0 {
                k = -k;
            }
            if k == 0 {
                continue;
            }
            assert!(1 <= k && k <= n);

            if col.stat == GLP_BS {
                let ii = col.bind;
                assert!(1 <= ii && ii <= m);
                assert_eq!(self.head[ii as usize], 0);
                self.head[ii as usize] = k;
            } else {
                jj += 1;
                self.head[(m + jj) as usize] = k;
                self.flag[jj as usize] = if col.stat == GLP_NU { 1 } else { 0 };
            }
        }

        assert_eq!(m + jj, n);
        self.valid = 1;
        self.bfd = P.bfd.take();
        P.valid = 0;
    }

    pub fn store_basis(
        &self,
        P: &mut glp_prob,
        map: &[i32],
        daeh: &mut [i32],
    ) {
        let m = self.m;
        let n = self.n;

        for kk in 1..=n {
            daeh[self.head[kk as usize] as usize] = kk;
        }

        assert_eq!(P.m, m);
        for i in 1..=m {
            let row = &mut P.row[i as usize];
            let mut k = map[i as usize];
            if k < 0 {
                k = -k;
            }
            if k == 0 {
                assert_eq!(row.type_, GLP_FX);
                row.stat = GLP_NS;
                row.bind = 0;
            } else {
                let kk = daeh[k as usize];
                if kk <= m {
                    P.head[kk as usize] = i;
                    row.stat = GLP_BS;
                    row.bind = kk;
                } else {
                    row.stat = match row.type_ {
                        GLP_FR => GLP_NF,
                        GLP_LO => GLP_NL,
                        GLP_UP => GLP_NU,
                        GLP_DB => {
                            if self.flag[(kk - m) as usize] != 0 {
                                GLP_NU
                            } else {
                                GLP_NL
                            }
                        }
                        GLP_FX => GLP_NS,
                        _ => panic!("invalid row type"),
                    };
                    row.bind = 0;
                }
            }
        }

        for j in 1..=P.n {
            let col = &mut P.col[j as usize];
            let mut k = map[(m + j) as usize];
            if k < 0 {
                k = -k;
            }
            if k == 0 {
                assert_eq!(col.type_, GLP_FX);
                col.stat = GLP_NS;
                col.bind = 0;
            } else {
                let kk = daeh[k as usize];
                if kk <= m {
                    P.head[kk as usize] = m + j;
                    col.stat = GLP_BS;
                    col.bind = kk;
                } else {
                    col.stat = match col.type_ {
                        GLP_FR => GLP_NF,
                        GLP_LO => GLP_NL,
                        GLP_UP => GLP_NU,
                        GLP_DB => {
                            if self.flag[(kk - m) as usize] != 0 {
                                GLP_NU
                            } else {
                                GLP_NL
                            }
                        }
                        GLP_FX => GLP_NS,
                        _ => panic!("invalid column type"),
                    };
                    col.bind = 0;
                }
            }
        }
    }

    pub fn store_sol(
        &self,
        P: &mut glp_prob,
        shift: i32,
        map: &[i32],
        daeh: &[i32],
        beta: &[f64],
        pi: &[f64],
        d: &[f64],
    ) {
        let m = self.m;
        let dir = match P.dir {
            GLP_MIN => 1.0,
            GLP_MAX => -1.0,
            _ => panic!("invalid direction"),
        };

        assert_eq!(P.m, m);
        for i in 1..=m {
            let row = &mut P.row[i as usize];
            let mut k = map[i as usize];
            if k < 0 {
                k = -k;
            }
            if k == 0 {
                assert_eq!(row.type_, GLP_FX);
                row.prim = row.lb;
                row.dual = -dir * pi[i as usize] * row.rii;
            } else {
                let kk = daeh[k as usize];
                if kk <= m {
                    row.prim = beta[kk as usize] / row.rii;
                    if shift != 0 {
                        row.prim += if map[i as usize] < 0 { row.ub } else { row.lb };
                    }
                    row.dual = 0.0;
                } else {
                    row.prim = if self.flag[(kk - m) as usize] != 0 { row.ub } else { row.lb };
                    row.dual = (dir * d[(kk - m) as usize]) * row.rii;
                }
            }
        }

        P.obj_val = P.c0;
        for j in 1..=P.n {
            let col = &mut P.col[j as usize];
            let mut k = map[(m + j) as usize];
            if k < 0 {
                k = -k;
            }
            if k == 0 {
                assert_eq!(col.type_, GLP_FX);
                col.prim = col.lb;
                let mut dk = dir * col.coef;
                let mut aij = col.ptr;
                while !aij.is_null() {
                    let aij_ref = unsafe { &*aij };
                    dk += (aij_ref.row.rii * aij_ref.val) * pi[aij_ref.row.i as usize];
                    aij = aij_ref.c_next;
                }
                col.dual = dir * dk;
            } else {
                let kk = daeh[k as usize];
                if kk <= m {
                    col.prim = beta[kk as usize] * col.sjj;
                    if shift != 0 {
                        col.prim += if map[(m + j) as usize] < 0 { col.ub } else { col.lb };
                    }
                    col.dual = 0.0;
                } else {
                    col.prim = if self.flag[(kk - m) as usize] != 0 { col.ub }