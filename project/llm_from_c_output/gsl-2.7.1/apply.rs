use std::cmp::max;

#[derive(Debug, Clone, Copy)]
pub enum MovStatEnd {
    Truncate,
    PadZero,
    PadValue,
}

pub struct MovStatAccum {
    pub init: fn(usize, &mut Vec<f64>),
    pub insert: fn(f64, &mut Vec<f64>),
    pub delete_oldest: Option<fn(&mut Vec<f64>)>,
    pub get: fn(&mut dyn std::any::Any, &mut [f64; 2], &mut Vec<f64>),
}

pub struct MovStatWorkspace {
    pub h: usize,
    pub j: usize,
    pub k: usize,
    pub state: Vec<f64>,
    pub work: Vec<f64>,
}

pub struct MovStatFunction {
    pub func: fn(&[f64], &mut dyn std::any::Any, &mut [f64; 2]),
}

pub fn movstat_apply_accum(
    endtype: MovStatEnd,
    x: &[f64],
    accum: &MovStatAccum,
    accum_params: &mut dyn std::any::Any,
    y: &mut [f64],
    z: Option<&mut [f64]>,
    w: &mut MovStatWorkspace,
) -> Result<(), &'static str> {
    if x.len() != y.len() {
        return Err("input and output vectors must have same length");
    }
    if let Some(z) = z {
        if x.len() != z.len() {
            return Err("input and output vectors must have same length");
        }
    }

    let n = x.len();
    let h = w.h;
    let j = w.j;
    let mut result = [0.0; 2];

    // Initialize accumulator
    (accum.init)(w.k, &mut w.state);

    // Pad initial window if necessary
    if !matches!(endtype, MovStatEnd::Truncate) {
        let (x1, xn) = match endtype {
            MovStatEnd::PadZero => (0.0, 0.0),
            MovStatEnd::PadValue => (x[0], x[n - 1]),
            _ => unreachable!(),
        };

        // Pad initial windows with H values
        for _ in 0..h {
            (accum.insert)(x1, &mut w.state);
        }
    } else if accum.delete_oldest.is_none() {
        // Save last K - 1 samples of x for later (needed for in-place input/output)
        let idx1 = max(n as isize - j as isize - h as isize, 0) as usize;
        let idx2 = n - 1;

        w.work.clear();
        w.work.extend_from_slice(&x[idx1..=idx2]);
    }

    // Process input vector and fill y[0..n - J - 1]
    for i in 0..n {
        let xi = x[i];
        let idx = i as isize - j as isize;

        (accum.insert)(xi, &mut w.state);

        if idx >= 0 {
            (accum.get)(accum_params, &mut result, &mut w.state);
            y[idx as usize] = result[0];

            if let Some(z) = z {
                z[idx as usize] = result[1];
            }
        }
    }

    if matches!(endtype, MovStatEnd::Truncate) {
        // Need to fill y[n-J..n-1] using shrinking windows
        let idx1 = max(n as isize - j as isize, 0) as usize;
        let idx2 = n - 1;

        if accum.delete_oldest.is_none() {
            let wsize = n - max(n as isize - j as isize - h as isize, 0) as usize;

            for i in idx1..=idx2 {
                let nsamp = n - max(i as isize - h as isize, 0) as usize;
                let mut j = wsize - nsamp;

                (accum.init)(w.k, &mut w.state);

                while j < wsize {
                    (accum.insert)(w.work[j], &mut w.state);
                    j += 1;
                }

                // yi = acc_get [ work(i:K-2) ]
                (accum.get)(accum_params, &mut result, &mut w.state);
                y[i] = result[0];

                if let Some(z) = z {
                    z[i] = result[1];
                }
            }
        } else {
            for i in idx1..=idx2 {
                if i > h {
                    // Delete oldest window sample as we move closer to edge
                    if let Some(delete_oldest) = accum.delete_oldest {
                        delete_oldest(&mut w.state);
                    }
                }

                // yi = acc_get [ work(i:K-2) ]
                (accum.get)(accum_params, &mut result, &mut w.state);
                y[i] = result[0];

                if let Some(z) = z {
                    z[i] = result[1];
                }
            }
        }
    } else {
        // Pad final windows and fill y[n-J..n-1]
        for i in 0..j {
            let idx = n as isize - j as isize + i as isize;
            let xn = match endtype {
                MovStatEnd::PadZero => 0.0,
                MovStatEnd::PadValue => x[n - 1],
                _ => unreachable!(),
            };

            (accum.insert)(xn, &mut w.state);

            if idx >= 0 {
                (accum.get)(accum_params, &mut result, &mut w.state);
                y[idx as usize] = result[0];

                if let Some(z) = z {
                    z[idx as usize] = result[1];
                }
            }
        }
    }

    Ok(())
}

pub fn movstat_apply(
    endtype: MovStatEnd,
    f: &MovStatFunction,
    x: &[f64],
    y: &mut [f64],
    w: &mut MovStatWorkspace,
) -> Result<(), &'static str> {
    let accum = MovStatAccum {
        init: |_, _| {},
        insert: |_, _| {},
        delete_oldest: None,
        get: |params, result, state| {
            let f = params.downcast_ref::<MovStatFunction>().unwrap();
            (f.func)(state, params, result);
        },
    };

    movstat_apply_accum(endtype, x, &accum, f, y, None, w)
}