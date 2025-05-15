use std::net::{SocketAddr, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::io::{Error, ErrorKind, Result};

type RStatus = Result<()>;

struct Context;
struct ServerPool;
struct Server;
struct Msg;

struct Conn {
    owner: Option<Arc<Mutex<dyn std::any::Any + Send + Sync>>>,
    sd: RawFd,
    family: i32,
    addr: Option<SocketAddr>,
    imsg_q: VecDeque<Arc<Msg>>,
    omsg_q: VecDeque<Arc<Msg>>,
    rmsg: Option<Arc<Msg>>,
    smsg: Option<Arc<Msg>>,
    recv_bytes: usize,
    send_bytes: usize,
    events: u32,
    err: i32,
    recv_active: bool,
    recv_ready: bool,
    send_active: bool,
    send_ready: bool,
    is_client: bool,
    is_proxy: bool,
    is_connecting: bool,
    is_connected: bool,
    eof: bool,
    done: bool,
    is_redis: bool,
    authenticated: bool,
}

static NCURR_CONN: AtomicU32 = AtomicU32::new(0);
static NCURR_CCONN: AtomicU32 = AtomicU32::new(0);
static NTOTAL_CONN: AtomicU64 = AtomicU64::new(0);
static FREE_CONNQ: Mutex<VecDeque<Arc<Mutex<Conn>>>> = Mutex::new(VecDeque::new());

impl Conn {
    fn new() -> Arc<Mutex<Self>> {
        let conn = Arc::new(Mutex::new(Conn {
            owner: None,
            sd: -1,
            family: 0,
            addr: None,
            imsg_q: VecDeque::new(),
            omsg_q: VecDeque::new(),
            rmsg: None,
            smsg: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
            recv_active: false,
            recv_ready: false,
            send_active: false,
            send_ready: false,
            is_client: false,
            is_proxy: false,
            is_connecting: false,
            is_connected: false,
            eof: false,
            done: false,
            is_redis: false,
            authenticated: false,
        }));

        NTOTAL_CONN.fetch_add(1, Ordering::SeqCst);
        NCURR_CONN.fetch_add(1, Ordering::SeqCst);

        conn
    }

    fn get(owner: Arc<Mutex<dyn std::any::Any + Send + Sync>>, is_client: bool, is_redis: bool) -> Arc<Mutex<Self>> {
        let conn = if let Some(free_conn) = FREE_CONNQ.lock().unwrap().pop_front() {
            free_conn
        } else {
            Self::new()
        };

        let mut conn = conn.lock().unwrap();
        conn.owner = Some(owner.clone());
        conn.is_redis = is_redis;
        conn.is_client = is_client;

        if is_client {
            NCURR_CCONN.fetch_add(1, Ordering::SeqCst);
        }

        Arc::new(Mutex::new(conn))
    }

    fn put(conn: Arc<Mutex<Self>>) {
        let mut conn = conn.lock().unwrap();
        assert!(conn.sd == -1);
        assert!(conn.owner.is_none());

        if conn.is_client {
            NCURR_CCONN.fetch_sub(1, Ordering::SeqCst);
        }
        NCURR_CONN.fetch_sub(1, Ordering::SeqCst);

        FREE_CONNQ.lock().unwrap().push_back(conn);
    }

    fn recv(&mut self, buf: &mut [u8]) -> Result<usize> {
        assert!(!buf.is_empty());
        assert!(self.recv_ready);

        loop {
            let stream = unsafe { TcpStream::from_raw_fd(self.sd) };
            let n = stream.read(buf);
            std::mem::forget(stream);

            match n {
                Ok(n) if n > 0 => {
                    if n < buf.len() {
                        self.recv_ready = false;
                    }
                    self.recv_bytes += n;
                    return Ok(n);
                }
                Ok(0) => {
                    self.recv_ready = false;
                    self.eof = true;
                    return Ok(0);
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    self.recv_ready = false;
                    return Err(Error::new(ErrorKind::WouldBlock, "EAGAIN"));
                }
                Err(e) => {
                    self.recv_ready = false;
                    self.err = e.raw_os_error().unwrap_or(0);
                    return Err(e);
                }
            }
        }
    }

    fn sendv(&mut self, buffers: &[&[u8]]) -> Result<usize> {
        assert!(!buffers.is_empty());
        assert!(self.send_ready);

        loop {
            let stream = unsafe { TcpStream::from_raw_fd(self.sd) };
            let n = stream.write_vectored(buffers);
            std::mem::forget(stream);

            match n {
                Ok(n) if n > 0 => {
                    if n < buffers.iter().map(|b| b.len()).sum() {
                        self.send_ready = false;
                    }
                    self.send_bytes += n;
                    return Ok(n);
                }
                Ok(0) => {
                    self.send_ready = false;
                    return Ok(0);
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    self.send_ready = false;
                    return Err(Error::new(ErrorKind::WouldBlock, "EAGAIN"));
                }
                Err(e) => {
                    self.send_ready = false;
                    self.err = e.raw_os_error().unwrap_or(0);
                    return Err(e);
                }
            }
        }
    }
}

fn conn_ncurr_conn() -> u32 {
    NCURR_CONN.load(Ordering::SeqCst)
}

fn conn_ntotal_conn() -> u64 {
    NTOTAL_CONN.load(Ordering::SeqCst)
}

fn conn_ncurr_cconn() -> u32 {
    NCURR_CCONN.load(Ordering::SeqCst)
}

fn conn_authenticated(conn: &Conn) -> bool {
    assert!(!conn.is_proxy);

    let pool = if conn.is_client {
        conn.owner.as_ref().unwrap().downcast_ref::<ServerPool>().unwrap()
    } else {
        conn.owner.as_ref().unwrap().downcast_ref::<Server>().unwrap().owner
    };

    if !pool.require_auth {
        return true;
    }

    if !conn.authenticated {
        return false;
    }

    true
}

fn conn_init() {
    // Initialization logic
}

fn conn_deinit() {
    FREE_CONNQ.lock().unwrap().clear();
}