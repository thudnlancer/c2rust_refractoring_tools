// err/stream.rs
//
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Gerard Jungman, Brian Gough
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

use std::io::{self, Write};
use std::sync::Mutex;

pub type GslStreamHandler = fn(&str, &str, i32, &str);

lazy_static::lazy_static! {
    static ref GSL_STREAM: Mutex<Box<dyn Write + Send>> = Mutex::new(Box::new(io::stderr()));
    static ref GSL_STREAM_HANDLER: Mutex<Option<GslStreamHandler>> = Mutex::new(None);
}

pub fn gsl_stream_printf(label: &str, file: &str, line: i32, reason: &str) {
    let handler = GSL_STREAM_HANDLER.lock().unwrap();
    if let Some(handler_fn) = *handler {
        handler_fn(label, file, line, reason);
        return;
    }

    let mut stream = GSL_STREAM.lock().unwrap();
    writeln!(stream, "gsl: {}:{}: {}: {}", file, line, label, reason).unwrap();
}

pub fn gsl_set_stream_handler(new_handler: Option<GslStreamHandler>) -> Option<GslStreamHandler> {
    let mut handler = GSL_STREAM_HANDLER.lock().unwrap();
    let previous_handler = *handler;
    *handler = new_handler;
    previous_handler
}

pub fn gsl_set_stream(new_stream: Box<dyn Write + Send>) -> Box<dyn Write + Send> {
    let mut stream = GSL_STREAM.lock().unwrap();
    let previous_stream = std::mem::replace(&mut *stream, new_stream);
    previous_stream
}