use std::cmp::{max, min};
use std::f64;

pub struct TriangResult {
    pub size: usize,
    pub rn: Vec<usize>,
    pub cn: Vec<usize>,
}

pub fn triang(
    m: usize,
    n: usize,
    mat: impl Fn(Option<&()>, isize, &mut [usize], &mut [f64]) -> usize,
    info: Option<&()>,
    tol: f64,
) -> TriangResult {
    // Allocate working arrays
    let mut cind = vec![0; m + 1];
    let mut cval = vec![0.0; m + 1];
    let mut rind = vec![0; n + 1];
    let mut rval = vec![0.0; n + 1];
    let mut cnt = vec![0; m + 1];
    let mut ptr = vec![0; m + 1];
    let mut list = vec![0; n + 1];
    let mut prev = vec![0; n + 1];
    let mut next = vec![0; n + 1];
    let mut big = vec![0.0; n + 1];
    let mut flag = vec![0; n + 1]; // 0=inactive, 1=active, 2=active with singleton

    // Build linked lists of columns having equal lengths
    for len in ptr.iter_mut() {
        *len = 0;
    }
    for j in 1..=n {
        // Get j-th column
        let len = mat(info, -(j as isize), &mut cind, &mut cval);
        assert!(len <= m);
        
        // Add to beginning of list ptr[len]
        next[j] = ptr[len];
        ptr[len] = j;
        
        // Determine maximal magnitude in this column
        big[j] = cval[1..=len].iter().fold(0.0, |acc, &x| acc.max(x.abs()));
    }

    // Build doubly linked list of columns ordered by decreasing lengths
    let mut head = 0;
    for len in 0..=m {
        let mut j = ptr[len];
        while j != 0 {
            let next_j = next[j];
            // Add j-th column to beginning of list
            prev[j] = 0;
            next[j] = head;
            if head != 0 {
                prev[head] = j;
            }
            head = j;
            j = next_j;
        }
    }

    // Build initial singleton list
    let mut ns = 0;
    for i in 1..=m {
        // Get i-th row
        let len = mat(info, i as isize, &mut rind, &mut rval);
        assert!(len <= n);
        cnt[i] = len;

        if len == 1 {
            // a[i,j] is row singleton
            let j = rind[1];
            assert!(j >= 1 && j <= n);
            if flag[j] != 2 {
                // Include j-th column in singleton list
                flag[j] = 2;
                ns += 1;
                list[ns] = j;
            }
        }
    }

    // Main loop
    let mut size = 0;
    let mut rn = vec![0; min(m, n) + 1];
    let mut cn = vec![0; min(m, n) + 1];

    while head != 0 {
        if ns == 0 {
            // Singleton list empty - remove column of maximal length
            let j = head;
            let len = mat(info, -(j as isize), &mut cind, &mut cval);
            assert!(len <= m);
            goto_drop = true;
        } else {
            // Take column from singleton list
            let j = list[ns];
            ns -= 1;
            assert!(flag[j] == 2);

            // Find row singleton with maximal magnitude
            let len = mat(info, -(j as isize), &mut cind, &mut cval);
            assert!(len <= m);
            let mut kk = 0;
            for k in 1..=len {
                let i = cind[k];
                assert!(i >= 1 && i <= m);
                if cnt[i] == 1 {
                    if kk == 0 || cval[kk].abs() < cval[k].abs() {
                        kk = k;
                    }
                }
            }
            assert!(kk > 0);

            if cval[kk].abs() < tol * big[j] {
                // Singleton too small - drop column
                goto_drop = true;
            } else {
                // Add to triangular part
                size += 1;
                rn[size] = cind[kk];
                cn[size] = j;
                goto_drop = false;
            }
        }

        if goto_drop {
            // Remove j-th column from active submatrix
            assert!(flag[j] != 0);
            flag[j] = 0;
            
            if prev[j] == 0 {
                head = next[j];
            } else {
                next[prev[j]] = next[j];
            }
            
            if next[j] != 0 {
                prev[next[j]] = prev[j];
            }

            // Decrease row counts
            for k in 1..=len {
                let i = cind[k];
                assert!(i >= 1 && i <= m);
                assert!(cnt[i] > 0);
                cnt[i] -= 1;

                if cnt[i] == 1 {
                    // New singleton - find corresponding column
                    let len2 = mat(info, i as isize, &mut rind, &mut rval);
                    assert!(len2 <= n);
                    let mut ks = 0;
                    for kk in 1..=len2 {
                        let jj = rind[kk];
                        assert!(jj >= 1 && jj <= n);
                        if flag[jj] != 0 {
                            assert!(ks == 0);
                            ks = kk;
                        }
                    }
                    assert!(ks > 0);
                    
                    let jj = rind[ks];
                    if flag[jj] != 2 {
                        // Add to singleton list
                        flag[jj] = 2;
                        ns += 1;
                        list[ns] = jj;
                    }
                }
            }
        }
    }

    // Verify all row counts are zero
    for i in 1..=m {
        assert!(cnt[i] == 0);
    }

    TriangResult {
        size,
        rn: rn[1..=size].to_vec(),
        cn: cn[1..=size].to_vec(),
    }
}