use std::cmp::Ordering;
use std::mem;

fn to_lower(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c + 32,
        _ => c,
    }
}

fn critical_factorization(needle: &[u8], period: &mut usize) -> usize {
    let len = needle.len();
    if len < 3 {
        *period = 1;
        return len - 1;
    }

    let mut max_suffix = usize::MAX;
    let mut j = 0;
    let mut p = 1;
    let mut k = p;

    while j + k < len {
        let a = to_lower(needle[j + k]);
        let b = to_lower(needle[max_suffix.wrapping_add(k)]);
        match a.cmp(&b) {
            Ordering::Less => {
                j += k;
                k = 1;
                p = j - max_suffix;
            }
            Ordering::Equal => {
                if k != p {
                    k += 1;
                } else {
                    j += p;
                    k = 1;
                }
            }
            Ordering::Greater => {
                max_suffix = j;
                j += 1;
                p = 1;
                k = p;
            }
        }
    }

    *period = p;
    let mut max_suffix_rev = usize::MAX;
    j = 0;
    p = 1;
    k = p;

    while j + k < len {
        let a = to_lower(needle[j + k]);
        let b = to_lower(needle[max_suffix_rev.wrapping_add(k)]);
        match b.cmp(&a) {
            Ordering::Less => {
                j += k;
                k = 1;
                p = j - max_suffix_rev;
            }
            Ordering::Equal => {
                if k != p {
                    k += 1;
                } else {
                    j += p;
                    k = 1;
                }
            }
            Ordering::Greater => {
                max_suffix_rev = j;
                j += 1;
                p = 1;
                k = p;
            }
        }
    }

    if max_suffix_rev + 1 < max_suffix + 1 {
        max_suffix + 1
    } else {
        *period = p;
        max_suffix_rev + 1
    }
}

fn two_way_short_needle(
    haystack: &[u8],
    needle: &[u8],
    suffix: usize,
    period: usize,
) -> Option<usize> {
    let needle_len = needle.len();
    let haystack_len = haystack.len();
    let mut j = 0;

    if needle_len > haystack_len {
        return None;
    }

    let mut memory = 0;
    while j <= haystack_len - needle_len {
        let mut i = if suffix < memory { memory } else { suffix };
        while i < needle_len && to_lower(needle[i]) == to_lower(haystack[j + i]) {
            i += 1;
        }

        if i == needle_len {
            let mut i = suffix - 1;
            while memory <= i && to_lower(needle[i]) == to_lower(haystack[j + i]) {
                i = i.wrapping_sub(1);
            }
            if i < memory {
                return Some(j);
            }
            j += period;
            memory = needle_len - period;
        } else {
            j += i - suffix + 1;
            memory = 0;
        }
    }

    None
}

fn two_way_long_needle(
    haystack: &[u8],
    needle: &[u8],
    suffix: usize,
    period: usize,
) -> Option<usize> {
    let needle_len = needle.len();
    let haystack_len = haystack.len();
    let mut shift_table = [needle_len; 256];

    for (i, c) in needle.iter().enumerate() {
        shift_table[to_lower(*c) as usize] = needle_len - i - 1;
    }

    let mut j = 0;
    let mut memory = 0;

    while j <= haystack_len - needle_len {
        let shift = shift_table[to_lower(haystack[j + needle_len - 1]) as usize];
        if shift > 0 {
            if memory != 0 && shift < period {
                j += needle_len - period;
            } else {
                j += shift;
            }
            memory = 0;
        } else {
            let mut i = if suffix < memory { memory } else { suffix };
            while i < needle_len - 1
                && to_lower(needle[i]) == to_lower(haystack[j + i])
            {
                i += 1;
            }

            if i == needle_len - 1 {
                let mut i = suffix - 1;
                while memory <= i
                    && to_lower(needle[i]) == to_lower(haystack[j + i])
                {
                    i = i.wrapping_sub(1);
                }
                if i < memory {
                    return Some(j);
                }
                j += period;
                memory = needle_len - period;
            } else {
                j += i - suffix + 1;
                memory = 0;
            }
        }
    }

    None
}

pub fn strcasestr(haystack: &str, needle: &str) -> Option<usize> {
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();

    if needle_bytes.is_empty() {
        return Some(0);
    }

    if haystack_bytes.len() < needle_bytes.len() {
        return None;
    }

    let mut period = 0;
    let suffix = critical_factorization(needle_bytes, &mut period);

    if needle_bytes.len() < 32 {
        two_way_short_needle(haystack_bytes, needle_bytes, suffix, period)
    } else {
        two_way_long_needle(haystack_bytes, needle_bytes, suffix, period)
    }
}