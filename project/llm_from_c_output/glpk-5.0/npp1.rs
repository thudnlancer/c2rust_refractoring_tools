use std::ptr;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_double};
use std::mem;
use std::collections::LinkedList;

#[repr(C)]
pub struct NPP {
    orig_dir: c_int,
    orig_m: c_int,
    orig_n: c_int,
    orig_nnz: c_int,
    pool: *mut DMP,
    name: *mut c_char,
    obj: *mut c_char,
    c0: c_double,
    nrows: c_int,
    ncols: c_int,
    r_head: *mut NPPROW,
    r_tail: *mut NPPROW,
    c_head: *mut NPPCOL,
    c_tail: *mut NPPCOL,
    stack: *mut DMP,
    top: *mut NPPTSE,
    m: c_int,
    n: c_int,
    nnz: c_int,
    row_ref: *mut c_int,
    col_ref: *mut c_int,
    sol: c_int,
    scaling: c_int,
    p_stat: c_int,
    d_stat: c_int,
    t_stat: c_int,
    i_stat: c_int,
    r_stat: *mut c_char,
    r_pi: *mut c_double,
    c_stat: *mut c_char,
    c_value: *mut c_double,
}

#[repr(C)]
pub struct NPPROW {
    i: c_int,
    name: *mut c_char,
    lb: c_double,
    ub: c_double,
    ptr: *mut NPPAIJ,
    temp: c_int,
    prev: *mut NPPROW,
    next: *mut NPPROW,
}

#[repr(C)]
pub struct NPPCOL {
    j: c_int,
    name: *mut c_char,
    is_int: c_int,
    lb: c_double,
    ub: c_double,
    coef: c_double,
    ptr: *mut NPPAIJ,
    temp: c_int,
    prev: *mut NPPCOL,
    next: *mut NPPCOL,
}

#[repr(C)]
pub struct NPPAIJ {
    row: *mut NPPROW,
    col: *mut NPPCOL,
    val: c_double,
    r_prev: *mut NPPAIJ,
    r_next: *mut NPPAIJ,
    c_prev: *mut NPPAIJ,
    c_next: *mut NPPAIJ,
}

#[repr(C)]
pub struct NPPTSE {
    func: Option<unsafe extern "C" fn(*mut NPP, *mut ()) -> c_int>,
    info: *mut (),
    link: *mut NPPTSE,
}

#[repr(C)]
pub struct DMP {
    // Dummy struct for memory pool
}

impl NPP {
    pub fn create_wksp() -> *mut NPP {
        unsafe {
            let npp = Box::into_raw(Box::new(NPP {
                orig_dir: 0,
                orig_m: 0,
                orig_n: 0,
                orig_nnz: 0,
                pool: DMP::create_pool(),
                name: ptr::null_mut(),
                obj: ptr::null_mut(),
                c0: 0.0,
                nrows: 0,
                ncols: 0,
                r_head: ptr::null_mut(),
                r_tail: ptr::null_mut(),
                c_head: ptr::null_mut(),
                c_tail: ptr::null_mut(),
                stack: DMP::create_pool(),
                top: ptr::null_mut(),
                m: 0,
                n: 0,
                nnz: 0,
                row_ref: ptr::null_mut(),
                col_ref: ptr::null_mut(),
                sol: 0,
                scaling: 0,
                p_stat: 0,
                d_stat: 0,
                t_stat: 0,
                i_stat: 0,
                r_stat: ptr::null_mut(),
                r_pi: ptr::null_mut(),
                c_stat: ptr::null_mut(),
                c_value: ptr::null_mut(),
            }));
            npp
        }
    }

    pub fn insert_row(&mut self, row: *mut NPPROW, where_: c_int) {
        unsafe {
            if where_ == 0 {
                (*row).prev = ptr::null_mut();
                (*row).next = self.r_head;
                if (*row).next.is_null() {
                    self.r_tail = row;
                } else {
                    (*(*row).next).prev = row;
                }
                self.r_head = row;
            } else {
                (*row).prev = self.r_tail;
                (*row).next = ptr::null_mut();
                if (*row).prev.is_null() {
                    self.r_head = row;
                } else {
                    (*(*row).prev).next = row;
                }
                self.r_tail = row;
            }
        }
    }

    pub fn remove_row(&mut self, row: *mut NPPROW) {
        unsafe {
            if (*row).prev.is_null() {
                self.r_head = (*row).next;
            } else {
                (*(*row).prev).next = (*row).next;
            }
            if (*row).next.is_null() {
                self.r_tail = (*row).prev;
            } else {
                (*(*row).next).prev = (*row).prev;
            }
        }
    }

