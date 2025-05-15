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

use std::time::{SystemTime, UNIX_EPOCH};
use std::ptr;
use std::mem;
use std::slice;
use std::cmp;
use std::time::Duration;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const RANDOM_CONTINUUM_ADDITION: u32 = 10;  /* # extra slots to build into continuum */
const RANDOM_POINTS_PER_SERVER: u32 = 1;

#[derive(Debug)]
struct Continuum {
    index: u32,
    value: u32,
}

struct Server {
    next_retry: u64,
    // other fields...
}

struct ServerPool {
    server: Vec<Server>,
    next_rebuild: u64,
    auto_eject_hosts: bool,
    nlive_server: u32,
    nserver_continuum: u32,
    ncontinuum: u32,
    continuum: Vec<Continuum>,
    idx: u32,
    name: String,
}

impl ServerPool {
    fn random_update(&mut self) -> Result<(), &'static str> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Failed to get current time")?
            .as_secs();

        let nserver = self.server.len() as u32;
        let mut nlive_server = 0;
        self.next_rebuild = 0;

        for (server_index, server) in self.server.iter_mut().enumerate() {
            if self.auto_eject_hosts {
                if server.next_retry <= now {
                    server.next_retry = 0;
                    nlive_server += 1;
                } else if self.next_rebuild == 0 || server.next_retry < self.next_rebuild {
                    self.next_rebuild = server.next_retry;
                }
            } else {
                nlive_server += 1;
            }
        }

        self.nlive_server = nlive_server;

        if nlive_server == 0 {
            if self.continuum.is_empty() || self.ncontinuum == 0 {
                return Err("No live servers but continuum is empty");
            }

            log_debug(format!(
                "no live servers for pool {} '{}'",
                self.idx, self.name
            ));
            return Ok(());
        }

        log_debug(format!(
            "{} of {} servers are live for pool {} '{}'",
            nlive_server, nserver, self.idx, self.name
        ));

        let continuum_addition = RANDOM_CONTINUUM_ADDITION;
        let points_per_server = RANDOM_POINTS_PER_SERVER;

        if nlive_server > self.nserver_continuum {
            let nserver_continuum = nlive_server + RANDOM_CONTINUUM_ADDITION;
            let ncontinuum = nserver_continuum * RANDOM_POINTS_PER_SERVER;

            self.continuum = vec![Continuum { index: 0, value: 0 }; ncontinuum as usize];
            self.nserver_continuum = nserver_continuum;
        }

        let mut continuum_index = 0;
        let mut pointer_counter = 0;

        for (server_index, server) in self.server.iter().enumerate() {
            if self.auto_eject_hosts && server.next_retry > now {
                continue;
            }

            let pointer_per_server = 1;

            self.continuum[continuum_index].index = server_index as u32;
            self.continuum[continuum_index].value = 0;
            continuum_index += 1;

            pointer_counter += pointer_per_server;
        }

        self.ncontinuum = pointer_counter;

        log_debug(format!(
            "updated pool {} '{}' with {} of {} servers live in {} slots and {} active points in {} slots",
            self.idx,
            self.name,
            nlive_server,
            nserver,
            self.nserver_continuum,
            self.ncontinuum,
            (self.nserver_continuum + continuum_addition) * points_per_server
        ));

        Ok(())
    }
}

fn random_dispatch(continuum: &[Continuum], ncontinuum: u32, hash: u32) -> u32 {
    assert!(!continuum.is_empty());
    assert!(ncontinuum != 0);

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..ncontinuum);
    continuum[idx as usize].index
}

fn log_debug(msg: String) {
    println!("{}", msg);
}