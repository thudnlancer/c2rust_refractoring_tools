use std::{
    f64::{INFINITY, NAN},
    mem,
    ptr,
    str::{self, FromStr},
    string::String,
    vec::Vec,
};

const REDIS_ERR: i32 = -1;
const REDIS_OK: i32 = 0;

const REDIS_ERR_IO: i32 = 1;
const REDIS_ERR_EOF: i32 = 3;
const REDIS_ERR_PROTOCOL: i32 = 4;
const REDIS_ERR_OOM: i32 = 5;
const REDIS_ERR_TIMEOUT: i32 = 6;
const REDIS_ERR_OTHER: i32 = 2;

const REDIS_REPLY_STRING: i32 = 1;
const REDIS_REPLY_ARRAY: i32 = 2;
const REDIS_REPLY_INTEGER: i32 = 3;
const REDIS_REPLY_NIL: i32 = 4;
const REDIS_REPLY_STATUS: i32 = 5;
const REDIS_REPLY_ERROR: i32 = 6;
const REDIS_REPLY_DOUBLE: i32 = 7;
const REDIS_REPLY_BOOL: i32 = 8;
const REDIS_REPLY_MAP: i32 = 9;
const REDIS_REPLY_SET: i32 = 10;
const REDIS_REPLY_ATTR: i32 = 11;
const REDIS_REPLY_PUSH: i32 = 12;
const REDIS_REPLY_BIGNUM: i32 = 13;
const REDIS_REPLY_VERB: i32 = 14;

const REDIS_READER_MAX_BUF: usize = 1024 * 16;
const REDIS_READER_MAX_ARRAY_ELEMENTS: i64 = (1 << 32) - 1;
const REDIS_READER_STACK_SIZE: usize = 9;

#[derive(Debug)]
pub struct RedisReadTask {
    type_: i32,
    elements: i64,
    idx: i32,
    obj: Option<Box<dyn std::any::Any>>,
    parent: Option<Box<RedisReadTask>>,
    privdata: Option<Box<dyn std::any::Any>>,
}

pub trait RedisReplyObjectFunctions {
    fn create_string(
        &self,
        task: &RedisReadTask,
        str: &str,
        len: usize,
    ) -> Option<Box<dyn std::any::Any>>;
    fn create_array(&self, task: &RedisReadTask, len: usize) -> Option<Box<dyn std::any::Any>>;
    fn create_integer(&self, task: &RedisReadTask, value: i64) -> Option<Box<dyn std::any::Any>>;
    fn create_double(
        &self,
        task: &RedisReadTask,
        value: f64,
        str: &str,
        len: usize,
    ) -> Option<Box<dyn std::any::Any>>;
    fn create_nil(&self, task: &RedisReadTask) -> Option<Box<dyn std::any::Any>>;
    fn create_bool(&self, task: &RedisReadTask, value: bool) -> Option<Box<dyn std::any::Any>>;
    fn free_object(&self, obj: Box<dyn std::any::Any>);
}

#[derive(Debug)]
pub struct RedisReader {
    err: i32,
    errstr: String,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
    maxbuf: usize,
    maxelements: i64,
    tasks: Vec<RedisReadTask>,
    ridx: i32,
    reply: Option<Box<dyn std::any::Any>>,
    fn_: Box<dyn RedisReplyObjectFunctions>,
    privdata: Option<Box<dyn std::any::Any>>,
}

impl RedisReader {
    pub fn new(fn_: Box<dyn RedisReplyObjectFunctions>) -> Option<Self> {
        let mut tasks = Vec::with_capacity(REDIS_READER_STACK_SIZE);
        for _ in 0..REDIS_READER_STACK_SIZE {
            tasks.push(RedisReadTask {
                type_: -1,
                elements: -1,
                idx: -1,
                obj: None,
                parent: None,
                privdata: None,
            });
        }

        Some(RedisReader {
            err: 0,
            errstr: String::new(),
            buf: Vec::new(),
            pos: 0,
            len: 0,
            maxbuf: REDIS_READER_MAX_BUF,
            maxelements: REDIS_READER_MAX_ARRAY_ELEMENTS,
            tasks,
            ridx: -1,
            reply: None,
            fn_,
            privdata: None,
        })
    }

