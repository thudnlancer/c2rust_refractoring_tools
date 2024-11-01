# Generated automatically from Makefile.in by configure.
# Makefile.in generated automatically by automake 1.3 from Makefile.am

# Copyright (C) 1994, 1995, 1996, 1997, 1998 Free Software Foundation, Inc.
# This Makefile.in is free software; the Free Software Foundation
# gives unlimited permission to copy and/or distribute it,
# with or without modifications, as long as this notice is preserved.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY, to the extent permitted by law; without
# even the implied warranty of MERCHANTABILITY or FITNESS FOR A
# PARTICULAR PURPOSE.

#
# Makefile for enscript.
# Copyright (c) 1995 Markku Rossi
#
# Author: Markku Rossi <mtr@iki.fi>
#

#
# This file is part of GNU enscript.
# 
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2, or (at your option)
# any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; see the file COPYING.  If not, write to
# the Free Software Foundation, 59 Temple Place - Suite 330,
# Boston, MA 02111-1307, USA.
#


SHELL = /bin/sh

srcdir = .
top_srcdir = ..
prefix = /usr/local
exec_prefix = ${prefix}

bindir = ${exec_prefix}/bin
sbindir = ${exec_prefix}/sbin
libexecdir = ${exec_prefix}/libexec
datadir = ${prefix}/share
sysconfdir = ${prefix}/etc
sharedstatedir = ${prefix}/com
localstatedir = ${prefix}/var
libdir = ${exec_prefix}/lib
infodir = ${prefix}/info
mandir = ${prefix}/man
includedir = ${prefix}/include
oldincludedir = /usr/include

DISTDIR =

pkgdatadir = $(datadir)/enscript
pkglibdir = $(libdir)/enscript
pkgincludedir = $(includedir)/enscript

top_builddir = ..

ACLOCAL = /home/c2rust/Desktop/c_prog/enscript-1.6.1/missing aclocal
AUTOCONF = /home/c2rust/Desktop/c_prog/enscript-1.6.1/missing autoconf
AUTOMAKE = /home/c2rust/Desktop/c_prog/enscript-1.6.1/missing automake
AUTOHEADER = /home/c2rust/Desktop/c_prog/enscript-1.6.1/missing autoheader

INSTALL = /usr/bin/install -c
INSTALL_PROGRAM = ${INSTALL}
INSTALL_DATA = ${INSTALL} -m 644
INSTALL_SCRIPT = ${INSTALL_PROGRAM}
transform = s,x,x,

NORMAL_INSTALL = :
PRE_INSTALL = :
POST_INSTALL = :
NORMAL_UNINSTALL = :
PRE_UNINSTALL = :
POST_UNINSTALL = :
CATALOGS =  de.gmo es.gmo fi.gmo fr.gmo nl.gmo sl.gmo
CATOBJEXT = .gmo
CC = gcc
CFLAGS = -g -O2
CPP = gcc -E
DATADIRNAME = share
GENCAT = 
GMOFILES =  de.gmo es.gmo fi.gmo fr.gmo nl.gmo sl.gmo
GMSGFMT = /usr/bin/msgfmt
GT_NO = 
GT_YES = #YES#
INSTOBJEXT = .mo
INTLDEPS = 
INTLLIBS = 
INTLOBJS = 
LDFLAGS = 
LEX = flex
LIBOBJS = 
MAKEINFO = /home/c2rust/Desktop/c_prog/enscript-1.6.1/missing makeinfo
MEDIA = A4
MKINSTALLDIRS = ./mkinstalldirs
MSGFMT = /usr/bin/msgfmt
PACKAGE = enscript
PERL = /usr/bin/perl
POFILES =  de.po es.po fi.po fr.po nl.po sl.po
POSUB = po
PSLEVEL = 2
RANLIB = ranlib
SPOOLER = lpr
U = 
USE_INCLUDED_LIBINTL = no
USE_NLS = yes
VERSION = 1.6.1
YACC = yacc
l = 

AUTOMAKE_OPTIONS = ansi2knr

