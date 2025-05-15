use std::cmp;
use std::collections::HashMap;
use std::f64;
use std::mem;
use std::ptr;

use num_rational::BigRational;
use num_traits::{One, Zero};

struct LUX {
    n: usize,
    pool: Vec<LUXELM>,
    f_row: Vec<Option<Box<LUXELM>>>,
    f_col: Vec<Option<Box<LUXELM>>>,
    v_piv: Vec<BigRational>,
    v_row: Vec<Option<Box<LUXELM>>>,
    v_col: Vec<Option<Box<LUXELM>>>,
    p_row: Vec<usize>,
    p_col: Vec<usize>,
    q_row: Vec<usize>,
    q_col: Vec<usize>,
    rank: usize,
}

struct LUXELM {
    i: usize,
    j: usize,
    val: BigRational,
    r_prev: Option<Box<LUXELM>>,
    r_next: Option<Box<LUXELM>>,
    c_prev: Option<Box<LUXELM>>,
    c_next: Option<Box<LUXELM>>,
}

struct LUXWKA {
    r_len: Vec<usize>,
    r_head: Vec<usize>,
    r_prev: Vec<usize>,
    r_next: Vec<usize>,
    c_len: Vec<usize>,
    c_head: Vec<usize>,
    c_prev: Vec<usize>,
    c_next: Vec<usize>,
}

impl LUX {
    fn new(n: usize) -> Self {
        if n < 1 {
            panic!("lux_create: n = {}; invalid parameter", n);
        }

        let mut lux = LUX {
            n,
            pool: Vec::new(),
            f_row: vec![None; n + 1],
            f_col: vec![None; n + 1],
            v_piv: vec![BigRational::zero(); n + 1],
            v_row: vec![None; n + 1],
            v_col: vec![None; n + 1],
            p_row: vec![0; n + 1],
            p_col: vec![0; n + 1],
            q_row: vec![0; n + 1],
            q_col: vec![0; n + 1],
            rank: n,
        };

        for k in 1..=n {
            lux.f_row[k] = None;
            lux.f_col[k] = None;
            lux.v_piv[k] = BigRational::one();
            lux.v_row[k] = None;
            lux.v_col[k] = None;
            lux.p_row[k] = k;
            lux.p_col[k] = k;
            lux.q_row[k] = k;
            lux.q_col[k] = k;
        }

        lux
    }

    fn decomp(
        &mut self,
        col: impl Fn(usize) -> (Vec<usize>, Vec<BigRational>),
    ) -> bool {
        let n = self.n;
        let mut wka = LUXWKA {
            r_len: vec![0; n + 1],
            r_head: vec![0; n + 1],
            r_prev: vec![0; n + 1],
            r_next: vec![0; n + 1],
            c_len: vec![0; n + 1],
            c_head: vec![0; n + 1],
            c_prev: vec![0; n + 1],
            c_next: vec![0; n + 1],
        };

        self.initialize(&col, &mut wka);

        let mut flag = vec![0; n + 1];
        let mut work = vec![BigRational::zero(); n + 1];

        for k in 1..=n {
            let piv = self.find_pivot(&wka);
            if piv.is_none() {
                break;
            }

            let piv = piv.unwrap();
            let p = piv.i;
            let q = piv.j;

            let i = self.p_col[p];
            let j = self.q_row[q];

            if !(k <= i && i <= n && k <= j && j <= n) {
                panic!("lux_decomp: invalid pivot indices");
            }

            let t = self.p_row[k];
            self.p_row[i] = t;
            self.p_col[t] = i;
            self.p_row[k] = p;
            self.p_col[p] = k;

            let t = self.q_col[k];
            self.q_col[j] = t;
            self.q_row[t] = j;
            self.q_col[k] = q;
            self.q_row[q] = k;

            self.eliminate(&mut wka, piv, &mut flag, &mut work);
        }

        self.rank = k - 1;

        for j in 1..=n {
            assert!(self.v_col[j].is_none());
        }

        for i in 1..=n {
            let mut vij = self.v_row[i].take();
            while let Some(mut elem) = vij {
                let j = elem.j;
                elem.c_prev = None;
                elem.c_next = self.v_col[j].take();
                if let Some(ref mut next) = elem.c_next {
                    next.c_prev = Some(elem);
                }
                self.v_col[j] = Some(elem);
                vij = elem.r_next.take();
            }
        }

        self.rank < n
    }

    fn f_solve(&self, tr: bool, x: &mut [BigRational]) {
        let n = self.n;
        let mut temp = BigRational::zero();

        if !tr {
            for j in 1..=n {
                let k = self.p_row[j];
                if !x[k].is_zero() {
                    let mut fik = self.f_col[k].as_ref();
                    while let Some(elem) = fik {
                        temp = &elem.val * &x[k];
                        x[elem.i] -= &temp;
                        fik = elem.c_next.as_ref();
                    }
                }
            }
        } else {
            for i in (1..=n).rev() {
                let k = self.p_row[i];
                if !x[k].is_zero() {
                    let mut fkj = self.f_row[k].as_ref();
                    while let Some(elem) = fkj {
                        temp = &elem.val * &x[k];
                        x[elem.j] -= &temp;
                        fkj = elem.r_next.as_ref();
                    }
                }
            }
        }
    }

