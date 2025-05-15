// Hook for making file descriptor functions close(), ioctl() extensible.
// Copyright (C) 2009-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published
// by the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::{Arc, Mutex};

type GlCloseFn = fn(fd: i32) -> i32;
type GlIoctlFn = fn(fd: i32, request: i32, arg: *mut std::ffi::c_void) -> i32;

#[derive(Debug)]
struct FdHook {
    next: Arc<Mutex<Option<Arc<Mutex<FdHook>>>>>,
    prev: Arc<Mutex<Option<Arc<Mutex<FdHook>>>>>,
    close_fn: Option<CloseHookFn>,
    ioctl_fn: Option<IoctlHookFn>,
}

type CloseHookFn = fn(
    remaining_list: Arc<Mutex<FdHook>>,
    primary: GlCloseFn,
    fd: i32,
) -> i32;

type IoctlHookFn = fn(
    remaining_list: Arc<Mutex<FdHook>>,
    primary: GlIoctlFn,
    fd: i32,
    request: i32,
    arg: *mut std::ffi::c_void,
) -> i32;

struct FdHookManager {
    anchor: Arc<Mutex<FdHook>>,
}

impl FdHookManager {
    fn new() -> Self {
        let anchor = Arc::new(Mutex::new(FdHook {
            next: Arc::new(Mutex::new(None)),
            prev: Arc::new(Mutex::new(None)),
            close_fn: None,
            ioctl_fn: None,
        }));
        *anchor.lock().unwrap().next = Some(anchor.clone());
        *anchor.lock().unwrap().prev = Some(anchor.clone());
        Self { anchor }
    }

    fn execute_close_hooks(
        &self,
        remaining_list: Arc<Mutex<FdHook>>,
        primary: GlCloseFn,
        fd: i32,
    ) -> i32 {
        if Arc::ptr_eq(&remaining_list, &self.anchor) {
            primary(fd)
        } else {
            let hook = remaining_list.lock().unwrap();
            if let Some(close_fn) = hook.close_fn {
                let next = hook.next.clone();
                close_fn(next.unwrap(), primary, fd)
            } else {
                primary(fd)
            }
        }
    }

    fn execute_all_close_hooks(&self, primary: GlCloseFn, fd: i32) -> i32 {
        let next = self.anchor.lock().unwrap().next.clone();
        self.execute_close_hooks(next.unwrap(), primary, fd)
    }

    fn execute_ioctl_hooks(
        &self,
        remaining_list: Arc<Mutex<FdHook>>,
        primary: GlIoctlFn,
        fd: i32,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        if Arc::ptr_eq(&remaining_list, &self.anchor) {
            primary(fd, request, arg)
        } else {
            let hook = remaining_list.lock().unwrap();
            if let Some(ioctl_fn) = hook.ioctl_fn {
                let next = hook.next.clone();
                ioctl_fn(next.unwrap(), primary, fd, request, arg)
            } else {
                primary(fd, request, arg)
            }
        }
    }

    fn execute_all_ioctl_hooks(
        &self,
        primary: GlIoctlFn,
        fd: i32,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        let next = self.anchor.lock().unwrap().next.clone();
        self.execute_ioctl_hooks(next.unwrap(), primary, fd, request, arg)
    }

    fn register_fd_hook(
        &self,
        close_hook: Option<CloseHookFn>,
        ioctl_hook: Option<IoctlHookFn>,
        link: Arc<Mutex<FdHook>>,
    ) {
        let mut link_guard = link.lock().unwrap();
        let next_guard = link_guard.next.lock().unwrap();
        let prev_guard = link_guard.prev.lock().unwrap();

        if next_guard.is_none() && prev_guard.is_none() {
            let close_fn = close_hook.unwrap_or(Self::execute_close_hooks);
            let ioctl_fn = ioctl_hook.unwrap_or(Self::execute_ioctl_hooks);

            let anchor_next = self.anchor.lock().unwrap().next.clone();
            *link_guard.next = anchor_next;
            *link_guard.prev = Some(self.anchor.clone());
            link_guard.close_fn = Some(close_fn);
            link_guard.ioctl_fn = Some(ioctl_fn);

            let mut anchor_next_guard = anchor_next.unwrap().lock().unwrap();
            *anchor_next_guard.prev = Some(link.clone());
            *self.anchor.lock().unwrap().next = Some(link.clone());
        } else if link_guard.close_fn != close_hook || link_guard.ioctl_fn != ioctl_hook {
            panic!("Hook already registered with different functions");
        }
    }

    fn unregister_fd_hook(&self, link: Arc<Mutex<FdHook>>) {
        let mut link_guard = link.lock().unwrap();
        let next = link_guard.next.clone();
        let prev = link_guard.prev.clone();

        if next.is_some() && prev.is_some() {
            let mut next_guard = next.unwrap().lock().unwrap();
            let mut prev_guard = prev.unwrap().lock().unwrap();

            *prev_guard.next = next.clone();
            *next_guard.prev = prev.clone();

            *link_guard.next = None;
            *link_guard.prev = None;
            link_guard.close_fn = None;
            link_guard.ioctl_fn = None;
        }
    }
}

fn default_close_hook(
    remaining_list: Arc<Mutex<FdHook>>,
    primary: GlCloseFn,
    fd: i32,
) -> i32 {
    let manager = FdHookManager::new();
    manager.execute_close_hooks(remaining_list, primary, fd)
}

fn default_ioctl_hook(
    remaining_list: Arc<Mutex<FdHook>>,
    primary: GlIoctlFn,
    fd: i32,
    request: i32,
    arg: *mut std::ffi::c_void,
) -> i32 {
    let manager = FdHookManager::new();
    manager.execute_ioctl_hooks(remaining_list, primary, fd, request, arg)
}