    fn set_error(&mut self, type_: i32, str: &str) {
        if let Some(reply) = self.reply.take() {
            self.fn_.free_object(reply);
        }

        self.buf.clear();
        self.pos = 0;
        self.len = 0;
        self.ridx = -1;

        self.err = type_;
        self.errstr = str.to_string();
    }

    fn set_error_protocol_byte(&mut self, byte: u8) {
        let byte_str = match byte {
            b'\\' | b'"' => format!("\"\\{}\"", byte as char),
            b'\n' => "\"\\n\"".to_string(),
            b'\r' => "\"\\r\"".to_string(),
            b'\t' => "\"\\t\"".to_string(),
            b'\a' => "\"\\a\"".to_string(),
            b'\b' => "\"\\b\"".to_string(),
            _ => {
                if byte.is_ascii_graphic() {
                    format!("\"{}\"", byte as char)
                } else {
                    format!("\"\\x{:02x}\"", byte)
                }
            }
        };

        self.set_error(
            REDIS_ERR_PROTOCOL,
            &format!("Protocol error, got {} as reply type byte", byte_str),
        );
    }

    fn set_error_oom(&mut self) {
        self.set_error(REDIS_ERR_OOM, "Out of memory");
    }

    fn read_bytes(&mut self, bytes: usize) -> Option<&[u8]> {
        if self.len - self.pos >= bytes {
            let start = self.pos;
            self.pos += bytes;
            Some(&self.buf[start..self.pos])
        } else {
            None
        }
    }

    fn seek_newline(s: &[u8]) -> Option<usize> {
        if s.len() < 2 {
            return None;
        }

        for i in 0..s.len() - 1 {
            if s[i] == b'\r' && s[i + 1] == b'\n' {
                return Some(i);
            }
        }

        None
    }

    fn string2ll(s: &[u8]) -> Option<i64> {
        if s.is_empty() {
            return None;
        }

        let (negative, mut p) = if s[0] == b'-' {
            (true, &s[1..])
        } else {
            (false, s)
        };

        if p.is_empty() {
            return None;
        }

        if p.len() == 1 && p[0] == b'0' {
            return Some(0);
        }

        if p[0] == b'0' {
            return None;
        }

        let mut v: u64 = 0;
        for &c in p {
            if !c.is_ascii_digit() {
                return None;
            }
            let digit = (c - b'0') as u64;
            if v > u64::MAX / 10 {
                return None;
            }
            v *= 10;
            if v > u64::MAX - digit {
                return None;
            }
            v += digit;
        }

        if negative {
            if v > (i64::MAX as u64) + 1 {
                return None;
            }
            Some(-(v as i64))
        } else {
            if v > i64::MAX as u64 {
                return None;
            }
            Some(v as i64)
        }
    }

    fn read_line(&mut self) -> Option<&[u8]> {
        let p = &self.buf[self.pos..];
        if let Some(pos) = Self::seek_newline(p) {
            let line = &p[..pos];
            self.pos += pos + 2;
            Some(line)
        } else {
            None
        }
    }

