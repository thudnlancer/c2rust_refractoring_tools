use std::cmp::Ordering;

fn glp_amd_post_tree(
    root: i32,
    k: i32,
    child: &mut [i32],
    sibling: &[i32],
    order: &mut [i32],
    stack: &mut [i32],
) -> i32 {
    let mut top = 0;
    let mut k = k;
    stack[top as usize] = root;
    
    while top >= 0 {
        let node = stack[top as usize];
        let mut child_node = child[node as usize];
        
        if child_node != -1 {
            child[node as usize] = sibling[child_node as usize];
            top += 1;
            stack[top as usize] = child_node;
        } else {
            order[k as usize] = node;
            k += 1;
            top -= 1;
        }
    }
    k
}

#[no_mangle]
pub fn glp_amd_postorder(
    nn: i32,
    parent: &mut [i32],
    nv: &mut [i32],
    fsize: &mut [i32],
    order: &mut [i32],
    child: &mut [i32],
    sibling: &mut [i32],
    stack: &mut [i32],
) {
    // Initialize child and sibling arrays
    for j in 0..nn as usize {
        child[j] = -1;
        sibling[j] = -1;
    }

    // Build the tree structure
    for j in (0..nn as usize).rev() {
        if nv[j] > 0 {
            let p = parent[j] as usize;
            if p != !0 as usize {
                sibling[j] = child[p];
                child[p] = j as i32;
            }
        }
    }

    // Reorder children by descending Fsize
    for i in 0..nn as usize {
        if nv[i] > 0 && child[i] != -1 {
            let mut fprev = -1;
            let mut max_fsize = -1;
            let mut big_fprev = -1;
            let mut big_f = -1;
            let mut f = child[i];

            while f != -1 {
                let current_fsize = fsize[f as usize];
                if current_fsize >= max_fsize {
                    max_fsize = current_fsize;
                    big_fprev = fprev;
                    big_f = f;
                }
                fprev = f;
                f = sibling[f as usize];
            }

            let fnext = sibling[big_f as usize];
            if fnext != -1 {
                if big_fprev == -1 {
                    child[i] = fnext;
                } else {
                    sibling[big_fprev as usize] = fnext;
                }
                sibling[big_f as usize] = -1;
                sibling[fprev as usize] = big_f;
            }
        }
    }

    // Initialize order array
    for i in 0..nn as usize {
        order[i] = -1;
    }

    // Postorder traversal of each tree
    let mut k = 0;
    for i in 0..nn as usize {
        if parent[i] == -1 && nv[i] > 0 {
            k = glp_amd_post_tree(
                i as i32,
                k,
                child,
                sibling,
                order,
                stack,
            );
        }
    }
}