    pub fn activate_row(&mut self, row: *mut NPPROW) {
        unsafe {
            if (*row).temp == 0 {
                (*row).temp = 1;
                self.remove_row(row);
                self.insert_row(row, 0);
            }
        }
    }

    pub fn deactivate_row(&mut self, row: *mut NPPROW) {
        unsafe {
            if (*row).temp != 0 {
                (*row).temp = 0;
                self.remove_row(row);
                self.insert_row(row, 1);
            }
        }
    }

    pub fn insert_col(&mut self, col: *mut NPPCOL, where_: c_int) {
        unsafe {
            if where_ == 0 {
                (*col).prev = ptr::null_mut();
                (*col).next = self.c_head;
                if (*col).next.is_null() {
                    self.c_tail = col;
                } else {
                    (*(*col).next).prev = col;
                }
                self.c_head = col;
            } else {
                (*col).prev = self.c_tail;
                (*col).next = ptr::null_mut();
                if (*col).prev.is_null() {
                    self.c_head = col;
                } else {
                    (*(*col).prev).next = col;
                }
                self.c_tail = col;
            }
        }
    }

    pub fn remove_col(&mut self, col: *mut NPPCOL) {
        unsafe {
            if (*col).prev.is_null() {
                self.c_head = (*col).next;
            } else {
                (*(*col).prev).next = (*col).next;
            }
            if (*col).next.is_null() {
                self.c_tail = (*col).prev;
            } else {
                (*(*col).next).prev = (*col).prev;
            }
        }
    }

    pub fn activate_col(&mut self, col: *mut NPPCOL) {
        unsafe {
            if (*col).temp == 0 {
                (*col).temp = 1;
                self.remove_col(col);
                self.insert_col(col, 0);
            }
        }
    }

    pub fn deactivate_col(&mut self, col: *mut NPPCOL) {
        unsafe {
            if (*col).temp != 0 {
                (*col).temp = 0;
                self.remove_col(col);
                self.insert_col(col, 1);
            }
        }
    }

    pub fn add_row(&mut self) -> *mut NPPROW {
        unsafe {
            let row = DMP::get_atom(self.pool, mem::size_of::<NPPROW>()) as *mut NPPROW;
            (*row).i = {
                self.nrows += 1;
                self.nrows
            };
            (*row).name = ptr::null_mut();
            (*row).lb = -f64::MAX;
            (*row).ub = f64::MAX;
            (*row).ptr = ptr::null_mut();
            (*row).temp = 0;
            self.insert_row(row, 1);
            row
        }
    }

    pub fn add_col(&mut self) -> *mut NPPCOL {
        unsafe {
            let col = DMP::get_atom(self.pool, mem::size_of::<NPPCOL>()) as *mut NPPCOL;
            (*col).j = {
                self.ncols += 1;
                self.ncols
            };
            (*col).name = ptr::null_mut();
            (*col).is_int = 0;
            (*col).lb = 0.0;
            (*col).ub = 0.0;
            (*col).coef = 0.0;
            (*col).ptr = ptr::null_mut();
            (*col).temp = 0;
            self.insert_col(col, 1);
            col
        }
    }

    pub fn add_aij(&mut self, row: *mut NPPROW, col: *mut NPPCOL, val: c_double) -> *mut NPPAIJ {
        unsafe {
            let aij = DMP::get_atom(self.pool, mem::size_of::<NPPAIJ>()) as *mut NPPAIJ;
            (*aij).row = row;
            (*aij).col = col;
            (*aij).val = val;
            (*aij).r_prev = ptr::null_mut();
            (*aij).r_next = (*row).ptr;
            (*aij).c_prev = ptr::null_mut();
            (*aij).c_next = (*col).ptr;
            if !(*aij).r_next.is_null() {
                (*(*aij).r_next).r_prev = aij;
            }
            if !(*aij).c_next.is_null() {
                (*(*aij).c_next).c_prev = aij;
            }
            (*row).ptr = aij;
            (*col).ptr = aij;
            aij
        }
    }

    pub fn row_nnz(&self, row: *mut NPPROW) -> c_int {
        unsafe {
            let mut nnz = 0;
            let mut aij = (*row).ptr;
            while !aij.is_null() {
                nnz += 1;
                aij = (*aij).r_next;
            }
            nnz
        }
    }

    pub fn col_nnz(&self, col: *mut NPPCOL) -> c_int {
        unsafe {
            let mut nnz = 0;
            let mut aij = (*col).ptr;
            while !aij.is_null() {
                nnz += 1;
                aij = (*aij).c_next;
            }
            nnz
        }
    }

