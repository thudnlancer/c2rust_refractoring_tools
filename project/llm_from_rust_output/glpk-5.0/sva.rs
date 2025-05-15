use std::ptr;
use std::mem;
use std::cmp;
use std::fmt;

#[derive(Debug)]
pub struct SVA {
    n_max: i32,
    n: i32,
    ptr: Vec<i32>,
    len: Vec<i32>,
    cap: Vec<i32>,
    size: i32,
    m_ptr: i32,
    r_ptr: i32,
    head: i32,
    tail: i32,
    prev: Vec<i32>,
    next: Vec<i32>,
    ind: Vec<i32>,
    val: Vec<f64>,
    talky: i32,
}

impl SVA {
    pub fn create_area(n_max: i32, size: i32) -> Self {
        assert!(0 < n_max && n_max < i32::MAX, "0 < n_max && n_max < INT_MAX");
        assert!(0 < size && size < i32::MAX, "0 < size && size < INT_MAX");

        SVA {
            n_max,
            n: 0,
            ptr: vec![0; (1 + n_max) as usize],
            len: vec![0; (1 + n_max) as usize],
            cap: vec![0; (1 + n_max) as usize],
            size,
            m_ptr: 1,
            r_ptr: size + 1,
            head: 0,
            tail: 0,
            prev: vec![0; (1 + n_max) as usize],
            next: vec![0; (1 + n_max) as usize],
            ind: vec![0; (1 + size) as usize],
            val: vec![0.0; (1 + size) as usize],
            talky: 0,
        }
    }

    pub fn alloc_vecs(&mut self, nnn: i32) -> i32 {
        if self.talky != 0 {
            println!("sva_alloc_vecs: nnn = {}", nnn);
        }
        assert!(nnn > 0, "nnn > 0");

        let new_n = self.n + nnn;
        assert!(new_n > self.n, "new_n > n");

        if self.n_max < new_n {
            let mut new_n_max = self.n_max;
            while new_n_max < new_n {
                new_n_max += new_n_max;
                assert!(new_n_max > 0, "n_max > 0");
            }

            self.n_max = new_n_max;
            self.ptr.resize((1 + new_n_max) as usize, 0);
            self.len.resize((1 + new_n_max) as usize, 0);
            self.cap.resize((1 + new_n_max) as usize, 0);
            self.prev.resize((1 + new_n_max) as usize, 0);
            self.next.resize((1 + new_n_max) as usize, 0);
        }

        self.n = new_n;
        for k in (self.n + 1)..=new_n {
            self.cap[k as usize] = 0;
            self.len[k as usize] = self.cap[k as usize];
            self.ptr[k as usize] = self.len[k as usize];
            self.next[k as usize] = -1;
            self.prev[k as usize] = self.next[k as usize];
        }

        if self.talky != 0 {
            println!("now sva->n_max = {}, sva->n = {}", self.n_max, self.n);
        }

        self.n + 1
    }

    pub fn resize_area(&mut self, delta: i32) {
        if self.talky != 0 {
            println!("sva_resize_area: delta = {}", delta);
        }
        assert!(delta != 0, "delta != 0");

        let r_size = self.size - self.r_ptr + 1;
        if delta < 0 {
            assert!(delta >= self.m_ptr - self.r_ptr, "delta >= m_ptr - r_ptr");
            self.r_ptr += delta;

            let src = self.r_ptr as usize;
            let dst = self.r_ptr as usize;
            self.ind.copy_within(src..src + r_size as usize, dst);
            self.val.copy_within(src..src + r_size as usize, dst);
        }

        assert!(delta < i32::MAX - self.size, "delta < INT_MAX - sva->size");
        self.size += delta;
        self.ind.resize((1 + self.size) as usize, 0);
        self.val.resize((1 + self.size) as usize, 0.0);

        if delta > 0 {
            self.r_ptr += delta;
            let src = self.r_ptr as usize;
            let dst = self.r_ptr as usize;
            self.ind.copy_within(src..src + r_size as usize, dst);
            self.val.copy_within(src..src + r_size as usize, dst);
        }

        for k in 1..=self.n {
            if self.ptr[k as usize] >= self.r_ptr {
                self.ptr[k as usize] += delta;
            }
        }

        if self.talky != 0 {
            println!("now sva->size = {}", self.size);
        }
    }

