use num_traits::{Float, NumCast, ToPrimitive};
use std::cmp::Ordering;

pub type size_t = usize;

pub fn gsl_stats_Sn0_from_sorted_data<T: Float + ToPrimitive>(
    sorted_data: &[T],
    stride: usize,
    n: usize,
    work: &mut [T],
) -> T {
    let np1_2 = ((n + 1) / 2) as i32;
    let half_n = n / 2;

    work[0] = sorted_data[half_n * stride] - sorted_data[0];

    for i in 2..=np1_2 {
        let i_usize = i as usize;
        let n_a = i - 1;
        let n_b = (n - i_usize) as i32;
        let diff = n_b - n_a;

        let mut left_b = 1;
        let mut left_a = left_b;
        let mut right_b = n_b;
        let mut right_a = right_b;

        let amin = diff / 2 + 1;
        let amax = diff / 2 + n_a;

        while left_a < right_a {
            let length = right_a - left_a + 1;
            let even = 1 - length % 2;
            let half = (length - 1) / 2;
            let try_a = left_a + half;
            let try_b = left_b + half;

            if try_a < amin {
                right_b = try_b;
                left_a = try_a + even;
            } else if try_a > amax {
                right_a = try_a;
                left_b = try_b + even;
            } else {
                let med_a = sorted_data[(i_usize - 1) * stride] 
                    - sorted_data[(i_usize - try_a as usize + amin as usize - 2) * stride];
                let med_b = sorted_data[(try_b as usize + i_usize - 1) * stride] 
                    - sorted_data[(i_usize - 1) * stride];

                if med_a >= med_b {
                    right_a = try_a;
                    left_b = try_b + even;
                } else {
                    right_b = try_b;
                    left_a = try_a + even;
                }
            }
        }

        if left_a > amax {
            work[i_usize - 1] = sorted_data[(left_b as usize + i_usize - 1) * stride] 
                - sorted_data[(i_usize - 1) * stride];
        } else {
            let med_a = sorted_data[(i_usize - 1) * stride] 
                - sorted_data[(i_usize - left_a as usize + amin as usize - 2) * stride];
            let med_b = sorted_data[(left_b as usize + i_usize - 1) * stride] 
                - sorted_data[(i_usize - 1) * stride];
            work[i_usize - 1] = if med_a < med_b { med_a } else { med_b };
        }
    }

    for i in (np1_2 + 1)..=(n as i32 - 1) {
        let i_usize = i as usize;
        let n_a = (n - i_usize) as i32;
        let n_b = i - 1;
        let diff = n_b - n_a;

        let mut left_b = 1;
        let mut left_a = left_b;
        let mut right_b = n_b;
        let mut right_a = right_b;

        let amin = diff / 2 + 1;
        let amax = diff / 2 + n_a;

        while left_a < right_a {
            let length = right_a - left_a + 1;
            let even = 1 - length % 2;
            let half = (length - 1) / 2;
            let try_a = left_a + half;
            let try_b = left_b + half;

            if try_a < amin {
                right_b = try_b;
                left_a = try_a + even;
            } else if try_a > amax {
                right_a = try_a;
                left_b = try_b + even;
            } else {
                let med_a = sorted_data[(i_usize + try_a as usize - amin as usize) * stride] 
                    - sorted_data[(i_usize - 1) * stride];
                let med_b = sorted_data[(i_usize - 1) * stride] 
                    - sorted_data[(i_usize - try_b as usize - 1) * stride];

                if med_a >= med_b {
                    right_a = try_a;
                    left_b = try_b + even;
                } else {
                    right_b = try_b;
                    left_a = try_a + even;
                }
            }
        }

        if left_a > amax {
            work[i_usize - 1] = sorted_data[(i_usize - 1) * stride] 
                - sorted_data[(i_usize - left_b as usize - 1) * stride];
        } else {
            let med_a = sorted_data[(i_usize + left_a as usize - amin as usize) * stride] 
                - sorted_data[(i_usize - 1) * stride];
            let med_b = sorted_data[(i_usize - 1) * stride] 
                - sorted_data[(i_usize - left_b as usize - 1) * stride];
            work[i_usize - 1] = if med_a < med_b { med_a } else { med_b };
        }
    }

    work[n - 1] = sorted_data[(n - 1) * stride] - sorted_data[(np1_2 as usize - 1) * stride];

    work.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    work[np1_2 as usize - 1]
}

pub fn gsl_stats_Sn_from_sorted_data<T: Float + ToPrimitive>(
    sorted_data: &[T],
    stride: usize,
    n: usize,
    work: &mut [T],
) -> f64 {
    let scale = 1.1926;
    let sn0 = gsl_stats_Sn0_from_sorted_data(sorted_data, stride, n, work).to_f64().unwrap();
    let cn = if n <= 9 {
        match n {
            2 => 0.743,
            3 => 1.851,
            4 => 0.954,
            5 => 1.351,
            6 => 0.993,
            7 => 1.198,
            8 => 1.005,
            9 => 1.131,
            _ => 1.0,
        }
    } else if n % 2 == 1 {
        n as f64 / (n as f64 - 0.9)
    } else {
        1.0
    };

    scale * cn * sn0
}

// Implementations for other types would follow similar patterns, using generics
// and trait bounds to handle the different numeric types safely.