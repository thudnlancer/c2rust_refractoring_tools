use std::ffi::CString;
use std::process::abort;

type GlpErrFunc = Option<unsafe extern "C" fn(*const libc::c_char, ...)>;

fn glp_assert(expr: &str, file: &str, line: i32) {
    unsafe {
        let expr_c = CString::new(expr).unwrap();
        let file_c = CString::new(file).unwrap();
        glp_assert_(expr_c.as_ptr(), file_c.as_ptr(), line);
    }
}

fn glp_error(file: &str, line: i32) -> GlpErrFunc {
    unsafe {
        let file_c = CString::new(file).unwrap();
        glp_error_(file_c.as_ptr(), line)
    }
}

extern "C" {
    fn glp_assert_(expr: *const libc::c_char, file: *const libc::c_char, line: libc::c_int);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> GlpErrFunc;
}

#[no_mangle]
pub extern "C" fn _glp_genqmd(
    neqns: *mut libc::c_int,
    xadj: *mut libc::c_int,
    adjncy: *mut libc::c_int,
    perm: *mut libc::c_int,
    invp: *mut libc::c_int,
    deg: *mut libc::c_int,
    marker: *mut libc::c_int,
    rchset: *mut libc::c_int,
    nbrhd: *mut libc::c_int,
    qsize: *mut libc::c_int,
    qlink: *mut libc::c_int,
    nofsub: *mut libc::c_int,
) {
    let func = "genqmd";
    assert!(!neqns.is_null(), "neqns is null");
    assert!(!xadj.is_null(), "xadj is null");
    assert!(!adjncy.is_null(), "adjncy is null");
    assert!(!perm.is_null(), "perm is null");
    assert!(!invp.is_null(), "invp is null");
    assert!(!deg.is_null(), "deg is null");
    assert!(!marker.is_null(), "marker is null");
    assert!(!rchset.is_null(), "rchset is null");
    assert!(!nbrhd.is_null(), "nbrhd is null");
    assert!(!qsize.is_null(), "qsize is null");
    assert!(!qlink.is_null(), "qlink is null");
    assert!(!nofsub.is_null(), "nofsub is null");

    unsafe {
        if let Some(err_func) = glp_error("misc/qmd.c", 22) {
            err_func(
                CString::new("%s: sorry, this routine is temporarily disabled due to licensing problems\n")
                    .unwrap()
                    .as_ptr(),
                CString::new(func).unwrap().as_ptr(),
            );
        }
        abort();
    }
}

#[no_mangle]
pub extern "C" fn _glp_qmdrch(
    root: *mut libc::c_int,
    xadj: *mut libc::c_int,
    adjncy: *mut libc::c_int,
    deg: *mut libc::c_int,
    marker: *mut libc::c_int,
    rchsze: *mut libc::c_int,
    rchset: *mut libc::c_int,
    nhdsze: *mut libc::c_int,
    nbrhd: *mut libc::c_int,
) {
    let func = "qmdrch";
    assert!(!root.is_null(), "root is null");
    assert!(!xadj.is_null(), "xadj is null");
    assert!(!adjncy.is_null(), "adjncy is null");
    assert!(!deg.is_null(), "deg is null");
    assert!(!marker.is_null(), "marker is null");
    assert!(!rchsze.is_null(), "rchsze is null");
    assert!(!rchset.is_null(), "rchset is null");
    assert!(!nhdsze.is_null(), "nhdsze is null");
    assert!(!nbrhd.is_null(), "nbrhd is null");

    unsafe {
        if let Some(err_func) = glp_error("misc/qmd.c", 40) {
            err_func(
                CString::new("%s: sorry, this routine is temporarily disabled due to licensing problems\n")
                    .unwrap()
                    .as_ptr(),
                CString::new(func).unwrap().as_ptr(),
            );
        }
        abort();
    }
}

