/* ========================================================================= */
/* === AMD_info ============================================================ */
/* ========================================================================= */

/* ------------------------------------------------------------------------- */
/* AMD, Copyright (c) Timothy A. Davis,                                      */
/* Patrick R. Amestoy, and Iain S. Duff.  See ../README.txt for License.     */
/* email: davis at cise.ufl.edu    CISE Department, Univ. of Florida.        */
/* web: http://www.cise.ufl.edu/research/sparse/amd                          */
/* ------------------------------------------------------------------------- */

/* User-callable.  Prints the output statistics for AMD.  See amd.h
 * for details.  If the Info array is not present, nothing is printed.
 */

use std::fmt;

const AMD_MAIN_VERSION: i32 = 2;
const AMD_SUB_VERSION: i32 = 4;
const AMD_SUBSUB_VERSION: i32 = 1;
const AMD_DATE: &str = "Nov 30, 2022";

#[derive(Debug, PartialEq)]
enum AmdStatus {
    Ok,
    OutOfMemory,
    Invalid,
    OkButJumbled,
    Unknown,
}

impl fmt::Display for AmdStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AmdStatus::Ok => write!(f, "OK"),
            AmdStatus::OutOfMemory => write!(f, "out of memory"),
            AmdStatus::Invalid => write!(f, "invalid matrix"),
            AmdStatus::OkButJumbled => write!(f, "OK, but jumbled"),
            AmdStatus::Unknown => write!(f, "unknown"),
        }
    }
}

fn print_if_positive(format: &str, value: f64) {
    if value >= 0.0 {
        println!(format, value);
    }
}

pub fn amd_info(info: Option<&[f64]>) {
    println!("\nAMD version {}.{}.{}, {}, results:", 
        AMD_MAIN_VERSION, AMD_SUB_VERSION, AMD_SUBSUB_VERSION, AMD_DATE);

    let info = match info {
        Some(info) => info,
        None => return,
    };

    let n = info[0];
    let ndiv = info[1];
    let nmultsubs_ldl = info[2];
    let nmultsubs_lu = info[3];
    let lnz = info[4];
    let lnzd = if n >= 0.0 && lnz >= 0.0 { n + lnz } else { -1.0 };

    /* AMD return status */
    print!("    status: ");
    let status = match info[5] as i32 {
        0 => AmdStatus::Ok,
        1 => AmdStatus::OutOfMemory,
        2 => AmdStatus::Invalid,
        3 => AmdStatus::OkButJumbled,
        _ => AmdStatus::Unknown,
    };
    println!("{}", status);

    /* statistics about the input matrix */
    print_if_positive("    n, dimension of A:                                  {:.20}", n);
    print_if_positive("    nz, number of nonzeros in A:                        {:.20}", info[6]);
    print_if_positive("    symmetry of A:                                      {:.4}", info[7]);
    print_if_positive("    number of nonzeros on diagonal:                     {:.20}", info[8]);
    print_if_positive("    nonzeros in pattern of A+A' (excl. diagonal):       {:.20}", info[9]);
    print_if_positive("    # dense rows/columns of A+A':                       {:.20}", info[10]);

    /* statistics about AMD's behavior */
    print_if_positive("    memory used, in bytes:                              {:.20}", info[11]);
    print_if_positive("    # of memory compactions:                            {:.20}", info[12]);

    /* statistics about the ordering quality */
    println!("\n    The following approximate statistics are for a subsequent");
    println!("    factorization of A(P,P) + A(P,P)'.  They are slight upper");
    println!("    bounds if there are no dense rows/columns in A+A', and become");
    println!("    looser if dense rows/columns exist.\n");

    print_if_positive("    nonzeros in L (excluding diagonal):                 {:.20}", lnz);
    print_if_positive("    nonzeros in L (including diagonal):                 {:.20}", lnzd);
    print_if_positive("    # divide operations for LDL' or LU:                 {:.20}", ndiv);
    print_if_positive("    # multiply-subtract operations for LDL':            {:.20}", nmultsubs_ldl);
    print_if_positive("    # multiply-subtract operations for LU:              {:.20}", nmultsubs_lu);
    print_if_positive("    max nz. in any column of L (incl. diagonal):        {:.20}", info[13]);

    /* total flop counts for various factorizations */
    if n >= 0.0 && ndiv >= 0.0 && nmultsubs_ldl >= 0.0 && nmultsubs_lu >= 0.0 {
        println!("\n    chol flop count for real A, sqrt counted as 1 flop: {:.20}", 
            n + ndiv + 2.0 * nmultsubs_ldl);
        println!("    LDL' flop count for real A:                         {:.20}", 
            ndiv + 2.0 * nmultsubs_ldl);
        println!("    LDL' flop count for complex A:                      {:.20}", 
            9.0 * ndiv + 8.0 * nmultsubs_ldl);
        println!("    LU flop count for real A (with no pivoting):        {:.20}", 
            ndiv + 2.0 * nmultsubs_lu);
        println!("    LU flop count for complex A (with no pivoting):     {:.20}\n", 
            9.0 * ndiv + 8.0 * nmultsubs_lu);
    }
}