bin_PROGRAMS = enscript mkafmmap
enscript_SOURCES = main.c psgen.c util.c prt_lpr.c
mkafmmap_SOURCES = mkafmmap.c

LDADD = ../afmlib/libafm.a  ../compat/libcompat.a -lm
enscript_DEPENDENCIES = ../afmlib/libafm.a  ../compat/libcompat.a
mkafmmap_DEPENDENCIES = ../afmlib/libafm.a  ../compat/libcompat.a

noinst_HEADERS = gsint.h

localedir = $(prefix)/share/locale

DEFS = -DHAVE_CONFIG_H -DLOCALEDIR=\"$(localedir)\" -DLIBRARY=\"$(datadir)/enscript\" -DSYSCONFDIR=\"$(sysconfdir)\" -I. -I$(srcdir) -I..
INCLUDES = -I$(srcdir)/../compat -I$(srcdir)/../afmlib -I$(srcdir)/../intl -I../intl

EXTRA_DIST = prt_dos.c ansi2knr.c ansi2knr.1
mkinstalldirs = $(SHELL) $(top_srcdir)/mkinstalldirs
CONFIG_HEADER = ../config.h
CONFIG_CLEAN_FILES = 
PROGRAMS =  $(bin_PROGRAMS)

CPPFLAGS = 
LIBS = 
ANSI2KNR = 
enscript_OBJECTS =  main$U.o psgen$U.o util$U.o prt_lpr$U.o
enscript_LDADD = $(LDADD)
enscript_LDFLAGS = 
mkafmmap_OBJECTS =  mkafmmap$U.o
mkafmmap_LDADD = $(LDADD)
mkafmmap_LDFLAGS = 
COMPILE = $(CC) $(DEFS) $(INCLUDES) $(CPPFLAGS) $(CFLAGS)
LINK = $(CC) $(CFLAGS) $(LDFLAGS) -o $@
HEADERS =  $(noinst_HEADERS)

DIST_COMMON =  ChangeLog Makefile.am Makefile.in ansi2knr.1 ansi2knr.c


DISTFILES = $(DIST_COMMON) $(SOURCES) $(HEADERS) $(TEXINFOS) $(EXTRA_DIST)

TAR = gtar
GZIP = --best
SOURCES = $(enscript_SOURCES) $(mkafmmap_SOURCES)
OBJECTS = $(enscript_OBJECTS) $(mkafmmap_OBJECTS)

all: Makefile $(ANSI2KNR) $(PROGRAMS) $(HEADERS)

.SUFFIXES:
.SUFFIXES: .S .c .o .s
$(srcdir)/Makefile.in: Makefile.am $(top_srcdir)/configure.in $(ACLOCAL_M4)
	cd $(top_srcdir) && $(AUTOMAKE) --gnu --include-deps src/Makefile

Makefile: $(srcdir)/Makefile.in  $(top_builddir)/config.status
	cd $(top_builddir) \
	  && CONFIG_FILES=$(subdir)/$@ CONFIG_HEADERS= $(SHELL) ./config.status


mostlyclean-binPROGRAMS:

clean-binPROGRAMS:
	-test -z "$(bin_PROGRAMS)" || rm -f $(bin_PROGRAMS)

distclean-binPROGRAMS:

maintainer-clean-binPROGRAMS:

install-binPROGRAMS: $(bin_PROGRAMS)
	@$(NORMAL_INSTALL)
	$(mkinstalldirs) $(DESTDIR)$(bindir)
	@list='$(bin_PROGRAMS)'; for p in $$list; do \
	  if test -f $$p; then \
	    echo "  $(INSTALL_PROGRAM) $$p $(DESTDIR)$(bindir)/`echo $$p|sed '$(transform)'`"; \
	     $(INSTALL_PROGRAM) $$p $(DESTDIR)$(bindir)/`echo $$p|sed '$(transform)'`; \
	  else :; fi; \
	done

uninstall-binPROGRAMS:
	@$(NORMAL_UNINSTALL)
	list='$(bin_PROGRAMS)'; for p in $$list; do \
	  rm -f $(DESTDIR)$(bindir)/`echo $$p|sed '$(transform)'`; \
	done

