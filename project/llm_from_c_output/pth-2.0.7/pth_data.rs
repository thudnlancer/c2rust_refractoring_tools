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
**  pth_data.c: Pth per-thread specific data
*/
                             /* ``Breakthrough ideas
                                  are not from teams.''
                                       --- Hans von Ohain */

use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

const PTH_KEY_MAX: usize = 1024;
const PTH_DESTRUCTOR_ITERATIONS: usize = 4;

struct PthKeytab {
    used: AtomicBool,
    destructor: Mutex<Option<Box<dyn FnMut(*mut ()) + Send + Sync>>>,
}

struct ThreadData {
    data_value: Mutex<HashMap<usize, *mut ()>>,
    data_count: Mutex<usize>,
}

lazy_static::lazy_static! {
    static ref PTH_KEYTAB: Vec<PthKeytab> = {
        let mut v = Vec::with_capacity(PTH_KEY_MAX);
        for _ in 0..PTH_KEY_MAX {
            v.push(PthKeytab {
                used: AtomicBool::new(false),
                destructor: Mutex::new(None),
            });
        }
        v
    };
    static ref CURRENT_THREAD: Mutex<ThreadData> = Mutex::new(ThreadData {
        data_value: Mutex::new(HashMap::new()),
        data_count: Mutex::new(0),
    });
}

pub type PthKeyT = usize;

pub fn pth_key_create(key: &mut PthKeyT, func: Option<Box<dyn FnMut(*mut ()) + Send + Sync>>) -> Result<(), i32> {
    if key.is_null() {
        return Err(libc::EINVAL);
    }

    for k in 0..PTH_KEY_MAX {
        if let Ok(false) = PTH_KEYTAB[k].used.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
            *key = k;
            *PTH_KEYTAB[k].destructor.lock().unwrap() = func;
            return Ok(());
        }
    }

    Err(libc::EAGAIN)
}

pub fn pth_key_delete(key: PthKeyT) -> Result<(), i32> {
    if key >= PTH_KEY_MAX {
        return Err(libc::EINVAL);
    }

    if !PTH_KEYTAB[key].used.swap(false, Ordering::SeqCst) {
        return Err(libc::ENOENT);
    }

    *PTH_KEYTAB[key].destructor.lock().unwrap() = None;
    Ok(())
}

pub fn pth_key_setdata(key: PthKeyT, value: Option<*mut ()>) -> Result<(), i32> {
    if key >= PTH_KEY_MAX {
        return Err(libc::EINVAL);
    }

    if !PTH_KEYTAB[key].used.load(Ordering::SeqCst) {
        return Err(libc::ENOENT);
    }

    let current = CURRENT_THREAD.lock().unwrap();
    let mut data_value = current.data_value.lock().unwrap();
    let mut data_count = current.data_count.lock().unwrap();

    match (data_value.get(&key).copied(), value) {
        (None, Some(_)) => *data_count += 1,
        (Some(_), None) => *data_count -= 1,
        _ => (),
    }

    if let Some(v) = value {
        data_value.insert(key, v);
    } else {
        data_value.remove(&key);
    }

    Ok(())
}

pub fn pth_key_getdata(key: PthKeyT) -> Result<*mut (), i32> {
    if key >= PTH_KEY_MAX {
        return Err(libc::EINVAL);
    }

    if !PTH_KEYTAB[key].used.load(Ordering::SeqCst) {
        return Err(libc::ENOENT);
    }

    let current = CURRENT_THREAD.lock().unwrap();
    let data_value = current.data_value.lock().unwrap();
    Ok(*data_value.get(&key).unwrap_or(&ptr::null_mut()))
}

pub fn pth_key_destroydata() {
    let current = CURRENT_THREAD.lock().unwrap();
    let mut data_value = current.data_value.lock().unwrap();
    let mut data_count = current.data_count.lock().unwrap();

    if *data_count == 0 {
        return;
    }

    for _ in 0..PTH_DESTRUCTOR_ITERATIONS {
        let mut keys_to_remove = Vec::new();
        let mut destructors = Vec::new();

        for (key, value) in data_value.iter() {
            if *value != ptr::null_mut() && PTH_KEYTAB[*key].used.load(Ordering::SeqCst) {
                if let Some(destructor) = PTH_KEYTAB[*key].destructor.lock().unwrap().as_mut() {
                    destructors.push((destructor, *value));
                    keys_to_remove.push(*key);
                }
            }
        }

        for key in keys_to_remove {
            data_value.remove(&key);
            *data_count -= 1;
        }

        for (destructor, value) in destructors {
            destructor(value);
        }

        if *data_count == 0 {
            break;
        }
    }

    data_value.clear();
    *data_count = 0;
}