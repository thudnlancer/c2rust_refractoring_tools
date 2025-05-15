use std::collections::HashMap;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::str;
use std::string::String;
use std::vec::Vec;

struct FunctionTableEntry {
    name: String,
    len: u8,
    minimum_args: u8,
    maximum_args: u8,
    expand_args: bool,
    alloc_fn: bool,
    adds_command: bool,
    func_ptr: fn(&mut String, &[String], &str) -> &mut String,
}

struct Variable {
    name: String,
    value: String,
    origin: VariableOrigin,
    recursive: bool,
    exp_count: u32,
}

enum VariableOrigin {
    OInvalid,
    ODefault,
    OEnv,
    OFile,
    OEnvOverride,
    OCommand,
    OOverride,
    OAutomatic,
}

struct HashTable {
    entries: HashMap<String, FunctionTableEntry>,
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            entries: HashMap::new(),
        }
    }

    fn insert(&mut self, entry: FunctionTableEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }

    fn lookup(&self, name: &str) -> Option<&FunctionTableEntry> {
        self.entries.get(name)
    }
}

fn subst_expand(
    output: &mut String,
    text: &str,
    subst: &str,
    replace: &str,
    slen: usize,
    rlen: usize,
    by_word: bool,
) -> &mut String {
    let mut t = text;
    let mut p;

    if slen == 0 && !by_word {
        output.push_str(t);
        if rlen > 0 {
            output.push_str(replace);
        }
        return output;
    }

    loop {
        if by_word && slen == 0 {
            p = end_of_token(next_token(t));
        } else {
            p = match t.find(subst) {
                Some(pos) => &t[pos..],
                None => {
                    output.push_str(t);
                    return output;
                }
            };
        }

        if p.as_ptr() > t.as_ptr() {
            let diff = unsafe { p.as_ptr().offset_from(t.as_ptr()) } as usize;
            output.push_str(&t[..diff]);
        }

        if by_word && (p.as_ptr() > text.as_ptr() && !p.chars().nth(0).unwrap().is_whitespace())
            || !STOP_SET(p.chars().nth(slen).unwrap(), MAP_SPACE | MAP_NUL)
        {
            output.push_str(subst);
        } else if rlen > 0 {
            output.push_str(replace);
        }

        t = &p[slen..];
        if t.is_empty() {
            break;
        }
    }

    output
}

fn patsubst_expand_pat(
    output: &mut String,
    text: &str,
    pattern: &str,
    replace: &str,
    pattern_percent: &str,
    replace_percent: &str,
) -> &mut String {
    let replace_prepercent_len = if !replace_percent.is_empty() {
        replace_percent.as_ptr() as usize - replace.as_ptr() as usize - 1
    } else {
        replace.len()
    };

    let replace_postpercent_len = if !replace_percent.is_empty() {
        replace_percent.len()
    } else {
        0
    };

    if pattern_percent.is_empty() {
        return subst_expand(
            output,
            text,
            pattern,
            replace,
            pattern.len(),
            replace.len(),
            true,
        );
    }

    let pattern_prepercent_len = pattern_percent.as_ptr() as usize - pattern.as_ptr() as usize - 1;
    let pattern_postpercent_len = pattern_percent.len();

    let mut doneany = false;
    let mut text_iter = text;
    let mut len;

    while let Some(t) = find_next_token(&mut text_iter, &mut len) {
        let mut fail = false;

        if len < pattern_prepercent_len + pattern_postpercent_len {
            fail = true;
        }

        if !fail
            && pattern_prepercent_len > 0
            && (t.chars().next() != pattern.chars().next()
                || t.chars().nth(pattern_prepercent_len - 1)
                    != pattern_percent.chars().nth_back(1)
                || !strneq(
                    &t[1..],
                    &pattern[1..],
                    pattern_prepercent_len - 1,
                ))
        {
            fail = true;
        }

        if !fail
            && pattern_postpercent_len > 0
            && (t.chars().last() != pattern_percent.chars().last()
                || t.chars().nth(len - pattern_postpercent_len) != pattern_percent.chars().next()
                || !strneq(
                    &t[len - pattern_postpercent_len..],
                    pattern_percent,
                    pattern_postpercent_len - 1,
                ))
        {
            fail = true;
        }

        if fail {
            output.push_str(&t[..len]);
        } else {
            output.push_str(&replace[..replace_prepercent_len]);

            if !replace_percent.is_empty() {
                output.push_str(&t[pattern_prepercent_len..len - pattern_postpercent_len]);
                output.push_str(replace_percent);
            }
        }

        output.push(' ');
        doneany = true;
    }

    if doneany {
        output.pop();
    }

    output
}

