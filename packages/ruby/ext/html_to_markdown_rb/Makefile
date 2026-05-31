RB_SYS_BUILD_DIR ?= /Users/naamanhirschfeld/workspace/kreuzberg-dev/html-to-markdown/packages/ruby/ext/html_to_markdown_rb/.rb-sys
CARGO ?= cargo
CARGO_BUILD_TARGET ?=
SOEXT ?= dylib


# Determine the prefix Cargo uses for the lib.
ifneq ($(SOEXT),dll)
	SOEXT_PREFIX ?= lib
endif

RB_SYS_CARGO_PROFILE ?= release
RB_SYS_CARGO_FEATURES ?=
RB_SYS_GLOBAL_RUSTFLAGS ?=
RB_SYS_EXTRA_RUSTFLAGS ?=
RB_SYS_EXTRA_CARGO_ARGS ?=
RB_SYS_CARGO_MANIFEST_DIR ?= ./native

# Set dirname for the profile, since the profiles do not directly map to target dir (i.e. dev -> debug)
ifeq ($(RB_SYS_CARGO_PROFILE),dev)
	RB_SYS_CARGO_PROFILE_DIR ?= debug
else
	RB_SYS_CARGO_PROFILE_DIR ?= $(RB_SYS_CARGO_PROFILE)
endif

# Set the build profile (dev, release, etc.).
	RB_SYS_CARGO_PROFILE_FLAG = --profile $(RB_SYS_CARGO_PROFILE)

# Account for sub-directories when using `--target` argument with Cargo
RB_SYS_CARGO_TARGET_DIR ?= target
ifneq ($(CARGO_BUILD_TARGET),)
	RB_SYS_FULL_TARGET_DIR = $(RB_SYS_CARGO_TARGET_DIR)/$(CARGO_BUILD_TARGET)
else
	RB_SYS_FULL_TARGET_DIR = $(RB_SYS_CARGO_TARGET_DIR)
endif

target_prefix =
TARGET_NAME = html_to_markdown_rb
TARGET_ENTRY = Init_$(TARGET_NAME)
RUBYARCHDIR = $(sitearchdir)$(target_prefix)
TARGET = html_to_markdown_rb
DLLIB = $(TARGET).bundle
RUSTLIBDIR = $(RB_SYS_FULL_TARGET_DIR)/$(RB_SYS_CARGO_PROFILE_DIR)
RUSTLIB = $(RUSTLIBDIR)/$(SOEXT_PREFIX)$(TARGET_NAME).$(SOEXT)
TIMESTAMP_DIR = .
POSTLINK = dsymutil $@ 2>/dev/null; { test -z '$(RUBY_CODESIGN)' || codesign -s '$(RUBY_CODESIGN)' $@; }

CLEANOBJS = $(RUSTLIBDIR) $(RB_SYS_BUILD_DIR)
CLEANLIBS = $(DLLIB) $(RUSTLIB)
RUBYGEMS_CLEAN_DIRS = $(CLEANOBJS) $(CLEANFILES) ./cargo-vendor


SHELL = /bin/sh

# V=0 quiet, V=1 verbose.  other values don't work.
V = 0
V0 = $(V:0=)
Q1 = $(V:1=)
Q = $(Q1:0=@)
ECHO1 = $(V:1=@ :)
ECHO = $(ECHO1:0=@ echo)
NULLCMD = :

#### Start of system configuration section. ####

srcdir = ./native
topdir = /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0
hdrdir = $(topdir)
arch_hdrdir = /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0/arm64-darwin25
PATH_SEPARATOR = :
VPATH = $(srcdir):$(arch_hdrdir)/ruby:$(hdrdir)/ruby

prefix = $(DESTDIR)/Users/naamanhirschfeld/.rbenv/versions/3.4.8

rubysitearchprefix = $(rubylibprefix)/$(sitearch)

rubyarchprefix = $(rubylibprefix)/$(arch)

rubylibprefix = $(libdir)/$(RUBY_BASE_NAME)

exec_prefix = $(prefix)

vendorarchhdrdir = $(vendorhdrdir)/$(sitearch)

sitearchhdrdir = $(sitehdrdir)/$(sitearch)

rubyarchhdrdir = $(rubyhdrdir)/$(arch)

vendorhdrdir = $(rubyhdrdir)/vendor_ruby

