use std::ptr;
use std::mem;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use regex_syntax::hir::Hir;
use regex_syntax::Parser;
use regex::Regex;
use memchr::{memchr, memrchr};

struct DfaComp {
    kwset: Option<kwset::KWSet>,
    dfa: dfa::DFA,
    patterns: Vec<regex::Regex>,
    kwset_exact_matches: usize,
    begline: bool,
}

impl DfaComp {
    fn new() -> Self {
        DfaComp {
            kwset: None,
            dfa: dfa::DFA::new(),
            patterns: Vec::new(),
            kwset_exact_matches: 0,
            begline: false,
        }
    }
}

fn dfawarn(message: &str) {
    eprintln!("warning: {}", message);
}

fn kwsmusts(dc: &mut DfaComp) {
    if let Some(dm) = dc.dfa.dfamust() {
        dc.kwset = Some(kwset::KWSet::new(false));
        
        if dm.exact {
            dc.kwset_exact_matches += 1;
            let old_len = dm.must.len();
            let new_len = old_len + dm.begline as usize + dm.endline as usize;
            let mut must = Vec::with_capacity(new_len);
            
            if dm.begline {
                must.push(b'\n');
            }
            must.extend_from_slice(dm.must.as_bytes());
            
            if dm.endline {
                must.push(b'\n');
            }
            
            dc.kwset.as_mut().unwrap().kwsincr(&must);
            dc.begline |= dm.begline;
        } else {
            dc.kwset.as_mut().unwrap().kwsincr(dm.must.as_bytes());
        }
        
        dc.kwset.as_mut().unwrap().kwsprep();
    }
}

fn possible_backrefs_in_pattern(keys: &[u8], bs_safe: bool) -> bool {
    let second_backslash = if bs_safe { b'\\' } else { 0xFF };
    let len = keys.len().saturating_sub(1);

    for i in 0..len {
        if keys[i] == b'\\' {
            if (b'1'..=b'9').contains(&keys[i+1]) {
                return true;
            }
            if keys[i+1] == second_backslash {
                if i + 1 == len {
                    break;
                }
            }
        }
    }
    false
}

