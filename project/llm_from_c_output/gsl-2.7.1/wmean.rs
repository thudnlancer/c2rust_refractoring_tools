// Weighted mean calculation for different floating-point types

pub mod long_double {
    pub fn weighted_mean(data: &[f64], weights: &[f64]) -> Option<f64> {
        if data.len() != weights.len() || data.is_empty() {
            return None;
        }
        
        let mut sum = 0.0;
        let mut sum_weights = 0.0;
        
        for (&d, &w) in data.iter().zip(weights.iter()) {
            sum += d * w;
            sum_weights += w;
        }
        
        if sum_weights == 0.0 {
            None
        } else {
            Some(sum / sum_weights)
        }
    }
}

pub mod double {
    pub fn weighted_mean(data: &[f64], weights: &[f64]) -> Option<f64> {
        if data.len() != weights.len() || data.is_empty() {
            return None;
        }
        
        let mut sum = 0.0;
        let mut sum_weights = 0.0;
        
        for (&d, &w) in data.iter().zip(weights.iter()) {
            sum += d * w;
            sum_weights += w;
        }
        
        if sum_weights == 0.0 {
            None
        } else {
            Some(sum / sum_weights)
        }
    }
}

pub mod float {
    pub fn weighted_mean(data: &[f32], weights: &[f32]) -> Option<f32> {
        if data.len() != weights.len() || data.is_empty() {
            return None;
        }
        
        let mut sum = 0.0;
        let mut sum_weights = 0.0;
        
        for (&d, &w) in data.iter().zip(weights.iter()) {
            sum += f64::from(d) * f64::from(w);
            sum_weights += f64::from(w);
        }
        
        if sum_weights == 0.0 {
            None
        } else {
            Some((sum / sum_weights) as f32)
        }
    }
}