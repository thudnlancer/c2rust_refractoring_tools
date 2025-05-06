#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn abort() -> !;
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[no_mangle]
pub unsafe extern "C" fn _glp_genqmd(
    mut neqns: *mut i32,
    mut xadj: *mut i32,
    mut adjncy: *mut i32,
    mut perm: *mut i32,
    mut invp: *mut i32,
    mut deg: *mut i32,
    mut marker: *mut i32,
    mut rchset: *mut i32,
    mut nbrhd: *mut i32,
    mut qsize: *mut i32,
    mut qlink: *mut i32,
    mut nofsub: *mut i32,
) {
    static mut func: [i8; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b"genqmd\0")
    };
    (neqns == neqns
        || {
            glp_assert_(
                b"neqns == neqns\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                10 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                11 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                12 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (perm == perm
        || {
            glp_assert_(
                b"perm == perm\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                13 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (invp == invp
        || {
            glp_assert_(
                b"invp == invp\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                14 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                15 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                16 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                17 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                18 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (qsize == qsize
        || {
            glp_assert_(
                b"qsize == qsize\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                19 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (qlink == qlink
        || {
            glp_assert_(
                b"qlink == qlink\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                20 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nofsub == nofsub
        || {
            glp_assert_(
                b"nofsub == nofsub\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                21 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const i8, 22 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const i8,
        func.as_ptr(),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn _glp_qmdrch(
    mut root: *mut i32,
    mut xadj: *mut i32,
    mut adjncy: *mut i32,
    mut deg: *mut i32,
    mut marker: *mut i32,
    mut rchsze: *mut i32,
    mut rchset: *mut i32,
    mut nhdsze: *mut i32,
    mut nbrhd: *mut i32,
) {
    static mut func: [i8; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b"qmdrch\0")
    };
    (root == root
        || {
            glp_assert_(
                b"root == root\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                31 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                32 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                33 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                34 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                35 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (rchsze == rchsze
        || {
            glp_assert_(
                b"rchsze == rchsze\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                36 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                37 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nhdsze == nhdsze
        || {
            glp_assert_(
                b"nhdsze == nhdsze\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                38 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                39 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const i8, 40 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const i8,
        func.as_ptr(),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn _glp_qmdqt(
    mut root: *mut i32,
    mut xadj: *mut i32,
    mut adjncy: *mut i32,
    mut marker: *mut i32,
    mut rchsze: *mut i32,
    mut rchset: *mut i32,
    mut nbrhd: *mut i32,
) {
    static mut func: [i8; 6] = unsafe {
        *::core::mem::transmute::<&[u8; 6], &[i8; 6]>(b"qmdqt\0")
    };
    (root == root
        || {
            glp_assert_(
                b"root == root\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                48 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                49 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                50 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                51 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (rchsze == rchsze
        || {
            glp_assert_(
                b"rchsze == rchsze\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                52 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                53 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                54 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const i8, 55 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const i8,
        func.as_ptr(),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn _glp_qmdupd(
    mut xadj: *mut i32,
    mut adjncy: *mut i32,
    mut nlist: *mut i32,
    mut list: *mut i32,
    mut deg: *mut i32,
    mut qsize: *mut i32,
    mut qlink: *mut i32,
    mut marker: *mut i32,
    mut rchset: *mut i32,
    mut nbrhd: *mut i32,
) {
    static mut func: [i8; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b"qmdupd\0")
    };
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                64 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                65 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nlist == nlist
        || {
            glp_assert_(
                b"nlist == nlist\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                66 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (list == list
        || {
            glp_assert_(
                b"list == list\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                67 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                68 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (qsize == qsize
        || {
            glp_assert_(
                b"qsize == qsize\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                69 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (qlink == qlink
        || {
            glp_assert_(
                b"qlink == qlink\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                70 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                71 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                72 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                73 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const i8, 74 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const i8,
        func.as_ptr(),
    );
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn _glp_qmdmrg(
    mut xadj: *mut i32,
    mut adjncy: *mut i32,
    mut deg: *mut i32,
    mut qsize: *mut i32,
    mut qlink: *mut i32,
    mut marker: *mut i32,
    mut deg0: *mut i32,
    mut nhdsze: *mut i32,
    mut nbrhd: *mut i32,
    mut rchset: *mut i32,
    mut ovrlp: *mut i32,
) {
    static mut func: [i8; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b"qmdmrg\0")
    };
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                83 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                84 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                85 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (qsize == qsize
        || {
            glp_assert_(
                b"qsize == qsize\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                86 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (qlink == qlink
        || {
            glp_assert_(
                b"qlink == qlink\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                87 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                88 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (deg0 == deg0
        || {
            glp_assert_(
                b"deg0 == deg0\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                89 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nhdsze == nhdsze
        || {
            glp_assert_(
                b"nhdsze == nhdsze\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                90 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                91 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                92 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (ovrlp == ovrlp
        || {
            glp_assert_(
                b"ovrlp == ovrlp\0" as *const u8 as *const i8,
                b"misc/qmd.c\0" as *const u8 as *const i8,
                93 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const i8, 94 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const i8,
        func.as_ptr(),
    );
    abort();
}