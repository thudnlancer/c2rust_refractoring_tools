use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::path::{Path, PathBuf};
use std::borrow::Cow;

lazy_static! {
    static ref PREFIX_TABLE: [Option<HashTable<String>>; 2] = [None, None];
}

struct HashTable<T> {
    // Implementation of a hash table would go here
    // For simplicity, we'll use a Vec for demonstration
    entries: Vec<T>,
}

impl<T: Eq + Hash> HashTable<T> {
    fn new() -> Self {
        HashTable { entries: Vec::new() }
    }

    fn insert(&mut self, value: T) -> Option<T> {
        if self.entries.contains(&value) {
            None
        } else {
            self.entries.push(value);
            Some(value)
        }
    }

    fn contains(&self, value: &T) -> bool {
        self.entries.contains(value)
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

fn hash_string_hasher(name: &str, n_buckets: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    (hasher.finish() as usize) % n_buckets
}

fn hash_string_compare(name1: &str, name2: &str) -> bool {
    name1 == name2
}

fn hash_string_insert_prefix(
    table: &mut Option<HashTable<String>>,
    string: &str,
    len: usize,
) -> (bool, Option<String>) {
    let s = if len > 0 {
        string[..len].to_string()
    } else {
        string.to_string()
    };

    let table = table.get_or_insert_with(HashTable::new);
    if let Some(existing) = table.insert(s.clone()) {
        (false, None)
    } else {
        (true, Some(s))
    }
}

fn removed_prefixes_p() -> bool {
    PREFIX_TABLE[0].as_ref().map_or(false, |t| t.len() > 0) ||
    PREFIX_TABLE[1].as_ref().map_or(false, |t| t.len() > 0)
}

fn safer_name_suffix(file_name: &str, link_target: bool, absolute_names: bool) -> Cow<'_, str> {
    let p = if absolute_names {
        file_name
    } else {
        // Skip file system prefixes, leading file name components that contain "..",
        // and leading slashes.
        let mut prefix_len = 0; // FILE_SYSTEM_PREFIX_LEN would be implemented here
        let mut chars = file_name.chars().collect::<Vec<_>>();
        let mut i = prefix_len;

        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == '.' && chars[i + 1] == '.' {
                if i + 2 >= chars.len() || is_slash(chars[i + 2]) {
                    prefix_len = i + 2;
                }
            }

            while i < chars.len() {
                let c = chars[i];
                i += 1;
                if is_slash(c) {
                    break;
                }
            }
        }

        while prefix_len < chars.len() && is_slash(chars[prefix_len]) {
            prefix_len += 1;
        }

        if prefix_len > 0 {
            let prefix = &file_name[..prefix_len];
            let mut table = PREFIX_TABLE[link_target as usize].as_mut().unwrap();
            let (inserted, new_prefix) = hash_string_insert_prefix(&mut table, file_name, prefix_len);
            if inserted {
                let diagnostic = if link_target {
                    "Removing leading `{}' from hard link targets"
                } else {
                    "Removing leading `{}' from member names"
                };
                eprintln!(diagnostic, new_prefix.unwrap());
            }
        }

        &file_name[prefix_len..]
    };

    if p.is_empty() {
        let diagnostic = if link_target {
            "Substituting `.' for empty hard link target"
        } else {
            "Substituting `.' for empty member name"
        };
        eprintln!("{}", diagnostic);
        Cow::Borrowed(".")
    } else {
        Cow::Borrowed(p)
    }
}

fn is_slash(c: char) -> bool {
    c == '/' || c == '\\'
}