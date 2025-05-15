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

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::core::{Conn, Context, Msg, ServerPool};
use crate::log::{log_debug, log_error, log_verbose, LogLevel};
use crate::stats::{Stats, StatsType};
use crate::util::{KeyPos, StringWrapper};

pub fn req_get(conn: &mut Conn, redis: bool) -> Result<Msg, Error> {
    assert!(conn.client && !conn.proxy);

    let msg = Msg::get(conn, true, redis)?;
    Ok(msg)
}

fn req_log(req: &Msg) {
    if !LogLevel::Notice.is_loggable() {
        return;
    }

    if req.frag_id != 0 && req.frag_owner != req {
        return;
    }

    if req.mlen == 0 {
        return;
    }

    if req.start_ts == 0 {
        return;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64;
    let req_time = now - req.start_ts;

    let rsp = req.peer.as_ref();
    let req_len = req.mlen;
    let rsp_len = rsp.map_or(0, |r| r.mlen);

    if req.keys.is_empty() {
        return;
    }

    let kpos = &req.keys[0];

    let peer_str = match req.owner.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "unknown".to_string(),
    };

    let req_type = req.type_string();

    log_debug(
        LogLevel::Notice,
        &format!(
            "req {} done on c {} req_time {}.{:03} msec type {} narg {} req_len {} rsp_len {} key0 '{}' peer '{}' done {} error {}",
            req.id,
            req.owner.fd(),
            req_time / 1000,
            req_time % 1000,
            req_type,
            req.narg,
            req_len,
            rsp_len,
            String::from_utf8_lossy(&kpos.start[..kpos.end - kpos.start]),
            peer_str,
            req.done,
            req.error
        ),
    );
}

pub fn req_put(msg: Msg) {
    assert!(msg.request);

    req_log(&msg);

    if let Some(pmsg) = msg.peer {
        assert!(!pmsg.request && pmsg.peer.as_ref() == Some(&msg));
        drop(pmsg);
    }

    msg.tmo_delete();

    msg.put();
}

pub fn req_done(conn: &Conn, msg: &Msg) -> bool {
    assert!(conn.client && !conn.proxy);
    assert!(msg.request);

    if !msg.done {
        return false;
    }

    let id = msg.frag_id;
    if id == 0 {
        return true;
    }

    if msg.fdone {
        return true;
    }

    if msg.nfrag_done < msg.nfrag {
        return false;
    }

    let mut nfragment = 0;
    let mut all_done = true;

    for cmsg in msg.fragments.iter() {
        if !cmsg.done {
            all_done = false;
            break;
        }
        nfragment += 1;
    }

    if !all_done {
        return false;
    }

    for cmsg in msg.fragments.iter_mut() {
        cmsg.fdone = true;
    }

    assert!(msg.frag_owner.nfrag == nfragment);

    msg.post_coalesce(&msg.frag_owner);

    log_debug(
        LogLevel::Debug,
        &format!(
            "req from c {} with fid {} and {} fragments is done",
            conn.fd(),
            id,
            nfragment
        ),
    );

    true
}

pub fn req_error(conn: &Conn, msg: &Msg) -> bool {
    assert!(msg.request && req_done(conn, msg));

    if msg.error {
        return true;
    }

    let id = msg.frag_id;
    if id == 0 {
        return false;
    }

    if msg.ferror {
        return true;
    }

    let mut has_error = false;
    let mut nfragment = 1;

    for cmsg in msg.fragments.iter() {
        if cmsg.error {
            has_error = true;
            break;
        }
        nfragment += 1;
    }

    if !has_error {
        return false;
    }

    for cmsg in msg.fragments.iter_mut() {
        cmsg.ferror = true;
    }

    log_debug(
        LogLevel::Debug,
        &format!(
            "req from c {} with fid {} and {} fragments is in error",
            conn.fd(),
            id,
            nfragment
        ),
    );

    true
}

pub fn req_server_enqueue_imsgq(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(msg.request);
    assert!(!conn.client && !conn.proxy);

    if !msg.noreply {
        msg.tmo_insert(&msg, conn);
    }

    conn.imsg_q.push_back(msg);

    Stats::incr(ctx, &conn.owner, StatsType::InQueue);
    Stats::incr_by(ctx, &conn.owner, StatsType::InQueueBytes, msg.mlen);
}

pub fn req_server_enqueue_imsgq_head(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(msg.request);
    assert!(!conn.client && !conn.proxy);

    if !msg.noreply {
        msg.tmo_insert(&msg, conn);
    }

    conn.imsg_q.push_front(msg);

    Stats::incr(ctx, &conn.owner, StatsType::InQueue);
    Stats::incr_by(ctx, &conn.owner, StatsType::InQueueBytes, msg.mlen);
}

