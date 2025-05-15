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

const FNV_64_INIT: u64 = 0xcbf29ce484222325;
const FNV_64_PRIME: u64 = 0x100000001b3;
const FNV_32_INIT: u32 = 2166136261;
const FNV_32_PRIME: u32 = 16777619;

pub fn hash_fnv1_64(key: &[u8]) -> u32 {
    let mut hash = FNV_64_INIT;
    
    for &byte in key {
        hash = hash.wrapping_mul(FNV_64_PRIME);
        hash ^= u64::from(byte);
    }
    
    hash as u32
}

pub fn hash_fnv1a_64(key: &[u8]) -> u32 {
    let mut hash = FNV_64_INIT as u32;
    
    for &byte in key {
        let val = u32::from(byte);
        hash ^= val;
        hash = hash.wrapping_mul(FNV_64_PRIME as u32);
    }
    
    hash
}

pub fn hash_fnv1_32(key: &[u8]) -> u32 {
    let mut hash = FNV_32_INIT;
    
    for &byte in key {
        let val = u32::from(byte);
        hash = hash.wrapping_mul(FNV_32_PRIME);
        hash ^= val;
    }
    
    hash
}

pub fn hash_fnv1a_32(key: &[u8]) -> u32 {
    let mut hash = FNV_32_INIT;
    
    for &byte in key {
        let val = u32::from(byte);
        hash ^= val;
        hash = hash.wrapping_mul(FNV_32_PRIME);
    }
    
    hash
}