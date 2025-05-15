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

use std::os::raw::{c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::{SocketAddr, TcpStream};
use std::io::{Error, ErrorKind, Result};
use std::sync::atomic::{AtomicU32, Ordering};
use libc::{getrlimit, rlimit, RLIMIT_NOFILE};

const NC_OK: i32 = 0;
const NC_ERROR: i32 = -1;
const NC_EAGAIN: i32 = -2;
const NC_ENOMEM: i32 = -3;
const RESERVED_FDS: u32 = 32;

type Rstatus = i32;
type Err = i32;

static CTX_ID: AtomicU32 = AtomicU32::new(0);

struct Array;
struct String;
struct Context;
struct Conn;
struct ConnTqh;
struct Msg;
struct MsgTqh;
struct Server;
struct ServerPool;
struct Mbuf;
struct Mhdr;
struct Conf;
struct Stats;
struct Instance;
struct EventBase;

struct Context {
    id: u32,
    cf: *mut Conf,
    stats: *mut Stats,
    pool: Array,
    evb: *mut EventBase,
    max_timeout: i32,
    timeout: i32,
    max_nfd: u32,
    max_ncconn: u32,
    max_nsconn: u32,
}

struct Instance {
    ctx: *mut Context,
    log_level: i32,
    log_filename: *const i8,
    conf_filename: *const i8,
    stats_port: u16,
    stats_interval: i32,
    stats_addr: *const i8,
    hostname: [i8; 256],
    mbuf_chunk_size: usize,
    pid: i32,
    pid_filename: *const i8,
    pidfile: bool,
}

fn core_calc_connections(ctx: &mut Context) -> Rstatus {
    let mut limit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    unsafe {
        if getrlimit(RLIMIT_NOFILE, &mut limit) < 0 {
            return NC_ERROR;
        }
    }

    ctx.max_nfd = limit.rlim_cur as u32;
    ctx.max_ncconn = ctx.max_nfd - ctx.max_nsconn - RESERVED_FDS;
    NC_OK
}

fn core_ctx_create(nci: &Instance) -> *mut Context {
    let ctx = Box::into_raw(Box::new(Context {
        id: CTX_ID.fetch_add(1, Ordering::SeqCst),
        cf: ptr::null_mut(),
        stats: ptr::null_mut(),
        pool: Array,
        evb: ptr::null_mut(),
        max_timeout: nci.stats_interval,
        timeout: nci.stats_interval,
        max_nfd: 0,
        max_ncconn: 0,
        max_nsconn: 0,
    }));

    unsafe {
        (*ctx).cf = conf_create(nci.conf_filename);
        if (*ctx).cf.is_null() {
            Box::from_raw(ctx);
            return ptr::null_mut();
        }

        if server_pool_init(&mut (*ctx).pool, &(*(*ctx).cf).pool, ctx) != NC_OK {
            conf_destroy((*ctx).cf);
            Box::from_raw(ctx);
            return ptr::null_mut();
        }

        if core_calc_connections(&mut *ctx) != NC_OK {
            server_pool_deinit(&mut (*ctx).pool);
            conf_destroy((*ctx).cf);
            Box::from_raw(ctx);
            return ptr::null_mut();
        }

        (*ctx).stats = stats_create(
            nci.stats_port,
            nci.stats_addr,
            nci.stats_interval,
            nci.hostname.as_ptr(),
            &(*ctx).pool,
        );
        if (*ctx).stats.is_null() {
            server_pool_deinit(&mut (*ctx).pool);
            conf_destroy((*ctx).cf);
            Box::from_raw(ctx);
            return ptr::null_mut();
        }

        (*ctx).evb = event_base_create(EVENT_SIZE, Some(core_core));
        if (*ctx).evb.is_null() {
            stats_destroy((*ctx).stats);
            server_pool_deinit(&mut (*ctx).pool);
            conf_destroy((*ctx).cf);
            Box::from_raw(ctx);
            return ptr::null_mut();
        }

        if server_pool_preconnect(ctx) != NC_OK {
            server_pool_disconnect(ctx);
            event_base_destroy((*ctx).evb);
            stats_destroy((*ctx).stats);
            server_pool_deinit(&mut (*ctx).pool);
            conf_destroy((*ctx).cf);
            Box::from_raw(ctx);
            return ptr::null_mut();
        }

        if proxy_init(ctx) != NC_OK {
            server_pool_disconnect(ctx);
            event_base_destroy((*ctx).evb);
            stats_destroy((*ctx).stats);
            server_pool_deinit(&mut (*ctx).pool);
            conf_destroy((*ctx).cf);
            Box::from_raw(ctx);
            return ptr::null_mut();
        }

        ctx
    }
}

fn core_ctx_destroy(ctx: *mut Context) {
    unsafe {
        proxy_deinit(ctx);
        server_pool_disconnect(ctx);
        event_base_destroy((*ctx).evb);
        stats_destroy((*ctx).stats);
        server_pool_deinit(&mut (*ctx).pool);
        conf_destroy((*ctx).cf);
        Box::from_raw(ctx);
    }
}

#[no_mangle]
pub extern "C" fn core_start(nci: *mut Instance) -> *mut Context {
    unsafe {
        mbuf_init(&*nci);
        msg_init();
        conn_init();

        let ctx = core_ctx_create(&*nci);
        if !ctx.is_null() {
            (*nci).ctx = ctx;
            return ctx;
        }

        conn_deinit();
        msg_deinit();
        mbuf_deinit();

        ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn core_stop(ctx: *mut Context) {
    unsafe {
        conn_deinit();
        msg_deinit();
        mbuf_deinit();
        core_ctx_destroy(ctx);
    }
}

fn core_recv(ctx: *mut Context, conn: *mut Conn) -> Rstatus {
    unsafe {
        let status = ((*conn).recv)(ctx, conn);
        if status != NC_OK {
            // Log error
        }
        status
    }
}

fn core_send(ctx: *mut Context, conn: *mut Conn) -> Rstatus {
    unsafe {
        let status = ((*conn).send)(ctx, conn);
        if status != NC_OK {
            // Log error
        }
        status
    }
}

fn core_close(ctx: *mut Context, conn: *mut Conn) {
    unsafe {
        if (*conn).sd <= 0 {
            return;
        }

        // Log connection close

        let status = event_del_conn((*ctx).evb, conn);
        if status < 0 {
            // Log warning
        }

        ((*conn).close)(ctx, conn);
    }
}

fn core_error(ctx: *mut Context, conn: *mut Conn) {
    unsafe {
        let status = nc_get_soerror((*conn).sd);
        if status < 0 {
            // Log warning
        }
        (*conn).err = Error::last_os_error().raw_os_error().unwrap_or(0);

        core_close(ctx, conn);
    }
}

fn core_timeout(ctx: *mut Context) {
    loop {
        unsafe {
            let msg = msg_tmo_min();
            if msg.is_null() {
                (*ctx).timeout = (*ctx).max_timeout;
                return;
            }

            if (*msg).error || (*msg).done {
                msg_tmo_delete(msg);
                continue;
            }

            let conn = (*msg).tmo_rbe.data;
            let then = (*msg).tmo_rbe.key;

            let now = nc_msec_now();
            if now < then {
                let delta = (then - now) as i32;
                (*ctx).timeout = std::cmp::min(delta, (*ctx).max_timeout);
                return;
            }

            // Log timeout

            msg_tmo_delete(msg);
            (*conn).err = libc::ETIMEDOUT;

            core_close(ctx, conn);
        }
    }
}

#[no_mangle]
pub extern "C" fn core_core(arg: *mut c_void, events: u32) -> Rstatus {
    unsafe {
        let conn = arg as *mut Conn;
        if (*conn).owner.is_null() {
            return NC_OK;
        }

        let ctx = conn_to_ctx(conn);
        (*conn).events = events;

        if events & EVENT_ERR != 0 {
            core_error(ctx, conn);
            return NC_ERROR;
        }

        if events & EVENT_READ != 0 {
            let status = core_recv(ctx, conn);
            if status != NC_OK || (*conn).done || (*conn).err != 0 {
                core_close(ctx, conn);
                return NC_ERROR;
            }
        }

        if events & EVENT_WRITE != 0 {
            let status = core_send(ctx, conn);
            if status != NC_OK || (*conn).done || (*conn).err != 0 {
                core_close(ctx, conn);
                return NC_ERROR;
            }
        }

        NC_OK
    }
}

#[no_mangle]
pub extern "C" fn core_loop(ctx: *mut Context) -> Rstatus {
    unsafe {
        let nsd = event_wait((*ctx).evb, (*ctx).timeout);
        if nsd < 0 {
            return nsd;
        }

        core_timeout(ctx);
        stats_swap((*ctx).stats);

        NC_OK
    }
}