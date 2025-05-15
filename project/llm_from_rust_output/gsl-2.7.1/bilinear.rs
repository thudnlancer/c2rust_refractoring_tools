use std::ffi::CStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDom = 1,
    ERange = 2,
    EDefault = 3,
    EInval = 4,
    EFailed = 5,
    EFactor = 6,
    ESanity = 7,
    ENomem = 8,
    EBadfunc = 9,
    ERunaway = 10,
    EMaxiter = 11,
    EZerodiv = 12,
    EBadtol = 13,
    ETol = 14,
    EUndrflw = 15,
    EOvrflw = 16,
    ELoss = 17,
    ERound = 18,
    EBadlen = 19,
    ENotsqr = 20,
    ESing = 21,
    EDiverge = 22,
    EUnsup = 23,
    EUnimpl = 24,
    ECache = 25,
    ETable = 26,
    ENoprog = 27,
    ENoprogj = 28,
    ETolf = 29,
    ETolx = 30,
    ETolg = 31,
    Eof = 32,
}

#[derive(Debug, Clone, Copy)]
pub struct GslInterpAccel {
    pub cache: usize,
    pub miss_count: usize,
    pub hit_count: usize,
}

impl GslInterpAccel {
    pub fn new() -> Self {
        Self {
            cache: 0,
            miss_count: 0,
            hit_count: 0,
        }
    }

    pub fn find(&mut self, xa: &[f64], x: f64) -> usize {
        let mut x_index = self.cache;
        if x < xa[x_index] {
            self.miss_count += 1;
            self.cache = Self::bsearch(xa, x, 0, x_index);
        } else if x >= xa[x_index + 1] {
            self.miss_count += 1;
            self.cache = Self::bsearch(xa, x, x_index, xa.len() - 1);
        } else {
            self.hit_count += 1;
        }
        self.cache
    }

    fn bsearch(x_array: &[f64], x: f64, index_lo: usize, index_hi: usize) -> usize {
        let mut ilo = index_lo;
        let mut ihi = index_hi;
        while ihi > ilo + 1 {
            let i = (ihi + ilo) / 2;
            if x_array[i] > x {
                ihi = i;
            } else {
                ilo = i;
            }
        }
        ilo
    }
}

pub struct GslInterp2dType {
    pub name: &'static str,
    pub min_size: u32,
    pub alloc: Option<fn(usize, usize) -> Box<[u8]>>,
    pub init: Option<fn(&mut [u8], &[f64], &[f64], &[f64], usize, usize) -> GslError>,
    pub eval: Option<
        fn(
            &[u8],
            &[f64],
            &[f64],
            &[f64],
            usize,
            usize,
            f64,
            f64,
            &mut GslInterpAccel,
            &mut GslInterpAccel,
            &mut f64,
        ) -> GslError,
    >,
    pub eval_deriv_x: Option<
        fn(
            &[u8],
            &[f64],
            &[f64],
            &[f64],
            usize,
            usize,
            f64,
            f64,
            &mut GslInterpAccel,
            &mut GslInterpAccel,
            &mut f64,
        ) -> GslError,
    >,
    pub eval_deriv_y: Option<
        fn(
            &[u8],
            &[f64],
            &[f64],
            &[f64],
            usize,
            usize,
            f64,
            f64,
            &mut GslInterpAccel,
            &mut GslInterpAccel,
            &mut f64,
        ) -> GslError,
    >,
    pub eval_deriv_xx: Option<
        fn(
            &[u8],
            &[f64],
            &[f64],
            &[f64],
            usize,
            usize,
            f64,
            f64,
            &mut GslInterpAccel,
            &mut GslInterpAccel,
            &mut f64,
        ) -> GslError,
    >,
    pub eval_deriv_xy: Option<
        fn(
            &[u8],
            &[f64],
            &[f64],
            &[f64],
            usize,
            usize,
            f64,
            f64,
            &mut GslInterpAccel,
            &mut GslInterpAccel,
            &mut f64,
        ) -> GslError,
    >,
    pub eval_deriv_yy: Option<
        fn(
            &[u8],
            &[f64],
            &[f64],
            &[f64],
            usize,
            usize,
            f64,
            f64,
            &mut GslInterpAccel,
            &mut GslInterpAccel,
            &mut f64,
        ) -> GslError,
    >,
    pub free: Option<fn(&mut [u8])>,
}

fn bilinear_init(
    _state: &mut [u8],
    _xa: &[f64],
    _ya: &[f64],
    _za: &[f64],
    _xsize: usize,
    _ysize: usize,
) -> GslError {
    GslError::Success
}

fn bilinear_eval(
    _state: &[u8],
    xarr: &[f64],
    yarr: &[f64],
    zarr: &[f64],
    xsize: usize,
    _ysize: usize,
    x: f64,
    y: f64,
    xa: &mut GslInterpAccel,
    ya: &mut GslInterpAccel,
    z: &mut f64,
) -> GslError {
    let xi = if xa.cache == 0 {
        GslInterpAccel::bsearch(xarr, x, 0, xarr.len() - 1)
    } else {
        xa.find(xarr, x)
    };

    let yi = if ya.cache == 0 {
        GslInterpAccel::bsearch(yarr, y, 0, yarr.len() - 1)
    } else {
        ya.find(yarr, y)
    };

    let xmin = xarr[xi];
    let xmax = xarr[xi + 1];
    let ymin = yarr[yi];
    let ymax = yarr[yi + 1];

    let zminmin = zarr[yi * xsize + xi];
    let zminmax = zarr[(yi + 1) * xsize + xi];
    let zmaxmin = zarr[yi * xsize + xi + 1];
    let zmaxmax = zarr[(yi + 1) * xsize + xi + 1];

    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let t = (x - xmin) / dx;
    let u = (y - ymin) / dy;

    *z = (1.0 - t) * (1.0 - u) * zminmin 
        + t * (1.0 - u) * zmaxmin 
        + (1.0 - t) * u * zminmax 
        + t * u * zmaxmax;

    GslError::Success
}

