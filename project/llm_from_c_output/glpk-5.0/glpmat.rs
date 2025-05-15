use std::f64;
use std::mem;
use std::ptr;

#[derive(Debug)]
pub enum MatrixError {
    InvalidDimension,
    InvalidPattern,
    NonPositiveDiagonal,
}

pub fn check_fvs(n: usize, nnz: usize, ind: &[usize], vec: &[f64]) -> Result<(), MatrixError> {
    if n == 0 {
        return Err(MatrixError::InvalidDimension);
    }
    if nnz > n {
        return Err(MatrixError::InvalidDimension);
    }

    let mut flag = vec![false; n + 1];

    for &i in &ind[1..=nnz] {
        if i == 0 || i > n {
            return Err(MatrixError::InvalidPattern);
        }
        if flag[i] {
            return Err(MatrixError::InvalidPattern);
        }
        flag[i] = true;
    }

    for i in 1..=n {
        if !flag[i] && vec[i] != 0.0 {
            return Err(MatrixError::InvalidPattern);
        }
    }

    Ok(())
}

pub fn check_pattern(m: usize, n: usize, a_ptr: &[usize], a_ind: &[usize]) -> Result<(), MatrixError> {
    if m == 0 {
        return Err(MatrixError::InvalidDimension);
    }
    if n == 0 {
        return Err(MatrixError::InvalidDimension);
    }
    if a_ptr[1] != 1 {
        return Err(MatrixError::InvalidPattern);
    }

    let mut flag = vec![false; n + 1];

    for i in 1..=m {
        for ptr in a_ptr[i]..a_ptr[i + 1] {
            let j = a_ind[ptr];
            if j == 0 || j > n {
                return Err(MatrixError::InvalidPattern);
            }
            if flag[j] {
                return Err(MatrixError::InvalidPattern);
            }
            flag[j] = true;
        }
        for ptr in a_ptr[i]..a_ptr[i + 1] {
            let j = a_ind[ptr];
            flag[j] = false;
        }
    }

    Ok(())
}

pub fn transpose(
    m: usize,
    n: usize,
    a_ptr: &[usize],
    a_ind: &[usize],
    a_val: Option<&[f64]>,
    at_ptr: &mut [usize],
    at_ind: &mut [usize],
    at_val: Option<&mut [f64]>,
) {
    // Determine row lengths of resultant matrix
    for j in 1..=n {
        at_ptr[j] = 0;
    }
    for i in 1..=m {
        for t in a_ptr[i]..a_ptr[i + 1] {
            at_ptr[a_ind[t]] += 1;
        }
    }

    // Set up row pointers of resultant matrix
    let mut pos = 1;
    for j in 1..=n {
        let len = at_ptr[j];
        at_ptr[j] = pos;
        pos += len;
    }
    at_ptr[n + 1] = pos;

    // Build resultant matrix
    for i in (1..=m).rev() {
        for t in a_ptr[i]..a_ptr[i + 1] {
            let j = a_ind[t];
            pos = at_ptr[j];
            at_ptr[j] = pos + 1;
            at_ind[pos] = i;
            if let (Some(a_val), Some(at_val)) = (a_val, at_val.as_ref()) {
                at_val[pos] = a_val[t];
            }
        }
    }
}

