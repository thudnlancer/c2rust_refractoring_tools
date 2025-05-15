use std::f64;

struct GlpTree {
    parm: GlpIocp,
    tail: Option<Box<IosNpd>>,
    head: Option<Box<IosNpd>>,
    mip: GlpProb,
    slot: Vec<IosSlot>,
}

struct GlpProb {
    mip_stat: i32,
    dir: i32,
    mip_obj: f64,
}

struct GlpIocp {
    bt_tech: i32,
}

struct IosNpd {
    p: i32,
    up: Option<Box<IosNpd>>,
    next: Option<Box<IosNpd>>,
    bound: f64,
    ii_sum: f64,
}

struct IosSlot {
    node: Option<Box<IosNpd>>,
}

fn glp_ios_choose_node(T: &GlpTree) -> i32 {
    match T.parm.bt_tech {
        1 => T.tail.as_ref().map_or_else(|| panic!("T->tail != NULL"), |tail| tail.p),
        2 => T.head.as_ref().map_or_else(|| panic!("T->head != NULL"), |head| head.p),
        3 => best_node(T),
        4 => {
            if T.mip.mip_stat == 1 {
                most_feas(T)
            } else {
                best_proj(T)
            }
        }
        _ => panic!("T != T"),
    }
}

fn most_feas(T: &GlpTree) -> i32 {
    let mut p = 0;
    let mut best = f64::MAX;
    let mut node = T.head.as_ref();

    while let Some(current) = node {
        let up = current.up.as_ref().expect("node->up != NULL");
        if best > up.ii_sum {
            p = current.p;
            best = up.ii_sum;
        }
        node = current.next.as_ref();
    }

    p
}

fn best_proj(T: &GlpTree) -> i32 {
    assert_eq!(T.mip.mip_stat, 2, "T->mip->mip_stat == GLP_FEAS");
    
    let root = T.slot.get(1)
        .and_then(|slot| slot.node.as_ref())
        .expect("root != NULL");
    
    assert!(root.ii_sum > 0.0, "root->ii_sum > 0.0");
    
    let deg = (T.mip.mip_obj - root.bound) / root.ii_sum;
    let mut p = 0;
    let mut best = f64::MAX;
    let mut node = T.head.as_ref();

    while let Some(current) = node {
        let up = current.up.as_ref().expect("node->up != NULL");
        let mut obj = up.bound + deg * up.ii_sum;
        
        if T.mip.dir == 2 {
            obj = -obj;
        }
        
        if best > obj {
            p = current.p;
            best = obj;
        }
        
        node = current.next.as_ref();
    }

    p
}

fn best_node(T: &GlpTree) -> i32 {
    let mut best = None;
    let (bound, eps) = match T.mip.dir {
        1 => {
            let bound = T.head.as_ref().map_or(f64::MAX, |mut node| {
                let mut min = node.bound;
                while let Some(next) = node.next.as_ref() {
                    if next.bound < min {
                        min = next.bound;
                    }
                    node = next;
                }
                min
            });
            assert_ne!(bound, f64::MAX, "bound != +DBL_MAX");
            (bound, 1e-10 * (1.0 + bound.abs()))
        }
        2 => {
            let bound = T.head.as_ref().map_or(f64::MIN, |mut node| {
                let mut max = node.bound;
                while let Some(next) = node.next.as_ref() {
                    if next.bound > max {
                        max = next.bound;
                    }
                    node = next;
                }
                max
            });
            assert_ne!(bound, f64::MIN, "bound != -DBL_MAX");
            (bound, 1e-10 * (1.0 + bound.abs()))
        }
        _ => panic!("T != T"),
    };

    let mut node = T.head.as_ref();
    while let Some(current) = node {
        let within_bounds = match T.mip.dir {
            1 => current.bound <= bound + eps,
            2 => current.bound >= bound - eps,
            _ => false,
        };

        if within_bounds {
            let up = current.up.as_ref().expect("node->up != NULL");
            best = match best {
                None => Some((current, up.ii_sum)),
                Some((best_node, best_sum)) if up.ii_sum < best_sum => {
                    Some((current, up.ii_sum))
                }
                other => other,
            };
        }
        node = current.next.as_ref();
    }

    let (best_node, _) = best.expect("best != NULL");
    best_node.p
}