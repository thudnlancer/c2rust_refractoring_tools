use std::mem;
use std::ptr;

pub fn cblas_cswap(n: i32, x: &mut [f32], inc_x: i32, y: &mut [f32], inc_y: i32) -> Result<(), &'static str> {
    if n <= 0 {
        return Ok(());
    }

    if inc_x == 0 || inc_y == 0 {
        return Err("Increment cannot be zero");
    }

    if x.len() < ((n - 1) * inc_x.abs() + 1) as usize * 2 {
        return Err("X vector too short");
    }

    if y.len() < ((n - 1) * inc_y.abs() + 1) as usize * 2 {
        return Err("Y vector too short");
    }

    let mut ix = if inc_x > 0 { 0 } else { (-(n - 1) * inc_x) as usize };
    let mut iy = if inc_y > 0 { 0 } else { (-(n - 1) * inc_y) as usize };

    for _ in 0..n {
        unsafe {
            ptr::swap_nonoverlapping(
                x.as_mut_ptr().add(ix * 2),
                y.as_mut_ptr().add(iy * 2),
                2,
            );
        }

        ix = (ix as i32 + inc_x) as usize;
        iy = (iy as i32 + inc_y) as usize;
    }

    Ok(())
}