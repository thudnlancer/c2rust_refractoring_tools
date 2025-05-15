// Hook for making file descriptor functions close(), ioctl() extensible.
// Copyright (C) 2009-2022 Free Software Foundation, Inc.
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

type GlCloseFn = Box<dyn Fn(i32) -> i32 + Send + Sync>;
type GlIoctlFn = Box<dyn Fn(i32, i32, *mut std::ffi::c_void) -> i32 + Send + Sync>;

#[derive(Debug)]
struct FdHook {
    next: Mutex<Option<Arc<FdHook>>>,
    prev: Mutex<Option<Weak<FdHook>>>,
    close_fn: Mutex<Option<CloseHookFn>>,
    ioctl_fn: Mutex<Option<IoctlHookFn>>,
}

type CloseHookFn = Box<dyn Fn(&Arc<FdHook>, &GlCloseFn, i32) -> i32 + Send + Sync>;
type IoctlHookFn = Box<dyn Fn(&Arc<FdHook>, &GlIoctlFn, i32, i32, *mut std::ffi::c_void) -> i32 + Send + Sync>;

#[derive(Debug)]
struct FdHookManager {
    anchor: Arc<FdHook>,
}

impl FdHookManager {
    fn new() -> Self {
        let anchor = Arc::new(FdHook {
            next: Mutex::new(None),
            prev: Mutex::new(None),
            close_fn: Mutex::new(None),
            ioctl_fn: Mutex::new(None),
        });
        *anchor.prev.lock().unwrap() = Some(Arc::downgrade(&anchor));
        *anchor.next.lock().unwrap() = Some(anchor.clone());
        Self { anchor }
    }

    fn execute_close_hooks(&self, remaining_list: &Arc<FdHook>, primary: &GlCloseFn, fd: i32) -> i32 {
        if Arc::ptr_eq(remaining_list, &self.anchor) {
            primary(fd)
        } else {
            if let Some(close_fn) = remaining_list.close_fn.lock().unwrap().as_ref() {
                if let Some(next) = remaining_list.next.lock().unwrap().as_ref() {
                    close_fn(next, primary, fd)
                } else {
                    -1
                }
            } else {
                -1
            }
        }
    }

    fn execute_all_close_hooks(&self, primary: &GlCloseFn, fd: i32) -> i32 {
        if let Some(next) = self.anchor.next.lock().unwrap().as_ref() {
            self.execute_close_hooks(next, primary, fd)
        } else {
            -1
        }
    }

    fn execute_ioctl_hooks(
        &self,
        remaining_list: &Arc<FdHook>,
        primary: &GlIoctlFn,
        fd: i32,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        if Arc::ptr_eq(remaining_list, &self.anchor) {
            primary(fd, request, arg)
        } else {
            if let Some(ioctl_fn) = remaining_list.ioctl_fn.lock().unwrap().as_ref() {
                if let Some(next) = remaining_list.next.lock().unwrap().as_ref() {
                    ioctl_fn(next, primary, fd, request, arg)
                } else {
                    -1
                }
            } else {
                -1
            }
        }
    }

    fn execute_all_ioctl_hooks(
        &self,
        primary: &GlIoctlFn,
        fd: i32,
        request: i32,
        arg: *mut std::ffi::c_void,
    ) -> i32 {
        if let Some(next) = self.anchor.next.lock().unwrap().as_ref() {
            self.execute_ioctl_hooks(next, primary, fd, request, arg)
        } else {
            -1
        }
    }

    fn register_fd_hook(
        &self,
        close_hook: Option<CloseHookFn>,
        ioctl_hook: Option<IoctlHookFn>,
        link: Arc<FdHook>,
    ) {
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

        let mut link_next = link.next.lock().unwrap();
        let mut link_prev = link.prev.lock().unwrap();

        if link_next.is_none() && link_prev.is_none() {
            let mut anchor_next = self.anchor.next.lock().unwrap();
            let anchor_prev = self.anchor.prev.lock().unwrap();

            *link_next = anchor_next.clone();
            *link_prev = Some(Arc::downgrade(&self.anchor));
            *link.close_fn.lock().unwrap() = Some(close_hook);
            *link.ioctl_fn.lock().unwrap() = Some(ioctl_hook);

            if let Some(next) = anchor_next.as_ref() {
                *next.prev.lock().unwrap() = Some(Arc::downgrade(&link));
            }

            *anchor_next = Some(link);
        } else {
            let current_close = link.close_fn.lock().unwrap();
            let current_ioctl = link.ioctl_fn.lock().unwrap();
            if current_close.as_ref().map(|f| f as *const _) != Some(&close_hook as &_ as *const _)
                || current_ioctl.as_ref().map(|f| f as *const _) != Some(&ioctl_hook as &_ as *const _)
            {
                panic!("Hook functions mismatch");
            }
        }
    }

    fn unregister_fd_hook(&self, link: &Arc<FdHook>) {
        let mut next = link.next.lock().unwrap();
        let mut prev = link.prev.lock().unwrap();

        if next.is_some() && prev.is_some() {
            if let Some(next_arc) = next.as_ref() {
                let mut next_prev = next_arc.prev.lock().unwrap();
                *next_prev = prev.clone();
            }

            if let Some(prev_weak) = prev.as_ref() {
                if let Some(prev_arc) = prev_weak.upgrade() {
                    let mut prev_next = prev_arc.next.lock().unwrap();
                    *prev_next = next.clone();
                }
            }

            *next = None;
            *prev = None;
            *link.close_fn.lock().unwrap() = None;
            *link.ioctl_fn.lock().unwrap() = None;
        }
    }
}