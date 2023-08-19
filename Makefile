PREFIX ?= /usr
SYSCONFDIR ?= /etc
BINDIR = $(PREFIX)/bin
SHAREDIR = $(PREFIX)/share

APPNAME = stratawm
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

install: $(APPNAME)
	cargo build --release
	install -Dm0755 "target/$(TARGET)/$(APPNAME)" "$(TARGET_BIN)"
	mkdir -p "$(TARGET_LIB)"
	cp -r "lua" "$(TARGET_LIB)"

uninstall:
	rm -f "$(TARGET_BIN)"
	rm -rf "$(TARGET_LIB)"

.PHONY: all clean install uninstall
