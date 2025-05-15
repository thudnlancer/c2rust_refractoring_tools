/*!
0-1 Knapsack Problem Solver

This module provides several algorithms to solve the 0-1 knapsack problem:
- Complete enumeration (for small problems)
- Martello & Toth algorithm MT1
- Greedy heuristic
*/

use std::cmp::Ordering;
use std::i32;

/// Maximum number of items for enumeration algorithm
const N_MAX: usize = 40;

/// Solves 0-1 knapsack problem by complete enumeration
///
/// # Arguments
/// * `n` - number of items
/// * `a` - item weights (1-based indexing)
/// * `b` - knapsack capacity
/// * `c` - item values (1-based indexing)
/// * `x` - solution vector (1-based indexing)
///
/// # Returns
/// Optimal objective value or i32::MIN if infeasible
pub fn ks_enum(n: usize, a: &[i32], b: i32, c: &[i32], x: &mut [u8]) -> i32 {
    assert!(n <= N_MAX, "n must be <= {}", N_MAX);
    assert_eq!(a.len(), n + 1);
    assert_eq!(c.len(), n + 1);
    assert_eq!(x.len(), n + 1);

    let mut x_current = vec![0u8; n + 1];
    let mut x_best = vec![0u8; n + 1];
    let mut z_best = i32::MIN;

    loop {
        // Compute constraint and objective
        let (s, z) = (1..=n).fold((0, 0), |(s, z), j| {
            if x_current[j] == 1 {
                (s + a[j], z + c[j])
            } else {
                (s, z)
            }
        });

        // Check constraint and update best solution
        if s <= b && z > z_best {
            z_best = z;
            x_best[1..=n].copy_from_slice(&x_current[1..=n]);
        }

        // Generate next combination
        let mut j = 1;
        while j <= n && x_current[j] == 1 {
            x_current[j] = 0;
            j += 1;
        }

        if j > n {
            break;
        }
        x_current[j] = 1;
    }

    if z_best != i32::MIN {
        x[1..=n].copy_from_slice(&x_best[1..=n]);
    }
    z_best
}

/// Structure for reduced knapsack problem
struct Knapsack {
    orig_n: usize,
    n: usize,
    a: Vec<i32>,
    b: i32,
    c: Vec<i32>,
    c0: i32,
    x: Vec<u8>,
}

impl Knapsack {
    /// Creates a reduced knapsack instance from original problem
    fn reduce(n: usize, a: &[i32], b: i32, c: &[i32]) -> Option<Self> {
        assert_eq!(a.len(), n + 1);
        assert_eq!(c.len(), n + 1);

        let mut ks = Knapsack {
            orig_n: n,
            n: 0,
            a: vec![0; n + 1],
            b,
            c: vec![0; n + 1],
            c0: 0,
            x: vec![0; n + 1],
        };

        // Make all a[j] non-negative
        for j in 1..=n {
            if a[j] >= 0 {
                ks.x[j] = 0x10;
                ks.a[j] = a[j];
                ks.c[j] = c[j];
            } else {
                ks.x[j] = 0x11;
                ks.a[j] = -a[j];
                ks.b += ks.a[j];
                ks.c0 += c[j];
                ks.c[j] = -c[j];
            }
        }

        if ks.b < 0 {
            return None;
        }

        // Build reduced instance
        for j in 1..=n {
            if ks.a[j] == 0 {
                if ks.c[j] <= 0 {
                    ks.x[j] ^= 0x10;
                } else {
                    ks.x[j] ^= 0x11;
                    ks.c0 += ks.c[j];
                }
            } else if ks.a[j] > ks.b || ks.c[j] <= 0 {
                ks.x[j] ^= 0x10;
            } else {
                ks.n += 1;
                ks.a[ks.n] = ks.a[j];
                ks.c[ks.n] = ks.c[j];
            }
        }

        // Check condition (6)
        let s: i32 = ks.a[1..=ks.n].iter().sum();
        if s <= ks.b {
            for j in 1..=n {
                if ks.x[j] & 0x10 != 0 {
                    ks.x[j] ^= 0x11;
                }
            }
            for j in 1..=ks.n {
                ks.c0 += ks.c[j];
            }
            ks.n = 0;
        }

        assert!(ks.n == 0 || ks.n >= 2);
        Some(ks)
    }

    /// Restores solution to original problem
    fn restore(&mut self, x: &[u8]) -> i32 {
        let mut z = self.c0;
        let mut k = 0;

        for j in 1..=self.orig_n {
            if self.x[j] & 0x10 != 0 {
                k += 1;
                assert!(k <= self.n);
                assert!(x[k] == 0 || x[k] == 1);

                if self.x[j] & 1 != 0 {
                    self.x[j] = 1 - x[k];
                } else {
                    self.x[j] = x[k];
                }

                if x[k] == 1 {
                    z += self.c[k];
                }
            }
        }

        assert_eq!(k, self.n);
        z
    }
}

