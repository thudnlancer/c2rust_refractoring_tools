/* interpolation/accel.rs

 * Copyright (C) 1996, 1997, 1998, 1999, 2000 Gerard Jungman
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or (at
 * your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
 */

/* Author:  G. Jungman
 */

#[derive(Debug, Default)]
pub struct InterpAccel {
    cache: usize,
    hit_count: usize,
    miss_count: usize,
}

impl InterpAccel {
    pub fn new() -> Result<Self, &'static str> {
        Ok(InterpAccel {
            cache: 0,
            hit_count: 0,
            miss_count: 0,
        })
    }

    pub fn reset(&mut self) -> Result<(), &'static str> {
        self.cache = 0;
        self.hit_count = 0;
        self.miss_count = 0;
        Ok(())
    }
}

impl Drop for InterpAccel {
    fn drop(&mut self) {
        // Rust's ownership system automatically handles memory deallocation
    }
}