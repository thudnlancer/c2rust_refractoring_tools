pub fn cblas_sswap(
    n: i32,
    x: &mut [f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
) {
    assert!(n >= 0, "n must be non-negative");
    assert!(inc_x != 0, "inc_x must not be zero");
    assert!(inc_y != 0, "inc_y must not be zero");
    
    let mut ix = if inc_x > 0 {
        0
    } else {
        (n - 1) * -inc_x
    };
    
    let mut iy = if inc_y > 0 {
        0
    } else {
        (n - 1) * -inc_y
    };
    
    for _ in 0..n {
        let x_idx = ix as usize;
        let y_idx = iy as usize;
        
        assert!(x_idx < x.len(), "x index out of bounds");
        assert!(y_idx < y.len(), "y index out of bounds");
        
        x.swap(x_idx, y_idx);
        
        ix += inc_x;
        iy += inc_y;
    }
}