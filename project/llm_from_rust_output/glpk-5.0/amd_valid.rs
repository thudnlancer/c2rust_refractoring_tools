use std::cmp::Ordering;

#[no_mangle]
pub fn _glp_amd_valid(n_row: i32, n_col: i32, ap: &[i32], ai: &[i32]) -> i32 {
    if n_row < 0 || n_col < 0 {
        return -2;
    }

    if ap.is_empty() || ai.is_empty() {
        return -2;
    }

    let nz = *ap.last().unwrap();
    if ap[0] != 0 || nz < 0 {
        return -2;
    }

    let mut result = 0;

    for j in 0..n_col as usize {
        let p1 = ap[j];
        let p2 = ap[j + 1];

        if p1 > p2 {
            return -2;
        }

        let mut ilast = -1;

        for p in p1..p2 {
            let i = ai[p as usize];
            if i < 0 || i >= n_row {
                return -2;
            }
            if i <= ilast {
                result = 1;
            }
            ilast = i;
        }
    }

    result
}