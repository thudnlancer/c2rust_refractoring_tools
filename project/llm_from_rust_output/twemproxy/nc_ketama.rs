use std::cmp::Ordering;
use std::mem;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

const KETAMA_POINTS_PER_SERVER: u32 = 160;
const KETAMA_POINTERS_PER_HASH: u32 = 4;
const KETAMA_CONTINUUM_ADDITION: u32 = 10;
const KETAMA_HOSTNAME_MAX_LEN: usize = 273;

#[derive(Debug, Clone, Copy)]
struct Continuum {
    index: u32,
    value: u32,
}

#[derive(Debug)]
struct Server {
    name: String,
    weight: u32,
    next_retry: i64,
    // Other fields omitted for brevity
}

#[derive(Debug)]
struct ServerPool {
    server: Vec<Server>,
    continuum: Vec<Continuum>,
    nserver_continuum: u32,
    nlive_server: u32,
    next_rebuild: i64,
    auto_eject_hosts: bool,
    // Other fields omitted for brevity
}

fn ketama_hash(key: &[u8], alignment: u32) -> u32 {
    let mut result = md5::compute(key);
    let offset = (3 + alignment * 4) as usize;
    
    ((result[offset] as u32 & 0xff) << 24) |
    ((result[offset - 1] as u32 & 0xff) << 16) |
    ((result[offset - 2] as u32 & 0xff) << 8) |
    (result[offset - 3] as u32 & 0xff)
}

fn ketama_item_cmp(a: &Continuum, b: &Continuum) -> Ordering {
    a.value.cmp(&b.value)
}

fn ketama_update(pool: &mut ServerPool) -> i32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    if now < 0 {
        return -1;
    }

    let mut nlive_server = 0;
    let mut total_weight = 0;
    pool.next_rebuild = 0;

    for server in &pool.server {
        if pool.auto_eject_hosts {
            if server.next_retry <= now {
                nlive_server += 1;
            } else if pool.next_rebuild == 0 || server.next_retry < pool.next_rebuild {
                pool.next_rebuild = server.next_retry;
            }
        } else {
            nlive_server += 1;
        }

        if !pool.auto_eject_hosts || server.next_retry <= now {
            total_weight += server.weight;
        }
    }

    pool.nlive_server = nlive_server;
    if nlive_server == 0 {
        return 0;
    }

    let points_per_server = KETAMA_POINTS_PER_SERVER;
    let nserver_continuum = nlive_server + KETAMA_CONTINUUM_ADDITION;
    let ncontinuum = nserver_continuum * points_per_server;

    if nlive_server > pool.nserver_continuum {
        pool.continuum = vec![Continuum { index: 0, value: 0 }; ncontinuum as usize];
        pool.nserver_continuum = nserver_continuum;
    }

    let mut continuum_index = 0;
    let mut pointer_counter = 0;

    for (server_index, server) in pool.server.iter().enumerate() {
        if pool.auto_eject_hosts && server.next_retry > now {
            continue;
        }

        let pct = server.weight as f32 / total_weight as f32;
        let pointer_per_server = ((pct * (points_per_server as f32 / 4.0) * nlive_server as f32 + 0.0000000001).floor() as u32 * 4;

        for pointer_index in 1..=(pointer_per_server / KETAMA_POINTERS_PER_HASH) {
            let host = format!("{}-{}", server.name, pointer_index - 1);
            let host_bytes = host.as_bytes();

            for x in 0..KETAMA_POINTERS_PER_HASH {
                let value = ketama_hash(host_bytes, x);
                pool.continuum[continuum_index as usize] = Continuum {
                    index: server_index as u32,
                    value,
                };
                continuum_index += 1;
            }
        }

        pointer_counter += pointer_per_server;
    }

    pool.ncontinuum = pointer_counter;
    pool.continuum.sort_by(ketama_item_cmp);

    0
}

fn ketama_dispatch(continuum: &[Continuum], hash: u32) -> u32 {
    match continuum.binary_search_by(|c| c.value.cmp(&hash)) {
        Ok(idx) => continuum[idx].index,
        Err(idx) => {
            if idx >= continuum.len() {
                continuum[0].index
            } else {
                continuum[idx].index
            }
        }
    }
}