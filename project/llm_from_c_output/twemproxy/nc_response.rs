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

use std::error::Error as StdError;
use std::fmt;
use std::io::{Error as IoError, ErrorKind};
use std::ptr::NonNull;

use crate::nc_core::*;
use crate::nc_server::*;

#[derive(Debug)]
pub enum RspError {
    Io(IoError),
    InvalidState,
    PeerNotFound,
}

impl fmt::Display for RspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RspError::Io(e) => write!(f, "IO error: {}", e),
            RspError::InvalidState => write!(f, "Invalid state"),
            RspError::PeerNotFound => write!(f, "Peer message not found"),
        }
    }
}

impl StdError for RspError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            RspError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<IoError> for RspError {
    fn from(err: IoError) -> Self {
        RspError::Io(err)
    }
}

pub fn rsp_get(conn: &mut Conn) -> Result<Option<Msg>, RspError> {
    assert!(!conn.client && !conn.proxy);

    let msg = msg_get(conn, false, conn.redis)?;
    if msg.is_none() {
        conn.err = Some(IoError::last_os_error());
    }

    Ok(msg)
}

pub fn rsp_put(msg: Msg) {
    assert!(!msg.request);
    assert!(msg.peer.is_none());
    msg_put(msg);
}

fn rsp_make_error(ctx: &mut Context, conn: &mut Conn, msg: Msg) -> Result<Msg, RspError> {
    assert!(conn.client && !conn.proxy);
    assert!(msg.request && req_error(conn, &msg));
    assert!(msg.owner == conn);

    let id = msg.frag_id;
    let err = if id != 0 {
        let mut err = None;
        let mut cmsg = msg.c_tqe.next;
        
        while let Some(mut current) = cmsg {
            let nmsg = current.c_tqe.next;
            
            conn.dequeue_outq(ctx, current)?;
            if err.is_none() && current.err.is_some() {
                err = current.err;
            }

            req_put(current);
            cmsg = nmsg;
        }
        err.unwrap_or_else(|| IoError::new(ErrorKind::Other, "No error found in fragments"))
    } else {
        msg.err.ok_or(RspError::InvalidState)?
    };

    if let Some(pmsg) = msg.peer {
        assert!(!pmsg.request && pmsg.peer == Some(msg));
        msg_get_error(conn.redis, err)
    } else {
        Err(RspError::PeerNotFound)
    }
}

pub fn rsp_recv_next(ctx: &mut Context, conn: &mut Conn, alloc: bool) -> Result<Option<Msg>, RspError> {
    assert!(!conn.client && !conn.proxy);

    if conn.eof {
        if let Some(msg) = conn.rmsg.take() {
            assert!(msg.peer.is_none());
            assert!(!msg.request);

            log_error(format!("eof s {} discarding incomplete rsp {} len {}", 
                conn.sd, msg.id, msg.mlen));
            rsp_put(msg);
        }

        conn.done = true;
        log_error(format!("s {} active {} is done", conn.sd, conn.active(conn)));
        return Ok(None);
    }

    if let Some(msg) = conn.rmsg.as_ref() {
        assert!(!msg.request);
        return Ok(Some(msg.clone()));
    }

    if !alloc {
        return Ok(None);
    }

    let msg = rsp_get(conn)?;
    if let Some(msg) = msg {
        conn.rmsg = Some(msg.clone());
        Ok(Some(msg))
    } else {
        Ok(None)
    }
}

fn rsp_filter(ctx: &mut Context, conn: &mut Conn, msg: &Msg) -> Result<bool, RspError> {
    assert!(!conn.client && !conn.proxy);

    if msg_empty(msg) {
        assert!(conn.rmsg.is_none());
        log_debug(LOG_VERB, format!("filter empty rsp {} on s {}", msg.id, conn.sd));
        rsp_put(msg.clone());
        return Ok(true);
    }

    if let Some(pmsg) = conn.omsg_q.first() {
        if msg.failure() {
            log_debug(LOG_INFO, format!("server failure rsp {} len {} type {} on s {}", 
                msg.id, msg.mlen, msg.type_, conn.sd));
            rsp_put(msg.clone());

            conn.err = Some(IoError::new(ErrorKind::InvalidInput, "Invalid response"));
            conn.done = true;
            return Ok(true);
        }

        if pmsg.swallow {
            conn.swallow_msg(pmsg, msg);

            conn.dequeue_outq(ctx, pmsg)?;
            pmsg.done = true;

            log_debug(LOG_INFO, format!("swallow rsp {} len {} of req {} on s {}", 
                msg.id, msg.mlen, pmsg.id, conn.sd));

            rsp_put(msg.clone());
            req_put(pmsg.clone());
            return Ok(true);
        }

        Ok(false)
    } else {
        log_debug(LOG_ERR, format!("filter stray rsp {} len {} on s {}", 
            msg.id, msg.mlen, conn.sd));
        rsp_put(msg.clone());

        conn.err = Some(IoError::new(ErrorKind::InvalidInput, "Stray response"));
        conn.done = true;
        Ok(true)
    }
}

