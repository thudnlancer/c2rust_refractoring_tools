use std::ffi::{c_char, c_double, c_int, c_void};
use std::ptr;

type MatFunc = Option<
    unsafe extern "C" fn(*mut c_void, c_int, *mut c_int, *mut c_double) -> c_int,
>;

pub fn glp_triang(
    m: c_int,
    n: c_int,
    mat: MatFunc,
    info: *mut c_void,
    tol: c_double,
    rn: *mut c_int,
    cn: *mut c_int,
) -> c_int {
    unsafe {
        let mut head = 0;
        let mut ns = 0;
        let mut size = 0;

        // Allocate memory
        let cind = glp_alloc(1 + m, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
        let cval = glp_alloc(1 + m, std::mem::size_of::<c_double>() as c_int) as *mut c_double;
        let rind = glp_alloc(1 + n, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
        let rval = glp_alloc(1 + n, std::mem::size_of::<c_double>() as c_int) as *mut c_double;
        let ptr = glp_alloc(1 + m, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
        let cnt = ptr;
        let list = glp_alloc(1 + n, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
        let prev = glp_alloc(1 + n, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
        let next = glp_alloc(1 + n, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
        let big = glp_alloc(1 + n, std::mem::size_of::<c_double>() as c_int) as *mut c_double;
        let flag = glp_alloc(1 + n, std::mem::size_of::<c_char>() as c_int) as *mut c_char;

        // Initialize ptr array
        for len in 0..=m {
            *ptr.offset(len as isize) = 0;
        }

        // Process columns
        for j in 1..=n {
            let len = mat.expect("non-null function pointer")(info, -j, cind, cval);
            assert!(0 <= len && len <= m, "0 <= len && len <= m");
            *next.offset(j as isize) = *ptr.offset(len as isize);
            *ptr.offset(len as isize) = j;
            *big.offset(j as isize) = 0.0;

            for k in 1..=len {
                let val = (*cval.offset(k as isize)).abs();
                if *big.offset(j as isize) < val {
                    *big.offset(j as isize) = val;
                }
            }
        }

        // Build linked list
        head = 0;
        for len in 0..=m {
            let mut j = *ptr.offset(len as isize);
            while j != 0 {
                let next_j = *next.offset(j as isize);
                *prev.offset(j as isize) = 0;
                *next.offset(j as isize) = head;
                if head != 0 {
                    *prev.offset(head as isize) = j;
                }
                head = j;
                j = next_j;
            }
        }

        // Initialize flags
        for j in 1..=n {
            *flag.offset(j as isize) = 1;
        }

        // Process rows
        for i in 1..=m {
            let len = mat.expect("non-null function pointer")(info, i, rind, rval);
            *cnt.offset(i as isize) = len;
            assert!(0 <= len && len <= n, "0 <= len && len <= n");

            if len == 1 {
                let j = *rind.offset(1);
                assert!(1 <= j && j <= n, "1 <= j && j <= n");
                if *flag.offset(j as isize) != 2 {
                    *flag.offset(j as isize) = 2;
                    ns += 1;
                    *list.offset(ns as isize) = j;
                }
            }
        }

        // Main processing loop
        while head != 0 {
            let (j, len) = if ns == 0 {
                let j = head;
                let len = mat.expect("non-null function pointer")(info, -j, cind, cval);
                assert!(0 <= len && len <= m, "0 <= len && len <= m");
                (j, len)
            } else {
                ns -= 1;
                let j = *list.offset((ns + 1) as isize);
                assert!(*flag.offset(j as isize) == 2, "flag[j] == 2");
                let len = mat.expect("non-null function pointer")(info, -j, cind, cval);
                assert!(0 <= len && len <= m, "0 <= len && len <= m");

                let mut kk = 0;
                for k in 1..=len {
                    let i = *cind.offset(k as isize);
                    assert!(1 <= i && i <= m, "1 <= i && i <= m");
                    if *cnt.offset(i as isize) == 1 {
                        if kk == 0 || (*cval.offset(kk as isize)).abs() < (*cval.offset(k as isize)).abs() {
                            kk = k;
                        }
                    }
                }
                assert!(kk > 0, "kk > 0");

                if (*cval.offset(kk as isize)).abs() >= tol * *big.offset(j as isize) {
                    size += 1;
                    *rn.offset(size as isize) = *cind.offset(kk as isize);
                    *cn.offset(size as isize) = j;
                }
                (j, len)
            };

            assert!(*flag.offset(j as isize) != 0, "flag[j]");
            *flag.offset(j as isize) = 0;

            if *prev.offset(j as isize) == 0 {
                head = *next.offset(j as isize);
            } else {
                *next.offset(*prev.offset(j as isize) as isize) = *next.offset(j as isize);
            }

            if *next.offset(j as isize) != 0 {
                *prev.offset(*next.offset(j as isize) as isize) = *prev.offset(j as isize);
            }

            for k in 1..=len {
                let i = *cind.offset(k as isize);
                assert!(1 <= i && i <= m, "1 <= i && i <= m");
                assert!(*cnt.offset(i as isize) > 0, "cnt[i] > 0");
                *cnt.offset(i as isize) -= 1;

                if *cnt.offset(i as isize) == 1 {
                    let len2 = mat.expect("non-null function pointer")(info, i, rind, rval);
                    assert!(0 <= len2 && len2 <= n, "0 <= len2 && len2 <= n");

                    let mut ks = 0;
                    for kk in 1..=len2 {
                        let jj = *rind.offset(kk as isize);
                        assert!(1 <= jj && jj <= n, "1 <= jj && jj <= n");
                        if *flag.offset(jj as isize) != 0 {
                            assert!(ks == 0, "ks == 0");
                            ks = kk;
                        }
                    }
                    assert!(ks > 0, "ks > 0");

                    let jj = *rind.offset(ks as isize);
                    if *flag.offset(jj as isize) != 2 {
                        *flag.offset(jj as isize) = 2;
                        ns += 1;
                        *list.offset(ns as isize) = jj;
                    }
                }
            }
        }

        // Verify all counts are zero
        for i in 1..=m {
            assert!(*cnt.offset(i as isize) == 0, "cnt[i] == 0");
        }

        // Free memory
        glp_free(cind as *mut c_void);
        glp_free(cval as *mut c_void);
        glp_free(rind as *mut c_void);
        glp_free(rval as *mut c_void);
        glp_free(ptr as *mut c_void);
        glp_free(list as *mut c_void);
        glp_free(prev as *mut c_void);
        glp_free(next as *mut c_void);
        glp_free(big as *mut c_void);
        glp_free(flag as *mut c_void);

        size
    }
}

// External C functions
extern "C" {
    fn fabs(_: c_double) -> c_double;
    fn glp_free(ptr: *mut c_void);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
}