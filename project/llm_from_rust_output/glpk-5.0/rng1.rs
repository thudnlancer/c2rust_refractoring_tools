use std::f64;

#[derive(Debug, Clone)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: usize,
}

impl RNG {
    pub fn next_rand(&mut self) -> i32 {
        // Implementation of _glp_rng_next_rand would go here
        // This is a placeholder since the original implementation wasn't provided
        0
    }

    pub fn unif_01(&mut self) -> f64 {
        let x = self.next_rand() as f64 / 2_147_483_647.0;
        assert!((0.0..=1.0).contains(&x), "0.0 <= x && x <= 1.0");
        x
    }

    pub fn uniform(&mut self, a: f64, b: f64) -> f64 {
        assert!(a < b, "a < b");
        let x = self.unif_01();
        let result = a * (1.0 - x) + b * x;
        assert!((a..=b).contains(&result), "a <= x && x <= b");
        result
    }
}