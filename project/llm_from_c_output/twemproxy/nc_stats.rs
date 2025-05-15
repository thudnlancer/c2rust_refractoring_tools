use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatsType {
    Invalid,
    Counter,
    Gauge,
    Timestamp,
    Sentinel,
}

#[derive(Debug, Clone)]
pub struct StatsMetric {
    pub type_: StatsType,
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct StatsServer {
    pub name: String,
    pub metrics: Vec<StatsMetric>,
}

#[derive(Debug, Clone)]
pub struct StatsPool {
    pub name: String,
    pub metrics: Vec<StatsMetric>,
    pub servers: Vec<StatsServer>,
}

#[derive(Debug)]
pub struct StatsBuffer {
    pub len: usize,
    pub data: Vec<u8>,
    pub size: usize,
}

#[derive(Debug)]
pub struct Stats {
    pub port: u16,
    pub interval: i32,
    pub addr: String,
    pub start_ts: i64,
    pub buf: StatsBuffer,
    pub current: Vec<StatsPool>,
    pub shadow: Vec<StatsPool>,
    pub sum: Vec<StatsPool>,
    pub tid: Option<thread::JoinHandle<()>>,
    pub sd: Option<TcpListener>,
    pub service_str: String,
    pub service: String,
    pub source_str: String,
    pub source: String,
    pub version_str: String,
    pub version: String,
    pub uptime_str: String,
    pub timestamp_str: String,
    pub ntotal_conn_str: String,
    pub ncurr_conn_str: String,
    pub aggregate: bool,
    pub updated: bool,
}

impl Stats {
    pub fn new(
        stats_port: u16,
        stats_ip: &str,
        stats_interval: i32,
        source: &str,
        server_pools: &[StatsPool],
    ) -> Result<Arc<Mutex<Self>>, String> {
        let start_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut stats = Self {
            port: stats_port,
            interval: stats_interval,
            addr: stats_ip.to_string(),
            start_ts,
            buf: StatsBuffer {
                len: 0,
                data: Vec::new(),
                size: 0,
            },
            current: server_pools.to_vec(),
            shadow: server_pools.to_vec(),
            sum: server_pools.to_vec(),
            tid: None,
            sd: None,
            service_str: "service".to_string(),
            service: "nutcracker".to_string(),
            source_str: "source".to_string(),
            source: source.to_string(),
            version_str: "version".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_str: "uptime".to_string(),
            timestamp_str: "timestamp".to_string(),
            ntotal_conn_str: "total_connections".to_string(),
            ncurr_conn_str: "curr_connections".to_string(),
            aggregate: false,
            updated: false,
        };

        stats.create_buf()?;
        let stats = Arc::new(Mutex::new(stats));
        stats.lock().unwrap().start_aggregator()?;
        Ok(stats)
    }

    fn create_buf(&mut self) -> Result<(), String> {
        let int64_max_digits = 20;
        let key_value_extra = 8;
        let pool_extra = 8;
        let server_extra = 8;
        let mut size = 0;

        size += 1;

        size += self.service_str.len();
        size += self.service.len();
        size += key_value_extra;

        size += self.source_str.len();
        size += self.source.len();
        size += key_value_extra;

        size += self.version_str.len();
        size += self.version.len();
        size += key_value_extra;

        size += self.uptime_str.len();
        size += int64_max_digits;
        size += key_value_extra;

        size += self.timestamp_str.len();
        size += int64_max_digits;
        size += key_value_extra;

        size += self.ntotal_conn_str.len();
        size += int64_max_digits;
        size += key_value_extra;

        size += self.ncurr_conn_str.len();
        size += int64_max_digits;
        size += key_value_extra;

        for pool in &self.sum {
            size += pool.name.len();
            size += pool_extra;

            for metric in &pool.metrics {
                size += metric.name.len();
                size += int64_max_digits;
                size += key_value_extra;
            }

            for server in &pool.servers {
                size += server.name.len();
                size += server_extra;

                for metric in &server.metrics {
                    size += metric.name.len();
                    size += int64_max_digits;
                    size += key_value_extra;
                }
            }
        }

        size += 2;
        size = (size + 7) & !7;

        self.buf.data = vec![0; size];
        self.buf.size = size;

        Ok(())
    }

