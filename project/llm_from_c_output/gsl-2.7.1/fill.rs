use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovStatEndType {
    Truncate,
    PadZero,
    PadValue,
}

pub fn movstat_fill(
    endtype: MovStatEndType,
    x: &[f64],
    idx: usize,
    h: usize,
    j: usize,
    window: &mut [f64],
) -> Result<usize, &'static str> {
    if idx >= x.len() {
        return Err("window center index must be between 0 and n - 1");
    }

    let n = x.len() as isize;
    let iidx = idx as isize;
    let ih = h as isize;
    let ij = j as isize;

    let (idx1, idx2) = if endtype == MovStatEndType::Truncate {
        (
            max(iidx - ih, 0),
            min(iidx + ij, n - 1),
        )
    } else {
        (iidx - ih, iidx + ij)
    };

    let window_size = (idx2 - idx1 + 1) as usize;

    for (widx, j) in (idx1..=idx2).enumerate() {
        window[widx] = if j < 0 {
            match endtype {
                MovStatEndType::PadZero => 0.0,
                MovStatEndType::PadValue => x[0],
                _ => unreachable!(),
            }
        } else if j >= n {
            match endtype {
                MovStatEndType::PadZero => 0.0,
                MovStatEndType::PadValue => x[n as usize - 1],
                _ => unreachable!(),
            }
        } else {
            x[j as usize]
        };
    }

    Ok(window_size)
}