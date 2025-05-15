//! A simple ring buffer implementation in Rust.
//! 
//! This file is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version.
//! 
//! This file is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Lesser General Public License for more details.
//! 
//! You should have received a copy of the GNU Lesser General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

const I_RING_SIZE: usize = 4;

/// A ring buffer structure.
/// 
/// When `empty` is true, the ring is empty.
/// Otherwise, `data[back..=front]` are defined, where back..=front is the contiguous
/// range of indices, modulo I_RING_SIZE, from back to front, inclusive.
/// Undefined elements of `data` are always set to `default_val`.
/// Popping from an empty ring will panic.
/// Pushing onto a full ring returns the displaced value.
/// An empty ring has front == back and empty == true.
/// A ring with one entry still has front == back, but now empty == false.
#[derive(Debug)]
pub struct IRing {
    data: [i32; I_RING_SIZE],
    default_val: i32,
    front: usize,
    back: usize,
    empty: bool,
}

impl IRing {
    /// Creates a new IRing with the given default value.
    pub fn new(default_val: i32) -> Self {
        Self {
            data: [default_val; I_RING_SIZE],
            default_val,
            front: 0,
            back: 0,
            empty: true,
        }
    }

    /// Checks if the ring is empty.
    pub fn is_empty(&self) -> bool {
        self.empty
    }

    /// Pushes a value onto the ring.
    /// If the ring is full, returns the displaced value.
    pub fn push(&mut self, val: i32) -> i32 {
        let dest_idx = (self.front + !self.empty as usize) % I_RING_SIZE;
        let old_val = self.data[dest_idx];
        self.data[dest_idx] = val;
        self.front = dest_idx;
        
        if dest_idx == self.back {
            self.back = (self.back + !self.empty as usize) % I_RING_SIZE;
        }
        
        self.empty = false;
        old_val
    }

    /// Pops a value from the ring.
    /// Panics if the ring is empty.
    pub fn pop(&mut self) -> i32 {
        if self.empty {
            panic!("Attempt to pop from empty ring");
        }

        let top_val = self.data[self.front];
        self.data[self.front] = self.default_val;

        if self.front == self.back {
            self.empty = true;
        } else {
            self.front = (self.front + I_RING_SIZE - 1) % I_RING_SIZE;
        }

        top_val
    }
}