/* ========================================================================= */
/* === AMD_aat ============================================================= */
/* ========================================================================= */

/* ------------------------------------------------------------------------- */
/* AMD, Copyright (c) Timothy A. Davis,                                      */
/* Patrick R. Amestoy, and Iain S. Duff.  See ../README.txt for License.     */
/* email: davis at cise.ufl.edu    CISE Department, Univ. of Florida.        */
/* web: http://www.cise.ufl.edu/research/sparse/amd                          */
/* ------------------------------------------------------------------------- */

use std::f64;

/// Computes the symmetry of the pattern of A, and counts the number of
/// nonzeros in each column of A+A' (excluding the diagonal).
/// Assumes the input matrix has no errors, with sorted columns and no duplicates.
pub fn amd_aat(
    n: usize,
    ap: &[usize],
    ai: &[usize],
    len: &mut [usize],
    tp: &mut [usize],
    info: Option<&mut [f64]>,
) -> usize {
    // Constants
    const EMPTY: f64 = -1.0;
    const AMD_STATUS: usize = 0;
    const AMD_N: usize = 1;
    const AMD_NZ: usize = 2;
    const AMD_SYMMETRY: usize = 3;
    const AMD_NZDIAG: usize = 4;
    const AMD_NZ_A_PLUS_AT: usize = 5;
    const AMD_INFO: usize = 6;

    // Initialize variables
    let mut nzdiag = 0;
    let mut nzboth = 0;
    let nz = ap[n];
    let mut nzaat = 0;

    // Initialize workspace
    for k in 0..n {
        tp[k] = usize::MAX; // Using MAX as EMPTY equivalent
    }

    // Clear info array if it exists
    if let Some(info) = info {
        for i in 0..AMD_INFO {
            info[i] = EMPTY;
        }
        info[AMD_STATUS] = 1.0; // AMD_OK
    }

    // Initialize Len array
    for k in 0..n {
        len[k] = 0;
    }

    // Main computation loop
    for k in 0..n {
        let p1 = ap[k];
        let p2 = ap[k + 1];

        // Construct A+A'
        let mut p = p1;
        while p < p2 {
            // Scan the upper triangular part of A
            let j = ai[p];
            if j < k {
                // Entry A(j,k) is in strictly upper triangular part
                // Add both A(j,k) and A(k,j) to A+A'
                len[j] += 1;
                len[k] += 1;
                p += 1;
            } else if j == k {
                // Skip the diagonal
                p += 1;
                nzdiag += 1;
                break;
            } else {
                // First entry below the diagonal
                break;
            }

            // Scan lower triangular part of A, in column j until reaching row k
            assert_ne!(tp[j], usize::MAX);
            assert!(ap[j] <= tp[j] && tp[j] <= ap[j + 1]);

            let pj2 = ap[j + 1];
            let mut pj = tp[j];
            while pj < pj2 {
                let i = ai[pj];
                if i < k {
                    // A(i,j) is only in lower part, not in upper
                    // Add both A(i,j) and A(j,i) to A+A'
                    len[i] += 1;
                    len[j] += 1;
                    pj += 1;
                } else if i == k {
                    // Entry A(k,j) in lower part and A(j,k) in upper
                    pj += 1;
                    nzboth += 1;
                    break;
                } else {
                    // Consider this entry later when k advances to i
                    break;
                }
            }
            tp[j] = pj;
        }
        // Tp[k] points to entry just below diagonal in column k
        tp[k] = p;
    }

    // Clean up remaining mismatched entries
    for j in 0..n {
        let mut pj = tp[j];
        while pj < ap[j + 1] {
            let i = ai[pj];
            // A(i,j) is only in lower part, not in upper
            // Add both A(i,j) and A(j,i) to A+A'
            len[i] += 1;
            len[j] += 1;
            pj += 1;
        }
    }

    // Compute symmetry of the nonzero pattern of A
    let sym = if nz == nzdiag {
        1.0
    } else {
        (2.0 * nzboth as f64) / ((nz - nzdiag) as f64)
    };

    // Compute total nonzeros in A+A' excluding diagonal
    nzaat = len.iter().sum();

    // Update info array if it exists
    if let Some(info) = info {
        info[AMD_STATUS] = 1.0; // AMD_OK
        info[AMD_N] = n as f64;
        info[AMD_NZ] = nz as f64;
        info[AMD_SYMMETRY] = sym;
        info[AMD_NZDIAG] = nzdiag as f64;
        info[AMD_NZ_A_PLUS_AT] = nzaat as f64;
    }

    nzaat
}