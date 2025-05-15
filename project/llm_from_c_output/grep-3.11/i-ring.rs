//! A simple ring buffer implementation in Rust.

use std::panic;

const I_RING_SIZE: usize = 4;

/// A ring buffer structure.
/// When `ir_empty` is true, the ring is empty.
/// Otherwise, `ir_data[B..F]` are defined, where B..F is the contiguous
/// range of indices, modulo I_RING_SIZE, from back to front, inclusive.
/// Undefined elements of `ir_data` are always set to `ir_default_val`.
/// Popping from an empty ring will panic.
/// Pushing onto a full ring returns the displaced value.
/// An empty ring has F==B and `ir_empty == true`.
/// A ring with one entry still has F==B, but now `ir_empty == false`.
#[derive(Debug)]
pub struct IRing {
    ir_data: [i32; I_RING_SIZE],
    ir_default_val: i32,
    ir_front: usize,
    ir_back: usize,
    ir_empty: bool,
}

impl IRing {
    /// Creates a new IRing with the given default value.
    pub fn new(default_val: i32) -> Self {
        Self {
            ir_data: [default_val; I_RING_SIZE],
            ir_default_val: default_val,
            ir_front: 0,
            ir_back: 0,
            ir_empty: true,
        }
    }

    /// Checks if the ring is empty.
    pub fn is_empty(&self) -> bool {
        self.ir_empty
    }

    /// Pushes a value onto the ring.
    /// If the ring is full, returns the displaced value.
    pub fn push(&mut self, val: i32) -> i32 {
        let dest_idx = (self.ir_front + !self.ir_empty as usize) % I_RING_SIZE;
        let old_val = self.ir_data[dest_idx];
        self.ir_data[dest_idx] = val;
        self.ir_front = dest_idx;
        
        if dest_idx == self.ir_back {
            self.ir_back = (self.ir_back + !self.ir_empty as usize) % I_RING_SIZE;
        }
        
        self.ir_empty = false;
        old_val
    }

    /// Pops a value from the ring.
    /// Panics if the ring is empty.
    pub fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Attempt to pop from empty IRing");
        }

        let top_val = self.ir_data[self.ir_front];
        self.ir_data[self.ir_front] = self.ir_default_val;
        
        if self.ir_front == self.ir_back {
            self.ir_empty = true;
        } else {
            self.ir_front = (self.ir_front + I_RING_SIZE - 1) % I_RING_SIZE;
        }
        
        top_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iring_basic() {
        let mut ir = IRing::new(0);
        assert!(ir.is_empty());
        
        assert_eq!(ir.push(1), 0);
        assert!(!ir.is_empty());
        
        assert_eq!(ir.push(2), 0);
        assert_eq!(ir.push(3), 0);
        assert_eq!(ir.push(4), 0);
        
        // Now the ring is full
        assert_eq!(ir.push(5), 1);
        
        assert_eq!(ir.pop(), 5);
        assert_eq!(ir.pop(), 4);
        assert_eq!(ir.pop(), 3);
        assert_eq!(ir.pop(), 2);
        
        assert!(ir.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_pop_empty() {
        let mut ir = IRing::new(0);
        ir.pop();
    }
}