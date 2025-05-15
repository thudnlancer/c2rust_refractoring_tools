#[no_mangle]
pub extern "C" fn _glp_amd_defaults(control: &mut [f64; 5]) {
    control.iter_mut().for_each(|x| *x = 0.0);
    control[0] = 10.0;
    control[1] = 1.0;
}