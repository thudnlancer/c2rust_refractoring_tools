use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

/// Case-insensitive string search using Knuth-Morris-Pratt algorithm
pub fn mbscasestr(haystack: &str, needle: &str) -> Option<&str> {
    if needle.is_empty() {
        return Some(haystack);
    }

    // For ASCII strings, use simple case-insensitive comparison
    if haystack.is_ascii() && needle.is_ascii() {
        return haystack
            .as_bytes()
            .windows(needle.len())
            .position(|window| {
                window
                    .iter()
                    .zip(needle.as_bytes())
                    .all(|(&h, &n)| h.eq_ignore_ascii_case(&n))
            })
            .map(|pos| &haystack[pos..pos + needle.len()]);
    }

    // For multibyte strings, use more complex case-insensitive comparison
    let mut haystack_chars = haystack.chars().peekable();
    let first_needle_char = needle.chars().next()?.to_lowercase().next()?;

    let mut outer_loop_count = 0;
    let mut comparison_count = 0;
    let mut last_ccount = 0;
    let mut needle_last_ccount = needle.chars();
    let mut try_kmp = true;

    while let Some(haystack_char) = haystack_chars.next() {
        // Check if we should try KMP algorithm
        if try_kmp
            && outer_loop_count >= 10
            && comparison_count >= 5 * outer_loop_count
        {
            // Advance needle_last_ccount by comparison_count - last_ccount
            let count = comparison_count - last_ccount;
            for _ in 0..count {
                if needle_last_ccount.next().is_none() {
                    // Try KMP
                    if let Some(pos) = kmp_search(haystack, needle) {
                        return Some(&haystack[pos..pos + needle.len()]);
                    }
                    try_kmp = false;
                    break;
                }
            }
            last_ccount = comparison_count;
        }

        outer_loop_count += 1;
        comparison_count += 1;

        if haystack_char.to_lowercase().eq(first_needle_char.to_lowercase()) {
            let mut rhaystack = haystack_chars.clone();
            let mut rneedle = needle.chars();
            rneedle.next(); // Skip first character we already matched

            let mut matched = true;
            for needle_char in rneedle {
                comparison_count += 1;
                match rhaystack.next() {
                    Some(h) if h.to_lowercase().eq(needle_char.to_lowercase()) => continue,
                    _ => {
                        matched = false;
                        break;
                    }
                }
            }

            if matched {
                let start_pos = haystack.len() - haystack_chars.as_str().len() - 1;
                return Some(&haystack[start_pos..start_pos + needle.len()]);
            }
        }
    }

    None
}

/// Knuth-Morris-Pratt search implementation
fn kmp_search(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let pattern = needle
        .chars()
        .map(|c| c.to_lowercase().collect::<String>())
        .collect::<Vec<_>>();
    let text = haystack
        .chars()
        .map(|c| c.to_lowercase().collect::<String>())
        .collect::<Vec<_>>();

    let lps = compute_lps_array(&pattern);

    let mut i = 0; // index for text
    let mut j = 0; // index for pattern

    while i < text.len() {
        if pattern[j] == text[i] {
            i += 1;
            j += 1;
        }

        if j == pattern.len() {
            return Some(i - j);
        } else if i < text.len() && pattern[j] != text[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    None
}

/// Compute the longest prefix suffix array for KMP algorithm
fn compute_lps_array(pattern: &[String]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut len = 0;
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }

    lps
}