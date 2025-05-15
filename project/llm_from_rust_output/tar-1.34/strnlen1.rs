use std::ffi::CStr;

pub type size_t = usize;

pub fn strnlen1(string: &[u8], maxlen: size_t) -> size_t {
    if let Some(pos) = string.iter().take(maxlen).position(|&c| c == 0) {
        pos + 1
    } else {
        maxlen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strnlen1() {
        let s = b"hello\0world";
        assert_eq!(strnlen1(s, 10), 6);
        assert_eq!(strnlen1(s, 5), 5);
        assert_eq!(strnlen1(b"hello", 5), 5);
    }
}