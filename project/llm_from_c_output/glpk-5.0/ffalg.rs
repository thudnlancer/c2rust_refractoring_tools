// ffalg.rs (Ford-Fulkerson algorithm)

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct FordFulkersonError {
    message: String,
}

impl fmt::Display for FordFulkersonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ford-Fulkerson error: {}", self.message)
    }
}

impl Error for FordFulkersonError {}

pub fn ffalg(
    nv: usize,
    na: usize,
    tail: &[usize],
    head: &[usize],
    s: usize,
    t: usize,
    cap: &[i32],
    x: &mut [i32],
    cut: Option<&mut [bool]>,
) -> Result<(), Box<dyn Error>> {
    // Sanity checks
    if nv < 2 {
        return Err(Box::new(FordFulkersonError {
            message: "nv must be >= 2".to_string(),
        }));
    }
    if s < 1 || s > nv {
        return Err(Box::new(FordFulkersonError {
            message: "s must be between 1 and nv".to_string(),
        }));
    }
    if t < 1 || t > nv {
        return Err(Box::new(FordFulkersonError {
            message: "t must be between 1 and nv".to_string(),
        }));
    }
    if s == t {
        return Err(Box::new(FordFulkersonError {
            message: "s and t must be different".to_string(),
        }));
    }
    for a in 0..na {
        let i = tail[a];
        let j = head[a];
        if i < 1 || i > nv || j < 1 || j > nv {
            return Err(Box::new(FordFulkersonError {
                message: "tail and head must be between 1 and nv".to_string(),
            }));
        }
        if i == j {
            return Err(Box::new(FordFulkersonError {
                message: "self-loops are not allowed".to_string(),
            }));
        }
        if cap[a] < 0 {
            return Err(Box::new(FordFulkersonError {
                message: "capacities must be non-negative".to_string(),
            }));
        }
    }

    // Allocate working arrays
    let mut ptr = vec![0; nv + 2];
    let mut arc = vec![0; 2 * na];
    let mut link = vec![0; nv + 1];
    let mut list = vec![0; nv + 1];

    // ptr[i] := (degree of node i)
    for i in 1..=nv {
        ptr[i] = 0;
    }
    for a in 0..na {
        ptr[tail[a]] += 1;
        ptr[head[a]] += 1;
    }

    // Initialize arc pointers
    ptr[1] += 1;
    for i in 1..nv {
        ptr[i + 1] += ptr[i];
    }
    ptr[nv + 1] = ptr[nv];

    // Build arc lists
    for a in (0..na).rev() {
        ptr[tail[a]] -= 1;
        arc[ptr[tail[a]]] = a + 1; // +1 to match 1-based indexing
        ptr[head[a]] -= 1;
        arc[ptr[head[a]]] = a + 1;
    }

    // Initialize arc flows
    for a in 0..na {
        x[a] = 0;
    }

    loop {
        // Build augmenting tree rooted at s
        link.fill(0);
        link[s] = usize::MAX; // Using MAX to represent -1
        list[1] = s;
        let mut pos1 = 1;
        let mut pos2 = 1;
        let mut found = false;

        // Breadth first search
        while pos1 <= pos2 {
            let i = list[pos1];
            pos1 += 1;

            // Consider all arcs incident to node i
            for k in ptr[i]..ptr[i + 1] {
                let a = arc[k];
                let a_idx = a - 1; // Convert back to 0-based

                let (j, is_forward) = if tail[a_idx] == i {
                    // Forward arc i->j
                    (head[a_idx], true)
                } else if head[a_idx] == i {
                    // Backward arc i<-j
                    (tail[a_idx], false)
                } else {
                    panic!("Invalid arc");
                };

                // Skip if node j is already labelled
                if link[j] != 0 {
                    continue;
                }

                // Check flow constraints
                if is_forward && x[a_idx] == cap[a_idx] {
                    continue;
                }
                if !is_forward && x[a_idx] == 0 {
                    continue;
                }

                // Label node j and enqueue it
                link[j] = a;
                pos2 += 1;
                list[pos2] = j;

                // Check for breakthrough
                if j == t {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            // No augmenting path exists; current flow is maximal
            if let Some(cut_slice) = cut {
                for i in 1..=nv {
                    cut_slice[i - 1] = link[i] != 0;
                }
            }
            break;
        }

        // BREAKTHROUGH
        // Walk through arcs of the augmenting path (s, ..., t) in reverse order
        let mut delta = i32::MAX;
        let mut j = t;
        while j != s {
            let a = link[j];
            let a_idx = a - 1;

            let (i, temp) = if head[a_idx] == j {
                // Forward arc i->j
                (tail[a_idx], cap[a_idx] - x[a_idx])
            } else if tail[a_idx] == j {
                // Backward arc i<-j
                (head[a_idx], x[a_idx])
            } else {
                panic!("Invalid arc");
            };

            if delta > temp {
                delta = temp;
            }
            j = i;
        }

        // Increase the flow along the path
        j = t;
        while j != s {
            let a = link[j];
            let a_idx = a - 1;

            if head[a_idx] == j {
                // Forward arc i->j
                x[a_idx] += delta;
                j = tail[a_idx];
            } else if tail[a_idx] == j {
                // Backward arc i<-j
                x[a_idx] -= delta;
                j = head[a_idx];
            } else {
                panic!("Invalid arc");
            }
        }
    }

    Ok(())
}