    pub fn defrag_area(&mut self) {
        if self.talky != 0 {
            println!("sva_defrag_area:");
            println!(
                "before defragmenting = {} {} {}",
                self.m_ptr - 1,
                self.r_ptr - self.m_ptr,
                self.size + 1 - self.r_ptr
            );
        }

        let mut m_ptr = 1;
        let mut tail = 0;
        let mut head = tail;
        let mut k = self.head;

        while k != 0 {
            let next_k = self.next[k as usize];
            let len_k = self.len[k as usize];

            if len_k == 0 {
                self.cap[k as usize] = 0;
                self.ptr[k as usize] = self.cap[k as usize];
                self.next[k as usize] = -1;
                self.prev[k as usize] = self.next[k as usize];
            } else {
                let ptr_k = self.ptr[k as usize];
                assert!(m_ptr <= ptr_k, "m_ptr <= ptr_k");

                if m_ptr < ptr_k {
                    self.ind.copy_within(
                        ptr_k as usize..(ptr_k + len_k) as usize,
                        m_ptr as usize,
                    );
                    self.val.copy_within(
                        ptr_k as usize..(ptr_k + len_k) as usize,
                        m_ptr as usize,
                    );
                    self.ptr[k as usize] = m_ptr;
                }

                self.cap[k as usize] = len_k;
                m_ptr += len_k;
                self.prev[k as usize] = tail;
                self.next[k as usize] = 0;

                if head == 0 {
                    head = k;
                } else {
                    self.next[tail as usize] = k;
                }
                tail = k;
            }
            k = next_k;
        }

        assert!(m_ptr <= self.r_ptr, "m_ptr <= sva->r_ptr");
        self.m_ptr = m_ptr;
        self.head = head;
        self.tail = tail;

        if self.talky != 0 {
            println!(
                "after defragmenting = {} {} {}",
                self.m_ptr - 1,
                self.r_ptr - self.m_ptr,
                self.size + 1 - self.r_ptr
            );
        }
    }

    pub fn more_space(&mut self, m_size: i32) {
        if self.talky != 0 {
            println!("sva_more_space: m_size = {}", m_size);
        }
        assert!(
            m_size > self.r_ptr - self.m_ptr,
            "m_size > sva->r_ptr - sva->m_ptr"
        );

        self.defrag_area();
        let m_size = if m_size < self.m_ptr - 1 {
            self.m_ptr - 1
        } else {
            m_size
        };

        if self.r_ptr - self.m_ptr < m_size {
            let mut size = self.size;
            loop {
                let delta = size - self.size;
                if self.r_ptr - self.m_ptr + delta >= m_size {
                    break;
                }
                size += size;
                assert!(size > 0, "size > 0");
            }
            self.resize_area(size - self.size);
            assert!(
                self.r_ptr - self.m_ptr >= m_size,
                "sva->r_ptr - sva->m_ptr >= m_size"
            );
        }
    }

    pub fn enlarge_cap(&mut self, k: i32, new_cap: i32, skip: bool) {
        assert!(1 <= k && k <= self.n, "1 <= k && k <= sva->n");
        assert!(new_cap > self.cap[k as usize], "new_cap > cap[k]");
        assert!(
            self.r_ptr - self.m_ptr >= new_cap,
            "sva->r_ptr - sva->m_ptr >= new_cap"
        );

        if self.cap[k as usize] == 0 {
            assert!(self.ptr[k as usize] == 0, "ptr[k] == 0");
            assert!(self.len[k as usize] == 0, "len[k] == 0");
        } else {
            assert!(
                self.ptr[k as usize] + self.len[k as usize] <= self.m_ptr,
                "ptr[k] + len[k] <= sva->m_ptr"
            );

            if self.len[k as usize] > 0 {
                let src = self.ptr[k as usize] as usize;
                let dst = self.m_ptr as usize;
                let len = self.len[k as usize] as usize;

                self.ind.copy_within(src..src + len, dst);
                if !skip {
                    self.val.copy_within(src..src + len, dst);
                }
            }

            if self.prev[k as usize] == 0 {
                self.head = self.next[k as usize];
            } else {
                self.cap[self.prev[k as usize] as usize] += self.cap[k as usize];
                self.next[self.prev[k as usize] as usize] = self.next[k as usize];
            }

            if self.next[k as usize] == 0 {
                self.tail = self.prev[k as usize];
            } else {
                self.prev[self.next[k as usize] as usize] = self.prev[k as usize];
            }
        }

        self.ptr[k as usize] = self.m_ptr;
        self.cap[k as usize] = new_cap;
        self.prev[k as usize] = self.tail;
        self.next[k as usize] = 0;

        if self.head == 0 {
            self.head = k;
        } else {
            self.next[self.tail as usize] = k;
        }

        self.tail = k;
        self.m_ptr += new_cap;
        assert!(self.m_ptr <= self.r_ptr, "sva->m_ptr <= sva->r_ptr");
    }

