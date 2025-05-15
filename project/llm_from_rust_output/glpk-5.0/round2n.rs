pub fn glp_round2n(x: f64) -> f64 {
    assert!(x > 0.0, "x must be positive");
    
    let (f, e) = x.frexp();
    let exp = if f <= 0.75 { e - 1 } else { e };
    f64::ldexp(1.0, exp)
}