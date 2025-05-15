use std::collections::LinkedList;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::ptr;
use std::slice;
use std::str;
use std::sync::Mutex;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransformType {
    First,
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplaceSegmType {
    Literal,
    Backref,
    CaseCtl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaseCtlType {
    Stop,
    UpcaseNext,
    LocaseNext,
    Upcase,
    Locase,
}

struct ReplaceSegm {
    next: Option<Box<ReplaceSegm>>,
    typ: ReplaceSegmType,
    v: ReplaceSegmVariant,
}

enum ReplaceSegmVariant {
    Literal { ptr: String, size: usize },
    Backref { ref_num: usize },
    CaseCtl { ctl: CaseCtlType },
}

struct Transform {
    next: Option<Box<Transform>>,
    transform_type: TransformType,
    flags: i32,
    match_number: usize,
    regex: Regex,
    repl_head: Option<Box<ReplaceSegm>>,
    repl_tail: Option<*mut ReplaceSegm>,
    segm_count: usize,
}

lazy_static! {
    static ref TRANSFORM_FLAGS: Mutex<i32> = Mutex::new(XFORM_ALL);
    static ref TRANSFORM_HEAD: Mutex<Option<Box<Transform>>> = Mutex::new(None);
    static ref TRANSFORM_TAIL: Mutex<Option<*mut Transform>> = Mutex::new(None);
}

const XFORM_ALL: i32 = 0;
const XFORM_REGFILE: i32 = 1 << 0;
const XFORM_LINK: i32 = 1 << 1;
const XFORM_SYMLINK: i32 = 1 << 2;

fn new_transform() -> Box<Transform> {
    let mut tf = Box::new(Transform {
        next: None,
        transform_type: TransformType::First,
        flags: 0,
        match_number: 0,
        regex: Regex::new("").unwrap(),
        repl_head: None,
        repl_tail: None,
        segm_count: 0,
    });

    let mut tail = TRANSFORM_TAIL.lock().unwrap();
    let mut head = TRANSFORM_HEAD.lock().unwrap();

    if let Some(tail_ptr) = *tail {
        unsafe {
            (*tail_ptr).next = Some(tf.clone());
        }
    } else {
        *head = Some(tf.clone());
    }

    *tail = Some(Box::into_raw(tf.clone()));
    tf
}

fn add_segment(tf: &mut Transform) -> &mut ReplaceSegm {
    let segm = Box::new(ReplaceSegm {
        next: None,
        typ: ReplaceSegmType::Literal,
        v: ReplaceSegmVariant::Literal {
            ptr: String::new(),
            size: 0,
        },
    });

    let segm_ptr = Box::into_raw(segm);
    unsafe {
        if let Some(tail_ptr) = tf.repl_tail {
            (*tail_ptr).next = Some(Box::from_raw(segm_ptr));
        } else {
            tf.repl_head = Some(Box::from_raw(segm_ptr));
        }
        tf.repl_tail = Some(segm_ptr);
        tf.segm_count += 1;
        &mut *segm_ptr
    }
}

fn add_literal_segment(tf: &mut Transform, str: &str, end: &str) {
    let len = end.len() - str.len();
    if len > 0 {
        let segm = add_segment(tf);
        segm.typ = ReplaceSegmType::Literal;
        segm.v = ReplaceSegmVariant::Literal {
            ptr: str[..len].to_string(),
            size: len,
        };
    }
}

fn add_char_segment(tf: &mut Transform, chr: char) {
    let segm = add_segment(tf);
    segm.typ = ReplaceSegmType::Literal;
    segm.v = ReplaceSegmVariant::Literal {
        ptr: chr.to_string(),
        size: 1,
    };
}

fn add_backref_segment(tf: &mut Transform, ref_num: usize) {
    let segm = add_segment(tf);
    segm.typ = ReplaceSegmType::Backref;
    segm.v = ReplaceSegmVariant::Backref { ref_num };
}

fn parse_xform_flags(pflags: &mut i32, c: char) -> Result<(), String> {
    match c {
        'r' => *pflags |= XFORM_REGFILE,
        'R' => *pflags &= !XFORM_REGFILE,
        'h' => *pflags |= XFORM_LINK,
        'H' => *pflags &= !XFORM_LINK,
        's' => *pflags |= XFORM_SYMLINK,
        'S' => *pflags &= !XFORM_SYMLINK,
        _ => return Err(format!("Unknown transform flag: {}", c)),
    }
    Ok(())
}

fn add_case_ctl_segment(tf: &mut Transform, ctl: CaseCtlType) {
    let segm = add_segment(tf);
    segm.typ = ReplaceSegmType::CaseCtl;
    segm.v = ReplaceSegmVariant::CaseCtl { ctl };
}

fn parse_transform_expr(expr: &str) -> Result<&str, String> {
    let mut expr = expr;
    let delim;
    let mut tf = new_transform();

    if expr.is_empty() {
        return Err("Empty transform expression".to_string());
    }

    if expr.starts_with("flags=") {
        let mut flags = 0;
        expr = &expr[6..];
        while !expr.is_empty() {
            let c = expr.chars().next().unwrap();
            if c == ';' {
                expr = &expr[1..];
                break;
            }
            parse_xform_flags(&mut flags, c)?;
            expr = &expr[1..];
        }
        *TRANSFORM_FLAGS.lock().unwrap() = flags;
        return Ok(expr);
    }

    if !expr.starts_with('s') {
        return Err("Invalid transform expression".to_string());
    }

    delim = expr.chars().nth(1).ok_or("Invalid transform expression")?;
    expr = &expr[2..];

    // Scan regular expression
    let mut i = 0;
    let mut chars = expr.chars();
    while let Some(c) = chars.next() {
        if c == delim {
            break;
        }
        if c == '\\' {
            chars.next();
        }
        i += 1;
    }

    if i >= expr.len() || expr.chars().nth(i).unwrap() != delim {
        return Err("Invalid transform expression".to_string());
    }

    // Scan replacement expression
    let mut j = i + 1;
    let mut chars = expr[i + 1..].chars();
    while let Some(c) = chars.next() {
        if c == delim {
            break;
        }
        if c == '\\' {
            chars.next();
        }
        j += 1;
    }

    if j >= expr.len() || expr.chars().nth(j).unwrap() != delim {
        return Err("Invalid transform expression".to_string());
    }

    // Check flags
    tf.transform_type = TransformType::First;
    tf.flags = *TRANSFORM_FLAGS.lock().unwrap();
    let mut cflags = regex::RegexBuilder::new("");
    let mut p = j + 1;
    while p < expr.len() {
        let c = expr.chars().nth(p).unwrap();
        match c {
            'g' => tf.transform_type = TransformType::Global,
            'i' => cflags.case_insensitive(true),
            'x' => cflags.ignore_whitespace(true),
            '0'..='9' => {
                let num_str = &expr[p..];
                let num_end = num_str.find(|c: char| !c.is_digit(10)).unwrap_or(num_str.len());
                tf.match_number = num_str[..num_end].parse().unwrap();
                p += num_end - 1;
            }
            _ => {
                if parse_xform_flags(&mut tf.flags, c).is_err() {
                    return Err(format!("Unknown flag in transform expression: {}", c));
                }
            }
        }
        p += 1;
    }

    if p < expr.len() && expr.chars().nth(p).unwrap() == ';' {
        p += 1;
    }

    // Extract and compile regex
    let regex_str = &expr[..i];
    tf.regex = cflags.build(regex_str).map_err(|e| e.to_string())?;

    // Extract and compile replacement expr
    let repl_str = &expr[i + 1..j];
    let mut cur = repl_str;
    let mut beg = repl_str;

    while !cur.is_empty() {
        if cur.starts_with('\\') {
            add_literal_segment(&mut tf, beg, &cur[..1]);
            cur = &cur[1..];
            if cur.is_empty() {
                break;
            }
            let c = cur.chars().next().unwrap();
            match c {
                '0'..='9' => {
                    let num_end = cur.find(|c: char| !c.is_digit(10)).unwrap_or(cur.len());
                    let n = cur[..num_end].parse().unwrap();
                    if n > tf.regex.captures_len().unwrap_or(0) {
                        return Err("Invalid transform replacement: back reference out of range".to_string());
                    }
                    add_backref_segment(&mut tf, n);
                    cur = &cur[num_end..];
                }
                '\\' => {
                    add_char_segment(&mut tf, '\\');
                    cur = &cur[1..];
                }
                'a' => {
                    add_char_segment(&mut tf, '\x07');
                    cur = &cur[1..];
                }
                'b' => {
                    add_char_segment(&mut tf, '\x08');
                    cur = &cur[1..];
                }
                'f' => {
                    add_char_segment(&mut tf, '\x0C');
                    cur = &cur[1..];
                }
                'n' => {
                    add_char_segment(&mut tf, '\n');
                    cur = &cur[1..];
                }
                'r' => {
                    add_char_segment(&mut tf, '\r');
                    cur = &cur[1..];
                }
                't' => {
                    add_char_segment(&mut tf, '\t');
                    cur = &cur[1..];
                }
                'v' => {
                    add_char_segment(&mut tf, '\x0B');
                    cur = &cur[1..];
                }
                '&' => {
                    add_char_segment(&mut tf, '&');
                    cur = &cur[1..];
                }
                'L' => {
                    add_case_ctl_segment(&mut tf, CaseCtlType::Locase);
                    cur = &cur[1..];
                }
                'l' => {
                    add_case_ctl_segment(&mut tf, CaseCtlType::LocaseNext);
                    cur = &cur[1..];
                }
                'U' => {
                    add_case_ctl_segment(&mut tf, CaseCtlType::Upcase);
                    cur = &cur[1..];
                }
                'u' => {
                    add_case_ctl_segment(&mut tf, CaseCtlType::UpcaseNext);
                    cur = &cur[1..];
                }
                'E' => {
                    add_case_ctl_segment(&mut tf, CaseCtlType::Stop);
                    cur = &cur[1..];
                }
                _ => {
                    if c == delim {
                        add_char_segment(&mut tf, delim);
                    } else {
                        add_literal_segment(&mut tf, &format!("\\{}", c), &format!("\\{}", c));
                    }
                    cur = &cur[1..];
                }
            }
            beg = cur;
        } else if cur.starts_with('&') {
            add_literal_segment(&mut tf, beg, &cur[..1]);
            add_backref_segment(&mut tf, 0);
            cur = &cur[1..];
            beg = cur;
        } else {
            cur = &cur[1..];
        }
    }
    add_literal_segment(&mut tf, beg, cur);

    Ok(&expr[p..])
}

fn set_transform_expr(expr: &str) -> Result<(), String> {
    let mut expr = expr;
    while !expr.is_empty() {
        expr = parse_transform_expr(expr)?;
    }
    Ok(())
}

fn run_case_conv(case_ctl: CaseCtlType, s: &str) -> String {
    match case_ctl {
        CaseCtlType::UpcaseNext => {
            let mut chars = s.chars();
            if let Some(c) = chars.next() {
                c.to_uppercase().chain(chars).collect()
            } else {
                String::new()
            }
        }
        CaseCtlType::LocaseNext => {
            let mut chars = s.chars();
            if let Some(c) = chars.next() {
                c.to_lowercase().chain(chars).collect()
            } else {
                String::new()
            }
        }
        CaseCtlType::Upcase => s.to_uppercase(),
        CaseCtlType::Locase => s.to_lowercase(),
        CaseCtlType::Stop => s.to_string(),
    }
}

struct TransformObstack {
    data: Vec<u8>,
}

impl TransformObstack {
    fn new() -> Self {
        TransformObstack { data: Vec::new() }
    }

    fn grow(&mut self, s: &str) {
        self.data.extend_from_slice(s.as_bytes());
    }

    fn finish(&mut self) -> String {
        self.data.push(0);
        let s = unsafe { CString::from_vec_with_nul_unchecked(self.data.clone()) };
        self.data.clear();
        s.into_string().unwrap()
    }
}

lazy_static! {
    static ref STK: Mutex<TransformObstack> = Mutex::new(TransformObstack::new());
}

fn single_transform_name_to_obstack(tf: &Transform, input: &str) -> String {
    let mut stk = STK.lock().unwrap();
    let mut case_ctl = CaseCtlType::Stop;
    let mut save_ctl = CaseCtlType::Stop;
    let mut input = input.to_string();
    let mut nmatches = 0;

    while !input.is_empty() {
        if let Some(caps) = tf.regex.captures(&input) {
            let full_match = caps.get(0).unwrap();
            let disp = full_match.end();

            if full_match.start() > 0 {
                stk.grow(&input[..full_match.start()]);
            }

            nmatches += 1;
            if tf.match_number > 0 && nmatches < tf.match_number {
                stk.grow(&input[..disp]);
                input = input[disp..].to_string();
                continue;
            }

            let mut segm = &tf.repl_head;
            while let Some(s) = segm {
                match &s.v {
                    ReplaceSegmVariant::Literal { ptr, size } => {
                        let s = if case_ctl == CaseCtlType::Stop {
                            ptr.clone()
                        } else {
                            run_case_conv(case_ctl, ptr)
                        };
                        stk.grow(&s);
                        if case_ctl == CaseCtlType::UpcaseNext || case_ctl == CaseCtlType::LocaseNext {
                            case_ctl = save_ctl;
                            save_ctl = CaseCtlType::Stop;
                        }
                    }
                    ReplaceSegmVariant::Backref { ref_num } => {
                        if let Some(m) = caps.get(*ref_num) {
                            let s = if case_ctl == CaseCtlType::Stop {
                                m.as_str().to_string()
                            } else {
                                run_case_conv(case_ctl, m.as_str())
                            };
                            stk.grow(&s);
                            if case_ctl == CaseCtlType::UpcaseNext || case_ctl == CaseCtlType::LocaseNext {
                                case_ctl = save_ctl;
                                save_ctl = CaseCtlType::Stop;
                            }
                        }
                    }
                    ReplaceSegmVariant::CaseCtl { ctl } => {
                        match ctl {
                            CaseCtlType::UpcaseNext | CaseCtlType::LocaseNext => {
                                match save_ctl {
                                    CaseCtlType::Stop | CaseCtlType::Upcase | CaseCtlType::Locase => {
                                        save_ctl = case_ctl;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                        case_ctl = *ctl;
                    }
                }
                segm = &s.next;
            }

            input = input[disp..].to_string();

            if tf.transform_type == TransformType::First {
                stk.grow(&input);
                break;
            }
        } else {
            stk.grow(&input);
            break;
        }
    }

    stk.finish()
}

fn transform_name_to_obstack(flags: i32, input: &str) -> Option<String> {
    let mut output = input.to_string();
    let mut alloced = false;
    let head = TRANSFORM_HEAD.lock().unwrap();

    if let Some(tf) = &*head {
        let mut current = tf;
        loop {
            if current.flags & flags != 0 {
                output = single_transform_name_to_obstack(current, &output);
                alloced = true;
            }
            if let Some(next) = &current.next {
                current = next;
            } else {
                break;
            }
        }
    }

    if alloced {
        Some(output)
    } else {
        None
    }
}

fn transform_name_fp(
    pinput: &mut String,
    flags: i32,
    fun: Option<fn(&str) -> String>,
) -> bool {
    if let Some(output) = transform_name_to