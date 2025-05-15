/* ========================================================================= */
/* === AMD_dump ============================================================ */
/* ========================================================================= */

/* ------------------------------------------------------------------------- */
/* AMD, Copyright (c) Timothy A. Davis,                                      */
/* Patrick R. Amestoy, and Iain S. Duff.  See ../README.txt for License.     */
/* email: davis at cise.ufl.edu    CISE Department, Univ. of Florida.        */
/* web: http://www.cise.ufl.edu/research/sparse/amd                          */
/* ------------------------------------------------------------------------- */

/* Debugging routines for AMD.  Not used if NDEBUG is not defined at compile-
 * time (the default).  See comments in amd_internal.h on how to enable
 * debugging.  Not user-callable.
 */

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[cfg(not(debug_assertions))]
const NDEBUG: bool = true;

#[cfg(debug_assertions))]
const NDEBUG: bool = false;

static mut AMD_DEBUG: i32 = -999; // default is no debug printing

const EMPTY: i32 = -1;

/* ========================================================================= */
/* === AMD_debug_init ====================================================== */
/* ========================================================================= */

/// Sets the debug print level, by reading the file debug.amd (if it exists)
fn amd_debug_init(s: &str) -> io::Result<()> {
    let path = Path::new("debug.amd");
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            unsafe { AMD_DEBUG = -999 };
            return Ok(());
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let debug_level: i32 = contents.trim().parse().unwrap_or(-999);
    unsafe { AMD_DEBUG = debug_level };

    if unsafe { AMD_DEBUG >= 0 } {
        println!("{}: AMD_debug_init, D= {}", s, unsafe { AMD_DEBUG });
    }

    Ok(())
}

/* ========================================================================= */
/* === AMD_dump ============================================================ */
/* ========================================================================= */

/// Dump AMD's data structure, except for the hash buckets. This routine
/// cannot be called when the hash buckets are non-empty.
fn amd_dump(
    n: i32,
    pe: &[i32],
    iw: &[i32],
    len: &[i32],
    iwlen: i32,
    pfree: i32,
    nv: &[i32],
    next: &[i32],
    last: &[i32],
    head: &[i32],
    elen: &[i32],
    degree: &[i32],
    w: &[i32],
    nel: i32,
) {
    if unsafe { AMD_DEBUG < 0 } {
        return;
    }

    debug_assert!(pfree <= iwlen);
    if unsafe { AMD_DEBUG >= 3 } {
        println!("\nAMD dump, pfree: {}", pfree);
    }

    for i in 0..n as usize {
        let pe_i = pe[i];
        let elen_i = elen[i];
        let nv_i = nv[i];
        let len_i = len[i];
        let w_i = w[i];

        if elen_i >= EMPTY {
            if nv_i == 0 {
                if unsafe { AMD_DEBUG >= 3 } {
                    print!("\nI {}: nonprincipal:    ", i);
                }
                debug_assert!(elen_i == EMPTY);
                if pe_i == EMPTY {
                    if unsafe { AMD_DEBUG >= 3 } {
                        println!(" dense node");
                    }
                    debug_assert!(w_i == 1);
                } else {
                    debug_assert!(pe_i < EMPTY);
                    if unsafe { AMD_DEBUG >= 3 } {
                        println!(" i {} -> parent {}", i, !pe_i);
                    }
                }
            } else {
                if unsafe { AMD_DEBUG >= 3 } {
                    println!("\nI {}: active principal supervariable:", i);
                    println!("   nv(i): {}  Flag: {}", nv_i, nv_i < 0);
                }
                debug_assert!(elen_i >= 0);
                debug_assert!(nv_i > 0 && pe_i >= 0);
                let mut p = pe_i as usize;
                if unsafe { AMD_DEBUG >= 3 } {
                    print!("   e/s: ");
                    if elen_i == 0 {
                        print!(" : ");
                    }
                }
                debug_assert!(pe_i + len_i <= pfree);
                for k in 0..len_i as usize {
                    let j = iw[p];
                    if unsafe { AMD_DEBUG >= 3 } {
                        print!("  {}", j);
                    }
                    debug_assert!(j >= 0 && j < n);
                    if k == elen_i as usize - 1 && unsafe { AMD_DEBUG >= 3 } {
                        print!(" : ");
                    }
                    p += 1;
                }
                if unsafe { AMD_DEBUG >= 3 } {
                    println!();
                }
            }
        } else {
            let e = i;
            if w_i == 0 {
                if unsafe { AMD_DEBUG >= 3 } {
                    println!("\nE {}: absorbed element: w {}", e, w_i);
                }
                debug_assert!(nv_i > 0 && pe_i < 0);
                if unsafe { AMD_DEBUG >= 3 } {
                    println!(" e {} -> parent {}", e, !pe_i);
                }
            } else {
                if unsafe { AMD_DEBUG >= 3 } {
                    println!("\nE {}: unabsorbed element: w {}", e, w_i);
                }
                debug_assert!(nv_i > 0 && pe_i >= 0);
                let mut p = pe_i as usize;
                if unsafe { AMD_DEBUG >= 3 } {
                    print!(" : ");
                }
                debug_assert!(pe_i + len_i <= pfree);
                for _ in 0..len_i as usize {
                    let j = iw[p];
                    if unsafe { AMD_DEBUG >= 3 } {
                        print!("  {}", j);
                    }
                    debug_assert!(j >= 0 && j < n);
                    p += 1;
                }
                if unsafe { AMD_DEBUG >= 3 } {
                    println!();
                }
            }
        }
    }

    if unsafe { AMD_DEBUG >= 3 } {
        println!("\nDegree lists:");
    }
    if nel >= 0 {
        let mut cnt = 0;
        for deg in 0..n {
            if head[deg as usize] == EMPTY {
                continue;
            }
            let mut ilast = EMPTY;
            if unsafe { AMD_DEBUG >= 3 } {
                println!("{}: ", deg);
            }
            let mut i = head[deg as usize];
            while i != EMPTY {
                if unsafe { AMD_DEBUG >= 3 } {
                    println!(
                        "   {} : next {} last {} deg {}",
                        i,
                        next[i as usize],
                        last[i as usize],
                        degree[i as usize]
                    );
                }
                debug_assert!(
                    i >= 0 && i < n && ilast == last[i as usize] && deg == degree[i as usize]
                );
                cnt += nv[i as usize];
                ilast = i;
                i = next[i as usize];
            }
            if unsafe { AMD_DEBUG >= 3 } {
                println!();
            }
        }
        debug_assert!(cnt == n - nel);
    }
}