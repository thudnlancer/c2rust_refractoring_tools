use std::cmp::{max, min};
use std::ptr;
use std::slice;
use libc::{c_double, c_int, c_ulong, c_void};

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct GslBlock {
    pub size: size_t,
    pub data: Vec<c_double>,
}

#[derive(Debug, Clone)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: Vec<c_double>,
    pub block: Option<Box<GslBlock>>,
    pub owner: c_int,
}

#[derive(Debug, Clone)]
pub struct GslMatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: Vec<c_double>,
    pub block: Option<Box<GslBlock>>,
    pub owner: c_int,
}

#[derive(Debug, Clone)]
pub struct GslBsplineWorkspace {
    pub k: size_t,
    pub km1: size_t,
    pub l: size_t,
    pub nbreak: size_t,
    pub n: size_t,
    pub knots: GslVector,
    pub deltal: GslVector,
    pub deltar: GslVector,
    pub b: GslVector,
    pub a: GslMatrix,
    pub db: GslMatrix,
}

impl GslVector {
    pub fn new(n: size_t) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::Invalid);
        }
        Ok(Self {
            size: n,
            stride: 1,
            data: vec![0.0; n as usize],
            block: None,
            owner: 1,
        })
    }

    pub fn get(&self, i: size_t) -> Result<c_double, GslError> {
        if i >= self.size {
            return Err(GslError::Invalid);
        }
        Ok(self.data[(i * self.stride) as usize])
    }

    pub fn set(&mut self, i: size_t, x: c_double) -> Result<(), GslError> {
        if i >= self.size {
            return Err(GslError::Invalid);
        }
        self.data[(i * self.stride) as usize] = x;
        Ok(())
    }
}

impl GslMatrix {
    pub fn new(n1: size_t, n2: size_t) -> Result<Self, GslError> {
        if n1 == 0 || n2 == 0 {
            return Err(GslError::Invalid);
        }
        Ok(Self {
            size1: n1,
            size2: n2,
            tda: n2,
            data: vec![0.0; (n1 * n2) as usize],
            block: None,
            owner: 1,
        })
    }

    pub fn get(&self, i: size_t, j: size_t) -> Result<c_double, GslError> {
        if i >= self.size1 || j >= self.size2 {
            return Err(GslError::Invalid);
        }
        Ok(self.data[(i * self.tda + j) as usize])
    }

    pub fn set(&mut self, i: size_t, j: size_t, x: c_double) -> Result<(), GslError> {
        if i >= self.size1 || j >= self.size2 {
            return Err(GslError::Invalid);
        }
        self.data[(i * self.tda + j) as usize] = x;
        Ok(())
    }

    pub fn column(&self, j: size_t) -> Result<GslVector, GslError> {
        if j >= self.size2 {
            return Err(GslError::Invalid);
        }
        Ok(GslVector {
            size: self.size1,
            stride: self.tda,
            data: self.data[j as usize..].to_vec(),
            block: None,
            owner: 0,
        })
    }
}

impl GslBsplineWorkspace {
    pub fn new(k: size_t, nbreak: size_t) -> Result<Self, GslError> {
        if k == 0 {
            return Err(GslError::Invalid);
        }
        if nbreak < 2 {
            return Err(GslError::Invalid);
        }

        let km1 = k - 1;
        let l = nbreak - 1;
        let n = l + k - 1;

        let knots = GslVector::new(n + k)?;
        let deltal = GslVector::new(k)?;
        let deltar = GslVector::new(k)?;
        let b = GslVector::new(k)?;
        let a = GslMatrix::new(k, k)?;
        let db = GslMatrix::new(k, k + 1)?;

        Ok(Self {
            k,
            km1,
            l,
            nbreak,
            n,
            knots,
            deltal,
            deltar,
            b,
            a,
            db,
        })
    }

    pub fn ncoeffs(&self) -> size_t {
        self.n
    }

    pub fn order(&self) -> size_t {
        self.k
    }

    pub fn nbreak(&self) -> size_t {
        self.nbreak
    }

    pub fn breakpoint(&self, i: size_t) -> Result<c_double, GslError> {
        let j = i + self.k - 1;
        self.knots.get(j)
    }