    fn v_solve(&self, tr: bool, x: &mut [BigRational]) {
        let n = self.n;
        let mut b = vec![BigRational::zero(); n + 1];
        for k in 1..=n {
            b[k] = x[k].clone();
            x[k] = BigRational::zero();
        }

        let mut temp = BigRational::zero();

        if !tr {
            for k in (1..=n).rev() {
                let i = self.p_row[k];
                let j = self.q_col[k];
                if !b[i].is_zero() {
                    x[j] = b[i].clone();
                    x[j] /= &self.v_piv[i];
                    let mut vij = self.v_col[j].as_ref();
                    while let Some(elem) = vij {
                        temp = &elem.val * &x[j];
                        b[elem.i] -= &temp;
                        vij = elem.c_next.as_ref();
                    }
                }
            }
        } else {
            for k in 1..=n {
                let i = self.p_row[k];
                let j = self.q_col[k];
                if !b[j].is_zero() {
                    x[i] = b[j].clone();
                    x[i] /= &self.v_piv[i];
                    let mut vij = self.v_row[i].as_ref();
                    while let Some(elem) = vij {
                        temp = &elem.val * &x[i];
                        b[elem.j] -= &temp;
                        vij = elem.r_next.as_ref();
                    }
                }
            }
        }
    }

    fn solve(&self, tr: bool, x: &mut [BigRational]) {
        if self.rank < self.n {
            panic!("lux_solve: LU-factorization has incomplete rank");
        }

        if !tr {
            self.f_solve(false, x);
            self.v_solve(false, x);
        } else {
            self.v_solve(true, x);
            self.f_solve(true, x);
        }
    }

    fn initialize(
        &mut self,
        col: &impl Fn(usize) -> (Vec<usize>, Vec<BigRational>),
        wka: &mut LUXWKA,
    ) {
        let n = self.n;
        self.pool.clear();

        for i in 1..=n {
            while let Some(mut fij) = self.f_row[i].take() {
                self.f_row[i] = fij.r_next.take();
                self.f_col[fij.j] = None;
            }
        }

        for k in 1..=n {
            self.v_piv[k] = BigRational::zero();
        }

        for i in 1..=n {
            while let Some(mut vij) = self.v_row[i].take() {
                self.v_row[i] = vij.r_next.take();
                self.v_col[vij.j] = None;
            }
        }

        for j in 1..=n {
            let (ind, val) = col(j);
            let len = ind.len();
            if !(0 <= len && len <= n) {
                panic!("lux_decomp: j = {}: len = {}; invalid column length", j, len);
            }

            for k in 0..len {
                let i = ind[k];
                if !(1 <= i && i <= n) {
                    panic!("lux_decomp: j = {}: i = {}; row index out of range", j, i);
                }

                if self.v_row[i].as_ref().map_or(false, |v| v.j == j) {
                    panic!("lux_decomp: j = {}: i = {}; duplicate row indices not allowed", j, i);
                }

                if val[k].is_zero() {
                    panic!("lux_decomp: j = {}: i = {}; zero elements not allowed", j, i);
                }

                let vij = LUXELM {
                    i,
                    j,
                    val: val[k].clone(),
                    r_prev: None,
                    r_next: self.v_row[i].take(),
                    c_prev: None,
                    c_next: self.v_col[j].take(),
                };

                if let Some(ref mut next) = vij.r_next {
                    next.r_prev = Some(Box::new(vij));
                }
                if let Some(ref mut next) = vij.c_next {
                    next.c_prev = Some(Box::new(vij));
                }

                self.v_row[i] = Some(Box::new(vij));
                self.v_col[j] = Some(Box::new(vij));
            }
        }

        for k in 1..=n {
            self.p_row[k] = k;
            self.p_col[k] = k;
            self.q_row[k] = k;
            self.q_col[k] = k;
        }

        self.rank = usize::MAX;

        for i in 1..=n {
            let mut len = 0;
            let mut vij = self.v_row[i].as_ref();
            while let Some(elem) = vij {
                len += 1;
                vij = elem.r_next.as_ref();
            }
            wka.r_len[i] = len;
        }

        for len in 0..=n {
            wka.r_head[len] = 0;
        }

        for i in 1..=n {
            let len = wka.r_len[i];
            wka.r_prev[i] = 0;
            wka.r_next[i] = wka.r_head[len];
            if wka.r_next[i] != 0 {
                wka.r_prev[wka.r_next[i]] = i;
            }
            wka.r_head[len] = i;
        }

        for j in 1..=n {
            let mut len = 0;
            let mut vij = self.v_col[j].as_ref();
            while let Some(elem) = vij {
                len += 1;
                vij = elem.c_next.as_ref();
            }
            wka.c_len[j] = len;
        }

        for len in 0..=n {
            wka.c_head[len] = 0;
        }

        for j in 1..=n {
            let len = wka.c_len[j];
            wka.c_prev[j] = 0;
            wka.c_next[j] = wka.c_head[len];
            if wka.c_next[j] != 0 {
                wka.c_prev[wka.c_next[j]] = j;
            }
            wka.c_head[len] = j;
        }
    }

