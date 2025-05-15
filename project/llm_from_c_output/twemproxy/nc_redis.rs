/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::mem;
use std::ptr;
use std::cmp;
use std::fmt;
use std::io;
use std::net;
use std::time;
use std::sync::Arc;
use std::collections::HashMap;
use std::os::raw::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::error::Error;
use std::convert::TryFrom;
use std::num::TryFromIntError;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, TcpListener};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use log::{debug, error, info, log, warn, Level};
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

const CR: u8 = b'\r';
const LF: u8 = b'\n';
const CRLF_LEN: usize = 2;

#[derive(Debug, Clone)]
struct String {
    data: Bytes,
    len: usize,
}

impl String {
    fn new(s: &str) -> Self {
        String {
            data: Bytes::from(s),
            len: s.len(),
        }
    }
}

#[derive(Debug, Clone)]
struct KeyPos {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct Msg {
    id: u64,
    type_: MsgType,
    request: bool,
    redis: bool,
    noforward: bool,
    quit: bool,
    error: bool,
    err: i32,
    swallow: bool,
    integer: u64,
    narg: usize,
    narg_start: usize,
    narg_end: usize,
    rnarg: usize,
    rlen: usize,
    vlen: usize,
    pos: usize,
    state: ParseState,
    result: ParseResult,
    token: Option<usize>,
    is_top_level: bool,
    frag_id: u64,
    nfrag: usize,
    nfrag_done: usize,
    frag_owner: Option<Arc<Msg>>,
    frag_seq: Vec<Option<Arc<Msg>>>,
    keys: Vec<KeyPos>,
    mhdr: Vec<Bytes>,
    mlen: usize,
    peer: Option<Arc<Msg>>,
    owner: Option<Arc<dyn Connection>>,
    authenticated: bool,
}

impl Msg {
    fn new(id: u64, request: bool, redis: bool) -> Self {
        Msg {
            id,
            type_: MsgType::Unknown,
            request,
            redis,
            noforward: false,
            quit: false,
            error: false,
            err: 0,
            swallow: false,
            integer: 0,
            narg: 0,
            narg_start: 0,
            narg_end: 0,
            rnarg: 0,
            rlen: 0,
            vlen: 0,
            pos: 0,
            state: ParseState::Start,
            result: ParseResult::Again,
            token: None,
            is_top_level: false,
            frag_id: 0,
            nfrag: 0,
            nfrag_done: 0,
            frag_owner: None,
            frag_seq: Vec::new(),
            keys: Vec::new(),
            mhdr: Vec::new(),
            mlen: 0,
            peer: None,
            owner: None,
            authenticated: false,
        }
    }

    fn gen_frag_id() -> u64 {
        // TODO: Implement proper fragment ID generation
        0
    }

    fn ensure_mbuf(&mut self, len: usize) -> Option<BytesMut> {
        let mut buf = BytesMut::with_capacity(len);
        Some(buf.freeze())
    }

    fn append(&mut self, data: &[u8]) -> io::Result<()> {
        self.mhdr.push(Bytes::copy_from_slice(data));
        self.mlen += data.len();
        Ok(())
    }

    fn prepend_format(&mut self, format: &str, args: &[&dyn fmt::Display]) -> io::Result<()> {
        let s = format!(
            format,
            args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", ")
        );
        self.mhdr.insert(0, Bytes::from(s));
        self.mlen += s.len();
        Ok(())
    }

