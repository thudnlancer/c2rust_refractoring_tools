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

use std::sync::{Arc, Mutex};
use std::os::unix::io::RawFd;

type GlCloseFn = fn(RawFd) -> i32;
type GlIoctlFn = fn(RawFd, i32, *mut std::ffi::c_void) -> i32;

#[derive(Debug)]
struct FdHook {
    next: Option<Arc<Mutex<FdHook>>>,
    prev: Option<Arc<Mutex<FdHook>>>,
    close_fn: Option<CloseHookFn>,
    ioctl_fn: Option<IoctlHookFn>,
}

type CloseHookFn = fn(&Arc<Mutex<FdHook>>, GlCloseFn, RawFd) -> i32;
type IoctlHookFn = fn(&Arc<Mutex<FdHook>>, GlIoctlFn, RawFd, i32, *mut std::ffi::c_void) -> i32;

struct FdHookManager {
    anchor: Arc<Mutex<FdHook>>,
}

impl FdHookManager {
    fn new() -> Self {
        let anchor = Arc::new(Mutex::new(FdHook {
            next: None,
            prev: None,
            close_fn: None,
            ioctl_fn: None,
        }));
        let mut anchor_lock = anchor.lock().unwrap();
        anchor_lock.next = Some(anchor.clone());
        anchor_lock.prev = Some(anchor.clone());
        drop(anchor_lock);
        
        FdHookManager { anchor }
    }

    fn execute_close_hooks(&self, remaining_list: &Arc<Mutex<FdHook>>, primary: GlCloseFn, fd: RawFd) -> i32 {
        let remaining = remaining_list.lock().unwrap();
        if Arc::ptr_eq(remaining_list, &self.anchor) {
            primary(fd)
        } else {
            if let Some(ref close_fn) = remaining.close_fn {
                if let Some(ref next) = remaining.next {
                    close_fn(next, primary, fd)
                } else {
                    primary(fd)
                }
            } else {
                primary(fd)
            }
        }
    }

    fn execute_all_close_hooks(&self, primary: GlCloseFn, fd: RawFd) -> i32 {
        let next = self.anchor.lock().unwrap().next.clone();
        if let Some(next) = next {
            self.execute_close_hooks(&next, primary, fd)
        } else {
            primary(fd)
        }
    }

    fn execute_ioctl_hooks(
        &self,
        remaining_list: &Arc<Mutex<FdHook>>,
        primary: GlIoctlFn,
        fd: RawFd,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        let remaining = remaining_list.lock().unwrap();
        if Arc::ptr_eq(remaining_list, &self.anchor) {
            primary(fd, request, arg)
        } else {
            if let Some(ref ioctl_fn) = remaining.ioctl_fn {
                if let Some(ref next) = remaining.next {
                    ioctl_fn(next, primary, fd, request, arg)
                } else {
                    primary(fd, request, arg)
                }
            } else {
                primary(fd, request, arg)
            }
        }
    }

    fn execute_all_ioctl_hooks(
        &self,
        primary: GlIoctlFn,
        fd: RawFd,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        let next = self.anchor.lock().unwrap().next.clone();
        if let Some(next) = next {
            self.execute_ioctl_hooks(&next, primary, fd, request, arg)
        } else {
            primary(fd, request, arg)
        }
    }

    fn register_fd_hook(
        &self,
        close_hook: Option<CloseHookFn>,
        ioctl_hook: Option<IoctlHookFn>,
        link: Arc<Mutex<FdHook>>,
    ) {
        let close_hook = close_hook.unwrap_or(|next, primary, fd| {
            let manager = FdHookManager::new();
            manager.execute_close_hooks(next, primary, fd)
        });

        let ioctl_hook = ioctl_hook.unwrap_or(|next, primary, fd, request, arg| {
            let manager = FdHookManager::new();
            manager.execute_ioctl_hooks(next, primary, fd, request, arg)
        });

        let mut link_lock = link.lock().unwrap();
        if link_lock.next.is_none() && link_lock.prev.is_none() {
            let mut anchor_lock = self.anchor.lock().unwrap();
            let old_next = anchor_lock.next.take().unwrap();

            link_lock.next = Some(old_next.clone());
            link_lock.prev = Some(self.anchor.clone());
            link_lock.close_fn = Some(close_hook);
            link_lock.ioctl_fn = Some(ioctl_hook);

            old_next.lock().unwrap().prev = Some(link.clone());
            anchor_lock.next = Some(link.clone());
        } else {
            if link_lock.close_fn != Some(close_hook) || link_lock.ioctl_fn != Some(ioctl_hook) {
                panic!("Hook already registered with different functions");
            }
        }
    }

    fn unregister_fd_hook(&self, link: Arc<Mutex<FdHook>>) {
        let mut link_lock = link.lock().unwrap();
        if let (Some(next), Some(prev)) = (link_lock.next.take(), link_lock.prev.take()) {
            let mut next_lock = next.lock().unwrap();
            let mut prev_lock = prev.lock().unwrap();

            prev_lock.next = Some(next.clone());
            next_lock.prev = Some(prev.clone());

            link_lock.close_fn = None;
            link_lock.ioctl_fn = None;
        }
    }
}

// Default implementations for the hook functions
fn default_close_hook(next: &Arc<Mutex<FdHook>>, primary: GlCloseFn, fd: RawFd) -> i32 {
    let manager = FdHookManager::new();
    manager.execute_close_hooks(next, primary, fd)
}

fn default_ioctl_hook(
    next: &Arc<Mutex<FdHook>>,
    primary: GlIoctlFn,
    fd: RawFd,
    request: i32,
    arg: *mut std::ffi::c_void,
) -> i32 {
    let manager = FdHookManager::new();
    manager.execute_ioctl_hooks(next, primary, fd, request, arg)
}