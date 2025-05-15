use rand::Rng;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct DiscreteError {
    details: String,
}

impl DiscreteError {
    fn new(msg: &str) -> DiscreteError {
        DiscreteError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DiscreteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DiscreteError {
    fn description(&self) -> &str {
        &self.details
    }
}

struct Stack {
    v: Vec<usize>,
}

impl Stack {
    fn new(size: usize) -> Stack {
        Stack {
            v: Vec::with_capacity(size),
        }
    }

    fn push(&mut self, value: usize) -> Result<(), DiscreteError> {
        if self.v.len() >= self.v.capacity() {
            return Err(DiscreteError::new("stack overflow"));
        }
        self.v.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<usize, DiscreteError> {
        self.v.pop().ok_or_else(|| DiscreteError::new("stack exhausted"))
    }

    fn size(&self) -> usize {
        self.v.len()
    }
}

pub struct Discrete {
    k: usize,
    f: Vec<f64>,
    a: Vec<usize>,
}

impl Discrete {
    pub fn new(kevents: usize, prob_array: &[f64]) -> Result<Discrete, DiscreteError> {
        if kevents < 1 {
            return Err(DiscreteError::new(
                "number of events must be a positive integer",
            ));
        }

        for &p in prob_array {
            if p < 0.0 {
                return Err(DiscreteError::new("probabilities must be non-negative"));
            }
        }

        let p_total: f64 = prob_array.iter().sum();
        let mut e: Vec<f64> = prob_array.iter().map(|&p| p / p_total).collect();

        let mut f = vec![0.0; kevents];
        let mut a = vec![0; kevents];

        let mean = 1.0 / kevents as f64;
        let mut bigs = Stack::new(kevents);
        let mut smalls = Stack::new(kevents);

        for (k, &ek) in e.iter().enumerate() {
            if ek < mean {
                smalls.push(k)?;
            } else {
                bigs.push(k)?;
            }
        }

        while smalls.size() > 0 {
            let s = smalls.pop()?;
            if bigs.size() == 0 {
                a[s] = s;
                f[s] = 1.0;
                continue;
            }

            let b = bigs.pop()?;
            a[s] = b;
            f[s] = kevents as f64 * e[s];

            let d = mean - e[s];
            e[s] += d;
            e[b] -= d;

            if e[b] < mean {
                smalls.push(b)?;
            } else if e[b] > mean {
                bigs.push(b)?;
            } else {
                a[b] = b;
                f[b] = 1.0;
            }
        }

        while bigs.size() > 0 {
            let b = bigs.pop()?;
            a[b] = b;
            f[b] = 1.0;
        }

        // Apply Knuth convention
        for k in 0..kevents {
            f[k] = (k as f64 + f[k]) / kevents as f64;
        }

        Ok(Discrete { k: kevents, f, a })
    }

    pub fn sample<R: Rng>(&self, rng: &mut R) -> usize {
        let u = rng.gen::<f64>();
        let c = (u * self.k as f64) as usize;
        let f = self.f[c];

        if u < f {
            c
        } else {
            self.a[c]
        }
    }

    pub fn pdf(&self, k: usize) -> f64 {
        if k >= self.k {
            return 0.0;
        }

        let mut p = 0.0;
        for i in 0..self.k {
            let mut f = self.f[i];
            f = self.k as f64 * f - i as f64;

            if i == k {
                p += f;
            } else if k == self.a[i] {
                p += 1.0 - f;
            }
        }
        p / self.k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_discrete() {
        let probs = vec![0.1, 0.2, 0.3, 0.4];
        let d = Discrete::new(4, &probs).unwrap();
        let mut rng = thread_rng();

        for _ in 0..100 {
            let s = d.sample(&mut rng);
            assert!(s < 4);
        }

        assert!((d.pdf(0) - 0.1).abs() < 1e-10);
        assert!((d.pdf(1) - 0.2).abs() < 1e-10);
        assert!((d.pdf(2) - 0.3).abs() < 1e-10);
        assert!((d.pdf(3) - 0.4).abs() < 1e-10);
    }
}