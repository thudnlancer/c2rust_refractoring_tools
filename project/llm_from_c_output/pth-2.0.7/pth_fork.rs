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
**  pth_fork.c: Pth process forking support
*/
                             /* ``Every day of my life
                                  I am forced to add another
                                  name to the list of people
                                  who piss me off!''
                                            -- Calvin          */

use std::process;
use std::io::{Error, ErrorKind};

const PTH_ATFORK_MAX: usize = 32;

struct PthAtfork {
    prepare: Option<Box<dyn FnMut(&mut ())>>,
    parent: Option<Box<dyn FnMut(&mut ())>>,
    child: Option<Box<dyn FnMut(&mut ())>>,
    arg: Box<dyn std::any::Any>,
}

static mut PTH_ATFORK_LIST: [Option<PthAtfork>; PTH_ATFORK_MAX] = [None; PTH_ATFORK_MAX];
static mut PTH_ATFORK_IDX: usize = 0;

fn pth_atfork_push(
    prepare: Option<Box<dyn FnMut(&mut ())>>,
    parent: Option<Box<dyn FnMut(&mut ())>>,
    child: Option<Box<dyn FnMut(&mut ())>>,
    arg: Box<dyn std::any::Any>,
) -> Result<(), Error> {
    unsafe {
        if PTH_ATFORK_IDX > PTH_ATFORK_MAX - 1 {
            return Err(Error::new(ErrorKind::Other, "No space left in fork handlers list"));
        }
        PTH_ATFORK_LIST[PTH_ATFORK_IDX] = Some(PthAtfork {
            prepare,
            parent,
            child,
            arg,
        });
        PTH_ATFORK_IDX += 1;
    }
    Ok(())
}

fn pth_atfork_pop() -> Result<(), Error> {
    unsafe {
        if PTH_ATFORK_IDX <= 0 {
            return Err(Error::new(ErrorKind::Other, "No fork handlers to pop"));
        }
        PTH_ATFORK_IDX -= 1;
    }
    Ok(())
}

fn pth_fork() -> Result<process::Child, Error> {
    unsafe {
        // Run preparation handlers in LIFO order
        for i in (0..PTH_ATFORK_IDX).rev() {
            if let Some(handler) = &mut PTH_ATFORK_LIST[i] {
                if let Some(prepare) = &mut handler.prepare {
                    prepare(&mut ());
                }
            }
        }

        // Fork the process
        let child = process::Command::new(std::env::current_exe()?).spawn()?;

        if child.id() != 0 {
            // Parent
            for i in 0..PTH_ATFORK_IDX {
                if let Some(handler) = &mut PTH_ATFORK_LIST[i] {
                    if let Some(parent) = &mut handler.parent {
                        parent(&mut ());
                    }
                }
            }
        } else {
            // Child
            pth_scheduler_drop()?;

            for i in 0..PTH_ATFORK_IDX {
                if let Some(handler) = &mut PTH_ATFORK_LIST[i] {
                    if let Some(child) = &mut handler.child {
                        child(&mut ());
                    }
                }
            }
        }

        Ok(child)
    }
}

fn pth_scheduler_drop() -> Result<(), Error> {
    // Implementation of scheduler drop would go here
    Ok(())
}

fn pth_error(success: bool, err: ErrorKind) -> Result<bool, Error> {
    if success {
        Ok(true)
    } else {
        Err(Error::new(err, ""))
    }
}