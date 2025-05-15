use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

pub type SizeT = usize;
pub type MedAccType = f64;

#[derive(Clone)]
pub struct MedAccState {
    n: i32,
    idx: i32,
    ct: i32,
    data: Vec<MedAccType>,
    pos: Vec<i32>,
    heap: Vec<i32>,
}

impl MedAccState {
    fn new(n: SizeT) -> Self {
        let n_i32 = n as i32;
        let mut state = MedAccState {
            n: n_i32,
            idx: 0,
            ct: 0,
            data: vec![0.0; n],
            pos: vec![0; n],
            heap: vec![0; n],
        };

        for k in (0..n_i32).rev() {
            state.pos[k as usize] = (k + 1) / 2 * if k & 1 != 0 { -1 } else { 1 };
            state.heap[state.pos[k as usize].abs() as usize] = k;
        }

        state
    }

    fn insert(&mut self, x: MedAccType) -> GslError {
        let is_new = self.ct < self.n;
        let p = self.pos[self.idx as usize];
        let old = self.data[self.idx as usize];
        
        self.data[self.idx as usize] = x;
        self.idx = (self.idx + 1) % self.n;
        self.ct += is_new as i32;

        if p > 0 {
            if is_new == 0 && old < x {
                self.min_sort_down(p * 2);
            } else if self.min_sort_up(p) != 0 {
                self.max_sort_down(-1);
            }
        } else if p < 0 {
            if is_new == 0 && x < old {
                self.max_sort_down(p * 2);
            } else if self.max_sort_up(p) != 0 {
                self.min_sort_down(1);
            }
        } else {
            if self.ct / 2 != 0 {
                self.max_sort_down(-1);
            }
            if (self.ct - 1) / 2 != 0 {
                self.min_sort_down(1);
            }
        }

        GslError::Success
    }

    fn get(&self) -> MedAccType {
        let median = self.data[self.heap[0].abs() as usize];
        if self.ct & 1 == 0 {
            (median + self.data[self.heap[1].abs() as usize]) / 2.0
        } else {
            median
        }
    }

    fn mm_less(&self, i: i32, j: i32) -> bool {
        self.data[self.heap[i.abs() as usize] as usize] < self.data[self.heap[j.abs() as usize] as usize]
    }

    fn mm_exchange(&mut self, i: i32, j: i32) {
        self.heap.swap(i.abs() as usize, j.abs() as usize);
        self.pos[self.heap[i.abs() as usize] as usize] = i;
        self.pos[self.heap[j.abs() as usize] as usize] = j;
    }

    fn mm_cmp_exch(&mut self, i: i32, j: i32) -> bool {
        if self.mm_less(i, j) {
            self.mm_exchange(i, j);
            true
        } else {
            false
        }
    }

    fn min_sort_down(&mut self, mut i: i32) {
        while i <= (self.ct - 1) / 2 {
            if i > 1 && i < (self.ct - 1) / 2 && self.mm_less(i + 1, i) {
                i += 1;
            }
            if !self.mm_cmp_exch(i, i / 2) {
                break;
            }
            i *= 2;
        }
    }

    fn max_sort_down(&mut self, mut i: i32) {
        while i >= -(self.ct / 2) {
            if i < -1 && i > -(self.ct / 2) && self.mm_less(i, i - 1) {
                i -= 1;
            }
            if !self.mm_cmp_exch(i / 2, i) {
                break;
            }
            i *= 2;
        }
    }

    fn min_sort_up(&mut self, mut i: i32) -> bool {
        while i > 0 && self.mm_cmp_exch(i, i / 2) {
            i /= 2;
        }
        i == 0
    }

    fn max_sort_up(&mut self, mut i: i32) -> bool {
        while i < 0 && self.mm_cmp_exch(i / 2, i) {
            i /= 2;
        }
        i == 0
    }
}

pub struct GslMovstatAccum {
    size: fn(SizeT) -> SizeT,
    init: fn(SizeT) -> Result<MedAccState, GslError>,
    insert: fn(&mut MedAccState, MedAccType) -> GslError,
    get: fn(&MedAccState) -> MedAccType,
}

pub fn gsl_movstat_accum_median() -> GslMovstatAccum {
    GslMovstatAccum {
        size: |n| {
            std::mem::size_of::<MedAccState>() + 
            n * std::mem::size_of::<MedAccType>() + 
            2 * n * std::mem::size_of::<i32>()
        },
        init: |n| Ok(MedAccState::new(n)),
        insert: |state, x| state.insert(x),
        get: |state| state.get(),
    }
}