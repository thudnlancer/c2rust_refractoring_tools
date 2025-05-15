/*
 * Printer interface for popen() / lpr systems.
 * Copyright (c) 1995 Markku Rossi.
 *
 * Author: Markku Rossi <mtr@iki.fi>
 */

/*
 * This file is part of GNU enscript.
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; see the file COPYING.  If not, write to
 * the Free Software Foundation, 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

use std::process::{Command, Stdio};
use std::io::{self, Write};

/*
 * Global functions.
 */

pub fn printer_open(
    cmd: &str,
    options: Option<&str>,
    queue_param: &str,
    printer_name: Option<&str>,
) -> io::Result<Box<dyn Write>> {
    let mut command = Command::new(cmd);
    
    if let Some(opts) = options {
        command.arg(opts);
    }
    
    if let Some(name) = printer_name {
        command.arg(queue_param).arg(name);
    }
    
    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    
    Ok(Box::new(child.stdin.unwrap()))
}

pub fn printer_close(writer: Box<dyn Write>) -> io::Result<()> {
    // The writer will be automatically closed when dropped
    Ok(())
}