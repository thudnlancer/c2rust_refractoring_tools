// qmd.rs (quotient minimum degree algorithm)

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2001 Free Software Foundation, Inc.
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

pub mod qmd {
    /// GENeral Quotient Minimum Degree algorithm
    pub fn genqmd(
        neqns: &mut i32,
        xadj: &mut [i32],
        adjncy: &mut [i32],
        perm: &mut [i32],
        invp: &mut [i32],
        deg: &mut [i32],
        marker: &mut [i32],
        rchset: &mut [i32],
        nbrhd: &mut [i32],
        qsize: &mut [i32],
        qlink: &mut [i32],
        nofsub: &mut i32,
    ) {
        panic!("genqmd: sorry, this routine is temporarily disabled due to licensing problems");
    }

    /// Quotient MD ReaCHable set
    pub fn qmdrch(
        root: &mut i32,
        xadj: &mut [i32],
        adjncy: &mut [i32],
        deg: &mut [i32],
        marker: &mut [i32],
        rchsze: &mut i32,
        rchset: &mut [i32],
        nhdsze: &mut i32,
        nbrhd: &mut [i32],
    ) {
        panic!("qmdrch: sorry, this routine is temporarily disabled due to licensing problems");
    }

    /// Quotient MD Quotient graph Transformation
    pub fn qmdqt(
        root: &mut i32,
        xadj: &mut [i32],
        adjncy: &mut [i32],
        marker: &mut [i32],
        rchsze: &mut i32,
        rchset: &mut [i32],
        nbrhd: &mut [i32],
    ) {
        panic!("qmdqt: sorry, this routine is temporarily disabled due to licensing problems");
    }

    /// Quotient MD UPDate
    pub fn qmdupd(
        xadj: &mut [i32],
        adjncy: &mut [i32],
        nlist: &mut i32,
        list: &mut [i32],
        deg: &mut [i32],
        qsize: &mut [i32],
        qlink: &mut [i32],
        marker: &mut [i32],
        rchset: &mut [i32],
        nbrhd: &mut [i32],
    ) {
        panic!("qmdupd: sorry, this routine is temporarily disabled due to licensing problems");
    }

    /// Quotient MD MeRGe
    pub fn qmdmrg(
        xadj: &mut [i32],
        adjncy: &mut [i32],
        deg: &mut [i32],
        qsize: &mut [i32],
        qlink: &mut [i32],
        marker: &mut [i32],
        deg0: &mut i32,
        nhdsze: &mut i32,
        nbrhd: &mut [i32],
        rchset: &mut [i32],
        ovrlp: &mut [i32],
    ) {
        panic!("qmdmrg: sorry, this routine is temporarily disabled due to licensing problems");
    }
}