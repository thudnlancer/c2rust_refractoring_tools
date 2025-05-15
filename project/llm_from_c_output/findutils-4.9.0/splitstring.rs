// splitstring.rs -- split a const string into fields.
// Original C code Copyright (C) 2011-2022 Free Software Foundation, Inc.
// Rust translation maintains same license terms.

/// Split a string into fields. The string is never modified.
///
/// Returns `Some((pos, len))` if there is a next field, where:
/// - `pos` is the starting position of the field
/// - `len` is the length of the field
///
/// Returns `None` when there are no more fields.
///
/// # Parameters
/// - `s`: The input string to split
/// - `separators`: Characters that separate fields
/// - `first`: Set to `true` on the first call for any given string
/// - `state`: A mutable reference to the splitting state (position and length)
///
/// Any character in `separators` is a field separator.
/// Consecutive separators indicate empty fields.
pub fn split_string(
    s: &str,
    separators: &str,
    first: bool,
    state: &mut (usize, usize),
) -> Option<(usize, usize)> {
    if first {
        *state = (0, 0);
    } else {
        state.0 += state.1; // Advance to next field
        if s.chars().nth(state.0).is_some() {
            state.0 += 1; // Skip separator
        } else {
            return None; // Reached end
        }
    }

    let remaining = &s[state.0..];
    state.1 = field_length(remaining, separators);
    Some(*state)
}

/// Calculate the length of the current field
fn field_length(s: &str, separators: &str) -> usize {
    // If no separators, whole input is one field
    if separators.is_empty() {
        return s.len();
    }

    s.find(|c| separators.contains(c))
        .unwrap_or_else(|| s.len())
}