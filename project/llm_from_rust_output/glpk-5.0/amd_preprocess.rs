pub fn glp_amd_preprocess(
    n: i32,
    ap: &[i32],
    ai: &[i32],
    rp: &mut [i32],
    ri: &mut [i32],
    w: &mut [i32],
    flag: &mut [i32],
) {
    // Initialize W and Flag arrays
    for i in 0..n as usize {
        w[i] = 0;
        flag[i] = -1;
    }

    // Count non-zeros in each row
    for j in 0..n as usize {
        let p2 = ap[j + 1];
        for p in ap[j]..p2 {
            let i = ai[p as usize] as usize;
            if flag[i] != j as i32 {
                w[i] += 1;
                flag[i] = j as i32;
            }
        }
    }

    // Compute row pointers
    rp[0] = 0;
    for i in 0..n as usize {
        rp[i + 1] = rp[i] + w[i];
    }

    // Initialize W with row pointers and reset Flag
    for i in 0..n as usize {
        w[i] = rp[i];
        flag[i] = -1;
    }

    // Fill column indices
    for j in 0..n as usize {
        let p2 = ap[j + 1];
        for p in ap[j]..p2 {
            let i = ai[p as usize] as usize;
            if flag[i] != j as i32 {
                let pos = w[i] as usize;
                ri[pos] = j as i32;
                w[i] += 1;
                flag[i] = j as i32;
            }
        }
    }
}