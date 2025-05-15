use std::cmp::Ordering;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use std::ptr;
use std::slice;
use std::time::{SystemTime, UNIX_EPOCH};

use libc::{size_t, uint32_t};
use md5::{Digest, Md5};

const KETAMA_CONTINUUM_ADDITION: u32 = 10;
const KETAMA_POINTS_PER_SERVER: u32 = 160;
const KETAMA_MAX_HOSTLEN: usize = 273;

#[derive(Debug, Clone, Copy)]
struct Continuum {
    index: u32,
    value: u32,
}

impl PartialEq for Continuum {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Continuum {}

impl PartialOrd for Continuum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Continuum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

fn ketama_hash(key: &[u8], alignment: u32) -> u32 {
    let mut hasher = Md5::new();
    hasher.update(key);
    let result = hasher.finalize();

    ((u32::from(result[(3 + alignment * 4) as usize]) & 0xFF) << 24)
        | ((u32::from(result[(2 + alignment * 4) as usize]) & 0xFF) << 16)
        | ((u32::from(result[(1 + alignment * 4) as usize]) & 0xFF) << 8)
        | (u32::from(result[(0 + alignment * 4) as usize]) & 0xFF)
}

struct ServerPool {
    continuum: Vec<Continuum>,
    nserver_continuum: u32,
    ncontinuum: u32,
    nlive_server: u32,
    next_rebuild: i64,
    auto_eject_hosts: bool,
    servers: Vec<Server>,
    idx: u32,
    name: String,
}

struct Server {
    weight: u32,
    next_retry: i64,
    name: String,
}

impl ServerPool {
    fn ketama_update(&mut self) -> Result<(), &'static str> {
        let nserver = self.servers.len() as u32;
        let mut nlive_server = 0;
        let mut total_weight = 0;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "System time before UNIX EPOCH")?
            .as_micros() as i64;

        self.next_rebuild = 0;

        for server in &self.servers {
            if self.auto_eject_hosts {
                if server.next_retry <= now {
                    nlive_server += 1;
                } else if self.next_rebuild == 0 || server.next_retry < self.next_rebuild {
                    self.next_rebuild = server.next_retry;
                }
            } else {
                nlive_server += 1;
            }

            if !self.auto_eject_hosts || server.next_retry <= now {
                total_weight += server.weight;
            }
        }

        self.nlive_server = nlive_server;

        if nlive_server == 0 {
            return Ok(());
        }

        let points_per_server = KETAMA_POINTS_PER_SERVER;
        if nlive_server > self.nserver_continuum {
            let nserver_continuum = nlive_server + KETAMA_CONTINUUM_ADDITION;
            let ncontinuum = nserver_continuum * points_per_server;
            self.continuum = vec![Continuum { index: 0, value: 0 }; ncontinuum as usize];
            self.nserver_continuum = nserver_continuum;
        }

        let mut continuum_index = 0;
        let mut pointer_counter = 0;

        for (server_index, server) in self.servers.iter().enumerate() {
            if self.auto_eject_hosts && server.next_retry > now {
                continue;
            }

            let pct = server.weight as f32 / total_weight as f32;
            let pointer_per_server = ((pct * KETAMA_POINTS_PER_SERVER as f32 / 4.0 * nlive_server as f32 + 0.0000000001).floor()) as u32 * 4;
            let pointer_per_hash = 4;

            for pointer_index in 1..=(pointer_per_server / pointer_per_hash) {
                let host = format!("{}-{}", server.name, pointer_index - 1);
                let host_bytes = host.as_bytes();

                if host_bytes.len() >= KETAMA_MAX_HOSTLEN {
                    // Truncate if necessary
                    let truncated = &host_bytes[..KETAMA_MAX_HOSTLEN - 1];
                    for x in 0..pointer_per_hash {
                        let value = ketama_hash(truncated, x);
                        self.continuum[continuum_index] = Continuum {
                            index: server_index as u32,
                            value,
                        };
                        continuum_index += 1;
                    }
                } else {
                    for x in 0..pointer_per_hash {
                        let value = ketama_hash(host_bytes, x);
                        self.continuum[continuum_index] = Continuum {
                            index: server_index as u32,
                            value,
                        };
                        continuum_index += 1;
                    }
                }
            }
            pointer_counter += pointer_per_server;
        }

        self.ncontinuum = pointer_counter;
        self.continuum[..self.ncontinuum as usize].sort();

        Ok(())
    }

    fn ketama_dispatch(&self, hash: u32) -> u32 {
        let continuum = &self.continuum[..self.ncontinuum as usize];
        match continuum.binary_search_by(|c| c.value.cmp(&hash)) {
            Ok(idx) => continuum[idx].index,
            Err(idx) => {
                if idx == continuum.len() {
                    continuum[0].index
                } else {
                    continuum[idx].index
                }
            }
        }
    }
}