#[no_mangle]
pub extern "C" fn _glp_qmdqt(
    root: *mut libc::c_int,
    xadj: *mut libc::c_int,
    adjncy: *mut libc::c_int,
    marker: *mut libc::c_int,
    rchsze: *mut libc::c_int,
    rchset: *mut libc::c_int,
    nbrhd: *mut libc::c_int,
) {
    let func = "qmdqt";
    assert!(!root.is_null(), "root is null");
    assert!(!xadj.is_null(), "xadj is null");
    assert!(!adjncy.is_null(), "adjncy is null");
    assert!(!marker.is_null(), "marker is null");
    assert!(!rchsze.is_null(), "rchsze is null");
    assert!(!rchset.is_null(), "rchset is null");
    assert!(!nbrhd.is_null(), "nbrhd is null");

    unsafe {
        if let Some(err_func) = glp_error("misc/qmd.c", 55) {
            err_func(
                CString::new("%s: sorry, this routine is temporarily disabled due to licensing problems\n")
                    .unwrap()
                    .as_ptr(),
                CString::new(func).unwrap().as_ptr(),
            );
        }
        abort();
    }
}

#[no_mangle]
pub extern "C" fn _glp_qmdupd(
    xadj: *mut libc::c_int,
    adjncy: *mut libc::c_int,
    nlist: *mut libc::c_int,
    list: *mut libc::c_int,
    deg: *mut libc::c_int,
    qsize: *mut libc::c_int,
    qlink: *mut libc::c_int,
    marker: *mut libc::c_int,
    rchset: *mut libc::c_int,
    nbrhd: *mut libc::c_int,
) {
    let func = "qmdupd";
    assert!(!xadj.is_null(), "xadj is null");
    assert!(!adjncy.is_null(), "adjncy is null");
    assert!(!nlist.is_null(), "nlist is null");
    assert!(!list.is_null(), "list is null");
    assert!(!deg.is_null(), "deg is null");
    assert!(!qsize.is_null(), "qsize is null");
    assert!(!qlink.is_null(), "qlink is null");
    assert!(!marker.is_null(), "marker is null");
    assert!(!rchset.is_null(), "rchset is null");
    assert!(!nbrhd.is_null(), "nbrhd is null");

    unsafe {
        if let Some(err_func) = glp_error("misc/qmd.c", 74) {
            err_func(
                CString::new("%s: sorry, this routine is temporarily disabled due to licensing problems\n")
                    .unwrap()
                    .as_ptr(),
                CString::new(func).unwrap().as_ptr(),
            );
        }
        abort();
    }
}

#[no_mangle]
pub extern "C" fn _glp_qmdmrg(
    xadj: *mut libc::c_int,
    adjncy: *mut libc::c_int,
    deg: *mut libc::c_int,
    qsize: *mut libc::c_int,
    qlink: *mut libc::c_int,
    marker: *mut libc::c_int,
    deg0: *mut libc::c_int,
    nhdsze: *mut libc::c_int,
    nbrhd: *mut libc::c_int,
    rchset: *mut libc::c_int,
    ovrlp: *mut libc::c_int,
) {
    let func = "qmdmrg";
    assert!(!xadj.is_null(), "xadj is null");
    assert!(!adjncy.is_null(), "adjncy is null");
    assert!(!deg.is_null(), "deg is null");
    assert!(!qsize.is_null(), "qsize is null");
    assert!(!qlink.is_null(), "qlink is null");
    assert!(!marker.is_null(), "marker is null");
    assert!(!deg0.is_null(), "deg0 is null");
    assert!(!nhdsze.is_null(), "nhdsze is null");
    assert!(!nbrhd.is_null(), "nbrhd is null");
    assert!(!rchset.is_null(), "rchset is null");
    assert!(!ovrlp.is_null(), "ovrlp is null");

    unsafe {
        if let Some(err_func) = glp_error("misc/qmd.c", 94) {
            err_func(
                CString::new("%s: sorry, this routine is temporarily disabled due to licensing problems\n")
                    .unwrap()
                    .as_ptr(),
                CString::new(func).unwrap().as_ptr(),
            );
        }
        abort();
    }
}