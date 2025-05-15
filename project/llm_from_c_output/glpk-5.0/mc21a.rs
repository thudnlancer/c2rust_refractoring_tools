// mc21a.rs (permutations for zero-free diagonal)

/* 
 * This code is a translation of the original C code from GLPK (GNU Linear Programming Kit).
 * The original C code was written by Andrew Makhorin <mao@gnu.org>.
 * 
 * The Rust translation maintains the same functionality while adhering to Rust's safety practices.
 * 
 * Original copyright notice:
 * Copyright (C) 2009-2013 Free Software Foundation, Inc.
 * GLPK is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 */

/// Finds permutations for zero-free diagonal in a sparse matrix.
///
/// Given the pattern of nonzeros of a sparse matrix, this function attempts
/// to find a permutation of its rows that makes the matrix have no zeros
/// on its diagonal.
///
/// # Arguments
///
/// * `n` - Order of matrix.
/// * `icn` - Array containing column indices of non-zeros. Column indices
///           belonging to a single row must be contiguous.
/// * `ip` - `ip[i]` is the position in array `icn` of the first column index
///          of a non-zero in row i (1-based).
/// * `lenr` - `lenr[i]` is the number of non-zeros in row i.
/// * `iperm` - Output array that will contain the permutation to make diagonal
///             have the smallest number of zeros.
/// * `pr` - Working array of length n+1 (index 0 unused).
/// * `arp` - Working array of length n+1 (index 0 unused).
/// * `cv` - Working array of length n+1 (index 0 unused).
/// * `out` - Working array of length n+1 (index 0 unused).
///
/// # Returns
///
/// The number of non-zeros on diagonal of permuted matrix.
pub fn mc21a(
    n: usize,
    icn: &[usize],
    ip: &[usize],
    lenr: &[usize],
    iperm: &mut [usize],
    pr: &mut [usize],
    arp: &mut [usize],
    cv: &mut [usize],
    out: &mut [usize],
) -> usize {
    // Initialization of arrays
    for i in 1..=n {
        arp[i] = lenr[i] - 1;
        cv[i] = 0;
        iperm[i] = 0;
    }
    let mut numnz = 0;

    // Main loop
    for jord in 1..=n {
        let mut j = jord;
        pr[j] = 0; // Using 0 instead of -1 since we're using usize
        for _ in 1..=jord {
            // Look for a cheap assignment
            let mut in1 = arp[j];
            if in1 != usize::MAX { // Check for >= 0 equivalent
                let in2 = ip[j] + lenr[j] - 1;
                in1 = in2 - in1;
                for ii in in1..=in2 {
                    let i = icn[ii];
                    if iperm[i] == 0 {
                        // New assignment is made
                        iperm[i] = j;
                        arp[j] = in2 - ii - 1;
                        numnz += 1;
                        let mut k = j;
                        for _ in 1..=jord {
                            k = pr[k];
                            if k == 0 {
                                break;
                            }
                            let ii = ip[k] + lenr[k] - out[k] - 2;
                            let i = icn[ii];
                            iperm[i] = k;
                        }
                        continue;
                    }
                }
                // No cheap assignment in row
                arp[j] = usize::MAX; // Set to -1 equivalent
            }

            // Begin looking for assignment chain starting with row j
            out[j] = lenr[j] - 1;

            // Inner loop - extends chain by one or backtracks
            for _ in 1..=jord {
                let mut in1 = out[j];
                if in1 != usize::MAX { // Check for >= 0 equivalent
                    let in2 = ip[j] + lenr[j] - 1;
                    in1 = in2 - in1;
                    // Forward scan
                    for ii in in1..=in2 {
                        let i = icn[ii];
                        if cv[i] != jord {
                            // Column i has not yet been accessed during this pass
                            let j1 = j;
                            j = iperm[i];
                            cv[i] = jord;
                            pr[j] = j1;
                            out[j1] = in2 - ii - 1;
                            continue;
                        }
                    }
                }
                // Backtracking step
                j = pr[j];
                if j == 0 {
                    break;
                }
            }
        }
    }

    // If matrix is structurally singular, complete the permutation iperm
    if numnz < n {
        for i in 1..=n {
            arp[i] = 0;
        }
        let mut k = 0;
        let mut out_ptr = 1;
        
        for i in 1..=n {
            if iperm[i] == 0 {
                out[out_ptr] = i;
                out_ptr += 1;
            } else {
                arp[iperm[i]] = i;
            }
        }
        
        k = 0;
        for i in 1..=n {
            if arp[i] == 0 {
                k += 1;
                iperm[out[k]] = i;
            }
        }
    }

    numnz
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn fa01bs(max: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=max)
    }

    fn ranmat(
        m: usize,
        n: usize,
        icn: &mut [usize],
        iptr: &mut [usize],
        nnnp1: usize,
        knum: &mut usize,
        iw: &mut [usize],
        sing: bool,
    ) {
        let mut inum = (*knum / n) * 2;
        if inum > n - 1 {
            inum = n - 1;
        }
        let mut matnum = 1;

        for j in 1..=m {
            iptr[j] = matnum;
            if !sing && j <= n {
                icn[matnum] = j;
                matnum += 1;
            }
            if n == 1 {
                continue;
            }

            for i in 1..=n {
                iw[i] = 0;
            }
            if !sing {
                iw[j] = 1;
            }

            let lrow = fa01bs(inum);
            let lrow = lrow - 1;
            if lrow == 0 {
                continue;
            }

            for _ in 1..=lrow {
                loop {
                    let i = fa01bs(n);
                    if iw[i] != 1 {
                        iw[i] = 1;
                        icn[matnum] = i;
                        matnum += 1;
                        break;
                    }
                }
            }
        }

        for i in m + 1..=nnnp1 {
            iptr[i] = matnum;
        }
        *knum = matnum - 1;
    }

    #[test]
    fn test_mc21a() {
        let licn = 1000;
        for n in 1..=20 {
            let mut nov4 = n / 4;
            if nov4 < 1 {
                nov4 = 1;
            }

            let l = fa01bs(nov4);
            let mut knum = l * n;
            if knum > licn {
                continue;
            }

            let sing = (n / 2) * 2 == n;
            let mut icn = vec![0; licn + 1];
            let mut ip = vec![0; n + 2];
            let mut iw1 = vec![0; 4 * n + 1];

            ranmat(n, n, &mut icn, &mut ip, n + 1, &mut knum, &mut iw1, sing);
            if knum > licn {
                continue;
            }

            let mut lenr = vec![0; n + 1];
            for i in 1..=n {
                lenr[i] = ip[i + 1] - ip[i];
            }

            let mut iperm = vec![0; n + 1];
            let mut pr = vec![0; n + 1];
            let mut arp = vec![0; n + 1];
            let mut cv = vec![0; n + 1];
            let mut out = vec![0; n + 1];

            let numnz = mc21a(
                n,
                &icn,
                &ip,
                &lenr,
                &mut iperm,
                &mut pr,
                &mut arp,
                &mut cv,
                &mut out,
            );

            let mut num = 0;
            for i in 1..=n {
                let iold = iperm[i];
                let j1 = ip[iold];
                let j2 = j1 + lenr[iold] - 1;
                if j2 < j1 {
                    continue;
                }
                for jj in j1..=j2 {
                    let j = icn[jj];
                    if j == i {
                        num += 1;
                        break;
                    }
                }
            }

            assert_eq!(num, numnz, "Failure in mc21a, numnz = {} instead of {}", numnz, num);
        }
    }
}