/// Item information for sorting
struct Item {
    index: usize,
    ratio: f32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.ratio == other.ratio
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.ratio.partial_cmp(&self.ratio)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Solves 0-1 knapsack problem with Martello & Toth algorithm MT1
///
/// # Arguments
/// * `n` - number of items
/// * `a` - item weights (1-based indexing)
/// * `b` - knapsack capacity
/// * `c` - item values (1-based indexing)
/// * `x` - solution vector (1-based indexing)
///
/// # Returns
/// Optimal objective value or i32::MIN if infeasible
pub fn ks_mt1(n: usize, a: &[i32], b: i32, c: &[i32], x: &mut [u8]) -> i32 {
    assert_eq!(a.len(), n + 1);
    assert_eq!(c.len(), n + 1);
    assert_eq!(x.len(), n + 1);

    let ks = match Knapsack::reduce(n, a, b, c) {
        Some(ks) => ks,
        None => return i32::MIN,
    };

    let mut x_reduced = vec![0u8; ks.n + 1];
    let z = if ks.n > 0 {
        mt1a(ks.n, &ks.a, ks.b, &ks.c, &mut x_reduced)
    } else {
        0
    };

    let mut ks = ks;
    let z = ks.restore(&x_reduced);
    x[1..=n].copy_from_slice(&ks.x[1..=n]);

    // Verify solution
    let (s1, s2) = (1..=n).fold((0, 0), |(s1, s2), j| {
        assert!(x[j] == 0 || x[j] == 1);
        if x[j] == 1 {
            (s1 + a[j], s2 + c[j])
        } else {
            (s1, s2)
        }
    });

    assert!(s1 <= b);
    assert_eq!(s2, z);
    z
}

/// Interface to MT1 algorithm
fn mt1a(n: usize, a: &[i32], b: i32, c: &[i32], x: &mut [u8]) -> i32 {
    assert!(n >= 2);
    assert_eq!(a.len(), n + 1);
    assert_eq!(c.len(), n + 1);
    assert_eq!(x.len(), n + 1);

    // Sort items by c[j]/a[j] ratio in descending order
    let mut items: Vec<Item> = (1..=n)
        .map(|j| Item {
            index: j,
            ratio: c[j] as f32 / a[j] as f32,
        })
        .collect();
    items.sort();

    // Prepare input for MT1
    let p: Vec<i32> = std::iter::once(0)
        .chain(items.iter().map(|item| c[item.index]))
        .collect();
    let w: Vec<i32> = std::iter::once(0)
        .chain(items.iter().map(|item| a[item.index]))
        .collect();

    // Call MT1 algorithm (implementation not shown)
    let mut x1 = vec![0i32; n + 2];
    let mut xx = vec![0i32; n + 2];
    let mut min = vec![0i32; n + 2];
    let mut psign = vec![0i32; n + 2];
    let mut wsign = vec![0i32; n + 2];
    let mut zsign = vec![0i32; n + 2];

    let z = mt1(
        n as i32,
        &p,
        &w,
        b,
        &mut x1,
        1,
        &mut xx,
        &mut min,
        &mut psign,
        &mut wsign,
        &mut zsign,
    );

    assert!(z >= 0);

    // Store solution
    for (i, item) in items.iter().enumerate().take(n) {
        x[item.index] = x1[i + 1] as u8;
    }

    z
}

/// Placeholder for MT1 algorithm implementation
fn mt1(
    _n: i32,
    _p: &[i32],
    _w: &[i32],
    _b: i32,
    _x1: &mut [i32],
    _jflag: i32,
    _xx: &mut [i32],
    _min: &mut [i32],
    _psign: &mut [i32],
    _wsign: &mut [i32],
    _zsign: &mut [i32],
) -> i32 {
    // Actual MT1 implementation would go here
    // This is a placeholder that needs to be implemented
    unimplemented!("MT1 algorithm implementation required")
}

/// Solves 0-1 knapsack problem with greedy heuristic
///
/// # Arguments
/// * `n` - number of items
/// * `a` - item weights (1-based indexing)
/// * `b` - knapsack capacity
/// * `c` - item values (1-based indexing)
/// * `x` - solution vector (1-based indexing)
///
/// # Returns
/// Optimal objective value or i32::MIN if infeasible
pub fn ks_greedy(n: usize, a: &[i32], b: i32, c: &[i32], x: &mut [u8]) -> i32 {
    assert_eq!(a.len(), n + 1);
    assert_eq!(c.len(), n + 1);
    assert_eq!(x.len(), n + 1);

    let ks = match Knapsack::reduce(n, a, b, c) {
        Some(ks) => ks,
        None => return i32::MIN,
    };

    let mut x_reduced = vec![0u8; ks.n + 1];
    let z = if ks.n > 0 {
        greedy(ks.n, &ks.a, ks.b, &ks.c, &mut x_reduced)
    } else {
        0
    };

    let mut ks = ks;
    let z = ks.restore(&x_reduced);
    x[1..=n].copy_from_slice(&ks.x[1..=n]);

    // Verify solution
    let (s1, s2) = (1..=n).fold((0, 0), |(s1, s2), j| {
        assert!(x[j] == 0 || x[j] == 1);
        if x[j] == 1 {
            (s1 + a[j], s2 + c[j])
        } else {
            (s1, s2)
        }
    });

    assert!(s1 <= b);
    assert_eq!(s2, z);
    z
}

/// Greedy algorithm for normalized 0-1 knapsack instance
fn greedy(n: usize, a: &[i32], b: i32, c: &[i32], x: &mut [u8]) -> i32 {
    assert!(n >= 2);
    assert_eq!(a.len(), n + 1);
    assert_eq!(c.len(), n + 1);
    assert_eq!(x.len(), n + 1);

    // Sort items by c[j]/a[j] ratio in descending order
    let mut items: Vec<Item> = (1..=n)
        .map(|j| Item {
            index: j,
            ratio: c[j] as f32 / a[j] as f32,
        })
        .collect();
    items.sort();

    let mut s = 0;
    let mut z = 0;

    for item in items {
        if s + a[item.index] > b {
            break;
        }
        x[item.index] = 1;
        s += a[item.index];
        z += c[item.index];
    }

    z
}