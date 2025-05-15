/* glpios12.rs (node selection heuristics) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2003-2018 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::f64::{MAX, MIN, EPSILON};

#[derive(Debug)]
pub struct GlpTree {
    parm: GlpParm,
    head: Option<Box<IOSNPD>>,
    tail: Option<Box<IOSNPD>>,
    slot: Vec<Slot>,
    mip: GlpMIP,
}

#[derive(Debug)]
struct GlpParm {
    bt_tech: BtTech,
}

#[derive(Debug, PartialEq)]
enum BtTech {
    DFS,
    BFS,
    BLB,
    BPH,
}

#[derive(Debug)]
struct IOSNPD {
    p: i32,
    up: Option<Box<IOSNPD>>,
    next: Option<Box<IOSNPD>>,
    bound: f64,
    ii_sum: f64,
    lp_obj: f64,
}

#[derive(Debug)]
struct Slot {
    node: Option<Box<IOSNPD>>,
}

#[derive(Debug)]
struct GlpMIP {
    mip_stat: MIPStat,
    mip_obj: f64,
    dir: Direction,
}

#[derive(Debug, PartialEq)]
enum MIPStat {
    UNDEF,
    FEAS,
}

#[derive(Debug, PartialEq)]
enum Direction {
    MIN,
    MAX,
}

pub fn ios_choose_node(tree: &GlpTree) -> i32 {
    match tree.parm.bt_tech {
        BtTech::DFS => {
            // depth first search
            tree.tail.as_ref().expect("Tail node is null").p
        }
        BtTech::BFS => {
            // breadth first search
            tree.head.as_ref().expect("Head node is null").p
        }
        BtTech::BLB => {
            // select node with best local bound
            best_node(tree)
        }
        BtTech::BPH => {
            if tree.mip.mip_stat == MIPStat::UNDEF {
                // "most integer feasible" subproblem
                most_feas(tree)
            } else {
                // best projection heuristic
                best_proj(tree)
            }
        }
    }
}

fn most_feas(tree: &GlpTree) -> i32 {
    // select subproblem whose parent has minimal sum of integer infeasibilities
    let mut p = 0;
    let mut best = MAX;
    
    let mut node = tree.head.as_ref();
    while let Some(current) = node {
        let up = current.up.as_ref().expect("Parent node is null");
        if best > up.ii_sum {
            p = current.p;
            best = up.ii_sum;
        }
        node = current.next.as_ref();
    }
    p
}

fn best_proj(tree: &GlpTree) -> i32 {
    // select subproblem using the best projection heuristic
    // the global bound must exist
    assert!(tree.mip.mip_stat == MIPStat::FEAS);
    
    // obtain pointer to the root node, which must exist
    let root = tree.slot[0].node.as_ref().expect("Root node is null");
    
    // deg estimates degradation of the objective function per unit
    // of the sum of integer infeasibilities
    assert!(root.ii_sum > 0.0);
    let deg = (tree.mip.mip_obj - root.bound) / root.ii_sum;
    
    // nothing has been selected so far
    let mut p = 0;
    let mut best = MAX;
    
    // walk through the list of active subproblems
    let mut node = tree.head.as_ref();
    while let Some(current) = node {
        let up = current.up.as_ref().expect("Parent node is null");
        // obj estimates optimal objective value if the sum of integer
        // infeasibilities were zero
        let mut obj = up.bound + deg * up.ii_sum;
        if tree.mip.dir == Direction::MAX {
            obj = -obj;
        }
        // select the subproblem which has the best estimated optimal
        // objective value
        if best > obj {
            p = current.p;
            best = obj;
        }
        node = current.next.as_ref();
    }
    p
}

fn best_node(tree: &GlpTree) -> i32 {
    // select subproblem with best local bound
    let mut best_node: Option<&IOSNPD> = None;
    let (bound, eps) = match tree.mip.dir {
        Direction::MIN => {
            let mut bound = MAX;
            let mut node = tree.head.as_ref();
            while let Some(current) = node {
                if bound > current.bound {
                    bound = current.bound;
                }
                node = current.next.as_ref();
            }
            assert!(bound != MAX);
            let eps = 1e-10 * (1.0 + bound.abs());
            (bound, eps)
        }
        Direction::MAX => {
            let mut bound = MIN;
            let mut node = tree.head.as_ref();
            while let Some(current) = node {
                if bound < current.bound {
                    bound = current.bound;
                }
                node = current.next.as_ref();
            }
            assert!(bound != MIN);
            let eps = 1e-10 * (1.0 + bound.abs());
            (bound, eps)
        }
    };

    let mut node = tree.head.as_ref();
    while let Some(current) = node {
        let is_candidate = match tree.mip.dir {
            Direction::MIN => current.bound <= bound + eps,
            Direction::MAX => current.bound >= bound - eps,
        };
        
        if is_candidate {
            let up = current.up.as_ref().expect("Parent node is null");
            if best_node.is_none() || 
                best_node.unwrap().up.as_ref().unwrap().ii_sum > up.ii_sum {
                best_node = Some(current);
            }
        }
        node = current.next.as_ref();
    }
    
    best_node.expect("No best node found").p
}