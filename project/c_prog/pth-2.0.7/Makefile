##
##  GNU Pth - The GNU Portable Threads
##  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
##
##  This file is part of GNU Pth, a non-preemptive thread scheduling
##  library which can be found at http://www.gnu.org/software/pth/.
##
##  This library is free software; you can redistribute it and/or
##  modify it under the terms of the GNU Lesser General Public
##  License as published by the Free Software Foundation; either
##  version 2.1 of the License, or (at your option) any later version.
##
##  This library is distributed in the hope that it will be useful,
##  but WITHOUT ANY WARRANTY; without even the implied warranty of
##  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
##  Lesser General Public License for more details.
##
##  You should have received a copy of the GNU Lesser General Public
##  License along with this library; if not, write to the Free Software
##  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
##  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
##
##  Makefile.in: Pth Makefile input
##
                             # ``UNIX -- where you can do anything
                             #   in two keystrokes, or less...''


##
##  ____ PATH DEFINITIONS ___________________________________________
##


srcdir      = .
top_srcdir  = .

S           = 
C           = ./

prefix      = /usr/local
exec_prefix = ${prefix}
bindir      = ${exec_prefix}/bin
libdir      = ${exec_prefix}/lib
includedir  = ${prefix}/include
mandir      = ${prefix}/man
datadir     = ${prefix}/share

DESTDIR     =

##
##  ____ TOOL DEFINITIONS ___________________________________________
##

SHELL       = /bin/sh
CC          = gcc
CPPFLAGS    =  -I.
CFLAGS      = -O2 -pipe
LDFLAGS     = 
MKFLAGS     = $(MFLAGS) DESTDIR=$(DESTDIR)
LIBS        = -ldl -lnsl 
SHTOOL      = $(srcdir)/shtool
LIBTOOL     = $(C)libtool
RM          = rm -f
RMDIR       = rmdir
TRUE        = true
CAT         = cat
CP          = cp
SED         = sed
ECHO        = echo
GZIP        = gzip
GTAR        = gtar
TARDY       = tardy
POD2MAN     = pod2man
GDB         = gdb
PERL        = perl

##
##  ____ TARGET DEFINITIONS _________________________________________
##

BATCH            = no
PLATFORM         = x86_64-unknown-linux-gnu
PTH_MCTX_ID      = mcsc/sc/mc
PTH_STACK_GROWTH = down

TARGET_ALL  = $(TARGET_PREQ) $(TARGET_LIBS) $(TARGET_TEST)
TARGET_PREQ = pth_p.h
TARGET_LIBS = libpth.la 
TARGET_MANS = $(S)pth-config.1 $(S)pth.3  
TARGET_TEST = test_std test_mp test_misc test_philo test_sig \
              test_select test_httpd test_sfio test_uctx 

#   object files for library generation
#   (order is just aesthetically important)
LOBJS = pth_debug.lo pth_ring.lo pth_pqueue.lo pth_time.lo pth_errno.lo pth_mctx.lo \
        pth_uctx.lo pth_tcb.lo pth_sched.lo pth_attr.lo pth_lib.lo pth_event.lo \
        pth_data.lo pth_clean.lo pth_cancel.lo pth_msg.lo pth_sync.lo pth_fork.lo \
        pth_util.lo pth_high.lo pth_syscall.lo pth_ext.lo pth_compat.lo pth_string.lo

#   source files for header generation
#   (order is important and has to follow dependencies in pth_p.h)
HSRCS = $(S)pth_compat.c $(S)pth_debug.c $(S)pth_syscall.c $(S)pth_errno.c $(S)pth_ring.c $(S)pth_mctx.c \
        $(S)pth_uctx.c $(S)pth_clean.c $(S)pth_time.c $(S)pth_tcb.c $(S)pth_util.c $(S)pth_pqueue.c $(S)pth_event.c \
        $(S)pth_sched.c $(S)pth_data.c $(S)pth_msg.c $(S)pth_cancel.c $(S)pth_sync.c $(S)pth_attr.c $(S)pth_lib.c \
        $(S)pth_fork.c $(S)pth_high.c $(S)pth_ext.c $(S)pth_string.c $(S)pthread.c

##
##  ____ UTILITY DEFINITIONS _________________________________________
##

_VERSION_FILE = \
	$(S)pth_vers.c

