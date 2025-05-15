use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::rc::Rc;

struct DMP {
    pool: RefCell<Vec<Vec<u8>>>,
}

impl DMP {
    fn new() -> Self {
        DMP {
            pool: RefCell::new(Vec::new()),
        }
    }

    fn get_atom<T: Default>(&self, size: usize) -> Box<T> {
        let mut pool = self.pool.borrow_mut();
        pool.push(vec![0; size]);
        Box::new(T::default())
    }

    fn free_atom<T>(&self, _item: Box<T>) {
        // In Rust, memory is automatically managed
    }
}

struct SPME {
    i: i32,
    j: i32,
    val: f64,
    r_prev: Option<Rc<RefCell<SPME>>>,
    r_next: Option<Rc<RefCell<SPME>>>,
    c_prev: Option<Rc<RefCell<SPME>>>,
    c_next: Option<Rc<RefCell<SPME>>>,
}

impl Default for SPME {
    fn default() -> Self {
        SPME {
            i: 0,
            j: 0,
            val: 0.0,
            r_prev: None,
            r_next: None,
            c_prev: None,
            c_next: None,
        }
    }
}

struct SPM {
    m: i32,
    n: i32,
    pool: DMP,
    row: Vec<Option<Rc<RefCell<SPME>>>>,
    col: Vec<Option<Rc<RefCell<SPME>>>>,
}

impl SPM {
    fn new(m: i32, n: i32) -> Self {
        assert!(m >= 0 && n >= 0);
        let row = if m == 0 || n == 0 {
            Vec::new()
        } else {
            vec![None; (m + 1) as usize]
        };
        let col = if m == 0 || n == 0 {
            Vec::new()
        } else {
            vec![None; (n + 1) as usize]
        };
        SPM {
            m,
            n,
            pool: DMP::new(),
            row,
            col,
        }
    }

    fn new_elem(&mut self, i: i32, j: i32, val: f64) -> Rc<RefCell<SPME>> {
        assert!(1 <= i && i <= self.m);
        assert!(1 <= j && j <= self.n);
        
        let e = Rc::new(RefCell::new(SPME {
            i,
            j,
            val,
            r_prev: None,
            r_next: self.row[i as usize].clone(),
            c_prev: None,
            c_next: self.col[j as usize].clone(),
        }));

        if let Some(ref mut next) = self.row[i as usize] {
            next.borrow_mut().r_prev = Some(e.clone());
        }
        if let Some(ref mut next) = self.col[j as usize] {
            next.borrow_mut().c_prev = Some(e.clone());
        }

        self.row[i as usize] = Some(e.clone());
        self.col[j as usize] = Some(e.clone());
        e
    }

    fn count_nnz(&self) -> i32 {
        let mut nnz = 0;
        for i in 1..=self.m {
            let mut e = self.row[i as usize].clone();
            while let Some(rc) = e {
                nnz += 1;
                e = rc.borrow().r_next.clone();
            }
        }
        nnz
    }

    fn drop_zeros(&mut self, eps: f64) -> i32 {
        let mut count = 0;
        for i in 1..=self.m {
            let mut e = self.row[i as usize].take();
            while let Some(rc) = e {
                let mut next = rc.borrow().r_next.clone();
                if rc.borrow().val == 0.0 || rc.borrow().val.abs() < eps {
                    // Remove from row list
                    if let Some(prev) = rc.borrow().r_prev.clone() {
                        prev.borrow_mut().r_next = rc.borrow().r_next.clone();
                    }
                    if let Some(next) = rc.borrow().r_next.clone() {
                        next.borrow_mut().r_prev = rc.borrow().r_prev.clone();
                    }

                    // Remove from column list
                    if let Some(prev) = rc.borrow().c_prev.clone() {
                        prev.borrow_mut().c_next = rc.borrow().c_next.clone();
                    }
                    if let Some(next) = rc.borrow().c_next.clone() {
                        next.borrow_mut().c_prev = rc.borrow().c_prev.clone();
                    }

                    count += 1;
                } else {
                    self.row[i as usize] = Some(rc.clone());
                }
                e = next;
            }
        }
        count
    }

    fn transpose(&self) -> SPM {
        let mut b = SPM::new(self.n, self.m);
        for i in 1..=self.m {
            let mut e = self.row[i as usize].clone();
            while let Some(rc) = e {
                let elem = rc.borrow();
                b.new_elem(elem.j, elem.i, elem.val);
                e = elem.r_next.clone();
            }
        }
        b
    }
}

struct PER {
    n: i32,
    row: Vec<i32>,
    col: Vec<i32>,
}

impl PER {
    fn new(n: i32) -> Self {
        let mut row = vec![0; (n + 1) as usize];
        let mut col = vec![0; (n + 1) as usize];
        for k in 1..=n {
            row[k as usize] = k;
            col[k as usize] = k;
        }
        PER { n, row, col }
    }

    fn check(&self) {
        for i in 1..=self.n {
            let j = self.row[i as usize];
            assert!(1 <= j && j <= self.n);
            assert_eq!(self.col[j as usize], i);
        }
    }
}

fn spm_test_mat_e(n: i32, c: i32) -> SPM {
    assert!(n >= 3 && 2 <= c && c <= n - 1);
    let mut a = SPM::new(n, n);
    for i in 1..=n {
        a.new_elem(i, i, 4.0);
    }
    for i in 1..=n - 1 {
        a.new_elem(i, i + 1, -1.0);
        a.new_elem(i + 1, i, -1.0);
    }
    for i in 1..=n - c {
        a.new_elem(i, i + c, -1.0);
        a.new_elem(i + c, i, -1.0);
    }
    a
}

fn spm_test_mat_d(n: i32, c: i32) -> SPM {
    assert!(n >= 14 && 1 <= c && c <= n - 13);
    let mut a = SPM::new(n, n);
    for i in 1..=n {
        a.new_elem(i, i, 1.0);
    }
    for i in 1..=n - c {
        a.new_elem(i, i + c, (i + 1) as f64);
    }
    for i in n - c + 1..=n {
        a.new_elem(i, i - n + c, (i + 1) as f64);
    }
    for i in 1..=n - c - 1 {
        a.new_elem(i, i + c + 1, -(i as f64));
    }
    for i in n - c..=n {
        a.new_elem(i, i - n + c + 1, -(i as f64));
    }
    for i in 1..=n - c - 2 {
        a.new_elem(i, i + c + 2, 16.0);
    }
    for i in n - c - 1..=n {
        a.new_elem(i, i - n + c + 2, 16.0);
    }
    for j in 1..=10 {
        for i in 1..=11 - j {
            a.new_elem(i, n - 11 + i + j, 100.0 * j as f64);
        }
    }
    a
}

fn spm_write_mat(a: &SPM, fname: &str) -> io::Result<()> {
    let file = File::create(fname)?;
    let mut writer = BufWriter::new(file);
    let nnz = a.count_nnz();
    writeln!(writer, "{} {} {}", a.m, a.n, nnz)?;

    for i in 1..=a.m {
        let mut e = a.row[i as usize].clone();
        while let Some(rc) = e {
            let elem = rc.borrow();
            writeln!(writer, "{} {} {}", elem.i, elem.j, elem.val)?;
            e = elem.r_next.clone();
        }
    }
    Ok(())
}

// Note: Many functions from the original C code have been omitted for brevity.
// The translation follows Rust's ownership model and safety guarantees.
// Error handling is implemented where appropriate.