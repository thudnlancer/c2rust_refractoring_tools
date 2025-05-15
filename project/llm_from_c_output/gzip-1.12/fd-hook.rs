// Hook for making file descriptor functions close(), ioctl() extensible.
// Copyright (C) 2009-2022 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

use std::sync::{Arc, Mutex};
use std::collections::LinkedList;

#[cfg(windows)]
mod fd_hook {
    use super::*;

    pub type GlCloseFn = fn(fd: i32) -> i32;
    pub type GlIoctlFn = fn(fd: i32, request: i32, arg: *mut ()) -> i32;

    pub struct FdHook {
        close_fn: Option<fn(&LinkedList<Arc<Mutex<FdHook>>>, GlCloseFn, i32) -> i32>,
        ioctl_fn: Option<fn(&LinkedList<Arc<Mutex<FdHook>>>, GlIoctlFn, i32, i32, *mut ()) -> i32>,
    }

    impl FdHook {
        pub fn new(
            close_fn: Option<fn(&LinkedList<Arc<Mutex<FdHook>>>, GlCloseFn, i32) -> i32>,
            ioctl_fn: Option<fn(&LinkedList<Arc<Mutex<FdHook>>>, GlIoctlFn, i32, i32, *mut ()) -> i32>,
        ) -> Self {
            FdHook { close_fn, ioctl_fn }
        }
    }

    pub struct FdHookManager {
        hooks: Arc<Mutex<LinkedList<Arc<Mutex<FdHook>>>>>,
    }

    impl FdHookManager {
        pub fn new() -> Self {
            FdHookManager {
                hooks: Arc::new(Mutex::new(LinkedList::new())),
            }
        }

        pub fn execute_close_hooks(
            hooks: &LinkedList<Arc<Mutex<FdHook>>>,
            primary: GlCloseFn,
            fd: i32,
        ) -> i32 {
            if let Some(hook) = hooks.front() {
                let hook = hook.lock().unwrap();
                if let Some(close_fn) = hook.close_fn {
                    return close_fn(&hooks.iter().skip(1).cloned().collect(), primary, fd);
                }
            }
            primary(fd)
        }

        pub fn execute_all_close_hooks(&self, primary: GlCloseFn, fd: i32) -> i32 {
            let hooks = self.hooks.lock().unwrap();
            Self::execute_close_hooks(&hooks, primary, fd)
        }

        pub fn execute_ioctl_hooks(
            hooks: &LinkedList<Arc<Mutex<FdHook>>>,
            primary: GlIoctlFn,
            fd: i32,
            request: i32,
            arg: *mut (),
        ) -> i32 {
            if let Some(hook) = hooks.front() {
                let hook = hook.lock().unwrap();
                if let Some(ioctl_fn) = hook.ioctl_fn {
                    return ioctl_fn(
                        &hooks.iter().skip(1).cloned().collect(),
                        primary,
                        fd,
                        request,
                        arg,
                    );
                }
            }
            primary(fd, request, arg)
        }

        pub fn execute_all_ioctl_hooks(
            &self,
            primary: GlIoctlFn,
            fd: i32,
            request: i32,
            arg: *mut (),
        ) -> i32 {
            let hooks = self.hooks.lock().unwrap();
            Self::execute_ioctl_hooks(&hooks, primary, fd, request, arg)
        }

        pub fn register_fd_hook(
            &self,
            close_hook: Option<fn(&LinkedList<Arc<Mutex<FdHook>>>, GlCloseFn, i32) -> i32>,
            ioctl_hook: Option<fn(&LinkedList<Arc<Mutex<FdHook>>>, GlIoctlFn, i32, i32, *mut ()) -> i32>,
        ) -> Arc<Mutex<FdHook>> {
            let hook = Arc::new(Mutex::new(FdHook::new(
                close_hook.or(Some(Self::execute_close_hooks)),
                ioctl_hook.or(Some(Self::execute_ioctl_hooks)),
            )));
            self.hooks.lock().unwrap().push_back(hook.clone());
            hook
        }

        pub fn unregister_fd_hook(&self, hook: Arc<Mutex<FdHook>>) {
            let mut hooks = self.hooks.lock().unwrap();
            if let Some(index) = hooks.iter().position(|h| Arc::ptr_eq(h, &hook)) {
                hooks.remove(index);
            }
        }
    }
}

#[cfg(windows)]
pub use fd_hook::*;