use std::mem;

pub fn memchr2(
    s: &[u8],
    c1_in: u8,
    c2_in: u8,
) -> Option<usize> {
    let c1 = c1_in;
    let c2 = c2_in;

    if c1 == c2 {
        return s.iter().position(|&x| x == c1);
    }

    // Process unaligned bytes at the start
    let mut offset = 0;
    while offset < s.len() && (s.as_ptr() as usize + offset) % mem::size_of::<usize>() != 0 {
        let current = s[offset];
        if current == c1 || current == c2 {
            return Some(offset);
        }
        offset += 1;
    }

    if offset >= s.len() {
        return None;
    }

    // Process aligned chunks
    let word_size = mem::size_of::<usize>();
    let repeated_one = usize::MAX / 255;
    let repeated_c1 = c1 as usize * repeated_one;
    let repeated_c2 = c2 as usize * repeated_one;

    while offset + word_size <= s.len() {
        let word_ptr = unsafe { (s.as_ptr().add(offset) as *const usize).read_unaligned() };
        let xor1 = word_ptr ^ repeated_c1;
        let xor2 = word_ptr ^ repeated_c2;
        let temp = (xor1.wrapping_sub(repeated_one) & !xor1) | 
                   (xor2.wrapping_sub(repeated_one) & !xor2);
        
        if (temp & (repeated_one << 7)) != 0 {
            break;
        }
        offset += word_size;
    }

    // Process remaining bytes
    while offset < s.len() {
        let current = s[offset];
        if current == c1 || current == c2 {
            return Some(offset);
        }
        offset += 1;
    }

    None
}