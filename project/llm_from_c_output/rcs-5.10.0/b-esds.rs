// b-esds.rs --- embarrassingly simple data structures

// Copyright (C) 2010-2020 Thien-Thi Nguyen
//
// This file is part of GNU RCS.
//
// GNU RCS is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// GNU RCS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Link<T> {
    pub entry: Rc<T>,
    pub next: Option<Rc<RefCell<Link<T>>>>,
}

#[derive(Debug)]
pub struct WLink<T> {
    pub entry: Rc<RefCell<T>>,
    pub next: Option<Rc<RefCell<WLink<T>>>>,
}

pub struct Divvy; // Placeholder for allocation context

impl<T> Link<T> {
    pub fn extend(tp: Rc<RefCell<Self>>, x: T, _to: &Divvy) -> Rc<RefCell<Self>> {
        let pair = Rc::new(RefCell::new(Link {
            entry: Rc::new(x),
            next: None,
        }));
        tp.borrow_mut().next = Some(pair.clone());
        pair
    }

    pub fn prepend(x: T, ls: Option<Rc<RefCell<Self>>, _to: &Divvy) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Link {
            entry: Rc::new(x),
            next: ls,
        }))
    }
}

impl<T> WLink<T> {
    pub fn wextend(tp: Rc<RefCell<Self>>, x: T, _to: &Divvy) -> Rc<RefCell<Self>> {
        let pair = Rc::new(RefCell::new(WLink {
            entry: Rc::new(RefCell::new(x)),
            next: None,
        }));
        tp.borrow_mut().next = Some(pair.clone());
        pair
    }

    pub fn wprepend(x: T, ls: Option<Rc<RefCell<Self>>, _to: &Divvy) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(WLink {
            entry: Rc::new(RefCell::new(x)),
            next: ls,
        }))
    }
}

// b-esds.rs ends here