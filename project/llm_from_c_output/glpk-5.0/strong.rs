use std::mem;
use std::ptr;

/// Finds all strongly connected components of a graph.
///
/// # Arguments
///
/// * `graph` - The graph to analyze
/// * `v_num` - Offset in vertex data to store component number (if >= 0)
///
/// # Returns
///
/// The total number of components found.
pub fn glp_strong_comp(graph: &glp_graph, v_num: i32) -> Result<i32, String> {
    if v_num >= 0 && v_num > graph.v_size - mem::size_of::<i32>() as i32 {
        return Err(format!("glp_strong_comp: v_num = {}; invalid offset", v_num));
    }

    let n = graph.nv;
    if n == 0 {
        return Ok(0);
    }

    let na = graph.na;
    let mut icn = vec![0; na + 1];
    let mut ip = vec![0; n + 1];
    let mut lenr = vec![0; n + 1];
    let mut ior = vec![0; n + 1];
    let mut ib = vec![0; n + 1];
    let mut lowl = vec![0; n + 1];
    let mut numb = vec![0; n + 1];
    let mut prev = vec![0; n + 1];

    let mut k = 1;
    for i in 1..=n {
        let v = &graph.v[i];
        ip[i] = k;
        let mut a = &v.out;
        while let Some(arc) = a {
            icn[k] = arc.head.i;
            k += 1;
            a = &arc.t_next;
        }
        lenr[i] = k - ip[i];
    }

    if na != k - 1 {
        return Err("Number of arcs doesn't match".to_string());
    }

    let nc = mc13d(
        n, &icn, &ip, &lenr, &mut ior, &mut ib, &mut lowl, &mut numb, &mut prev,
    );

    if v_num >= 0 {
        if ib[1] != 1 {
            return Err("Invalid component data".to_string());
        }

        for k in 1..=nc {
            let last = if k < nc { ib[k + 1] } else { n + 1 };
            if ib[k] >= last {
                return Err("Invalid component range".to_string());
            }

            for i in ib[k]..last {
                let v = &graph.v[ior[i] as usize];
                unsafe {
                    ptr::write(
                        (v.data as *const u8).offset(v_num as isize) as *mut i32,
                        k as i32,
                    );
                }
            }
        }
    }

    Ok(nc as i32)
}

// Note: The following types and functions would need to be defined elsewhere:
// - glp_graph structure
// - glp_vertex structure
// - glp_arc structure
// - mc13d function implementation
// The unsafe block is necessary for the pointer arithmetic when writing component numbers,
// but is minimized and carefully checked.