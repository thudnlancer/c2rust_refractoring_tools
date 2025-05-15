/* relax4.rs */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2012-2013 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Relax4Error {
    details: String
}

impl Relax4Error {
    fn new(msg: &str) -> Relax4Error {
        Relax4Error{details: msg.to_string()}
    }
}

impl fmt::Display for Relax4Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for Relax4Error {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct Relax4Csa {
    // input parameters
    pub n: i32,
    pub na: i32,
    pub large: i32,
    pub repeat: bool,
    pub crash: i32,
    pub startn: Vec<i32>,
    pub endn: Vec<i32>,
    pub fou: Vec<i32>,
    pub nxtou: Vec<i32>,
    pub fin: Vec<i32>,
    pub nxtin: Vec<i32>,
    
    // updated parameters
    pub rc: Vec<i32>,
    pub u: Vec<i32>,
    pub dfct: Vec<i32>,
    
    // output parameters
    pub x: Vec<i32>,
    pub nmultinode: i32,
    pub iter: i32,
    pub num_augm: i32,
    pub num_ascnt: i32,
    pub nsp: i32,
    
    // working parameters
    pub label: Vec<i32>,
    pub prdcsr: Vec<i32>,
    pub save: Vec<i32>,
    pub tfstou: Vec<i32>,
    pub tnxtou: Vec<i32>,
    pub tfstin: Vec<i32>,
    pub tnxtin: Vec<i32>,
    pub nxtqueue: Vec<i32>,
    pub scan: Vec<bool>,
    pub mark: Vec<bool>,
    
    // working parameters used by routine auction only
    pub extend_arc: Vec<i32>,
    pub sb_level: Vec<i32>,
    pub sb_arc: Vec<i32>,
}

pub fn relax4(csa: &mut Relax4Csa) -> Result<i32, Relax4Error> {
    Err(Relax4Error::new("relax4: sorry, this routine is temporarily disabled due to licensing problems"))
}

pub fn relax4_inidat(csa: &mut Relax4Csa) -> Result<(), Relax4Error> {
    Err(Relax4Error::new("relax4_inidat: sorry, this routine is temporarily disabled due to licensing problems"))
}