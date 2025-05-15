use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixMod {
    None,
    Transpose,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Result<f64, &'static str> {
        if row >= self.rows || col >= self.cols {
            Err("index out of bounds")
        } else {
            Ok(self.data[row][col])
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), &'static str> {
        if row >= self.rows || col >= self.cols {
            Err("index out of bounds")
        } else {
            self.data[row][col] = value;
            Ok(())
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

pub fn matmult(a: &Matrix, b: &Matrix, c: &mut Matrix) -> Result<(), &'static str> {
    if a.cols() != b.rows() || a.rows() != c.rows() || b.cols() != c.cols() {
        return Err("matrix sizes are not conformant");
    }

    for i in 0..c.rows() {
        for j in 0..c.cols() {
            let mut temp = a.get(i, 0)? * b.get(0, j)?;
            for k in 1..a.cols() {
                temp += a.get(i, k)? * b.get(k, j)?;
            }
            c.set(i, j, temp)?;
        }
    }

    Ok(())
}

pub fn matmult_mod(
    a: &Matrix,
    mod_a: MatrixMod,
    b: &Matrix,
    mod_b: MatrixMod,
    c: &mut Matrix,
) -> Result<(), &'static str> {
    if mod_a == MatrixMod::None && mod_b == MatrixMod::None {
        return matmult(a, b, c);
    }

    let mut dim1_a = a.rows();
    let mut dim2_a = a.cols();
    let mut dim1_b = b.rows();
    let mut dim2_b = b.cols();
    let dim1_c = c.rows();
    let dim2_c = c.cols();

    if mod_a == MatrixMod::Transpose {
        mem::swap(&mut dim1_a, &mut dim2_a);
    }
    if mod_b == MatrixMod::Transpose {
        mem::swap(&mut dim1_b, &mut dim2_b);
    }

    if dim2_a != dim1_b || dim1_a != dim1_c || dim2_b != dim2_c {
        return Err("matrix sizes are not conformant");
    }

    for i in 0..dim1_c {
        for j in 0..dim2_c {
            let (mut a1, mut a2) = (i, 0);
            let (mut b1, mut b2) = (0, j);

            if mod_a == MatrixMod::Transpose {
                mem::swap(&mut a1, &mut a2);
            }
            if mod_b == MatrixMod::Transpose {
                mem::swap(&mut b1, &mut b2);
            }

            let mut temp = a.get(a1, a2)? * b.get(b1, b2)?;

            for k in 1..dim2_a {
                let (mut a1, mut a2) = (i, k);
                let (mut b1, mut b2) = (k, j);

                if mod_a == MatrixMod::Transpose {
                    mem::swap(&mut a1, &mut a2);
                }
                if mod_b == MatrixMod::Transpose {
                    mem::swap(&mut b1, &mut b2);
                }

                temp += a.get(a1, a2)? * b.get(b1, b2)?;
            }

            c.set(i, j, temp)?;
        }
    }

    Ok(())
}