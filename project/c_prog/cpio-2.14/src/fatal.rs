use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn fatal_exit() {
    exit(2 as libc::c_int);
}