    pub fn knots(&mut self, breakpts: &GslVector) -> Result<(), GslError> {
        if breakpts.size != self.nbreak {
            return Err(GslError::BadLen);
        }

        for i in 0..self.k {
            self.knots.set(i, breakpts.get(0)?)?;
        }

        for i in 1..self.l {
            self.knots
                .set(self.k - 1 + i, breakpts.get(i)?)?;
        }

        for i in self.n..(self.n + self.k) {
            self.knots.set(i, breakpts.get(self.l)?)?;
        }

        Ok(())
    }

    pub fn knots_uniform(&mut self, a: c_double, b: c_double) -> Result<(), GslError> {
        let delta = (b - a) / self.l as c_double;
        let mut x = a + delta;

        for i in 0..self.k {
            self.knots.set(i, a)?;
        }

        for i in 0..(self.l - 1) {
            self.knots.set(self.k + i, x)?;
            x += delta;
        }

        for i in self.n..(self.n + self.k) {
            self.knots.set(i, b)?;
        }

        Ok(())
    }

    pub fn eval(&self, x: c_double, b: &mut GslVector) -> Result<(), GslError> {
        if b.size != self.n {
            return Err(GslError::BadLen);
        }

        let (istart, iend) = self.eval_nonzero(x, &mut self.b.clone())?;

        for i in 0..istart {
            b.set(i, 0.0)?;
        }

        for i in istart..=iend {
            b.set(i, self.b.get(i - istart)?)?;
        }

        for i in (iend + 1)..self.n {
            b.set(i, 0.0)?;
        }

        Ok(())
    }

    pub fn eval_nonzero(
        &self,
        x: c_double,
        bk: &mut GslVector,
    ) -> Result<(size_t, size_t), GslError> {
        if bk.size != self.k {
            return Err(GslError::BadLen);
        }

        let (i, flag) = self.find_interval(x)?;
        self.process_interval_for_eval(x, i, flag)?;

        let istart = i - self.k + 1;
        let iend = i;

        self.bsplvb(&self.knots, self.k, 1, x, iend, &mut self.deltal.clone(), &mut self.deltar.clone(), bk)?;

        Ok((istart, iend))
    }

    pub fn deriv_eval(
        &self,
        x: c_double,
        nderiv: size_t,
        db: &mut GslMatrix,
    ) -> Result<(), GslError> {
        if db.size1 != self.n {
            return Err(GslError::BadLen);
        }
        if db.size2 < nderiv + 1 {
            return Err(GslError::BadLen);
        }

        let (istart, iend) = self.deriv_eval_nonzero(x, nderiv, &mut self.db.clone())?;

        for j in 0..=nderiv {
            for i in 0..istart {
                db.set(i, j, 0.0)?;
            }

            for i in istart..=iend {
                db.set(i, j, self.db.get(i - istart, j)?)?;
            }

            for i in (iend + 1)..self.n {
                db.set(i, j, 0.0)?;
            }
        }

        Ok(())
    }

    pub fn deriv_eval_nonzero(
        &self,
        x: c_double,
        nderiv: size_t,
        db: &mut GslMatrix,
    ) -> Result<(size_t, size_t), GslError> {
        if db.size1 != self.k {
            return Err(GslError::BadLen);
        }
        if db.size2 < nderiv + 1 {
            return Err(GslError::BadLen);
        }

        let (i, flag) = self.find_interval(x)?;
        self.process_interval_for_eval(x, i, flag)?;

        let istart = i - self.k + 1;
        let iend = i;

        self.bsplvd(
            &self.knots,
            self.k,
            x,
            iend,
            &mut self.deltal.clone(),
            &mut self.deltar.clone(),
            &mut self.a.clone(),
            db,
            nderiv,
        )?;

        let min_nderivk = min(nderiv, self.k - 1);
        for j in (min_nderivk + 1)..=nderiv {
            for i in 0..self.k {
                db.set(i, j, 0.0)?;
            }
        }

        Ok((istart, iend))
    }

    fn find_interval(&self, x: c_double) -> Result<(size_t, c_int), GslError> {
        if x < self.knots.get(0)? {
            return Ok((0, -1));
        }

        let mut i = self.k - 1;
        while i < self.k + self.l - 1 {
            let ti = self.knots.get(i)?;
            let tip1 = self.knots.get(i + 1)?;

            if tip1 < ti {
                return Err(GslError::Invalid);
            }

            if (ti <= x && x < tip1) || (ti < x && x == tip1 && tip1 == self.knots.get(self.k + self.l - 1)?) {
                break;
            }

            i += 1;
        }

        if i == self.k + self.l - 1 {
            Ok((i, 1))
        } else {
            Ok((i, 0))
        }
    }

