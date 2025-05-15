/* GNU Guile interface for GNU Make.
Copyright (C) 2011-2023 Free Software Foundation, Inc.
This file is part of GNU Make.

GNU Make is free software; you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation; either version 3 of the License, or (at your option) any later
version.

GNU Make is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::{CString, CStr};
use std::ptr;
use std::sync::Once;

#[cfg(feature = "guile")]
mod guile_support {
    use super::*;
    use guile_sys::*;
    use std::os::raw::c_void;

    static mut MAKE_MOD: SCM = SCM_EOL;
    static mut OBJ_TO_STR: SCM = SCM_EOL;
    static INIT: Once = Once::new();

    /// Convert an SCM object into a string.
    fn cvt_scm_to_str(obj: SCM) -> Option<String> {
        unsafe {
            let str_obj = scm_call_1(OBJ_TO_STR, obj);
            let c_str = scm_to_locale_string(str_obj);
            if c_str.is_null() {
                None
            } else {
                let rust_str = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                libc::free(c_str as *mut c_void);
                Some(rust_str)
            }
        }
    }

    /// Perform the GNU Make expansion function.
    unsafe extern "C" fn guile_expand_wrapper(obj: SCM) -> SCM {
        if let Some(str) = cvt_scm_to_str(obj) {
            let c_str = CString::new(str).unwrap();
            let res = gmk_expand(c_str.as_ptr());
            let ret = scm_from_locale_string(res);
            libc::free(res as *mut c_void);
            ret
        } else {
            SCM_BOOL_F
        }
    }

    /// Perform the GNU Make eval function.
    unsafe extern "C" fn guile_eval_wrapper(obj: SCM) -> SCM {
        if let Some(str) = cvt_scm_to_str(obj) {
            let c_str = CString::new(str).unwrap();
            gmk_eval(c_str.as_ptr(), 0);
        }
        SCM_BOOL_F
    }

    /// Invoked by scm_c_define_module(), in the context of the GNU Make module.
    unsafe extern "C" fn guile_define_module(_data: *mut c_void) {
        // Ingest the predefined Guile module for GNU Make.
        include!("gmk-default.h");

        // Register a subr for GNU Make's eval capability.
        scm_c_define_gsubr(
            b"gmk-expand\0".as_ptr() as *const i8,
            1,
            0,
            0,
            Some(guile_expand_wrapper),
        );

        // Register a subr for GNU Make's eval capability.
        scm_c_define_gsubr(
            b"gmk-eval\0".as_ptr() as *const i8,
            1,
            0,
            0,
            Some(guile_eval_wrapper),
        );

        // Define the rest of the module.
        scm_c_eval_string(GUILE_module_defn.as_ptr() as *const i8);
    }

    /// Initialize the GNU Make Guile module.
    unsafe extern "C" fn guile_init(_arg: *mut c_void) -> *mut c_void {
        // Define the module.
        MAKE_MOD = scm_c_define_module(
            b"gnu make\0".as_ptr() as *const i8,
            Some(guile_define_module),
            ptr::null_mut(),
        );

        // Get a reference to the object-to-string translator, for later.
        OBJ_TO_STR = scm_variable_ref(scm_c_module_lookup(
            MAKE_MOD,
            b"obj-to-str\0".as_ptr() as *const i8,
        ));

        // Import the GNU Make module exports into the generic space.
        scm_c_eval_string(b"(use-modules (gnu make))\0".as_ptr() as *const i8);

        ptr::null_mut()
    }

    unsafe extern "C" fn internal_guile_eval(arg: *mut c_void) -> *mut c_void {
        let c_str = arg as *const i8;
        let input = CStr::from_ptr(c_str).to_string_lossy().into_owned();
        let result = scm_eval_string(scm_from_utf8_string(input.as_ptr() as *const i8));
        let output = cvt_scm_to_str(result).unwrap_or_default();
        CString::new(output).unwrap().into_raw() as *mut c_void
    }

    /// This is the function registered with make
    pub unsafe extern "C" fn func_guile(
        _funcname: *const i8,
        _argc: u32,
        argv: *mut *mut i8,
    ) -> *mut i8 {
        INIT.call_once(|| {
            scm_with_guile(Some(guile_init), ptr::null_mut());
        });

        if !(*argv).is_null() && *(*argv) != 0 {
            let result = scm_with_guile(Some(internal_guile_eval), *argv as *mut c_void);
            result as *mut i8
        } else {
            ptr::null_mut()
        }
    }

    /// Public interface for setting up Guile support
    pub unsafe extern "C" fn guile_gmake_setup(_flocp: *const c_void) -> i32 {
        gmk_add_function(
            b"guile\0".as_ptr() as *const i8,
            Some(func_guile),
            0,
            1,
            GMK_FUNC_DEFAULT,
        );
        1
    }
}

#[cfg(not(feature = "guile"))]
mod guile_support {
    use std::os::raw::c_void;

    /// Public interface when Guile is not supported
    pub unsafe extern "C" fn guile_gmake_setup(_flocp: *const c_void) -> i32 {
        1
    }
}

pub use guile_support::guile_gmake_setup;