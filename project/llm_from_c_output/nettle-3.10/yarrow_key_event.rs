// yarrow_key_event.rs

// Example entropy estimator for key-like input events.

// Copyright (C) 2001 Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.

// or

//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

pub const YARROW_KEY_EVENT_BUFFER: usize = 16;

pub struct YarrowKeyEventCtx {
    chars: [u32; YARROW_KEY_EVENT_BUFFER],
    index: usize,
    previous: u32,
}

impl YarrowKeyEventCtx {
    pub fn new() -> Self {
        Self {
            chars: [0; YARROW_KEY_EVENT_BUFFER],
            index: 0,
            previous: 0,
        }
    }

    pub fn estimate(&mut self, key: u32, time: u32) -> u32 {
        let mut entropy = 0;

        // Look at timing first.
        if self.previous != 0 && time > self.previous {
            if time - self.previous >= 256 {
                entropy += 1;
            }
        }
        self.previous = time;

        if key == 0 {
            return entropy;
        }

        // Check if key is in recent characters
        if self.chars.iter().any(|&c| c == key) {
            return entropy;
        }

        // Count one bit of entropy, unless this was one of the initial 16 characters.
        if self.chars[self.index] != 0 {
            entropy += 1;
        }

        // Remember the character.
        self.chars[self.index] = key;
        self.index = (self.index + 1) % YARROW_KEY_EVENT_BUFFER;

        entropy
    }
}