    pub fn push_tse(
        &mut self,
        func: Option<unsafe extern "C" fn(*mut NPP, *mut ()) -> c_int>,
        size: usize,
    ) -> *mut () {
        unsafe {
            let tse = DMP::get_atom(self.stack, mem::size_of::<NPPTSE>()) as *mut NPPTSE;
            (*tse).func = func;
            (*tse).info = DMP::get_atom(self.stack, size);
            (*tse).link = self.top;
            self.top = tse;
            (*tse).info
        }
    }

    pub fn erase_row(&mut self, row: *mut NPPROW) {
        unsafe {
            while !(*row).ptr.is_null() {
                let aij = (*row).ptr;
                (*row).ptr = (*aij).r_next;
                if (*aij).c_prev.is_null() {
                    (*(*aij).col).ptr = (*aij).c_next;
                } else {
                    (*(*aij).c_prev).c_next = (*aij).c_next;
                }
                if !(*aij).c_next.is_null() {
                    (*(*aij).c_next).c_prev = (*aij).c_prev;
                }
                DMP::free_atom(self.pool, aij as *mut (), mem::size_of::<NPPAIJ>());
            }
        }
    }

    pub fn del_row(&mut self, row: *mut NPPROW) {
        unsafe {
            if !(*row).name.is_null() {
                let len = CStr::from_ptr((*row).name).to_bytes().len() + 1;
                DMP::free_atom(self.pool, (*row).name as *mut (), len);
            }
            self.erase_row(row);
            self.remove_row(row);
            DMP::free_atom(self.pool, row as *mut (), mem::size_of::<NPPROW>());
        }
    }

    pub fn del_col(&mut self, col: *mut NPPCOL) {
        unsafe {
            if !(*col).name.is_null() {
                let len = CStr::from_ptr((*col).name).to_bytes().len() + 1;
                DMP::free_atom(self.pool, (*col).name as *mut (), len);
            }
            while !(*col).ptr.is_null() {
                let aij = (*col).ptr;
                (*col).ptr = (*aij).c_next;
                if (*aij).r_prev.is_null() {
                    (*(*aij).row).ptr = (*aij).r_next;
                } else {
                    (*(*aij).r_prev).r_next = (*aij).r_next;
                }
                if !(*aij).r_next.is_null() {
                    (*(*aij).r_next).r_prev = (*aij).r_prev;
                }
                DMP::free_atom(self.pool, aij as *mut (), mem::size_of::<NPPAIJ>());
            }
            self.remove_col(col);
            DMP::free_atom(self.pool, col as *mut (), mem::size_of::<NPPCOL>());
        }
    }

    pub fn del_aij(&mut self, aij: *mut NPPAIJ) {
        unsafe {
            if (*aij).r_prev.is_null() {
                (*(*aij).row).ptr = (*aij).r_next;
            } else {
                (*(*aij).r_prev).r_next = (*aij).r_next;
            }
            if !(*aij).r_next.is_null() {
                (*(*aij).r_next).r_prev = (*aij).r_prev;
            }
            if (*aij).c_prev.is_null() {
                (*(*aij).col).ptr = (*aij).c_next;
            } else {
                (*(*aij).c_prev).c_next = (*aij).c_next;
            }
            if !(*aij).c_next.is_null() {
                (*(*aij).c_next).c_prev = (*aij).c_prev;
            }
            DMP::free_atom(self.pool, aij as *mut (), mem::size_of::<NPPAIJ>());
        }
    }

    pub fn delete_wksp(npp: *mut NPP) {
        unsafe {
            if !(*npp).pool.is_null() {
                DMP::delete_pool((*npp).pool);
            }
            if !(*npp).stack.is_null() {
                DMP::delete_pool((*npp).stack);
            }
            if !(*npp).row_ref.is_null() {
                libc::free((*npp).row_ref as *mut ());
            }
            if !(*npp).col_ref.is_null() {
                libc::free((*npp).col_ref as *mut ());
            }
            if !(*npp).r_stat.is_null() {
                libc::free((*npp).r_stat as *mut ());
            }
            if !(*npp).r_pi.is_null() {
                libc::free((*npp).r_pi as *mut ());
            }
            if !(*npp).c_stat.is_null() {
                libc::free((*npp).c_stat as *mut ());
            }
            if !(*npp).c_value.is_null() {
                libc::free((*npp).c_value as *mut ());
            }
            libc::free(npp as *mut ());
        }
    }
}

impl DMP {
    pub fn create_pool() -> *mut DMP {
        // Implementation depends on memory pool implementation
        ptr::null_mut()
    }

    pub fn get_atom(pool: *mut DMP, size: usize) -> *mut () {
        // Implementation depends on memory pool implementation
        ptr::null_mut()
    }

    pub fn free_atom(pool: *mut DMP, atom: *mut (), size: usize) {
        // Implementation depends on memory pool implementation
    }

    pub fn delete_pool(pool: *mut DMP) {
        // Implementation depends on memory pool implementation
    }
}