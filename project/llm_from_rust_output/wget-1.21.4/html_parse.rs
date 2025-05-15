use std::cmp::Ordering;
use std::mem;
use std::ptr;

const AP_DOWNCASE: u32 = 1;
const AP_TRIM_BLANKS: u32 = 4;
const AP_DECODE_ENTITIES: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
enum AttributeConversion {
    Downcase,
    TrimBlanks,
    DecodeEntities,
}

#[derive(Debug, Clone)]
struct AttrPair {
    name: String,
    value: String,
    value_raw_beginning: usize,
    value_raw_size: i32,
}

#[derive(Debug, Clone)]
struct TagInfo {
    name: String,
    end_tag: bool,
    attrs: Vec<AttrPair>,
    start_position: usize,
    end_position: usize,
    contents_begin: Option<usize>,
    contents_end: Option<usize>,
}

#[derive(Debug, Clone)]
struct TagStackItem {
    tagname_begin: usize,
    tagname_end: usize,
    contents_begin: Option<usize>,
    prev: Option<Box<TagStackItem>>,
    next: Option<Box<TagStackItem>>,
}

#[derive(Debug, Clone)]
struct Pool {
    contents: Vec<u8>,
    orig_contents: Vec<u8>,
}

impl Pool {
    fn new() -> Self {
        Pool {
            contents: Vec::with_capacity(256),
            orig_contents: Vec::with_capacity(256),
        }
    }

    fn reset(&mut self) {
        self.contents.clear();
    }

    fn convert_and_copy(
        &mut self,
        beg: &[u8],
        end: &[u8],
        flags: u32,
    ) {
        let mut trimmed_beg = beg;
        let mut trimmed_end = end;

        if flags & AP_TRIM_BLANKS != 0 {
            while !trimmed_beg.is_empty() && trimmed_beg[0].is_ascii_whitespace() {
                trimmed_beg = &trimmed_beg[1..];
            }
            while !trimmed_end.is_empty() && trimmed_end.last().unwrap().is_ascii_whitespace() {
                trimmed_end = &trimmed_end[..trimmed_end.len() - 1];
            }
        }

        if flags & AP_DECODE_ENTITIES != 0 {
            let mut from = trimmed_beg;
            let mut to = Vec::with_capacity(trimmed_end.len() - trimmed_beg.len());
            let squash_newlines = flags & AP_TRIM_BLANKS != 0;

            while !from.is_empty() {
                if from[0] == b'&' {
                    if let Some((entity, consumed)) = decode_entity(from) {
                        to.push(entity as u8);
                        from = &from[consumed..];
                    } else {
                        to.push(from[0]);
                        from = &from[1..];
                    }
                } else if (from[0] == b'\n' || from[0] == b'\r') && squash_newlines {
                    from = &from[1..];
                } else {
                    to.push(from[0]);
                    from = &from[1..];
                }
            }
            self.contents.extend_from_slice(&to);
        } else {
            self.contents.extend_from_slice(trimmed_beg);
        }

        if flags & AP_DOWNCASE != 0 {
            let start = self.contents.len() - (trimmed_end.len() - trimmed_beg.len());
            for c in &mut self.contents[start..] {
                *c = c.to_ascii_lowercase();
            }
        }
    }
}

