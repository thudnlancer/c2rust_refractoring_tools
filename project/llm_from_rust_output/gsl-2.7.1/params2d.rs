use std::slice;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslHistogram2d {
    nx: size_t,
    ny: size_t,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram2d {
    pub fn xmax(&self) -> f64 {
        self.xrange[self.nx]
    }

    pub fn xmin(&self) -> f64 {
        self.xrange[0]
    }

    pub fn ymax(&self) -> f64 {
        self.yrange[self.ny]
    }

    pub fn ymin(&self) -> f64 {
        self.yrange[0]
    }

    pub fn nx(&self) -> size_t {
        self.nx
    }

    pub fn ny(&self) -> size_t {
        self.ny
    }
}

// FFI-compatible version for C interop
#[repr(C)]
pub struct gsl_histogram2d {
    nx: size_t,
    ny: size_t,
    xrange: *mut f64,
    yrange: *mut f64,
    bin: *mut f64,
}

impl From<&GslHistogram2d> for gsl_histogram2d {
    fn from(hist: &GslHistogram2d) -> Self {
        gsl_histogram2d {
            nx: hist.nx,
            ny: hist.ny,
            xrange: hist.xrange.as_ptr() as *mut f64,
            yrange: hist.yrange.as_ptr() as *mut f64,
            bin: hist.bin.as_ptr() as *mut f64,
        }
    }
}

impl From<gsl_histogram2d> for GslHistogram2d {
    unsafe fn from(hist: gsl_histogram2d) -> Self {
        let xrange = slice::from_raw_parts(hist.xrange, hist.nx + 1).to_vec();
        let yrange = slice::from_raw_parts(hist.yrange, hist.ny + 1).to_vec();
        let bin = slice::from_raw_parts(hist.bin, hist.nx * hist.ny).to_vec();
        
        GslHistogram2d {
            nx: hist.nx,
            ny: hist.ny,
            xrange,
            yrange,
            bin,
        }
    }
}