sitehdrdir = $(rubyhdrdir)/site_ruby

rubyhdrdir = $(includedir)/$(RUBY_VERSION_NAME)

vendorarchdir = $(vendorlibdir)/$(sitearch)

vendorlibdir = $(vendordir)/$(ruby_version)

vendordir = $(rubylibprefix)/vendor_ruby

sitearchdir = $(sitelibdir)/$(sitearch)

sitelibdir = $(sitedir)/$(ruby_version)

sitedir = $(rubylibprefix)/site_ruby

rubyarchdir = $(rubylibdir)/$(arch)

rubylibdir = $(rubylibprefix)/$(ruby_version)

sitearchincludedir = $(includedir)/$(sitearch)

archincludedir = $(includedir)/$(arch)

sitearchlibdir = $(libdir)/$(sitearch)

archlibdir = $(libdir)/$(arch)

ridir = $(datarootdir)/$(RI_BASE_NAME)

modular_gc_dir = $(DESTDIR)

mandir = $(datarootdir)/man

localedir = $(datarootdir)/locale

libdir = $(exec_prefix)/lib

psdir = $(docdir)

pdfdir = $(docdir)

dvidir = $(docdir)

htmldir = $(docdir)

infodir = $(datarootdir)/info

docdir = $(datarootdir)/doc/$(PACKAGE)

oldincludedir = $(DESTDIR)/usr/include

includedir = $(SDKROOT)$(prefix)/include

runstatedir = $(localstatedir)/run

localstatedir = $(prefix)/var

sharedstatedir = $(prefix)/com

sysconfdir = $(prefix)/etc

datadir = $(datarootdir)

datarootdir = $(prefix)/share

libexecdir = $(exec_prefix)/libexec

sbindir = $(exec_prefix)/sbin

bindir = $(exec_prefix)/bin

archdir = $(rubyarchdir)



CC_WRAPPER =
CC = clang
CXX = clang++
LIBRUBY = $(LIBRUBY_SO)
LIBRUBY_A = lib$(RUBY_SO_NAME)-static.a
LIBRUBYARG_SHARED = -l$(RUBY_SO_NAME)
LIBRUBYARG_STATIC = -l$(RUBY_SO_NAME)-static -framework CoreFoundation $(MAINLIBS)
empty =
OUTFLAG = -o $(empty)
COUTFLAG = -o $(empty)
CSRCFLAG = $(empty)

RUBY_EXTCONF_H =
cflags   = $(hardenflags) -fdeclspec  $(optflags) $(debugflags) $(warnflags)
cxxflags =
optflags = -O3 -fno-fast-math
debugflags = -ggdb3
warnflags = -Wall -Wextra -Wextra-tokens -Wdeprecated-declarations -Wdivision-by-zero -Wdiv-by-zero -Wimplicit-function-declaration -Wimplicit-int -Wpointer-arith -Wshorten-64-to-32 -Wwrite-strings -Wold-style-definition -Wmissing-noreturn -Wno-cast-function-type -Wno-constant-logical-operand -Wno-long-long -Wno-missing-field-initializers -Wno-overlength-strings -Wno-parentheses-equality -Wno-self-assign -Wno-tautological-compare -Wno-unused-parameter -Wno-unused-value -Wunused-variable -Wmisleading-indentation -Wundef
cppflags =
CCDLFLAGS = -fno-common
CFLAGS   = $(CCDLFLAGS) $(cflags) -fno-common -pipe $(ARCH_FLAG)
INCFLAGS = -I. -I$(arch_hdrdir) -I$(hdrdir)/ruby/backward -I$(hdrdir) -I$(srcdir)
DEFS     =
CPPFLAGS =  -D_XOPEN_SOURCE -D_DARWIN_C_SOURCE -D_DARWIN_UNLIMITED_SELECT -D_REENTRANT $(DEFS) $(cppflags)
CXXFLAGS = $(CCDLFLAGS) -fdeclspec $(ARCH_FLAG)
ldflags  = -L. -fstack-protector-strong -L/opt/homebrew/Cellar/gmp/6.3.0/lib
dldflags = -L/opt/homebrew/Cellar/gmp/6.3.0/lib -Wl,-undefined,dynamic_lookup
ARCH_FLAG = -arch arm64
DLDFLAGS = $(ldflags) $(dldflags) $(ARCH_FLAG)
LDSHARED = $(CC) -dynamic -bundle
LDSHAREDXX = $(CXX) -dynamic -bundle
POSTLINK = dsymutil $@ 2>/dev/null; { test -z '$(RUBY_CODESIGN)' || codesign -s '$(RUBY_CODESIGN)' $@; }
AR = ar
LD = ld
EXEEXT =


