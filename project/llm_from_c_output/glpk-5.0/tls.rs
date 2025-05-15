/* tls.rs (thread local storage) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2001-2017 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

#[cfg(feature = "thread_local")]
use std::thread_local;

#[cfg(not(feature = "thread_local"))]
static TLS: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());

#[cfg(feature = "thread_local")]
thread_local! {
    static TLS: *mut () = ptr::null_mut();
}

/***********************************************************************
*  NAME
*
*  tls_set_ptr - store global pointer in TLS
*
*  SYNOPSIS
*
*  use glpk::env;
*  fn tls_set_ptr(ptr: *mut ());
*
*  DESCRIPTION
*
*  The routine tls_set_ptr stores a pointer specified by the parameter
*  ptr in the Thread Local Storage (TLS). */

pub fn tls_set_ptr(ptr: *mut ()) {
    #[cfg(not(feature = "thread_local"))]
    TLS.store(ptr, Ordering::SeqCst);
    
    #[cfg(feature = "thread_local")]
    TLS.with(|tls| unsafe { *tls.as_ptr() = ptr });
}

/***********************************************************************
*  NAME
*
*  tls_get_ptr - retrieve global pointer from TLS
*
*  SYNOPSIS
*
*  use glpk::env;
*  fn tls_get_ptr() -> *mut ();
*
*  RETURNS
*
*  The routine tls_get_ptr returns a pointer previously stored by the
*  routine tls_set_ptr. If the latter has not been called yet, NULL is
*  returned. */

pub fn tls_get_ptr() -> *mut () {
    #[cfg(not(feature = "thread_local"))]
    TLS.load(Ordering::SeqCst)
    
    #[cfg(feature = "thread_local")]
    TLS.with(|tls| *tls)
}

/**********************************************************************/

#[cfg(windows)]
mod windows {
    use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONERROR};
    use winapi::um::winbase::GetVersion;
    use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
    use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH};

    const VISTA: DWORD = 0x06;

    #[no_mangle]
    pub extern "system" fn DllMain(
        _hinstDLL: HINSTANCE,
        fdwReason: DWORD,
        _lpvReserved: LPVOID,
    ) -> BOOL {
        #[cfg(feature = "thread_local")]
        {
            if fdwReason == DLL_PROCESS_ATTACH {
                let version = unsafe { GetVersion() };
                let major_version = version & 0xff;
                if major_version < VISTA {
                    unsafe {
                        MessageBoxA(
                            ptr::null_mut(),
                            b"The GLPK library called by this application is configured to use thread local storage which is not fully supported by your version of Microsoft Windows.\n\nMicrosoft Windows Vista or a later version of Windows is required to run this application.\0".as_ptr(),
                            b"GLPK\0".as_ptr(),
                            MB_OK | MB_ICONERROR,
                        );
                    }
                    return 0;
                }
            }
        }
        1
    }
}