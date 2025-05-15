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
 * HashKit
 * Copyright (C) 2009 Brian Aker
 * All rights reserved.
 *
 * Use and distribution licensed under the BSD license.  See
 * the COPYING file in the parent directory for full text.
 */

/*
 * This has is Jenkin's "One at A time Hash".
 * http://en.wikipedia.org/wiki/Jenkins_hash_function
 */

pub fn hash_one_at_a_time(key: &[u8]) -> u32 {
    let mut value = 0u32;

    for &val in key {
        value += u32::from(val);
        value += value << 10;
        value ^= value >> 6;
    }
    value += value << 3;
    value ^= value >> 11;
    value += value << 15;

    value
}