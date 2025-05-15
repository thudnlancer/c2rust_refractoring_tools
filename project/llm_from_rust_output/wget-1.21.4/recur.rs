use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

#[repr(C)]
pub struct Iri {
    uri_encoding: *mut c_char,
    content_encoding: *mut c_char,
    orig_url: *mut c_char,
    utf8_encode: bool,
}

#[repr(C)]
pub struct Url {
    url: *mut c_char,
    scheme: UrlScheme,
    host: *mut c_char,
    port: c_int,
    path: *mut c_char,
    params: *mut c_char,
    query: *mut c_char,
    fragment: *mut c_char,
    dir: *mut c_char,
    file: *mut c_char,
    user: *mut c_char,
    passwd: *mut c_char,
}

#[repr(u32)]
pub enum UrlScheme {
    Http,
    Https,
    Ftps,
    Ftp,
    Invalid,
}

#[repr(C)]
pub struct UrlPos {
    url: *mut Url,
    local_name: *mut c_char,
    flags: u16,
    refresh_timeout: c_int,
    convert: ConvertOptions,
    pos: c_int,
    size: c_int,
    next: *mut UrlPos,
}

#[repr(u32)]
pub enum ConvertOptions {
    NoConvert,
    ConvertToRelative,
    ConvertBasenameOnly,
    ConvertToComplete,
    NullifyBase,
}

#[repr(C)]
pub struct UrlQueue {
    head: *mut QueueElement,
    tail: *mut QueueElement,
    count: c_int,
    maxcount: c_int,
}

#[repr(C)]
pub struct QueueElement {
    url: *const c_char,
    referer: *const c_char,
    depth: c_int,
    html_allowed: bool,
    iri: *mut Iri,
    css_allowed: bool,
    next: *mut QueueElement,
}

#[repr(u32)]
pub enum RejectReason {
    Success,
    Blacklist,
    NotHttps,
    NonHttp,
    Absolute,
    Domain,
    Parent,
    List,
    Regex,
    Rules,
    SpannedHost,
    Robots,
}

extern "C" {
    fn xcalloc(n: usize, size: usize) -> *mut c_void;
    fn xmalloc(size: usize) -> *mut c_void;
    fn xstrdup(s: *const c_char) -> *mut c_char;
    fn rpl_free(ptr: *mut c_void);
    fn url_parse(
        url: *const c_char,
        err: *mut c_int,
        iri: *mut Iri,
        percent_encode: bool,
    ) -> *mut Url;
    fn url_free(url: *mut Url);
    fn url_escape(url: *const c_char) -> *mut c_char;
    fn url_unescape(url: *mut c_char);
    fn string_set_add(ht: *mut c_void, str: *const c_char);
    fn string_set_contains(ht: *mut c_void, str: *const c_char) -> c_int;
    fn make_string_hash_table(size: c_int) -> *mut c_void;
    fn string_set_free(ht: *mut c_void);
    fn schemes_are_similar_p(a: UrlScheme, b: UrlScheme) -> bool;
    fn accept_domain(url: *mut Url) -> bool;
    fn subdir_p(parent: *const c_char, child: *const c_char) -> bool;
    fn acceptable(file: *const c_char) -> bool;
    fn accept_url(url: *const c_char) -> bool;
    fn accdir(dir: *const c_char) -> bool;
    fn has_html_suffix_p(file: *const c_char) -> bool;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

pub unsafe fn url_queue_new() -> *mut UrlQueue {
    let queue = xcalloc(1, std::mem::size_of::<UrlQueue>()) as *mut UrlQueue;
    queue
}

pub unsafe fn url_queue_delete(queue: *mut UrlQueue) {
    rpl_free(queue as *mut c_void);
}

pub unsafe fn url_enqueue(
    queue: *mut UrlQueue,
    iri: *mut Iri,
    url: *const c_char,
    referer: *const c_char,
    depth: c_int,
    html_allowed: bool,
    css_allowed: bool,
) {
    let qel = xmalloc(std::mem::size_of::<QueueElement>()) as *mut QueueElement;
    (*qel).iri = iri;
    (*qel).url = url;
    (*qel).referer = referer;
    (*qel).depth = depth;
    (*qel).html_allowed = html_allowed;
    (*qel).css_allowed = css_allowed;
    (*qel).next = ptr::null_mut();

    (*queue).count += 1;
    if (*queue).count > (*queue).maxcount {
        (*queue).maxcount = (*queue).count;
    }

    if !(*queue).tail.is_null() {
        (*(*queue).tail).next = qel;
    }
    (*queue).tail = qel;
    if (*queue).head.is_null() {
        (*queue).head = (*queue).tail;
    }
}

pub unsafe fn url_dequeue(
    queue: *mut UrlQueue,
    iri: *mut *mut Iri,
    url: *mut *const c_char,
    referer: *mut *const c_char,
    depth: *mut c_int,
    html_allowed: *mut bool,
    css_allowed: *mut bool,
) -> bool {
    let qel = (*queue).head;
    if qel.is_null() {
        return false;
    }

    (*queue).head = (*qel).next;
    if (*queue).head.is_null() {
        (*queue).tail = ptr::null_mut();
    }

    *iri = (*qel).iri;
    *url = (*qel).url;
    *referer = (*qel).referer;
    *depth = (*qel).depth;
    *html_allowed = (*qel).html_allowed;
    *css_allowed = (*qel).css_allowed;

    (*queue).count -= 1;

    rpl_free(qel as *mut c_void);
    true
}

pub unsafe fn blacklist_add(blacklist: *mut c_void, url: *const c_char) {
    let url_unescaped = xstrdup(url);
    url_unescape(url_unescaped);
    string_set_add(blacklist, url_unescaped);
    rpl_free(url_unescaped as *mut c_void);
}

pub unsafe fn blacklist_contains(blacklist: *mut c_void, url: *const c_char) -> bool {
    let url_unescaped = xstrdup(url);
    url_unescape(url_unescaped);
    let ret = string_set_contains(blacklist, url_unescaped) != 0;
    rpl_free(url_unescaped as *mut c_void);
    ret
}