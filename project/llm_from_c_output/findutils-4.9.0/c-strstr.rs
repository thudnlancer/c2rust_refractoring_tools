/* Searching in a string.
   Copyright (C) 2001-2003, 2006, 2009-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* The functions defined in this file assume a nearly ASCII compatible
   character set.  */

/* Find the first occurrence of NEEDLE in HAYSTACK.
   This function is safe to be called, even in a multibyte locale, if NEEDLE
     1. consists solely of printable ASCII characters excluding '\\' and '~'
        [this restriction is needed because of Shift_JIS and JOHAB]
        or of the control ASCII characters '\a' '\b' '\f' '\n' '\r' '\t' '\v'
        [this restriction is needed because of VISCII], and
     2. has at least length 2
        [this restriction is needed because of BIG5, BIG5-HKSCS, GBK, GB18030,
         Shift_JIS, JOHAB], and
     3. does not consist entirely of decimal digits, or has at least length 4
        [this restriction is needed because of GB18030].
   This function is also safe to be called, even in a multibyte locale, if
   HAYSTACK and NEEDLE are known to both consist solely of printable ASCII
   characters excluding '\\' and '~'.  */
pub fn c_strstr(haystack: &str, needle: &str) -> Option<&str> {
    haystack.find(needle).map(|pos| &haystack[pos..])
}

/* c-strstr.c -- substring search in C locale
   Copyright (C) 2005-2022 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2005, 2007.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* POSIX says that strstr() interprets the strings as byte sequences, not
   as character sequences in the current locale.  */