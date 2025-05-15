use std::collections::HashMap;
use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

#[derive(Debug)]
struct AttrPair {
    name: String,
    value: String,
    value_raw_beginning: *const c_char,
    value_raw_size: c_int,
    name_pool_index: c_int,
    value_pool_index: c_int,
}

#[derive(Debug)]
struct TagInfo {
    name: String,
    end_tag_p: bool,
    nattrs: c_int,
    attrs: Vec<AttrPair>,
    start_position: *const c_char,
    end_position: *const c_char,
    contents_begin: *const c_char,
    contents_end: *const c_char,
}

type HashTable = HashMap<String, ()>;

type TagMapper = fn(&TagInfo, &mut c_void);

const MHT_STRICT_COMMENTS: c_int = 1;
const MHT_TRIM_VALUES: c_int = 2;

const AP_DOWNCASE: c_int = 1;
const AP_DECODE_ENTITIES: c_int = 2;
const AP_TRIM_BLANKS: c_int = 4;

struct Pool {
    contents: Vec<u8>,
    size: usize,
    tail: usize,
    resized: bool,
    orig_contents: Vec<u8>,
    orig_size: usize,
}

impl Pool {
    fn new(initial_storage: &[u8]) -> Self {
        Pool {
            contents: initial_storage.to_vec(),
            size: initial_storage.len(),
            tail: 0,
            resized: false,
            orig_contents: initial_storage.to_vec(),
            orig_size: initial_storage.len(),
        }
    }

    fn grow(&mut self, increase: usize) {
        let needed = self.tail + increase;
        if needed > self.size {
            let mut new_size = self.size;
            while new_size < needed {
                new_size <<= 1;
            }
            if self.resized {
                self.contents.resize(new_size, 0);
            } else {
                let mut new_contents = vec![0; new_size];
                new_contents[..self.size].copy_from_slice(&self.contents);
                self.contents = new_contents;
                self.resized = true;
            }
            self.size = new_size;
        }
    }

    fn append(&mut self, data: &[u8]) {
        self.grow(data.len());
        self.contents[self.tail..self.tail + data.len()].copy_from_slice(data);
        self.tail += data.len();
    }

    fn append_char(&mut self, ch: char) {
        self.grow(1);
        self.contents[self.tail] = ch as u8;
        self.tail += 1;
    }

    fn rewind(&mut self) {
        self.tail = 0;
    }

    fn free(&mut self) {
        if self.resized {
            self.contents = self.orig_contents.clone();
            self.size = self.orig_size;
            self.resized = false;
        }
        self.tail = 0;
    }
}

struct TagStackItem {
    tagname_begin: *const c_char,
    tagname_end: *const c_char,
    contents_begin: *const c_char,
    prev: Option<Box<TagStackItem>>,
    next: Option<Box<TagStackItem>>,
}

impl TagStackItem {
    fn new() -> Self {
        TagStackItem {
            tagname_begin: ptr::null(),
            tagname_end: ptr::null(),
            contents_begin: ptr::null(),
            prev: None,
            next: None,
        }
    }
}

fn tagstack_push(head: &mut Option<Box<TagStackItem>>, tail: &mut Option<Box<TagStackItem>>) -> Option<Box<TagStackItem>> {
    let mut ts = Box::new(TagStackItem::new());
    if head.is_none() {
        *head = Some(ts);
        *tail = head.clone();
        ts.prev = None;
        ts.next = None;
    } else {
        if let Some(tail_item) = tail {
            tail_item.next = Some(ts);
            ts.prev = Some(tail_item);
        }
        *tail = Some(ts);
        ts.next = None;
    }
    Some(ts)
}

fn tagstack_pop(head: &mut Option<Box<TagStackItem>>, tail: &mut Option<Box<TagStackItem>>, ts: Option<Box<TagStackItem>>) {
    if head.is_none() {
        return;
    }

    if let Some(ts) = ts {
        if Some(&ts) == tail.as_ref() {
            if Some(&ts) == head.as_ref() {
                *head = None;
                *tail = None;
            } else {
                if let Some(prev) = ts.prev {
                    prev.next = None;
                    *tail = Some(prev);
                }
            }
        } else {
            if Some(&ts) == head.as_ref() {
                *head = None;
            }
            if let Some(prev) = ts.prev {
                prev.next = None;
                *tail = Some(prev);
            }
            let mut next = ts.next;
            while let Some(item) = next {
                next = item.next;
            }
        }
    }
}

fn tagstack_find(tail: Option<&Box<TagStackItem>>, tagname_begin: *const c_char, tagname_end: *const c_char) -> Option<Box<TagStackItem>> {
    let len = unsafe { tagname_end.offset_from(tagname_begin) } as usize;
    let mut current = tail;
    while let Some(item) = current {
        let item_len = unsafe { item.tagname_end.offset_from(item.tagname_begin) } as usize;
        if len == item_len {
            let tagname = unsafe { CStr::from_ptr(tagname_begin).to_string_lossy().to_lowercase() };
            let item_tagname = unsafe { CStr::from_ptr(item.tagname_begin).to_string_lossy().to_lowercase() };
            if tagname == item_tagname {
                return Some(item.clone());
            }
        }
        current = item.prev.as_ref();
    }
    None
}

