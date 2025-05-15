use std::cmp::Ordering;

#[derive(Debug)]
pub struct Permutation {
    data: Vec<usize>,
    size: usize,
}

impl Permutation {
    pub fn new(size: usize) -> Self {
        Permutation {
            data: (0..size).collect(),
            size,
        }
    }

    pub fn linear_to_canonical(&self, q: &mut Permutation) -> Result<(), &'static str> {
        if q.size != self.size {
            return Err("size of q does not match size of p");
        }

        let n = self.size;
        let mut t = n;
        let pp = &self.data;
        let qq = &mut q.data;

        for i in 0..n {
            let mut k = pp[i];
            let mut s = 1;

            while k > i {
                k = pp[k];
                s += 1;
            }

            if k < i {
                continue;
            }

            t -= s;
            qq[t] = i;

            let mut k = pp[i];
            let mut s = 1;

            while k > i {
                qq[t + s] = k;
                k = pp[k];
                s += 1;
            }

            if t == 0 {
                break;
            }
        }

        Ok(())
    }

    pub fn canonical_to_linear(&self, p: &mut Permutation) -> Result<(), &'static str> {
        if p.size != self.size {
            return Err("size of p does not match size of q");
        }

        let n = self.size;
        let qq = &self.data;
        let pp = &mut p.data;

        for i in 0..n {
            pp[i] = i;
        }

        let mut k = qq[0];
        let mut first = pp[k];

        for i in 1..n {
            let kk = qq[i];

            if kk > first {
                pp[k] = pp[kk];
                k = kk;
            } else {
                pp[k] = first;
                k = kk;
                first = pp[kk];
            }
        }

        pp[k] = first;

        Ok(())
    }

    pub fn inversions(&self) -> usize {
        let mut count = 0;
        let size = self.size;

        for i in 0..size - 1 {
            for j in i + 1..size {
                if self.data[i] > self.data[j] {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn linear_cycles(&self) -> usize {
        let mut count = 0;
        let size = self.size;

        for i in 0..size {
            let mut k = self.data[i];

            while k > i {
                k = self.data[k];
            }

            if k < i {
                continue;
            }

            count += 1;
        }

        count
    }

    pub fn canonical_cycles(&self) -> usize {
        let mut count = 1;
        let mut min = self.data[0];

        for &item in self.data.iter() {
            match item.cmp(&min) {
                Ordering::Less => {
                    min = item;
                    count += 1;
                }
                _ => {}
            }
        }

        count
    }
}