/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Page fault handling library.
   Copyright (C) 1998-2023 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use libc::{c_int, c_void, size_t, uintptr_t};
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

#[repr(C)]
pub struct VmaStruct {
    pub start: uintptr_t,
    pub end: uintptr_t,
    pub prev_end: uintptr_t,
    pub is_near_this: extern "C" fn(uintptr_t, *const VmaStruct) -> bool,
}

extern "C" {
    fn sigsegv_get_vma(address: uintptr_t, vma: *mut VmaStruct) -> c_int;
}

pub const LIBSIGSEGV_VERSION: c_int = 0x020D;
pub static libsigsegv_version: c_int = LIBSIGSEGV_VERSION;

pub const SIGSEGV_FAULT_ADDRESS_ALIGNMENT: uintptr_t = 1;

pub type SigsegvHandlerT = extern "C" fn(fault_address: *mut c_void, serious: c_int) -> c_int;
pub type StackoverflowHandlerT = extern "C" fn(emergency: c_int, scp: *mut c_void);

static USER_HANDLER: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static STK_USER_HANDLER: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static STK_EXTRA_STACK: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static STK_EXTRA_STACK_SIZE: usize = 0;
static STACK_TOP: uintptr_t = 0;

#[no_mangle]
pub extern "C" fn sigsegv_install_handler(handler: SigsegvHandlerT) -> c_int {
    USER_HANDLER.store(handler as *mut c_void, Ordering::SeqCst);
    // TODO: Install signal handlers
    0
}

#[no_mangle]
pub extern "C" fn sigsegv_deinstall_handler() {
    USER_HANDLER.store(ptr::null_mut(), Ordering::SeqCst);
    // TODO: Restore default signal handlers
}

#[no_mangle]
pub extern "C" fn sigsegv_leave_handler(
    continuation: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    cont_arg1: *mut c_void,
    cont_arg2: *mut c_void,
    cont_arg3: *mut c_void,
) -> c_int {
    // TODO: Reset on-stack flag
    continuation(cont_arg1, cont_arg2, cont_arg3);
    1
}

#[no_mangle]
pub extern "C" fn stackoverflow_install_handler(
    handler: StackoverflowHandlerT,
    extra_stack: *mut c_void,
    extra_stack_size: size_t,
) -> c_int {
    unsafe {
        if STACK_TOP == 0 {
            let mut dummy = 0;
            remember_stack_top(&mut dummy as *mut _ as *mut c_void);
            if STACK_TOP == 0 {
                return -1;
            }
        }
    }

    STK_USER_HANDLER.store(handler as *mut c_void, Ordering::SeqCst);
    STK_EXTRA_STACK.store(extra_stack, Ordering::SeqCst);
    STK_EXTRA_STACK_SIZE = extra_stack_size;

    // TODO: Set up alternate signal stack
    // TODO: Install signal handlers with SA_ONSTACK
    0
}

#[no_mangle]
pub extern "C" fn stackoverflow_deinstall_handler() {
    STK_USER_HANDLER.store(ptr::null_mut(), Ordering::SeqCst);
    // TODO: Restore signal handlers
    // TODO: Disable alternate signal stack
}

unsafe extern "C" fn remember_stack_top(some_variable_on_stack: *mut c_void) {
    let mut vma = mem::zeroed::<VmaStruct>();
    if sigsegv_get_vma(some_variable_on_stack as uintptr_t, &mut vma) >= 0 {
        STACK_TOP = vma.end - 1;
    }
}

// TODO: Implement platform-specific signal handlers
// TODO: Implement sigsegv_handler function
// TODO: Implement install_for function
// TODO: Implement sigsegv_reset_onstack_flag function