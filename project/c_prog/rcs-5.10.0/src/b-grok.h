/* b-grok.h --- comma-v parsing

   Copyright (C) 2010-2020 Thien-Thi Nguyen

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

extern struct repo *empty_repo (struct divvy *to)
  ALL_NONNULL;
extern struct repo *grok_all (struct divvy *to, struct fro *f)
  ALL_NONNULL;
extern void grok_resynch (struct repo *repo)
  ALL_NONNULL;

/* b-grok.h ends here */
