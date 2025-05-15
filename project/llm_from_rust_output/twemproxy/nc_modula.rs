use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_int, c_long, c_uint, c_ulong, c_void};

type uint32_t = c_uint;
type int64_t = c_long;
type size_t = c_ulong;
type rstatus_t = c_int;

#[derive(Debug, Clone)]
struct Array {
    nelem: uint32_t,
    elem: *mut c_void,
    size: size_t,
    nalloc: uint32_t,
}

#[derive(Debug, Clone)]
struct Server {
    idx: uint32_t,
    weight: uint32_t,
    next_retry: int64_t,
}

#[derive(Debug, Clone)]
struct Continuum {
    index: uint32_t,
    value: uint32_t,
}

#[derive(Debug, Clone)]
struct ServerPool {
    server: Array,
    nlive_server: uint32_t,
    next_rebuild: int64_t,
    nserver_continuum: uint32_t,
    ncontinuum: uint32_t,
    continuum: *mut Continuum,
    auto_eject_hosts: bool,
}

fn array_n(a: &Array) -> uint32_t {
    a.nelem
}

fn array_get(a: &Array, idx: uint32_t) -> *mut c_void {
    unsafe { ptr::offset(a.elem, idx as isize) }
}

fn nc_usec_now() -> int64_t {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as int64_t
}

fn modula_update(pool: &mut ServerPool) -> rstatus_t {
    let now = nc_usec_now();
    if now < 0 {
        return -1;
    }

    let nserver = array_n(&pool.server);
    let mut nlive_server = 0;
    let mut total_weight = 0;
    pool.next_rebuild = 0;

    // First pass: count live servers and total weight
    for server_index in 0..nserver {
        let server = unsafe { &mut *(array_get(&pool.server, server_index) as *mut Server) };
        
        if pool.auto_eject_hosts {
            if server.next_retry <= now {
                server.next_retry = 0;
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

    // Allocate continuum if needed
    if total_weight > pool.nserver_continuum {
        let nserver_continuum = total_weight + 10;
        let ncontinuum = nserver_continuum * 1;
        
        let new_size = std::mem::size_of::<Continuum>() * ncontinuum as usize;
        let continuum = unsafe {
            libc::realloc(
                pool.continuum as *mut c_void,
                new_size
            ) as *mut Continuum
        };
        
        if continuum.is_null() {
            return -3;
        }
        
        pool.continuum = continuum;
        pool.nserver_continuum = nserver_continuum;
    }

    // Build continuum
    let mut continuum_index = 0;
    let mut pointer_counter = 0;
    
    for server_index in 0..nserver {
        let server = unsafe { &mut *(array_get(&pool.server, server_index) as *mut Server };
        
        if pool.auto_eject_hosts && server.next_retry > now {
            continue;
        }

        for _ in 0..server.weight {
            unsafe {
                (*pool.continuum.add(continuum_index as usize)).index = server_index;
                (*pool.continuum.add(continuum_index as usize)).value = 0;
            }
            continuum_index += 1;
            pointer_counter += 1;
        }
    }

    pool.ncontinuum = pointer_counter;
    0
}

fn modula_dispatch(continuum: &[Continuum], hash: uint32_t) -> uint32_t {
    let ncontinuum = continuum.len() as uint32_t;
    let c = &continuum[(hash % ncontinuum) as usize];
    c.index
}