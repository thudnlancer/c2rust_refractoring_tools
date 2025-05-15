use std::ptr;

/* ========================================================================= */
/* === AMD_1 =============================================================== */
/* ========================================================================= */

/* ------------------------------------------------------------------------- */
/* AMD, Copyright (c) Timothy A. Davis,                                      */
/* Patrick R. Amestoy, and Iain S. Duff.  See ../README.txt for License.     */
/* email: davis at cise.ufl.edu    CISE Department, Univ. of Florida.        */
/* web: http://www.cise.ufl.edu/research/sparse/amd                          */
/* ------------------------------------------------------------------------- */

/// Construct A+A' for a sparse matrix A and perform the AMD ordering.
///
/// The n-by-n sparse matrix A can be unsymmetric. It is stored in MATLAB-style
/// compressed-column form, with sorted row indices in each column, and no
/// duplicate entries. Diagonal entries may be present, but they are ignored.
/// Row indices of column j of A are stored in Ai[Ap[j]..Ap[j+1]].
/// Ap[0] must be zero, and nz = Ap[n] is the number of entries in A. The
/// size of the matrix, n, must be greater than or equal to zero.
///
/// This routine must be preceded by a call to AMD_aat, which computes the
/// number of entries in each row/column in A+A', excluding the diagonal.
/// Len[j], on input, is the number of entries in row/column j of A+A'. This
/// routine constructs the matrix A+A' and then calls AMD_2. No error checking
/// is performed (this was done in AMD_valid).
pub fn amd_1(
    n: usize,
    ap: &[usize],
    ai: &[usize],
    p: &mut [usize],
    pinv: &mut [usize],
    len: &mut [usize],
    slen: usize,
    s: &mut [usize],
    control: &[f64],
    info: &mut [f64],
) {
    assert!(n > 0);

    let iwlen = slen - 6 * n;
    let (pe, rest) = s.split_at_mut(n);
    let (nv, rest) = rest.split_at_mut(n);
    let (head, rest) = rest.split_at_mut(n);
    let (elen, rest) = rest.split_at_mut(n);
    let (degree, rest) = rest.split_at_mut(n);
    let (w, rest) = rest.split_at_mut(n);
    let iw = rest;

    // Use nv and w as workspace for sp and tp
    let sp = nv;
    let tp = w;
    let mut pfree = 0;

    for j in 0..n {
        pe[j] = pfree;
        sp[j] = pfree;
        pfree += len[j];
    }

    assert!(iwlen >= pfree + n);

    #[cfg(debug_assertions)]
    for p in 0..iwlen {
        iw[p] = usize::MAX; // EMPTY
    }

    for k in 0..n {
        let p1 = ap[k];
        let p2 = ap[k + 1];

        // Construct A+A'
        let mut p = p1;
        while p < p2 {
            // Scan the upper triangular part of A
            let j = ai[p];
            assert!(j < n);
            if j < k {
                // Entry A(j,k) in the strictly upper triangular part
                assert!(sp[j] < if j == n - 1 { pfree } else { pe[j + 1] });
                assert!(sp[k] < if k == n - 1 { pfree } else { pe[k + 1] });
                iw[sp[j]] = k;
                sp[j] += 1;
                iw[sp[k]] = j;
                sp[k] += 1;
                p += 1;
            } else if j == k {
                // Skip the diagonal
                p += 1;
                break;
            } else {
                // First entry below the diagonal
                break;
            }

            // Scan lower triangular part of A, in column j until reaching row k
            assert!(ap[j] <= tp[j] && tp[j] <= ap[j + 1]);
            let pj2 = ap[j + 1];
            let mut pj = tp[j];
            while pj < pj2 {
                let i = ai[pj];
                assert!(i < n);
                if i < k {
                    // A(i,j) is only in the lower part, not in upper
                    assert!(sp[i] < if i == n - 1 { pfree } else { pe[i + 1] });
                    assert!(sp[j] < if j == n - 1 { pfree } else { pe[j + 1] });
                    iw[sp[i]] = j;
                    sp[i] += 1;
                    iw[sp[j]] = i;
                    sp[j] += 1;
                    pj += 1;
                } else if i == k {
                    // Entry A(k,j) in lower part and A(j,k) in upper
                    pj += 1;
                    break;
                } else {
                    // Consider this entry later, when k advances to i
                    break;
                }
            }
            tp[j] = pj;
        }
        tp[k] = p;
    }

    // Clean up remaining mismatched entries
    for j in 0..n {
        let mut pj = tp[j];
        while pj < ap[j + 1] {
            let i = ai[pj];
            assert!(i < n);
            // A(i,j) is only in the lower part, not in upper
            assert!(sp[i] < if i == n - 1 { pfree } else { pe[i + 1] });
            assert!(sp[j] < if j == n - 1 { pfree } else { pe[j + 1] });
            iw[sp[i]] = j;
            sp[i] += 1;
            iw[sp[j]] = i;
            sp[j] += 1;
            pj += 1;
        }
    }

    #[cfg(debug_assertions)]
    {
        for j in 0..n - 1 {
            assert!(sp[j] == pe[j + 1]);
        }
        assert!(sp[n - 1] == pfree);
    }

    // Order the matrix
    amd_2(
        n,
        pe,
        iw,
        len,
        iwlen,
        pfree,
        nv,
        pinv,
        p,
        head,
        elen,
        degree,
        w,
        control,
        info,
    );
}

// Placeholder for AMD_2 function
fn amd_2(
    _n: usize,
    _pe: &[usize],
    _iw: &[usize],
    _len: &mut [usize],
    _iwlen: usize,
    _pfree: usize,
    _nv: &mut [usize],
    _pinv: &mut [usize],
    _p: &mut [usize],
    _head: &mut [usize],
    _elen: &mut [usize],
    _degree: &mut [usize],
    _w: &mut [usize],
    _control: &[f64],
    _info: &mut [f64],
) {
    unimplemented!()
}