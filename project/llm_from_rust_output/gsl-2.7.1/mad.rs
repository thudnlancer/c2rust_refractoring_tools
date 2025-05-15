use num_traits::{Float, ToPrimitive};

pub fn gsl_stats_mad0<T: ToPrimitive>(
    data: &[T],
    stride: usize,
    work: &mut [f64],
) -> f64 {
    for (i, val) in data.iter().enumerate().step_by(stride).take(work.len()) {
        work[i] = val.to_f64().unwrap();
    }
    let median = gsl_stats_median(work, 1);

    for (i, val) in data.iter().enumerate().step_by(stride).take(work.len()) {
        work[i] = (val.to_f64().unwrap() - median).abs();
    }
    gsl_stats_median(work, 1)
}

pub fn gsl_stats_mad<T: ToPrimitive>(
    data: &[T],
    stride: usize,
    work: &mut [f64],
) -> f64 {
    let mad0 = gsl_stats_mad0(data, stride, work);
    1.482602218505602f64 * mad0
}

// Helper function that assumes the data is already sorted
fn gsl_stats_median(sorted_data: &[f64], stride: usize) -> f64 {
    let n = sorted_data.len() / stride;
    if n == 0 {
        return 0.0;
    }

    if n % 2 == 1 {
        sorted_data[(n / 2) * stride]
    } else {
        (sorted_data[((n / 2) - 1) * stride] + sorted_data[(n / 2) * stride]) / 2.0
    }
}