fn decode_entity(ptr: &mut *const c_char, end: *const c_char) -> i32 {
    unsafe {
        let mut p = *ptr;
        if p >= end {
            return -1;
        }
        p = p.offset(1);
        if p >= end {
            return -1;
        }

        match *p {
            b'#' => {
                p = p.offset(1);
                let mut digits = 0;
                let mut value = 0;
                if p < end && *p == b'x' {
                    p = p.offset(1);
                    while value < 256 && p < end && (*p as char).is_ascii_hexdigit() {
                        value = (value << 4) + (*p as char).to_digit(16).unwrap() as i32;
                        p = p.offset(1);
                        digits += 1;
                    }
                } else {
                    while value < 256 && p < end && (*p as char).is_ascii_digit() {
                        value = (value * 10) + (*p as char).to_digit(10).unwrap() as i32;
                        p = p.offset(1);
                        digits += 1;
                    }
                }
                if digits == 0 {
                    return -1;
                }
                if value == 0 || value > 127 {
                    return -1;
                }
                if p < end && *p == b';' {
                    p = p.offset(1);
                }
                *ptr = p;
                value
            }
            b'g' => {
                if p.offset(1) < end && *p.offset(1) == b't' {
                    *ptr = p.offset(2);
                    if p.offset(2) < end && *p.offset(2) == b';' {
                        *ptr = p.offset(3);
                    }
                    b'>' as i32
                } else {
                    -1
                }
            }
            b'l' => {
                if p.offset(1) < end && *p.offset(1) == b't' {
                    *ptr = p.offset(2);
                    if p.offset(2) < end && *p.offset(2) == b';' {
                        *ptr = p.offset(3);
                    }
                    b'<' as i32
                } else {
                    -1
                }
            }
            b'a' => {
                if p.offset(1) < end && *p.offset(1) == b'm' && p.offset(2) < end && *p.offset(2) == b'p' {
                    *ptr = p.offset(3);
                    if p.offset(3) < end && *p.offset(3) == b';' {
                        *ptr = p.offset(4);
                    }
                    b'&' as i32
                } else if p.offset(1) < end && *p.offset(1) == b'p' && p.offset(2) < end && *p.offset(2) == b'o' && p.offset(3) < end && *p.offset(3) == b's' {
                    *ptr = p.offset(4);
                    if p.offset(4) < end && *p.offset(4) == b';' {
                        *ptr = p.offset(5);
                    }
                    b'\'' as i32
                } else {
                    -1
                }
            }
            b'q' => {
                if p.offset(1) < end && *p.offset(1) == b'u' && p.offset(2) < end && *p.offset(2) == b'o' && p.offset(3) < end && *p.offset(3) == b't' {
                    *ptr = p.offset(4);
                    if p.offset(4) < end && *p.offset(4) == b';' {
                        *ptr = p.offset(5);
                    }
                    b'\"' as i32
                } else {
                    -1
                }
            }
            _ => -1,
        }
    }
}

fn convert_and_copy(pool: &mut Pool, beg: *const c_char, end: *const c_char, flags: c_int) {
    let old_tail = pool.tail;
    let mut beg = beg;
    let mut end = end;

    unsafe {
        if (flags & AP_TRIM_BLANKS) != 0 {
            while beg < end && (*beg as char).is_whitespace() {
                beg = beg.offset(1);
            }
            while end > beg && (*(end.offset(-1)) as char).is_whitespace() {
                end = end.offset(-1);
            }
        }

        if (flags & AP_DECODE_ENTITIES) != 0 {
            let mut from = beg;
            pool.grow((end as usize - beg as usize) as usize);
            let to = pool.contents.as_mut_ptr().offset(pool.tail as isize);

            let mut to_ptr = to;
            let mut squash_newlines = (flags & AP_TRIM_BLANKS) != 0;

            while from < end {
                if *from == b'&' {
                    let mut ptr = from;
                    let entity = decode_entity(&mut ptr, end);
                    if entity != -1 {
                        *to_ptr = entity as u8;
                        to_ptr = to_ptr.offset(1);
                        from = ptr;
                    } else {
                        *to_ptr = *from;
                        to_ptr = to_ptr.offset(1);
                        from = from.offset(1);
                    }
                } else if ((*from == b'\n' || *from == b'\r') && squash_newlines) {
                    from = from.offset(1);
                } else {
                    *to_ptr = *from;
                    to_ptr = to_ptr.offset(1);
                    from = from.offset(1);
                }
            }

            pool.tail = (to_ptr as usize - pool.contents.as_ptr() as usize) as usize;
            pool.append_char('\0');
        } else {
            let slice = std::slice::from_raw_parts(beg, end as usize - beg as usize);
            pool.append(slice);
            pool.append_char('\0');
        }

        if (flags & AP_DOWNCASE) != 0 {
            for i in old_tail..pool.tail {
                pool.contents[i] = (pool.contents[i] as char).to_ascii_lowercase() as u8;
            }
        }
    }
}

