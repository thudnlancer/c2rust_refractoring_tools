/*
 * Copyright 1996,1997,1999,2001,2002,2009,2021 Alain Knaff.
 * This file is part of mtools.
 *
 * Mtools is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Mtools is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Force I/O to be done to complete transfer length
 *
 * written by:
 *
 * Alain L. Knaff
 * alain@knaff.lu
 */

use std::io::{self, Read, Write, Seek};
use std::cmp::min;

trait StreamClass {
    fn write(&self, stream: &Stream, buf: &[u8]) -> io::Result<usize>;
    fn pwrite(&self, stream: &Stream, buf: &[u8], offset: u64) -> io::Result<usize>;
    fn pread(&self, stream: &Stream, buf: &mut [u8], offset: u64) -> io::Result<usize>;
}

struct Stream {
    class: Box<dyn StreamClass>,
}

fn force_pio<F>(
    stream: &Stream,
    mut buf: &mut [u8],
    mut start: u64,
    mut len: usize,
    io_func: F,
) -> io::Result<usize>
where
    F: Fn(&Stream, &mut [u8], u64) -> io::Result<usize>,
{
    let mut done = 0;

    while len > 0 {
        let ret = io_func(stream, buf, start)?;
        if ret == 0 {
            if done > 0 {
                return Ok(done);
            } else {
                return Ok(0);
            }
        }

        debug_assert!(ret <= len);
        start += ret as u64;
        done += ret;
        len -= ret;
        buf = &mut buf[ret..];
    }

    Ok(done)
}

fn write_wrapper(stream: &Stream, buf: &mut [u8], _offset: u64) -> io::Result<usize> {
    stream.class.write(stream, buf)
}

fn force_write(stream: &Stream, buf: &mut [u8], len: usize) -> io::Result<usize> {
    force_pio(stream, buf, 0, len, write_wrapper)
}

fn force_pwrite(stream: &Stream, buf: &mut [u8], start: u64, len: usize) -> io::Result<usize> {
    force_pio(stream, buf, start, len, |s, b, off| s.class.pwrite(s, b, off))
}

fn force_pread(stream: &Stream, buf: &mut [u8], start: u64, len: usize) -> io::Result<usize> {
    force_pio(stream, buf, start, len, |s, b, off| s.class.pread(s, b, off))
}