fn lookup_function(s: &str) -> Option<&FunctionTableEntry> {
    let mut e = s;
    while STOP_SET(e.chars().next().unwrap(), MAP_USERFUNC) {
        e = &e[1..];
    }

    if e == s || !STOP_SET(e.chars().next().unwrap(), MAP_NUL | MAP_SPACE) {
        return None;
    }

    let len = (e.as_ptr() as usize - s.as_ptr() as usize) as u8;
    function_table.lookup(&s[..len as usize])
}

fn pattern_matches(pattern: &str, percent: &str, s: &str) -> bool {
    if percent.is_empty() {
        let mut new_chars = pattern.to_string();
        let percent_pos = find_percent(&new_chars);
        if percent_pos.is_none() {
            return new_chars == s;
        }
        pattern = &new_chars;
        percent = percent_pos.unwrap();
    }

    let sfxlen = percent.len() - 1;
    let strlength = s.len();

    if strlength < (percent.as_ptr() as usize - pattern.as_ptr() as usize) + sfxlen
        || !strneq(pattern, s, percent.as_ptr() as usize - pattern.as_ptr() as usize)
    {
        return false;
    }

    &percent[1..] == &s[strlength - sfxlen..]
}

fn find_next_argument(
    startparen: char,
    endparen: char,
    ptr: &str,
    end: &str,
) -> Option<&str> {
    let mut count = 0;
    let mut p = ptr;

    while p < end {
        if !STOP_SET(p.chars().next().unwrap(), MAP_VARSEP | MAP_COMMA) {
            p = &p[1..];
            continue;
        }

        if p.starts_with(startparen) {
            count += 1;
            p = &p[1..];
        } else if p.starts_with(endparen) {
            count -= 1;
            if count < 0 {
                return None;
            }
            p = &p[1..];
        } else if p.starts_with(',') && count == 0 {
            return Some(p);
        } else {
            p = &p[1..];
        }
    }

    None
}

fn string_glob(line: &str) -> String {
    let mut result = String::new();
    let mut chain = parse_file_seq(line, true, false, false);

    while let Some(name) = chain.pop() {
        result.push_str(&name);
        result.push(' ');
    }

    if !result.is_empty() {
        result.pop();
    }

    result
}

fn func_patsubst(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    patsubst_expand(output, argv[2].as_str(), argv[0].as_str(), argv[1].as_str())
}

fn func_join(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let mut doneany = false;
    let mut list1_iter = argv[0].as_str();
    let mut list2_iter = argv[1].as_str();
    let mut len1;
    let mut len2;

    loop {
        let tp = find_next_token(&mut list1_iter, &mut len1);
        let pp = find_next_token(&mut list2_iter, &mut len2);

        if tp.is_some() {
            output.push_str(&tp.unwrap()[..len1]);
        }

        if pp.is_some() {
            output.push_str(&pp.unwrap()[..len2]);
        }

        if tp.is_some() || pp.is_some() {
            output.push(' ');
            doneany = true;
        } else {
            break;
        }
    }

    if doneany {
        output.pop();
    }

    output
}

fn func_origin(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let v = lookup_variable(&argv[0], argv[0].len());
    if v.is_none() {
        output.push_str("undefined");
    } else {
        match v.unwrap().origin {
            VariableOrigin::OInvalid => unreachable!(),
            VariableOrigin::ODefault => output.push_str("default"),
            VariableOrigin::OEnv => output.push_str("environment"),
            VariableOrigin::OFile => output.push_str("file"),
            VariableOrigin::OEnvOverride => output.push_str("environment override"),
            VariableOrigin::OCommand => output.push_str("command line"),
            VariableOrigin::OOverride => output.push_str("override"),
            VariableOrigin::OAutomatic => output.push_str("automatic"),
        }
    }

    output
}

fn func_flavor(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let v = lookup_variable(&argv[0], argv[0].len());
    if v.is_none() {
        output.push_str("undefined");
    } else if v.unwrap().recursive {
        output.push_str("recursive");
    } else {
        output.push_str("simple");
    }

    output
}

fn func_notdir_suffix(
    output: &mut String,
    argv: &[String],
    funcname: &str,
) -> &mut String {
    let is_suffix = funcname.starts_with('s');
    let is_notdir = !is_suffix;
    let stop = if is_suffix { MAP_DOT } else { 0 } | MAP_DIRSEP;

    let mut list_iter = argv[0].as_str();
    let mut len;
    let mut doneany = false;

    while let Some(p2) = find_next_token(&mut list_iter, &mut len) {
        let mut p = p2.chars().rev().peekable();
        let mut pos = len;

        while pos > 0 && !STOP_SET(p.peek().unwrap(), stop) {
            p.next();
            pos -= 1;
        }

        if pos > 0 {
            if is_notdir {
                pos += 1;
            } else if p2.chars().nth(pos - 1).unwrap() != '.' {
                continue;
            }
            output.push_str(&p2[pos..len]);
        } else if is_notdir {
            output.push_str(p2);
        }

        output.push(' ');
        doneany = true;
    }

    if doneany {
        output.pop();
    }

    output
}

