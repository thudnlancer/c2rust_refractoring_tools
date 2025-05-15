#[no_mangle]
pub extern "C" fn priv_set_remove_linkdir() -> libc::c_int {
    -1
}

#[no_mangle]
pub extern "C" fn priv_set_restore_linkdir() -> libc::c_int {
    -1
}