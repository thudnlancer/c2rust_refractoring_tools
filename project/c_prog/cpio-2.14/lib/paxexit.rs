use ::libc;
extern "C" {
    fn exit(_: libc::c_int) -> !;
    static mut exit_status: libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pax_exit() {
    exit(exit_status);
}
