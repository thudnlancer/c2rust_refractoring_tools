

srcdir = .


top_srcdir = ..

include ../config.make

PRE_CPPFLAGS = -I.. -I$(top_srcdir)
PRE_LDFLAGS = -L..

OPENSSL_LIBFLAGS = -lcrypto
BENCH_LIBS =  -lm

HOGWEED_TARGETS = rsa-keygen$(EXEEXT) rsa-sign$(EXEEXT) \
	      rsa-verify$(EXEEXT) rsa-encrypt$(EXEEXT) rsa-decrypt$(EXEEXT) \
	      random-prime$(EXEEXT) \
	      hogweed-benchmark$(EXEEXT) ecc-benchmark$(EXEEXT)

ENC_TARGETS = base16enc$(EXEEXT) base16dec$(EXEEXT) \
	      base64enc$(EXEEXT) base64dec$(EXEEXT)
TARGETS = nettle-benchmark$(EXEEXT) \
	  $(ENC_TARGETS) # $(HOGWEED_TARGETS)

SOURCES = nettle-benchmark.c hogweed-benchmark.c ecc-benchmark.c \
	random-prime.c \
	nettle-openssl.c \
	io.c read_rsa_key.c \
	rsa-encrypt.c rsa-decrypt.c rsa-keygen.c rsa-sign.c rsa-verify.c \
	base16enc.c base16dec.c base64enc.c base64dec.c timing.c


GETOPT_OBJS = ../getopt.$(OBJEXT) ../getopt1.$(OBJEXT)

TS_ALL = rsa-sign-test rsa-verify-test rsa-encrypt-test

DISTFILES= $(SOURCES) Makefile.in $(TS_ALL) setup-env teardown-env \
	io.h rsa-session.h timing.h

all: $(TARGETS)

%.$(OBJEXT): %.c
	$(COMPILE) -c $< && $(DEP_PROCESS)

# NOTE: If we required GNU make, we could use a single rule with $(@F)
# or $(notdir $@)
../getopt.$(OBJEXT):
	( cd .. && $(MAKE) getopt.$(OBJEXT))
../getopt1.$(OBJEXT):
	( cd .. && $(MAKE) getopt1.$(OBJEXT))
../non-nettle.$(OBJEXT):
	( cd .. && $(MAKE) non-nettle.$(OBJEXT))

# For Solaris and BSD make, we have to use an explicit rule for each executable
random-prime$(EXEEXT): random-prime.$(OBJEXT) io.$(OBJEXT) $(GETOPT_OBJS)
	$(LINK) random-prime.$(OBJEXT) io.$(OBJEXT) $(GETOPT_OBJS) \
	-lhogweed -lnettle $(LIBS) -o random-prime$(EXEEXT)

rsa-keygen$(EXEEXT): rsa-keygen.$(OBJEXT) io.$(OBJEXT) $(GETOPT_OBJS)
	$(LINK) rsa-keygen.$(OBJEXT) io.$(OBJEXT) $(GETOPT_OBJS) \
	-lhogweed -lnettle $(LIBS) -o rsa-keygen$(EXEEXT)

rsa-sign$(EXEEXT): rsa-sign.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT)
	$(LINK) rsa-sign.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT) \
	-lhogweed -lnettle $(LIBS) -o rsa-sign$(EXEEXT)

rsa-verify$(EXEEXT): rsa-verify.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT)
	$(LINK) rsa-verify.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT) \
	-lhogweed -lnettle $(LIBS) -o rsa-verify$(EXEEXT)

rsa-encrypt$(EXEEXT): rsa-encrypt.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT) $(GETOPT_OBJS)
	$(LINK) rsa-encrypt.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT) \
	$(GETOPT_OBJS) \
	-lhogweed -lnettle $(LIBS) -o rsa-encrypt$(EXEEXT)

rsa-decrypt$(EXEEXT): rsa-decrypt.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT)
	$(LINK) rsa-decrypt.$(OBJEXT) io.$(OBJEXT) read_rsa_key.$(OBJEXT) \
	-lhogweed -lnettle $(LIBS) -o rsa-decrypt$(EXEEXT)

base16enc$(EXEEXT): base16enc.$(OBJEXT) io.$(OBJEXT)
	$(LINK) base16enc.$(OBJEXT) io.$(OBJEXT) \
	-lnettle $(LIBS) -o base16enc$(EXEEXT)

base16dec$(EXEEXT): base16dec.$(OBJEXT) io.$(OBJEXT)
	$(LINK) base16dec.$(OBJEXT) io.$(OBJEXT) \
	-lnettle $(LIBS) -o base16dec$(EXEEXT)

base64enc$(EXEEXT): base64enc.$(OBJEXT) io.$(OBJEXT)
	$(LINK) base64enc.$(OBJEXT) io.$(OBJEXT) \
	-lnettle $(LIBS) -o base64enc$(EXEEXT)

base64dec$(EXEEXT): base64dec.$(OBJEXT) io.$(OBJEXT)
	$(LINK) base64dec.$(OBJEXT) io.$(OBJEXT) \
	-lnettle $(LIBS) -o base64dec$(EXEEXT)

BENCH_OBJS = nettle-benchmark.$(OBJEXT) nettle-openssl.$(OBJEXT) \
	$(GETOPT_OBJS) ../non-nettle.$(OBJEXT) timing.$(OBJEXT)
nettle-benchmark$(EXEEXT): $(BENCH_OBJS)
	$(LINK) $(BENCH_OBJS) -lnettle $(BENCH_LIBS) $(OPENSSL_LIBFLAGS) -o nettle-benchmark$(EXEEXT)

ECC_BENCH_OBJS = ecc-benchmark.$(OBJEXT) timing.$(OBJEXT)
ecc-benchmark$(EXEEXT): $(ECC_BENCH_OBJS)
	$(LINK) $(ECC_BENCH_OBJS) -lhogweed -lnettle $(BENCH_LIBS) $(LIBS) \
	-o ecc-benchmark$(EXEEXT)

HOGWEED_BENCH_OBJS = hogweed-benchmark.$(OBJEXT) timing.$(OBJEXT)
hogweed-benchmark$(EXEEXT): $(HOGWEED_BENCH_OBJS)
	$(LINK) $(HOGWEED_BENCH_OBJS) \
	-lhogweed -lnettle $(BENCH_LIBS) $(LIBS) $(OPENSSL_LIBFLAGS) \
	-o hogweed-benchmark$(EXEEXT)

$(TARGETS) : io.$(OBJEXT) ../libnettle.stamp
$(HOGWEED_TARGETS): ../libhogweed.stamp

check: $(TS_ALL)
	TEST_SHLIB_DIR="$(TEST_SHLIB_DIR)" \
	  srcdir="$(srcdir)" EMULATOR="$(EMULATOR)" EXEEXT="$(EXEEXT)" \
          "$(top_srcdir)"/run-tests $(TS_ALL)

Makefile: $(srcdir)/Makefile.in ../config.status
	cd .. && $(SHELL) ./config.status examples/$@

install uninstall:
	true

# NOTE: I'd like to use $^, but that's a GNU extension. $? should be
# more portable, equivalent for phony targets.
distdir: $(DISTFILES)
	cp $? $(distdir)

clean:
	-rm -f $(TARGETS) *.$(OBJEXT) *.$(OBJEXT).d

distclean: clean
	-rm -f Makefile *.d

tags:
	etags -o $(srcdir)/TAGS --include $(top_srcdir) $(srcdir)/*.c $(srcdir)/*.h

-include $(SOURCES:.c=.$(OBJEXT).d)