RUBY_INSTALL_NAME = $(RUBY_BASE_NAME)

RUBY_SO_NAME = ruby.3.4

RUBYW_INSTALL_NAME =

RUBY_VERSION_NAME = $(RUBY_BASE_NAME)-$(ruby_version)

RUBYW_BASE_NAME = rubyw

RUBY_BASE_NAME = ruby


arch = arm64-darwin25
sitearch = $(arch)
ruby_version = 3.4.0
ruby = $(bindir)/$(RUBY_BASE_NAME)
RUBY = $(ruby)
BUILTRUBY = $(bindir)/$(RUBY_BASE_NAME)
ruby_headers = $(hdrdir)/ruby.h $(hdrdir)/ruby/backward.h $(hdrdir)/ruby/ruby.h $(hdrdir)/ruby/defines.h $(hdrdir)/ruby/missing.h $(hdrdir)/ruby/intern.h $(hdrdir)/ruby/st.h $(hdrdir)/ruby/subst.h $(arch_hdrdir)/ruby/config.h

RM = rm -f
RM_RF = rm -fr
RMDIRS = rmdir -p
MAKEDIRS = /opt/homebrew/bin/gmkdir -p
INSTALL = /opt/homebrew/bin/ginstall -c
INSTALL_PROG = $(INSTALL) -m 0755
INSTALL_DATA = $(INSTALL) -m 644
COPY = cp
TOUCH = exit >

#### End of system configuration section. ####

preload =

CLEANFILES = mkmf.log
DISTCLEANFILES =

all static install-rb: Makefile
	@$(NULLCMD)
.PHONY: all static install-rb
.PHONY: clean clean-so clean-static clean-rb



clean-static::
clean-rb-default::
clean-rb::
clean-so::
clean: clean-so clean-static clean-rb-default clean-rb
		-$(Q)$(RM_RF) $(CLEANLIBS) $(CLEANOBJS) $(CLEANFILES) .*.time

distclean-rb-default::
distclean-rb::
distclean-so::
distclean-static::
distclean: clean distclean-so distclean-static distclean-rb-default distclean-rb
		-$(Q)$(RM) Makefile $(RUBY_EXTCONF_H) conftest.* mkmf.log
		-$(Q)$(RM) core ruby$(EXEEXT) *~ $(DISTCLEANFILES)
		-$(Q)$(RMDIRS) $(DISTCLEANDIRS) 2> /dev/null || true

realclean: distclean


.PHONY: gemclean

ifneq ($(RB_SYS_VERBOSE),)
	Q = $(0=@)
endif

