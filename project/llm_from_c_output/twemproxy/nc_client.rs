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

use std::net::SocketAddr;
use std::os::unix::io::RawFd;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::{Arc, Weak};
use log::{debug, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Connection error")]
    ConnectionError,
}

pub struct Conn {
    pub client: bool,
    pub proxy: bool,
    pub owner: Option<Arc<ServerPool>>,
    pub sd: RawFd,
    pub family: i32,
    pub addrlen: usize,
    pub addr: Option<SocketAddr>,
    pub err: Option<ClientError>,
    pub eof: bool,
    pub rmsg: Option<Arc<Msg>>,
    pub smsg: Option<Arc<Msg>>,
    pub imsg_q: VecDeque<Arc<Msg>>,
    pub omsg_q: VecDeque<Arc<Msg>>,
}

pub struct ServerPool {
    pub name: String,
    pub nc_conn_q: usize,
    pub c_conn_q: VecDeque<Arc<Conn>>,
}

pub struct Msg {
    pub id: u64,
    pub mlen: u32,
    pub msg_type: i32,
    pub request: bool,
    pub done: bool,
    pub error: bool,
    pub swallow: bool,
    pub peer: Option<Weak<Msg>>,
}

pub struct Context {
    // Context fields would be defined here
}

pub fn client_active(conn: &Conn) -> bool {
    debug_assert!(conn.client && !conn.proxy);
    debug_assert!(conn.imsg_q.is_empty());

    if !conn.omsg_q.is_empty() {
        debug!("c {} is active", conn.sd);
        return true;
    }

    if conn.rmsg.is_some() {
        debug!("c {} is active", conn.sd);
        return true;
    }

    if conn.smsg.is_some() {
        debug!("c {} is active", conn.sd);
        return true;
    }

    debug!("c {} is inactive", conn.sd);
    false
}

pub fn client_ref(conn: Arc<Conn>, owner: Arc<ServerPool>) {
    debug_assert!(conn.client && !conn.proxy);
    debug_assert!(conn.owner.is_none());

    // Reset connection address info
    let mut conn = Arc::try_unwrap(conn).unwrap();
    conn.family = 0;
    conn.addrlen = 0;
    conn.addr = None;
    let conn = Arc::new(conn);

    owner.nc_conn_q += 1;
    owner.c_conn_q.push_back(Arc::clone(&conn));

    // Set owner of the client connection
    Arc::get_mut(&mut Arc::clone(&conn)).unwrap().owner = Some(Arc::clone(&owner));

    debug!(
        "ref conn {:?} owner {:?} into pool '{}'",
        conn, owner, owner.name
    );
}

pub fn client_unref(conn: Arc<Conn>) {
    debug_assert!(conn.client && !conn.proxy);
    debug_assert!(conn.owner.is_some());

    let owner = conn.owner.as_ref().unwrap().clone();
    Arc::get_mut(&mut Arc::clone(&conn)).unwrap().owner = None;

    debug_assert!(owner.nc_conn_q != 0);
    owner.nc_conn_q -= 1;
    owner.c_conn_q.retain(|c| !Arc::ptr_eq(c, &conn));

    debug!(
        "unref conn {:?} owner {:?} from pool '{}'",
        conn, owner, owner.name
    );
}

fn client_close_stats(ctx: &Context, pool: &ServerPool, err: Option<ClientError>, eof: bool) {
    // stats_pool_decr(ctx, pool, client_connections);

    if eof {
        // stats_pool_incr(ctx, pool, client_eof);
        return;
    }

    if let Some(err) = err {
        match err {
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::TimedOut => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::ConnectionReset => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::ConnectionAborted => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::NotConnected => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::NetworkDown => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::NetworkUnreachable => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::HostDown => {}
            ClientError::Io(e) if e.kind() == std::io::ErrorKind::HostUnreachable => {}
            _ => {}
        }
        // stats_pool_incr(ctx, pool, client_err);
    }
}

pub fn client_close(ctx: &Context, conn: Arc<Conn>) -> Result<(), ClientError> {
    debug_assert!(conn.client && !conn.proxy);

    client_close_stats(ctx, conn.owner.as_ref().unwrap(), conn.err.clone(), conn.eof);

    if conn.sd < 0 {
        client_unref(Arc::clone(&conn));
        // conn_put(conn);
        return Ok(());
    }

    if let Some(msg) = conn.rmsg.take() {
        debug_assert!(msg.peer.is_none());
        debug_assert!(msg.request && !msg.done);

        debug!(
            "close c {} discarding pending req {} len {} type {}",
            conn.sd, msg.id, msg.mlen, msg.msg_type
        );
        // req_put(msg);
    }

    debug_assert!(conn.smsg.is_none());
    debug_assert!(conn.imsg_q.is_empty());

    while let Some(msg) = conn.omsg_q.pop_front() {
        if msg.done {
            debug!(
                "close c {} discarding {} req {} len {} type {}",
                conn.sd,
                if msg.error { "error" } else { "completed" },
                msg.id,
                msg.mlen,
                msg.msg_type
            );
            // req_put(msg);
        } else {
            let mut msg = Arc::try_unwrap(msg).unwrap();
            msg.swallow = true;

            debug_assert!(msg.request);
            debug_assert!(msg.peer.is_none());

            debug!(
                "close c {} schedule swallow of req {} len {} type {}",
                conn.sd, msg.id, msg.mlen, msg.msg_type
            );
        }
    }

    debug_assert!(conn.omsg_q.is_empty());
    client_unref(Arc::clone(&conn));

    if let Err(e) = unsafe { libc::close(conn.sd) } {
        error!("close c {} failed, ignored: {}", conn.sd, std::io::Error::from_raw_os_error(e));
    }
    Arc::get_mut(&mut Arc::clone(&conn)).unwrap().sd = -1;

    // conn_put(conn);
    Ok(())
}