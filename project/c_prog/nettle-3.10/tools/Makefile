

srcdir = .


top_srcdir = ..

INSTALL = /usr/bin/install -c
INSTALL_DATA = ${INSTALL} -m 644
INSTALL_PROGRAM = ${INSTALL}
INSTALL_SCRIPT = ${INSTALL}
INSTALL_STRIP_PROGRAM = $(INSTALL_PROGRAM) -s
MKDIR_P = /bin/mkdir -p

include ../config.make

PRE_CPPFLAGS = -I.. -I$(top_srcdir)
PRE_LDFLAGS = -L..

HOGWEED_TARGETS = pkcs1-conv$(EXEEXT)
TARGETS = sexp-conv$(EXEEXT) nettle-hash$(EXEEXT) nettle-pbkdf2$(EXEEXT) \
	  nettle-lfib-stream$(EXEEXT) \
	  # $(HOGWEED_TARGETS)

TS_ALL = sexp-conv-test pkcs1-conv-test nettle-pbkdf2-test

all: $(TARGETS)

getopt_OBJS = ../getopt.$(OBJEXT) ../getopt1.$(OBJEXT)

sexp_conv_SOURCES = sexp-conv.c input.c output.c parse.c misc.c
pkcs1_conv_SOURCES = pkcs1-conv.c misc.c
nettle_hash_SOURCES = nettle-hash.c misc.c
nettle_pbkdf2_SOURCES = nettle-pbkdf2.c misc.c

SOURCES = $(sexp_conv_SOURCES) nettle-hash.c nettle-lfib-stream.c pkcs1-conv.c nettle-pbkdf2.c

DISTFILES = $(SOURCES) $(TS_ALL) Makefile.in input.h misc.h output.h parse.h

sexp_conv_OBJS = $(sexp_conv_SOURCES:.c=.$(OBJEXT)) $(getopt_OBJS)
sexp-conv$(EXEEXT): $(sexp_conv_OBJS) ../libnettle.stamp
	$(LINK) $(sexp_conv_OBJS) -lnettle $(LIBS) -o $@

nettle-lfib-stream$(EXEEXT): nettle-lfib-stream.$(OBJEXT) ../libnettle.stamp
	$(LINK) nettle-lfib-stream.$(OBJEXT) -lnettle $(LIBS) -o $@

pkcs1_conv_OBJS = $(pkcs1_conv_SOURCES:.c=.$(OBJEXT)) $(getopt_OBJS)
pkcs1-conv$(EXEEXT): $(pkcs1_conv_OBJS) ../libnettle.stamp ../libhogweed.stamp
	$(LINK) $(pkcs1_conv_OBJS) -lhogweed -lnettle $(LIBS) -o $@

# FIXME: Avoid linking with gmp
nettle_hash_OBJS = $(nettle_hash_SOURCES:.c=.$(OBJEXT)) $(getopt_OBJS)
nettle-hash$(EXEEXT): $(nettle_hash_OBJS) ../libnettle.stamp
	$(LINK) $(nettle_hash_OBJS) -lnettle -o $@

nettle_pbkdf2_OBJS = $(nettle_pbkdf2_SOURCES:.c=.$(OBJEXT)) $(getopt_OBJS)
nettle-pbkdf2$(EXEEXT): $(nettle_pbkdf2_OBJS) ../libnettle.stamp
	$(LINK) $(nettle_pbkdf2_OBJS) -lnettle -o $@


%.$(OBJEXT): %.c
	$(COMPILE) -c $< && $(DEP_PROCESS)

# NOTE: If we required GNU make, we could use a single rule with $(@F)
# or $(notdir $@)
../getopt.$(OBJEXT):
	( cd .. && $(MAKE) getopt.$(OBJEXT))
../getopt1.$(OBJEXT):
	( cd .. && $(MAKE) getopt1.$(OBJEXT))

Makefile: $(srcdir)/Makefile.in ../config.status
	cd .. && $(SHELL) ./config.status tools/$@

check: $(TS_ALL)
	TEST_SHLIB_DIR="$(TEST_SHLIB_DIR)" \
	  srcdir="$(srcdir)" \
	  EMULATOR="$(EMULATOR)" EXEEXT="$(EXEEXT)" \
          $(top_srcdir)/run-tests $(TS_ALL)


install: $(TARGETS)
	$(MKDIR_P) $(DESTDIR)$(bindir)
	for f in $(TARGETS) ; do \
	  $(INSTALL_PROGRAM) $$f $(DESTDIR)$(bindir) ; \
	done

uninstall:
	for f in $(TARGETS) ; do \
	  rm -f $(DESTDIR)$(bindir)/$$f ; \
	done

# NOTE: I'd like to use $^, but that's a GNU extension. $? should be
# more portable, equivalent for phony targets.
distdir: $(DISTFILES)
	cp $? $(distdir)

clean:
	-rm -f $(TARGETS) *.$(OBJEXT) *.$(OBJEXT).d test.in test1.out test2.out \
	testkey.priv testkey.pub testtmp testsignature testsignature2

distclean: clean
	-rm -f Makefile *.d

tags:
	etags -o $(srcdir)/TAGS --include $(top_srcdir) $(srcdir)/*.c $(srcdir)/*.h

-include $(SOURCES:.c=.$(OBJEXT).d)