_VERSION = \
	echo "updating package version"; \
	$(SHTOOL) version -lc -n 'GNU Pth' -p pth_internal_ $$OPT $(_VERSION_FILE); \
	V=`$(SHTOOL) version -lc -dlong $(_VERSION_FILE)`; \
	$(SED) -e "s/Version .*(.*)/Version $$V/g" <README >README.n && mv README.n README; \
	V=`$(SHTOOL) version -lc -dshort $(_VERSION_FILE)`; \
	$(SED) -e "s/%define ver .*/%define ver $$V/g" <pth.spec >pth.spec.n && mv pth.spec.n pth.spec

_TARBALL = \
	$(SHTOOL) tarball -o $${NAME}.tar.gz -d $${NAME} -u gnu -g pth \
	                  -e 'CVS,\.cvsignore,\.configure,\.\#.*,\.[ao]$$' -c 'gzip --best' .

_MANPAGE = \
	echo "Formatting $${BASENAME}.pod into $${BASENAME}.$${SEC} via $(POD2MAN)"; \
	V1=`$(SHTOOL) version -lc -dshort $(_VERSION_FILE)`; \
	V2=`$(SHTOOL) version -lc -dlong $(_VERSION_FILE)`; \
	D=`$(SHTOOL) version -lc -dlong $(_VERSION_FILE) |\
	   $(SED) -e 's;.*(;;' -e 's;).*;;'`; \
	$(POD2MAN) --section=$${SEC} --center="$${ONELINE}" \
	           --release="$$D" --date="$${NAME} $$V1" $(S)$${BASENAME}.pod |\
	$(SED) -e "s;PTH_VERSION_STR;$$V2;" >$${BASENAME}.$${SEC}

_DEBUG = \
	core=""; \
	if [ -f $$TEST.core ]; then \
	    core="$$TEST.core"; \
    elif [ -f core ]; then \
		core="core"; \
    fi; \
	$(LIBTOOL) --mode=execute $(GDB) $$TEST $$core

##
##  ____ END USER TARGETS ____________________________________________
##

#   be aware of libtool when building the objects
.SUFFIXES:
.SUFFIXES: .c .o .lo
.c.o:
	$(CC) -c $(CPPFLAGS) $(CFLAGS) $<
.c.lo:
	$(LIBTOOL) --mode=compile --quiet $(CC) -c $(CPPFLAGS) $(CFLAGS) $<

#   the default target
all: all-normal
all-normal: $(TARGET_ALL)
	@touch .done-all >/dev/null 2>&1 || $(TRUE)
	@$(MAKE) $(MKFLAGS) what-next
all-for-install: $(TARGET_ALL)
	@touch .done-all >/dev/null 2>&1 || $(TRUE)

#   build the private shared header file
pth_p.h: $(S)pth_p.h.in
	$(SHTOOL) scpp -o pth_p.h -t $(S)pth_p.h.in -Dcpp -Cintern -M '==#==' $(HSRCS)

#   build the static and possibly shared libraries
libpth.la: $(LOBJS)
	$(LIBTOOL) --mode=link --quiet $(CC) -o libpth.la $(LOBJS) \
	-rpath $(libdir) -version-info `$(SHTOOL) version -lc -dlibtool $(_VERSION_FILE)`
libpthread.la: pthread.lo $(LOBJS)
	$(LIBTOOL) --mode=link --quiet $(CC) -o libpthread.la pthread.lo $(LOBJS) \
	-rpath $(libdir) -version-info `$(SHTOOL) version -lc -dlibtool $(_VERSION_FILE)`

#   build the manual pages
$(S)pth-config.1: $(S)pth-config.pod $(_VERSION_FILE)
	@BASENAME="pth-config"; SEC=1; \
	NAME="GNU Pth"; ONELINE="GNU Portable Threads"; \
	$(_MANPAGE)
$(S)pth.3: $(S)pth.pod $(_VERSION_FILE)
	@BASENAME="pth"; SEC=3; \
	NAME="GNU Pth"; ONELINE="GNU Portable Threads"; \
	$(_MANPAGE)
$(S)pthread-config.1: $(S)pthread-config.pod $(_VERSION_FILE)
	@BASENAME="pthread-config"; SEC=1; \
	NAME="GNU Pth"; ONELINE="POSIX Threading API of GNU Pth"; \
	$(_MANPAGE)
$(S)pthread.3: $(S)pthread.pod $(_VERSION_FILE)
	@BASENAME="pthread"; SEC=3; \
	NAME="GNU Pth"; ONELINE="POSIX Threading API of GNU Pth"; \
	$(_MANPAGE)

