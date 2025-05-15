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

use std::os::unix::io::{AsRawFd, RawFd};
use std::net::{SocketAddr, TcpListener, UnixListener};
use std::path::Path;
use std::fs;
use std::io::{self, ErrorKind};
use libc::{self, c_int};
use nix::sys::socket::{self, SockAddr};
use nix::unistd;
use log::{debug, error, warn};

use crate::nc_core::*;
use crate::nc_server::*;

pub fn proxy_ref(conn: &mut Conn, owner: &mut ServerPool) {
    assert!(!conn.client && conn.proxy);
    assert!(conn.owner.is_none());

    conn.family = owner.info.family;
    conn.addrlen = owner.info.addrlen;
    conn.addr = Some(owner.info.addr.clone());

    owner.p_conn = Some(conn.clone());

    conn.owner = Some(owner.clone());

    debug!(target: LOG_VVERB, "ref conn {:?} owner {:?} into pool {}", conn, owner, owner.idx);
}

pub fn proxy_unref(conn: &mut Conn) {
    assert!(!conn.client && conn.proxy);
    assert!(conn.owner.is_some());

    let pool = conn.owner.take().unwrap();
    pool.p_conn = None;

    debug!(target: LOG_VVERB, "unref conn {:?} owner {:?} from pool {}", conn, pool, pool.idx);
}

pub fn proxy_close(ctx: &Context, conn: &mut Conn) -> Result<(), io::Error> {
    assert!(!conn.client && conn.proxy);

    if conn.sd < 0 {
        conn.unref();
        conn_put(conn);
        return Ok(());
    }

    assert!(conn.rmsg.is_none());
    assert!(conn.smsg.is_none());
    assert!(conn.imsg_q.is_empty());
    assert!(conn.omsg_q.is_empty());

    conn.unref();

    let res = unistd::close(conn.sd);
    if let Err(e) = res {
        error!("close p {} failed, ignored: {}", conn.sd, e);
    }
    conn.sd = -1;

    conn_put(conn);
    Ok(())
}

fn proxy_reuse(p: &Conn) -> Result<(), io::Error> {
    match p.family {
        libc::AF_INET | libc::AF_INET6 => {
            nc_set_reuseaddr(p.sd)?;
        }
        libc::AF_UNIX => {
            if let Some(SockAddr::Unix(unix_addr)) = &p.addr {
                let path = unix_addr.path().unwrap_or_default();
                let _ = fs::remove_file(path);
            }
        }
        _ => {
            panic!("Unreachable");
        }
    }
    Ok(())
}

fn proxy_listen(ctx: &Context, p: &mut Conn) -> Result<(), io::Error> {
    let pool = p.owner.as_ref().unwrap();
    assert!(p.proxy);

    p.sd = socket::socket(
        p.family,
        socket::SockType::Stream,
        socket::SockFlag::empty(),
        None,
    )?;

    proxy_reuse(p)?;

    if pool.reuseport {
        nc_set_reuseport(p.sd)?;
    }

    let addr = p.addr.as_ref().unwrap();
    socket::bind(p.sd, addr)?;

    if p.family == libc::AF_UNIX && pool.perm != 0 {
        if let Some(SockAddr::Unix(unix_addr)) = &p.addr {
            let path = unix_addr.path().unwrap_or_default();
            fs::set_permissions(path, fs::Permissions::from_mode(pool.perm))?;
        }
    }

    socket::listen(p.sd, pool.backlog as i32)?;
    nc_set_nonblocking(p.sd)?;

    event_add_conn(ctx.evb, p)?;
    event_del_out(ctx.evb, p)?;

    Ok(())
}

pub fn proxy_each_init(elem: &mut ServerPool, _data: Option<&mut ()>) -> Result<(), io::Error> {
    let mut p = conn_get_proxy(elem)?;
    
    proxy_listen(&elem.ctx, &mut p)?;

    debug!(target: LOG_NOTICE, "p {} listening on '{}' in {} pool {} '{}' with {} servers", 
        p.sd, pool.addrstr, if pool.redis { "redis" } else { "memcache" },
        pool.idx, pool.name, pool.server.len());

    Ok(())
}