fn bilinear_deriv_x(
    _state: &[u8],
    xarr: &[f64],
    yarr: &[f64],
    zarr: &[f64],
    xsize: usize,
    _ysize: usize,
    x: f64,
    y: f64,
    xa: &mut GslInterpAccel,
    ya: &mut GslInterpAccel,
    z_p: &mut f64,
) -> GslError {
    let xi = if xa.cache == 0 {
        GslInterpAccel::bsearch(xarr, x, 0, xarr.len() - 1)
    } else {
        xa.find(xarr, x)
    };

    let yi = if ya.cache == 0 {
        GslInterpAccel::bsearch(yarr, y, 0, yarr.len() - 1)
    } else {
        ya.find(yarr, y)
    };

    let xmin = xarr[xi];
    let xmax = xarr[xi + 1];
    let ymin = yarr[yi];
    let ymax = yarr[yi + 1];

    let zminmin = zarr[yi * xsize + xi];
    let zminmax = zarr[(yi + 1) * xsize + xi];
    let zmaxmin = zarr[yi * xsize + xi + 1];
    let zmaxmax = zarr[(yi + 1) * xsize + xi + 1];

    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let dt = 1.0 / dx;
    let u = (y - ymin) / dy;

    *z_p = dt * (-(1.0 - u) * zminmin + (1.0 - u) * zmaxmin - u * zminmax + u * zmaxmax);

    GslError::Success
}

fn bilinear_deriv_y(
    _state: &[u8],
    xarr: &[f64],
    yarr: &[f64],
    zarr: &[f64],
    xsize: usize,
    _ysize: usize,
    x: f64,
    y: f64,
    xa: &mut GslInterpAccel,
    ya: &mut GslInterpAccel,
    z_p: &mut f64,
) -> GslError {
    let xi = if xa.cache == 0 {
        GslInterpAccel::bsearch(xarr, x, 0, xarr.len() - 1)
    } else {
        xa.find(xarr, x)
    };

    let yi = if ya.cache == 0 {
        GslInterpAccel::bsearch(yarr, y, 0, yarr.len() - 1)
    } else {
        ya.find(yarr, y)
    };

    let xmin = xarr[xi];
    let xmax = xarr[xi + 1];
    let ymin = yarr[yi];
    let ymax = yarr[yi + 1];

    let zminmin = zarr[yi * xsize + xi];
    let zminmax = zarr[(yi + 1) * xsize + xi];
    let zmaxmin = zarr[yi * xsize + xi + 1];
    let zmaxmax = zarr[(yi + 1) * xsize + xi + 1];

    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let t = (x - xmin) / dx;
    let du = 1.0 / dy;

    *z_p = du * (-(1.0 - t) * zminmin - t * zmaxmin + (1.0 - t) * zminmax + t * zmaxmax;

    GslError::Success
}

fn bilinear_deriv2(
    _state: &[u8],
    _xarr: &[f64],
    _yarr: &[f64],
    _zarr: &[f64],
    _xsize: usize,
    _ysize: usize,
    _x: f64,
    _y: f64,
    _xa: &mut GslInterpAccel,
    _ya: &mut GslInterpAccel,
    z_pp: &mut f64,
) -> GslError {
    *z_pp = 0.0;
    GslError::Success
}

fn bilinear_derivxy(
    _state: &[u8],
    xarr: &[f64],
    yarr: &[f64],
    zarr: &[f64],
    xsize: usize,
    _ysize: usize,
    x: f64,
    y: f64,
    xa: &mut GslInterpAccel,
    ya: &mut GslInterpAccel,
    z_pp: &mut f64,
) -> GslError {
    let xi = if xa.cache == 0 {
        GslInterpAccel::bsearch(xarr, x, 0, xarr.len() - 1)
    } else {
        xa.find(xarr, x)
    };

    let yi = if ya.cache == 0 {
        GslInterpAccel::bsearch(yarr, y, 0, yarr.len() - 1)
    } else {
        ya.find(yarr, y)
    };

    let xmin = xarr[xi];
    let xmax = xarr[xi + 1];
    let ymin = yarr[yi];
    let ymax = yarr[yi + 1];

    let zminmin = zarr[yi * xsize + xi];
    let zminmax = zarr[(yi + 1) * xsize + xi];
    let zmaxmin = zarr[yi * xsize + xi + 1];
    let zmaxmax = zarr[(yi + 1) * xsize + xi + 1];

    let dx = xmax - xmin;
    let dy = ymax - ymin;
    let dt = 1.0 / dx;
    let du = 1.0 / dy;

    *z_pp = dt * du * (zminmin - zmaxmin - zminmax + zmaxmax);

    GslError::Success
}

pub static GSL_INTERP2D_BILINEAR: GslInterp2dType = GslInterp2dType {
    name: "bilinear",
    min_size: 2,
    alloc: None,
    init: Some(bilinear_init),
    eval: Some(bilinear_eval),
    eval_deriv_x: Some(bilinear_deriv_x),
    eval_deriv_y: Some(bilinear_deriv_y),
    eval_deriv_xx: Some(bilinear_deriv2),
    eval_deriv_xy: Some(bilinear_derivxy),
    eval_deriv_yy: Some(bilinear_deriv2),
    free: None,
};