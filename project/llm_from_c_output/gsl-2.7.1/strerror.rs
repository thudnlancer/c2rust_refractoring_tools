// err/strerror.rs
//
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Gerard Jungman, Brian Gough
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

use gsl_errno::*;

pub fn gsl_strerror(gsl_errno: i32) -> &'static str {
    match gsl_errno {
        GSL_SUCCESS => "success",
        GSL_FAILURE => "failure",
        GSL_CONTINUE => "the iteration has not converged yet",
        GSL_EDOM => "input domain error",
        GSL_ERANGE => "output range error",
        GSL_EFAULT => "invalid pointer",
        GSL_EINVAL => "invalid argument supplied by user",
        GSL_EFAILED => "generic failure",
        GSL_EFACTOR => "factorization failed",
        GSL_ESANITY => "sanity check failed - shouldn't happen",
        GSL_ENOMEM => "malloc failed",
        GSL_EBADFUNC => "problem with user-supplied function",
        GSL_ERUNAWAY => "iterative process is out of control",
        GSL_EMAXITER => "exceeded max number of iterations",
        GSL_EZERODIV => "tried to divide by zero",
        GSL_EBADTOL => "specified tolerance is invalid or theoretically unattainable",
        GSL_ETOL => "failed to reach the specified tolerance",
        GSL_EUNDRFLW => "underflow",
        GSL_EOVRFLW => "overflow",
        GSL_ELOSS => "loss of accuracy",
        GSL_EROUND => "roundoff error",
        GSL_EBADLEN => "matrix/vector sizes are not conformant",
        GSL_ENOTSQR => "matrix not square",
        GSL_ESING => "singularity or extremely bad function behavior detected",
        GSL_EDIVERGE => "integral or series is divergent",
        GSL_EUNSUP => "the required feature is not supported by this hardware platform",
        GSL_EUNIMPL => "the requested feature is not (yet) implemented",
        GSL_ECACHE => "cache limit exceeded",
        GSL_ETABLE => "table limit exceeded",
        GSL_ENOPROG => "iteration is not making progress towards solution",
        GSL_ENOPROGJ => "jacobian evaluations are not improving the solution",
        GSL_ETOLF => "cannot reach the specified tolerance in F",
        GSL_ETOLX => "cannot reach the specified tolerance in X",
        GSL_ETOLG => "cannot reach the specified tolerance in gradient",
        GSL_EOF => "end of file",
        _ => "unknown error code",
    }
}