    fn find_pivot(&self, wka: &LUXWKA) -> Option<&LUXELM> {
        let n = self.n;
        let mut piv = None;
        let mut best = f64::MAX;
        let mut ncand = 0;
        let piv_lim = 5;

        let j = wka.c_head[1];
        if j != 0 {
            assert_eq!(wka.c_len[j], 1);
            piv = self.v_col[j].as_ref();
            assert!(piv.as_ref().map_or(true, |v| v.c_next.is_none()));
            return piv;
        }

        let i = wka.r_head[1];
        if i != 0 {
            assert_eq!(wka.r_len[i], 1);
            piv = self.v_row[i].as_ref();
            assert!(piv.as_ref().map_or(true, |v| v.r_next.is_none()));
            return piv;
        }

        for len in 2..=n {
            let mut j = wka.c_head[len];
            while j != 0 {
                let mut some = None;
                let mut min_len = usize::MAX;
                let mut vij = self.v_col[j].as_ref();
                while let Some(elem) = vij {
                    if min_len > wka.r_len[elem.i] {
                        some = Some(elem);
                        min_len = wka.r_len[elem.i];
                    }
                    if min_len <= len {
                        piv = some;
                        return piv;
                    }
                    vij = elem.c_next.as_ref();
                }

                assert!(some.is_some());
                ncand += 1;
                let cost = (min_len - 1) as f64 * (len - 1) as f64;
                if cost < best {
                    piv = some;
                    best = cost;
                }
                if ncand == piv_lim {
                    return piv;
                }
                j = wka.c_next[j];
            }

            let mut i = wka.r_head[len];
            while i != 0 {
                let mut some = None;
                let mut min_len = usize::MAX;
                let mut vij = self.v_row[i].as_ref();
                while let Some(elem) = vij {
                    if min_len > wka.c_len[elem.j] {
                        some = Some(elem);
                        min_len = wka.c_len[elem.j];
                    }
                    if min_len <= len {
                        piv = some;
                        return piv;
                    }
                    vij = elem.r_next.as_ref();
                }

                assert!(some.is_some());
                ncand += 1;
                let cost = (len - 1) as f64 * (min_len - 1) as f64;
                if cost < best {
                    piv = some;
                    best = cost;
                }
                if ncand == piv_lim {
                    return piv;
                }
                i = wka.r_next[i];
            }
        }

        piv
    }

    fn eliminate(
        &mut self,
        wka: &mut LUXWKA,
        piv: &LUXELM,
        flag: &mut [usize],
        work: &mut [BigRational],
    ) {
        let n = self.n;
        let p = piv.i;
        let q = piv.j;

        if wka.r_prev[p] == 0 {
            wka.r_head[wka.r_len[p]] = wka.r_next[p];
        } else {
            wka.r_next[wka.r_prev[p]] = wka.r_next[p];
        }

        if wka.r_next[p] != 0 {
            wka.r_prev[wka.r_next[p]] = wka.r_prev[p];
        }

        if wka.c_prev[q] == 0 {
            wka.c_head[wka.c_len[q]] = wka.c_next[q];
        } else {
            wka.c_next[wka.c_prev[q]] = wka.c_next[q];
        }

        if wka.c_next[q] != 0 {
            wka.c_prev[wka.c_next[q]] = wka.c_prev[q];
        }

        self.v_piv[p] = piv.val.clone();

        if piv.r_prev.is_none() {
            self.v_row[p] = piv.r_next.clone();
        } else {
            piv.r_prev.as_ref().unwrap().r_next = piv.r_next.clone();
        }

        if piv.r_next.is_some() {
            piv.r_next.as_ref().unwrap().r_prev = piv.r_prev.clone();
        }

        wka.r_len[p] -= 1;

        if piv.c_prev.is_none() {
            self.v_col[q] = piv.c_next.clone();
        } else {
            piv.c_prev.as_ref().unwrap().c_next = piv.c_next.clone();
        }

        if piv.c_next.is_some() {
            piv.c_next.as_ref().unwrap().c_prev = piv.c_prev.clone();
        }

        wka.c_len[q] -= 1;

        let mut vpj = self.v_row[p].as_ref();
        while let Some(elem) = vpj {
            let j = elem.j;
            flag[j] = 1;
            work[j] = elem.val.clone();

            if wka.c_prev[j] == 0 {
                wka.c_head[wka.c_len[j]] = wka.c_next[j];
            } else {
                wka.c_next[wka.c_prev[j]] = wka.c_next[j];
            }

            if wka.c_next[j] != 0 {
                wka.c_prev[wka.c_next[j]] = wka.c_