use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;
use std::fmt::Write;
use std::slice;
use hiredis::{RedisReply, RedisReplyType, RedisAsyncContext};
use libc::{free, malloc, memcpy};

mod cmd;
mod common;
mod http;

use cmd::Cmd;
use http::{HttpResponse, HeaderCopy};

extern "C" {
    fn format_send_error(cmd: *mut Cmd, code: i32, msg: *const u8);
    fn format_send_reply(cmd: *mut Cmd, buf: *const u8, len: usize, mime: *const u8);
    fn cmd_is_subscribe(cmd: *const Cmd) -> bool;
    fn cmd_free(cmd: *mut Cmd);
    fn http_response_init(w: *mut c_void, code: i32, msg: *const u8) -> *mut HttpResponse;
    fn http_response_set_header(
        resp: *mut HttpResponse,
        name: *const u8,
        value: *const u8,
        copy: HeaderCopy,
    );
    fn http_response_set_keep_alive(resp: *mut HttpResponse, keep_alive: bool);
    fn http_response_write(resp: *mut HttpResponse, fd: i32);
}

pub unsafe extern "C" fn custom_type_reply(
    c: *mut RedisAsyncContext,
    r: *mut c_void,
    privdata: *mut c_void,
) {
    let reply = r as *mut RedisReply;
    let cmd = privdata as *mut Cmd;

    if reply.is_null() {
        format_send_error(cmd, 503, b"Service Unavailable\0".as_ptr());
        return;
    }

    let reply = &*reply;
    let cmd_ref = &*cmd;

    if !cmd_ref.mime.is_null() {
        match reply.type_ {
            RedisReplyType::Nil => {
                format_send_error(cmd, 404, b"Not found\0".as_ptr());
                return;
            }
            RedisReplyType::String => {
                format_send_reply(
                    cmd,
                    reply.str.as_ptr(),
                    reply.len,
                    cmd_ref.mime,
                );
                return;
            }
            RedisReplyType::Status | RedisReplyType::Error => {
                let mut status_buf = Vec::with_capacity(1 + reply.len);
                status_buf.push(if reply.type_ == RedisReplyType::Status { b'+' } else { b'-' });
                status_buf.extend_from_slice(unsafe {
                    slice::from_raw_parts(reply.str.as_ptr(), reply.len)
                });
                format_send_reply(
                    cmd,
                    status_buf.as_ptr(),
                    status_buf.len(),
                    cmd_ref.mime,
                );
                return;
            }
            RedisReplyType::Integer => {
                let mut int_buffer = [0u8; 50];
                let int_len = write!(&mut int_buffer[..], "{}", reply.integer).unwrap();
                format_send_reply(
                    cmd,
                    int_buffer.as_ptr(),
                    int_len,
                    cmd_ref.mime,
                );
                return;
            }
            RedisReplyType::Array => {
                let mut sz = 0;
                let array_out = custom_array(cmd, reply, &mut sz);
                format_send_reply(cmd, array_out, sz, cmd_ref.mime);
                free(array_out as *mut _);
                return;
            }
            _ => {}
        }
    }

    let resp = http_response_init(cmd_ref.w, 400, b"Bad Request\0".as_ptr());
    http_response_set_header(resp, b"Content-Length\0".as_ptr(), b"0\0".as_ptr(), HeaderCopy::None);
    http_response_set_keep_alive(resp, cmd_ref.keep_alive);
    http_response_write(resp, cmd_ref.fd);

    if cmd_is_subscribe(cmd) == 0 {
        cmd_free(cmd);
    }
}

unsafe fn custom_array(cmd: *mut Cmd, r: *const RedisReply, sz: &mut usize) -> *mut u8 {
    let cmd_ref = &*cmd;
    let reply = &*r;
    let mut sep_len = 0;

    if !cmd_ref.separator.is_null() {
        sep_len = CStr::from_ptr(cmd_ref.separator).to_bytes().len();
    }

    *sz = 0;
    for i in 0..reply.elements {
        let e = reply.element[i as usize];
        match (*e).type_ {
            RedisReplyType::String => {
                if sep_len != 0 && i != 0 {
                    *sz += sep_len;
                }
                *sz += (*e).len;
            }
            _ => {}
        }
    }

    let ret = malloc(*sz) as *mut u8;
    let mut p = ret;

    for i in 0..reply.elements {
        let e = reply.element[i as usize];
        match (*e).type_ {
            RedisReplyType::String => {
                if sep_len != 0 && i != 0 {
                    memcpy(
                        p as *mut _,
                        cmd_ref.separator as *const _,
                        sep_len,
                    );
                    p = p.add(sep_len);
                }
                memcpy(
                    p as *mut _,
                    (*e).str as *const _,
                    (*e).len,
                );
                p = p.add((*e).len);
            }
            _ => {}
        }
    }

    ret
}