pub fn proxy_init(ctx: &mut Context) -> Result<(), io::Error> {
    assert!(!ctx.pool.is_empty());

    for pool in &mut ctx.pool {
        proxy_each_init(pool, None)?;
    }

    debug!(target: LOG_VVERB, "init proxy with {} pools", ctx.pool.len());
    Ok(())
}

pub fn proxy_each_deinit(elem: &mut ServerPool, _data: Option<&mut ()>) -> Result<(), io::Error> {
    if let Some(mut p) = elem.p_conn.take() {
        p.close(&elem.ctx, &mut p)?;
    }
    Ok(())
}

pub fn proxy_deinit(ctx: &mut Context) -> Result<(), io::Error> {
    assert!(!ctx.pool.is_empty());

    for pool in &mut ctx.pool {
        proxy_each_deinit(pool, None)?;
    }

    debug!(target: LOG_VVERB, "deinit proxy with {} pools", ctx.pool.len());
    Ok(())
}

fn proxy_accept(ctx: &Context, p: &mut Conn) -> Result<(), io::Error> {
    let pool = p.owner.as_ref().unwrap();
    assert!(p.proxy && !p.client);
    assert!(p.sd > 0);
    assert!(p.recv_active && p.recv_ready);

    loop {
        let res = socket::accept(p.sd);
        match res {
            Ok((sd, _)) => {
                if conn_ncurr_cconn() >= ctx.max_ncconn {
                    debug!(target: LOG_CRIT, "client connections {} exceed limit {}", 
                        conn_ncurr_cconn(), ctx.max_ncconn);
                    let _ = unistd::close(sd);
                    return Ok(());
                }

                let mut c = conn_get(&pool, true, p.redis)?;
                c.sd = sd;

                stats_pool_incr(ctx, &c.owner, "client_connections");

                nc_set_nonblocking(c.sd)?;

                if pool.tcpkeepalive {
                    if let Err(e) = nc_set_tcpkeepalive(c.sd) {
                        warn!("set tcpkeepalive on c {} from p {} failed, ignored: {}", 
                            c.sd, p.sd, e);
                    }
                }

                if p.family == libc::AF_INET || p.family == libc::AF_INET6 {
                    if let Err(e) = nc_set_tcpnodelay(c.sd) {
                        warn!("set tcpnodelay on c {} from p {} failed, ignored: {}", 
                            c.sd, p.sd, e);
                    }
                }

                event_add_conn(ctx.evb, &mut c)?;

                debug!(target: LOG_NOTICE, "accepted c {} on p {} from '{}'", 
                    c.sd, p.sd, nc_unresolve_peer_desc(c.sd));

                return Ok(());
            }
            Err(e) => match e {
                nix::Error::EINTR => continue,
                nix::Error::EAGAIN | nix::Error::EWOULDBLOCK | nix::Error::ECONNABORTED => {
                    p.recv_ready = false;
                    return Ok(());
                }
                nix::Error::EMFILE | nix::Error::ENFILE => {
                    debug!(target: LOG_CRIT, 
                        "accept on p {} with max fds {} used connections {} max client connections {} curr client connections {} failed: {}", 
                        p.sd, ctx.max_nfd, conn_ncurr_conn(), ctx.max_ncconn, conn_ncurr_cconn(), e);
                    p.recv_ready = false;
                    return Ok(());
                }
                _ => {
                    error!("accept on p {} failed: {}", p.sd, e);
                    return Err(e.into());
                }
            },
        }
    }
}

pub fn proxy_recv(ctx: &Context, conn: &mut Conn) -> Result<(), io::Error> {
    assert!(conn.proxy && !conn.client);
    assert!(conn.recv_active);

    conn.recv_ready = true;
    while conn.recv_ready {
        proxy_accept(ctx, conn)?;
    }

    Ok(())
}