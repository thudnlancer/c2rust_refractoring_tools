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

type GlCloseFn = Box<dyn Fn(i32) -> i32 + Send + Sync>;
type GlIoctlFn = Box<dyn Fn(i32, i32, *mut std::ffi::c_void) -> i32 + Send + Sync>;

#[derive(Debug)]
struct FdHook {
    next: Option<Arc<Mutex<FdHook>>>,
    prev: Option<Arc<Mutex<FdHook>>>,
    close_fn: Option<Box<dyn Fn(Option<Arc<Mutex<FdHook>>, GlCloseFn, i32) -> i32 + Send + Sync>>,
    ioctl_fn: Option<Box<dyn Fn(Option<Arc<Mutex<FdHook>>, GlIoctlFn, i32, i32, *mut std::ffi::c_void) -> i32 + Send + Sync>>,
}

impl FdHook {
    fn new() -> Self {
        FdHook {
            next: None,
            prev: None,
            close_fn: None,
            ioctl_fn: None,
        }
    }
}

#[derive(Clone)]
struct FdHookManager {
    anchor: Arc<Mutex<FdHook>>,
}

impl FdHookManager {
    fn new() -> Self {
        let anchor = Arc::new(Mutex::new(FdHook::new()));
        let mut anchor_guard = anchor.lock().unwrap();
        anchor_guard.next = Some(anchor.clone());
        anchor_guard.prev = Some(anchor.clone());
        drop(anchor_guard);
        FdHookManager { anchor }
    }

    fn execute_close_hooks(
        &self,
        remaining_list: Option<Arc<Mutex<FdHook>>>,
        primary: GlCloseFn,
        fd: i32,
    ) -> i32 {
        if let Some(list) = remaining_list {
            let list_guard = list.lock().unwrap();
            if Arc::ptr_eq(&list, &self.anchor) {
                primary(fd)
            } else if let Some(close_fn) = &list_guard.close_fn {
                close_fn(list_guard.next.clone(), primary, fd)
            } else {
                primary(fd)
            }
        } else {
            primary(fd)
        }
    }

    fn execute_all_close_hooks(&self, primary: GlCloseFn, fd: i32) -> i32 {
        let anchor_guard = self.anchor.lock().unwrap();
        self.execute_close_hooks(anchor_guard.next.clone(), primary, fd)
    }

    fn execute_ioctl_hooks(
        &self,
        remaining_list: Option<Arc<Mutex<FdHook>>>,
        primary: GlIoctlFn,
        fd: i32,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        if let Some(list) = remaining_list {
            let list_guard = list.lock().unwrap();
            if Arc::ptr_eq(&list, &self.anchor) {
                primary(fd, request, arg)
            } else if let Some(ioctl_fn) = &list_guard.ioctl_fn {
                ioctl_fn(list_guard.next.clone(), primary, fd, request, arg)
            } else {
                primary(fd, request, arg)
            }
        } else {
            primary(fd, request, arg)
        }
    }

    fn execute_all_ioctl_hooks(
        &self,
        primary: GlIoctlFn,
        fd: i32,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        let anchor_guard = self.anchor.lock().unwrap();
        self.execute_ioctl_hooks(anchor_guard.next.clone(), primary, fd, request, arg)
    }

    fn register_fd_hook(
        &self,
        close_hook: Option<Box<dyn Fn(Option<Arc<Mutex<FdHook>>, GlCloseFn, i32) -> i32 + Send + Sync>>,
        ioctl_hook: Option<Box<dyn Fn(Option<Arc<Mutex<FdHook>>, GlIoctlFn, i32, i32, *mut std::ffi::c_void) -> i32 + Send + Sync>>,
        link: Arc<Mutex<FdHook>>,
    ) {
        let mut link_guard = link.lock().unwrap();
        let close_hook = close_hook.unwrap_or_else(|| {
            Box::new(|remaining_list, primary, fd| {
                self.execute_close_hooks(remaining_list, primary, fd)
            })
        });
        let ioctl_hook = ioctl_hook.unwrap_or_else(|| {
            Box::new(|remaining_list, primary, fd, request, arg| {
                self.execute_ioctl_hooks(remaining_list, primary, fd, request, arg)
            })
        });

        if link_guard.next.is_none() && link_guard.prev.is_none() {
            let anchor_guard = self.anchor.lock().unwrap();
            let next = anchor_guard.next.clone();
            link_guard.next = next.clone();
            link_guard.prev = Some(self.anchor.clone());
            link_guard.close_fn = Some(close_hook);
            link_guard.ioctl_fn = Some(ioctl_hook);

            if let Some(next) = next {
                let mut next_guard = next.lock().unwrap();
                next_guard.prev = Some(link.clone());
            }
            drop(anchor_guard);
            let mut anchor_guard = self.anchor.lock().unwrap();
            anchor_guard.next = Some(link.clone());
        } else {
            if link_guard.close_fn.as_ref().map(|f| f as *const _) != Some(&close_hook as *const _)
                || link_guard.ioctl_fn.as_ref().map(|f| f as *const _) != Some(&ioctl_hook as *const _)
            {
                panic!("Hook functions mismatch");
            }
        }
    }

    fn unregister_fd_hook(&self, link: Arc<Mutex<FdHook>>) {
        let mut link_guard = link.lock().unwrap();
        let next = link_guard.next.take();
        let prev = link_guard.prev.take();

        if next.is_some() && prev.is_some() {
            if let Some(next) = &next {
                let mut next_guard = next.lock().unwrap();
                next_guard.prev = prev.clone();
            }
            if let Some(prev) = &prev {
                let mut prev_guard = prev.lock().unwrap();
                prev_guard.next = next.clone();
            }
            link_guard.close_fn = None;
            link_guard.ioctl_fn = None;
        }
    }
}