/* Data base of default implicit rules for GNU Make.
Copyright (C) 1988-2023 Free Software Foundation, Inc.
This file is part of GNU Make.

GNU Make is free software; you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation; either version 3 of the License, or (at your option) any later
version.

GNU Make is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;

#[cfg(any(target_os = "vms", target_os = "emx"))]
const GCC_IS_NATIVE: bool = true;
#[cfg(not(any(target_os = "vms", target_os = "emx")))]
const GCC_IS_NATIVE: bool = false;

#[cfg(target_os = "vms")]
const DEFAULT_SUFFIXES: &str = ".out .exe .a .olb .hlb .tlb .mlb .ln .o .obj .c .cxx .cc .cpp .pas .p \
.for .f .r .y .l .ym .yl .mar .s .ss .i .ii .mod .sym .def .h .info .dvi \
.tex .texinfo .texi .txinfo .mem .hlp .brn .rnh .rno .rnt .rnx .w .ch .cweb \
.web .com .sh .elc .el";

#[cfg(target_os = "emx")]
const DEFAULT_SUFFIXES: &str = ".out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S \
.mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo \
.w .ch .web .sh .elc .el .obj .exe .dll .lib";

#[cfg(not(any(target_os = "vms", target_os = "emx")))]
const DEFAULT_SUFFIXES: &str = ".out .a .ln .o .c .cc .C .cpp .p .f .F .m .r .y .l .ym .yl .s .S \
.mod .sym .def .h .info .dvi .tex .texinfo .texi .txinfo \
.w .ch .web .sh .elc .el";

struct PatternRule {
    target: &'static str,
    deps: &'static str,
    commands: &'static str,
}

#[cfg(target_os = "vms")]
const DEFAULT_PATTERN_RULES: &[PatternRule] = &[
    PatternRule {
        target: "(%)",
        deps: "%",
        commands: "@if f$$search(\"$@\") .eqs. \"\" then $(LIBRARY)/CREATE/$(or $(patsubst %,TEXT,$(filter %.tlb %.TLB,$@)),$(patsubst %,HELP,$(filter %.hlb %.HLB,$@)),$(patsubst %,MACRO,$(filter %.mlb %.MLB,$@)),$(and $(patsubst %,SHARE,$(filter %.olb %.OLB,$@)),$(patsubst %,SHARE,$(filter %.exe %.EXE,$<))),OBJECT) $@\n$(AR) $(ARFLAGS) $@ $<",
    },
    PatternRule {
        target: "%.exe",
        deps: "%",
        commands: "$(CP) $< $@",
    },
    PatternRule {
        target: "%.out",
        deps: "%",
        commands: "@rm -f $@ \n cp $< $@",
    },
    PatternRule {
        target: "%.c",
        deps: "%.w %.ch",
        commands: "$(CTANGLE) $^ $@",
    },
    PatternRule {
        target: "%.tex",
        deps: "%.w %.ch",
        commands: "$(CWEAVE) $^ $@",
    },
];

#[cfg(not(target_os = "vms"))]
const DEFAULT_PATTERN_RULES: &[PatternRule] = &[
    PatternRule {
        target: "(%)",
        deps: "%",
        commands: "$(AR) $(ARFLAGS) $@ $<",
    },
    PatternRule {
        target: "%.out",
        deps: "%",
        commands: "@rm -f $@ \n cp $< $@",
    },
    PatternRule {
        target: "%.c",
        deps: "%.w %.ch",
        commands: "$(CTANGLE) $^ $@",
    },
    PatternRule {
        target: "%.tex",
        deps: "%.w %.ch",
        commands: "$(CWEAVE) $^ $@",
    },
];

#[cfg(target_os = "vms")]
const DEFAULT_TERMINAL_RULES: &[PatternRule] = &[
    PatternRule {
        target: "%",
        deps: "%$$5lv",
        commands: "if f$$search(\"$@\") .nes. \"\" then +$(CHECKOUT,v)",
    },
    PatternRule {
        target: "%",
        deps: "[.$$rcs]%$$5lv",
        commands: "if f$$search(\"$@\") .nes. \"\" then +$(CHECKOUT,v)",
    },
    PatternRule {
        target: "%",
        deps: "%_v",
        commands: "if f$$search(\"$@\") .nes. \"\" then +$(CHECKOUT,v)",
    },
    PatternRule {
        target: "%",
        deps: "[.rcs]%_v",
        commands: "if f$$search(\"$@\") .nes. \"\" then +$(CHECKOUT,v)",
    },
];

#[cfg(not(target_os = "vms")))]
const DEFAULT_TERMINAL_RULES: &[PatternRule] = &[
    PatternRule {
        target: "%",
        deps: "%,v",
        commands: "$(CHECKOUT,v)",
    },
    PatternRule {
        target: "%",
        deps: "RCS/%,v",
        commands: "$(CHECKOUT,v)",
    },
    PatternRule {
        target: "%",
        deps: "RCS/%",
        commands: "$(CHECKOUT,v)",
    },
    PatternRule {
        target: "%",
        deps: "s.%",
        commands: "$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<",
    },
    PatternRule {
        target: "%",
        deps: "SCCS/s.%",
        commands: "$(GET) $(GFLAGS) $(SCCS_OUTPUT_OPTION) $<",
    },
];

#[cfg(target_os = "vms")]
const DEFAULT_SUFFIX_RULES: &[(&str, &str)] = &[
    (".o", "$(LINK.obj) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".obj", "$(LINK.obj) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".s", "$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".S", "$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".c", "$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".cc", "$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".C", "$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".cpp", "$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".f", "$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".m", "$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".p", "$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".F", "$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".r", "$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".mod", "$(COMPILE.mod) -o $@ -e $@ $^"),
    (".def.sym", "$(COMPILE.def) -o $@ $<"),
    (".sh", "copy $< >$@"),
    (".obj.exe", "$(LINK.obj) $^ $(LOADLIBES) $(LDLIBS) $(CRT0) /exe=$@"),
    (".mar.exe", "$(COMPILE.mar) $^ \n $(LINK.obj) $(subst .mar,.obj,$^) $(LOADLIBES) $(LDLIBS) $(CRT0) /exe=$@"),
    (".s.o", "$(COMPILE.s) -o $@ $<"),
    (".s.exe", "$(COMPILE.s) $^ \n $(LINK.obj) $(subst .s,.obj,$^) $(LOADLIBES) $(LDLIBS) $(CRT0) /exe=$@"),
    (".c.exe", "$(COMPILE.c) $^ \n $(LINK.obj) $(subst .c,.obj,$^) $(LOADLIBES) $(LDLIBS) $(CRT0) /exe=$@"),
    #[cfg(GCC_IS_NATIVE)]
    (".cc.exe", "$(COMPILE.cc) $^ \n $(LINK.obj) $(CXXSTARTUP),sys$$disk:[]$(subst .cc,.obj,$^) $(LOADLIBES) $(LXLIBS) $(LDLIBS) $(CXXRT0) /exe=$@"),
    #[cfg(not(GCC_IS_NATIVE))]
    (".cc.exe", "$(COMPILE.cc) $^ \n $(CXXLINK.obj) $(subst .cc,.obj,$^) $(LOADLIBES) $(LXLIBS) $(LDLIBS) $(CXXRT0) /exe=$@"),
    (".cxx.exe", "$(COMPILE.cxx) $^ \n $(CXXLINK.obj) $(subst .cxx,.obj,$^) $(LOADLIBES) $(LXLIBS) $(LDLIBS) $(CXXRT0) /exe=$@"),
    (".for.exe", "$(COMPILE.for) $^ \n $(LINK.obj) $(subst .for,.obj,$^) $(LOADLIBES) $(LDLIBS) /exe=$@"),
    (".pas.exe", "$(COMPILE.pas) $^ \n $(LINK.obj) $(subst .pas,.obj,$^) $(LOADLIBES) $(LDLIBS) /exe=$@"),
    (".com", "copy $< >$@"),
    (".mar.obj", "$(COMPILE.mar) /obj=$@ $<"),
    (".s.obj", "$(COMPILE.s) /obj=$@ $<"),
    (".ss.obj", "$(COMPILE.s) /obj=$@ $<"),
    (".c.i", "$(COMPILE.c)/prep /list=$@ $<"),
    (".c.s", "$(COMPILE.c)/noobj/machine /list=$@ $<"),
    (".i.s", "$(COMPILE.c)/noprep/noobj/machine /list=$@ $<"),
    (".c.obj", "$(COMPILE.c) /obj=$@ $<"),
    (".c.o", "$(COMPILE.c) /obj=$@ $<"),
    (".cc.ii", "$(COMPILE.cc)/prep /list=$@ $<"),
    (".cc.ss", "$(COMPILE.cc)/noobj/machine /list=$@ $<"),
    (".ii.ss", "$(COMPILE.cc)/noprep/noobj/machine /list=$@ $<"),
    (".cc.obj", "$(COMPILE.cc) /obj=$@ $<"),
    (".cc.o", "$(COMPILE.cc) /obj=$@ $<"),
    (".cxx.obj", "$(COMPILE.cxx) /obj=$@ $<"),
    (".cxx.o", "$(COMPILE.cxx) /obj=$@ $<"),
    (".for.obj", "$(COMPILE.for) /obj=$@ $<"),
    (".for.o", "$(COMPILE.for) /obj=$@ $<"),
    (".pas.obj", "$(COMPILE.pas) /obj=$@ $<"),
    (".pas.o", "$(COMPILE.pas) /obj=$@ $<"),
    (".y.c", "$(YACC.y) $< \n rename y_tab.c $@"),
    (".l.c", "$(LEX.l) $< \n rename lexyy.c $@"),
    (".texinfo.info", "$(MAKEINFO) $<"),
    (".tex.dvi", "$(TEX) $<"),
    (".cpp.o", "$(COMPILE.cpp) $(OUTPUT_OPTION) $<"),
    (".f.o", "$(COMPILE.f) $(OUTPUT_OPTION) $<"),
    (".m.o", "$(COMPILE.m) $(OUTPUT_OPTION) $<"),
    (".p.o", "$(COMPILE.p) $(OUTPUT_OPTION) $<"),
    (".r.o", "$(COMPILE.r) $(OUTPUT_OPTION) $<"),
    (".mod.o", "$(COMPILE.mod) -o $@ $<"),
    (".c.ln", "$(LINT.c) -C$* $<"),
    (".y.ln", "$(YACC.y) $< \n rename y_tab.c $@"),
    (".l.ln", "@$(RM) $*.c\n $(LEX.l) $< > $*.c\n$(LINT.c) -i $*.c -o $@\n $(RM) $*.c"),
];

#[cfg(not(target_os = "vms")))]
const DEFAULT_SUFFIX_RULES: &[(&str, &str)] = &[
    (".o", "$(LINK.o) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".s", "$(LINK.s) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".S", "$(LINK.S) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".c", "$(LINK.c) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".cc", "$(LINK.cc) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".C", "$(LINK.C) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".cpp", "$(LINK.cpp) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".f", "$(LINK.f) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".m", "$(LINK.m) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".p", "$(LINK.p) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".F", "$(LINK.F) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".r", "$(LINK.r) $^ $(LOADLIBES) $(LDLIBS) -o $@"),
    (".mod", "$(COMPILE.mod) -o $@ -e $@ $^"),
    (".def.sym", "$(COMPILE.def) -o $@ $<"),
    (".sh", "cat $< >$@ \n chmod a+x $@"),
    (".s.o", "$(COMPILE.s) -o $@ $<"),
    (".S.o", "$(COMPILE.S) -o $@ $<"),
    (".c.o", "$(COMPILE.c) $(OUTPUT_OPTION) $<"),
    (".cc.o", "$(COMPILE.cc) $(OUTPUT_OPTION) $<"),
    (".C.o", "$(COMPILE.C) $(OUTPUT_OPTION) $<"),
    (".cpp.o", "$(COMPILE.cpp) $(OUTPUT_OPTION) $<"),
    (".f.o", "$(COMPILE.f) $(OUTPUT_OPTION) $<"),
    (".m.o", "$(COMPILE.m) $(OUTPUT_OPTION) $<"),
    (".p.o", "$(COMPILE.p) $(OUTPUT_OPTION) $<"),
    (".F.o", "$(COMPILE.F) $(OUTPUT_OPTION) $<"),
    (".r.o", "$(COMPILE.r) $(OUTPUT_OPTION) $<"),
    (".mod.o", "$(COMPILE.mod) -o $@ $<"),
    (".c.ln", "$(LINT.c) -C$* $<"),
    #[cfg(not(target_os = "msdos"))]
    (".y.ln", "$(YACC.y) $< \n $(LINT.c) -C$* y.tab.c \n $(RM) y.tab.c"),
    #[cfg(target