CC = clang
CXX = clang++
AR = ar
export RBCONFIG_DESTDIR :=
export RBCONFIG_MAJOR := 3
export RBCONFIG_MINOR := 4
export RBCONFIG_TEENY := 8
export RBCONFIG_PATCHLEVEL := 72
export RBCONFIG_INSTALL := /opt/homebrew/bin/ginstall -c
export RBCONFIG_EXEEXT :=
export RBCONFIG_prefix := /Users/naamanhirschfeld/.rbenv/versions/3.4.8
export RBCONFIG_ruby_install_name := ruby
export RBCONFIG_RUBY_INSTALL_NAME := ruby
export RBCONFIG_RUBY_SO_NAME := ruby.3.4
export RBCONFIG_exec := exec
export RBCONFIG_ruby_pc := ruby-3.4.pc
export RBCONFIG_CC_WRAPPER :=
export RBCONFIG_PACKAGE := ruby
export RBCONFIG_BUILTIN_TRANSSRCS := enc/trans/newline.c
export RBCONFIG_MKMF_VERBOSE := 0
export RBCONFIG_MANTYPE := doc
export RBCONFIG_vendorarchhdrdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0/vendor_ruby/arm64-darwin25
export RBCONFIG_sitearchhdrdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0/site_ruby/arm64-darwin25
export RBCONFIG_rubyarchhdrdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0/arm64-darwin25
export RBCONFIG_vendorhdrdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0/vendor_ruby
export RBCONFIG_sitehdrdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0/site_ruby
export RBCONFIG_rubyhdrdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/ruby-3.4.0
export RBCONFIG_RUBY_SEARCH_PATH :=
export RBCONFIG_UNIVERSAL_INTS :=
export RBCONFIG_UNIVERSAL_ARCHNAMES :=
export RBCONFIG_configure_args := '--prefix=/Users/naamanhirschfeld/.rbenv/versions/3.4.8' '--with-openssl-dir=/opt/homebrew/opt/openssl@3' '--enable-shared' '--with-libyaml-dir=/opt/homebrew/opt/libyaml' '--with-gmp-dir=/opt/homebrew/opt/gmp' '--with-ext=openssl,psych,+' 'CC=clang'
export RBCONFIG_CONFIGURE := configure
export RBCONFIG_vendorarchdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/vendor_ruby/3.4.0/arm64-darwin25
export RBCONFIG_vendorlibdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/vendor_ruby/3.4.0
export RBCONFIG_vendordir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/vendor_ruby
export RBCONFIG_sitearchdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/site_ruby/3.4.0/arm64-darwin25
export RBCONFIG_sitelibdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/site_ruby/3.4.0
export RBCONFIG_sitedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/site_ruby
export RBCONFIG_rubyarchdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/3.4.0/arm64-darwin25
export RBCONFIG_rubylibdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/3.4.0
export RBCONFIG_ruby_version := 3.4.0
export RBCONFIG_sitearch := arm64-darwin25
export RBCONFIG_arch := arm64-darwin25
export RBCONFIG_sitearchincludedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/arm64-darwin25
export RBCONFIG_archincludedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include/arm64-darwin25
export RBCONFIG_sitearchlibdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/arm64-darwin25
export RBCONFIG_archlibdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/arm64-darwin25
export RBCONFIG_libdirname := libdir
export RBCONFIG_RUBY_EXEC_PREFIX := /Users/naamanhirschfeld/.rbenv/versions/3.4.8
export RBCONFIG_RUBY_LIB_VERSION :=
export RBCONFIG_RUBY_LIB_VERSION_STYLE := 3	/* full */
export RBCONFIG_RI_BASE_NAME := ri
export RBCONFIG_ridir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/ri
export RBCONFIG_rubysitearchprefix := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/arm64-darwin25
export RBCONFIG_rubyarchprefix := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/arm64-darwin25
export RBCONFIG_MAKEFILES := Makefile GNUmakefile
export RBCONFIG_USE_LLVM_WINDRES :=
export RBCONFIG_PLATFORM_DIR :=
export RBCONFIG_COROUTINE_TYPE := arm64
export RBCONFIG_THREAD_MODEL := pthread
export RBCONFIG_SYMBOL_PREFIX := _
export RBCONFIG_EXPORT_PREFIX :=
export RBCONFIG_COMMON_HEADERS :=
export RBCONFIG_COMMON_MACROS :=
export RBCONFIG_COMMON_LIBS :=
export RBCONFIG_MAINLIBS := -lgmp -ldl -lobjc -lpthread
export RBCONFIG_ENABLE_SHARED := yes
export RBCONFIG_DLDSHARED := clang -dynamiclib
export RBCONFIG_DLDLIBS :=
export RBCONFIG_SOLIBS := -lgmp -ldl -lobjc -lpthread
export RBCONFIG_LIBRUBYARG_SHARED := -lruby.3.4
export RBCONFIG_LIBRUBYARG_STATIC := -lruby.3.4-static -framework CoreFoundation -lgmp -ldl -lobjc -lpthread
export RBCONFIG_LIBRUBYARG := -lruby.3.4
export RBCONFIG_LIBRUBY := libruby.3.4.dylib
export RBCONFIG_LIBRUBY_ALIASES := libruby.dylib
export RBCONFIG_LIBRUBY_SONAME := libruby.3.4.dylib
export RBCONFIG_LIBRUBY_SO := libruby.3.4.dylib
export RBCONFIG_LIBRUBY_A := libruby.3.4-static.a
export RBCONFIG_RUBYW_INSTALL_NAME :=
export RBCONFIG_rubyw_install_name :=
export RBCONFIG_EXTDLDFLAGS :=
export RBCONFIG_EXTLDFLAGS :=
export RBCONFIG_hardenflags := -fstack-protector-strong -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2
export RBCONFIG_strict_warnflags :=
export RBCONFIG_warnflags := -Wall -Wextra -Wextra-tokens -Wdeprecated-declarations -Wdivision-by-zero -Wdiv-by-zero -Wimplicit-function-declaration -Wimplicit-int -Wpointer-arith -Wshorten-64-to-32 -Wwrite-strings -Wold-style-definition -Wmissing-noreturn -Wno-cast-function-type -Wno-constant-logical-operand -Wno-long-long -Wno-missing-field-initializers -Wno-overlength-strings -Wno-parentheses-equality -Wno-self-assign -Wno-tautological-compare -Wno-unused-parameter -Wno-unused-value -Wunused-variable -Wmisleading-indentation -Wundef
export RBCONFIG_debugflags := -ggdb3
export RBCONFIG_optflags := -O3 -fno-fast-math
export RBCONFIG_NULLCMD := :
export RBCONFIG_ENABLE_DEBUG_ENV :=
export RBCONFIG_DLNOBJ := dln.o
export RBCONFIG_RJIT_SUPPORT := yes
export RBCONFIG_YJIT_OBJ := yjit.o
export RBCONFIG_YJIT_LIBS := yjit/target/release/libyjit.a
export RBCONFIG_CARGO_BUILD_ARGS :=
export RBCONFIG_YJIT_SUPPORT := yes
export RBCONFIG_CARGO :=
export RBCONFIG_RUSTC := rustc
export RBCONFIG_INSTALL_STATIC_LIBRARY := no
export RBCONFIG_modular_gc_dir :=
export RBCONFIG_EXECUTABLE_EXTS :=
export RBCONFIG_ARCHFILE :=
export RBCONFIG_LIBRUBY_RELATIVE := no
export RBCONFIG_EXTOUT := .ext
export RBCONFIG_PREP := miniruby
export RBCONFIG_CROSS_COMPILING := no
export RBCONFIG_TEST_RUNNABLE := yes
export RBCONFIG_rubylibprefix := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby
export RBCONFIG_setup := Setup
export RBCONFIG_SOEXT := dylib
export RBCONFIG_TRY_LINK :=
export RBCONFIG_PRELOADENV := DYLD_INSERT_LIBRARIES
export RBCONFIG_LIBPATHENV := DYLD_LIBRARY_PATH
export RBCONFIG_RPATHFLAG :=
export RBCONFIG_LIBPATHFLAG := -L%s
export RBCONFIG_LINK_SO := \ndsymutil $@ 2>/dev/null; { test -z '$(RUBY_CODESIGN)' || codesign -s '$(RUBY_CODESIGN)' $@; }
export RBCONFIG_ADDITIONAL_DLDFLAGS :=
export RBCONFIG_ENCSTATIC :=
export RBCONFIG_EXTSTATIC :=
export RBCONFIG_ASMEXT := S
export RBCONFIG_LIBEXT := a
export RBCONFIG_DLEXT := bundle
export RBCONFIG_LDSHAREDXX := clang++ -dynamic -bundle
export RBCONFIG_LDSHARED := clang -dynamic -bundle
export RBCONFIG_CCDLFLAGS := -fno-common
export RBCONFIG_STATIC :=
export RBCONFIG_ARCH_FLAG := -arch arm64
export RBCONFIG_DLDFLAGS := -L/opt/homebrew/Cellar/gmp/6.3.0/lib -Wl,-undefined,dynamic_lookup
export RBCONFIG_ALLOCA :=
export RBCONFIG_EGREP := /usr/bin/grep -E
export RBCONFIG_GREP := /usr/bin/grep
export RBCONFIG_dsymutil := dsymutil
export RBCONFIG_codesign := codesign
export RBCONFIG_cleanlibs := $(TARGET_SO:=.dSYM)
export RBCONFIG_POSTLINK := dsymutil $@ 2>/dev/null; { test -z '$(RUBY_CODESIGN)' || codesign -s '$(RUBY_CODESIGN)' $@; }
export RBCONFIG_incflags := -I/opt/homebrew/Cellar/gmp/6.3.0/include
export RBCONFIG_WERRORFLAG := -Werror
export RBCONFIG_RUBY_DEVEL :=
export RBCONFIG_CHDIR := cd -P
export RBCONFIG_RMALL := rm -fr
export RBCONFIG_RMDIRS := rmdir -p
export RBCONFIG_RMDIR := rmdir
export RBCONFIG_CP := cp
export RBCONFIG_RM := rm -f
export RBCONFIG_PKG_CONFIG := pkg-config
export RBCONFIG_DOXYGEN :=
export RBCONFIG_DOT :=
export RBCONFIG_MKDIR_P := /opt/homebrew/bin/gmkdir -p
export RBCONFIG_INSTALL_DATA := /opt/homebrew/bin/ginstall -c -m 644
export RBCONFIG_INSTALL_SCRIPT := /opt/homebrew/bin/ginstall -c
export RBCONFIG_INSTALL_PROGRAM := /opt/homebrew/bin/ginstall -c
export RBCONFIG_SET_MAKE :=
export RBCONFIG_LN_S := ln -s
export RBCONFIG_DLLWRAP :=
export RBCONFIG_WINDRES :=
export RBCONFIG_ARFLAGS := rcu
export RBCONFIG_try_header :=
export RBCONFIG_CC_VERSION_MESSAGE := Apple clang version 17.0.0 (clang-1700.6.3.2)\nTarget: arm64-apple-darwin25.3.0\nThread model: posix\nInstalledDir: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin
export RBCONFIG_CC_VERSION := clang --version
export RBCONFIG_CSRCFLAG :=
export RBCONFIG_COUTFLAG := -o
export RBCONFIG_OUTFLAG := -o
export RBCONFIG_CPPOUTFILE := -o conftest.i
export RBCONFIG_GNU_LD := no
export RBCONFIG_GCC := yes
export RBCONFIG_CPP := clang -E
export RBCONFIG_CXXFLAGS := -fdeclspec
export RBCONFIG_OBJEXT := o
export RBCONFIG_CPPFLAGS := -D_XOPEN_SOURCE -D_DARWIN_C_SOURCE -D_DARWIN_UNLIMITED_SELECT -D_REENTRANT
export RBCONFIG_LDFLAGS := -L. -fstack-protector-strong -L/opt/homebrew/Cellar/gmp/6.3.0/lib
export RBCONFIG_CFLAGS := -fstack-protector-strong -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fdeclspec  -O3 -fno-fast-math -ggdb3 -Wall -Wextra -Wextra-tokens -Wdeprecated-declarations -Wdivision-by-zero -Wdiv-by-zero -Wimplicit-function-declaration -Wimplicit-int -Wpointer-arith -Wshorten-64-to-32 -Wwrite-strings -Wold-style-definition -Wmissing-noreturn -Wno-cast-function-type -Wno-constant-logical-operand -Wno-long-long -Wno-missing-field-initializers -Wno-overlength-strings -Wno-parentheses-equality -Wno-self-assign -Wno-tautological-compare -Wno-unused-parameter -Wno-unused-value -Wunused-variable -Wmisleading-indentation -Wundef -fno-common -pipe
export RBCONFIG_STRIP := strip -A -n
export RBCONFIG_RANLIB := ranlib
export RBCONFIG_OBJDUMP := objdump
export RBCONFIG_OBJCOPY := :
export RBCONFIG_NM := nm --no-llvm-bc
export RBCONFIG_LD := ld
export RBCONFIG_CXX := clang++
export RBCONFIG_AS := as
export RBCONFIG_AR := ar
export RBCONFIG_CC := clang
export RBCONFIG_wasmoptflags :=
export RBCONFIG_WASMOPT :=
export RBCONFIG_config_target := aarch64-apple-darwin25.3.0
export RBCONFIG_target_os := darwin25
export RBCONFIG_target_vendor := apple
export RBCONFIG_target_cpu := arm64
export RBCONFIG_target := arm64-apple-darwin25
export RBCONFIG_host_os := darwin25
export RBCONFIG_host_vendor := apple
export RBCONFIG_host_cpu := arm64
export RBCONFIG_host := arm64-apple-darwin25
export RBCONFIG_build_os := darwin25.3.0
export RBCONFIG_build_vendor := apple
export RBCONFIG_build_cpu := aarch64
export RBCONFIG_build := aarch64-apple-darwin25.3.0
export RBCONFIG_RUBY_VERSION_NAME := ruby-3.4.0
export RBCONFIG_RUBYW_BASE_NAME := rubyw
export RBCONFIG_RUBY_BASE_NAME := ruby
export RBCONFIG_RUBY_PROGRAM_VERSION := 3.4.8
export RBCONFIG_RUBY_API_VERSION := 3.4
export RBCONFIG_HAVE_GIT := yes
export RBCONFIG_GIT := git
export RBCONFIG_cxxflags :=
export RBCONFIG_cppflags :=
export RBCONFIG_cflags := -fstack-protector-strong -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=2 -fdeclspec  -O3 -fno-fast-math -ggdb3 -Wall -Wextra -Wextra-tokens -Wdeprecated-declarations -Wdivision-by-zero -Wdiv-by-zero -Wimplicit-function-declaration -Wimplicit-int -Wpointer-arith -Wshorten-64-to-32 -Wwrite-strings -Wold-style-definition -Wmissing-noreturn -Wno-cast-function-type -Wno-constant-logical-operand -Wno-long-long -Wno-missing-field-initializers -Wno-overlength-strings -Wno-parentheses-equality -Wno-self-assign -Wno-tautological-compare -Wno-unused-parameter -Wno-unused-value -Wunused-variable -Wmisleading-indentation -Wundef
export RBCONFIG_MAKEDIRS := /opt/homebrew/bin/gmkdir -p
export RBCONFIG_target_alias :=
export RBCONFIG_host_alias :=
export RBCONFIG_build_alias :=
export RBCONFIG_LIBS := -lpthread
export RBCONFIG_ECHO_T :=
export RBCONFIG_ECHO_N :=
export RBCONFIG_ECHO_C := \\c
export RBCONFIG_DEFS :=
export RBCONFIG_mandir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/man
export RBCONFIG_localedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/locale
export RBCONFIG_libdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib
export RBCONFIG_psdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/doc/ruby
export RBCONFIG_pdfdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/doc/ruby
export RBCONFIG_dvidir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/doc/ruby
export RBCONFIG_htmldir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/doc/ruby
export RBCONFIG_infodir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/info
export RBCONFIG_docdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share/doc/ruby
export RBCONFIG_oldincludedir := /usr/include
export RBCONFIG_includedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/include
export RBCONFIG_runstatedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/var/run
export RBCONFIG_localstatedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/var
export RBCONFIG_sharedstatedir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/com
export RBCONFIG_sysconfdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/etc
export RBCONFIG_datadir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share
export RBCONFIG_datarootdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/share
export RBCONFIG_libexecdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/libexec
export RBCONFIG_sbindir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/sbin
export RBCONFIG_bindir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/bin
export RBCONFIG_exec_prefix := /Users/naamanhirschfeld/.rbenv/versions/3.4.8
export RBCONFIG_PACKAGE_URL :=
export RBCONFIG_PACKAGE_BUGREPORT :=
export RBCONFIG_PACKAGE_STRING :=
export RBCONFIG_PACKAGE_VERSION :=
export RBCONFIG_PACKAGE_TARNAME :=
export RBCONFIG_PACKAGE_NAME :=
export RBCONFIG_PATH_SEPARATOR := :
export RBCONFIG_SHELL := /bin/sh
export RBCONFIG_UNICODE_VERSION := 15.0.0
export RBCONFIG_UNICODE_EMOJI_VERSION := 15.0
export RBCONFIG_SDKROOT :=
export RBCONFIG_platform := arm64-darwin25
export RBCONFIG_archdir := /Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib/ruby/3.4.0/arm64-darwin25
export RBCONFIG_topdir := .
export RBCONFIG_srcdir := .
export RUSTFLAGS := $(RB_SYS_GLOBAL_RUSTFLAGS) $(RB_SYS_EXTRA_RUSTFLAGS) $(RUSTFLAGS)

