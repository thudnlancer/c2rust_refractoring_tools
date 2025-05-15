/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * By Paul Hsieh (C) 2004, 2005.  Covered under the Paul Hsieh
 * derivative license.
 * See: http://www.azillionmonkeys.com/qed/weblicense.html for license
 * details.
 * http://www.azillionmonkeys.com/qed/hash.html
*/

use std::convert::TryInto;

fn get16bits(data: &[u8]) -> u32 {
    u32::from_le_bytes(data[..2].try_into().unwrap())
}

pub fn hash_hsieh(key: &[u8]) -> u32 {
    if key.is_empty() {
        return 0;
    }

    let mut hash: u32 = 0;
    let mut rem = key.len() % 4;
    let mut key_length = key.len() / 4;
    let mut pos = 0;

    /* Main loop */
    while key_length > 0 {
        if pos + 2 > key.len() { break; }
        hash += get16bits(&key[pos..]);
        
        if pos + 4 > key.len() { break; }
        let tmp = (get16bits(&key[pos+2..]) << 11) ^ hash;
        hash = (hash << 16) ^ tmp;
        pos += 4;
        hash += hash >> 11;
        key_length -= 1;
    }

    /* Handle end cases */
    match rem {
        3 => {
            if pos + 2 > key.len() { return hash; }
            hash += get16bits(&key[pos..]);
            hash ^= hash << 16;
            if pos + 2 >= key.len() { return hash; }
            hash ^= (key[pos+2] as u32) << 18;
            hash += hash >> 11;
        },
        2 => {
            if pos + 2 > key.len() { return hash; }
            hash += get16bits(&key[pos..]);
            hash ^= hash << 11;
            hash += hash >> 17;
        },
        1 => {
            if pos >= key.len() { return hash; }
            hash += key[pos] as u32;
            hash ^= hash << 10;
            hash += hash >> 1;
        },
        _ => (),
    }

    /* Force "avalanching" of final 127 bits */
    hash ^= hash << 3;
    hash += hash >> 5;
    hash ^= hash << 4;
    hash += hash >> 17;
    hash ^= hash << 25;
    hash += hash >> 6;

    hash
}