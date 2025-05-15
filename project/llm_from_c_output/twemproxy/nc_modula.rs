use std::time::{SystemTime, UNIX_EPOCH};
use std::ptr;
use std::cmp;
use std::fmt;

const MODULA_CONTINUUM_ADDITION: u32 = 10;  /* # extra slots to build into continuum */
const MODULA_POINTS_PER_SERVER: u32 = 1;

#[derive(Debug)]
struct Continuum {
    index: u32,
    value: u32,
}

struct Server {
    weight: u32,
    next_retry: i64,
}

struct ServerPool {
    server: Vec<Server>,
    continuum: Vec<Continuum>,
    nlive_server: u32,
    nserver_continuum: u32,
    ncontinuum: u32,
    next_rebuild: i64,
    auto_eject_hosts: bool,
    idx: u32,
    name: Name,
}

struct Name {
    len: usize,
    data: Vec<u8>,
}

#[derive(Debug)]
enum Error {
    InvalidTime,
    OutOfMemory,
}

type Result<T> = std::result::Result<T, Error>;

fn nc_usec_now() -> Result<i64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .map_err(|_| Error::InvalidTime)
}

fn modula_update(pool: &mut ServerPool) -> Result<()> {
    let now = nc_usec_now()?;
    
    let nserver = pool.server.len() as u32;
    let mut nlive_server = 0;
    let mut total_weight = 0;
    pool.next_rebuild = 0;

    for (server_index, server) in pool.server.iter().enumerate() {
        if pool.auto_eject_hosts {
            if server.next_retry <= now {
                nlive_server += 1;
            } else if pool.next_rebuild == 0 || server.next_retry < pool.next_rebuild {
                pool.next_rebuild = server.next_retry;
            }
        } else {
            nlive_server += 1;
        }

        debug_assert!(server.weight > 0);

        if !pool.auto_eject_hosts || server.next_retry <= now {
            total_weight += server.weight;
        }
    }

    pool.nlive_server = nlive_server;

    if nlive_server == 0 {
        debug_assert!(!pool.continuum.is_empty());
        debug_assert!(pool.ncontinuum != 0);

        log::debug!("no live servers for pool {} '{}'",
                  pool.idx, String::from_utf8_lossy(&pool.name.data));

        return Ok(());
    }

    log::debug!("{} of {} servers are live for pool {} '{}'", 
              nlive_server, nserver, pool.idx, 
              String::from_utf8_lossy(&pool.name.data));

    let continuum_addition = MODULA_CONTINUUM_ADDITION;
    let points_per_server = MODULA_POINTS_PER_SERVER;

    if total_weight > pool.nserver_continuum {
        let nserver_continuum = total_weight + MODULA_CONTINUUM_ADDITION;
        let ncontinuum = nserver_continuum * MODULA_POINTS_PER_SERVER;

        pool.continuum = Vec::with_capacity(ncontinuum as usize);
        pool.nserver_continuum = nserver_continuum;
    }

    let mut continuum_index = 0;
    let mut pointer_counter = 0;
    for (server_index, server) in pool.server.iter().enumerate() {
        if pool.auto_eject_hosts && server.next_retry > now {
            continue;
        }

        for _ in 0..server.weight {
            let pointer_per_server = 1;

            pool.continuum.push(Continuum {
                index: server_index as u32,
                value: 0,
            });
            continuum_index += 1;
            pointer_counter += pointer_per_server;
        }
    }
    pool.ncontinuum = pointer_counter;

    log::debug!("updated pool {} '{}' with {} of {} servers live in {} slots and {} active points in {} slots", 
              pool.idx, String::from_utf8_lossy(&pool.name.data), 
              nlive_server, nserver, pool.nserver_continuum, 
              pool.ncontinuum, (pool.nserver_continuum + continuum_addition) * points_per_server);

    Ok(())
}

fn modula_dispatch(continuum: &[Continuum], hash: u32) -> u32 {
    debug_assert!(!continuum.is_empty());
    
    let index = hash % continuum.len() as u32;
    continuum[index as usize].index
}