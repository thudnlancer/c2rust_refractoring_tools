use std::cmp::Ordering;

#[derive(Debug)]
pub struct Combination {
    n: usize,
    k: usize,
    data: Vec<usize>,
}

impl Combination {
    pub fn new(n: usize, k: usize) -> Option<Self> {
        if k > n {
            return None;
        }
        let mut data = Vec::with_capacity(k);
        for i in 0..k {
            data.push(i);
        }
        Some(Combination { n, k, data })
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn is_valid(&self) -> Result<(), &'static str> {
        if self.k > self.n {
            return Err("combination has k greater than n");
        }

        for i in 0..self.k {
            let ci = self.data[i];
            if ci >= self.n {
                return Err("combination index outside range");
            }

            for j in 0..i {
                if self.data[j] == ci {
                    return Err("duplicate combination index");
                }
                if self.data[j] > ci {
                    return Err("combination indices not in increasing order");
                }
            }
        }

        Ok(())
    }

    pub fn next(&mut self) -> Result<(), &'static str> {
        if self.k == 0 {
            return Err("no next combination");
        }

        let mut i = self.k - 1;

        while i > 0 && self.data[i] == self.n - self.k + i {
            i -= 1;
        }

        if i == 0 && self.data[i] == self.n - self.k {
            return Err("no next combination");
        }

        self.data[i] += 1;
        for j in i + 1..self.k {
            self.data[j] = self.data[j - 1] + 1;
        }

        Ok(())
    }

    pub fn prev(&mut self) -> Result<(), &'static str> {
        if self.k == 0 {
            return Err("no previous combination");
        }

        let mut i = self.k - 1;

        while i > 0 && self.data[i] == self.data[i - 1] + 1 {
            i -= 1;
        }

        if i == 0 && self.data[i] == 0 {
            return Err("no previous combination");
        }

        self.data[i] -= 1;
        for j in i + 1..self.k {
            self.data[j] = self.n - self.k + j;
        }

        Ok(())
    }

    pub fn copy_from(&mut self, src: &Combination) -> Result<(), &'static str> {
        if self.n != src.n || self.k != src.k {
            return Err("combination lengths are not equal");
        }

        self.data.copy_from_slice(&src.data);
        Ok(())
    }
}