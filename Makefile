PREFIX ?= /usr
SYSCONFDIR ?= /etc
BINDIR = $(PREFIX)/bin
SHAREDIR = $(PREFIX)/share
XDG_CONFIG_HOME ?= $(HOME)/.config

APPNAME = strata
LUA_LIB = lua
ID = com.strata.Compositor
TARGET = release
DEBUG ?= 0

TARGET_BIN = $(DESTDIR)$(BINDIR)/$(APPNAME)
TARGET_LIB = $(DESTDIR)$(SHAREDIR)/$(APPNAME)/

ifeq ($(DEBUG),0)
	TARGET = release
	ARGS += --release
endif

all: $(APPNAME)

$(APPNAME):
	cargo build $(ARGS)

clean:
	cargo clean

install: $(APPNAME) install_lib
	install -Dm0755 "target/$(TARGET)/$(APPNAME)" "$(TARGET_BIN)"

install_lib:
	mkdir -p "$(TARGET_LIB)"
	cp -r "lua" "$(TARGET_LIB)"

install_default_config:
	mkdir -p "$(XDG_CONFIG_HOME)/$(APPNAME)"
	cp "./strata.default.lua" "$(XDG_CONFIG_HOME)/$(APPNAME)"

uninstall:
	rm -f "$(TARGET_BIN)"
	rm -rf "$(TARGET_LIB)"

.PHONY: all clean install uninstall