    fn backend_idx(&self, key: &[u8], keylen: usize) -> usize {
        // TODO: Implement proper backend selection
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MsgType {
    Unknown,
    ReqRedisGet,
    ReqRedisSet,
    ReqRedisDel,
    ReqRedisMget,
    ReqRedisMset,
    ReqRedisPing,
    ReqRedisQuit,
    ReqRedisAuth,
    // Add other message types as needed
    RspRedisStatus,
    RspRedisError,
    RspRedisInteger,
    RspRedisBulk,
    RspRedisMultibulk,
    Sentinel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParseState {
    Start,
    Narg,
    NargLf,
    ReqTypeLen,
    ReqTypeLenLf,
    ReqType,
    ReqTypeLf,
    KeyLen,
    KeyLenLf,
    Key,
    KeyLf,
    Arg1Len,
    Arg1LenLf,
    Arg1,
    Arg1Lf,
    Arg2Len,
    Arg2LenLf,
    Arg2,
    Arg2Lf,
    Arg3Len,
    Arg3LenLf,
    Arg3,
    Arg3Lf,
    ArgNLen,
    ArgNLenLf,
    ArgN,
    ArgNLf,
    Status,
    Error,
    IntegerStart,
    Simple,
    Bulk,
    BulkLf,
    BulkArg,
    BulkArgLf,
    Multibulk,
    MultibulkNargLf,
    MultibulkArgNLen,
    MultibulkArgNLenLf,
    MultibulkArgN,
    MultibulkArgNLf,
    RuntoCrlf,
    AlmostDone,
    Sentinel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParseResult {
    Ok,
    Again,
    Repair,
    Error,
}

trait Connection: Send + Sync {
    fn enqueue_inq(&self, ctx: &Context, conn: &Arc<dyn Connection>, msg: Arc<Msg>);
    fn err(&self) -> i32;
    fn set_err(&self, err: i32);
    fn authenticated(&self) -> bool;
    fn set_authenticated(&self, auth: bool);
}

struct Context {
    // TODO: Add context fields
}

lazy_static! {
    static ref RSP_OK: String = String::new("+OK\r\n");
    static ref RSP_PONG: String = String::new("+PONG\r\n");
    static ref RSP_INVALID_PASSWORD: String = String::new("-ERR invalid password\r\n");
    static ref RSP_AUTH_REQUIRED: String = String::new("-NOAUTH Authentication required\r\n");
    static ref RSP_NO_PASSWORD: String = String::new("-ERR Client sent AUTH, but no password is set\r\n");
}

fn redis_parse_req(r: &mut Msg) {
    // TODO: Implement Redis request parsing
}

fn redis_parse_rsp(r: &mut Msg) {
    // TODO: Implement Redis response parsing
}

fn redis_failure(r: &Msg) -> bool {
    matches!(
        r.type_,
        MsgType::RspRedisError
            | MsgType::RspRedisErrorOom
            | MsgType::RspRedisErrorBusy
            | MsgType::RspRedisErrorLoading
    )
}

fn redis_copy_bulk(dst: Option<&mut Msg>, src: &mut Msg) -> io::Result<()> {
    // TODO: Implement bulk copy
    Ok(())
}

fn redis_pre_coalesce(r: &mut Msg) {
    // TODO: Implement pre-coalesce
}

fn redis_append_key(r: &mut Msg, key: &[u8], keylen: usize) -> io::Result<()> {
    // TODO: Implement key appending
    Ok(())
}

fn redis_fragment_argx(
    r: &mut Msg,
    nserver: usize,
    frag_msgq: &mut Vec<Arc<Msg>>,
    key_step: usize,
) -> io::Result<()> {
    // TODO: Implement argument fragmentation
    Ok(())
}

fn redis_fragment(r: &mut Msg, nserver: usize, frag_msgq: &mut Vec<Arc<Msg>>) -> io::Result<()> {
    if r.keys.len() == 1 {
        return Ok(());
    }

    match r.type_ {
        MsgType::ReqRedisMget
        | MsgType::ReqRedisDel
        | MsgType::ReqRedisTouch
        | MsgType::ReqRedisUnlink => redis_fragment_argx(r, nserver, frag_msgq, 1),
        MsgType::ReqRedisMset => redis_fragment_argx(r, nserver, frag_msgq, 2),
        _ => Ok(()),
    }
}

fn redis_reply(r: &Arc<Msg>) -> io::Result<()> {
    let response = r.peer.as_ref().unwrap();
    let c_conn = response.owner.as_ref().unwrap();

    if r.type_ == MsgType::ReqRedisAuth {
        return redis_handle_auth_req(r, response);
    }

    if !c_conn.authenticated() {
        return response.append(&RSP_AUTH_REQUIRED.data);
    }

    match r.type_ {
        MsgType::ReqRedisPing => response.append(&RSP_PONG.data),
        _ => {
            panic!("Not implemented");
        }
    }
}

fn redis_post_coalesce_mset(request: &Arc<Msg>) {
    let response = request.peer.as_ref().unwrap();
    if let Err(e) = response.append(&RSP_OK.data) {
        response.error = true;
        response.err = e.raw_os_error().unwrap_or(0);
    }
}

fn redis_post_coalesce_del_or_touch(request: &Arc<Msg>) {
    let response = request.peer.as_ref().unwrap();
    if let Err(e) = response.prepend_format(":%d\r\n", &[&request.integer]) {
        response.error = true;
        response.err = e.raw_os_error().unwrap_or(0);
    }
}

fn redis_post_coalesce_mget(request: &Arc<Msg>) {
    let response = request.peer.as_ref().unwrap();
    if let Err(e) = response.prepend_format("*%d\r\n", &[&(request.narg - 1)]) {
        response.owner.as_ref().unwrap().set_err(e.raw_os_error().unwrap_or(0));
        return;
    }

    for i in 0..request.keys.len() {
        let sub_msg = request.frag_seq[i].as_ref().unwrap().peer.as_ref().unwrap();
        if sub_msg.is_none() {
            response.owner.as_ref().unwrap().set_err(1);
            return;
        }
        if let Err(e) = redis_copy_bulk(Some(response), sub_msg) {
            response.owner.as_ref().unwrap().set_err(e.raw_os_error().unwrap_or(0));
            return;
        }
    }
}

fn redis_post_coalesce(r: &Arc<Msg>) {
    let pr = r.peer.as_ref().unwrap();

    assert!(pr.request && (r.frag_owner.as_ref().unwrap() == r));
    if r.error || r.ferror {
        return;
    }

    match r.type_ {
        MsgType::ReqRedisMget => redis_post_coalesce_mget(r),
        MsgType::ReqRedisDel
        | MsgType::ReqRedisTouch
        | MsgType::ReqRedisUnlink => redis_post_coalesce_del_or_touch(r),
        MsgType::ReqRedisMset => redis_post_coalesce_mset(r),
        _ => panic!("Not implemented"),
    }
}

fn redis_handle_auth_req(req: &Arc<Msg>, rsp: &Arc<Msg>) -> io::Result<()> {
    let conn = rsp.owner.as_ref().unwrap();
    let pool = conn.owner.as_ref().unwrap();

    if !pool.require_auth {
        return rsp.append(&RSP_NO_PASSWORD.data);
    }

    let kpos = &req.keys[0];
    let key = &req.mhdr[0][kpos.start..kpos.end];
    let valid = key.len() == pool.redis_auth.len
        && key == pool.redis_auth.data.as_ref();

    if valid {
        conn.set_authenticated(true);
        return rsp.append(&RSP_OK.data);
    }

    conn.set_authenticated(false);
    rsp.append(&RSP_INVALID_PASSWORD.data)
}

fn redis_add_auth(
    ctx: &Context,
    c_conn: &Arc<dyn Connection>,
    s_conn: &Arc<dyn Connection>,
) -> io::Result<()> {
    let pool = c_conn.owner.as_ref().unwrap();
    let mut msg = Msg::new(0, true, true);

    msg.prepend_format(
        "*2\r\n$4\r\nAUTH\r\n$%d\r\n%s\r\n",
        &[&pool.redis_auth.len, &pool.redis_auth.data],
    )?;

    msg.swallow = true;
    s_conn.enqueue_inq(ctx, s_conn, Arc::new(msg));
    s_conn.set_authenticated(true);

    Ok(())
}

fn redis_post_connect(
    ctx: &Context,
    conn: &Arc<dyn Connection>,
    server: &Arc<dyn Connection>,
) {
    let pool = server.owner.as_ref().unwrap();
    if pool.redis_db <= 0 {
        return;
    }

    let mut msg = Msg::new(0, true, true);
    let digits = if pool.redis_db >= 10 {
        (pool.redis_db as f64).log10() as usize + 1
    } else {
        1
    };

    if let Err(e) = msg.prepend_format(
        "*2\r\n$6\r\nSELECT\r\n$%d\r\n%d\r\n",
        &[&digits, &pool.redis_db],
    ) {
        return;
    }

    msg.type_ = MsgType::ReqRedisSelect;
    msg.result = ParseResult::Ok;
    msg.swallow = true;
    msg.owner = None;

    req_server_enqueue_imsgq_head(ctx, conn, Arc::new(msg));
    msg_send(ctx, conn);

    debug!(
        "sent 'SELECT {}' to {} | {}",
        pool.redis_db, pool.name.data, server.name.data
    );
}

fn redis_swallow_msg(
    conn: &Arc<dyn Connection>,
    pmsg: Option<&Arc<Msg>>,
    msg: Option<&Arc<Msg>>,
) {
    if let (Some(pmsg), Some(msg)) = (pmsg, msg) {
        if pmsg.type_ == MsgType::ReqRedisSelect && redis_error(msg) {
            let conn_server = conn.owner.as_ref().unwrap();
            let conn_pool = conn_server.owner.as_ref().unwrap();
            let rsp_buffer = msg.mhdr.last().unwrap();
            let copy_len = cmp::min(rsp_buffer.len() - 3, 127);

            let message = String::from_utf8_lossy(&rsp_buffer[1..copy_len+1]);
            warn!(
                "SELECT {} failed on {} | {}: {}",
                conn_pool.redis_db, conn_pool.name.data, conn_server.name.data, message
            );
        }
    }
}

// Helper functions that need to be implemented
fn req_server_enqueue_imsgq_head(ctx: &Context, conn: &Arc<dyn Connection>, msg: Arc<Msg>) {
    // TODO: Implement
}

fn msg_send(ctx: &Context, conn: &Arc<dyn Connection>) {
    // TODO: Implement
}

fn redis_error(msg: &Arc<Msg>) -> bool {
    // TODO: Implement
    false
}