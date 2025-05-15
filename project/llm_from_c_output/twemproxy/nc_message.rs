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
use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::core::{Conn, Context, Error, Result};
use crate::mbuf::Mbuf;
use crate::protocol::{MemcacheParser, RedisParser};
use crate::server::ServerPool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgParseResult {
    Ok,
    Error,
    Repair,
    Again,
}

pub type MsgParseFn = fn(&mut Msg) -> ();
pub type MsgAddAuthFn = fn(&Context, &Conn, &Conn) -> Result<()>;
pub type MsgFragmentFn = fn(&mut Msg, u32, &mut VecDeque<Msg>) -> Result<()>;
pub type MsgCoalesceFn = fn(&mut Msg) -> ();
pub type MsgReplyFn = fn(&mut Msg) -> Result<()>;
pub type MsgFailureFn = fn(&Msg) -> bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    Unknown,
    ReqMcGet,
    ReqMcGets,
    ReqMcDelete,
    ReqMcCas,
    ReqMcSet,
    ReqMcAdd,
    ReqMcReplace,
    ReqMcAppend,
    ReqMcPrepend,
    ReqMcIncr,
    ReqMcDecr,
    ReqMcTouch,
    ReqMcQuit,
    ReqMcVersion,
    RspMcNum,
    RspMcStored,
    RspMcNotStored,
    RspMcExists,
    RspMcNotFound,
    RspMcEnd,
    RspMcValue,
    RspMcDeleted,
    RspMcTouched,
    RspMcVersion,
    RspMcError,
    RspMcClientError,
    RspMcServerError,
    ReqRedisCopy,
    ReqRedisDel,
    ReqRedisExists,
    ReqRedisExpire,
    ReqRedisExpireAt,
    ReqRedisMove,
    ReqRedisPexpire,
    ReqRedisPexpireAt,
    ReqRedisPersist,
    ReqRedisPttl,
    ReqRedisSort,
    ReqRedisTouch,
    ReqRedisTtl,
    ReqRedisType,
    ReqRedisUnlink,
    ReqRedisAppend,
    ReqRedisBitcount,
    ReqRedisBitfield,
    ReqRedisBitpos,
    ReqRedisDecr,
    ReqRedisDecrBy,
    ReqRedisDump,
    ReqRedisGet,
    ReqRedisGetbit,
    ReqRedisGetdel,
    ReqRedisGetex,
    ReqRedisGetrange,
    ReqRedisGetset,
    ReqRedisIncr,
    ReqRedisIncrBy,
    ReqRedisIncrByFloat,
    ReqRedisMget,
    ReqRedisMset,
    ReqRedisPsetex,
    ReqRedisRestore,
    ReqRedisSet,
    ReqRedisSetbit,
    ReqRedisSetex,
    ReqRedisSetnx,
    ReqRedisSetrange,
    ReqRedisStrlen,
    ReqRedisHdel,
    ReqRedisHexists,
    ReqRedisHget,
    ReqRedisHgetall,
    ReqRedisHincrby,
    ReqRedisHincrbyFloat,
    ReqRedisHkeys,
    ReqRedisHlen,
    ReqRedisHmget,
    ReqRedisHmset,
    ReqRedisHrandfield,
    ReqRedisHset,
    ReqRedisHsetnx,
    ReqRedisHscan,
    ReqRedisHstrlen,
    ReqRedisHvals,
    ReqRedisLindex,
    ReqRedisLinsert,
    ReqRedisLlen,
    ReqRedisLmove,
    ReqRedisLpop,
    ReqRedisLpos,
    ReqRedisLpush,
    ReqRedisLpushx,
    ReqRedisLrange,
    ReqRedisLrem,
    ReqRedisLset,
    ReqRedisLtrim,
    ReqRedisPfadd,
    ReqRedisPfcount,
    ReqRedisPfmerge,
    ReqRedisRpop,
    ReqRedisRpoplpush,
    ReqRedisRpush,
    ReqRedisRpushx,
    ReqRedisSadd,
    ReqRedisScard,
    ReqRedisSdiff,
    ReqRedisSdiffstore,
    ReqRedisSinter,
    ReqRedisSinterstore,
    ReqRedisSismember,
    ReqRedisSmismember,
    ReqRedisSmembers,
    ReqRedisSmove,
    ReqRedisSpop,
    ReqRedisSrandmember,
    ReqRedisSrem,
    ReqRedisSunion,
    ReqRedisSunionstore,
    ReqRedisSscan,
    ReqRedisZadd,
    ReqRedisZcard,
    ReqRedisZcount,
    ReqRedisZdiff,
    ReqRedisZdiffstore,
    ReqRedisZincrby,
    ReqRedisZinter,
    ReqRedisZinterstore,
    ReqRedisZlexcount,
    ReqRedisZmscore,
    ReqRedisZpopmin,
    ReqRedisZpopmax,
    ReqRedisZrandmember,
    ReqRedisZrange,
    ReqRedisZrangebylex,
    ReqRedisZrangebyscore,
    ReqRedisZrangestore,
    ReqRedisZrank,
    ReqRedisZrem,
    ReqRedisZremrangebyrank,
    ReqRedisZremrangebylex,
    ReqRedisZremrangebyscore,
    ReqRedisZrevrange,
    ReqRedisZrevrangebylex,
    ReqRedisZrevrangebyscore,
    ReqRedisZrevrank,
    ReqRedisZunion,
    ReqRedisZscan,
    ReqRedisZscore,
    ReqRedisZunionstore,
    ReqRedisGeoadd,
    ReqRedisGeodist,
    ReqRedisGeohash,
    ReqRedisGeoradius,
    ReqRedisGeoradiusbymember,
    ReqRedisGeopos,
    ReqRedisGeosearch,
    ReqRedisGeosearchstore,
    ReqRedisEval,
    ReqRedisEvalsha,
    ReqRedisPing,
    ReqRedisQuit,
    ReqRedisAuth,
    ReqRedisSelect,
    ReqRedisCommand,
    ReqRedisLolwut,
    RspRedisStatus,
    RspRedisError,
    RspRedisErrorErr,
    RspRedisErrorOom,
    RspRedisErrorBusy,
    RspRedisErrorNoauth,
    RspRedisErrorLoading,
    RspRedisErrorBusykey,
    RspRedisErrorMisconf,
    RspRedisErrorNoscript,
    RspRedisErrorReadonly,
    RspRedisErrorWrongtype,
    RspRedisErrorExecabort,
    RspRedisErrorMasterdown,
    RspRedisErrorNoreplicas,
    RspRedisInteger,
    RspRedisBulk,
    RspRedisMultibulk,
    Sentinel,
}

