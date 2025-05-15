use std::ptr;
use std::mem;
use std::cmp;
use std::fmt;

#[derive(Debug)]
pub struct SVA {
    n_max: usize,
    n: usize,
    ptr: Vec<usize>,
    len: Vec<usize>,
    cap: Vec<usize>,
    size: usize,
    m_ptr: usize,
    r_ptr: usize,
    head: usize,
    tail: usize,
    prev: Vec<isize>,
    next: Vec<isize>,
    ind: Vec<usize>,
    val: Vec<f64>,
    talky: bool,
}

impl SVA {
    pub fn create_area(n_max: usize, size: usize) -> Self {
        assert!(n_max > 0 && n_max < isize::MAX as usize);
        assert!(size > 0 && size < isize::MAX as usize);

        SVA {
            n_max,
            n: 0,
            ptr: vec![0; n_max + 1],
            len: vec![0; n_max + 1],
            cap: vec![0; n_max + 1],
            size,
            m_ptr: 1,
            r_ptr: size + 1,
            head: 0,
            tail: 0,
            prev: vec![-1; n_max + 1],
            next: vec![-1; n_max + 1],
            ind: vec![0; size + 1],
            val: vec![0.0; size + 1],
            talky: false,
        }
    }

    pub fn alloc_vecs(&mut self, nnn: usize) -> usize {
        assert!(nnn > 0);
        if self.talky {
            println!("sva_alloc_vecs: nnn = {}", nnn);
        }

        let new_n = self.n + nnn;
        assert!(new_n > self.n);

        if self.n_max < new_n {
            let mut new_n_max = self.n_max;
            while new_n_max < new_n {
                new_n_max += new_n_max;
                assert!(new_n_max > 0);
            }

            self.n_max = new_n_max;
            self.ptr.resize(new_n_max + 1, 0);
            self.len.resize(new_n_max + 1, 0);
            self.cap.resize(new_n_max + 1, 0);
            self.prev.resize(new_n_max + 1, -1);
            self.next.resize(new_n_max + 1, -1);
        }

        for k in (self.n + 1)..=new_n {
            self.ptr[k] = 0;
            self.len[k] = 0;
            self.cap[k] = 0;
            self.prev[k] = -1;
            self.next[k] = -1;
        }

        self.n = new_n;

        if self.talky {
            println!("now sva.n_max = {}, sva.n = {}", self.n_max, self.n);
        }

        self.n - nnn + 1
    }

    pub fn resize_area(&mut self, delta: isize) {
        assert!(delta != 0);
        if self.talky {
            println!("sva_resize_area: delta = {}", delta);
        }

        let r_size = self.size - self.r_ptr + 1;

        if delta < 0 {
            assert!(delta >= (self.m_ptr as isize - self.r_ptr as isize));
            self.r_ptr = (self.r_ptr as isize + delta) as usize;
            self.ind.copy_within(self.r_ptr..self.r_ptr + r_size, self.r_ptr);
            self.val.copy_within(self.r_ptr..self.r_ptr + r_size, self.r_ptr);
        }

        self.size = (self.size as isize + delta) as usize;
        self.ind.resize(self.size + 1, 0);
        self.val.resize(self.size + 1, 0.0);

        if delta > 0 {
            self.r_ptr = (self.r_ptr as isize + delta) as usize;
            self.ind.copy_within(self.r_ptr - delta..self.r_ptr - delta + r_size, self.r_ptr);
            self.val.copy_within(self.r_ptr - delta..self.r_ptr - delta + r_size, self.r_ptr);
        }

        for k in 1..=self.n {
            if self.ptr[k] >= self.r_ptr {
                self.ptr[k] = (self.ptr[k] as isize + delta) as usize;
            }
        }

        if self.talky {
            println!("now sva.size = {}", self.size);
        }
    }

    pub fn defrag_area(&mut self) {
        if self.talky {
            println!("sva_defrag_area:");
            println!(
                "before defragmenting = {} {} {}",
                self.m_ptr - 1,
                self.r_ptr - self.m_ptr,
                self.size + 1 - self.r_ptr
            );
        }

        let mut m_ptr = 1;
        let mut head = 0;
        let mut tail = 0;
        let mut k = self.head;

        while k != 0 {
            let next_k = self.next[k] as usize;
            let len_k = self.len[k];

            if len_k == 0 {
                self.ptr[k] = 0;
                self.cap[k] = 0;
                self.prev[k] = -1;
                self.next[k] = -1;
            } else {
                let ptr_k = self.ptr[k];
                assert!(m_ptr <= ptr_k);

                if m_ptr < ptr_k {
                    self.ind.copy_within(ptr_k..ptr_k + len_k, m_ptr);
                    self.val.copy_within(ptr_k..ptr_k + len_k, m_ptr);
                    self.ptr[k] = m_ptr;
                }

                self.cap[k] = len_k;
                m_ptr += len_k;

                self.prev[k] = tail as isize;
                self.next[k] = 0;

                if head == 0 {
                    head = k;
                } else {
                    self.next[tail] = k as isize;
                }
                tail = k;
            }

            k = next_k;
        }

        assert!(m_ptr <= self.r_ptr);
        self.m_ptr = m_ptr;
        self.head = head;
        self.tail = tail;

        if self.talky {
            println!(
                "after defragmenting = {} {} {}",
                self.m_ptr - 1,
                self.r_ptr - self.m_ptr,
                self.size + 1 - self.r_ptr
            );
        }
    }