fn func_basename_dir(
    output: &mut String,
    argv: &[String],
    funcname: &str,
) -> &mut String {
    let is_basename = funcname.starts_with('b');
    let is_dir = !is_basename;
    let stop = MAP_DIRSEP | if is_basename { MAP_DOT } else { 0 } | MAP_NUL;

    let mut p3 = argv[0].as_str();
    let mut len;
    let mut doneany = false;

    while let Some(p2) = find_next_token(&mut p3, &mut len) {
        let mut p = p2.chars().rev().peekable();
        let mut pos = len;

        while pos > 0 && !STOP_SET(p.peek().unwrap(), stop) {
            p.next();
            pos -= 1;
        }

        if pos > 0 && is_dir {
            output.push_str(&p2[..pos + 1]);
        } else if pos > 0 && p2.chars().nth(pos - 1).unwrap() == '.' {
            output.push_str(&p2[..pos - 1]);
        } else if is_dir {
            output.push_str("./");
        } else {
            output.push_str(p2);
        }

        output.push(' ');
        doneany = true;
    }

    if doneany {
        output.pop();
    }

    output
}

fn func_addsuffix_addprefix(
    output: &mut String,
    argv: &[String],
    funcname: &str,
) -> &mut String {
    let fixlen = argv[0].len();
    let is_addprefix = funcname.chars().nth(3).unwrap() == 'p';
    let is_addsuffix = !is_addprefix;

    let mut list_iter = argv[1].as_str();
    let mut len;
    let mut doneany = false;

    while let Some(p) = find_next_token(&mut list_iter, &mut len) {
        if is_addprefix {
            output.push_str(&argv[0]);
        }
        output.push_str(p);
        if is_addsuffix {
            output.push_str(&argv[0]);
        }
        output.push(' ');
        doneany = true;
    }

    if doneany {
        output.pop();
    }

    output
}

fn func_subst(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    subst_expand(
        output,
        &argv[2],
        &argv[0],
        &argv[1],
        argv[0].len(),
        argv[1].len(),
        false,
    )
}

fn func_firstword(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let mut words = argv[0].as_str();
    let mut len;
    if let Some(p) = find_next_token(&mut words, &mut len) {
        output.push_str(&p[..len]);
    }
    output
}

fn func_lastword(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let mut words = argv[0].as_str();
    let mut len;
    let mut last = None;

    while let Some(p) = find_next_token(&mut words, &mut len) {
        last = Some((p, len));
    }

    if let Some((p, len)) = last {
        output.push_str(&p[..len]);
    }

    output
}

fn func_words(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let mut word_iter = argv[0].as_str();
    let mut count = 0;

    while find_next_token(&mut word_iter, &mut 0).is_some() {
        count += 1;
    }

    output.push_str(&count.to_string());
    output
}

fn strip_whitespace(begpp: &mut &str, endpp: &mut &str) -> &str {
    while begpp <= endpp && begpp.chars().next().unwrap().is_whitespace() {
        *begpp = &begpp[1..];
    }
    while endpp >= begpp && endpp.chars().next().unwrap().is_whitespace() {
        *endpp = &endpp[..endpp.len() - 1];
    }
    *begpp
}

fn parse_numeric(s: &str, msg: &str) -> Result<i64, String> {
    let mut beg = s.trim_start();
    let mut end = s.trim_end();
    if beg.len() > end.len() {
        return Err(format!("{}: empty value", msg));
    }

    match beg.parse::<i64>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("{}: '{}'", msg, s)),
    }
}

fn func_word(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let i = match parse_numeric(&argv[0], "invalid first argument to 'word' function") {
        Ok(val) if val > 0 => val as usize,
        _ => return output,
    };

    let mut end_p = argv[1].as_str();
    let mut p = None;
    let mut count = i;

    while let Some(token) = find_next_token(&mut end_p, &mut 0) {
        count -= 1;
        if count == 0 {
            p = Some(token);
            break;
        }
    }

    if let Some(p) = p {
        output.push_str(p);
    }

    output
}

fn func_wordlist(
    output: &mut String,
    argv: &[String],
    _funcname: &str,
) -> &mut String {
    let start = match parse_numeric(&argv[0], "invalid first argument to 'wordlist' function") {
        Ok(val) if val >= 1 => val as usize,
        Ok(val) => return output,
        Err(e) => return output,
    };

    let stop = match parse_numeric(&argv[1], "invalid second argument to 'wordlist' function") {
        Ok(val) if val >= 0 => val as