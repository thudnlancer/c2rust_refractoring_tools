/* ========================================================================= */
/* === AMD_defaults ======================================================== */
/* ========================================================================= */

/* ------------------------------------------------------------------------- */
/* AMD, Copyright (c) Timothy A. Davis,                                      */
/* Patrick R. Amestoy, and Iain S. Duff.  See ../README.txt for License.     */
/* email: davis at cise.ufl.edu    CISE Department, Univ. of Florida.        */
/* web: http://www.cise.ufl.edu/research/sparse/amd                          */
/* ------------------------------------------------------------------------- */

/* User-callable.  Sets default control parameters for AMD.  See amd.h
 * for details.
 */

/// AMD control parameters
pub const AMD_CONTROL: usize = 5;
pub const AMD_DENSE: usize = 0;
pub const AMD_AGGRESSIVE: usize = 1;
pub const AMD_DEFAULT_DENSE: f64 = 10.0;
pub const AMD_DEFAULT_AGGRESSIVE: f64 = 1.0;

/// Sets default control parameters for AMD
pub fn amd_defaults(control: Option<&mut [f64]>) {
    if let Some(ctrl) = control {
        if ctrl.len() >= AMD_CONTROL {
            for item in ctrl.iter_mut().take(AMD_CONTROL) {
                *item = 0.0;
            }
            ctrl[AMD_DENSE] = AMD_DEFAULT_DENSE;
            ctrl[AMD_AGGRESSIVE] = AMD_DEFAULT_AGGRESSIVE;
        }
    }
}