    pub fn reserve_cap(&mut self, k: i32, new_cap: i32) {
        assert!(1 <= k && k <= self.n, "1 <= k && k <= sva->n");
        assert!(new_cap > 0, "new_cap > 0");
        assert!(
            self.ptr[k as usize] == 0 && self.len[k as usize] == 0 && self.cap[k as usize] == 0,
            "ptr[k] == 0 && len[k] == 0 && cap[k] == 0"
        );
        assert!(
            self.r_ptr - self.m_ptr >= new_cap,
            "sva->r_ptr - sva->m_ptr >= new_cap"
        );

        self.ptr[k as usize] = self.r_ptr - new_cap;
        self.cap[k as usize] = new_cap;
        self.r_ptr -= new_cap;
    }

    pub fn make_static(&mut self, k: i32) {
        assert!(1 <= k && k <= self.n, "1 <= k && k <= sva->n");

        if self.cap[k as usize] == 0 {
            assert!(self.ptr[k as usize] == 0, "ptr[k] == 0");
            assert!(self.len[k as usize] == 0, "len[k] == 0");
        } else {
            let len_k = self.len[k as usize];
            assert!(
                self.r_ptr - self.m_ptr >= len_k,
                "sva->r_ptr - sva->m_ptr >= len_k"
            );

            if self.prev[k as usize] == 0 {
                self.head = self.next[k as usize];
            } else {
                self.cap[self.prev[k as usize] as usize] += self.cap[k as usize];
                self.next[self.prev[k as usize] as usize] = self.next[k as usize];
            }

            if self.next[k as usize] == 0 {
                self.tail = self.prev[k as usize];
            } else {
                self.prev[self.next[k as usize] as usize] = self.prev[k as usize];
            }

            if len_k == 0 {
                self.cap[k as usize] = 0;
                self.ptr[k as usize] = self.cap[k as usize];
            } else {
                let ptr_k = self.r_ptr - len_k;
                let src = self.ptr[k as usize] as usize;
                let dst = ptr_k as usize;

                self.ind.copy_within(src..src + len_k as usize, dst);
                self.val.copy_within(src..src + len_k as usize, dst);

                self.ptr[k as usize] = ptr_k;
                self.cap[k as usize] = len_k;
                self.r_ptr -= len_k;
            }
        }
    }

    pub fn check_area(&self) {
        assert!(0 <= self.n && self.n <= self.n_max, "0 <= n && n <= n_max");
        assert!(
            1 <= self.m_ptr && self.m_ptr <= self.r_ptr && self.r_ptr <= self.size + 1,
            "1 <= m_ptr && m_ptr <= r_ptr && r_ptr <= size+1"
        );

        let mut k = self.head;
        while k != 0 {
            assert!(1 <= k && k <= self.n, "1 <= k && k <= n");
            assert!(self.cap[k as usize] > 0, "cap[k] > 0");
            assert!(
                0 <= self.len[k as usize] && self.len[k as usize] <= self.cap[k as usize],
                "0 <= len[k] && len[k] <= cap[k]"
            );

            if self.prev[k as usize] == 0 {
                assert!(k == self.head, "k == head");
            } else {
                assert!(
                    1 <= self.prev[k as usize] && self.prev[k as usize] <= self.n,
                    "1 <= prev[k] && prev[k] <= n"
                );
                assert!(
                    self.next[self.prev[k as usize] as usize] == k,
                    "next[prev[k]] == k"
                );
            }

            if self.next[k as usize] == 0 {
                assert!(k == self.tail, "k == tail");
                assert!(
                    self.ptr[k as usize] + self.cap[k as usize] <= self.m_ptr,
                    "ptr[k] + cap[k] <= m_ptr"
                );
            } else {
                assert!(
                    1 <= self.next[k as usize] && self.next[k as usize] <= self.n,
                    "1 <= next[k] && next[k] <= n"
                );
                assert!(
                    self.prev[self.next[k as usize] as usize] == k,
                    "prev[next[k]] == k"
                );
                assert!(
                    self.ptr[k as usize] + self.cap[k as usize]
                        <= self.ptr[self.next[k as usize] as usize],
                    "ptr[k] + cap[k] <= ptr[next[k]]"
                );
            }

            k = self.next[k as usize];
        }

        for k in 1..=self.n {
            if self.cap[k as usize] < 0 {
                // Already checked in the linked list
            } else if self.cap[k as usize] == 0 {
                assert!(self.ptr[k as usize] == 0, "ptr[k] == 0");
                assert!(self.len[k as usize] == 0, "len[k] == 0");
            } else {
                assert!(
                    0 <= self.len[k as usize] && self.len[k as usize] <= self.cap[k as usize],
                    "0 <= len[k] && len[k] <= cap[k]"
                );
                assert!(
                    self.r_ptr <= self.ptr[k as usize]
                        && self.ptr[k as usize] + self.cap[k as usize] <= self.size + 1,
                    "r_ptr <= ptr[k] && ptr[k] + cap[k] <= size+1"
                );
            }
        }
    }
}