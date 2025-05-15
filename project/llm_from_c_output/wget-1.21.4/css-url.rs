use std::fs;
use std::path::Path;
use std::io::{self, Read};
use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::str;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

struct MapContext {
    text: String,
    head: Option<UrlPos>,
    base: Option<String>,
    parent_base: Option<String>,
    document_file: String,
    nofollow: bool,
}

struct UrlPos {
    uri: String,
    pos: usize,
    length: usize,
    link_inline_p: bool,
    link_css_p: bool,
    link_expect_css: bool,
    next: Option<Box<UrlPos>>,
}

fn get_uri_string(at: &str, pos: &mut usize, length: &mut usize) -> Option<String> {
    if *length < 4 {
        return None;
    }

    if !at[*pos..].to_ascii_lowercase().starts_with("url(") {
        return None;
    }

    *pos += 4;
    *length -= 5;

    while *length > 0 && at[*pos..].chars().next().unwrap().is_whitespace() {
        *pos += 1;
        *length -= 1;
        if *length == 0 {
            return None;
        }
    }

    while *length > 0 && at[*pos + *length - 1..].chars().next().unwrap().is_whitespace() {
        *length -= 1;
    }

    if *length >= 2 && (at[*pos..].starts_with('\'') || at[*pos..].starts_with('"')) {
        *pos += 1;
        *length -= 2;
    }

    if *length <= 0 {
        return None;
    }

    Some(at[*pos..*pos + *length].to_string())
}

fn get_urls_css(ctx: &mut MapContext, offset: usize, buf_length: usize) {
    let text = &ctx.text[offset..offset + buf_length];
    let mut buffer_pos = 0;
    let import_re = Regex::new(r"(?i)@import\s+(?:url\()?['\"]?([^'\")]+)['\"]?\)?").unwrap();
    let url_re = Regex::new(r"(?i)url\(['\"]?([^'\")]+)['\"]?\)").unwrap();

    for cap in import_re.captures_iter(text) {
        let uri = cap.get(1).map(|m| m.as_str().to_string());
        if let Some(uri) = uri {
            let pos = buffer_pos + offset + cap.get(0).unwrap().start();
            let length = cap.get(0).unwrap().len();
            let up = append_url(&uri, pos, length, ctx);
            if let Some(up) = up {
                up.link_inline_p = true;
                up.link_css_p = true;
                up.link_expect_css = true;
            }
        }
        buffer_pos += cap.get(0).unwrap().end();
    }

    for cap in url_re.captures_iter(text) {
        let uri = cap.get(1).map(|m| m.as_str().to_string());
        if let Some(uri) = uri {
            let pos = buffer_pos + offset + cap.get(0).unwrap().start();
            let length = cap.get(0).unwrap().len();
            let up = append_url(&uri, pos, length, ctx);
            if let Some(up) = up {
                up.link_inline_p = true;
                up.link_css_p = true;
            }
        }
        buffer_pos += cap.get(0).unwrap().end();
    }
}

fn append_url(uri: &str, pos: usize, length: usize, ctx: &mut MapContext) -> Option<&mut UrlPos> {
    let new_up = UrlPos {
        uri: uri.to_string(),
        pos,
        length,
        link_inline_p: false,
        link_css_p: false,
        link_expect_css: false,
        next: None,
    };

    let mut current = &mut ctx.head;
    while let Some(ref mut up) = current {
        current = &mut up.next;
    }
    *current = Some(Box::new(new_up));
    current.as_mut().map(|up| &mut **up)
}

fn get_urls_css_file(file: &str, url: Option<&str>) -> Result<Option<UrlPos>, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let mut ctx = MapContext {
        text: content,
        head: None,
        base: None,
        parent_base: url.map(|s| s.to_string()),
        document_file: file.to_string(),
        nofollow: false,
    };

    get_urls_css(&mut ctx, 0, ctx.text.len());
    Ok(ctx.head)
}