#   build test program
test_std: test_std.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_std test_std.o test_common.o libpth.la $(LIBS)
test_httpd: test_httpd.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_httpd test_httpd.o test_common.o libpth.la $(LIBS)
test_misc: test_misc.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_misc test_misc.o test_common.o libpth.la $(LIBS)
test_mp: test_mp.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_mp test_mp.o test_common.o libpth.la $(LIBS)
test_philo: test_philo.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_philo test_philo.o test_common.o libpth.la $(LIBS)
test_sig: test_sig.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_sig test_sig.o test_common.o libpth.la $(LIBS)
test_select: test_select.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_select test_select.o test_common.o libpth.la $(LIBS)
test_sfio: test_sfio.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_sfio test_sfio.o test_common.o libpth.la $(LIBS)
test_uctx: test_uctx.o test_common.o libpth.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_uctx test_uctx.o test_common.o libpth.la $(LIBS)
test_pthread: test_pthread.o test_common.o libpthread.la
	$(LIBTOOL) --mode=link --quiet $(CC) $(LDFLAGS) -o test_pthread test_pthread.o test_common.o libpthread.la $(LIBS)

#   install the package
install: all-for-install
	@$(MAKE) $(MKFLAGS) install-dirs install-pth 
	@touch .done-install >/dev/null 2>&1 || $(TRUE)
	@$(MAKE) $(MKFLAGS) what-next
install-dirs:
	$(SHTOOL) mkdir -f -p -m 755 $(DESTDIR)$(prefix)
	$(SHTOOL) mkdir -f -p -m 755 $(DESTDIR)$(bindir)
	$(SHTOOL) mkdir -f -p -m 755 $(DESTDIR)$(includedir)
	$(SHTOOL) mkdir -f -p -m 755 $(DESTDIR)$(libdir)
	$(SHTOOL) mkdir -f -p -m 755 $(DESTDIR)$(mandir)/man1
	$(SHTOOL) mkdir -f -p -m 755 $(DESTDIR)$(mandir)/man3
	$(SHTOOL) mkdir -f -p -m 755 $(DESTDIR)$(datadir)/aclocal
install-pth:
	$(SHTOOL) install -c -m 755 pth-config $(DESTDIR)$(bindir)/pth-config
	$(SHTOOL) install -c -m 644 $(S)pth-config.1 $(DESTDIR)$(mandir)/man1/pth-config.1
	$(SHTOOL) install -c -m 644 $(S)pth.3 $(DESTDIR)$(mandir)/man3/pth.3
	$(SHTOOL) install -c -m 644 $(S)pth.m4 $(DESTDIR)$(datadir)/aclocal/pth.m4
	$(SHTOOL) install -c -m 644 pth.h $(DESTDIR)$(includedir)/pth.h
	@umask 022; $(LIBTOOL) --mode=install \
	$(SHTOOL) install -c libpth.la $(DESTDIR)$(libdir)/libpth.la
install-pthread:
	$(SHTOOL) install -c -m 755 pthread-config $(DESTDIR)$(bindir)/pthread-config
	$(SHTOOL) install -c -m 644 $(S)pthread-config.1 $(DESTDIR)$(mandir)/man1/pthread-config.1
	$(SHTOOL) install -c -m 644 pthread.h $(DESTDIR)$(includedir)/pthread.h
	$(SHTOOL) install -c -m 644 $(S)pthread.3 $(DESTDIR)$(mandir)/man3/pthread.3
	@umask 022; $(LIBTOOL) --mode=install \
	$(SHTOOL) install -c libpthread.la $(DESTDIR)$(libdir)/libpthread.la

#   uninstall the package
uninstall:
	@$(MAKE) $(MKFLAGS)  uninstall-pth uninstall-dirs
	@touch .done-uninstall >/dev/null 2>&1 || $(TRUE)
	@$(MAKE) $(MKFLAGS) what-next
uninstall-pthread:
	$(RM) $(DESTDIR)$(bindir)/pthread-config
	$(RM) $(DESTDIR)$(mandir)/man1/pthread-config.1
	$(RM) $(DESTDIR)$(includedir)/pthread.h
	$(RM) $(DESTDIR)$(mandir)/man3/pthread.3
	@$(LIBTOOL) --mode=uninstall $(RM) $(DESTDIR)$(libdir)/libpthread.la
