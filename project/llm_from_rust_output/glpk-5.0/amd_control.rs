use std::ffi::CString;
use std::os::raw::{c_double, c_int};

fn glp_printf(fmt: &str, args: std::fmt::Arguments) {
    let fmt_cstr = CString::new(fmt).unwrap();
    unsafe {
        libc::printf(fmt_cstr.as_ptr(), args);
    }
}

pub fn glp_amd_control(control: Option<&[c_double]>) {
    let (alpha, aggressive) = match control {
        Some(ctrl) => (
            ctrl[0],
            if ctrl[1] != 0.0 { 1 } else { 0 },
        ),
        None => (10.0, 1),
    };

    glp_printf(
        "\nAMD version {}.{}.{}, {}: approximate minimum degree ordering\n    dense row parameter: {}\n\0",
        format_args!("2", "2", "0", "May 31, 2007", alpha)
    );

    if alpha < 0.0 {
        glp_printf("    no rows treated as dense\n\0", format_args!());
    } else {
        glp_printf(
            "    (rows with more than max ({} * sqrt (n), 16) entries are\n    considered \"dense\", and placed last in output permutation)\n\0",
            format_args!(alpha)
        );
    }

    if aggressive != 0 {
        glp_printf("    aggressive absorption:  yes\n\0", format_args!());
    } else {
        glp_printf("    aggressive absorption:  no\n\0", format_args!());
    }

    glp_printf(
        "    size of AMD integer: {}\n\n\0",
        format_args!(std::mem::size_of::<c_int>())
    );
}