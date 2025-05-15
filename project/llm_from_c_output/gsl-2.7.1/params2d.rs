// histogram/params2d.rs
//
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Brian Gough
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

pub struct Histogram2D {
    pub nx: usize,
    pub ny: usize,
    pub xrange: Vec<f64>,
    pub yrange: Vec<f64>,
}

impl Histogram2D {
    pub fn xmax(&self) -> f64 {
        self.xrange[self.nx]
    }

    pub fn xmin(&self) -> f64 {
        self.xrange[0]
    }

    pub fn ymax(&self) -> f64 {
        self.yrange[self.ny]
    }

    pub fn ymin(&self) -> f64 {
        self.yrange[0]
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }
}