fn rsp_forward_stats(ctx: &mut Context, server: &mut Server, msg: &Msg, msgsize: u32) {
    assert!(!msg.request);

    stats_server_incr(ctx, server, StatsType::Responses);
    stats_server_incr_by(ctx, server, StatsType::ResponseBytes, msgsize);
}

fn rsp_forward(ctx: &mut Context, s_conn: &mut Conn, msg: Msg) -> Result<(), RspError> {
    assert!(!s_conn.client && !s_conn.proxy);
    let msgsize = msg.mlen;

    server_ok(ctx, s_conn);

    let pmsg = s_conn.omsg_q.first_mut().ok_or(RspError::PeerNotFound)?;
    assert!(pmsg.peer.is_none());
    assert!(pmsg.request && !pmsg.done);

    s_conn.dequeue_outq(ctx, pmsg)?;
    pmsg.done = true;

    pmsg.peer = Some(msg.clone());
    msg.peer = Some(pmsg.clone());

    msg.pre_coalesce();

    let c_conn = pmsg.owner.as_mut().ok_or(RspError::InvalidState)?;
    assert!(c_conn.client && !c_conn.proxy);

    if req_done(c_conn, c_conn.omsg_q.first()) {
        if event_add_out(ctx.evb, c_conn).is_err() {
            c_conn.err = Some(IoError::last_os_error());
        }
    }

    rsp_forward_stats(ctx, s_conn.owner.as_mut().ok_or(RspError::InvalidState)?, &msg, msgsize);
    Ok(())
}

pub fn rsp_recv_done(ctx: &mut Context, conn: &mut Conn, msg: Msg, nmsg: Option<Msg>) -> Result<(), RspError> {
    assert!(!conn.client && !conn.proxy);
    assert!(conn.rmsg.as_ref() == Some(&msg));
    assert!(!msg.request);
    assert!(msg.owner == conn);
    assert!(nmsg.as_ref().map_or(true, |m| !m.request));

    conn.rmsg = nmsg;

    if rsp_filter(ctx, conn, &msg)? {
        return Ok(());
    }

    rsp_forward(ctx, conn, msg)
}

pub fn rsp_send_next(ctx: &mut Context, conn: &mut Conn) -> Result<Option<Msg>, RspError> {
    assert!(conn.client && !conn.proxy);

    let pmsg = conn.omsg_q.first();
    if pmsg.is_none() || !req_done(conn, pmsg) {
        if pmsg.is_none() && conn.eof {
            conn.done = true;
            log_debug(LOG_INFO, format!("c {} is done", conn.sd));
        }

        if event_del_out(ctx.evb, conn).is_err() {
            conn.err = Some(IoError::last_os_error());
        }
        return Ok(None);
    }

    let msg = conn.smsg.as_ref();
    let pmsg = if let Some(msg) = msg {
        assert!(!msg.request && msg.peer.is_some());
        assert!(req_done(conn, msg.peer.as_ref().unwrap()));
        msg.peer.as_ref().unwrap().c_tqe.next
    } else {
        None
    };

    if pmsg.is_none() || !req_done(conn, pmsg) {
        conn.smsg = None;
        return Ok(None);
    }
    assert!(pmsg.unwrap().request && !pmsg.unwrap().swallow);

    let msg = if req_error(conn, pmsg.unwrap()) {
        let msg = rsp_make_error(ctx, conn, pmsg.unwrap().clone())?;
        msg.peer = Some(pmsg.unwrap().clone());
        pmsg.unwrap().peer = Some(msg.clone());
        stats_pool_incr(ctx, conn.owner.as_mut().unwrap(), StatsType::ForwardError);
        msg
    } else {
        pmsg.unwrap().peer.as_ref().unwrap().clone()
    };
    assert!(!msg.request);

    conn.smsg = Some(msg.clone());

    log_debug(LOG_VVERB, format!("send next rsp {} on c {}", msg.id, conn.sd));
    Ok(Some(msg))
}

pub fn rsp_send_done(ctx: &mut Context, conn: &mut Conn, msg: Msg) -> Result<(), RspError> {
    assert!(conn.client && !conn.proxy);
    assert!(conn.smsg.is_none());

    log_debug(LOG_VVERB, format!("send done rsp {} on c {}", msg.id, conn.sd));

    let pmsg = msg.peer.as_ref().ok_or(RspError::PeerNotFound)?;
    assert!(!msg.request && pmsg.request);
    assert!(pmsg.peer.as_ref() == Some(&msg));
    assert!(pmsg.done && !pmsg.swallow);

    conn.dequeue_outq(ctx, pmsg)?;
    req_put(pmsg.clone());
    Ok(())
}