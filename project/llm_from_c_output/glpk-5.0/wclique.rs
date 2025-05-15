use std::time::{Instant, Duration};

struct Csa {
    n: usize,
    wt: Vec<i32>,
    a: Vec<u8>,
    record: i32,
    rec_level: usize,
    rec: Vec<usize>,
    clique: Vec<i32>,
    set: Vec<usize>,
}

fn is_edge(csa: &Csa, i: usize, j: usize) -> bool {
    if i == j {
        return false;
    }
    let (i, j) = if i > j { (i, j) } else { (j, i) };
    let k = (i * (i - 1)) / 2 + j;
    let byte = k / 8;
    let bit = 7 - (k % 8);
    (csa.a[byte] & (1 << bit)) != 0
}

fn sub(csa: &mut Csa, ct: isize, table: &[usize], level: usize, weight: i32, l_weight: i32) {
    let mut newtable = vec![0; csa.n];
    if ct <= 0 {
        let mut weight = weight;
        let mut level = level;
        if ct == 0 {
            csa.set[level] = table[0];
            level += 1;
            weight += l_weight;
        }
        if weight > csa.record {
            csa.record = weight;
            csa.rec_level = level;
            csa.rec[..level].copy_from_slice(&csa.set[..level]);
        }
        return;
    }

    for i in (0..=ct as usize).rev() {
        if level == 0 && i < ct as usize {
            break;
        }
        let k = table[i];
        if level > 0 && csa.clique[k] <= (csa.record - weight) {
            break;
        }
        csa.set[level] = k;
        let curr_weight = weight + csa.wt[k];
        let l_weight = l_weight - csa.wt[k];
        if l_weight <= (csa.record - curr_weight) {
            break;
        }

        let mut p1 = 0;
        let mut left_weight = 0;
        for &j in &table[..i] {
            if is_edge(csa, j, k) {
                newtable[p1] = j;
                p1 += 1;
                left_weight += csa.wt[j];
            }
        }

        if left_weight <= (csa.record - curr_weight) {
            continue;
        }

        sub(csa, p1 as isize - 1, &newtable[..p1], level + 1, curr_weight, left_weight);
    }
}

pub fn wclique(n: usize, w: &[i32], a: &[u8], ind: &mut [usize]) -> usize {
    let mut csa = Csa {
        n,
        wt: w.to_vec(),
        a: a.to_vec(),
        record: 0,
        rec_level: 0,
        rec: vec![0; n],
        clique: vec![0; n],
        set: vec![0; n],
    };

    let mut used = vec![false; n];
    let mut nwt = vec![0; n];
    let mut pos = vec![0; n];

    for i in 0..n {
        nwt[i] = 0;
        for j in 0..n {
            if is_edge(&csa, i, j) {
                nwt[i] += csa.wt[j];
            }
        }
    }

    for i in (0..n).rev() {
        let mut max_wt = -1;
        let mut max_nwt = -1;
        let mut p = 0;
        for j in 0..n {
            if !used[j] && (csa.wt[j] > max_wt || (csa.wt[j] == max_wt && nwt[j] > max_nwt)) {
                max_wt = csa.wt[j];
                max_nwt = nwt[j];
                p = j;
            }
        }
        pos[i] = p;
        used[p] = true;
        for j in 0..n {
            if !used[j] && j != p && is_edge(&csa, p, j) {
                nwt[j] -= csa.wt[p];
            }
        }
    }

    let mut wth = 0;
    let timer = Instant::now();
    for i in 0..n {
        wth += csa.wt[pos[i]];
        sub(&mut csa, i as isize, &pos[..=i], 0, 0, wth);
        csa.clique[pos[i]] = csa.record;
        if timer.elapsed() >= Duration::from_secs(5) {
            println!("level = {} ({}); best = {}", i + 1, n, csa.record);
        }
    }

    for i in 0..csa.rec_level {
        ind[i + 1] = csa.rec[i] + 1;
    }

    csa.rec_level
}