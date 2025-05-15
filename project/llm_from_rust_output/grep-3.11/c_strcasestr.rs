use std::cmp::Ordering;
use std::ptr;

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
        return max_suffix + 1;
    }
    *period = p;
    max_suffix_rev + 1
}

fn two_way_short_needle(
    haystack: &[u8],
    needle: &[u8],
    suffix: usize,
    period: usize,
) -> Option<usize> {
    let needle_len = needle.len();
    let haystack_len = haystack.len();

    if strncasecmp(&needle, &needle[period..], suffix) == 0 {
        let mut memory = 0;
        let mut j = 0;

        while j + needle_len <= haystack_len {
            let mut i = if suffix < memory { memory } else { suffix };
            while i < needle_len && to_lower(needle[i]) == to_lower(haystack[j + i]) {
                i += 1;
            }

            if i == needle_len {
                let mut i = suffix.wrapping_sub(1);
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
    } else {
        let period = if suffix < needle_len - suffix {
            needle_len - suffix
        } else {
            suffix
        } + 1;
        let mut j = 0;

        while j + needle_len <= haystack_len {
            let mut i = suffix;
            while i < needle_len && to_lower(needle[i]) == to_lower(haystack[j + i]) {
                i += 1;
            }

            if i == needle_len {
                let mut i = suffix.wrapping_sub(1);
                while i != usize::MAX && to_lower(needle[i]) == to_lower(haystack[j + i]) {
                    i = i.wrapping_sub(1);
                }

                if i == usize::MAX {
                    return Some(j);
                }
                j += period;
            } else {
                j += i - suffix + 1;
            }
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

    if strncasecmp(&needle, &needle[period..], suffix) == 0 {
        let mut memory = 0;
        let mut j = 0;

        while j + needle_len <= haystack_len {
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

                if needle_len - 1 <= i {
                    let mut i = suffix.wrapping_sub(1);
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
    } else {
        let period = if suffix < needle_len - suffix {
            needle_len - suffix
        } else {
            suffix
        } + 1;
        let mut j = 0;

        while j + needle_len <= haystack_len {
            let shift = shift_table[to_lower(haystack[j + needle_len - 1]) as usize];
            if shift > 0 {
                j += shift;
            } else {
                let mut i = suffix;
                while i < needle_len - 1
                    && to_lower(needle[i]) == to_lower(haystack[j + i])
                {
                    i += 1;
                }

                if needle_len - 1 <= i {
                    let mut i = suffix.wrapping_sub(1);
                    while i != usize::MAX
                        && to_lower(needle[i]) == to_lower(haystack[j + i])
                    {
                        i = i.wrapping_sub(1);
                    }

                    if i == usize::MAX {
                        return Some(j);
                    }
                    j += period;
                } else {
                    j += i - suffix + 1;
                }
            }
        }
    }
    None
}

fn strncasecmp(s1: &[u8], s2: &[u8], n: usize) -> i32 {
    for i in 0..n {
        let c1 = to_lower(s1[i]);
        let c2 = to_lower(s2[i]);
        if c1 != c2 {
            return (c1 as i32) - (c2 as i32);
        }
    }
    0
}

pub fn c_strcasestr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let needle_len = needle.len();
    if haystack.len() < needle_len {
        return None;
    }

    let mut ok = true;
    for (h, n) in haystack.iter().zip(needle.iter()) {
        if to_lower(*h) != to_lower(*n) {
            ok = false;
            break;
        }
    }
    if ok {
        return Some(0);
    }

    let mut period = 0;
    let suffix = critical_factorization(needle, &mut period);

    if needle_len < 32 {
        two_way_short_needle(&haystack[1..], needle, suffix, period)
            .map(|pos| pos + 1)
    } else {
        two_way_long_needle(&haystack[1..], needle, suffix, period)
            .map(|pos| pos + 1)
    }
}