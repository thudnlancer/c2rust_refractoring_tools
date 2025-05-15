// Rust implementation of Gastwirth's location estimator for various numeric types

use std::cmp::Ordering;

macro_rules! implement_gastwirth {
    ($type:ty) => {
        pub fn gastwirth(data: &[$type]) -> Option<$type> {
            let n = data.len();
            if n == 0 {
                return None;
            }

            let mut sorted = data.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let j = n / 3;
            let i = (n + 1) / 3;
            let k = (2 * n + 1) / 3;

            let lq = if i == j {
                sorted[i]
            } else {
                (sorted[i] + sorted[j]) / 2.0
            };

            let median = if n % 2 == 1 {
                sorted[n/2]
            } else {
                (sorted[n/2 - 1] + sorted[n/2]) / 2.0
            };

            let uq = if k == j {
                sorted[k]
            } else {
                (sorted[k] + sorted[j]) / 2.0
            };

            Some(0.3 * lq + 0.4 * median + 0.3 * uq)
        }
    };
}

implement_gastwirth!(f64);
implement_gastwirth!(f32);
implement_gastwirth!(i8);
implement_gastwirth!(i16);
implement_gastwirth!(i32);
implement_gastwirth!(i64);
implement_gastwirth!(u8);
implement_gastwirth!(u16);
implement_gastwirth!(u32);
implement_gastwirth!(u64);