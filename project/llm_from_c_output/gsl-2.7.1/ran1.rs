/* rng/ran1.rs

This is a Rust implementation of the algorithm used in Numerical
Recipe's ran1 generator. It is MINSTD with a 32-element shuffle-box.
*/

const M: i64 = 2147483647;
const A: i64 = 16807;
const Q: i64 = 127773;
const R: i64 = 2836;

const N_SHUFFLE: usize = 32;
const N_DIV: i64 = 1 + 2147483646 / N_SHUFFLE as i64;

#[derive(Debug, Clone)]
pub struct Ran1State {
    x: i64,
    n: i64,
    shuffle: [i64; N_SHUFFLE],
}

impl Ran1State {
    pub fn new(seed: i64) -> Self {
        let mut state = Ran1State {
            x: 0,
            n: 0,
            shuffle: [0; N_SHUFFLE],
        };
        state.set(seed);
        state
    }

    fn set(&mut self, mut s: i64) {
        if s == 0 {
            s = 1; // default seed is 1
        }

        // Warm up the generator
        for _ in 0..8 {
            let h = s / Q;
            let t = A * (s - h * Q) - h * R;
            s = if t < 0 { t + M } else { t };
        }

        // Initialize the shuffle table
        for i in (0..N_SHUFFLE).rev() {
            let h = s / Q;
            let t = A * (s - h * Q) - h * R;
            s = if t < 0 { t + M } else { t };
            self.shuffle[i] = s;
        }

        self.x = s;
        self.n = s;
    }

    pub fn get(&mut self) -> i64 {
        let x = self.x;

        let h = x / Q;
        let t = A * (x - h * Q) - h * R;

        self.x = if t < 0 { t + M } else { t };

        let j = (self.n / N_DIV) as usize;
        self.n = self.shuffle[j];
        self.shuffle[j] = self.x;

        self.n
    }

    pub fn get_double(&mut self) -> f64 {
        const X_MAX: f32 = 1.0 - 1.2e-7; // Numerical Recipes version of 1-FLT_EPS

        let x = self.get() as f32 / 2147483647.0f32;
        if x > X_MAX {
            X_MAX as f64
        } else {
            x as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ran1() {
        let mut rng = Ran1State::new(1);
        let values: Vec<i64> = (0..10).map(|_| rng.get()).collect();
        assert_eq!(
            values,
            vec![
                1806547169,
                846930886,
                1681692777,
                1714636915,
                1957747793,
                424238335,
                719885386,
                1649760492,
                596516649,
                1189641421
            ]
        );

        let mut rng = Ran1State::new(1);
        let values: Vec<f64> = (0..10).map(|_| rng.get_double()).collect();
        let expected = vec![
            0.841088,
            0.394383,
            0.783099,
            0.798398,
            0.911647,
            0.197551,
            0.335215,
            0.768221,
            0.277775,
            0.553970,
        ];
        for (i, (a, b)) in values.iter().zip(expected.iter()).enumerate() {
            assert!((a - b).abs() < 1e-6, "Mismatch at index {}: {} vs {}", i, a, b);
        }
    }
}