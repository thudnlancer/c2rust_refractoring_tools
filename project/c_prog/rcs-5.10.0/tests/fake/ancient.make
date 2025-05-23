#!/bin/sh
# ancient.make --- make ancient

# Copyright (C) 2018-2020 Thien-Thi Nguyen
#
# This file is part of GNU RCS.
#
# GNU RCS is free software: you can redistribute it and/or modify it
# under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# GNU RCS is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty
# of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
# See the GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

rm -f ancient ancient,v
date='1990-03-30 10:11:12 +00'
echo extol > ancient
touch -d "$date" ancient
rcs -i -t-'YY-only!' ancient
ci -d"$date" -mogre -sOld ancient
mv ancient,v ancient
exit 0

# ancient.make ends here
