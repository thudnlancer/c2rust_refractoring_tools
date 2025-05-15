use std::io::{self, Read};
use std::fs::File;
use std::ptr;
use std::mem;

/// A dynamic string that grows as needed.
#[derive(Debug)]
pub struct DynamicString {
    size: usize,    // Actual amount of storage allocated
    idx: usize,     // Index of the next free byte
    string: String, // String storage
}

impl Default for DynamicString {
    fn default() -> Self {
        DynamicString {
            size: 0,
            idx: 0,
            string: String::new(),
        }
    }
}

impl DynamicString {
    /// Creates a new empty DynamicString.
    pub fn new() -> Self {
        Self::default()
    }

    /// Frees the dynamic string storage.
    pub fn free(&mut self) {
        self.string.clear();
        self.size = 0;
        self.idx = 0;
    }

    /// Resizes the string to have capacity for at least `additional` more bytes.
    fn resize(&mut self, additional: usize) {
        let required_len = self.idx + additional;
        if required_len >= self.size {
            let new_size = if self.size == 0 {
                required_len.max(1)
            } else {
                let mut new_size = self.size;
                while new_size <= required_len {
                    new_size *= 2;
                }
                new_size
            };
            self.string.reserve(new_size - self.size);
            self.size = new_size;
        }
    }

    /// Resets the string to given length.
    pub fn reset(&mut self, len: usize) {
        self.resize(len);
        self.idx = len;
        if self.idx > 0 {
            unsafe { self.string.as_mut_vec().set_len(len) };
        }
    }

    /// Reads a string from file until EOS character is encountered.
    pub fn fgetstr(&mut self, f: &mut File, eos: char) -> io::Result<Option<&str>> {
        self.idx = 0;
        self.string.clear();

        let mut buf = [0u8; 1];
        loop {
            match f.read_exact(&mut buf) {
                Ok(_) => {
                    let ch = buf[0] as char;
                    if ch == eos {
                        break;
                    }
                    self.string.push(ch);
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    if self.string.is_empty() {
                        return Ok(None);
                    }
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        Ok(Some(&self.string))
    }

    /// Appends a character to the string.
    pub fn append(&mut self, c: char) {
        self.resize(1);
        self.string.push(c);
        self.idx = self.string.len();
    }

    /// Concatenates another string to this one.
    pub fn concat(&mut self, s: &str) {
        let len = s.len();
        self.resize(len);
        self.string.push_str(s);
        self.idx = self.string.len();
    }

    /// Reads a line from file (until newline).
    pub fn fgets(&mut self, f: &mut File) -> io::Result<Option<&str>> {
        self.fgetstr(f, '\n')
    }

    /// Reads a null-terminated string from file.
    pub fn fgetname(&mut self, f: &mut File) -> io::Result<Option<&str>> {
        self.fgetstr(f, '\0')
    }

    /// Returns the current length of the string.
    pub fn len(&self) -> usize {
        self.idx
    }

    /// Checks if the string ends with given character.
    pub fn ends_with(&self, c: char) -> bool {
        self.string.ends_with(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_operations() {
        let mut ds = DynamicString::new();
        assert_eq!(ds.len(), 0);

        ds.append('a');
        assert_eq!(ds.len(), 1);
        assert_eq!(ds.string, "a");

        ds.concat("bc");
        assert_eq!(ds.len(), 3);
        assert_eq!(ds.string, "abc");

        assert!(ds.ends_with('c'));
        assert!(!ds.ends_with('a'));

        ds.free();
        assert_eq!(ds.len(), 0);
        assert!(ds.string.is_empty());
    }

    #[test]
    fn test_fgetstr() {
        let mut ds = DynamicString::new();
        let mut f = Cursor::new(b"hello\nworld");
        
        assert_eq!(ds.fgets(&mut f).unwrap().unwrap(), "hello");
        assert_eq!(ds.fgets(&mut f).unwrap().unwrap(), "world");
        assert!(ds.fgets(&mut f).unwrap().is_none());
    }
}