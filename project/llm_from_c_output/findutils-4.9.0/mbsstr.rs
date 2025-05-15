use std::cmp::Ordering;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Clone, Copy)]
struct MbChar {
    bytes: [u8; 4],
    len: usize,
}

impl MbChar {
    fn new() -> Self {
        MbChar {
            bytes: [0; 4],
            len: 0,
        }
    }

    fn from_char(c: char) -> Self {
        let mut mb = MbChar::new();
        let bytes = c.encode_utf8(&mut mb.bytes).as_bytes();
        mb.len = bytes.len();
        mb
    }

    fn equal(&self, other: &MbChar) -> bool {
        self.len == other.len && self.bytes[..self.len] == other.bytes[..other.len]
    }
}

struct MbStrIter<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> MbStrIter<'a> {
    fn new(s: &'a str) -> Self {
        MbStrIter {
            s: s.as_bytes(),
            pos: 0,
        }
    }

    fn avail(&self) -> bool {
        self.pos < self.s.len()
    }

    fn advance(&mut self) {
        if self.avail() {
            let c = self.s[self.pos] as char;
            self.pos += c.len_utf8();
        }
    }

    fn current(&self) -> Option<MbChar> {
        if !self.avail() {
            return None;
        }
        let c = char::from_u32(self.s[self.pos] as u32)?;
        Some(MbChar::from_char(c))
    }

    fn current_ptr(&self) -> *const c_char {
        unsafe { self.s.as_ptr().add(self.pos) as *const c_char }
    }
}

fn mbslen(s: &str) -> usize {
    s.chars().count()
}

fn knuth_morris_pratt_multibyte(
    haystack: &str,
    needle: &str,
) -> Result<Option<*const c_char>, &'static str> {
    let m = mbslen(needle);
    if m == 0 {
        return Ok(Some(haystack.as_ptr() as *const c_char));
    }

    let mut needle_mbchars = Vec::with_capacity(m);
    let mut table = Vec::with_capacity(m);

    // Fill needle_mbchars
    for c in needle.chars() {
        needle_mbchars.push(MbChar::from_char(c));
    }

    // Fill the table
    table.push(0); // table[0] unused
    if m > 1 {
        table.push(1); // table[1] = 1
    }

    let mut j = 0;
    for i in 2..m {
        loop {
            if needle_mbchars[i - 1].equal(&needle_mbchars[j]) {
                j += 1;
                table.push(i - j);
                break;
            }
            if j == 0 {
                table.push(i);
                break;
            }
            j = j - table[j];
        }
    }

    // Search using the table
    let mut j = 0;
    let mut rhaystack = MbStrIter::new(haystack);
    let mut phaystack = MbStrIter::new(haystack);

    while phaystack.avail() {
        if let (Some(needle_char), Some(haystack_char)) =
            (needle_mbchars.get(j), phaystack.current())
        {
            if needle_char.equal(&haystack_char) {
                j += 1;
                phaystack.advance();
                if j == m {
                    return Ok(Some(rhaystack.current_ptr()));
                }
            } else if j > 0 {
                let count = table[j];
                j -= count;
                for _ in 0..count {
                    if !rhaystack.avail() {
                        return Err("Invalid string position");
                    }
                    rhaystack.advance();
                }
            } else {
                if !rhaystack.avail() {
                    return Err("Invalid string position");
                }
                rhaystack.advance();
                phaystack.advance();
            }
        }
    }

    Ok(None)
}

pub fn mbsstr(haystack: &str, needle: &str) -> Option<*const c_char> {
    if needle.is_empty() {
        return Some(haystack.as_ptr() as *const c_char);
    }

    let mut try_kmp = true;
    let mut outer_loop_count = 0;
    let mut comparison_count = 0;
    let mut last_ccount = 0;
    let mut needle_last_ccount = needle;

    let mut iter_haystack = MbStrIter::new(haystack);
    let mut iter_needle = MbStrIter::new(needle);

    if !iter_needle.avail() {
        return Some(haystack.as_ptr() as *const c_char);
    }

    let first_needle_char = iter_needle.current().unwrap();

    while iter_haystack.avail() {
        if try_kmp
            && outer_loop_count >= 10
            && comparison_count >= 5 * outer_loop_count
        {
            let count = comparison_count - last_ccount;
            let mut iter = MbStrIter::new(needle_last_ccount);
            for _ in 0..count {
                if !iter.avail() {
                    break;
                }
                iter.advance();
            }
            last_ccount = comparison_count;
            if !iter.avail() {
                if let Ok(result) = knuth_morris_pratt_multibyte(haystack, needle) {
                    return result;
                }
                try_kmp = false;
            }
        }

        outer_loop_count += 1;
        comparison_count += 1;

        if let Some(haystack_char) = iter_haystack.current() {
            if haystack_char.equal(&first_needle_char) {
                let mut rhaystack = iter_haystack.clone();
                rhaystack.advance();

                let mut rneedle = MbStrIter::new(needle);
                rneedle.advance();

                loop {
                    if !rneedle.avail() {
                        return Some(iter_haystack.current_ptr());
                    }
                    if !rhaystack.avail() {
                        return None;
                    }
                    comparison_count += 1;
                    match (rneedle.current(), rhaystack.current()) {
                        (Some(nc), Some(hc)) if nc.equal(&hc) => {
                            rneedle.advance();
                            rhaystack.advance();
                        }
                        _ => break,
                    }
                }
            }
        }

        iter_haystack.advance();
    }

    None
}