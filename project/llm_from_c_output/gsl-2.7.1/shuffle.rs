use rand::{Rng, seq::SliceRandom};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct GslError {
    message: String,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GslError {}

const GSL_SUCCESS: i32 = 0;
const GSL_EINVAL: i32 = 1;

/// Randomly permute (shuffle) the elements in the slice
pub fn gsl_ran_shuffle<T: Clone, R: Rng + ?Sized>(rng: &mut R, data: &mut [T]) {
    data.shuffle(rng);
}

/// Choose k distinct elements from src and copy them to dest, preserving order
pub fn gsl_ran_choose<T: Clone, R: Rng + ?Sized>(
    rng: &mut R,
    dest: &mut [T],
    src: &[T],
) -> Result<(), GslError> {
    if dest.len() > src.len() {
        return Err(GslError {
            message: "k is greater than n, cannot sample more than n items".to_string(),
        });
    }

    let mut j = 0;
    for (i, item) in src.iter().enumerate() {
        if (src.len() - i) as f64 * rng.gen::<f64>() < (dest.len() - j) as f64 {
            dest[j] = item.clone();
            j += 1;
            if j == dest.len() {
                break;
            }
        }
    }

    Ok(())
}

/// Sample k elements from src with replacement and copy them to dest
pub fn gsl_ran_sample<T: Clone, R: Rng + ?Sized>(rng: &mut R, dest: &mut [T], src: &[T]) {
    for item in dest.iter_mut() {
        let j = rng.gen_range(0..src.len());
        *item = src[j].clone();
    }
}