    fn start_aggregator(&mut self) -> Result<(), String> {
        let listener = TcpListener::bind(format!("{}:{}", self.addr, self.port))
            .map_err(|e| format!("Failed to bind to stats server: {}", e))?;
        self.sd = Some(listener);

        let stats = Arc::new(Mutex::new(self.clone()));
        let stats_clone = Arc::clone(&stats);
        let handle = thread::spawn(move || {
            stats_loop(stats_clone);
        });
        self.tid = Some(handle);

        Ok(())
    }

    pub fn destroy(&mut self) {
        if let Some(listener) = self.sd.take() {
            drop(listener);
        }
        if let Some(handle) = self.tid.take() {
            handle.join().unwrap();
        }
    }

    pub fn swap(&mut self) {
        if self.aggregate {
            return;
        }
        if !self.updated {
            return;
        }

        std::mem::swap(&mut self.current, &mut self.shadow);
        self.reset_current();
        self.updated = false;
        self.aggregate = true;
    }

    fn reset_current(&mut self) {
        for pool in &mut self.current {
            for metric in &mut pool.metrics {
                match metric.type_ {
                    StatsType::Counter | StatsType::Gauge => metric.value = 0,
                    StatsType::Timestamp => metric.value = 0,
                    _ => (),
                }
            }

            for server in &mut pool.servers {
                for metric in &mut server.metrics {
                    match metric.type_ {
                        StatsType::Counter | StatsType::Gauge => metric.value = 0,
                        StatsType::Timestamp => metric.value = 0,
                        _ => (),
                    }
                }
            }
        }
    }
}

fn stats_loop(stats: Arc<Mutex<Stats>>) {
    loop {
        let mut stats = stats.lock().unwrap();
        stats.aggregate();

        if let Some(listener) = &stats.sd {
            if let Ok((stream, _)) = listener.accept() {
                if let Err(e) = stats.send_response(stream) {
                    eprintln!("Failed to send stats: {}", e);
                }
            }
        }
    }
}

impl Stats {
    fn aggregate(&mut self) {
        if !self.aggregate {
            return;
        }

        for (sum_pool, shadow_pool) in self.sum.iter_mut().zip(&self.shadow) {
            for (sum_metric, shadow_metric) in sum_pool.metrics.iter_mut().zip(&shadow_pool.metrics) {
                match sum_metric.type_ {
                    StatsType::Counter | StatsType::Gauge => {
                        sum_metric.value += shadow_metric.value;
                    }
                    StatsType::Timestamp => {
                        if shadow_metric.value != 0 {
                            sum_metric.value = shadow_metric.value;
                        }
                    }
                    _ => (),
                }
            }

            for (sum_server, shadow_server) in sum_pool.servers.iter_mut().zip(&shadow_pool.servers) {
                for (sum_metric, shadow_metric) in sum_server.metrics.iter_mut().zip(&shadow_server.metrics) {
                    match sum_metric.type_ {
                        StatsType::Counter | StatsType::Gauge => {
                            sum_metric.value += shadow_metric.value;
                        }
                        StatsType::Timestamp => {
                            if shadow_metric.value != 0 {
                                sum_metric.value = shadow_metric.value;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        self.aggregate = false;
    }

    fn send_response(&mut self, mut stream: TcpStream) -> Result<(), String> {
        self.make_response()?;
        std::io::Write::write_all(&mut stream, &self.buf.data)
            .map_err(|e| format!("Failed to send stats: {}", e))?;
        Ok(())
    }

    fn make_response(&mut self) -> Result<(), String> {
        self.add_header()?;

        for pool in &self.sum {
            self.begin_nesting(&pool.name)?;

            for metric in &pool.metrics {
                self.add_num(&metric.name, metric.value)?;
            }

            for server in &pool.servers {
                self.begin_nesting(&server.name)?;

                for metric in &server.metrics {
                    self.add_num(&metric.name, metric.value)?;
                }

                self.end_nesting()?;
            }

            self.end_nesting()?;
        }

        self.add_footer()?;
        Ok(())
    }

    fn add_header(&mut self) -> Result<(), String> {
        self.buf.data[0] = b'{';
        self.buf.len = 1;

        let cur_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let uptime = cur_ts - self.start_ts;

        self.add_string(&self.service_str, &self.service)?;
        self.add_string(&self.source_str, &self.source)?;
        self.add_string(&self.version_str, &self.version)?;
        self.add_num(&self.uptime_str, uptime)?;
        self.add_num(&self.timestamp_str, cur_ts)?;
        self.add_num(&self.ntotal_conn_str, 0)?; // TODO: Replace with actual connection counts
        self.add_num(&self.ncurr_conn_str, 0)?;

        Ok(())
    }

    fn add_footer(&mut self) -> Result<(), String> {
        if self.buf.len == self.buf.size {
            return Err("Buffer full".to_string());
        }

        let pos = self.buf.len - 1;
        self.buf.data[pos] = b'}';
        self.buf.data[pos + 1] = b'\n';
        self.buf.len += 1;

        Ok(())
    }

    fn add_string(&mut self, key: &str, val: &str) -> Result<(), String> {
        let s = format!("\"{}\":\"{}\", ", key, val);
        if self.buf.len + s.len() > self.buf.size {
            return Err("Buffer overflow".to_string());
        }
        self.buf.data[self.buf.len..self.buf.len + s.len()].copy_from_slice(s.as_bytes());
        self.buf.len += s.len();
        Ok(())
    }

    fn add_num(&mut self, key: &str, val: i64) -> Result<(), String> {
        let s = format!("\"{}\":{}, ", key, val);
        if self.buf.len + s.len() > self.buf.size {
            return Err("Buffer overflow".to_string());
        }
        self.buf.data[self.buf.len..self.buf.len + s.len()].copy_from_slice(s.as_bytes());
        self.buf.len += s.len();
        Ok(())
    }

    fn begin_nesting(&mut self, key: &str) -> Result<(), String> {
        let s = format!("\"{}\": {{", key);
        if self.buf.len + s.len() > self.buf.size {
            return Err("Buffer overflow".to_string());
        }
        self.buf.data[self.buf.len..self.buf.len + s.len()].copy_from_slice(s.as_bytes());
        self.buf.len += s.len();
        Ok(())
    }

    fn end_nesting(&mut self) -> Result<(), String> {
        let pos = self.buf.len - 2;
        match self.buf.data[pos] {
            b',' => {
                self.buf.data[pos] = b'}';
                self.buf.data[pos + 1] = b',';
            }
            b'}' => {
                if self.buf.len == self.buf.size {
                    return Err("Buffer full".to_string());
                }
                self.buf.data[pos + 1] = b'}';
                self.buf.data[pos + 2] = b',';
                self.buf.len += 1;
            }
            _ => return Err("Invalid nesting".to_string()),
        }
        Ok(())
    }
}

impl fmt::Display for StatsMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

pub fn stats_pool_incr(
    stats: &Arc<Mutex<Stats>>,
    pool_idx: usize,
    metric_idx: usize,
) -> Result<(), String> {
    let mut stats = stats.lock().unwrap();
    let pool = &mut stats.current[pool_idx];
    let metric = &mut pool.metrics[metric_idx];
    metric.value += 1;
    stats.updated = true;
    Ok(())
}

pub fn stats_server_incr(
    stats: &Arc<Mutex<Stats>>,
    pool_idx: usize,
    server_idx: usize,
    metric_idx: usize,
) -> Result<(), String> {
    let mut stats = stats.lock().unwrap();
    let pool = &mut stats.current[pool_idx];
    let server = &mut pool.servers[server_idx];
    let metric = &mut server.metrics[metric_idx];
    metric.value += 1;
    stats.updated = true;
    Ok(())
}