#[derive(Debug)]
pub struct KeyPos {
    pub start: *const u8,
    pub end: *const u8,
}

pub struct Msg {
    pub id: u64,
    pub peer: Option<Box<Msg>>,
    pub owner: Option<Box<Conn>>,
    
    pub mhdr: Vec<Mbuf>,
    pub mlen: u32,
    pub start_ts: i64,
    
    pub state: i32,
    pub pos: *const u8,
    pub token: *const u8,
    
    pub parser: Option<MsgParseFn>,
    pub result: MsgParseResult,
    
    pub fragment: Option<MsgFragmentFn>,
    pub reply: Option<MsgReplyFn>,
    pub add_auth: Option<MsgAddAuthFn>,
    pub failure: Option<MsgFailureFn>,
    
    pub pre_coalesce: Option<MsgCoalesceFn>,
    pub post_coalesce: Option<MsgCoalesceFn>,
    
    pub type_: MsgType,
    pub keys: Vec<KeyPos>,
    
    pub vlen: u32,
    pub end: *const u8,
    
    pub narg_start: *const u8,
    pub narg_end: *const u8,
    pub narg: u32,
    pub rnarg: u32,
    pub rlen: u32,
    pub integer: u32,
    pub is_top_level: u8,
    
    pub frag_owner: Option<Box<Msg>>,
    pub nfrag: u32,
    pub nfrag_done: u32,
    pub frag_id: u64,
    pub frag_seq: Vec<Msg>,
    
    pub err: Error,
    pub error: bool,
    pub ferror: bool,
    pub request: bool,
    pub quit: bool,
    pub noreply: bool,
    pub noforward: bool,
    pub done: bool,
    pub fdone: bool,
    pub swallow: bool,
    pub redis: bool,
}

impl Msg {
    pub fn new(conn: Option<Box<Conn>>, request: bool, redis: bool) -> Result<Self> {
        let mut msg = Msg {
            id: 0,
            peer: None,
            owner: conn,
            
            mhdr: Vec::new(),
            mlen: 0,
            start_ts: 0,
            
            state: 0,
            pos: ptr::null(),
            token: ptr::null(),
            
            parser: None,
            result: MsgParseResult::Ok,
            
            fragment: None,
            reply: None,
            add_auth: None,
            failure: None,
            
            pre_coalesce: None,
            post_coalesce: None,
            
            type_: MsgType::Unknown,
            keys: Vec::new(),
            
            vlen: 0,
            end: ptr::null(),
            
            narg_start: ptr::null(),
            narg_end: ptr::null(),
            narg: 0,
            rnarg: 0,
            rlen: 0,
            integer: 0,
            is_top_level: 0,
            
            frag_owner: None,
            nfrag: 0,
            nfrag_done: 0,
            frag_id: 0,
            frag_seq: Vec::new(),
            
            err: Error::Ok,
            error: false,
            ferror: false,
            request,
            quit: false,
            noreply: false,
            noforward: false,
            done: false,
            fdone: false,
            swallow: false,
            redis,
        };

        msg.id = MSG_ID.fetch_add(1, Ordering::SeqCst);
        
        if redis {
            if request {
                msg.parser = Some(RedisParser::parse_req);
            } else {
                msg.parser = Some(RedisParser::parse_rsp);
            }
            msg.add_auth = Some(RedisParser::add_auth);
            msg.fragment = Some(RedisParser::fragment);
            msg.reply = Some(RedisParser::reply);
            msg.failure = Some(RedisParser::failure);
            msg.pre_coalesce = Some(RedisParser::pre_coalesce);
            msg.post_coalesce = Some(RedisParser::post_coalesce);
        } else {
            if request {
                msg.parser = Some(MemcacheParser::parse_req);
            } else {
                msg.parser = Some(MemcacheParser::parse_rsp);
            }
            msg.add_auth = Some(MemcacheParser::add_auth);
            msg.fragment = Some(MemcacheParser::fragment);
            msg.failure = Some(MemcacheParser::failure);
            msg.pre_coalesce = Some(MemcacheParser::pre_coalesce);
            msg.post_coalesce = Some(MemcacheParser::post_coalesce);
        }

        Ok(msg)
    }

