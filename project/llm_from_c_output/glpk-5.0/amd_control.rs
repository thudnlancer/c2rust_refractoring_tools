/* ========================================================================= */
/* === AMD_control ========================================================= */
/* ========================================================================= */

/* ------------------------------------------------------------------------- */
/* AMD, Copyright (c) Timothy A. Davis,                                      */
/* Patrick R. Amestoy, and Iain S. Duff.  See ../README.txt for License.     */
/* email: davis at cise.ufl.edu    CISE Department, Univ. of Florida.        */
/* web: http://www.cise.ufl.edu/research/sparse/amd                          */
/* ------------------------------------------------------------------------- */

/* User-callable.  Prints the control parameters for AMD.  See amd.h
 * for details.  If the Control array is not present, the defaults are
 * printed instead.
 */

use std::os::raw::c_int;

const AMD_DENSE: usize = 0;
const AMD_AGGRESSIVE: usize = 1;
const AMD_DEFAULT_DENSE: f64 = 10.0;
const AMD_DEFAULT_AGGRESSIVE: bool = true;
const AMD_MAIN_VERSION: i32 = 2;
const AMD_SUB_VERSION: i32 = 4;
const AMD_SUBSUB_VERSION: i32 = 1;
const AMD_DATE: &str = "Nov 30, 2022";

type Int = c_int;

pub fn amd_control(control: Option<&[f64]>) {
    let (alpha, aggressive) = match control {
        Some(ctrl) => {
            let alpha = ctrl[AMD_DENSE];
            let aggressive = ctrl[AMD_AGGRESSIVE] != 0.0;
            (alpha, aggressive)
        }
        None => (AMD_DEFAULT_DENSE, AMD_DEFAULT_AGGRESSIVE),
    };

    println!(
        "\nAMD version {}.{}.{}, {}: approximate minimum degree ordering\n    dense row parameter: {}",
        AMD_MAIN_VERSION, AMD_SUB_VERSION, AMD_SUBSUB_VERSION, AMD_DATE, alpha
    );

    if alpha < 0.0 {
        println!("    no rows treated as dense");
    } else {
        println!(
            "    (rows with more than max ({} * sqrt (n), 16) entries are\n    considered \"dense\", and placed last in output permutation)",
            alpha
        );
    }

    if aggressive {
        println!("    aggressive absorption:  yes");
    } else {
        println!("    aggressive absorption:  no");
    }

    println!("    size of AMD integer: {}\n", std::mem::size_of::<Int>());
}