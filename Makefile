export prefix ?= /usr
sysconfdir ?= /etc
bindir = $(prefix)/bin
libdir = $(prefix)/lib/stratawm
sharedir = $(prefix)/share

BINARY = stratawm
LUA_LIB = lua
ID = com.strata.Compositor
TARGET = debug
DEBUG ?= 0

.PHONY = all clean install uninstall

ifeq ($(DEBUG),0)
	TARGET = release
	ARGS += --release
endif

TARGET_BIN="$(DESTDIR)$(bindir)/$(BINARY)"
TARGET_LIB="$(DESTDIR)$(libdir)/$(LUA_LIB)"

clean:
	cargo clean

install:
	install -Dm0755 "target/$(TARGET)/$(BINARY)" "$(TARGET_BIN)"
	install -Dm0644 "$(LUA_LIB)" "$(TARGET_LIB)/$(LUA_LIB)"

uninstall:
	rm "$(TARGET_BIN)"
