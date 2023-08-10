export prefix ?= /usr
sysconfdir ?= /etc
bindir = $(prefix)/bin
libdir = $(prefix)/lib
sharedir = $(prefix)/share

BINARY = stratawm
ID = com.strata.Compositor
TARGET = debug
DEBUG ?= 0

.PHONY = all clean install uninstall

ifeq ($(DEBUG),0)
	TARGET = release
	ARGS += --release
endif

TARGET_BIN="$(DESTDIR)$(bindir)/$(BINARY)"

clean:
	cargo clean

install:
	install -Dm0755 "target/$(TARGET)/$(BINARY)" "$(TARGET_BIN)"

uninstall:
	rm "$(TARGET_BIN)"
