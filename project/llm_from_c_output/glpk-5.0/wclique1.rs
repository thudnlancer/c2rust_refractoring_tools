use std::cmp::Ordering;
use std::ptr;
use std::mem;

struct Vertex {
    i: usize,
    cw: f64,
}

fn fcmp(x: &Vertex, y: &Vertex) -> Ordering {
    if x.cw > y.cw {
        Ordering::Less
    } else if x.cw < y.cw {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn wclique1(
    n: usize,
    w: &[f64],
    func: &dyn Fn(&mut Option<&mut dyn std::any::Any>, usize, &mut [usize]) -> usize,
    info: &mut Option<&mut dyn std::any::Any>,
    c: &mut [usize],
) -> usize {
    // Perform sanity checks
    assert!(w.len() >= n);
    for &weight in &w[1..=n] {
        assert!(weight >= 0.0);
    }

    // If the graph is empty, nothing to do
    if n == 0 {
        return 0;
    }

    // Allocate working arrays
    let mut ind = vec![0; n + 1];
    let mut v_list: Vec<Vertex> = (1..=n)
        .map(|i| Vertex { i, cw: 0.0 })
        .collect();
    v_list.insert(0, Vertex { i: 0, cw: 0.0 }); // Dummy element for 1-based indexing
    let mut c_list = vec![0; n + 1];
    let mut d_list = vec![0; n + 1];
    let mut d_flag = vec![false; n + 1];
    let mut skip = vec![false; n + 1];
    let mut sw = vec![0.0; n + 1];

    // Build the vertex list
    for i in 1..=n {
        v_list[i].i = i;
        // Compute the cumulative weight of each vertex i
        v_list[i].cw = w[i];
        let deg = func(info, i, &mut ind[1..]);
        assert!(deg < n);
        for k in 1..=deg {
            let j = ind[k];
            assert!(j >= 1 && j <= n && j != i);
            v_list[i].cw += w[j];
        }
    }

    // Sort the vertex list to access vertices in descending order of cumulative weights
    v_list[1..=n].sort_by(fcmp);

    // Initially all vertices are unmarked
    skip[1..=n].fill(false);

    // Clear flags of all vertices
    d_flag[1..=n].fill(false);

    let mut size = 0;
    let mut best = 0.0;

    // Look through all vertices of the graph
    for l in 1..=n {
        // Take vertex i
        let i = v_list[l].i;

        // If this vertex was already included in one of previously constructed cliques, skip it
        if skip[i] {
            continue;
        }

        // Use vertex i as the initial clique vertex
        let mut c_size = 1;    // size of current clique
        c_list[1] = i;         // list of vertices in current clique
        let mut c_wght = w[i]; // weight of current clique

        // Determine the candidate set D = { j : (i,j) in E }
        let mut d_size = func(info, i, &mut d_list[1..]);
        assert!(d_size < n);
        let mut d_wght = 0.0;  // weight of set D
        for k in 1..=d_size {
            let j = d_list[k];
            assert!(j >= 1 && j <= n && j != i);
            assert!(!d_flag[j]);
            d_flag[j] = true;
            d_wght += w[j];
        }

        // Check an upper bound to the final clique weight
        if c_wght + d_wght < best + 1e-5 * (1.0 + best.abs()) {
            // Skip constructing the current clique
            continue;
        }

        // Compute the summary weight of each vertex i in D
        for k in 1..=d_size {
            let i = d_list[k];
            sw[i] = w[i];
            // Consider vertices adjacent to vertex i
            let deg = func(info, i, &mut ind[1..]);
            assert!(deg < n);
            for kk in 1..=deg {
                let j = ind[kk];
                assert!(j >= 1 && j <= n && j != i);
                if d_flag[j] {
                    sw[i] += w[j];
                }
            }
        }

        // Grow the current clique by adding vertices from D
        while d_size > 0 {
            // Check an upper bound to the final clique weight
            if c_wght + d_wght < best + 1e-5 * (1.0 + best.abs()) {
                // Skip constructing the current clique
                break;
            }

            // Choose vertex i in D having maximal summary weight
            let mut max_i = d_list[1];
            for k in 2..=d_size {
                let j = d_list[k];
                if sw[max_i] < sw[j] {
                    max_i = j;
                }
            }
            let i = max_i;

            // Include vertex i in the current clique
            c_size += 1;
            c_list[c_size] = i;
            c_wght += w[i];

            // Remove all vertices not adjacent to vertex i, including vertex i itself, from D
            let deg = func(info, i, &mut ind[1..]);
            assert!(deg < n);
            for k in 1..=deg {
                let j = ind[k];
                assert!(j >= 1 && j <= n && j != i);
                // Vertex j is adjacent to vertex i
                if d_flag[j] {
                    // Mark vertex j to keep it in D
                    d_flag[j] = false; // Will be set to true again below
                }
            }

            let mut new_d_size = 0;
            for k in 1..=d_size {
                let j = d_list[k];
                if d_flag[j] {
                    // Keep vertex j in D
                    new_d_size += 1;
                    d_list[new_d_size] = j;
                    d_flag[j] = true;
                } else {
                    // Remove vertex j from D
                    d_wght -= w[j];
                }
            }
            d_size = new_d_size;
        }

        // The current clique has been completely constructed
        if best < c_wght {
            best = c_wght;
            size = c_size;
            assert!(size >= 1 && size <= n);
            c[1..=size].copy_from_slice(&c_list[1..=size]);
        }

        // Mark the current clique vertices in order not to use them as initial vertices anymore
        for k in 1..=c_size {
            skip[c_list[k]] = true;
        }

        // Set D can be non-empty, so clean up vertex flags
        for k in 1..=d_size {
            d_flag[d_list[k]] = false;
        }
    }

    size
}