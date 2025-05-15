use num_complex::Complex64;
use std::cmp::min;
use std::f64::consts::DBL_MIN;
use std::ops::{Mul, Div, Neg};

const CROSSOVER_LU: usize = 16;
const GSL_COMPLEX_ONE: Complex64 = Complex64 { re: 1.0, im: 0.0 };
const GSL_COMPLEX_NEGONE: Complex64 = Complex64 { re: -1.0, im: 0.0 };

#[derive(Debug)]
struct GslMatrixComplex {
    data: Vec<Vec<Complex64>>,
    size1: usize,
    size2: usize,
}

#[derive(Debug)]
struct GslVectorComplex {
    data: Vec<Complex64>,
    size: usize,
}

#[derive(Debug)]
struct GslVectorUint {
    data: Vec<usize>,
    size: usize,
}

#[derive(Debug)]
struct GslPermutation {
    data: Vec<usize>,
    size: usize,
}

impl GslMatrixComplex {
    fn new(size1: usize, size2: usize) -> Self {
        GslMatrixComplex {
            data: vec![vec![Complex64::default(); size2]; size1],
            size1,
            size2,
        }
    }

    fn get(&self, i: usize, j: usize) -> Complex64 {
        self.data[i][j]
    }

    fn set(&mut self, i: usize, j: usize, value: Complex64) {
        self.data[i][j] = value;
    }

    fn ptr(&mut self, i: usize, j: usize) -> &mut Complex64 {
        &mut self.data[i][j]
    }

    fn submatrix(&self, i: usize, j: usize, rows: usize, cols: usize) -> GslMatrixComplex {
        let mut sub = GslMatrixComplex::new(rows, cols);
        for row in 0..rows {
            for col in 0..cols {
                sub.data[row][col] = self.data[i + row][j + col];
            }
        }
        sub
    }

    fn submatrix_mut(&mut self, i: usize, j: usize, rows: usize, cols: usize) -> GslMatrixComplex {
        let mut sub = GslMatrixComplex::new(rows, cols);
        for row in 0..rows {
            for col in 0..cols {
                sub.data[row][col] = self.data[i + row][j + col];
            }
        }
        sub
    }

    fn subcolumn(&self, j: usize, offset: usize, len: usize) -> GslVectorComplex {
        let mut v = GslVectorComplex::new(len);
        for i in 0..len {
            v.data[i] = self.data[offset + i][j];
        }
        v
    }

    fn subcolumn_mut(&mut self, j: usize, offset: usize, len: usize) -> GslVectorComplex {
        let mut v = GslVectorComplex::new(len);
        for i in 0..len {
            v.data[i] = self.data[offset + i][j];
        }
        v
    }

    fn subrow(&self, i: usize, offset: usize, len: usize) -> GslVectorComplex {
        let mut v = GslVectorComplex::new(len);
        for j in 0..len {
            v.data[j] = self.data[i][offset + j];
        }
        v
    }

    fn row(&self, i: usize) -> GslVectorComplex {
        self.subrow(i, 0, self.size2)
    }

    fn row_mut(&mut self, i: usize) -> GslVectorComplex {
        let mut v = GslVectorComplex::new(self.size2);
        for j in 0..self.size2 {
            v.data[j] = self.data[i][j];
        }
        v
    }

    fn memcpy(&mut self, src: &GslMatrixComplex) -> Result<(), &'static str> {
        if self.size1 != src.size1 || self.size2 != src.size2 {
            return Err("matrix dimensions don't match");
        }
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                self.data[i][j] = src.data[i][j];
            }
        }
        Ok(())
    }
}

impl GslVectorComplex {
    fn new(size: usize) -> Self {
        GslVectorComplex {
            data: vec![Complex64::default(); size],
            size,
        }
    }

    fn get(&self, i: usize) -> Complex64 {
        self.data[i]
    }

    fn set(&mut self, i: usize, value: Complex64) {
        self.data[i] = value;
    }

    fn ptr(&mut self, i: usize) -> &mut Complex64 {
        &mut self.data[i]
    }

    fn memcpy(&mut self, src: &GslVectorComplex) -> Result<(), &'static str> {
        if self.size != src.size {
            return Err("vector sizes don't match");
        }
        for i in 0..self.size {
            self.data[i] = src.data[i];
        }
        Ok(())
    }

    fn axpy(&mut self, alpha: Complex64, x: &GslVectorComplex) -> Result<(), &'static str> {
        if self.size != x.size {
            return Err("vector sizes don't match");
        }
        for i in 0..self.size {
            self.data[i] += alpha * x.data[i];
        }
        Ok(())
    }
}

impl GslVectorUint {
    fn new(size: usize) -> Self {
        GslVectorUint {
            data: vec![0; size],
            size,
        }
    }

    fn get(&self, i: usize) -> usize {
        self.data[i]
    }

    fn set(&mut self, i: usize, value: usize) {
        self.data[i] = value;
    }

    fn ptr(&mut self, i: usize) -> &mut usize {
        &mut self.data[i]
    }

