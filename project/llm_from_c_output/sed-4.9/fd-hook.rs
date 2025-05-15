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

#![cfg(windows)]

use std::sync::{Arc, Mutex};
use std::os::windows::io::{AsRawFd, RawFd};

type GlCloseFn = fn(RawFd) -> std::io::Result<()>;
type GlIoctlFn = fn(RawFd, i32, &mut [u8]) -> std::io::Result<()>;

struct FdHook {
    next: Option<Arc<Mutex<FdHook>>>,
    prev: Option<Arc<Mutex<FdHook>>>,
    close_fn: Option<CloseHookFn>,
    ioctl_fn: Option<IoctlHookFn>,
}

type CloseHookFn = fn(
    remaining_list: Option<Arc<Mutex<FdHook>>>,
    primary: GlCloseFn,
    fd: RawFd,
) -> std::io::Result<()>;

type IoctlHookFn = fn(
    remaining_list: Option<Arc<Mutex<FdHook>>>,
    primary: GlIoctlFn,
    fd: RawFd,
    request: i32,
    arg: &mut [u8],
) -> std::io::Result<()>;

lazy_static::lazy_static! {
    static ref ANCHOR: Arc<Mutex<FdHook>> = Arc::new(Mutex::new(FdHook {
        next: None,
        prev: None,
        close_fn: None,
        ioctl_fn: None,
    }));
}

fn execute_close_hooks(
    remaining_list: Option<Arc<Mutex<FdHook>>>,
    primary: GlCloseFn,
    fd: RawFd,
) -> std::io::Result<()> {
    if let Some(list) = remaining_list {
        let list = list.lock().unwrap();
        if Arc::ptr_eq(&list, &ANCHOR) {
            primary(fd)
        } else {
            if let Some(close_fn) = list.close_fn {
                close_fn(list.next.clone(), primary, fd)
            } else {
                primary(fd)
            }
        }
    } else {
        primary(fd)
    }
}

fn execute_all_close_hooks(primary: GlCloseFn, fd: RawFd) -> std::io::Result<()> {
    let anchor = ANCHOR.lock().unwrap();
    execute_close_hooks(anchor.next.clone(), primary, fd)
}

fn execute_ioctl_hooks(
    remaining_list: Option<Arc<Mutex<FdHook>>>,
    primary: GlIoctlFn,
    fd: RawFd,
    request: i32,
    arg: &mut [u8],
) -> std::io::Result<()> {
    if let Some(list) = remaining_list {
        let list = list.lock().unwrap();
        if Arc::ptr_eq(&list, &ANCHOR) {
            primary(fd, request, arg)
        } else {
            if let Some(ioctl_fn) = list.ioctl_fn {
                ioctl_fn(list.next.clone(), primary, fd, request, arg)
            } else {
                primary(fd, request, arg)
            }
        }
    } else {
        primary(fd, request, arg)
    }
}

fn execute_all_ioctl_hooks(
    primary: GlIoctlFn,
    fd: RawFd,
    request: i32,
    arg: &mut [u8],
) -> std::io::Result<()> {
    let anchor = ANCHOR.lock().unwrap();
    execute_ioctl_hooks(anchor.next.clone(), primary, fd, request, arg)
}

fn register_fd_hook(
    close_hook: Option<CloseHookFn>,
    ioctl_hook: Option<IoctlHookFn>,
    link: Arc<Mutex<FdHook>>,
) {
    let mut link = link.lock().unwrap();
    let close_fn = close_hook.unwrap_or(execute_close_hooks);
    let ioctl_fn = ioctl_hook.unwrap_or(execute_ioctl_hooks);

    if link.next.is_none() && link.prev.is_none() {
        let mut anchor = ANCHOR.lock().unwrap();
        link.next = anchor.next.clone();
        link.prev = Some(ANCHOR.clone());
        link.close_fn = Some(close_fn);
        link.ioctl_fn = Some(ioctl_fn);
        
        if let Some(ref mut next) = anchor.next {
            next.lock().unwrap().prev = Some(link.clone());
        }
        anchor.next = Some(link.clone());
    } else {
        if link.close_fn != Some(close_fn) || link.ioctl_fn != Some(ioctl_fn) {
            panic!("Hook already registered with different functions");
        }
    }
}

fn unregister_fd_hook(link: Arc<Mutex<FdHook>>) {
    let mut link = link.lock().unwrap();
    if let (Some(next), Some(prev)) = (link.next.clone(), link.prev.clone()) {
        let mut next = next.lock().unwrap();
        let mut prev = prev.lock().unwrap();
        
        prev.next = link.next.clone();
        next.prev = link.prev.clone();
        
        link.next = None;
        link.prev = None;
        link.close_fn = None;
        link.ioctl_fn = None;
    }
}