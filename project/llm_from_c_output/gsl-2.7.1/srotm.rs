use std::error::Error;
use std::result::Result;

pub fn cblas_srotm(
    n: i32,
    x: &mut [f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
    p: &[f32; 5],
) -> Result<(), Box<dyn Error>> {
    if n <= 0 {
        return Ok(());
    }

    if inc_x == 1 && inc_y == 1 {
        for i in 0..n as usize {
            let xi = x[i];
            let yi = y[i];
            x[i] = p[0] * xi + p[2] * yi;
            y[i] = p[1] * xi + p[3] * yi;
        }
    } else {
        let mut ix = if inc_x > 0 { 0 } else { (1 - n as isize) * inc_x as isize } as usize;
        let mut iy = if inc_y > 0 { 0 } else { (1 - n as isize) * inc_y as isize } as usize;

        for _ in 0..n {
            let xi = x[ix];
            let yi = y[iy];
            x[ix] = p[0] * xi + p[2] * yi;
            y[iy] = p[1] * xi + p[3] * yi;
            ix = (ix as isize + inc_x as isize) as usize;
            iy = (iy as isize + inc_y as isize) as usize;
        }
    }

    Ok(())
}