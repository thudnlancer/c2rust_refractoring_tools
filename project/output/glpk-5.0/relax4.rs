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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax4_csa {
    pub n: i32,
    pub na: i32,
    pub large: i32,
    pub repeat: i32,
    pub crash: i32,
    pub startn: *mut i32,
    pub endn: *mut i32,
    pub fou: *mut i32,
    pub nxtou: *mut i32,
    pub fin: *mut i32,
    pub nxtin: *mut i32,
    pub rc: *mut i32,
    pub u: *mut i32,
    pub dfct: *mut i32,
    pub x: *mut i32,
    pub nmultinode: i32,
    pub iter: i32,
    pub num_augm: i32,
    pub num_ascnt: i32,
    pub nsp: i32,
    pub label: *mut i32,
    pub prdcsr: *mut i32,
    pub save: *mut i32,
    pub tfstou: *mut i32,
    pub tnxtou: *mut i32,
    pub tfstin: *mut i32,
    pub tnxtin: *mut i32,
    pub nxtqueue: *mut i32,
    pub scan: *mut i8,
    pub mark: *mut i8,
    pub extend_arc: *mut i32,
    pub sb_level: *mut i32,
    pub sb_arc: *mut i32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_relax4(mut csa: *mut relax4_csa) -> i32 {
    static mut func: [i8; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b"relax4\0")
    };
    (csa == csa
        || {
            glp_assert_(
                b"csa == csa\0" as *const u8 as *const i8,
                b"misc/relax4.c\0" as *const u8 as *const i8,
                8 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"misc/relax4.c\0" as *const u8 as *const i8, 9 as i32))
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
pub unsafe extern "C" fn _glp_relax4_inidat(mut csa: *mut relax4_csa) {
    static mut func: [i8; 14] = unsafe {
        *::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"relax4_inidat\0")
    };
    (csa == csa
        || {
            glp_assert_(
                b"csa == csa\0" as *const u8 as *const i8,
                b"misc/relax4.c\0" as *const u8 as *const i8,
                17 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"misc/relax4.c\0" as *const u8 as *const i8, 18 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const i8,
        func.as_ptr(),
    );
    abort();
}