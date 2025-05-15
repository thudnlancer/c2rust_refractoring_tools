/*
 * Translated from C to Rust
 * Original source: movstat/alloc.c
 * Original license: GNU General Public License v3.0
 */

use std::cmp::max;
use std::ptr::null_mut;
use std::alloc::{alloc_zeroed, Layout, dealloc};
use std::mem;

pub struct MovstatWorkspace {
    H: usize,
    J: usize,
    K: usize,
    state_size: usize,
    state: *mut u8,
    work: *mut f64,
}

impl MovstatWorkspace {
    /// Allocate a workspace for moving window statistics.
    /// The window around sample x_i is defined as:
    /// W_i^{H,J} = {x_{i-H},...,x_i,...x_{i+J}}
    /// The total window size is: K = H + J + 1
    /// If K is even, it is rounded up to the next odd
    pub fn new(K: usize) -> Result<Self, &'static str> {
        let H = K / 2;
        Self::new_with_size(0, H, H)
    }

    /// Allocate a workspace with specified H and J values
    pub fn new2(H: usize, J: usize) -> Result<Self, &'static str> {
        Self::new_with_size(0, H, J)
    }

    /// Allocate a workspace with predefined workspace size for accumulators
    pub fn new_with_size(accum_state_size: usize, H: usize, J: usize) -> Result<Self, &'static str> {
        let K = H + J + 1;
        let mut state_size = accum_state_size;

        // Allocate workspace structure
        let layout = Layout::new::<MovstatWorkspace>();
        let w_ptr = unsafe { alloc_zeroed(layout) } as *mut MovstatWorkspace;
        if w_ptr.is_null() {
            return Err("failed to allocate space for workspace");
        }

        let mut w = unsafe { &mut *w_ptr };
        w.H = H;
        w.J = J;
        w.K = K;

        if state_size == 0 {
            // Determine maximum state size needed by accumulators
            state_size = max(state_size, gsl_movstat_accum_mad_size(K));
            state_size = max(state_size, gsl_movstat_accum_mean_size(K));
            state_size = max(state_size, gsl_movstat_accum_min_size(K));
            state_size = max(state_size, gsl_movstat_accum_sum_size(K));
            state_size = max(state_size, gsl_movstat_accum_median_size(K));
            state_size = max(state_size, gsl_movstat_accum_Qn_size(K));
            state_size = max(state_size, gsl_movstat_accum_qqr_size(K));
            state_size = max(state_size, gsl_movstat_accum_Sn_size(K));
        }

        // Allocate state buffer
        let state_layout = Layout::from_size_align(state_size, mem::align_of::<u8>()).unwrap();
        w.state = unsafe { alloc_zeroed(state_layout) };
        if w.state.is_null() {
            unsafe { dealloc(w_ptr as *mut u8, layout); }
            return Err("failed to allocate space for accumulator state");
        }

        // Allocate work buffer
        let work_layout = Layout::array::<f64>(K).unwrap();
        w.work = unsafe { alloc_zeroed(work_layout) } as *mut f64;
        if w.work.is_null() {
            unsafe { dealloc(w.state, state_layout); }
            unsafe { dealloc(w_ptr as *mut u8, layout); }
            return Err("failed to allocate space for work");
        }

        w.state_size = state_size;
        Ok(unsafe { Box::from_raw(w_ptr) })
    }
}

impl Drop for MovstatWorkspace {
    fn drop(&mut self) {
        if !self.work.is_null() {
            let layout = Layout::array::<f64>(self.K).unwrap();
            unsafe { dealloc(self.work as *mut u8, layout); }
        }

        if !self.state.is_null() {
            let layout = Layout::from_size_align(self.state_size, mem::align_of::<u8>()).unwrap();
            unsafe { dealloc(self.state, layout); }
        }
    }
}

// Placeholder functions for accumulator sizes - these should be implemented based on actual requirements
fn gsl_movstat_accum_mad_size(_K: usize) -> usize { 0 }
fn gsl_movstat_accum_mean_size(_K: usize) -> usize { 0 }
fn gsl_movstat_accum_min_size(_K: usize) -> usize { 0 }
fn gsl_movstat_accum_sum_size(_K: usize) -> usize { 0 }
fn gsl_movstat_accum_median_size(_K: usize) -> usize { 0 }
fn gsl_movstat_accum_Qn_size(_K: usize) -> usize { 0 }
fn gsl_movstat_accum_qqr_size(_K: usize) -> usize { 0 }
fn gsl_movstat_accum_Sn_size(_K: usize) -> usize { 0 }