fn regex_compile(
    dc: &mut DfaComp,
    pattern: &str,
    syntax_bits: u32,
    syntax_only: bool,
) -> Result<(), String> {
    let mut builder = regex::RegexBuilder::new(pattern);
    
    if syntax_only {
        builder.syntax(regex_syntax::ExprBuilder::new()
            .allow_bytes(true)
            .unicode(false)
            .build());
    } else {
        builder.syntax(regex_syntax::ExprBuilder::new()
            .allow_bytes(true)
            .unicode(false)
            .ignore_whitespace(false)
            .build());
    }
    
    match builder.build() {
        Ok(re) => {
            if syntax_only {
                Ok(())
            } else {
                dc.patterns.push(re);
                Ok(())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn gea_compile(
    pattern: &str,
    size: usize,
    syntax_bits: u32,
    exact: bool,
) -> Result<DfaComp, String> {
    let mut dc = DfaComp::new();
    
    if match_icase {
        syntax_bits |= RE_ICASE;
    }
    
    let dfaopts = DFA_CONFUSING_BRACKETS_ERROR | DFA_STRAY_BACKSLASH_WARN | DFA_PLUS_WARN |
        if syntax_bits & RE_CONTEXT_INDEP_OPS != 0 { DFA_STAR_WARN } else { 0 } |
        if eolbyte { 0 } else { DFA_EOL_NUL };
    
    dc.dfa.set_syntax(syntax_bits, dfaopts);
    let bs_safe = !localeinfo.multibyte || localeinfo.using_utf8;
    
    let mut buf = Vec::new();
    let mut lineno = 0;
    let mut compilation_failed = false;
    
    for line in pattern.split('\n') {
        let backref = possible_backrefs_in_pattern(line.as_bytes(), bs_safe);
        
        if backref {
            if !buf.is_empty() {
                if let Err(e) = regex_compile(&mut dc, &String::from_utf8_lossy(&buf), syntax_bits, false) {
                    compilation_failed = true;
                    break;
                }
                buf.clear();
            }
            
            if let Err(e) = regex_compile(&mut dc, line, syntax_bits, !backref) {
                compilation_failed = true;
                break;
            }
            dc.pcount += 1;
        } else {
            buf.extend_from_slice(line.as_bytes());
            buf.push(b'\n');
        }
        
        lineno += 1;
    }
    
    if compilation_failed {
        return Err("Pattern compilation failed".into());
    }
    
    if !buf.is_empty() {
        buf.pop(); // Remove trailing newline
        if exact || !dc.dfa.supported() {
            if let Err(e) = regex_compile(&mut dc, &String::from_utf8_lossy(&buf), syntax_bits, false) {
                return Err(e);
            }
        }
    }
    
    if match_words || match_lines {
        let (beg, end) = if match_lines {
            if syntax_bits & RE_NO_BK_PARENS != 0 {
                ("^(", ")$")
            } else {
                ("^\\(", "\\)$")
            }
        } else {
            if syntax_bits & RE_NO_BK_PARENS != 0 {
                ("(^|[^[:alnum:]_])(", ")([^[:alnum:]_]|$)")
            } else {
                ("\\(^\\|[^[:alnum:]_]\\)\\(", "\\)\\([^[:alnum:]_]\\|$\\)")
            }
        };
        
        let mut new_pattern = String::new();
        new_pattern.push_str(beg);
        new_pattern.push_str(pattern);
        new_pattern.push_str(end);
        
        dc.dfa.parse(&new_pattern);
    } else {
        dc.dfa.parse(pattern);
    }
    
    kwsmusts(&mut dc);
    dc.dfa.compile();
    
    Ok(dc)
}

fn egexecute(
    dc: &DfaComp,
    buf: &[u8],
    size: usize,
    match_size: &mut usize,
    start_ptr: Option<usize>,
) -> isize {
    let eol = eolbyte;
    let buflim = buf.len();
    let mut beg = 0;
    let mut end = 0;
    let mut best_match = buflim;
    let mut best_len = 0;
    
    while end < buflim {
        end = buflim;
        
        if start_ptr.is_none() {
            let mut dfa_beg = beg;
            let mut exact_kwset_match = false;
            
            if let Some(kwset) = &dc.kwset {
                if let Some((offset, kwsm)) = kwset.exec(&buf[beg..], true) {
                    let match_pos = beg + offset;
                    beg = memrchr(eol, &buf[..match_pos]).map_or(0, |p| p + 1);
                    dfa_beg = beg;
                    
                    exact_kwset_match = kwsm.index < dc.kwset_exact_matches;
                    
                    if exact_kwset_match || !dc.dfa.is_fast() ||
                        std::cmp::max(16, match_pos - beg) < (match_pos - beg) / 4
                    {
                        end = memchr(eol, &buf[match_pos..]).map_or(buflim, |p| match_pos + p + 1);
                    } else {
                        let next = beg + 4 * std::cmp::max(16, match_pos - beg);
                        end = memchr(eol, &buf[next..]).map_or(buflim, |p| next + p + 1);
                    }
                    
                    if exact_kwset_match {
                        if !localeinfo.multibyte || localeinfo.using_utf8 {
                            *match_size = end - beg;
                            return beg as isize;
                        }
                        // Handle multibyte case...
                    }
                }
            }
            
            if let Some(superset) = dc.dfa.superset() {
                if !exact_kwset_match {
                    if let Some(next_beg) = superset.exec(&buf[dfa_beg..end], 0) {
                        beg = memrchr(eol, &buf[..dfa_beg + next_beg]).map_or(0, |p| p + 1);
                        end = memchr(eol, &buf[dfa_beg + next_beg..]).map_or(buflim, |p| dfa_beg + next_beg + p + 1);
                    }
                }
            }
            
            if let Some(next_beg) = dc.dfa.exec(&buf[dfa_beg..end], 0) {
                beg = memrchr(eol, &buf[..dfa_beg + next_beg]).map_or(0, |p| p + 1);
                end = memchr(eol, &buf[dfa_beg + next_beg..]).map_or(buflim, |p| dfa_beg + next_beg + p + 1);
                
                *match_size = end - beg;
                return beg as isize;
            }
        } else {
            // Exact match case...
        }
    }
    
    -1
}