uninstall-pth:
	$(RM) $(DESTDIR)$(bindir)/pth-config
	$(RM) $(DESTDIR)$(mandir)/man1/pth-config.1
	$(RM) $(DESTDIR)$(mandir)/man3/pth.3
	$(RM) $(DESTDIR)$(datadir)/aclocal/pth.m4
	$(RM) $(DESTDIR)$(includedir)/pth.h
	@$(LIBTOOL) --mode=uninstall $(RM) $(DESTDIR)$(libdir)/libpth.la
uninstall-dirs:
	-$(RMDIR) $(DESTDIR)$(datadir)/aclocal >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(datadir) >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(mandir)/man1 >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(mandir)/man3 >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(mandir) >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(libdir) >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(includedir) >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(bindir) >/dev/null 2>&1 || $(TRUE)
	-$(RMDIR) $(DESTDIR)$(prefix) >/dev/null 2>&1 || $(TRUE)

#   strip down the source tree to its minimum
striptease:
	@PERL=`which perl`; if [ ".$$PERL" = . ]; then \
	    echo "make:ERROR: command \"$(MAKE) striptease\" requires \"perl\"" 1>&2; \
		exit 1; \
	fi; \
	$$PERL striptease.pl

#   clean the source tree for re-building
clean:
	$(RM) $(TARGET_PREQ)
	$(RM) $(TARGET_TEST)
	$(RM) $(TARGET_LIBS)
	$(RM) *.o *.lo
	$(RM) .libs/*
	-$(RMDIR) .libs >/dev/null 2>&1 || $(TRUE)
	$(RM) core *.core
	$(RM) .done-*

#   clean the source tree for re-configuring and re-building
distclean: clean
	$(RM) Makefile
	$(RM) pth-config pth_acdef.h pth_acmac.h pth.h
	$(RM) pthread-config pthread.h
	$(RM) config.log config.cache config.status
	$(RM) libtool
	$(RM) *.bak *~

#   clean the source tree by removing really all generate stuff
realclean: distclean
	$(RM) $(TARGET_MANS)
	./devtool autoclean

#   give hints what to do as the next step
what-next:
	-@if [ ".$(BATCH)" != .yes ]; then \
	    if [ ".$(WHEN)" != . ]; then \
	        when="$(WHEN)"; \
	    else \
	        when="Now"; \
	    fi; \
	    echo ""; \
	    if [ ! -f .done-all ]; then \
	        cmd=`$(SHTOOL) echo -e "%Bmake%b"`; \
	        echo "$$when please type \`$$cmd' to compile. Good luck."; \
	    elif [ ! -f .done-test ]; then \
	        cmd=`$(SHTOOL) echo -e "%Bmake test%b"`; \
	        echo "$$when please type \`$$cmd' to run a quick test suite. Hope it works."; \
	    elif [ ! -f .done-install ]; then \
	        cmd=`$(SHTOOL) echo -e "%Bmake install%b"`; \
	        echo "$$when please type \`$$cmd' to install the package."; \
	    elif [ ! -f .done-uninstall ]; then \
	        cmd=`$(SHTOOL) echo -e "%Bmake uninstall%b"`; \
	        echo "$$when use the package (you can remove it later via \`$$cmd')."; \
	        cmd=`$(SHTOOL) echo -e "%Becho \"subscribe pth-users\" | mail pth-users-request@gnu.org%b"`; \
	        echo "Additionally you should consider subscribing to the GNU Pth support"; \
	        echo "mailing list pth-users@gnu.org by running the following command:"; \
	        echo "\`$$cmd'"; \
	    else \
	        echo "$$when there is nothing more you can do here... ;)"; \
	        echo "There is just \`make clean' left for you."; \
	    fi; \
	    echo ""; \
	fi

#   execute and debug one of the test programs
test: test-std
test-std: test_std
	-@./test_std; \
	if [ $$? -eq 0 ]; then \
	    touch .done-test >/dev/null 2>&1 || $(TRUE); \
	    if [ ".`grep $(PLATFORM) $(S)PORTING`" = . ]; then \
	        VERSION=`$(SHTOOL) version -lc -dshort $(_VERSION_FILE)`; \
	        echo "Please send the following summary line via Email to the author"; \
	        echo "Ralf S. Engelschall <rse@engelschall.com> for inclusion into"; \
	        echo "the list of successfully tested platforms (see PORTING file):"; \
	        echo ""; \
	        $(SHTOOL) echo -e "Pth: %BOK%b: %B$(PLATFORM)%b | %B$(PTH_MCTX_ID)%b | %B$(PTH_STACK_GROWTH)%b | %B$$VERSION%b"; \
	        $(MAKE) $(MKFLAGS) what-next WHEN="Then"; \
	    else \
	        $(MAKE) $(MKFLAGS) what-next WHEN="Now"; \
	    fi; \
	    exit 0; \
	else \
	    VERSION=`$(SHTOOL) version -lc -dshort $(_VERSION_FILE)`; \
	    echo "Please send the following summary line together with details about"; \
	    $(SHTOOL) echo -e "the configuration (%Bpth_acdef.h%b, %Bpth_acmac.h%b, %Bconfig.status%b, %Bconfig.log%b)"; \
	    echo "and build/test steps (output of 'make' and 'make test') to the author"; \
	    echo "Ralf S. Engelschall <rse@engelschall.com> to help him in tracking"; \
	    echo "down your platform problem."; \
	    echo ""; \
	    $(SHTOOL) echo -e "Pth: %BFAILED%b: %B$(PLATFORM)%b | %B$(PTH_MCTX_ID)%b | %B$(PTH_STACK_GROWTH)%b | %B$$VERSION%b"; \
	    echo ""; \
	    exit 1; \
	fi
test-httpd: test_httpd
	./test_httpd
test-mp: test_mp
	./test_mp
test-misc: test_misc
	./test_misc
test-philo: test_philo
	./test_philo
test-sig: test_sig
	./test_sig
test-select: test_select
	./test_select
test-sfio: test_sfio
	./test_sfio
test-uctx: test_uctx
	./test_uctx
test-pthread: test_pthread
	./test_pthread
debug: debug-std
debug-std: test_std
	TEST=test_std && $(_DEBUG)
debug-httpd: test_httpd
	TEST=test_httpd && $(_DEBUG)
debug-mp: test_mp
	TEST=test_mp && $(_DEBUG)
debug-misc: test_misc
	TEST=test_misc && $(_DEBUG)
debug-philo: test_philo
	TEST=test_philo && $(_DEBUG)
debug-sig: test_sig
	TEST=test_sig && $(_DEBUG)
debug-select: test_select
	TEST=test_select && $(_DEBUG)
debug-sfio: test_sfio
	TEST=test_sfio && $(_DEBUG)
debug-uctx: test_uctx
	TEST=test_uctx && $(_DEBUG)
debug-pthread: test_pthread
	TEST=test_pthread && $(_DEBUG)

#   GNU compat targets
check: test
installcheck:
installdirs: install-dirs
install-strip: install
mostlyclean: clean
maintainer-clean: realclean
info:
dvi:
TAGS:

##
##  ____ DEPENDENCY AREA ____________________________________________
##  (AUTOMATICALLY UPDATED - DO NOT EDIT)
##

$(LOBJS): Makefile

# DO NOT REMOVE
pth_attr.lo: pth_attr.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_cancel.lo: pth_cancel.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_clean.lo: pth_clean.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_compat.lo: pth_compat.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_data.lo: pth_data.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_debug.lo: pth_debug.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_errno.lo: pth_errno.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_event.lo: pth_event.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_ext.lo: pth_ext.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_fork.lo: pth_fork.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_high.lo: pth_high.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_lib.lo: pth_lib.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_mctx.lo: pth_mctx.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_msg.lo: pth_msg.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_pqueue.lo: pth_pqueue.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_ring.lo: pth_ring.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_sched.lo: pth_sched.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_string.lo: pth_string.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_sync.lo: pth_sync.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_syscall.lo: pth_syscall.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_tcb.lo: pth_tcb.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_time.lo: pth_time.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_util.lo: pth_util.c pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
pth_vers.lo: pth_vers.c pth_vers.c
pthread.o: pthread.c pthread.h pth_p.h pth_vers.c pth.h pth_acdef.h pth_acmac.h
test_common.o: test_common.c pth.h test_common.h
test_httpd.o: test_httpd.c pth.h test_common.h
test_misc.o: test_misc.c pth.h
test_mp.o: test_mp.c pth.h test_common.h
test_philo.o: test_philo.c pth.h test_common.h
test_pthread.o: test_pthread.c pthread.h
test_select.o: test_select.c pth.h
test_sfio.o: test_sfio.c pth.h
test_uctx.o: test_uctx.c pth.h
test_sig.o: test_sig.c pth.h
test_std.o: test_std.c pth.h
