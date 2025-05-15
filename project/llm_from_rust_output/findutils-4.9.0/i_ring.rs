const I_RING_SIZE: usize = 4;

#[derive(Clone)]
pub struct IRing {
    data: [i32; I_RING_SIZE],
    default_val: i32,
    front: usize,
    back: usize,
    is_empty: bool,
}

impl IRing {
    pub fn new(default_val: i32) -> Self {
        Self {
            data: [default_val; I_RING_SIZE],
            default_val,
            front: 0,
            back: 0,
            is_empty: true,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn push(&mut self, val: i32) -> i32 {
        let dest_idx = (self.front + !self.is_empty as usize) % I_RING_SIZE;
        let old_val = self.data[dest_idx];
        self.data[dest_idx] = val;
        self.front = dest_idx;

        if dest_idx == self.back {
            self.back = (self.back + !self.is_empty as usize) % I_RING_SIZE;
        }

        self.is_empty = false;
        old_val
    }

    pub fn pop(&mut self) -> i32 {
        if self.is_empty {
            panic!("Attempt to pop from empty IRing");
        }

        let top_val = self.data[self.front];
        self.data[self.front] = self.default_val;

        if self.front == self.back {
            self.is_empty = true;
        } else {
            self.front = (self.front + I_RING_SIZE - 1) % I_RING_SIZE;
        }

        top_val
    }
}