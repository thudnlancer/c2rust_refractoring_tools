use std::f64;
use std::io::{self, Write};
use rand::Rng;
use thiserror::Error;

const BINS_MAX: usize = 50;

#[derive(Error, Debug)]
pub enum VegasError {
    #[error("number of dimensions must match allocated size")]
    DimensionMismatch,
    #[error("xu must be greater than xl")]
    InvalidRange,
    #[error("range of integration is too large, please rescale")]
    RangeTooLarge,
    #[error("failed to allocate memory")]
    AllocationFailed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VegasMode {
    ImportanceOnly,
    Importance,
    Stratified,
}

#[derive(Debug)]
pub struct VegasParams {
    pub alpha: f64,
    pub iterations: usize,
    pub stage: usize,
    pub mode: VegasMode,
    pub verbose: i32,
    pub ostream: Option<Box<dyn Write>>,
}

#[derive(Debug)]
pub struct VegasState {
    dim: usize,
    bins_max: usize,
    delx: Vec<f64>,
    d: Vec<f64>,
    xi: Vec<f64>,
    xin: Vec<f64>,
    weight: Vec<f64>,
    box_coord: Vec<usize>,
    bin: Vec<usize>,
    x: Vec<f64>,
    vol: f64,
    jac: f64,
    calls_per_box: usize,
    boxes: usize,
    bins: usize,
    alpha: f64,
    verbose: i32,
    iterations: usize,
    mode: VegasMode,
    stage: usize,
    wtd_int_sum: f64,
    sum_wgts: f64,
    chi_sum: f64,
    it_num: usize,
    samples: usize,
    chisq: f64,
    result: f64,
    sigma: f64,
    it_start: usize,
    ostream: Option<Box<dyn Write>>,
}

impl VegasState {
    pub fn new(dim: usize) -> Result<Self, VegasError> {
        if dim == 0 {
            return Err(VegasError::AllocationFailed);
        }

        Ok(Self {
            dim,
            bins_max: BINS_MAX,
            delx: vec![0.0; dim],
            d: vec![0.0; BINS_MAX * dim],
            xi: vec![0.0; (BINS_MAX + 1) * dim],
            xin: vec![0.0; BINS_MAX + 1],
            weight: vec![0.0; BINS_MAX],
            box_coord: vec![0; dim],
            bin: vec![0; dim],
            x: vec![0.0; dim],
            vol: 0.0,
            jac: 0.0,
            calls_per_box: 0,
            boxes: 0,
            bins: BINS_MAX,
            alpha: 1.5,
            verbose: -1,
            iterations: 5,
            mode: VegasMode::Importance,
            stage: 0,
            wtd_int_sum: 0.0,
            sum_wgts: 0.0,
            chi_sum: 0.0,
            it_num: 0,
            samples: 0,
            chisq: 0.0,
            result: 0.0,
            sigma: 0.0,
            it_start: 0,
            ostream: None,
        })
    }

    pub fn init(&mut self) {
        self.stage = 0;
        self.alpha = 1.5;
        self.verbose = -1;
        self.iterations = 5;
        self.mode = VegasMode::Importance;
        self.chisq = 0.0;
        self.bins = self.bins_max;
        self.ostream = None;
    }

    pub fn chisq(&self) -> f64 {
        self.chisq
    }

    pub fn runval(&self) -> (f64, f64) {
        (self.result, self.sigma)
    }

    pub fn params_get(&self) -> VegasParams {
        VegasParams {
            alpha: self.alpha,
            iterations: self.iterations,
            stage: self.stage,
            mode: self.mode,
            verbose: self.verbose,
            ostream: self.ostream.as_ref().map(|_| Box::new(io::sink())),
        }
    }

    pub fn params_set(&mut self, params: &VegasParams) {
        self.alpha = params.alpha;
        self.iterations = params.iterations;
        self.stage = params.stage;
        self.mode = params.mode;
        self.verbose = params.verbose;
        self.ostream = params.ostream.as_ref().map(|_| Box::new(io::sink()));
    }

    fn coord(&self, i: usize, j: usize) -> f64 {
        self.xi[i * self.dim + j]
    }

    fn coord_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.xi[i * self.dim + j]
    }

    fn new_coord(&self, i: usize) -> f64 {
        self.xin[i]
    }

    fn new_coord_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.xin[i]
    }

    fn value(&self, i: usize, j: usize) -> f64 {
        self.d[i * self.dim + j]
    }

    fn value_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.d[i * self.dim + j]
    }
}

