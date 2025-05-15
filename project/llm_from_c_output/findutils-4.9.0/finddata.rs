// finddata.rs -- global data for "find".
// Copyright (C) 1990-2022 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/* config.rs must be included first. */
// Note: In Rust, module inclusion is handled differently via mod declarations

/* system headers would go here, but we include none. */

/* gnulib headers. */
// use save_cwd;

/* find headers. */
// use defs;

use std::sync::Arc;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Options {
    // Add fields as needed
}

#[derive(Debug, Default)]
pub struct State {
    // Add fields as needed
}

#[derive(Debug)]
pub struct SavedCwd {
    // Add fields as needed
    path: PathBuf,
}

pub static OPTIONS: Options = Options::default();
pub static STATE: State = State::default();
pub static INITIAL_WD: Option<Arc<SavedCwd>> = None;