    fn subvector(&self, offset: usize, len: usize) -> GslVectorUint {
        let mut v = GslVectorUint::new(len);
        for i in 0..len {
            v.data[i] = self.data[offset + i];
        }
        v
    }
}

impl GslPermutation {
    fn new(size: usize) -> Self {
        let mut data = vec![0; size];
        for i in 0..size {
            data[i] = i;
        }
        GslPermutation { data, size }
    }

    fn init(&mut self) {
        for i in 0..self.size {
            self.data[i] = i;
        }
    }
}

fn gsl_linalg_complex_LU_decomp(
    A: &mut GslMatrixComplex,
    p: &mut GslPermutation,
    signum: &mut i32,
) -> Result<(), &'static str> {
    let M = A.size1;

    if p.size != M {
        return Err("permutation length must match matrix size1");
    } else {
        let N = A.size2;
        let minMN = min(M, N);
        let mut ipiv = GslVectorUint::new(minMN);
        let mut AL = A.submatrix_mut(0, 0, M, minMN);

        LU_decomp_L3(&mut AL, &mut ipiv)?;

        if M < N {
            let mut AR = A.submatrix_mut(0, M, M, N - M);
            apply_pivots(&mut AR, &ipiv)?;
            ztrsm(
                CblasLeft,
                CblasLower,
                CblasNoTrans,
                CblasUnit,
                GSL_COMPLEX_ONE,
                &AL,
                &mut AR,
            )?;
        }

        p.init();
        *signum = 1;

        for i in 0..minMN {
            let pivi = ipiv.get(i);

            if p.data[pivi] != p.data[i] {
                let tmp = p.data[pivi];
                p.data[pivi] = p.data[i];
                p.data[i] = tmp;
                *signum = -(*signum);
            }
        }

        Ok(())
    }
}

fn LU_decomp_L2(A: &mut GslMatrixComplex, ipiv: &mut GslVectorUint) -> Result<(), &'static str> {
    let M = A.size1;
    let N = A.size2;
    let minMN = min(M, N);

    if ipiv.size != minMN {
        return Err("ipiv length must equal MIN(M,N)");
    } else {
        for j in 0..minMN {
            let v = A.subcolumn(j, j, M - j);
            let j_pivot = j + izamax(&v);

            ipiv.set(j, j_pivot);

            if j_pivot != j {
                let mut v1 = A.row_mut(j);
                let mut v2 = A.row_mut(j_pivot);
                zswap(&mut v1, &mut v2)?;
            }

            if j < M - 1 {
                let Ajj = A.get(j, j);
                let Ajjinv = Ajj.inv();

                if Ajj.norm() >= DBL_MIN {
                    let mut v1 = A.subcolumn_mut(j, j + 1, M - j - 1);
                    zscal(Ajjinv, &mut v1);
                } else {
                    for i in 1..M - j {
                        let ptr = A.ptr(j + i, j);
                        *ptr = *ptr * Ajjinv;
                    }
                }
            }

            if j < minMN - 1 {
                let mut A22 = A.submatrix_mut(j + 1, j + 1, M - j - 1, N - j - 1);
                let v1 = A.subcolumn(j, j + 1, M - j - 1);
                let v2 = A.subrow(j, j + 1, N - j - 1);

                zgeru(GSL_COMPLEX_NEGONE, &v1, &v2, &mut A22)?;
            }
        }

        Ok(())
    }
}

fn LU_decomp_L3(A: &mut GslMatrixComplex, ipiv: &mut GslVectorUint) -> Result<(), &'static str> {
    let M = A.size1;
    let N = A.size2;

    if M < N {
        return Err("matrix must have M >= N");
    } else if ipiv.size != min(M, N) {
        return Err("ipiv length must equal MIN(M,N)");
    } else if N <= CROSSOVER_LU {
        return LU_decomp_L2(A, ipiv);
    } else {
        let N1 = N / 2;
        let N2 = N - N1;
        let M2 = M - N1;

        let mut A11 = A.submatrix_mut(0, 0, N1, N1);
        let mut A12 = A.submatrix_mut(0, N1, N1, N2);
        let mut A21 = A.submatrix_mut(N1, 0, M2, N1);
        let mut A22 = A.submatrix_mut(N1, N1, M2, N2);

        let mut AL = A.submatrix_mut(0, 0, M, N1);
        let mut AR = A.submatrix_mut(0, N1, M, N2);

        let mut ipiv1 = ipiv.subvector(0, N1);
        let mut ipiv2 = ipiv.subvector(N1, N2);

        LU_decomp_L3(&mut AL, &mut ipiv1)?;
        apply_pivots(&mut AR, &ipiv1)?;
        ztrsm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            GSL_COMPLEX_ONE,
            &A11,
            &mut A12,
        )?;
        zgemm(
            CblasNoTrans,
            CblasNoTrans,
            GSL_COMPLEX_NEGONE,
            &A21,
            &A12,
            GSL_COMPLEX_ONE,
            &mut A22,
        )?;
        LU_decomp_L3(&mut A22, &mut ipiv2)?;
        apply_pivots(&mut A21, &ipiv2)?;

        for i in 0..N2 {
            let ptr = ipiv2.ptr(i);
            *ptr += N1;
        }

        Ok(())
    }
}

