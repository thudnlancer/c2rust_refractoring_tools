use std::cmp::Ordering;

type MedAccType = f64;

struct MedAccState {
    n: usize,                  // window size
    idx: usize,                // position in circular queue
    ct: usize,                 // count of items in queue
    data: Vec<MedAccType>,      // circular queue of values, size n
    pos: Vec<i32>,             // index into `heap` for each value, size 2*n
    heap: Vec<i32>,            // max/median/min heap holding indices into `data`
}

impl MedAccState {
    fn min_ct(&self) -> usize {
        (self.ct - 1) / 2
    }

    fn max_ct(&self) -> usize {
        self.ct / 2
    }

    fn item_less(a: MedAccType, b: MedAccType) -> bool {
        a < b
    }

    fn item_mean(a: MedAccType, b: MedAccType) -> MedAccType {
        (a + b) / 2.0
    }

    fn mm_less(&self, i: i32, j: i32) -> bool {
        Self::item_less(
            self.data[self.heap[i as usize] as usize],
            self.data[self.heap[j as usize] as usize],
        )
    }

    fn mm_exchange(&mut self, i: i32, j: i32) -> bool {
        self.heap.swap(i as usize, j as usize);
        self.pos[self.heap[i as usize] as usize] = i;
        self.pos[self.heap[j as usize] as usize] = j;
        true
    }

    fn mm_cmp_exch(&mut self, i: i32, j: i32) -> bool {
        if self.mm_less(i, j) {
            self.mm_exchange(i, j)
        } else {
            false
        }
    }

    fn min_sort_down(&mut self, mut i: i32) {
        while i <= self.min_ct() as i32 {
            let mut child = i;
            if i > 1 && i < self.min_ct() as i32 && self.mm_less(i + 1, i) {
                child += 1;
            }

            if !self.mm_cmp_exch(child, i / 2) {
                break;
            }
            i *= 2;
        }
    }

    fn max_sort_down(&mut self, mut i: i32) {
        while i >= -(self.max_ct() as i32) {
            let mut child = i;
            if i < -1 && i > -(self.max_ct() as i32) && self.mm_less(i, i - 1) {
                child -= 1;
            }

            if !self.mm_cmp_exch(i / 2, child) {
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

fn medacc_size(n: usize) -> usize {
    std::mem::size_of::<MedAccState>() + 
    n * std::mem::size_of::<MedAccType>() + 
    2 * n * std::mem::size_of::<i32>()
}

fn medacc_init(n: usize) -> Result<MedAccState, &'static str> {
    let mut state = MedAccState {
        n,
        idx: 0,
        ct: 0,
        data: vec![0.0; n],
        pos: vec![0; 2 * n],
        heap: vec![0; 2 * n],
    };

    // Set up initial heap fill pattern: median,max,min,max,...
    for k in 0..n {
        state.pos[k] = (((k + 1) / 2) as i32) * if (k & 1) != 0 { -1 } else { 1 });
        state.heap[state.pos[k] as usize] = k as i32;
    }

    Ok(state)
}

fn medacc_insert(state: &mut MedAccState, x: MedAccType) -> Result<(), &'static str> {
    let is_new = state.ct < state.n;
    let p = state.pos[state.idx];
    let old = state.data[state.idx];

    state.data[state.idx] = x;
    state.idx = (state.idx + 1) % state.n;
    state.ct += if is_new { 1 } else { 0 };

    match p.cmp(&0) {
        Ordering::Greater => { // new item is in minHeap
            if !is_new && MedAccState::item_less(old, x) {
                state.min_sort_down(p * 2);
            } else if state.min_sort_up(p) {
                state.max_sort_down(-1);
            }
        }
        Ordering::Less => { // new item is in maxHeap
            if !is_new && MedAccState::item_less(x, old) {
                state.max_sort_down(p * 2);
            } else if state.max_sort_up(p) {
                state.min_sort_down(1);
            }
        }
        Ordering::Equal => { // new item is at median
            if state.max_ct() > 0 {
                state.max_sort_down(-1);
            }
            if state.min_ct() > 0 {
                state.min_sort_down(1);
            }
        }
    }

    Ok(())
}

fn medacc_delete(state: &mut MedAccState) -> Result<(), &'static str> {
    if state.ct > 0 {
        let oldest_idx = (state.idx + state.n - state.ct) % state.n;
        let p = state.pos[oldest_idx];

        match p.cmp(&0) {
            Ordering::Greater => { // oldest item is in minHeap
                state.mm_exchange(p, state.min_ct() as i32);
                state.ct -= 1;
                state.min_sort_down(2 * p);
            }
            Ordering::Less => { // oldest item is in maxHeap
                state.mm_exchange(p, state.max_ct() as i32);
                state.ct -= 1;
                state.max_sort_down(2 * p);
            }
            Ordering::Equal => {} // oldest item is at median
        }
    }

    Ok(())
}

fn medacc_get(state: &MedAccState) -> Result<MedAccType, &'static str> {
    let median = state.data[state.heap[0] as usize];

    if (state.ct & 1) == 0 {
        Ok(MedAccState::item_mean(
            median,
            state.data[state.heap[-1] as usize],
        ))
    } else {
        Ok(median)
    }
}

pub struct MedianAccumulator {
    state: MedAccState,
}

impl MedianAccumulator {
    pub fn new(window_size: usize) -> Result<Self, &'static str> {
        Ok(Self {
            state: medacc_init(window_size)?,
        })
    }

    pub fn insert(&mut self, value: MedAccType) -> Result<(), &'static str> {
        medacc_insert(&mut self.state, value)
    }

    pub fn delete(&mut self) -> Result<(), &'static str> {
        medacc_delete(&mut self.state)
    }

    pub fn get(&self) -> Result<MedAccType, &'static str> {
        medacc_get(&self.state)
    }
}