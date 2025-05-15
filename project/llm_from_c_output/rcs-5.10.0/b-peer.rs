// b-peer.rs --- finding the ‘execv’able name of a peer program

// Copyright (C) 2010-2020 Thien-Thi Nguyen
//
// This file is part of GNU RCS.
//
// GNU RCS is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// GNU RCS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::path::{Path, PathBuf};
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::env;

#[derive(Debug)]
pub struct Symdef {
    meaningful: String,
    underlying: Option<CString>,
}

pub static mut PEER_SUPER: Symdef = Symdef {
    meaningful: String::new(),
    underlying: None,
};

pub fn one_beyond_last_dir_sep(name: &str) -> Option<&str> {
    name.rfind('/').map(|pos| &name[pos + 1..])
}

pub fn find_peer_prog(prog: &mut Symdef) -> Result<&CString, Box<dyn std::error::Error>> {
    if prog.underlying.is_none() {
        #[cfg(not(feature = "exeext"))]
        {
            if unsafe { BE.invdir.is_empty() } {
                let name = find_in_path(&PROGRAM.invoke)?;
                let end = one_beyond_last_dir_sep(&name)
                    .ok_or_else(|| format!("cannot determine directory (in PATH) of `{}`", name))?;
                
                unsafe {
                    BE.invdir = intern(PLEXUS, &name[..name.len() - end.len()]);
                }
                
                if name != PROGRAM.invoke {
                    // Memory is managed by Rust's ownership system
                }
            }

            let path = format!("{}{}", unsafe { &BE.invdir }, prog.meaningful);
            prog.underlying = Some(CString::new(path)?);
        }

        #[cfg(feature = "exeext")]
        {
            let path = format!("{}", prog.meaningful);
            prog.underlying = Some(CString::new(path)?);
        }
    }

    prog.underlying.as_ref()
        .ok_or_else(|| "Failed to find peer program".into())
}

// Idioms
pub fn peer_super() -> Result<&'static CString, Box<dyn std::error::Error>> {
    unsafe { find_peer_prog(&mut PEER_SUPER) }
}

// Mockups for missing C constructs
struct Be {
    invdir: String,
}

static mut BE: Be = Be {
    invdir: String::new(),
};

struct Program {
    invoke: String,
}

static PROGRAM: Program = Program {
    invoke: String::new(),
};

const PLEXUS: &str = "";

fn intern(_plexus: &str, s: &str) -> String {
    s.to_string()
}

fn find_in_path(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    env::var_os("PATH")
        .and_then(|paths| {
            env::split_paths(&paths)
                .map(|p| p.join(name))
                .find(|p| p.exists())
                .map(|p| p.to_string_lossy().into_owned())
        })
        .ok_or_else(|| format!("Could not find `{}` in PATH", name).into())
}

// b-peer.rs ends here