FORCE: ;

RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN ?= false

# Only run if the we are told to explicitly install the Rust toolchain
ifneq ($(RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN),false)
RB_SYS_RUSTUP_PROFILE ?= minimal

# If the user passed true, we assume stable Rust. Otherwise, use what
# was specified (i.e. RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN=beta)
ifeq ($(RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN),true)
  RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN = stable
endif

# If a $RUST_TARGET is specified (i.e. for rake-compiler-dock), append
# that to the profile.
ifeq ($(RUST_TARGET),)
  RB_SYS_DEFAULT_TOOLCHAIN = $(RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN)
else
  RB_SYS_DEFAULT_TOOLCHAIN = $(RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN)-$(RUST_TARGET)
endif

# Since we are forcing the installation of the Rust toolchain, we need
# to set these env vars unconditionally for the build.
export CARGO_HOME := $(RB_SYS_BUILD_DIR)/$(RB_SYS_DEFAULT_TOOLCHAIN)/cargo
export RUSTUP_HOME := $(RB_SYS_BUILD_DIR)/$(RB_SYS_DEFAULT_TOOLCHAIN)/rustup
export PATH := $(CARGO_HOME)/bin:$(RUSTUP_HOME)/bin:$(PATH)
export RUSTUP_TOOLCHAIN := $(RB_SYS_DEFAULT_TOOLCHAIN)
export CARGO := $(CARGO_HOME)/bin/cargo


