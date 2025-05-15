use std::cmp::Ordering;
use std::ptr;

struct RingBuf {
    data: Vec<f64>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl RingBuf {
    fn new(n: usize) -> Self {
        RingBuf {
            data: vec![0.0; n],
            head: 0,
            tail: 0,
            size: 0,
            capacity: n,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn insert(&mut self, x: f64) {
        if self.is_full() {
            self.pop_back();
        }
        self.data[self.head] = x;
        self.head = (self.head + 1) % self.capacity;
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<f64> {
        if self.is_empty() {
            None
        } else {
            self.head = (self.head + self.capacity - 1) % self.capacity;
            let val = self.data[self.head];
            self.size -= 1;
            Some(val)
        }
    }

    fn copy_to(&self, dest: &mut [f64]) -> usize {
        let n = self.size;
        if n == 0 {
            return 0;
        }

        let mut idx = self.tail;
        for i in 0..n {
            dest[i] = self.data[idx];
            idx = (idx + 1) % self.capacity;
        }
        n
    }
}

struct SnAccState {
    window: Vec<f64>,
    work: Vec<f64>,
    rbuf: RingBuf,
}

impl SnAccState {
    fn new(n: usize) -> Self {
        SnAccState {
            window: vec![0.0; n],
            work: vec![0.0; n],
            rbuf: RingBuf::new(n),
        }
    }
}

fn snacc_size(n: usize) -> usize {
    std::mem::size_of::<SnAccState>() + 2 * n * std::mem::size_of::<f64>() + n * std::mem::size_of::<f64>()
}

fn snacc_init(n: usize) -> SnAccState {
    SnAccState::new(n)
}

fn snacc_insert(x: f64, state: &mut SnAccState) {
    state.rbuf.insert(x);
}

fn snacc_delete(state: &mut SnAccState) {
    if !state.rbuf.is_empty() {
        state.rbuf.pop_back();
    }
}

fn snacc_get(state: &SnAccState) -> Result<f64, &'static str> {
    let n = state.rbuf.copy_to(&mut state.window[..]);
    if n == 0 {
        return Err("Empty window");
    }

    let window = &mut state.window[..n];
    window.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    // Calculate Sn statistic
    let work = &mut state.work[..n];
    let sn = gsl_stats_sn_from_sorted_data(window, work);

    Ok(sn)
}

fn gsl_stats_sn_from_sorted_data(sorted_data: &[f64], work: &mut [f64]) -> f64 {
    let n = sorted_data.len();
    let n_half = n / 2;
    let median = if n % 2 == 1 {
        sorted_data[n_half]
    } else {
        0.5 * (sorted_data[n_half - 1] + sorted_data[n_half])
    };

    for i in 0..n {
        work[i] = (sorted_data[i] - median).abs();
    }

    work.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let sn = if n % 2 == 1 {
        work[n_half]
    } else {
        0.5 * (work[n_half - 1] + work[n_half])
    };

    1.1926 * sn // scale factor for consistency with GSL
}

pub struct MovStatAccum {
    size_fn: fn(usize) -> usize,
    init_fn: fn(usize) -> SnAccState,
    insert_fn: fn(f64, &mut SnAccState),
    delete_fn: fn(&mut SnAccState),
    get_fn: fn(&SnAccState) -> Result<f64, &'static str>,
}

pub const SN_ACCUM_TYPE: MovStatAccum = MovStatAccum {
    size_fn: snacc_size,
    init_fn: snacc_init,
    insert_fn: snacc_insert,
    delete_fn: snacc_delete,
    get_fn: snacc_get,
};

pub static GSL_MOVSTAT_ACCUM_SN: &MovStatAccum = &SN_ACCUM_TYPE;