    fn move_to_next_task(&mut self) {
        while self.ridx >= 0 {
            if self.ridx == 0 {
                self.ridx = -1;
                return;
            }

            let cur = &mut self.tasks[self.ridx as usize];
            let prv = &mut self.tasks[(self.ridx - 1) as usize];

            match prv.type_ {
                REDIS_REPLY_ARRAY | REDIS_REPLY_MAP | REDIS_REPLY_ATTR | REDIS_REPLY_SET | REDIS_REPLY_PUSH => {
                    if cur.idx == prv.elements as i32 - 1 {
                        self.ridx -= 1;
                    } else {
                        cur.type_ = -1;
                        cur.elements = -1;
                        cur.idx += 1;
                        return;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn process_line_item(&mut self) -> i32 {
        let cur = &mut self.tasks[self.ridx as usize];
        let line = match self.read_line() {
            Some(line) => line,
            None => return REDIS_ERR,
        };

        let obj = match cur.type_ {
            REDIS_REPLY_INTEGER => {
                let v = match Self::string2ll(line) {
                    Some(v) => v,
                    None => {
                        self.set_error(REDIS_ERR_PROTOCOL, "Bad integer value");
                        return REDIS_ERR;
                    }
                };
                self.fn_.create_integer(cur, v)
            }
            REDIS_REPLY_DOUBLE => {
                let s = match str::from_utf8(line) {
                    Ok(s) => s,
                    Err(_) => {
                        self.set_error(REDIS_ERR_PROTOCOL, "Bad double value");
                        return REDIS_ERR;
                    }
                };

                let d = match s {
                    "inf" => INFINITY,
                    "-inf" => -INFINITY,
                    "nan" | "-nan" => NAN,
                    _ => match s.parse::<f64>() {
                        Ok(d) if d.is_finite() => d,
                        _ => {
                            self.set_error(REDIS_ERR_PROTOCOL, "Bad double value");
                            return REDIS_ERR;
                        }
                    },
                };

                self.fn_.create_double(cur, d, s, s.len())
            }
            REDIS_REPLY_NIL => {
                if !line.is_empty() {
                    self.set_error(REDIS_ERR_PROTOCOL, "Bad nil value");
                    return REDIS_ERR;
                }
                self.fn_.create_nil(cur)
            }
            REDIS_REPLY_BOOL => {
                if line.len() != 1 || !matches!(line[0], b't' | b'T' | b'f' | b'F') {
                    self.set_error(REDIS_ERR_PROTOCOL, "Bad bool value");
                    return REDIS_ERR;
                }
                let bval = matches!(line[0], b't' | b'T');
                self.fn_.create_bool(cur, bval)
            }
            REDIS_REPLY_BIGNUM => {
                let mut i = 0;
                if line[0] == b'-' {
                    i += 1;
                }
                while i < line.len() {
                    if !line[i].is_ascii_digit() {
                        self.set_error(REDIS_ERR_PROTOCOL, "Bad bignum value");
                        return REDIS_ERR;
                    }
                    i += 1;
                }
                self.fn_
                    .create_string(cur, str::from_utf8(line).unwrap(), line.len())
            }
            REDIS_REPLY_ERROR | REDIS_REPLY_STATUS => {
                if line.iter().any(|&c| c == b'\r' || c == b'\n') {
                    self.set_error(REDIS_ERR_PROTOCOL, "Bad simple string value");
                    return REDIS_ERR;
                }
                self.fn_
                    .create_string(cur, str::from_utf8(line).unwrap(), line.len())
            }
            _ => unreachable!(),
        };

        if obj.is_none() {
            self.set_error_oom();
            return REDIS_ERR;
        }

        if self.ridx == 0 {
            self.reply = obj;
        }
        self.move_to_next_task();
        REDIS_OK
    }

    fn process_bulk_item(&mut self) -> i32 {
        let cur = &mut self.tasks[self.ridx as usize];
        let line = match self.read_line() {
            Some(line) => line,
            None => return REDIS_ERR,
        };

        let len = match Self::string2ll(line) {
            Some(len) => len,
            None => {
                self.set_error(REDIS_ERR_PROTOCOL, "Bad bulk string length");
                return REDIS_ERR;
            }
        };

        if len < -1 || len > i32::MAX as i64 {
            self.set_error(REDIS_ERR_PROTOCOL, "Bulk string length out of range");
            return REDIS_ERR;
        }

        if len == -1 {
            let obj = self.fn_.create_nil(cur);
            if obj.is_none() {
                self.set_error_oom();
                return REDIS_ERR;
            }
            if self.ridx == 0 {
                self.reply = obj;
            }
            self.move_to_next_task();
            REDIS_OK
        } else {
            let bulk_len = len as usize;
            if self.pos + bulk_len + 2 > self.len {
                return REDIS_ERR;
            }

            if cur.type_ == REDIS_REPLY_VERB {
                if bulk_len < 4 || self.buf[self.pos + 4] != b':' {
                    self.set_error(
                        REDIS_ERR_PROTOCOL,
                        "Verbatim string 4 bytes of content type are missing or incorrectly encoded.",
                    );
                    return REDIS_ERR;
                }
            }

            let obj = self.fn_.create_string(
                cur,
                str::from_utf8(&self.buf[self.pos..self.pos + bulk_len]).unwrap(),
                bulk_len,
            );
            if obj.is_none() {
                self.set_error_oom();
                return REDIS_ERR;
            }

            self.pos += bulk_len + 2;
            if self.ridx == 0 {
                self.reply = obj;
            }
            self.move_to_next_task();
            REDIS_OK
        }
    }

    fn grow(&mut self) -> i32 {
        let new_len = self.tasks.len() + REDIS_READER_STACK_SIZE;
        self.tasks.reserve(REDIS_READER_STACK_SIZE);
        for _ in 0..REDIS_READER_STACK_SIZE {
            self.tasks.push(RedisReadTask {
                type_: -1,
                elements: -1,
                idx: -1,
                obj: None,
                parent: None,
                privdata: None,
            });
        }
        REDIS_OK
    }

    fn process_aggregate_item(&mut self) -> i32 {
        let cur = &mut self.tasks[self.ridx as usize];
        let line = match self.read_line() {
            Some(line) => line,
            None => return REDIS_ERR,
        };

        let elements = match Self::string2ll(line) {
            Some(elements) => elements,
            None => {
                self.set_error(REDIS_ERR_PROTOCOL, "Bad multi-bulk length");
                return REDIS_ERR;
            }
        };

        let root = self.ridx == 0;

        if elements < -1
            || elements > i32::MAX as i64
            || (self.maxelements > 0 && elements > self.maxelements)
        {
            self.set_error(REDIS_ERR_PROTOCOL, "Multi-bulk length out of range");
            return REDIS_ERR;
        }

        if elements == -1 {
            let obj = self.fn_.create_nil(cur);
            if obj.is_none() {
                self.set_error_oom();
                return REDIS_ERR;
            }
            self.move_to_next_task();
        } else {
            let elements = if cur.type_ == REDIS_REPLY_MAP || cur.type_ == REDIS_REPLY_ATTR {
                elements * 2
            } else {
                elements
            };

            let obj = self.fn_.create_array(cur, elements as usize);
            if obj.is_none() {
                self.set_error_oom();
                return REDIS_ERR;
            }

            if elements > 0 {
                cur.elements = elements;
                cur.obj = obj;
                self.ridx += 1;
                if self.ridx as usize >= self.tasks.len() {
                    if self.grow() == REDIS_ERR {
                        return REDIS_ERR;
                    }
                }
                let next = &mut self.tasks[self.ridx as usize];
                next.type_ = -1;
                next.elements = -1;
                next.idx = 0;
                next.obj = None;
                next.parent = Some(Box::new(mem::take(cur)));
                next.privdata = self.privdata.take();
            } else {
                self.move_to_next_task();
            }
        }

        if root {
            self.reply = self.tasks[0].obj.take();
        }
        REDIS_OK
    }

    fn process_item(&mut self) -> i32 {
        let cur = &mut self.tasks[self.ridx as usize];
        if cur.type_ < 0 {
            let p = match self.read_bytes(1) {
                Some(p) => p,
                None => return REDIS_ERR,
            };

            cur.type_ = match p[0] {
                b'-' => REDIS_REPLY_ERROR,
               