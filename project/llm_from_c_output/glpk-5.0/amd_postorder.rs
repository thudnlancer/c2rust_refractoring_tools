use std::i32;

const EMPTY: i32 = -1;

/// Perform a postordering (via depth-first search) of an assembly tree.
pub fn amd_postorder(
    nn: i32,
    parent: &[i32],
    nv: &[i32],
    fsize: &[i32],
    order: &mut [i32],
    child: &mut [i32],
    sibling: &mut [i32],
    stack: &mut [i32],
) {
    // Initialize child and sibling arrays
    for j in 0..nn as usize {
        child[j] = EMPTY;
        sibling[j] = EMPTY;
    }

    // Place the children in link lists - bigger elements tend to be last
    for j in (0..nn as usize).rev() {
        if nv[j] > 0 {
            // This is an element
            let p = parent[j];
            if p != EMPTY {
                // Place the element in link list of its parent's children
                sibling[j] = child[p as usize];
                child[p as usize] = j as i32;
            }
        }
    }

    // Place the largest child last in the list of children for each node
    for i in 0..nn as usize {
        if nv[i] > 0 && child[i] != EMPTY {
            let mut fprev = EMPTY;
            let mut max_frsize = EMPTY;
            let mut big_fprev = EMPTY;
            let mut big_f = EMPTY;

            let mut f = child[i];
            while f != EMPTY {
                let frsize = fsize[f as usize];
                if frsize >= max_frsize {
                    // This is the biggest seen so far
                    max_frsize = frsize;
                    big_fprev = fprev;
                    big_f = f;
                }
                fprev = f;
                f = sibling[f as usize];
            }

            debug_assert!(big_f != EMPTY);
            let fnext = sibling[big_f as usize];

            if fnext != EMPTY {
                // If fnext is EMPTY then big_f is already at the end of list
                if big_fprev == EMPTY {
                    // Delete big_f from the front of the list
                    child[i] = fnext;
                } else {
                    // Delete big_f from the middle of the list
                    sibling[big_fprev as usize] = fnext;
                }

                // Put big_f at the end of the list
                sibling[big_f as usize] = EMPTY;
                debug_assert!(child[i] != EMPTY);
                debug_assert!(fprev != big_f);
                debug_assert!(fprev != EMPTY);
                sibling[fprev as usize] = big_f;
            }
        }
    }

    // Initialize order array
    for i in 0..nn as usize {
        order[i] = EMPTY;
    }

    let mut k = 0;

    // Postorder the assembly tree
    for i in 0..nn as usize {
        if parent[i] == EMPTY && nv[i] > 0 {
            k = amd_post_tree(
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

fn amd_post_tree(
    root: i32,
    mut k: i32,
    child: &[i32],
    sibling: &[i32],
    order: &mut [i32],
    stack: &mut [i32],
) -> i32 {
    let mut top = 0;
    stack[0] = root;

    while top >= 0 {
        let node = stack[top as usize];
        top -= 1;

        // First child
        let mut first_child = child[node as usize];
        if first_child == EMPTY {
            // Leaf node
            order[k as usize] = node;
            k += 1;
        } else {
            // Push the node back on stack (will be processed after children)
            top += 1;
            stack[top as usize] = node;

            // Push children in reverse order (so they're processed left-to-right)
            let mut next = sibling[first_child as usize];
            while next != EMPTY {
                top += 1;
                stack[top as usize] = next;
                next = sibling[next as usize];
            }

            top += 1;
            stack[top as usize] = first_child;
        }
    }

    k
}