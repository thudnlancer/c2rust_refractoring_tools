/* b-feph.h --- (possibly) temporary files

   Copyright (C) 2010-2020 Thien-Thi Nguyen
   Copyright (C) 1990, 1991, 1992, 1993, 1994, 1995 Paul Eggert
   Copyright (C) 1982, 1988, 1989 Walter Tichy

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

extern void init_ephemstuff (void);
extern char const *maketemp (int n);
extern char const *makedirtemp (bool isworkfile);
extern void keepdirtemp (char const *name)
  ALL_NONNULL;
extern void tempunlink (void);
extern void dirtempunlink (void);

/* b-feph.h ends here */
