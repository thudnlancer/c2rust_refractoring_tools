pub fn cblas_dswap(
    n: i32,
    x: &mut [f64],
    inc_x: i32,
    y: &mut [f64],
    inc_y: i32,
) {
    if n <= 0 || inc_x == 0 || inc_y == 0 {
        return;
    }

    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } as usize;
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;
    let inc_x = inc_x as usize;
    let inc_y = inc_y as usize;

    for _ in 0..n {
        x.swap(ix, iy);
        ix = ix.wrapping_add(inc_x);
        iy = iy.wrapping_add(inc_y);
    }
}