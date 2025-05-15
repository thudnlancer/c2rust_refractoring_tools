/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  pth_ext.rs: Pth extensions (Rust translation)
*/
                             /* ``Killing for peace is
                                  like fucking for virginity.''
                                             -- Unknown  */

use std::io::{self, Read, Write, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::boxed::Box;

/*
 * Sfio Extension:
 *
 * We provide an Sfio discipline which can be pushed on an Sfio_t* stream
 * to use the Pth thread-aware I/O routines (pth_read/pth_write).
 */

#[cfg(feature = "pth_ext_sfio")]
mod sfio_extension {
    use super::*;

    pub struct SfioDisc {
        // In Rust, we would typically implement traits rather than function pointers
    }

    impl Read for SfioDisc {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            // Assuming pth_read is implemented elsewhere as a safe Rust function
            pth_read(self.as_raw_fd(), buf)
        }
    }

    impl Write for SfioDisc {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            pth_write(self.as_raw_fd(), buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Seek for SfioDisc {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
            // Assuming sfsk is implemented elsewhere as a safe Rust function
            sfsk(self, pos)
        }
    }

    impl SfioDisc {
        pub fn except(&self, event_type: i32, data: &mut [u8]) -> io::Result<i32> {
            match event_type {
                SF_LOCKED | SF_READ | SF_WRITE | SF_SEEK | SF_NEW | SF_CLOSE |
                SF_FINAL | SF_DPUSH | SF_DPOP | SF_DBUFFER | SF_DPOLL |
                SF_READY | SF_SYNC | SF_PURGE => Ok(0), // perform default action
                _ => Ok(0),
            }
        }
    }

    pub fn pth_sfiodisc() -> io::Result<Box<SfioDisc>> {
        let disc = Box::new(SfioDisc {});
        Ok(disc)
    }

    // Constants that would normally come from Sfio headers
    const SF_LOCKED: i32 = 0;
    const SF_READ: i32 = 1;
    const SF_WRITE: i32 = 2;
    const SF_SEEK: i32 = 3;
    const SF_NEW: i32 = 4;
    const SF_CLOSE: i32 = 5;
    const SF_FINAL: i32 = 6;
    const SF_DPUSH: i32 = 7;
    const SF_DPOP: i32 = 8;
    const SF_DBUFFER: i32 = 9;
    const SF_DPOLL: i32 = 10;
    const SF_READY: i32 = 11;
    const SF_SYNC: i32 = 12;
    const SF_PURGE: i32 = 13;
}

#[cfg(not(feature = "pth_ext_sfio"))]
pub fn pth_sfiodisc() -> io::Result<Box<dyn std::any::Any>> {
    Err(io::Error::new(io::ErrorKind::Unsupported, "ENOSYS"))
}

#[cfg(feature = "pth_ext_sfio")]
pub use sfio_extension::pth_sfiodisc;

// These would be implemented elsewhere in the Rust version
fn pth_read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    unimplemented!()
}

fn pth_write(fd: i32, buf: &[u8]) -> io::Result<usize> {
    unimplemented!()
}

fn sfsk<T: Seek>(stream: &mut T, pos: SeekFrom) -> io::Result<u64> {
    stream.seek(pos)
}