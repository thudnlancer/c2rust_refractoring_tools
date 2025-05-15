use std::ffi::CString;
use std::os::raw::c_double;

#[derive(Debug)]
enum AmdStatus {
    Ok,
    OutOfMemory,
    InvalidMatrix,
    OkButJumbled,
    Unknown,
}

impl AmdStatus {
    fn from_value(value: f64) -> Self {
        if value == 0.0 {
            AmdStatus::Ok
        } else if value == -1.0 {
            AmdStatus::OutOfMemory
        } else if value == -2.0 {
            AmdStatus::InvalidMatrix
        } else if value == 1.0 {
            AmdStatus::OkButJumbled
        } else {
            AmdStatus::Unknown
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            AmdStatus::Ok => "OK\n",
            AmdStatus::OutOfMemory => "out of memory\n",
            AmdStatus::InvalidMatrix => "invalid matrix\n",
            AmdStatus::OkButJumbled => "OK, but jumbled\n",
            AmdStatus::Unknown => "unknown\n",
        }
    }
}

fn print_info(label: &str, value: Option<f64>, format: &str) {
    if let Some(val) = value {
        if val >= 0.0 {
            let msg = format!("    {}: {}\n", label, format);
            let c_msg = CString::new(msg).unwrap();
            unsafe {
                libc::printf(c_msg.as_ptr(), val);
            }
        }
    }
}

pub fn glp_amd_info(info: Option<&[f64]>) {
    unsafe {
        let version_msg = CString::new("\nAMD version 2.2.0, May 31, 2007, results:\n").unwrap();
        libc::printf(version_msg.as_ptr());
    }

    let info = match info {
        Some(i) => i,
        None => return,
    };

    let status = AmdStatus::from_value(info[0]);
    unsafe {
        let status_label = CString::new("    status: ").unwrap();
        libc::printf(status_label.as_ptr());
        let status_msg = CString::new(status.as_str()).unwrap();
        libc::printf(status_msg.as_ptr());
    }

    let n = info[1];
    let nz = info[2];
    let symmetry = info[3];
    let diag_nz = info[4];
    let a_plus_at_nz = info[5];
    let dense_rows = info[6];
    let memory_used = info[7];
    let memory_compactions = info[8];
    let lnz = info[9];
    let ndiv = info[10];
    let nmultsubs_ldl = info[11];
    let nmultsubs_lu = info[12];
    let max_nz = info[13];

    let lnzd = if n >= 0.0 && lnz >= 0.0 {
        n + lnz
    } else {
        -1.0
    };

    print_info("n, dimension of A", Some(n), "%.20g");
    print_info("nz, number of nonzeros in A", Some(nz), "%.20g");
    print_info("symmetry of A", Some(symmetry), "%.4f");
    print_info("number of nonzeros on diagonal", Some(diag_nz), "%.20g");
    print_info("nonzeros in pattern of A+A' (excl. diagonal)", Some(a_plus_at_nz), "%.20g");
    print_info("# dense rows/columns of A+A'", Some(dense_rows), "%.20g");
    print_info("memory used, in bytes", Some(memory_used), "%.20g");
    print_info("# of memory compactions", Some(memory_compactions), "%.20g");

    unsafe {
        let note_msg = CString::new("\n    The following approximate statistics are for a subsequent\n    factorization of A(P,P) + A(P,P)'.  They are slight upper\n    bounds if there are no dense rows/columns in A+A', and become\n    looser if dense rows/columns exist.\n\n").unwrap();
        libc::printf(note_msg.as_ptr());
    }

    print_info("nonzeros in L (excluding diagonal)", Some(lnz), "%.20g");
    print_info("nonzeros in L (including diagonal)", Some(lnzd), "%.20g");
    print_info("# divide operations for LDL' or LU", Some(ndiv), "%.20g");
    print_info("# multiply-subtract operations for LDL'", Some(nmultsubs_ldl), "%.20g");
    print_info("# multiply-subtract operations for LU", Some(nmultsubs_lu), "%.20g");
    print_info("max nz. in any column of L (incl. diagonal)", Some(max_nz), "%.20g");

    if n >= 0.0 && ndiv >= 0.0 && nmultsubs_ldl >= 0.0 && nmultsubs_lu >= 0.0 {
        let chol_flop = n + ndiv + 2.0 * nmultsubs_ldl;
        let ldl_real_flop = ndiv + 2.0 * nmultsubs_ldl;
        let ldl_complex_flop = 9.0 * ndiv + 8.0 * nmultsubs_ldl;
        let lu_real_flop = ndiv + 2.0 * nmultsubs_lu;
        let lu_complex_flop = 9.0 * ndiv + 8.0 * nmultsubs_lu;

        unsafe {
            let flop_msg = CString::new("\n    chol flop count for real A, sqrt counted as 1 flop: %.20g\n    LDL' flop count for real A:                         %.20g\n    LDL' flop count for complex A:                      %.20g\n    LU flop count for real A (with no pivoting):        %.20g\n    LU flop count for complex A (with no pivoting):     %.20g\n\n").unwrap();
            libc::printf(flop_msg.as_ptr(), chol_flop, ldl_real_flop, ldl_complex_flop, lu_real_flop, lu_complex_flop);
        }
    }
}