$(CARGO):
	$(Q) $(MAKEDIRS) $(CARGO_HOME) $(RUSTUP_HOME)
	$(Q) curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" | sh -s -- --no-modify-path --profile $(RB_SYS_RUSTUP_PROFILE) --default-toolchain none -y
	$(Q) $(CARGO_HOME)/bin/rustup toolchain install $(RB_SYS_DEFAULT_TOOLCHAIN) --profile $(RB_SYS_RUSTUP_PROFILE) || (sleep 5; $(Q) $(CARGO_HOME)/bin/rustup toolchain install $(RB_SYS_DEFAULT_TOOLCHAIN) --profile $(RB_SYS_RUSTUP_PROFILE)) || (sleep 5; $(Q) $(CARGO_HOME)/bin/rustup toolchain install $(RB_SYS_DEFAULT_TOOLCHAIN) --profile $(RB_SYS_RUSTUP_PROFILE))
	$(Q) $(CARGO_HOME)/bin/rustup default $(RB_SYS_DEFAULT_TOOLCHAIN) || (sleep 5; $(Q) $(CARGO_HOME)/bin/rustup default $(RB_SYS_DEFAULT_TOOLCHAIN)) || (sleep 5; $(Q) $(CARGO_HOME)/bin/rustup default $(RB_SYS_DEFAULT_TOOLCHAIN))