fn advance_declaration(beg: *const c_char, end: *const c_char, strict_comments: bool) -> *const c_char {
    unsafe {
        let mut p = beg;
        if p >= end {
            return p;
        }

        let mut state = if *p == b'!' {
            p = p.offset(1);
            2 // AC_S_DEFAULT
        } else {
            return beg.offset(1);
        };

        let mut quote_char = 0;

        while state != 0 && state != 1 {
            if p >= end {
                state = 1; // AC_S_BACKOUT
                break;
            }

            let ch = *p;
            match state {
                2 => { // AC_S_DEFAULT
                    match ch {
                        b'-' => state = 5, // AC_S_DASH1
                        b' ' | b'\t' | b'\r' | b'\n' => p = p.offset(1),
                        b'<' | b'>' => state = 0, // AC_S_DONE
                        b'\'' | b'\"' => {
                            state = 10; // AC_S_QUOTE1
                            quote_char = ch;
                        }
                        _ => {
                            if (ch as char).is_ascii_alphanumeric() || ch == b'.' || ch == b'-' {
                                state = 4; // AC_S_DCLNAME
                            } else {
                                state = 1; // AC_S_BACKOUT
                            }
                        }
                    }
                }
                4 => { // AC_S_DCLNAME
                    if ch == b'-' {
                        state = 5; // AC_S_DASH1
                    } else if (ch as char).is_ascii_alphanumeric() || ch == b'.' || ch == b'-' {
                        p = p.offset(1);
                    } else {
                        state = 2; // AC_S_DEFAULT
                    }
                }
                5 => { // AC_S_DASH1
                    if ch == b'-' {
                        p = p.offset(1);
                        state = 6; // AC_S_DASH2
                    } else {
                        state = 1; // AC_S_BACKOUT
                    }
                }
                6 => { // AC_S_DASH2
                    if ch == b'-' {
                        p = p.offset(1);
                        state = 8; // AC_S_COMMENT
                    } else {
                        state = 1; // AC_S_BACKOUT
                    }
                }
                8 => { // AC_S_COMMENT
                    if ch == b'-' {
                        state = 9; // AC_S_DASH3
                    } else {
                        p = p.offset(1);
                    }
                }
                9 => { // AC_S_DASH3
                    if ch == b'-' {
                        p = p.offset(1);
                        state = 7; // AC_S_DASH4
                    } else {
                        state = 1; // AC_S_BACKOUT
                    }
                }
                7 => { // AC_S_DASH4
                    if ch == b'-' {
                        p = p.offset(1);
                        state = 2; // AC_S_DEFAULT
                    } else {
                        state = 8; // AC_S_COMMENT
                    }
                }
                10 => { // AC_S_QUOTE1
                    p = p.offset(1);
                    state = 11; // AC_S_IN_QUOTE
                }
                11 => { // AC_S_IN_QUOTE
                    if ch == quote_char {
                        state = 12; // AC_S_QUOTE2
                    } else {
                        p = p.offset(1);
                    }
                }
                12 => { // AC_S_QUOTE2
                    p = p.offset(1);
                    state = 2; // AC_S_DEFAULT
                }
                _ => state = 1, // AC_S_BACKOUT
            }
        }

        if state == 1 {
            return beg.offset(1);
        }
        p
    }
}

fn find_comment_end(beg: *const c_char, end: *const c_char) -> *const c_char {
    unsafe {
        let mut p = beg;
        while p.offset(2) < end {
            if *p.offset(2) == b'>' && *p.offset(1) == b'-' && *p == b'-' {
                return p.offset(3);
            }
            p = p.offset(1);
        }
        ptr::null()
    }
}

fn name_allowed(ht: Option<&HashTable>, b: *const c_char, e: *const c_char) -> bool {
    unsafe {
        if ht.is_none() {
            return true;
        }

        let len = e as usize - b as usize;
        let name = CStr::from_ptr(b).to_string_lossy().into_owned();
        ht.unwrap().contains_key(&name)
    }
}

#[no_mangle]
pub extern "C" fn map_html_tags(
    text: *const c_char,
    size: c_int,
    mapfun: TagMapper,
    maparg: *mut c_void,
    flags: c_int,
    allowed_tags: *const HashTable,
    allowed_attributes: *const HashTable,
) {
    let mut pool_initial_storage = vec![0; 256];
    let mut pool = Pool::new(&pool_initial_storage);

    let mut p = text;
    let end = unsafe { text.offset(size as isize) };

    let mut attr_pair_initial_storage = vec![AttrPair::default(); 8];
    let mut attr_pair_size = attr_pair_initial_storage.len();
