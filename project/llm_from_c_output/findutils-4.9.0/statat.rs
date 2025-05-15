// Return info about a file at a directory.
//
// Copyright (C) 2012-2022 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(non_snake_case)]

// Note: In Rust, we don't need config.h as build configurations
// are typically handled through Cargo features or cfg attributes.
// The STATAT_INLINE macro is not needed in Rust as inline hints
// are provided through the #[inline] attribute.

// Rust equivalent would typically be in a separate module
pub mod openat {
    // Rust's equivalent of _GL_EXTERN_INLINE would be #[inline]
    // applied to functions as needed
}