    pub fn more_space(&mut self, m_size: usize) {
        if self.talky {
            println!("sva_more_space: m_size = {}", m_size);
        }

        assert!(m_size > self.r_ptr - self.m_ptr);
        self.defrag_area();

        let m_size = cmp::max(m_size, self.m_ptr - 1);

        if self.r_ptr - self.m_ptr < m_size {
            let mut size = self.size;
            loop {
                let delta = size - self.size;
                if (self.r_ptr - self.m_ptr + delta) >= m_size {
                    break;
                }
                size += size;
                assert!(size > 0);
            }
            self.resize_area((size - self.size) as isize);
            assert!(self.r_ptr - self.m_ptr >= m_size);
        }
    }

    pub fn enlarge_cap(&mut self, k: usize, new_cap: usize, skip: bool) {
        assert!(1 <= k && k <= self.n);
        assert!(new_cap > self.cap[k]);
        assert!(self.r_ptr - self.m_ptr >= new_cap);

        if self.cap[k] == 0 {
            assert!(self.ptr[k] == 0);
            assert!(self.len[k] == 0);
        } else {
            assert!(self.ptr[k] + self.len[k] <= self.m_ptr);

            if self.len[k] > 0 {
                self.ind.copy_within(
                    self.ptr[k]..self.ptr[k] + self.len[k],
                    self.m_ptr,
                );
                if !skip {
                    self.val.copy_within(
                        self.ptr[k]..self.ptr[k] + self.len[k],
                        self.m_ptr,
                    );
                }
            }

            if self.prev[k] == 0 {
                self.head = self.next[k] as usize;
            } else {
                self.cap[self.prev[k] as usize] += self.cap[k];
                self.next[self.prev[k] as usize] = self.next[k];
            }

            if self.next[k] == 0 {
                self.tail = self.prev[k] as usize;
            } else {
                self.prev[self.next[k] as usize] = self.prev[k];
            }
        }

        self.ptr[k] = self.m_ptr;
        self.cap[k] = new_cap;

        self.prev[k] = self.tail as isize;
        self.next[k] = 0;

        if self.head == 0 {
            self.head = k;
        } else {
            self.next[self.tail] = k as isize;
        }
        self.tail = k;

        self.m_ptr += new_cap;
        assert!(self.m_ptr <= self.r_ptr);
    }

    pub fn reserve_cap(&mut self, k: usize, new_cap: usize) {
        assert!(1 <= k && k <= self.n);
        assert!(new_cap > 0);
        assert!(self.ptr[k] == 0 && self.len[k] == 0 && self.cap[k] == 0);
        assert!(self.r_ptr - self.m_ptr >= new_cap);

        self.ptr[k] = self.r_ptr - new_cap;
        self.cap[k] = new_cap;
        self.r_ptr -= new_cap;
    }

    pub fn make_static(&mut self, k: usize) {
        assert!(1 <= k && k <= self.n);

        if self.cap[k] == 0 {
            assert!(self.ptr[k] == 0);
            assert!(self.len[k] == 0);
            return;
        }

        let len_k = self.len[k];
        assert!(self.r_ptr - self.m_ptr >= len_k);

        if self.prev[k] == 0 {
            self.head = self.next[k] as usize;
        } else {
            self.cap[self.prev[k] as usize] += self.cap[k];
            self.next[self.prev[k] as usize] = self.next[k];
        }

        if self.next[k] == 0 {
            self.tail = self.prev[k] as usize;
        } else {
            self.prev[self.next[k] as usize] = self.prev[k];
        }

        if len_k == 0 {
            self.ptr[k] = 0;
            self.cap[k] = 0;
            return;
        }

        let ptr_k = self.r_ptr - len_k;
        self.ind.copy_within(self.ptr[k]..self.ptr[k] + len_k, ptr_k);
        self.val.copy_within(self.ptr[k]..self.ptr[k] + len_k, ptr_k);

        self.ptr[k] = ptr_k;
        self.cap[k] = len_k;
        self.r_ptr -= len_k;
    }

    pub fn check_area(&self) {
        assert!(self.n <= self.n_max);
        assert!(1 <= self.m_ptr && self.m_ptr <= self.r_ptr && self.r_ptr <= self.size + 1);

        let mut k = self.head;
        while k != 0 {
            assert!(1 <= k && k <= self.n);
            assert!(self.cap[k] > 0);
            assert!(self.len[k] <= self.cap[k]);

            if self.prev[k] == 0 {
                assert!(k == self.head);
            } else {
                assert!(1 <= self.prev[k] as usize && self.prev[k] as usize <= self.n);
                assert!(self.next[self.prev[k] as usize == k);
            }

            if self.next[k] == 0 {
                assert!(k == self.tail);
                assert!(self.ptr[k] + self.cap[k] <= self.m_ptr);
            } else {
                assert!(1 <= self.next[k] as usize && self.next[k] as usize <= self.n);
                assert!(self.prev[self.next[k] as usize == k as isize);
                assert!(self.ptr[k] + self.cap[k] <= self.ptr[self.next[k] as usize]);
            }

            k = self.next[k] as usize;
        }

        for k in 1..=self.n {
            if self.cap[k] == 0 {
                assert!(self.ptr[k] == 0);
                assert!(self.len[k] == 0);
            } else if self.ptr[k] >= self.r_ptr {
                assert!(self.len[k] <= self.cap[k]);
                assert!(self.r_ptr <= self.ptr[k] && self.ptr[k] + self.cap[k] <= self.size + 1);
            }
        }
    }
}

impl Drop for SVA {
    fn drop(&mut self) {
        // Rust's Vec will automatically deallocate when SVA is dropped
    }
}