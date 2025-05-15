pub fn glp_amd_post_tree(
    root: i32,
    mut k: i32,
    child: &mut [i32],
    sibling: &[i32],
    order: &mut [i32],
    stack: &mut [i32],
) -> i32 {
    let mut head = 0;
    stack[0] = root;

    while head >= 0 {
        let i = stack[head as usize];
        if child[i as usize] != -1 {
            let mut f = child[i as usize];
            while f != -1 {
                head += 1;
                f = sibling[f as usize];
            }

            let mut h = head;
            let mut f = child[i as usize];
            while f != -1 {
                stack[h as usize] = f;
                h -= 1;
                f = sibling[f as usize];
            }
            child[i as usize] = -1;
        } else {
            head -= 1;
            order[i as usize] = k;
            k += 1;
        }
    }
    k
}