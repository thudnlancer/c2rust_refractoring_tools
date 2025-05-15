/* okalg.rs (out-of-kilter algorithm) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2009-2013 Free Software Foundation, Inc.
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

use std::cmp::{min, max};

#[derive(Debug, PartialEq)]
pub enum OkalgResult {
    Optimal = 0,
    Infeasible = 1,
    Overflow = 2,
    OptimalityTestFailed = 3,
}

pub fn okalg(
    nv: usize,
    na: usize,
    tail: &[usize],
    head: &[usize],
    low: &[i32],
    cap: &[i32],
    cost: &[i32],
    x: &mut [i32],
    pi: &mut [i32],
) -> OkalgResult {
    // Sanity checks
    assert!(tail.len() >= na);
    assert!(head.len() >= na);
    assert!(low.len() >= na);
    assert!(cap.len() >= na);
    assert!(cost.len() >= na);
    assert!(x.len() >= na);
    assert!(pi.len() >= nv);

    for a in 0..na {
        let i = tail[a];
        let j = head[a];
        assert!(i >= 1 && i <= nv);
        assert!(j >= 1 && j <= nv);
        assert!(i != j);
        assert!(low[a] >= 0 && low[a] <= cap[a]);
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
        arc[ptr[tail[a]]] = a + 1; // 1-based index
        ptr[head[a]] -= 1;
        arc[ptr[head[a]]] = a + 1; // 1-based index
    }

    assert_eq!(ptr[1], 1);
    assert_eq!(ptr[nv + 1], 2 * na + 1);

    // Initialize arc flows and node potentials
    for a in 0..na {
        x[a] = 0;
    }
    for i in 0..nv {
        pi[i] = 0;
    }

    loop {
        // Find out-of-kilter arc
        let mut aok = 0;
        let mut s = 0;
        let mut t = 0;

        'outer: for a in 0..na {
            let i = tail[a];
            let j = head[a];
            
            let lambda = match cost[a].checked_add(pi[i - 1] - pi[j - 1]) {
                Some(v) => v,
                None => return OkalgResult::Overflow,
            };

            if x[a] < low[a] || (lambda < 0 && x[a] < cap[a]) {
                // Need to increase flow
                aok = a + 1;
                s = j;
                t = i;
                break 'outer;
            }
            if x[a] > cap[a] || (lambda > 0 && x[a] > low[a]) {
                // Need to decrease flow
                aok = a + 1;
                s = i;
                t = j;
                break 'outer;
            }
        }

        if aok == 0 {
            // All arcs are in kilter
            // Check for feasibility
            for a in 0..na {
                if !(low[a] <= x[a] && x[a] <= cap[a]) {
                    return OkalgResult::OptimalityTestFailed;
                }
            }

            for i in 1..=nv {
                let mut temp = 0;
                for k in ptr[i]..ptr[i + 1] {
                    let a = arc[k as usize - 1] - 1; // Convert to 0-based
                    if tail[a] == i {
                        temp += x[a];
                    } else if head[a] == i {
                        temp -= x[a];
                    } else {
                        unreachable!();
                    }
                }
                if temp != 0 {
                    return OkalgResult::OptimalityTestFailed;
                }
            }

            // Check for optimality
            for a in 0..na {
                let i = tail[a];
                let j = head[a];
                let lambda = match cost[a].checked_add(pi[i - 1] - pi[j - 1]) {
                    Some(v) => v,
                    None => return OkalgResult::Overflow,
                };
                if (lambda > 0 && x[a] != low[a]) || (lambda < 0 && x[a] != cap[a]) {
                    return OkalgResult::OptimalityTestFailed;
                }
            }

            return OkalgResult::Optimal;
        }

        // Find cycle (t, a, s, ..., t)
        for i in 1..=nv {
            link[i] = 0;
        }
        link[s] = aok;
        list[1] = s;
        let mut pos1 = 1;
        let mut pos2 = 1;
        let mut found = false;

        // Breadth first search
        while pos1 <= pos2 {
            let i = list[pos1];
            pos1 += 1;

            for k in ptr[i]..ptr[i + 1] {
                let a = arc[k as usize - 1]; // 1-based
                let a_idx = a - 1; // 0-based

                let (j, is_forward) = if tail[a_idx] == i {
                    (head[a_idx], true)
                } else if head[a_idx] == i {
                    (tail[a_idx], false)
                } else {
                    unreachable!();
                };

                if link[j] != 0 {
                    continue;
                }

                let lambda = if is_forward {
                    if x[a_idx] >= cap[a_idx] {
                        continue;
                    }
                    match cost[a_idx].checked_add(pi[i - 1] - pi[j - 1]) {
                        Some(v) => v,
                        None => return OkalgResult::Overflow,
                    }
                } else {
                    if x[a_idx] <= low[a_idx] {
                        continue;
                    }
                    match cost[a_idx].checked_add(pi[j - 1] - pi[i - 1]) {
                        Some(v) => v,
                        None => return OkalgResult::Overflow,
                    }
                };

                if (is_forward && lambda > 0 && x[a_idx] >= low[a_idx]) ||
                   (!is_forward && lambda < 0 && x[a_idx] <= cap[a_idx]) {
                    continue;
                }

                link[j] = a;
                pos2 += 1;
                list[pos2] = j;

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
            // Nonbreakthrough - adjust potentials
            let mut delta = None;

            for a in 0..na {
                let i = tail[a];
                let j = head[a];
                let lambda = match cost[a].checked_add(pi[i - 1] - pi[j - 1]) {
                    Some(v) => v,
                    None => return OkalgResult::Overflow,
                };

                if link[i] != 0 && link[j] == 0 {
                    if x[a] <= cap[a] && lambda > 0 {
                        delta = Some(delta.map_or(lambda, |d| min(d, lambda)));
                    }
                } else if link[i] == 0 && link[j] != 0 {
                    if x[a] >= low[a] && lambda < 0 {
                        delta = Some(delta.map_or(-lambda, |d| min(d, -lambda)));
                    }
                }
            }

            match delta {
                Some(d) => {
                    for i in 1..=nv {
                        if link[i] == 0 {
                            pi[i - 1] = match pi[i - 1].checked_add(d) {
                                Some(v) => v,
                                None => return OkalgResult::Overflow,
                            };
                        }
                    }
                }
                None => return OkalgResult::Infeasible,
            }
            continue;
        }

        // Breakthrough - adjust flows
        let mut delta = None;

        let mut j = t;
        loop {
            let a = link[j];
            let a_idx = a - 1;
            let lambda;

            if head[a_idx] == j {
                // Forward arc
                let i = tail[a_idx];
                lambda = match cost[a_idx].checked_add(pi[i - 1] - pi[j - 1]) {
                    Some(v) => v,
                    None => return OkalgResult::Overflow,
                };

                let temp = if lambda > 0 {
                    low[a_idx] - x[a_idx]
                } else {
                    cap[a_idx] - x[a_idx]
                };
                delta = Some(delta.map_or(temp, |d| min(d, temp)));
                j = i;
            } else if tail[a_idx] == j {
                // Backward arc
                let i = head[a_idx];
                lambda = match cost[a_idx].checked_add(pi[j - 1] - pi[i - 1]) {
                    Some(v) => v,
                    None => return OkalgResult::Overflow,
                };

                let temp = if lambda < 0 {
                    x[a_idx] - cap[a_idx]
                } else {
                    x[a_idx] - low[a_idx]
                };
                delta = Some(delta.map_or(temp, |d| min(d, temp)));
                j = i;
            } else {
                unreachable!();
            }

            if j == t {
                break;
            }
        }

        let delta = delta.unwrap();
        assert!(delta > 0);

        j = t;
        loop {
            let a = link[j];
            let a_idx = a - 1;

            if head[a_idx] == j {
                // Forward arc
                x[a_idx] += delta;
                j = tail[a_idx];
            } else if tail[a_idx] == j {
                // Backward arc
                x[a_idx] -= delta;
                j = head[a_idx];
            } else {
                unreachable!();
            }

            if j == t {
                break;
            }
        }
    }
}