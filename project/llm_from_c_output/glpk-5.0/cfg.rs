use std::collections::HashMap;
use std::ptr::null_mut;

struct DMP {
    // Simplified memory pool implementation
    // In a real implementation, this would manage memory allocations
}

impl DMP {
    fn create_pool() -> Self {
        DMP {}
    }

    fn delete_pool(&mut self) {
        // Clean up memory pool
    }

    fn talloc<T>(&mut self, _count: usize) -> *mut T {
        // Allocate memory from pool
        null_mut()
    }
}

struct CFGVLE {
    v: i32,
    next: *mut CFGVLE,
}

struct CFGCLE {
    vptr: *mut CFGVLE,
    next: *mut CFGCLE,
}

struct CFG {
    n: i32,
    pos: Vec<i32>,
    neg: Vec<i32>,
    pool: DMP,
    nv_max: i32,
    nv: i32,
    ref_: Vec<i32>,
    vptr: Vec<*mut CFGVLE>,
    cptr: Vec<*mut CFGCLE>,
}

impl CFG {
    fn create_graph(n: i32, nv_max: i32) -> Self {
        assert!(n >= 0);
        assert!(nv_max >= 0 && nv_max <= n + n);

        CFG {
            n,
            pos: vec![0; (n + 1) as usize],
            neg: vec![0; (n + 1) as usize],
            pool: DMP::create_pool(),
            nv_max,
            nv: 0,
            ref_: vec![0; (nv_max + 1) as usize],
            vptr: vec![null_mut(); (nv_max + 1) as usize],
            cptr: vec![null_mut(); (nv_max + 1) as usize],
        }
    }

    fn add_clique(&mut self, size: i32, ind: &[i32]) {
        assert!(size >= 2 && size <= self.nv_max);

        // Add new vertices to the conflict graph
        for &j in &ind[1..=size as usize] {
            if j > 0 {
                // Vertex corresponds to x[j]
                assert!(1 <= j && j <= self.n);
                if self.pos[j as usize] == 0 {
                    // No such vertex exists; add it
                    self.nv += 1;
                    let v = self.nv;
                    assert!(v <= self.nv_max);
                    self.pos[j as usize] = v;
                    self.ref_[v as usize] = j;
                    self.vptr[v as usize] = null_mut();
                    self.cptr[v as usize] = null_mut();

                    if self.neg[j as usize] != 0 {
                        // Now both vertices for x[j] and (1 - x[j]) exist
                        self.add_edge(v, self.neg[j as usize]);
                    }
                }
            } else {
                // Vertex corresponds to (1 - x[j])
                let j_abs = -j;
                assert!(1 <= j_abs && j_abs <= self.n);
                if self.neg[j_abs as usize] == 0 {
                    // No such vertex exists; add it
                    self.nv += 1;
                    let v = self.nv;
                    assert!(v <= self.nv_max);
                    self.neg[j_abs as usize] = v;
                    self.ref_[v as usize] = j_abs;
                    self.vptr[v as usize] = null_mut();
                    self.cptr[v as usize] = null_mut();

                    if self.pos[j_abs as usize] != 0 {
                        // Now both vertices for x[j] and (1 - x[j]) exist
                        self.add_edge(v, self.pos[j_abs as usize]);
                    }
                }
            }
        }

        // Add specified clique to the conflict graph
        if size == 2 {
            let v1 = if ind[1] > 0 {
                self.pos[ind[1] as usize]
            } else {
                self.neg[-ind[1] as usize]
            };
            let v2 = if ind[2] > 0 {
                self.pos[ind[2] as usize]
            } else {
                self.neg[-ind[2] as usize]
            };
            self.add_edge(v1, v2);
        } else {
            // Build list of clique vertices
            let mut vp = null_mut();
            for &j in &ind[1..=size as usize] {
                let vle = unsafe { &mut *self.pool.talloc::<CFGVLE>(1) };
                vle.v = if j > 0 {
                    self.pos[j as usize]
                } else {
                    self.neg[-j as usize]
                };
                vle.next = vp;
                vp = vle;
            }

            // Attach the clique to all its vertices
            for &j in &ind[1..=size as usize] {
                let cle = unsafe { &mut *self.pool.talloc::<CFGCLE>(1) };
                cle.vptr = vp;
                let v = if j > 0 {
                    self.pos[j as usize]
                } else {
                    self.neg[-j as usize]
                };
                cle.next = self.cptr[v as usize];
                self.cptr[v as usize] = cle;
            }
        }
    }

    fn add_edge(&mut self, v: i32, w: i32) {
        assert!(1 <= v && v <= self.nv);
        assert!(1 <= w && w <= self.nv);
        assert!(v != w);

        // Add edge from v to w
        let vle = unsafe { &mut *self.pool.talloc::<CFGVLE>(1) };
        vle.v = w;
        vle.next = self.vptr[v as usize];
        self.vptr[v as usize] = vle;

        // Add edge from w to v
        let vle = unsafe { &mut *self.pool.talloc::<CFGVLE>(1) };
        vle.v = v;
        vle.next = self.vptr[w as usize];
        self.vptr[w as usize] = vle;
    }