pub fn req_server_dequeue_imsgq(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(msg.request);
    assert!(!conn.client && !conn.proxy);

    if let Some(index) = conn.imsg_q.iter().position(|m| m.id == msg.id) {
        conn.imsg_q.remove(index);
    }

    Stats::decr(ctx, &conn.owner, StatsType::InQueue);
    Stats::decr_by(ctx, &conn.owner, StatsType::InQueueBytes, msg.mlen);
}

pub fn req_client_enqueue_omsgq(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(msg.request);
    assert!(conn.client && !conn.proxy);

    conn.omsg_q.push_back(msg);
}

pub fn req_server_enqueue_omsgq(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(msg.request);
    assert!(!conn.client && !conn.proxy);

    conn.omsg_q.push_back(msg);

    Stats::incr(ctx, &conn.owner, StatsType::OutQueue);
    Stats::incr_by(ctx, &conn.owner, StatsType::OutQueueBytes, msg.mlen);
}

pub fn req_client_dequeue_omsgq(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(msg.request);
    assert!(conn.client && !conn.proxy);

    if let Some(index) = conn.omsg_q.iter().position(|m| m.id == msg.id) {
        conn.omsg_q.remove(index);
    }
}

pub fn req_server_dequeue_omsgq(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(msg.request);
    assert!(!conn.client && !conn.proxy);

    msg.tmo_delete();

    if let Some(index) = conn.omsg_q.iter().position(|m| m.id == msg.id) {
        conn.omsg_q.remove(index);
    }

    Stats::decr(ctx, &conn.owner, StatsType::OutQueue);
    Stats::decr_by(ctx, &conn.owner, StatsType::OutQueueBytes, msg.mlen);
}

pub fn req_recv_next(ctx: &mut Context, conn: &mut Conn, alloc: bool) -> Option<Msg> {
    assert!(conn.client && !conn.proxy);

    if conn.eof {
        if let Some(msg) = conn.rmsg.take() {
            assert!(msg.peer.is_none());
            assert!(msg.request && !msg.done);

            log_error(
                &format!(
                    "eof c {} discarding incomplete req {} len {}",
                    conn.fd(),
                    msg.id,
                    msg.mlen
                ),
            );

            req_put(msg);
        }

        if !conn.active() {
            conn.done = true;
            log_debug(LogLevel::Info, &format!("c {} is done", conn.fd()));
        }
        return None;
    }

    if let Some(msg) = conn.rmsg.as_ref() {
        assert!(msg.request);
        return Some(msg.clone());
    }

    if !alloc {
        return None;
    }

    if let Ok(msg) = req_get(conn, conn.redis) {
        conn.rmsg = Some(msg.clone());
        Some(msg)
    } else {
        None
    }
}

fn req_make_reply(ctx: &mut Context, conn: &mut Conn, req: Msg) -> Result<(), Error> {
    let rsp = Msg::get(conn, false, conn.redis)?;

    req.peer = Some(rsp.clone());
    rsp.peer = Some(req.clone());
    rsp.request = false;

    req.done = true;
    conn.enqueue_outq(ctx, req);

    Ok(())
}

fn req_filter(conn: &mut Conn, msg: Msg) -> bool {
    assert!(conn.client && !conn.proxy);

    if msg.is_empty() {
        assert!(conn.rmsg.is_none());
        log_debug(
            LogLevel::Verbose,
            &format!("filter empty req {} from c {}", msg.id, conn.fd()),
        );
        req_put(msg);
        return true;
    }

    if msg.quit {
        log_debug(
            LogLevel::Info,
            &format!("filter quit req {} from c {}", msg.id, conn.fd()),
        );
        if let Some(rmsg) = conn.rmsg.as_ref() {
            log_debug(
                LogLevel::Info,
                &format!(
                    "discard invalid req {} len {} from c {} sent after quit req",
                    rmsg.id,
                    rmsg.mlen,
                    conn.fd()
                ),
            );
        }
        conn.eof = true;
        conn.recv_ready = false;
        req_put(msg);
        return true;
    }

    if !conn.authenticated() {
        msg.noforward = true;
    }

    false
}

fn req_forward_error(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(conn.client && !conn.proxy);

    log_debug(
        LogLevel::Info,
        &format!(
            "forward req {} len {} type {} from c {} failed: {}",
            msg.id,
            msg.mlen,
            msg.type_,
            conn.fd(),
            Error::last_os_error()
        ),
    );

    msg.done = true;
    msg.error = true;
    msg.err = Some(Error::last_os_error());

    if msg.noreply {
        req_put(msg);
        return;
    }

    if req_done(conn, conn.omsg_q.front().unwrap()) {
        if let Err(e) = ctx.evb.add_out(conn) {
            conn.err = Some(e);
        }
    }
}

fn req_forward_stats(ctx: &mut Context, server: &ServerPool, msg: &Msg) {
    assert!(msg.request);

    Stats::incr(ctx, server, StatsType::Requests);
    Stats::incr_by(ctx, server, StatsType::RequestBytes, msg.mlen);
}

