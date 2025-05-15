use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct GslFunction {
    pub function: fn(f64, &mut ()) -> f64,
    pub params: *mut (),
}

#[derive(Debug, Clone, Copy)]
pub struct GslIntegrationGlfixedTable {
    pub n: usize,
    pub x: *mut f64,
    pub w: *mut f64,
    pub precomputed: i32,
}

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_EINVAL: i32 = 4;
pub const GSL_ENOMEM: i32 = 8;

static GLAWSIZE: usize = 27;

static mut glaw: [GslIntegrationGlfixedTable; 27] = [
    GslIntegrationGlfixedTable {
        n: 2,
        x: [0.5773502691896258].as_ptr() as *mut f64,
        w: [1.0].as_ptr() as *mut f64,
        precomputed: 1,
    },
    // ... (other precomputed tables would go here)
];

static LTBL: [f64; 1024] = [
    0.0, 0.0, 0.5, 0.6666666666666666, 
    // ... (rest of the LTBL values)
];

pub fn gsl_integration_glfixed_table_alloc(n: usize) -> Option<Box<GslIntegrationGlfixedTable>> {
    if n > i32::MAX as usize {
        return None;
    }

    // Check precomputed tables first
    for table in unsafe { &glaw } {
        if table.n == n {
            return Some(Box::new(*table));
        }
    }

    // If not found, compute new table
    let m = (n + 1) / 2;
    let mut x = vec![0.0; m];
    let mut w = vec![0.0; m];

    gauss_legendre_tbl(n as i32, &mut x, &mut w, 1e-10);

    Some(Box::new(GslIntegrationGlfixedTable {
        n,
        x: x.as_mut_ptr(),
        w: w.as_mut_ptr(),
        precomputed: 0,
    }))
}

pub fn gsl_integration_glfixed_table_free(table: Box<GslIntegrationGlfixedTable>) {
    if table.precomputed == 0 {
        // Memory was allocated by us, so we need to free it
        unsafe {
            let _ = Vec::from_raw_parts(table.x, table.n, table.n);
            let _ = Vec::from_raw_parts(table.w, table.n, table.n);
        }
    }
    // Box will be dropped here automatically
}

pub fn gsl_integration_glfixed(
    f: GslFunction,
    a: f64,
    b: f64,
    table: &GslIntegrationGlfixedTable,
) -> f64 {
    let n = table.n as i32;
    let m = (n + 1) / 2;
    let a_scaled = 0.5 * (b - a);
    let b_scaled = 0.5 * (b + a);

    let x = unsafe { std::slice::from_raw_parts(table.x, m as usize) };
    let w = unsafe { std::slice::from_raw_parts(table.w, m as usize) };

    let mut sum = if n % 2 != 0 {
        w[0] * (f.function)(b_scaled, unsafe { &mut *f.params })
    } else {
        0.0
    };

    let start = if n % 2 != 0 { 1 } else { 0 };
    for i in start..m as usize {
        let ax = a_scaled * x[i];
        sum += w[i] * ((f.function)(b_scaled + ax, unsafe { &mut *f.params })
            + (f.function)(b_scaled - ax, unsafe { &mut *f.params }));
    }

    a_scaled * sum
}

fn gauss_legendre_tbl(n: i32, x: &mut [f64], w: &mut [f64], eps: f64) {
    let m = (n + 1) / 2;
    let t0 = 1.0 - (1.0 - 1.0 / n as f64) / (8.0 * (n * n) as f64);
    let t1 = 1.0 / (4.0 * n as f64 + 2.0);

    for i in 1..=m {
        let mut x0 = (PI * ((4 * i - 1) as f64) * t1).cos() * t0;
        let mut j = 0;
        let mut dw = f64::MAX;
        let mut dx = dw;

        loop {
            let (x1, w1) = legendre_poly(n, x0);
            if j == 0 {
                w[i-1] = 2.0 / ((1.0 - x0 * x0) * (n as f64 * (x0 * x1 - LTBL[1]) / (x0 * x0 - 1.0)).powi(2));
            }
            dx = x0 - x1;
            dw = w[i-1] - w1;
            x0 = x1;
            w[i-1] = w1;
            j += 1;

            if (dx.abs() <= eps && dw.abs() <= eps) || j >= 100 {
                break;
            }
        }

        x[m - i] = x0;
        w[m - i] = w[i-1];
    }
}

fn legendre_poly(n: i32, x: f64) -> (f64, f64) {
    let mut p_prev = 1.0;
    let mut p_curr = x;
    
    for k in 2..=n {
        let p_next = if k < 1024 {
            let ratio = LTBL[k as usize];
            x * p_curr + ratio * (x * p_curr - p_prev)
        } else {
            let ratio = (k - 1) as f64 / k as f64;
            x * p_curr + ratio * (x * p_curr - p_prev)
        };
        p_prev = p_curr;
        p_curr = p_next;
    }

    let dpdx = n as f64 * (x * p_curr - p_prev) / (x * x - 1.0);
    let x1 = x - p_curr / dpdx;
    let w1 = 2.0 / ((1.0 - x1 * x1) * dpdx * dpdx);
    
    (x1, w1)
}