$(RUSTLIB): $(CARGO)
endif


$(TIMESTAMP_DIR)/.sitearchdir.time:
	$(Q) $(MAKEDIRS) $(@D) $(RUBYARCHDIR)
	$(Q) $(TOUCH) $@

$(RUSTLIB): FORCE
	$(ECHO) generating $(@) \("$(RB_SYS_CARGO_PROFILE)"\)
	$(CARGO) rustc $(RB_SYS_EXTRA_CARGO_ARGS) --manifest-path $(RB_SYS_CARGO_MANIFEST_DIR)/Cargo.toml --target-dir $(RB_SYS_CARGO_TARGET_DIR) --lib $(RB_SYS_CARGO_PROFILE_FLAG) -- -C linker=clang -L native=/Users/naamanhirschfeld/.rbenv/versions/3.4.8/lib -L native=/opt/homebrew/Cellar/gmp/6.3.0/lib -C link-arg=-Wl,-undefined,dynamic_lookup -l pthread

$(DLLIB): $(RUSTLIB)
	$(Q) $(COPY) "$(RUSTLIB)" $@
	$(Q) $(POSTLINK)

install-so: $(DLLIB) $(TIMESTAMP_DIR)/.sitearchdir.time
	$(ECHO) installing $(DLLIB) to $(RUBYARCHDIR)
	$(Q) install_name_tool -id "" $(DLLIB)
	$(Q) $(MAKEDIRS) $(RUBYARCHDIR)
	$(INSTALL_PROG) $(DLLIB) $(RUBYARCHDIR)

gemclean: install-so
	$(ECHO) Cleaning gem artifacts
	-$(Q)$(RM_RF) $(RUBYGEMS_CLEAN_DIRS) 2> /dev/null || true

install: install-so

all: $(DLLIB)
