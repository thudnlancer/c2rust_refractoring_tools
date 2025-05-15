use std::f64;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[derive(Debug, Clone, Copy)]
pub struct GslRstatQuantileWorkspace {
    p: f64,
    q: [f64; 5],
    npos: [i32; 5],
    np: [f64; 5],
    dnp: [f64; 5],
    n: usize,
}

#[derive(Debug, Clone)]
pub struct GslRstatWorkspace {
    min: f64,
    max: f64,
    mean: f64,
    m2: f64,
    m3: f64,
    m4: f64,
    n: usize,
    median_workspace: Option<Box<GslRstatQuantileWorkspace>>,
}

impl GslRstatWorkspace {
    pub fn new() -> Result<Self, GslError> {
        let mut workspace = GslRstatWorkspace {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0,
            n: 0,
            median_workspace: None,
        };

        let quantile_workspace = Box::new(GslRstatQuantileWorkspace {
            p: 0.5,
            q: [0.0; 5],
            npos: [0; 5],
            np: [0.0; 5],
            dnp: [0.0; 5],
            n: 0,
        });

        workspace.median_workspace = Some(quantile_workspace);
        workspace.reset()?;
        Ok(workspace)
    }

    pub fn reset(&mut self) -> Result<(), GslError> {
        self.min = 0.0;
        self.max = 0.0;
        self.mean = 0.0;
        self.m2 = 0.0;
        self.m3 = 0.0;
        self.m4 = 0.0;
        self.n = 0;

        if let Some(ref mut ws) = self.median_workspace {
            ws.n = 0;
            ws.q = [0.0; 5];
            ws.npos = [0; 5];
            ws.np = [0.0; 5];
            ws.dnp = [0.0; 5];
        }

        Ok(())
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn add(&mut self, x: f64) -> Result<(), GslError> {
        let delta = x - self.mean;
        let n = self.n as f64;

        if self.n == 0 {
            self.min = x;
            self.max = x;
        } else {
            if x < self.min {
                self.min = x;
            }
            if x > self.max {
                self.max = x;
            }
        }

        self.n += 1;
        let n_new = self.n as f64;
        let delta_n = delta / n_new;
        let delta_nsq = delta_n * delta_n;
        let term1 = delta * delta_n * (n_new - 1.0);

        self.mean += delta_n;
        self.m4 += term1 * delta_nsq * (n_new * n_new - 3.0 * n_new + 3.0)
            + 6.0 * delta_nsq * self.m2
            - 4.0 * delta_n * self.m3;
        self.m3 += term1 * delta_n * (n_new - 2.0) - 3.0 * delta_n * self.m2;
        self.m2 += term1;

        if let Some(ref mut ws) = self.median_workspace {
            // Simplified quantile add logic
            ws.n += 1;
            let p = ws.p;
            let q = &mut ws.q;
            let npos = &mut ws.npos;
            let np = &mut ws.np;
            let dnp = &mut ws.dnp;

            for i in 0..5 {
                if ws.n <= 1 {
                    q[i] = x;
                } else {
                    let n = ws.n as f64;
                    let desired = np[i] + dnp[i];
                    if (npos[i] as f64) < desired {
                        npos[i] += 1;
                    } else if (npos[i] as f64) > desired {
                        npos[i] -= 1;
                    }
                    q[i] += (x - q[i]) / npos[i] as f64;
                }
            }
        }

        Ok(())
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn variance(&self) -> f64 {
        if self.n > 1 {
            self.m2 / (self.n as f64 - 1.0)
        } else {
            0.0
        }
    }

    pub fn sd(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn rms(&self) -> f64 {
        if self.n > 0 {
            let mean = self.mean();
            let sigma = self.sd();
            let n = self.n as f64;
            let a = ((n - 1.0) / n).sqrt();
            (mean.powi(2) + (a * sigma).powi(2)).sqrt()
        } else {
            0.0
        }
    }

    pub fn sd_mean(&self) -> f64 {
        if self.n > 0 {
            self.sd() / (self.n as f64).sqrt()
        } else {
            0.0
        }
    }

    pub fn median(&mut self) -> f64 {
        if let Some(ref mut ws) = self.median_workspace {
            // Simplified quantile get logic
            ws.q[2] // For p=0.5, return middle quantile
        } else {
            0.0
        }
    }

    pub fn skew(&self) -> f64 {
        if self.n > 0 {
            let n = self.n as f64;
            let fac = (n - 1.0).powf(1.5) / n;
            fac * self.m3 / self.m2.powf(1.5)
        } else {
            0.0
        }
    }

    pub fn kurtosis(&self) -> f64 {
        if self.n > 0 {
            let n = self.n as f64;
            let fac = (n - 1.0) / n * (n - 1.0);
            fac * self.m4 / (self.m2 * self.m2) - 3.0
        } else {
            0.0
        }
    }
}

impl Drop for GslRstatWorkspace {
    fn drop(&mut self) {
        // No need for explicit free in Rust
    }
}