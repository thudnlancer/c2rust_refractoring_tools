/*
 *  Copyright 2021 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 *
 * filter to support byte-swapped filesystems
 */

use std::io::{self, Read, Write, Seek, SeekFrom};
use std::sync::Arc;

trait Stream: Read + Write + Seek + Send + Sync {}
impl<T: Read + Write + Seek + Send + Sync> Stream for T {}

struct Swap {
    next: Arc<dyn Stream>,
}

fn swap_buffer(buf: &mut [u8]) {
    for chunk in buf.chunks_exact_mut(2) {
        chunk.swap(0, 1);
    }
}

impl Read for Swap {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut temp_buf = vec![0; buf.len()];
        let result = self.next.read(&mut temp_buf)?;
        
        if result > 0 {
            let data = &mut temp_buf[..result];
            swap_buffer(data);
            buf[..result].copy_from_slice(data);
        }
        
        Ok(result)
    }
}

impl Write for Swap {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut swapping = buf.to_vec();
        swap_buffer(&mut swapping);
        self.next.write(&swapping)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.next.flush()
    }
}

impl Seek for Swap {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.next.seek(pos)
    }
}

fn open_swap(next: Arc<dyn Stream>) -> Arc<dyn Stream> {
    Arc::new(Swap { next })
}