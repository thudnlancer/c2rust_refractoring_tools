const I_RING_SIZE: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct IRing {
    ir_data: [i32; I_RING_SIZE],
    ir_default_val: i32,
    ir_front: usize,
    ir_back: usize,
    ir_empty: bool,
}

impl IRing {
    pub fn new(default_val: i32) -> Self {
        Self {
            ir_data: [default_val; I_RING_SIZE],
            ir_default_val,
            ir_front: 0,
            ir_back: 0,
            ir_empty: true,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.ir_empty
    }

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