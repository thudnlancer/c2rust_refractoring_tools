/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  pth_msg.c: Pth message port facility
*/
                             /* ``Those who do not understand Unix
                                  are condemned to reinvent it, poorly.''
                                                   -- Henry Spencer      */

use std::ffi::CStr;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::collections::LinkedList;

type PthRingNode = ();
type PthRing = LinkedList<Arc<Mutex<PthMsgPort>>>;
type PthMessage = ();
type PthT = ();

struct PthMsgPort {
    mp_node: PthRingNode,
    mp_name: Option<String>,
    mp_tid: PthT,
    mp_queue: PthRing,
}

static PTH_MSGPORT: Mutex<PthRing> = Mutex::new(LinkedList::new());

pub fn pth_msgport_create(name: Option<&str>) -> Result<Arc<Mutex<PthMsgPort>>, &'static str> {
    let mp = Arc::new(Mutex::new(PthMsgPort {
        mp_node: (),
        mp_name: name.map(|s| s.to_string()),
        mp_tid: (),
        mp_queue: LinkedList::new(),
    }));

    PTH_MSGPORT.lock().unwrap().push_back(mp.clone());
    Ok(mp)
}

pub fn pth_msgport_destroy(mp: Arc<Mutex<PthMsgPort>>) {
    let mut mp_guard = mp.lock().unwrap();
    
    while let Some(m) = pth_msgport_get(mp.clone()) {
        pth_msgport_reply(m).unwrap();
    }

    let mut port_guard = PTH_MSGPORT.lock().unwrap();
    port_guard.retain(|x| !Arc::ptr_eq(x, &mp));
}

pub fn pth_msgport_find(name: &str) -> Option<Arc<Mutex<PthMsgPort>>> {
    let port_guard = PTH_MSGPORT.lock().unwrap();
    port_guard.iter().find(|mp| {
        let mp_guard = mp.lock().unwrap();
        mp_guard.mp_name.as_ref().map_or(false, |n| n == name)
    }).cloned()
}

pub fn pth_msgport_pending(mp: Arc<Mutex<PthMsgPort>>) -> Result<usize, &'static str> {
    let mp_guard = mp.lock().unwrap();
    Ok(mp_guard.mp_queue.len())
}

pub fn pth_msgport_put(mp: Arc<Mutex<PthMsgPort>>, m: PthMessage) -> Result<(), &'static str> {
    let mut mp_guard = mp.lock().unwrap();
    mp_guard.mp_queue.push_back(m);
    Ok(())
}

pub fn pth_msgport_get(mp: Arc<Mutex<PthMsgPort>>) -> Option<PthMessage> {
    let mut mp_guard = mp.lock().unwrap();
    mp_guard.mp_queue.pop_front()
}

pub fn pth_msgport_reply(m: PthMessage) -> Result<(), &'static str> {
    // Assuming m has a reply_port field
    // pth_msgport_put(m.reply_port, m)
    Ok(())
}