    fn get_adjacent(&self, v: i32, ind: &mut [i32]) -> i32 {
        assert!(1 <= v && v <= self.nv);
        let mut len = 0;
        let mut ref_ = self.ref_.clone();

        // Walk through the list of adjacent vertices
        let mut vle = self.vptr[v as usize];
        while !vle.is_null() {
            let w = unsafe { (*vle).v };
            assert!(1 <= w && w <= self.nv);
            assert!(w != v);
            if ref_[w as usize] > 0 {
                len += 1;
                ind[len as usize] = w;
                ref_[w as usize] = -ref_[w as usize];
            }
            vle = unsafe { (*vle).next };
        }

        // Walk through the list of incident cliques
        let mut cle = self.cptr[v as usize];
        while !cle.is_null() {
            // Walk through the list of clique vertices
            let mut vle = unsafe { (*cle).vptr };
            while !vle.is_null() {
                let w = unsafe { (*vle).v };
                assert!(1 <= w && w <= self.nv);
                if w != v && ref_[w as usize] > 0 {
                    len += 1;
                    ind[len as usize] = w;
                    ref_[w as usize] = -ref_[w as usize];
                }
                vle = unsafe { (*vle).next };
            }
            cle = unsafe { (*cle).next };
        }

        assert!(1 <= len && len < self.nv);
        // Unmark vertices included in the resultant adjacency list
        for k in 1..=len {
            let w = ind[k as usize];
            ref_[w as usize] = -ref_[w as usize];
        }

        len
    }

    fn expand_clique(&self, c_len: i32, c_ind: &mut [i32]) -> i32 {
        let mut c_len = c_len;
        assert!(0 <= c_len && c_len <= self.nv);

        let mut d_ind = vec![0; (self.nv + 1) as usize];
        let mut d_pos = vec![0; (self.nv + 1) as usize];
        let mut ind = vec![0; (self.nv + 1) as usize];

        // Initialize C := 0, D := V
        let mut d_len = self.nv;
        for k in 1..=self.nv {
            d_ind[k as usize] = k;
            d_pos[k as usize] = k;
        }

        // Expand C by vertices of specified initial clique C0
        for k in 1..=c_len {
            // v in C0
            let v = c_ind[k as usize];
            assert!(1 <= v && v <= self.nv);
            // Since C0 is clique, v should be in D
            assert!(d_pos[v as usize] != 0);

            // W := set of vertices adjacent to v
            let len = self.get_adjacent(v, &mut ind);
            // D := D inter W
            d_len = Self::intersection(d_len, &mut d_ind, &mut d_pos, len, &ind);
            // Since v not in W, now v should be not in D
            assert!(d_pos[v as usize] == 0);
        }

        // Expand C by some other vertices until D is empty
        while d_len > 0 {
            // v in D
            let v = d_ind[1];
            assert!(1 <= v && v <= self.nv);
            // Note that v is adjacent to all vertices in C (by design),
            // so add v to C
            c_len += 1;
            c_ind[c_len as usize] = v;

            // W := set of vertices adjacent to v
            let len = self.get_adjacent(v, &mut ind);
            // D := D inter W
            d_len = Self::intersection(d_len, &mut d_ind, &mut d_pos, len, &ind);
            // Since v not in W, now v should be not in D
            assert!(d_pos[v as usize] == 0);
        }

        c_len
    }

    fn intersection(
        d_len: i32,
        d_ind: &mut [i32],
        d_pos: &mut [i32],
        len: i32,
        ind: &[i32],
    ) -> i32 {
        // Compute intersection D := D inter W
        let mut new_len = 0;

        // Walk through vertices in W and mark vertices in D
        for t in 1..=len {
            let v = ind[t as usize];
            // Determine position of v in D
            let k = d_pos[v as usize];
            if k != 0 {
                // v in D
                assert!(d_ind[k as usize] == v);
                // Mark v to keep it in D
                d_ind[k as usize] = -v;
            }
        }

        // Remove all unmarked vertices from D
        for k in 1..=d_len {
            let v = d_ind[k as usize];
            if v < 0 {
                // v is marked; keep it
                let v = -v;
                new_len += 1;
                d_ind[new_len as usize] = v;
                d_pos[v as usize] = new_len;
            } else {
                // v is not marked; remove it
                d_pos[v as usize] = 0;
            }
        }

        new_len
    }

    fn check_clique(&self, c_len: i32, c_ind: &[i32]) {
        let mut ind = vec![0; (self.nv + 1) as usize];
        let mut flag = vec![false; (self.nv + 1) as usize];

        // Walk through clique vertices
        assert!(c_len >= 0);
        for k in 1..=c_len {
            // Get clique vertex v
            let v = c_ind[k as usize];
            assert!(1 <= v && v <= self.nv);

            // Get vertices adjacent to vertex v
            let len = self.get_adjacent(v, &mut ind);
            for kk in 1..=len {
                let w = ind[kk as usize];
                assert!(1 <= w && w <= self.nv);
                assert!(w != v);
                flag[w as usize] = true;
            }

            // Check that all clique vertices other than v are adjacent to v
            for kk in 1..=c_len {
                let w = c_ind[kk as usize];
                assert!(1 <= w && w <= self.nv);
                if w != v {
                    assert!(flag[w as usize]);
                }
            }

            // Reset vertex flags
            for kk in 1..=len {
                flag[ind[kk as usize] as usize] = false;
            }
        }
    }

    fn delete_graph(&mut self) {
        self.pool.delete_pool();
    }
}