/* ========================================================================= */
/* === AMD_valid =========================================================== */
/* ========================================================================= */

/* ------------------------------------------------------------------------- */
/* AMD, Copyright (c) Timothy A. Davis,                                      */
/* Patrick R. Amestoy, and Iain S. Duff.  See ../README.txt for License.     */
/* email: davis at cise.ufl.edu    CISE Department, Univ. of Florida.        */
/* web: http://www.cise.ufl.edu/research/sparse/amd                          */
/* ------------------------------------------------------------------------- */

/// Check if a column-form matrix is valid or not.
/// 
/// The matrix A is n_row-by-n_col. The row indices of entries in column j are in
/// Ai [Ap [j] ... Ap [j+1]-1]. Required conditions are:
///
/// - n_row >= 0
/// - n_col >= 0
/// - nz = Ap [n_col] >= 0        number of entries in the matrix
/// - Ap [0] == 0
/// - Ap [j] <= Ap [j+1] for all j in the range 0 to n_col.
/// - Ai [0 ... nz-1] must be in the range 0 to n_row-1.
///
/// If any of the above conditions fail, AMD_INVALID is returned. If the
/// following condition holds, AMD_OK_BUT_JUMBLED is returned (a warning,
/// not an error):
///
/// - row indices in Ai [Ap [j] ... Ap [j+1]-1] are not sorted in ascending
///   order, and/or duplicate entries exist.
///
/// Otherwise, AMD_OK is returned.
pub fn amd_valid(
    n_row: usize,
    n_col: usize,
    ap: &[usize],
    ai: &[usize],
) -> Result<(), AmdValidationError> {
    let nz = *ap.last().ok_or(AmdValidationError::Invalid)?;
    
    // Check basic validity conditions
    if ap.is_empty() || ai.len() < nz || ap[0] != 0 {
        return Err(AmdValidationError::Invalid);
    }

    let mut result = Ok(());

    for j in 0..n_col {
        let p1 = ap[j];
        let p2 = ap[j + 1];

        if p1 > p2 {
            return Err(AmdValidationError::Invalid);
        }

        let mut ilast = None;
        for p in p1..p2 {
            let i = ai[p];
            
            if i >= n_row {
                return Err(AmdValidationError::Invalid);
            }

            if let Some(last) = ilast {
                if i <= last {
                    result = Err(AmdValidationError::OkButJumbled);
                }
            }
            ilast = Some(i);
        }
    }

    result
}

/// AMD validation result
#[derive(Debug, PartialEq)]
pub enum AmdValidationError {
    /// Matrix is valid
    Ok,
    /// Matrix is valid but has unsorted/duplicate entries
    OkButJumbled,
    /// Matrix is invalid
    Invalid,
}

impl std::fmt::Display for AmdValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AmdValidationError::Ok => write!(f, "Matrix is valid"),
            AmdValidationError::OkButJumbled => write!(f, "Matrix is valid but has unsorted/duplicate entries"),
            AmdValidationError::Invalid => write!(f, "Matrix is invalid"),
        }
    }
}

impl std::error::Error for AmdValidationError {}