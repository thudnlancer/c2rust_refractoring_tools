// Weighted absolute deviation calculation for different floating-point types

pub mod long_double {
    pub fn wabsdev(w: &[f64], data: &[f64]) -> Result<f64, &'static str> {
        calculate_wabsdev(w, data)
    }
}

pub mod double {
    pub fn wabsdev(w: &[f64], data: &[f64]) -> Result<f64, &'static str> {
        calculate_wabsdev(w, data)
    }
}

pub mod float {
    pub fn wabsdev(w: &[f32], data: &[f32]) -> Result<f32, &'static str> {
        calculate_wabsdev(w, data)
    }
}

fn calculate_wabsdev<T>(w: &[T], data: &[T]) -> Result<T, &'static str> 
where
    T: num_traits::Float + std::iter::Sum,
{
    if w.len() != data.len() {
        return Err("w and data must have same length");
    }
    if w.is_empty() {
        return Err("empty input");
    }

    let wsum: T = w.iter().cloned().sum();
    if wsum.is_zero() {
        return Err("sum of weights is zero");
    }

    let mut wm = T::zero();
    for (&wi, &di) in w.iter().zip(data.iter()) {
        wm = wm + wi * di;
    }
    wm = wm / wsum;

    let mut wabsdev = T::zero();
    for (&wi, &di) in w.iter().zip(data.iter()) {
        wabsdev = wabsdev + wi * (di - wm).abs();
    }
    wabsdev = wabsdev / wsum;

    Ok(wabsdev)
}