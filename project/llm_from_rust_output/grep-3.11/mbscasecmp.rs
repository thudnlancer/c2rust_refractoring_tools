use std::cmp::Ordering;
use std::char;
use std::str;

#[derive(Debug, Clone, Copy)]
struct MbChar {
    bytes: usize,
    wc: Option<char>,
}

impl MbChar {
    fn new() -> Self {
        MbChar {
            bytes: 0,
            wc: None,
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        let c = s.chars().next()?;
        Some(MbChar {
            bytes: c.len_utf8(),
            wc: Some(c),
        })
    }
}

fn mbscasecmp(s1: &str, s2: &str) -> i32 {
    if s1 == s2 {
        return 0;
    }

    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    loop {
        let c1 = iter1.next();
        let c2 = iter2.next();

        match (c1, c2) {
            (None, None) => return 0,
            (None, _) => return -1,
            (_, None) => return 1,
            (Some(ch1), Some(ch2)) => {
                let lower1 = ch1.to_lowercase().next().unwrap();
                let lower2 = ch2.to_lowercase().next().unwrap();
                
                match lower1.cmp(&lower2) {
                    Ordering::Equal => continue,
                    Ordering::Less => return -1,
                    Ordering::Greater => return 1,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbscasecmp() {
        assert_eq!(mbscasecmp("hello", "HELLO"), 0);
        assert_eq!(mbscasecmp("hello", "world") < 0, true);
        assert_eq!(mbscasecmp("WORLD", "hello") > 0, true);
        assert_eq!(mbscasecmp("", ""), 0);
        assert_eq!(mbscasecmp("", "a"), -1);
        assert_eq!(mbscasecmp("a", ""), 1);
    }
}