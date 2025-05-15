use std::cmp::Ordering;
use std::ptr::null_mut;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    i: i32,
    cw: f64,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.cw == other.cw
    }
}

impl Eq for Vertex {}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cw.partial_cmp(&self.cw)
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn glp_wclique1(
    n: i32,
    w: &[f64],
    func: impl Fn(i32, &mut [i32]) -> i32,
    c: &mut [i32],
) -> i32 {
    assert!(n >= 0, "n must be non-negative");
    for &weight in &w[1..=n as usize] {
        assert!(weight >= 0.0, "weights must be non-negative");
    }

    if n == 0 {
        return 0;
    }

    let n = n as usize;
    let mut v_list: Vec<Vertex> = (1..=n)
        .map(|i| Vertex {
            i: i as i32,
            cw: w[i],
        })
        .collect();

    // Calculate cumulative weights
    let mut ind = vec![0; n + 1];
    for i in 1..=n {
        let deg = func(i as i32, &mut ind) as usize;
        assert!(deg < n, "degree must be less than n");
        for k in 1..=deg {
            let j = ind[k] as usize;
            assert!(j >= 1 && j <= n && j != i, "invalid neighbor index");
            v_list[i - 1].cw += w[j];
        }
    }

    v_list.sort();

    let mut skip = vec![false; n + 1];
    let mut d_flag = vec![0; n + 1];
    let mut c_list = vec![0; n + 1];
    let mut d_list = vec![0; n + 1];
    let mut sw = vec![0.0; n + 1];
    let mut best = 0.0;
    let mut size = 0;

    for vertex in &v_list {
        let i = vertex.i as usize;
        if skip[i] {
            continue;
        }

        let mut c_size = 1;
        c_list[1] = i as i32;
        let mut c_wght = w[i];
        
        let d_size = func(i as i32, &mut d_list) as usize;
        assert!(d_size < n, "d_size must be less than n");
        
        let mut d_wght = 0.0;
        for k in 1..=d_size {
            let j = d_list[k] as usize;
            assert!(j >= 1 && j <= n && j != i, "invalid neighbor index");
            assert_eq!(d_flag[j], 0, "d_flag must be 0");
            d_flag[j] = 1;
            d_wght += w[j];
        }

        if c_wght + d_wght < best + 1e-5 * (1.0 + best.abs()) {
            continue;
        }

        // Calculate sw values
        for k in 1..=d_size {
            let i = d_list[k] as usize;
            sw[i] = w[i];
            let deg = func(i as i32, &mut ind) as usize;
            assert!(deg < n, "degree must be less than n");
            for kk in 1..=deg {
                let j = ind[kk] as usize;
                assert!(j >= 1 && j <= n && j != i, "invalid neighbor index");
                if d_flag[j] != 0 {
                    sw[i] += w[j];
                }
            }
        }

        let mut current_d_size = d_size;
        while current_d_size > 0 && c_wght + d_wght >= best + 1e-5 * (1.0 + best.abs()) {
            // Find vertex with max sw in d_list
            let mut max_idx = 1;
            for k in 2..=current_d_size {
                if sw[d_list[k] as usize] > sw[d_list[max_idx] as usize] {
                    max_idx = k;
                }
            }
            let i = d_list[max_idx] as usize;

            // Add to clique
            c_size += 1;
            c_list[c_size] = i as i32;
            c_wght += w[i];

            // Update d_list and d_wght
            let deg = func(i as i32, &mut ind) as usize;
            assert!(deg < n, "degree must be less than n");
            for k in 1..=deg {
                let j = ind[k] as usize;
                assert!(j >= 1 && j <= n && j != i, "invalid neighbor index");
                if d_flag[j] != 0 {
                    assert_eq!(d_flag[j], 1, "d_flag must be 1");
                    d_flag[j] = 2;
                }
            }

            let mut new_d_size = 0;
            for k in 1..=current_d_size {
                let j = d_list[k] as usize;
                match d_flag[j] {
                    1 => {
                        d_flag[j] = 0;
                        d_wght -= w[j];
                    }
                    2 => {
                        new_d_size += 1;
                        d_list[new_d_size] = j as i32;
                        d_flag[j] = 1;
                    }
                    _ => panic!("invalid d_flag value"),
                }
            }
            current_d_size = new_d_size;
        }

        if best < c_wght {
            best = c_wght;
            size = c_size;
            assert!(size >= 1 && size <= n as i32, "invalid size");
            c[1..=size].copy_from_slice(&c_list[1..=size]);
        }

        // Mark vertices in clique as skipped
        for k in 1..=c_size {
            skip[c_list[k] as usize] = true;
        }

        // Reset d_flag
        for k in 1..=current_d_size {
            d_flag[d_list[k] as usize] = 0;
        }
    }

    size
}