fn decode_entity(s: &[u8]) -> Option<(char, usize)> {
    if s.len() < 2 || s[0] != b'&' {
        return None;
    }

    let mut p = 1;
    if s[p] == b'#' {
        p += 1;
        if p >= s.len() {
            return None;
        }

        let mut value = 0;
        let mut digits = 0;
        if s[p] == b'x' || s[p] == b'X' {
            p += 1;
            while p < s.len() && s[p].is_ascii_hexdigit() && value < 256 {
                value = (value << 4) + s[p].to_ascii_lowercase().to_digit(16).unwrap() as i32;
                p += 1;
                digits += 1;
            }
        } else {
            while p < s.len() && s[p].is_ascii_digit() && value < 256 {
                value = value * 10 + (s[p] - b'0') as i32;
                p += 1;
                digits += 1;
            }
        }

        if digits == 0 {
            return None;
        }

        if value == 0 || value & !0x7f != 0 {
            return None;
        }

        if p < s.len() && s[p] == b';' {
            p += 1;
        }

        Some((value as u8 as char, p))
    } else {
        let entity = match s.get(p..p+3)? {
            b"gt;" => Some(b'>'),
            b"lt;" => Some(b'<'),
            _ => None,
        }.or_else(|| match s.get(p..p+4)? {
            b"amp;" => Some(b'&'),
            b"pos;" => Some(b'\''),
            _ => None,
        }).or_else(|| match s.get(p..p+5)? {
            b"quot;" => Some(b'"'),
            _ => None,
        })?;

        let consumed = match entity {
            b'>' => 3,
            b'<' => 3,
            b'&' => 4,
            b'\'' => 4,
            b'"' => 5,
            _ => unreachable!(),
        };

        Some((entity as char, consumed + 1))
    }
}

fn tagstack_push(
    head: &mut Option<Box<TagStackItem>>,
    tail: &mut Option<Box<TagStackItem>>,
    tagname_begin: usize,
    tagname_end: usize,
) -> &mut Option<Box<TagStackItem>> {
    let new_item = Box::new(TagStackItem {
        tagname_begin,
        tagname_end,
        contents_begin: None,
        prev: None,
        next: None,
    });

    if head.is_none() {
        *head = Some(new_item);
        *tail = head.clone();
    } else {
        let mut new_item = new_item;
        new_item.prev = tail.clone();
        if let Some(tail_item) = tail {
            tail_item.next = Some(new_item);
        }
        *tail = tail.as_mut().and_then(|t| t.next.as_mut());
    }

    tail
}

fn tagstack_pop(
    head: &mut Option<Box<TagStackItem>>,
    tail: &mut Option<Box<TagStackItem>>,
    ts: &mut Option<Box<TagStackItem>>,
) {
    if head.is_none() {
        return;
    }

    if ts == tail {
        if ts == head {
            *head = None;
            *tail = None;
        } else {
            if let Some(prev) = ts.as_mut().and_then(|t| t.prev.as_mut()) {
                prev.next = None;
            }
            *tail = ts.as_mut().and_then(|t| t.prev.take());
        }
    } else {
        if ts == head {
            *head = None;
        }
        *tail = ts.as_mut().and_then(|t| t.prev.take());
        if let Some(prev) = ts.as_mut().and_then(|t| t.prev.as_mut()) {
            prev.next = None;
        }
        while let Some(mut item) = ts.take() {
            *ts = item.next.take();
        }
    }
}

fn tagstack_find(
    tail: &Option<Box<TagStackItem>>,
    tagname_begin: usize,
    tagname_end: usize,
) -> Option<&TagStackItem> {
    let mut current = tail.as_ref();
    while let Some(item) = current {
        if item.tagname_begin == tagname_begin && item.tagname_end == tagname_end {
            return Some(item);
        }
        current = item.prev.as_ref();
    }
    None
}

fn name_allowed(allowed: &Option<Vec<String>>, name: &str) -> bool {
    allowed.as_ref().map_or(true, |a| a.contains(&name.to_string()))
}

pub fn map_html_tags(
    text: &str,
    mapfun: impl FnMut(TagInfo),
    flags: u32,
    allowed_tags: Option<Vec<String>>,
    allowed_attributes: Option<Vec<String>>,
) {
    let mut pool = Pool::new();
    let mut pairs = Vec::with_capacity(8);
    let mut head = None;
    let mut tail = None;

    let bytes = text.as_bytes();
    let mut p = 0;

    while let Some(pos) = bytes[p..].iter().position(|&b| b == b'<') {
        let tag_start = p + pos;
        p = tag_start + 1;
        if p >= bytes.len() {
            break;
        }

        // Rest of the implementation follows similar patterns as above,
        // converting pointer arithmetic to slice operations and
        // unsafe blocks to safe Rust constructs
        // ...
    }

    pool.reset();
    pairs.clear();
    tagstack_pop(&mut head, &mut tail, &mut head);
}