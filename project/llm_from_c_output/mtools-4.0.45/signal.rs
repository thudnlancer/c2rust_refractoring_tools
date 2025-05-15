/*  Copyright 1996,1997,2001,2002,2007,2009 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::consts::signal::*;
use signal_hook::flag as signal_flag;
use signal_hook::iterator::Signals;
use std::io::Error;

static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);

pub struct SavedSigState {
    signals: Option<Signals>,
}

fn signal_handler() {
    GOT_SIGNAL.store(true, Ordering::Relaxed);
}

pub fn setup_signal() -> Result<(), Error> {
    let signals = Arc::new(GOT_SIGNAL.clone());
    
    #[cfg(feature = "sighup")]
    signal_flag::register(SIGHUP, signals.clone())?;
    
    #[cfg(feature = "sigint")]
    signal_flag::register(SIGINT, signals.clone())?;
    
    #[cfg(feature = "sigterm")]
    signal_flag::register(SIGTERM, signals.clone())?;
    
    #[cfg(feature = "sigquit")]
    signal_flag::register(SIGQUIT, signals)?;

    Ok(())
}

pub fn allow_interrupts() -> Result<SavedSigState, Error> {
    let mut signals = Signals::new(&[
        #[cfg(feature = "sighup")]
        SIGHUP,
        #[cfg(feature = "sigint")]
        SIGINT,
        #[cfg(feature = "sigterm")]
        SIGTERM,
        #[cfg(feature = "sigquit")]
        SIGQUIT,
    ])?;

    signals.set_handler(signal_handler)?;
    signals.forever()?;

    Ok(SavedSigState {
        signals: Some(signals),
    })
}

pub fn restore_interrupts(mut ss: SavedSigState) -> Result<(), Error> {
    if let Some(signals) = ss.signals.take() {
        signals.set_handler(|| {})?;
    }
    Ok(())
}

pub fn got_signal() -> bool {
    GOT_SIGNAL.load(Ordering::Relaxed)
}