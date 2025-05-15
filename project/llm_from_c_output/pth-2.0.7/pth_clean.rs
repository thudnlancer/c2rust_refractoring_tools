/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  pth_cleanup.c: Pth per-thread cleanup handler
*/
                             /* ``The concept seems to be clear by now.
                                  It has been defined several times
                                  by example of what it is not.''
                                                       -- Unknown */

use std::boxed::Box;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct PthError {
    kind: PthErrorKind,
    message: String,
}

#[derive(Debug)]
enum PthErrorKind {
    InvalidArgument,
    OutOfMemory,
}

impl fmt::Display for PthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PthError {}

struct PthCleanup {
    next: Option<Box<PthCleanup>>,
    func: Box<dyn FnOnce(*mut ())>,
    arg: *mut (),
}

struct PthThread {
    cleanups: Option<Box<PthCleanup>>,
}

fn pth_cleanup_push(func: Box<dyn FnOnce(*mut ())>, arg: *mut ()) -> Result<(), PthError> {
    if func.is_null() {
        return Err(PthError {
            kind: PthErrorKind::InvalidArgument,
            message: "Function pointer is null".to_string(),
        });
    }

    let cleanup = Box::new(PthCleanup {
        next: pth_current().cleanups.take(),
        func,
        arg,
    });

    pth_current().cleanups = Some(cleanup);
    Ok(())
}

fn pth_cleanup_pop(execute: bool) -> bool {
    let mut rc = false;
    if let Some(mut cleanup) = pth_current().cleanups.take() {
        pth_current().cleanups = cleanup.next.take();
        if execute {
            (cleanup.func)(cleanup.arg);
        }
        rc = true;
    }
    rc
}

fn pth_cleanup_popall(t: &mut PthThread, execute: bool) {
    while let Some(mut cleanup) = t.cleanups.take() {
        t.cleanups = cleanup.next.take();
        if execute {
            (cleanup.func)(cleanup.arg);
        }
    }
}

// Helper function to get current thread (simplified for translation)
fn pth_current() -> &'static mut PthThread {
    static mut CURRENT: PthThread = PthThread { cleanups: None };
    unsafe { &mut CURRENT }
}