.c.o:
	$(COMPILE) -c $<

.s.o:
	$(COMPILE) -c $<

.S.o:
	$(COMPILE) -c $<

mostlyclean-compile:
	-rm -f *.o core *.core

clean-compile:

distclean-compile:
	-rm -f *.tab.c

maintainer-clean-compile:

mostlyclean-krextra:

clean-krextra:
	-rm -f ansi2knr

distclean-krextra:

maintainer-clean-krextra:
ansi2knr: ansi2knr.o
	$(LINK) ansi2knr.o $(LIBS)
ansi2knr.o: $(CONFIG_HEADER)


mostlyclean-kr:
	-rm -f *_.c

clean-kr:

distclean-kr:

maintainer-clean-kr:

enscript: $(enscript_OBJECTS) $(enscript_DEPENDENCIES)
	@rm -f enscript
	$(LINK) $(enscript_LDFLAGS) $(enscript_OBJECTS) $(enscript_LDADD) $(LIBS)

mkafmmap: $(mkafmmap_OBJECTS) $(mkafmmap_DEPENDENCIES)
	@rm -f mkafmmap
	$(LINK) $(mkafmmap_LDFLAGS) $(mkafmmap_OBJECTS) $(mkafmmap_LDADD) $(LIBS)
main_.c: main.c $(ANSI2KNR)
	$(CPP) $(DEFS) $(INCLUDES) $(CPPFLAGS) `if test -f $(srcdir)/main.c; then echo $(srcdir)/main.c; else echo main.c; fi` | $(ANSI2KNR) main_.c
mkafmmap_.c: mkafmmap.c $(ANSI2KNR)
	$(CPP) $(DEFS) $(INCLUDES) $(CPPFLAGS) `if test -f $(srcdir)/mkafmmap.c; then echo $(srcdir)/mkafmmap.c; else echo mkafmmap.c; fi` | $(ANSI2KNR) mkafmmap_.c
prt_lpr_.c: prt_lpr.c $(ANSI2KNR)
	$(CPP) $(DEFS) $(INCLUDES) $(CPPFLAGS) `if test -f $(srcdir)/prt_lpr.c; then echo $(srcdir)/prt_lpr.c; else echo prt_lpr.c; fi` | $(ANSI2KNR) prt_lpr_.c
psgen_.c: psgen.c $(ANSI2KNR)
	$(CPP) $(DEFS) $(INCLUDES) $(CPPFLAGS) `if test -f $(srcdir)/psgen.c; then echo $(srcdir)/psgen.c; else echo psgen.c; fi` | $(ANSI2KNR) psgen_.c
util_.c: util.c $(ANSI2KNR)
	$(CPP) $(DEFS) $(INCLUDES) $(CPPFLAGS) `if test -f $(srcdir)/util.c; then echo $(srcdir)/util.c; else echo util.c; fi` | $(ANSI2KNR) util_.c
main_.o mkafmmap_.o prt_lpr_.o psgen_.o util_.o : $(ANSI2KNR)

tags: TAGS

ID: $(HEADERS) $(SOURCES) $(LISP)
	here=`pwd` && cd $(srcdir) \
	  && mkid -f$$here/ID $(SOURCES) $(HEADERS) $(LISP)

TAGS:  $(HEADERS) $(SOURCES)  $(TAGS_DEPENDENCIES) $(LISP)
	tags=; \
	here=`pwd`; \
	list='$(SOURCES) $(HEADERS)'; \
	unique=`for i in $$list; do echo $$i; done | \
	  awk '    { files[$$0] = 1; } \
	       END { for (i in files) print i; }'`; \
	test -z "$(ETAGS_ARGS)$$unique$(LISP)$$tags" \
	  || (cd $(srcdir) && etags $(ETAGS_ARGS) $$tags  $$unique $(LISP) -o $$here/TAGS)

mostlyclean-tags:

clean-tags:

distclean-tags:
	-rm -f TAGS ID

maintainer-clean-tags:

distdir = $(top_builddir)/$(PACKAGE)-$(VERSION)/$(subdir)

subdir = src

