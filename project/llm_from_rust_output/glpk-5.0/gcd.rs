pub fn glp_gcd(x: i32, y: i32) -> i32 {
    assert!(x > 0 && y > 0, "x > 0 && y > 0");
    
    let mut x = x;
    let mut y = y;
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

pub fn glp_gcdn(n: i32, x: &[i32]) -> i32 {
    assert!(n > 0, "n > 0");
    assert!(x.len() >= n as usize, "x array too small");
    
    let mut d = 0;
    for j in 0..n as usize {
        assert!(x[j] > 0, "x[j] > 0");
        
        if j == 0 {
            d = x[0];
        } else {
            d = glp_gcd(d, x[j]);
        }
        
        if d == 1 {
            break;
        }
    }
    d
}