pub fn vegas_integrate<F>(
    f: F,
    xl: &[f64],
    xu: &[f64],
    calls: usize,
    rng: &mut impl Rng,
    state: &mut VegasState,
) -> Result<(f64, f64), VegasError>
where
    F: Fn(&[f64]) -> f64,
{
    let dim = state.dim;

    if dim != xl.len() || dim != xu.len() {
        return Err(VegasError::DimensionMismatch);
    }

    for j in 0..dim {
        if xu[j] <= xl[j] {
            return Err(VegasError::InvalidRange);
        }
        if xu[j] - xl[j] > f64::MAX {
            return Err(VegasError::RangeTooLarge);
        }
    }

    if state.stage == 0 {
        init_grid(state, xl, xu, dim)?;

        if state.verbose >= 0 {
            print_lim(state, xl, xu, dim)?;
        }
    }

    if state.stage <= 1 {
        state.wtd_int_sum = 0.0;
        state.sum_wgts = 0.0;
        state.chi_sum = 0.0;
        state.it_num = 1;
        state.samples = 0;
        state.chisq = 0.0;
    }

    if state.stage <= 2 {
        let mut bins = state.bins_max;
        let mut boxes = 1;

        if state.mode != VegasMode::ImportanceOnly {
            boxes = (calls as f64 / 2.0).powf(1.0 / dim as f64).floor() as usize;
            state.mode = VegasMode::Importance;

            if 2 * boxes >= state.bins_max {
                let box_per_bin = (boxes / state.bins_max).max(1);
                bins = (boxes / box_per_bin).min(state.bins_max);
                boxes = box_per_bin * bins;
                state.mode = VegasMode::Stratified;
            }
        }

        let tot_boxes = (boxes as f64).powi(dim as i32);
        state.calls_per_box = (calls as f64 / tot_boxes).max(2.0) as usize;
        let calls = state.calls_per_box * tot_boxes as usize;

        state.jac = state.vol * (bins as f64).powi(dim as i32) / calls as f64;
        state.boxes = boxes;

        if bins != state.bins {
            resize_grid(state, bins)?;

            if state.verbose > 1 {
                print_grid(state, dim)?;
            }
        }

        if state.verbose >= 0 {
            print_head(state, dim, calls, state.it_num, state.bins, state.boxes)?;
        }
    }

    state.it_start = state.it_num;

    let mut cum_int = 0.0;
    let mut cum_sig = 0.0;

    for it in 0..state.iterations {
        let mut intgrl = 0.0;
        let mut intgrl_sq = 0.0;
        let mut tss = 0.0;
        let calls_per_box = state.calls_per_box;
        let jacbin = state.jac;

        state.it_num = state.it_start + it;

        reset_grid_values(state);
        init_box_coord(state);

        loop {
            let mut m = 0.0;
            let mut q = 0.0;
            let mut f_sq_sum = 0.0;

            for k in 0..calls_per_box {
                let mut bin_vol = 0.0;
                random_point(&mut state.x, &mut state.bin, &mut bin_vol, &state.box_coord, xl, state, rng);

                let fval = jacbin * bin_vol * f(&state.x);

                let d = fval - m;
                m += d / (k as f64 + 1.0);
                q += d * d * (k as f64 / (k as f64 + 1.0));

                if state.mode != VegasMode::Stratified {
                    let f_sq = fval * fval;
                    accumulate_distribution(state, &state.bin, f_sq);
                }
            }

            intgrl += m * calls_per_box as f64;
            f_sq_sum = q * calls_per_box as f64;
            tss += f_sq_sum;

            if state.mode == VegasMode::Stratified {
                accumulate_distribution(state, &state.bin, f_sq_sum);
            }

            if !change_box_coord(state) {
                break;
            }
        }

        let var = tss / (calls_per_box as f64 - 1.0);

        let wgt = if var > 0.0 {
            1.0 / var
        } else if state.sum_wgts > 0.0 {
            state.sum_wgts / state.samples as f64
        } else {
            0.0
        };

        intgrl_sq = intgrl * intgrl;
        let sig = var.sqrt();

        state.result = intgrl;
        state.sigma = sig;

        if wgt > 0.0 {
            let sum_wgts = state.sum_wgts;
            let wtd_int_sum = state.wtd_int_sum;
            let m = if sum_wgts > 0.0 {
                wtd_int_sum / sum_wgts
            } else {
                0.0
            };
            let q = intgrl - m;

            state.samples += 1;
            state.sum_wgts += wgt;
            state.wtd_int_sum += intgrl * wgt;
            state.chi_sum += intgrl_sq * wgt;

            cum_int = state.wtd_int_sum / state.sum_wgts;
            cum_sig = (1.0 / state.sum_wgts).sqrt();

            if state.samples == 1 {
                state.chisq = 0.0;
            } else {
                state.chisq *= (state.samples - 2) as f64;
                state.chisq += (wgt / (1.0 + (wgt / sum_wgts)) * q * q;
                state.chisq /= (state.samples - 1) as f64;
            }
        } else {
            cum_int += (intgrl - cum_int) / (it as f64 + 1.0);
            cum_sig = 0.0;
        }

        if state.verbose >= 0 {
            print_res(
                state,
                state.it_num,
                intgrl,
                sig,
                cum_int,
                cum_sig,
                state.chisq,
            )?;

            if it + 1 == state.iterations && state.verbose > 0 {
                print_grid(state, dim)?;
            }
        }

        if state.verbose > 1 {
            print_dist(state, dim)?;
        }

        refine_grid(state)?;

        if state.verbose > 1 {
            print_grid(state, dim)?;
        }
    }

    state.stage = 1;

    Ok((cum_int, cum_sig))
}

fn init_grid(state: &mut VegasState, xl: &[f64], xu: &[f64], dim: usize) -> Result<(), VegasError> {
    let mut vol = 1.0;
    state.bins = 1;

    for j in 0..dim {
        let dx = xu[j] - xl[j];
        state.delx[j] = dx;
        vol *= dx;

        *state.coord_mut(0, j) = 0.0;
        *state.coord_mut(1, j) = 1.0;
    }

    state.vol = vol;
    Ok(())
}

fn reset_grid_values(state: &mut VegasState) {
    let dim = state.dim;
    let bins = state.bins;

    for i in 0..bins {
        for j in 0..dim {
            *state.value_mut(i, j) = 0.0;
        }
    }
}

fn init_box_coord(state: &mut VegasState) {
    for coord in state.box_coord.iter_mut() {
        *coord = 0;
    }
}

fn change_box_coord(state: &mut VegasState) -> bool {
    let ng = state.boxes;
    let dim = state.dim;

    for j in (0..dim).rev() {
        state.box_coord[j] = (state.box_coord[j] + 1) % ng;

        if state.box_coord[j] != 0 {
            return true;
        }
    }

    false
}

fn accumulate_distribution(state: &mut VegasState, bin: &[usize], y: f64) {
    for j in 0..state.dim {
        let i = bin[j];
        *state.value_mut(i, j) += y;
    }
}

fn random_point(
    x: &mut [f64],
    bin: &mut [usize],
    bin_vol: &mut f64,
    box_coord: &[usize],
    xl: &[f64],
    state: &mut VegasState,
    rng: &mut impl Rng,
) {
    let dim = state.dim;
    let bins = state.bins;
    let boxes = state.boxes;

    let mut vol = 1.0;

    for j in 0..dim {
        let z = ((box_coord[j] as f64 + rng.gen::<f64>()) / boxes as f64) * bins as f64;
        let k = z as usize;

        let (y, bin_width) = if k == 0 {
            let bin_width = state.coord(1, j);
            (z * bin_width, bin_width)
        } else {
            let bin_width = state.coord(k + 1, j) - state.coord(k, j);
            (state.coord(k, j) + (z - k as f64) * bin_width, bin_width)
        };

        bin[j] = k;
        x[j] = xl[j] + y * state.delx[j];
        vol *= bin_width;
    }

    *bin_vol = vol;
}

fn resize_grid(state: &mut VegasState, bins: usize) -> Result<(), VegasError> {
    let dim = state.dim;
    let pts_per_bin = state.bins as f64 / bins as f64;

    for j in 0..dim {
        let mut xnew = 0.0;
        let mut dw = 0.0;
        let mut i = 1;

        for k in 1..=state.bins {
            dw += 1.0;
            let xold = xnew;
            xnew = state.coord(k, j);

            while dw > pts_per_bin && i <= bins {
                dw -= pts_per_bin;
                *state.new_coord_mut(i) = xnew - (xnew - xold) * dw;
                i += 1;
            }
        }

        for k in 1..bins {
            *state.coord_mut(k, j) = state.new_coord(k);
        }

        *state.coord_mut(bins, j) = 1.0;
    }

    state.bins = bins;
    Ok(())
}

fn refine_grid(state: &mut VegasState) -> Result<(), VegasError> {
    let dim = state.dim;
    let bins = state.bins;

    for j in 0..dim {
        let mut grid_tot_j = 0.0;
        let mut oldg = state.value(0, j);
        let mut newg = state.value(1, j);

        *state.value_mut(0, j) = (oldg + newg) / 2.0;
        grid_tot_j += state.value(0, j);

        for i in 1..bins - 1 {
            let rc = oldg + newg;
            oldg = newg;
            newg = state.value(i + 1, j);
            *state.value_mut(i, j) = (rc + newg) / 3.0;
            grid_tot_j += state.value(i, j);
        }

        *state.value_mut(bins - 1, j) = (newg + oldg) / 2.0;
        grid_tot_j += state.value(bins - 1, j);

        let mut tot_weight = 0.0;

        for i in 0..bins {
            state.weight[i] = 0.0;

            if state.value(i, j) > 0.0 {
                oldg = grid_tot_j / state.value(i,