fn req_forward(ctx: &mut Context, c_conn: &mut Conn, msg: Msg) {
    assert!(c_conn.client && !c_conn.proxy);

    if !msg.noreply {
        c_conn.enqueue_outq(ctx, msg.clone());
    }

    assert!(!msg.keys.is_empty());
    let kpos = &msg.keys[0];
    let key = &kpos.start[..kpos.end - kpos.start];
    let keylen = key.len();

    let s_conn = match ctx.server_pool_conn(c_conn.owner.clone(), key) {
        Some(conn) => conn,
        None => {
            if msg.frag_owner.is_some() {
                msg.frag_owner.as_mut().unwrap().nfrag_done += 1;
            }
            req_forward_error(ctx, c_conn, msg);
            return;
        }
    };

    assert!(!s_conn.client && !s_conn.proxy);

    if s_conn.imsg_q.is_empty() {
        if let Err(e) = ctx.evb.add_out(s_conn) {
            req_forward_error(ctx, c_conn, msg);
            s_conn.err = Some(e);
            return;
        }
    }

    if !s_conn.authenticated() {
        if let Err(e) = msg.add_auth(ctx, c_conn, s_conn) {
            req_forward_error(ctx, c_conn, msg);
            s_conn.err = Some(e);
            return;
        }
    }

    s_conn.enqueue_inq(ctx, msg.clone());

    req_forward_stats(ctx, s_conn.owner.as_ref(), &msg);

    log_debug(
        LogLevel::Verbose,
        &format!(
            "forward from c {} to s {} req {} len {} type {} with key '{}'",
            c_conn.fd(),
            s_conn.fd(),
            msg.id,
            msg.mlen,
            msg.type_,
            String::from_utf8_lossy(key)
        ),
    );
}

pub fn req_recv_done(
    ctx: &mut Context,
    conn: &mut Conn,
    msg: Msg,
    nmsg: Option<Msg>,
) -> Result<(), Error> {
    assert!(conn.client && !conn.proxy);
    assert!(msg.request);
    assert!(msg.owner == *conn);
    assert!(conn.rmsg.as_ref() == Some(&msg));
    assert!(nmsg.is_none() || nmsg.as_ref().unwrap().request);

    conn.rmsg = nmsg;

    if req_filter(conn, msg.clone()) {
        return Ok(());
    }

    if msg.noforward {
        req_make_reply(ctx, conn, msg.clone())?;
        msg.reply()?;
        ctx.evb.add_out(conn)?;
        return Ok(());
    }

    let pool = conn.owner.clone();
    let mut frag_msgq = VecDeque::new();
    msg.fragment(array_n(&pool.server), &mut frag_msgq)?;

    if frag_msgq.is_empty() {
        req_forward(ctx, conn, msg);
        return Ok(());
    }

    req_make_reply(ctx, conn, msg.clone())?;

    for sub_msg in frag_msgq {
        req_forward(ctx, conn, sub_msg);
    }

    Ok(())
}

pub fn req_send_next(ctx: &mut Context, conn: &mut Conn) -> Option<Msg> {
    assert!(!conn.client && !conn.proxy);

    if conn.connecting {
        ctx.server_connected(conn);
    }

    let nmsg = conn.imsg_q.front().cloned();
    if nmsg.is_none() {
        if let Err(e) = ctx.evb.del_out(conn) {
            conn.err = Some(e);
        }
        return None;
    }

    let msg = conn.smsg.as_ref();
    let nmsg = if let Some(msg) = msg {
        assert!(msg.request && !msg.done);
        conn.imsg_q.iter().find(|m| m.id > msg.id).cloned()
    } else {
        nmsg
    };

    conn.smsg = nmsg.clone();

    if let Some(nmsg) = nmsg {
        assert!(nmsg.request && !nmsg.done);
        log_debug(
            LogLevel::VVerbose,
            &format!(
                "send next req {} len {} type {} on s {}",
                nmsg.id,
                nmsg.mlen,
                nmsg.type_,
                conn.fd()
            ),
        );
    }

    nmsg
}

pub fn req_send_done(ctx: &mut Context, conn: &mut Conn, msg: Msg) {
    assert!(!conn.client && !conn.proxy);
    assert!(conn.smsg.is_none());
    assert!(msg.request && !msg.done);
    assert!(msg.owner != *conn);

    log_debug(
        LogLevel::VVerbose,
        &format!(
            "send done req {} len {} type {} on s {}",
            msg.id,
            msg.mlen,
            msg.type_,
            conn.fd()
        ),
    );

    conn.dequeue_inq(ctx, msg.clone());

    if !msg.noreply {
        conn.enqueue_outq(ctx, msg);
    } else {
        req_put(msg);
    }
}