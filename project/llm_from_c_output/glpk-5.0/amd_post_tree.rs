//! Post-ordering of a supernodal elimination tree.

/// Post-order a supernodal elimination tree.
///
/// # Arguments
///
/// * `root` - Root of the tree
/// * `k` - Start numbering at k
/// * `child` - Input argument of size nn, undefined on output. child[i] is the head of a link
///             list of all nodes that are children of node i in the tree.
/// * `sibling` - Input argument of size nn, not modified. If f is a node in the link list of the
///               children of node i, then sibling[f] is the next child of node i.
/// * `order` - Output order, of size nn. order[i] = k if node i is the kth node of the reordered tree.
/// * `stack` - Workspace of size nn
/// * `nn` - Nodes are in the range 0..nn-1 (debug only)
///
/// # Returns
///
/// The next available order number after processing this tree.
pub fn amd_post_tree(
    root: usize,
    mut k: usize,
    child: &mut [usize],
    sibling: &[usize],
    order: &mut [usize],
    stack: &mut [usize],
    #[cfg(debug_assertions)] nn: usize,
) -> usize {
    const EMPTY: usize = usize::MAX;

    // Push root on the stack
    let mut head = 0;
    stack[0] = root;

    while head != usize::MAX {
        // Get head of stack
        debug_assert!(head < stack.len());
        let i = stack[head];
        #[cfg(debug_assertions)] {
            debug_assert!(i < nn, "i out of bounds");
        }

        if child[i] != EMPTY {
            // The children of i are not yet ordered
            // Push each child onto the stack in reverse order
            // so that small ones at the head of the list get popped first
            // and the biggest one at the end of the list gets popped last
            let mut f = child[i];
            let mut count = 0;
            while f != EMPTY {
                count += 1;
                f = sibling[f];
            }

            head += count;
            debug_assert!(head < stack.len());

            let mut h = head;
            f = child[i];
            while f != EMPTY {
                debug_assert!(h > 0);
                stack[h] = f;
                h -= 1;
                #[cfg(debug_assertions)] {
                    debug_assert!(f < nn, "f out of bounds");
                }
                f = sibling[f];
            }
            debug_assert!(stack[h] == i);

            // Delete child list so that i gets ordered next time we see it
            child[i] = EMPTY;
        } else {
            // The children of i (if there were any) are already ordered
            // Remove i from the stack and order it
            head = head.wrapping_sub(1);
            order[i] = k;
            k += 1;
            #[cfg(debug_assertions)] {
                debug_assert!(k <= nn, "k exceeds nn");
            }
        }

        #[cfg(debug_assertions)] {
            debug_assert!(head < stack.len(), "stack overflow");
        }
    }
    k
}