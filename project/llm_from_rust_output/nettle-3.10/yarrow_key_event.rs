#[derive(Clone, Copy, Default)]
pub struct YarrowKeyEventCtx {
    pub index: u32,
    pub chars: [u32; 16],
    pub previous: u32,
}

impl YarrowKeyEventCtx {
    pub fn new() -> Self {
        Self {
            index: 0,
            chars: [0; 16],
            previous: 0,
        }
    }

    pub fn estimate(&mut self, key: u32, time: u32) -> u32 {
        let mut entropy = 0u32;

        if self.previous != 0 && time > self.previous {
            if time.wrapping_sub(self.previous) >= 256 {
                entropy = entropy.wrapping_add(1);
            }
        }
        self.previous = time;

        if key == 0 {
            return entropy;
        }

        if self.chars.iter().any(|&c| c == key) {
            return entropy;
        }

        if self.chars[self.index as usize] != 0 {
            entropy = entropy.wrapping_add(1);
        }

        self.chars[self.index as usize] = key;
        self.index = self.index.wrapping_add(1).wrapping_rem(16);

        entropy
    }
}