fn gsl_linalg_complex_LU_solve(
    LU: &GslMatrixComplex,
    p: &GslPermutation,
    b: &GslVectorComplex,
    x: &mut GslVectorComplex,
) -> Result<(), &'static str> {
    if LU.size1 != LU.size2 {
        return Err("LU matrix must be square");
    } else if LU.size1 != p.size {
        return Err("permutation length must match matrix size");
    } else if LU.size1 != b.size {
        return Err("matrix size must match b size");
    } else if LU.size2 != x.size {
        return Err("matrix size must match solution size");
    } else if singular(LU) {
        return Err("matrix is singular");
    } else {
        x.memcpy(b)?;
        gsl_linalg_complex_LU_svx(LU, p, x)
    }
}

fn gsl_linalg_complex_LU_svx(
    LU: &GslMatrixComplex,
    p: &GslPermutation,
    x: &mut GslVectorComplex,
) -> Result<(), &'static str> {
    if LU.size1 != LU.size2 {
        return Err("LU matrix must be square");
    } else if LU.size1 != p.size {
        return Err("permutation length must match matrix size");
    } else if LU.size1 != x.size {
        return Err("matrix size must match solution/rhs size");
    } else if singular(LU) {
        return Err("matrix is singular");
    } else {
        permute_vector_complex(p, x);
        ztrsv(CblasLower, CblasNoTrans, CblasUnit, LU, x)?;
        ztrsv(CblasUpper, CblasNoTrans, CblasNonUnit, LU, x)?;
        Ok(())
    }
}

fn gsl_linalg_complex_LU_refine(
    A: &GslMatrixComplex,
    LU: &GslMatrixComplex,
    p: &GslPermutation,
    b: &GslVectorComplex,
    x: &mut GslVectorComplex,
    work: &mut GslVectorComplex,
) -> Result<(), &'static str> {
    if A.size1 != A.size2 {
        return Err("matrix a must be square");
    }
    if LU.size1 != LU.size2 {
        return Err("LU matrix must be square");
    } else if A.size1 != LU.size2 {
        return Err("LU matrix must be decomposition of a");
    } else if LU.size1 != p.size {
        return Err("permutation length must match matrix size");
    } else if LU.size1 != b.size {
        return Err("matrix size must match b size");
    } else if LU.size1 != x.size {
        return Err("matrix size must match solution size");
    } else if LU.size1 != work.size {
        return Err("matrix size must match workspace size");
    } else if singular(LU) {
        return Err("matrix is singular");
    } else {
        work.memcpy(b)?;
        zgemv(
            CblasNoTrans,
            GSL_COMPLEX_ONE,
            A,
            x,
            GSL_COMPLEX_NEGONE,
            work,
        )?;
        gsl_linalg_complex_LU_svx(LU, p, work)?;
        work.axpy(GSL_COMPLEX_NEGONE, x)?;
        Ok(())
    }
}

fn gsl_linalg_complex_LU_invert(
    LU: &GslMatrixComplex,
    p: &GslPermutation,
    inverse: &mut GslMatrixComplex,
) -> Result<(), &'static str> {
    if LU.size1 != LU.size2 {
        return Err("LU matrix must be square");
    } else if LU.size1 != p.size {
        return Err("permutation length must match matrix size");
    } else if inverse.size1 != LU.size1 || inverse.size2 != LU.size2 {
        return Err("inverse matrix must match LU matrix dimensions");
    } else {
        inverse.memcpy(LU)?;
        gsl_linalg_complex_LU_invx(inverse, p)
    }
}

fn gsl_linalg_complex_LU_invx(
    LU: &mut GslMatrixComplex,
    p: &GslPermutation,
) -> Result<(), &'static str> {
    if LU.size1 != LU.size2 {
        return Err("LU matrix must be square");
    } else if LU.size1 != p.size {
        return Err("permutation length must match matrix size");
    } else if singular(LU) {
        return Err("matrix is singular");
    } else {
        let N = LU.size1;
        ztri_invert(CblasUpper, CblasNonUnit, LU)?;
        ztri_invert(CblasLower, CblasUnit, LU)?;
        ztri_UL(LU)?;
        for i in 0..N {
            let mut v = LU.row_mut(i);
            permute_vector_complex_inverse(p, &mut v);
        }
        Ok(())
    }
}

fn gsl_linalg_complex_LU_det(LU: &GslMatrixComplex, signum: i32) -> Complex64 {
    let n = LU.size1;
    let mut det = Complex64::new(signum as f64, 0.0);
    for i in 0..n {
        let zi = LU.get(i, i);
        det = det * zi;
    }
    det
}

fn gsl_linalg_complex_LU_lndet(LU: &GslMatrixComplex) -> f64 {
    let n = LU.size1;
    let mut lndet = 0.0;
    for i in 0..n {
        let z = LU