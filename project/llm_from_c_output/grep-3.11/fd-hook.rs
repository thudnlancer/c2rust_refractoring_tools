// Hook for making file descriptor functions close(), ioctl() extensible.
// Copyright (C) 2009-2023 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::{Arc, Mutex, Weak};

pub type GlCloseFn = Box<dyn Fn(i32) -> i32 + Send + Sync>;
pub type GlIoctlFn = Box<dyn Fn(i32, i32, *mut std::ffi::c_void) -> i32 + Send + Sync>;

pub struct FdHook {
    next: Mutex<Option<Arc<FdHook>>>,
    prev: Mutex<Weak<FdHook>>,
    close_fn: Mutex<Option<CloseHookFn>>,
    ioctl_fn: Mutex<Option<IoctlHookFn>>,
}

pub type CloseHookFn = Box<dyn Fn(&Arc<FdHook>, &GlCloseFn, i32) -> i32 + Send + Sync>;
pub type IoctlHookFn = Box<dyn Fn(&Arc<FdHook>, &GlIoctlFn, i32, i32, *mut std::ffi::c_void) -> i32 + Send + Sync>;

lazy_static::lazy_static! {
    static ref ANCHOR: Arc<FdHook> = Arc::new(FdHook {
        next: Mutex::new(None),
        prev: Mutex::new(Weak::new()),
        close_fn: Mutex::new(None),
        ioctl_fn: Mutex::new(None),
    });
}

pub fn execute_close_hooks(remaining_list: &Arc<FdHook>, primary: &GlCloseFn, fd: i32) -> i32 {
    if Arc::ptr_eq(remaining_list, &ANCHOR) {
        primary(fd)
    } else {
        let close_fn = remaining_list.close_fn.lock().unwrap();
        if let Some(ref close_fn) = *close_fn {
            let next = remaining_list.next.lock().unwrap();
            if let Some(ref next) = *next {
                close_fn(next, primary, fd)
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

pub fn execute_all_close_hooks(primary: &GlCloseFn, fd: i32) -> i32 {
    let next = ANCHOR.next.lock().unwrap();
    if let Some(ref next) = *next {
        execute_close_hooks(next, primary, fd)
    } else {
        primary(fd)
    }
}

pub fn execute_ioctl_hooks(
    remaining_list: &Arc<FdHook>,
    primary: &GlIoctlFn,
    fd: i32,
    request: i32,
    arg: *mut std::ffi::c_void,
) -> i32 {
    if Arc::ptr_eq(remaining_list, &ANCHOR) {
        primary(fd, request, arg)
    } else {
        let ioctl_fn = remaining_list.ioctl_fn.lock().unwrap();
        if let Some(ref ioctl_fn) = *ioctl_fn {
            let next = remaining_list.next.lock().unwrap();
            if let Some(ref next) = *next {
                ioctl_fn(next, primary, fd, request, arg)
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

pub fn execute_all_ioctl_hooks(
    primary: &GlIoctlFn,
    fd: i32,
    request: i32,
    arg: *mut std::ffi::c_void,
) -> i32 {
    let next = ANCHOR.next.lock().unwrap();
    if let Some(ref next) = *next {
        execute_ioctl_hooks(next, primary, fd, request, arg)
    } else {
        primary(fd, request, arg)
    }
}

pub fn register_fd_hook(
    close_hook: Option<CloseHookFn>,
    ioctl_hook: Option<IoctlHookFn>,
    link: &Arc<FdHook>,
) {
    let mut link_next = link.next.lock().unwrap();
    let mut link_prev = link.prev.lock().unwrap();

    if link_next.is_none() && link_prev.upgrade().is_none() {
        let close_fn = close_hook.unwrap_or_else(|| {
            Box::new(|remaining_list: &Arc<FdHook>, primary: &GlCloseFn, fd: i32| {
                execute_close_hooks(remaining_list, primary, fd)
            })
        });

        let ioctl_fn = ioctl_hook.unwrap_or_else(|| {
            Box::new(
                |remaining_list: &Arc<FdHook>,
                 primary: &GlIoctlFn,
                 fd: i32,
                 request: i32,
                 arg: *mut std::ffi::c_void| {
                    execute_ioctl_hooks(remaining_list, primary, fd, request, arg)
                },
            )
        });

        let mut anchor_next = ANCHOR.next.lock().unwrap();
        let mut anchor_prev = ANCHOR.prev.lock().unwrap();

        *link_next = Some(Arc::clone(&ANCHOR));
        *link_prev = Arc::downgrade(&ANCHOR);
        *link.close_fn.lock().unwrap() = Some(close_fn);
        *link.ioctl_fn.lock().unwrap() = Some(ioctl_fn);

        if let Some(ref old_next) = *anchor_next {
            *old_next.prev.lock().unwrap() = Arc::downgrade(link);
        }

        *link_next = anchor_next.take();
        *anchor_next = Some(Arc::clone(link));
    } else {
        let link_close_fn = link.close_fn.lock().unwrap();
        let link_ioctl_fn = link.ioctl_fn.lock().unwrap();

        if let Some(ref close_fn) = *link_close_fn {
            if let Some(ref ioctl_fn) = *link_ioctl_fn {
                if close_hook.map_or(false, |ch| !Arc::ptr_eq(&ch, close_fn))
                    || ioctl_hook.map_or(false, |ih| !Arc::ptr_eq(&ih, ioctl_fn))
                {
                    panic!("Hook functions mismatch");
                }
            }
        }
    }
}

pub fn unregister_fd_hook(link: &Arc<FdHook>) {
    let mut link_next = link.next.lock().unwrap();
    let mut link_prev = link.prev.lock().unwrap();

    if let Some(ref next) = *link_next {
        if let Some(prev) = link_prev.upgrade() {
            *prev.next.lock().unwrap() = Some(Arc::clone(next));
            *next.prev.lock().unwrap() = Arc::downgrade(&prev);

            *link_next = None;
            *link_prev = Weak::new();
            *link.close_fn.lock().unwrap() = None;
            *link.ioctl_fn.lock().unwrap() = None;
        }
    }
}