pub fn adat_symbolic(
    m: usize,
    n: usize,
    p_per: &[usize],
    a_ptr: &[usize],
    a_ind: &[usize],
    s_ptr: &mut [usize],
) -> Vec<usize> {
    // Build transpose of A
    let mut at_ptr = vec![0; n + 2];
    let mut at_ind = vec![0; a_ptr[m + 1] - 1];
    transpose(m, n, a_ptr, a_ind, None, &mut at_ptr, &mut at_ind, None);

    // Allocate S_ind
    let mut size = a_ptr[m + 1] - 1;
    if size < m {
        size = m;
    }
    let mut s_ind = vec![0; size + 1];

    // Working arrays
    let mut ind = vec![0; m + 1];
    let mut map = vec![0; m + 1];

    // Compute pattern of S
    s_ptr[1] = 1;
    for ii in 1..=m {
        let mut len = 0;
        let i = p_per[ii]; // i-th row of A = ii-th row of B

        for t in a_ptr[i]..a_ptr[i + 1] {
            let k = a_ind[t];
            // Walk through k-th column of A
            for tt in at_ptr[k]..at_ptr[k + 1] {
                let j = at_ind[tt];
                let jj = p_per[m + j]; // j-th row of A = jj-th row of B
                if ii < jj && map[jj] == 0 {
                    len += 1;
                    ind[len] = jj;
                    map[jj] = 1;
                }
            }
        }

        s_ptr[ii + 1] = s_ptr[ii] + len;
        if s_ptr[ii + 1] - 1 > size {
            size *= 2;
            s_ind.resize(size + 1, 0);
        }

        s_ind[s_ptr[ii]..s_ptr[ii + 1].copy_from_slice(&ind[1..=len]);

        // Clear the row pattern map
        for t in 1..=len {
            map[ind[t]] = 0;
        }
    }

    // Trim S_ind to actual size
    s_ind.truncate(s_ptr[m + 1]);
    s_ind
}

pub fn adat_numeric(
    m: usize,
    n: usize,
    p_per: &[usize],
    a_ptr: &[usize],
    a_ind: &[usize],
    a_val: &[f64],
    d_diag: &[f64],
    s_ptr: &[usize],
    s_ind: &[usize],
    s_val: &mut [f64],
    s_diag: &mut [f64],
) {
    let mut work = vec![0.0; n + 1];

    for ii in 1..=m {
        let i = p_per[ii]; // i-th row of A = ii-th row of B

        // work := i-th row of A
        for t in a_ptr[i]..a_ptr[i + 1] {
            work[a_ind[t]] = a_val[t];
        }

        // Compute ii-th row of S
        for t in s_ptr[ii]..s_ptr[ii + 1] {
            let jj = s_ind[t];
            let j = p_per[jj]; // j-th row of A = jj-th row of B
            let mut sum = 0.0;

            for tt in a_ptr[j]..a_ptr[j + 1] {
                let k = a_ind[tt];
                sum += work[k] * d_diag[k] * a_val[tt];
            }

            s_val[t] = sum;
        }

        // Compute diagonal element
        let mut sum = 0.0;
        for t in a_ptr[i]..a_ptr[i + 1] {
            let k = a_ind[t];
            sum += a_val[t] * d_diag[k] * a_val[t];
            work[k] = 0.0;
        }
        s_diag[ii] = sum;
    }
}

pub fn min_degree(n: usize, a_ptr: &[usize], a_ind: &[usize], p_per: &mut [usize]) {
    // This is a placeholder for the actual implementation
    // The original C code uses external QMD routines which would need to be ported
    unimplemented!("Minimum degree ordering requires porting QMD routines from C");
}

pub fn amd_order1(n: usize, a_ptr: &[usize], a_ind: &[usize], p_per: &mut [usize]) {
    // This is a placeholder for the actual implementation
    // The original C code uses external AMD routines which would need to be ported
    unimplemented!("AMD ordering requires porting AMD routines from C");
}

pub fn symamd_ord(n: usize, a_ptr: &[usize], a_ind: &[usize], p_per: &mut [usize]) {
    // This is a placeholder for the actual implementation
    // The original C code uses external COLAMD routines which would need to be ported
    unimplemented!("SYMAMD ordering requires porting COLAMD routines from C");
}

pub fn chol_symbolic(
    n: usize,
    a_ptr: &[usize],
    a_ind: &[usize],
    u_ptr: &mut [usize],
) -> Vec<usize> {
    let mut size = a_ptr[n + 1] - 1;
    if size < n {
        size = n;
    }
    size *= 2;
    let mut u_ind = vec![0; size + 1];

    let mut head = vec![0; n + 1];
    let mut next = vec![0; n + 1];
    let mut ind = vec![0; n + 1];
    let mut map = vec![0; n + 1];

    u_ptr[1] = 1;
    for k in 1..=n {
        // Compute pattern of k-th row of U
        let mut len = a_ptr[k + 1] - a_ptr[k];
        ind[1..=len].copy_from_slice(&a_ind[a_ptr[k]..a_ptr[k + 1]]);
        for t in 1..=len {
            let j = ind[t];
            map[j] = 1;
        }

        // Walk through rows of U whose leftmost non-zero is in column k
        let mut i = head[k];
        while i != 0 {
            for t in u_ptr[i]..u_ptr[i + 1] {
                let j = u_ind[t];
                if j > k && map[j] == 0 {
                    len += 1;
                    ind[len] = j;
                    map[j] = 1;
                }
            }
            i = next[i];
        }

        u_ptr[k + 1] = u_ptr[k] + len;
        if u_ptr[k + 1] - 1 > size {
            size *= 2;
            u_ind.resize(size + 1, 0);
        }

        u_ind[u_ptr[k]..u_ptr[k + 1]].copy_from_slice(&ind[1..=len]);

        // Find minimum column index and clear map
        let mut min_j = n + 1;
        for t in 1..=len {
            let j = ind[t];
            map[j] = 0;
            if j < min_j {
                min_j = j;
            }
        }

        // Include k-th row in linked list
        if min_j <= n {
            next[k] = head[min_j];
            head[min_j] = k;
        }
    }

    u_ind.truncate(u_ptr[n + 1] - 1);
    u_ind
}

pub fn chol_numeric(
    n: usize,
    a_ptr: &[usize],
    a_ind: &[usize],
    a_val: &[f64],
    a_diag: &[f64],
    u_ptr: &[usize],
    u_ind: &[usize],
    u_val: &mut [f64],
    u_diag: &mut [f64],
) -> usize {
    let mut work = vec![0.0; n + 1];
    let mut count = 0;

    // Initialize U with upper triangle of A
    for i in 1..=n {
        for t in a_ptr[i]..a_ptr[i + 1] {
            work[a_ind[t]] = a_val[t];
        }
        for t in u_ptr[i]..u_ptr[i + 1] {
            u_val[t] = work[u_ind[t]];
            work[u_ind[t]] = 0.0;
        }
        u_diag[i] = a_diag[i];
    }

    // Main elimination loop
    for k in 1..=n {
        // Transform k-th row of U
        let mut ukk = u_diag[k];
        if ukk > 0.0 {
            ukk = ukk.sqrt();
            u_diag[k] = ukk;
        } else {
            ukk = f64::MAX;
            u_diag[k] = ukk;
            count += 1;
        }

        // work := transformed k-th row
        for t in u_ptr[k]..u_ptr[k + 1] {
            let j = u_ind[t];
            u_val[t] /= ukk;
            work[j] = u_val[t];
        }

        // Transform other rows
        for t in u_ptr[k]..u_ptr[k + 1] {
            let i = u_ind[t];
            let uki = work[i];
            for t1 in u_ptr[i]..u_ptr[i + 1] {
                u_val[t1] -= uki * work[u_ind[t1]];
            }
            u_diag[i] -= uki * uki;
        }

        // Clear work
        for t in u_ptr[k]..u_ptr[k + 1] {
            work[u_ind[t]] = 0.0;
        }
    }

    count
}

pub fn u_solve(
    n: usize,
    u_ptr: &[usize],
    u_ind: &[usize],
    u_val: &[f64],
    u_diag: &[f64],
    x: &mut [f64],
) {
    for i in (1..=n).rev() {
        let mut temp = x[i];
        for t in u_ptr[i]..u_ptr[i + 1] {
            temp -= u_val[t] * x[u_ind[t]];
        }
        x[i] = temp / u_diag[i];
    }
}

pub fn ut_solve(
    n: usize,
    u_ptr: &[usize],
    u_ind: &[usize],
    u_val: &[f64],
    u_diag: &[f64],
    x: &mut [f64],
) {
    for i in 1..=n {
        x[i] /= u_diag[i];
        let temp = x[i];
        if temp == 0.0 {
            continue;
        }
        for t in u_ptr[i]..u_ptr[i + 1] {
            x[u_ind[t]] -= u_val[t] * temp;
        }
    }
}