    pub fn get_error(redis: bool, err: Error) -> Result<Self> {
        let mut msg = Msg::new(None, false, redis)?;
        msg.type_ = if redis {
            MsgType::RspRedisError
        } else {
            MsgType::RspMcServerError
        };

        let err_str = err.to_string();
        let prot_str = if redis { "-ERR" } else { "SERVER_ERROR" };
        let mut mbuf = Mbuf::new()?;
        
        write!(mbuf, "{} {} \r\n", prot_str, err_str)?;
        msg.mhdr.push(mbuf);
        msg.mlen = mbuf.len() as u32;

        Ok(msg)
    }

    pub fn empty(&self) -> bool {
        self.mlen == 0
    }

    pub fn backend_idx(&self, key: &[u8]) -> u32 {
        if let Some(owner) = &self.owner {
            if let Some(pool) = &owner.owner {
                return pool.server_idx(key);
            }
        }
        0
    }

    pub fn ensure_mbuf(&mut self, len: usize) -> Result<&mut Mbuf> {
        if self.mhdr.is_empty() || self.mhdr.last().unwrap().remaining() < len {
            let mbuf = Mbuf::new()?;
            self.mhdr.push(mbuf);
        }
        Ok(self.mhdr.last_mut().unwrap())
    }

    pub fn append(&mut self, data: &[u8]) -> Result<()> {
        let mbuf = self.ensure_mbuf(data.len())?;
        mbuf.write_all(data)?;
        self.mlen += data.len() as u32;
        Ok(())
    }

    pub fn prepend(&mut self, data: &[u8]) -> Result<()> {
        let mut mbuf = Mbuf::new()?;
        mbuf.write_all(data)?;
        self.mhdr.insert(0, mbuf);
        self.mlen += data.len() as u32;
        Ok(())
    }

    pub fn prepend_format(&mut self, fmt: &str, args: fmt::Arguments) -> Result<()> {
        let mut mbuf = Mbuf::new()?;
        write!(mbuf, "{}", args)?;
        self.mhdr.insert(0, mbuf);
        self.mlen += mbuf.len() as u32;
        Ok(())
    }

    pub fn gen_frag_id() -> u64 {
        FRAG_ID.fetch_add(1, Ordering::SeqCst)
    }

    pub fn set_placeholder_key(&mut self) -> bool {
        let key = b"placeholder";
        let kpos = KeyPos {
            start: key.as_ptr(),
            end: unsafe { key.as_ptr().add(key.len()) },
        };
        self.keys.push(kpos);
        true
    }
}

static MSG_ID: AtomicU64 = AtomicU64::new(0);
static FRAG_ID: AtomicU64 = AtomicU64::new(0);

pub struct MsgQueue {
    free: VecDeque<Msg>,
}

impl MsgQueue {
    pub fn new() -> Self {
        MsgQueue {
            free: VecDeque::new(),
        }
    }

    pub fn get(&mut self, conn: Option<Box<Conn>>, request: bool, redis: bool) -> Result<Msg> {
        if let Some(mut msg) = self.free.pop_front() {
            msg.id = MSG_ID.fetch_add(1, Ordering::SeqCst);
            msg.owner = conn;
            msg.request = request;
            msg.redis = redis;
            Ok(msg)
        } else {
            Msg::new(conn, request, redis)
        }
    }

    pub fn put(&mut self, mut msg: Msg) {
        msg.mhdr.clear();
        msg.keys.clear();
        msg.frag_seq.clear();
        self.free.push_back(msg);
    }
}

pub struct TimeoutQueue {
    queue: VecDeque<(Instant, Box<Msg>)>,
}

impl TimeoutQueue {
    pub fn new() -> Self {
        TimeoutQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, msg: Box<Msg>, timeout: Duration) {
        self.queue.push_back((Instant::now() + timeout, msg));
    }

    pub fn delete(&mut self, msg: &Msg) {
        self.queue.retain(|(_, m)| m.id != msg.id);
    }

    pub fn min(&self) -> Option<&Msg> {
        self.queue.front().map(|(_, m)| &**m)
    }
}