    fn process_interval_for_eval(
        &self,
        x: c_double,
        i: size_t,
        flag: c_int,
    ) -> Result<(), GslError> {
        if flag == -1 {
            return Err(GslError::Invalid);
        } else if flag == 1 {
            if x <= self.knots.get(i)? + 2.2204460492503131e-16 {
                return Ok(());
            } else {
                return Err(GslError::Invalid);
            }
        }

        if self.knots.get(i)? == self.knots.get(i + 1)? {
            return Err(GslError::Invalid);
        }

        Ok(())
    }

    fn bsplvb(
        &self,
        t: &GslVector,
        jhigh: size_t,
        index: size_t,
        x: c_double,
        left: size_t,
        deltal: &mut GslVector,
        deltar: &mut GslVector,
        biatx: &mut GslVector,
    ) -> Result<(), GslError> {
        let mut j = 0;

        if index == 1 {
            j = 0;
            biatx.set(0, 1.0)?;
        }

        while j < jhigh - 1 {
            deltar.set(
                j,
                t.get(left + j + 1)? - x,
            )?;
            deltal.set(j, x - t.get(left - j)?)?;

            let mut saved = 0.0;
            for i in 0..=j {
                let term = biatx.get(i)? / (deltar.get(i)? + deltal.get(j - i)?);
                biatx.set(i, saved + deltar.get(i)? * term)?;
                saved = deltal.get(j - i)? * term;
            }

            biatx.set(j + 1, saved)?;
            j += 1;
        }

        Ok(())
    }

    fn bsplvd(
        &self,
        t: &GslVector,
        k: size_t,
        x: c_double,
        left: size_t,
        deltal: &mut GslVector,
        deltar: &mut GslVector,
        a: &mut GslMatrix,
        dbiatx: &mut GslMatrix,
        nderiv: size_t,
    ) -> Result<(), GslError> {
        let mhigh = min(nderiv, k - 1) as c_int;
        let mut bsplvb_j = 0;

        let mut dbcol = dbiatx.column(0)?;
        self.bsplvb(
            t,
            k - mhigh as size_t,
            1,
            x,
            left,
            deltal,
            deltar,
            &mut dbcol,
        )?;

        if mhigh > 0 {
            let mut ideriv = mhigh;
            let mut m = 1;

            while m <= mhigh {
                let mut j = ideriv;
                let mut jp1mid = 0;

                while j < k as c_int {
                    dbiatx.set(
                        j as size_t,
                        ideriv as size_t,
                        dbiatx.get(jp1mid as size_t, 0)?,
                    )?;
                    j += 1;
                    jp1mid += 1;
                }

                ideriv -= 1;
                self.bsplvb(
                    t,
                    k - ideriv as size_t,
                    2,
                    x,
                    left,
                    deltal,
                    deltar,
                    &mut dbcol,
                )?;
                m += 1;
            }

            let mut jlow = 0;
            for i in 0..k as c_int {
                for j in jlow..k as c_int {
                    a.set(j as size_t, i as size_t, 0.0)?;
                }
                jlow = i;
                a.set(i as size_t, i as size_t, 1.0)?;
            }

            let mut m = 1;
            while m <= mhigh {
                let kmm = k - m as size_t;
                let fkmm = kmm as c_double;
                let mut il = left as c_int;
                let mut i = k as c_int - 1;
                let mut ldummy = 0;

                while ldummy < kmm as c_int {
                    let factor = fkmm / (t.get((il + kmm as c_int) as size_t)? - t.get(il as size_t)?);
                    for j in 0..=i {
                        a.set(
                            i as size_t,
                            j as size_t,
                            factor
                                * (a.get(i as size_t, j as size_t)?
                                    - a.get((i - 1) as size_t, j as size_t)?),
                        )?;
                    }
                    il -= 1;
                    i -= 1;
                    ldummy += 1;
                }

                for i in 0..k as c_int {
                    let mut sum = 0.0;
                    let jlow = max(i, m);
                    for j in jlow..k as c_int {
                        sum += a.get(j as size_t, i as size_t)? * dbiatx.get(j as size_t, m as size_t)?;
                    }
                    dbiatx.set(i as size_t, m as size_t, sum)?;
                }

                m += 1;
            }
        }

        Ok(())
    }
}