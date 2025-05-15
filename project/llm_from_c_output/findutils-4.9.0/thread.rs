/*
 * Creating and controlling threads.
 * Copyright (C) 2005-2022 Free Software Foundation, Inc.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::{Once, Mutex, Arc};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::ptr::null_mut;

pub type GlThread = thread::Thread;
pub type GlThreadResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub struct ThreadHandle {
    pub thread: thread::JoinHandle<()>,
    pub exit_value: Arc<Mutex<Option<Box<dyn std::any::Any + Send>>>>,
}

impl ThreadHandle {
    pub fn new(thread: thread::JoinHandle<()>) -> Self {
        ThreadHandle {
            thread,
            exit_value: Arc::new(Mutex::new(None)),
        }
    }
}

pub fn gl_thread_create(
    func: Box<dyn FnOnce() -> Box<dyn std::any::Any + Send> + Send>,
) -> Result<ThreadHandle, std::io::Error> {
    let exit_value = Arc::new(Mutex::new(None));
    let exit_value_clone = exit_value.clone();

    let thread = thread::Builder::new().spawn(move || {
        let result = func();
        *exit_value_clone.lock().unwrap() = Some(result);
    })?;

    Ok(ThreadHandle::new(thread))
}

pub fn gl_thread_self() -> GlThread {
    thread::current()
}

pub fn gl_thread_join(handle: ThreadHandle) -> Result<Box<dyn std::any::Any + Send>, Box<dyn std::error::Error>> {
    handle.thread.join().map_err(|_| "Thread join failed".into())?;
    let exit_value = Arc::try_unwrap(handle.exit_value)
        .map_err(|_| "Failed to unwrap exit value".into())?
        .into_inner()
        .unwrap();
    exit_value.ok_or_else(|| "No exit value".into())
}

pub fn gl_thread_exit(value: Box<dyn std::any::Any + Send>) -> ! {
    let exit_value = Arc::new(Mutex::new(Some(value)));
    panic!("Thread exit with value");
}

pub fn gl_thread_sigmask(
    how: i32,
    set: Option<&libc::sigset_t>,
    oldset: Option<&mut libc::sigset_t>,
) -> Result<(), std::io::Error> {
    unsafe {
        if libc::pthread_sigmask(how, set.map_or(null_mut(), |s| s), oldset.map_or(null_mut(), |s| s)) == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

pub fn gl_thread_atfork(
    prepare: Option<Box<dyn Fn() + Send + Sync>>,
    parent: Option<Box<dyn Fn() + Send + Sync>>,
    child: Option<Box<dyn Fn() + Send + Sync>>,
) -> Result<(), std::io::Error> {
    unsafe extern "C" fn prepare_wrapper(data: *mut libc::c_void) {
        let callback = &*(data as *const Box<dyn Fn() + Send + Sync>);
        callback();
    }

    unsafe extern "C" fn parent_wrapper(data: *mut libc::c_void) {
        let callback = &*(data as *const Box<dyn Fn() + Send + Sync>);
        callback();
    }

    unsafe extern "C" fn child_wrapper(data: *mut libc::c_void) {
        let callback = &*(data as *const Box<dyn Fn() + Send + Sync>);
        callback();
    }

    let prepare_data = prepare.map(|f| Box::into_raw(Box::new(f)));
    let parent_data = parent.map(|f| Box::into_raw(Box::new(f)));
    let child_data = child.map(|f| Box::into_raw(Box::new(f)));

    unsafe {
        if libc::pthread_atfork(
            prepare_data.map_or(None, |p| Some(prepare_wrapper as unsafe extern "C" fn(*mut libc::c_void))),
            parent_data.map_or(None, |p| Some(parent_wrapper as unsafe extern "C" fn(*mut libc::c_void))),
            child_data.map_or(None, |p| Some(child_wrapper as unsafe extern "C" fn(*mut libc::c_void))),
        ) == 0
        {
            if let Some(p) = prepare_data {
                drop(Box::from_raw(p));
            }
            if let Some(p) = parent_data {
                drop(Box::from_raw(p));
            }
            if let Some(p) = child_data {
                drop(Box::from_raw(p));
            }
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

pub fn gl_thread_self_pointer() -> *const () {
    thread::current().id().as_u64().get() as *const ()
}