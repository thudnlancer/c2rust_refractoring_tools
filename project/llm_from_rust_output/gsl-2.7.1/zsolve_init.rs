use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

#[derive(Debug)]
pub enum GslError {
    Domain,
    NoMemory,
    // Other error variants can be added as needed
}

#[derive(Debug)]
pub struct GslPolyComplexWorkspace {
    nc: usize,
    matrix: NonNull<f64>,
}

impl GslPolyComplexWorkspace {
    pub fn new(n: usize) -> Result<Self, GslError> {
        if n == 0 {
            return Err(GslError::Domain);
        }

        let nc = n - 1;
        let matrix_size = nc * nc;
        let matrix_layout = Layout::array::<f64>(matrix_size).map_err(|_| GslError::NoMemory)?;
        
        // SAFETY: We checked the layout is valid and size is non-zero
        let matrix_ptr = unsafe { alloc(matrix_layout) as *mut f64 };
        let matrix = NonNull::new(matrix_ptr).ok_or(GslError::NoMemory)?;

        Ok(Self { nc, matrix })
    }
}

impl Drop for GslPolyComplexWorkspace {
    fn drop(&mut self) {
        let matrix_size = self.nc * self.nc;
        let layout = Layout::array::<f64>(matrix_size).expect("Invalid layout");
        // SAFETY: We know the pointer was allocated with this layout
        unsafe {
            dealloc(self.matrix.as_ptr() as *mut u8, layout);
        }
    }
}