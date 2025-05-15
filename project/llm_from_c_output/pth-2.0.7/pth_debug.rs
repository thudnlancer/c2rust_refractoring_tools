/* 
** GNU Pth - The GNU Portable Threads
** Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
** This file is part of GNU Pth, a non-preemptive thread scheduling
** library which can be found at http://www.gnu.org/software/pth/.
**
** This library is free software; you can redistribute it and/or
** modify it under the terms of the GNU Lesser General Public
** License as published by the Free Software Foundation; either
** version 2.1 of the License, or (at your option) any later version.
**
** This library is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
** Lesser General Public License for more details.
**
** You should have received a copy of the GNU Lesser General Public
** License along with this library; if not, write to the Free Software
** Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
** USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
** pth_debug.rs: Pth debugging support (Rust translation)
*/
                             /* ``MY HACK: This universe.
                                  Just one little problem:
                                  core keeps dumping.'' 
                                              -- Unknown  */

use std::io::{self, Write};
use std::process;
use std::fmt;

#[cfg(not(feature = "debug"))]
macro_rules! pth_debug1 {
    ($a1:expr) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! pth_debug2 {
    ($a1:expr, $a2:expr) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! pth_debug3 {
    ($a1:expr, $a2:expr, $a3:expr) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! pth_debug4 {
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! pth_debug5 {
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! pth_debug6 {
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {};
}

#[cfg(feature = "debug")]
macro_rules! pth_debug1 {
    ($a1:expr) => {
        pth_debug(file!(), line!(), 1, $a1)
    };
}

#[cfg(feature = "debug")]
macro_rules! pth_debug2 {
    ($a1:expr, $a2:expr) => {
        pth_debug(file!(), line!(), 2, $a1, $a2)
    };
}

#[cfg(feature = "debug")]
macro_rules! pth_debug3 {
    ($a1:expr, $a2:expr, $a3:expr) => {
        pth_debug(file!(), line!(), 3, $a1, $a2, $a3)
    };
}

#[cfg(feature = "debug")]
macro_rules! pth_debug4 {
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        pth_debug(file!(), line!(), 4, $a1, $a2, $a3, $a4)
    };
}

#[cfg(feature = "debug")]
macro_rules! pth_debug5 {
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        pth_debug(file!(), line!(), 5, $a1, $a2, $a3, $a4, $a5)
    };
}

#[cfg(feature = "debug")]
macro_rules! pth_debug6 {
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        pth_debug(file!(), line!(), 6, $a1, $a2, $a3, $a4, $a5, $a6)
    };
}

fn pth_debug(file: &str, line: u32, argc: usize, fmt: &str, args: fmt::Arguments<'_>) -> io::Result<()> {
    let mut output = String::new();
    
    if !file.is_empty() {
        output.push_str(&format!("{}:{}:{:04}: ", process::id(), file, line));
    }
    
    if argc == 1 {
        output.push_str(fmt);
    } else {
        output.push_str(&format!("{}", args));
    }
    
    output.push('\n');
    io::stderr().write_all(output.as_bytes())
}

pub fn pth_dumpstate<W: Write>(mut writer: W) -> io::Result<()> {
    writeln!(writer, "+----------------------------------------------------------------------")?;
    writeln!(writer, "| Pth Version: {}", PTH_VERSION_STR)?;
    writeln!(writer, "| Load Average: {:.2}", pth_loadval)?;
    pth_dumpqueue(&mut writer, "NEW", &pth_NQ)?;
    pth_dumpqueue(&mut writer, "READY", &pth_RQ)?;
    writeln!(writer, "| Thread Queue RUNNING:")?;
    writeln!(writer, "|   1. thread {:p} (\"{}\")", pth_current, pth_current.name)?;
    pth_dumpqueue(&mut writer, "WAITING", &pth_WQ)?;
    pth_dumpqueue(&mut writer, "SUSPENDED", &pth_SQ)?;
    pth_dumpqueue(&mut writer, "DEAD", &pth_DQ)?;
    writeln!(writer, "+----------------------------------------------------------------------")?;
    Ok(())
}

pub fn pth_dumpqueue<W: Write>(mut writer: W, qn: &str, q: &pth_pqueue_t) -> io::Result<()> {
    writeln!(writer, "| Thread Queue {}:", qn)?;
    let n = pth_pqueue_elements(q);
    if n == 0 {
        writeln!(writer, "|   no threads")?;
    }
    
    let mut i = 1;
    let mut t = pth_pqueue_head(q);
    while let Some(thread) = t {
        writeln!(writer, "|   {}. thread {:p} (\"{}\")", i, thread, thread.name)?;
        i += 1;
        t = pth_pqueue_walk(q, thread, PTH_WALK_NEXT);
    }
    Ok(())
}