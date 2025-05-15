// Exit from GNU tar.

// Copyright 2009-2021 Free Software Foundation, Inc.

// This file is part of GNU tar.

// GNU tar is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// GNU tar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::process;
use std::sync::OnceLock;

static FATAL_EXIT_HOOK: OnceLock<Box<dyn Fn()>> = OnceLock::new();

pub fn set_fatal_exit_hook(hook: impl Fn() + 'static) {
    let _ = FATAL_EXIT_HOOK.set(Box::new(hook));
}

pub fn fatal_exit() -> ! {
    if let Some(hook) = FATAL_EXIT_HOOK.get() {
        hook();
    }
    eprintln!("Error is not recoverable: exiting now");
    process::exit(1);
}

pub fn xalloc_die() -> ! {
    eprintln!("memory exhausted");
    fatal_exit();
}