distdir: $(DISTFILES)
	@for file in $(DISTFILES); do \
	  d=$(srcdir); \
	  test -f $(distdir)/$$file \
	  || ln $$d/$$file $(distdir)/$$file 2> /dev/null \
	  || cp -p $$d/$$file $(distdir)/$$file; \
	done
main.o: main.c gsint.h ../config.h ../afmlib/afm.h ../afmlib/strhash.h \
	../compat/xalloc.h ../compat/getopt.h
mkafmmap.o: mkafmmap.c ../config.h ../afmlib/afm.h ../compat/getopt.h
prt_lpr.o: prt_lpr.c gsint.h ../config.h ../afmlib/afm.h \
	../afmlib/strhash.h ../compat/xalloc.h
psgen.o: psgen.c gsint.h ../config.h ../afmlib/afm.h ../afmlib/strhash.h \
	../compat/xalloc.h
util.o: util.c gsint.h ../config.h ../afmlib/afm.h ../afmlib/strhash.h \
	../compat/xalloc.h

info:
dvi:
check: all
	$(MAKE)
installcheck:
install-exec: install-binPROGRAMS
	@$(NORMAL_INSTALL)

install-data: 
	@$(NORMAL_INSTALL)

install: install-exec install-data all
	@:

uninstall: uninstall-binPROGRAMS

install-strip:
	$(MAKE) INSTALL_PROGRAM='$(INSTALL_PROGRAM) -s' INSTALL_SCRIPT='$(INSTALL_PROGRAM)' install
installdirs:
	$(mkinstalldirs)  $(DATADIR)$(bindir)


mostlyclean-generic:
	-test -z "$(MOSTLYCLEANFILES)" || rm -f $(MOSTLYCLEANFILES)

clean-generic:
	-test -z "$(CLEANFILES)" || rm -f $(CLEANFILES)

distclean-generic:
	-rm -f Makefile $(DISTCLEANFILES)
	-rm -f config.cache config.log stamp-h stamp-h[0-9]*
	-test -z "$(CONFIG_CLEAN_FILES)" || rm -f $(CONFIG_CLEAN_FILES)

maintainer-clean-generic:
	-test -z "$(MAINTAINERCLEANFILES)" || rm -f $(MAINTAINERCLEANFILES)
	-test -z "$(BUILT_SOURCES)" || rm -f $(BUILT_SOURCES)
mostlyclean:  mostlyclean-binPROGRAMS mostlyclean-compile \
		mostlyclean-krextra mostlyclean-kr mostlyclean-tags \
		mostlyclean-generic

clean:  clean-binPROGRAMS clean-compile clean-krextra clean-kr \
		clean-tags clean-generic mostlyclean

distclean:  distclean-binPROGRAMS distclean-compile distclean-krextra \
		distclean-kr distclean-tags distclean-generic clean
	-rm -f config.status

maintainer-clean:  maintainer-clean-binPROGRAMS maintainer-clean-compile \
		maintainer-clean-krextra maintainer-clean-kr \
		maintainer-clean-tags maintainer-clean-generic \
		distclean
	@echo "This command is intended for maintainers to use;"
	@echo "it deletes files that may require special tools to rebuild."

.PHONY: mostlyclean-binPROGRAMS distclean-binPROGRAMS clean-binPROGRAMS \
maintainer-clean-binPROGRAMS uninstall-binPROGRAMS install-binPROGRAMS \
mostlyclean-compile distclean-compile clean-compile \
maintainer-clean-compile mostlyclean-krextra distclean-krextra \
clean-krextra maintainer-clean-krextra mostlyclean-kr distclean-kr \
clean-kr maintainer-clean-kr tags mostlyclean-tags distclean-tags \
clean-tags maintainer-clean-tags distdir info dvi installcheck \
install-exec install-data install uninstall all installdirs \
mostlyclean-generic distclean-generic clean-generic \
maintainer-clean-generic clean mostlyclean distclean maintainer-clean


# Tell versions [3.59,3.63) of GNU make to not export all variables.
# Otherwise a system limit (for SysV at least) may be exceeded.
.NOEXPORT:
