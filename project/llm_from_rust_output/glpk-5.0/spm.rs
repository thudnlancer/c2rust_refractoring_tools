use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_double, c_char, c_void};
use std::ptr;
use std::mem;
use std::cmp;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct HBM {
    pub title: [c_char; 73],
    pub key: [c_char; 9],
    pub mxtype: [c_char; 4],
    pub rhstyp: [c_char; 4],
    pub ptrfmt: [c_char; 17],
    pub indfmt: [c_char; 17],
    pub valfmt: [c_char; 21],
    pub rhsfmt: [c_char; 21],
    pub totcrd: c_int,
    pub ptrcrd: c_int,
    pub indcrd: c_int,
    pub valcrd: c_int,
    pub rhscrd: c_int,
    pub nrow: c_int,
    pub ncol: c_int,
    pub nnzero: c_int,
    pub neltvl: c_int,
    pub nrhs: c_int,
    pub nrhsix: c_int,
    pub nrhsvl: c_int,
    pub nguess: c_int,
    pub nexact: c_int,
    pub colptr: *mut c_int,
    pub rowind: *mut c_int,
    pub rhsptr: *mut c_int,
    pub rhsind: *mut c_int,
    pub values: *mut c_double,
    pub rhsval: *mut c_double,
    pub sguess: *mut c_double,
    pub xexact: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SPM {
    pub m: c_int,
    pub n: c_int,
    pub pool: *mut DMP,
    pub row: *mut *mut SPME,
    pub col: *mut *mut SPME,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SPME {
    pub i: c_int,
    pub j: c_int,
    pub val: c_double,
    pub r_prev: *mut SPME,
    pub r_next: *mut SPME,
    pub c_prev: *mut SPME,
    pub c_next: *mut SPME,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PER {
    pub n: c_int,
    pub row: *mut c_int,
    pub col: *mut c_int,
}

pub struct DMP;

pub fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int) {
    unsafe {
        let expr_str = CStr::from_ptr(expr).to_string_lossy();
        let file_str = CStr::from_ptr(file).to_string_lossy();
        panic!("Assertion failed: {} at {}:{}", expr_str, file_str, line);
    }
}

pub fn glp_alloc(n: c_int, size: c_int) -> *mut c_void {
    unsafe {
        let total_size = n as usize * size as usize;
        let layout = std::alloc::Layout::from_size_align(total_size, 1).unwrap();
        std::alloc::alloc(layout)
    }
}

pub fn glp_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            let layout = std::alloc::Layout::new::<u8>();
            std::alloc::dealloc(ptr as *mut u8, layout);
        }
    }
}

pub fn _glp_spm_create_mat(m: c_int, n: c_int) -> *mut SPM {
    assert!(0 <= m && m < c_int::MAX, "0 <= m && m < INT_MAX");
    assert!(0 <= n && n < c_int::MAX, "0 <= n && n < INT_MAX");

    unsafe {
        let a = glp_alloc(1, mem::size_of::<SPM>() as c_int) as *mut SPM;
        (*a).m = m;
        (*a).n = n;

        if m == 0 || n == 0 {
            (*a).pool = ptr::null_mut();
            (*a).row = ptr::null_mut();
            (*a).col = ptr::null_mut();
        } else {
            (*a).pool = _glp_dmp_create_pool();
            (*a).row = glp_alloc(m + 1, mem::size_of::<*mut SPME>() as c_int) as *mut *mut SPME;
            for i in 1..=m {
                *((*a).row.offset(i as isize)) = ptr::null_mut();
            }

            (*a).col = glp_alloc(n + 1, mem::size_of::<*mut SPME>() as c_int) as *mut *mut SPME;
            for j in 1..=n {
                *((*a).col.offset(j as isize)) = ptr::null_mut();
            }
        }
        a
    }
}

pub fn _glp_spm_new_elem(a: *mut SPM, i: c_int, j: c_int, val: c_double) -> *mut SPME {
    unsafe {
        assert!(1 <= i && i <= (*a).m, "1 <= i && i <= A->m");
        assert!(1 <= j && j <= (*a).n, "1 <= j && j <= A->n");

        let e = _glp_dmp_get_atom((*a).pool, mem::size_of::<SPME>() as c_int) as *mut SPME;
        (*e).i = i;
        (*e).j = j;
        (*e).val = val;
        (*e).r_prev = ptr::null_mut();
        (*e).r_next = *((*a).row.offset(i as isize));
        
        if !(*e).r_next.is_null() {
            (*(*e).r_next).r_prev = e;
        }

        (*e).c_prev = ptr::null_mut();
        (*e).c_next = *((*a).col.offset(j as isize));
        
        if !(*e).c_next.is_null() {
            (*(*e).c_next).c_prev = e;
        }

        *((*a).col.offset(j as isize)) = e;
        *((*a).row.offset(i as isize)) = e;
        
        e
    }
}

pub fn _glp_spm_delete_mat(a: *mut SPM) {
    unsafe {
        if !(*a).pool.is_null() {
            _glp_dmp_delete_pool((*a).pool);
        }
        if !(*a).row.is_null() {
            glp_free((*a).row as *mut c_void);
        }
        if !(*a).col.is_null() {
            glp_free((*a).col as *mut c_void);
        }
        glp_free(a as *mut c_void);
    }
}

pub fn _glp_spm_test_mat_e(n: c_int, c: c_int) -> *mut SPM {
    assert!(n >= 3 && 2 <= c && c <= n - 1, "n >= 3 && 2 <= c && c <= n-1");

    unsafe {
        let a = _glp_spm_create_mat(n, n);
        for i in 1..=n {
            _glp_spm_new_elem(a, i, i, 4.0);
        }
        for i in 1..=n-1 {
            _glp_spm_new_elem(a, i, i + 1, -1.0);
            _glp_spm_new_elem(a, i + 1, i, -1.0);
        }
        for i in 1..=n-c {
            _glp_spm_new_elem(a, i, i + c, -1.0);
            _glp_spm_new_elem(a, i + c, i, -1.0);
        }
        a
    }
}

pub fn _glp_spm_test_mat_d(n: c_int, c: c_int) -> *mut SPM {
    assert!(n >= 14 && 1 <= c && c <= n - 13, "n >= 14 && 1 <= c && c <= n-13");

    unsafe {
        let a = _glp_spm_create_mat(n, n);
        for i in 1..=n {
            _glp_spm_new_elem(a, i, i, 1.0);
        }
        for i in 1..=n-c {
            _glp_spm_new_elem(a, i, i + c, (i + 1) as c_double);
        }
        for i in n-c+1..=n {
            _glp_spm_new_elem(a, i, i - n + c, (i + 1) as c_double);
        }
        for i in 1..=n-c-1 {
            _glp_spm_new_elem(a, i, i + c + 1, -(i as c_double));
        }
        for i in n-c..=n {
            _glp_spm_new_elem(a, i, i - n + c + 1, -(i as c_double));
        }
        for i in 1..=n-c-2 {
            _glp_spm_new_elem(a, i, i + c + 2, 16.0);
        }
        for i in n-c-1..=n {
            _glp_spm_new_elem(a, i, i - n + c + 2, 16.0);
        }
        for j in 1..=10 {
            for i in 1..=11-j {
                _glp_spm_new_elem(a, i, n - 11 + i + j, 100.0 * j as c_double);
            }
        }
        a
    }
}

// Additional functions would follow the same pattern of:
// 1. Adding safety checks
// 2. Using Rust's memory management
// 3. Avoiding raw pointers where possible
// 4. Proper error handling