PREFIX ?= /usr
SYSCONFDIR ?= /etc
BINDIR = $(PREFIX)/bin
LIBDIR = $(PREFIX)/lib/stratawm
SHAREDIR = $(PREFIX)/share

BINARY = stratawm
LUA_LIB = lua
ID = com.strata.Compositor
TARGET = release
DEBUG ?= 0

TARGET_BIN = $(DESTDIR)$(BINDIR)/$(BINARY)
TARGET_LIB = $(DESTDIR)$(LIBDIR)

.PHONY: all clean install uninstall

ifeq ($(DEBUG),0)
	TARGET = release
	ARGS += --release
endif

all: $(BINARY)

$(BINARY):
	cargo build $(ARGS)

clean:
	cargo clean

install: $(BINARY)
	cargo build --release
	install -Dm0755 "target/$(TARGET)/$(BINARY)" "$(TARGET_BIN)"
	mkdir -p "$(TARGET_LIB)"
	cp -r "lua" "$(TARGET_LIB)"

uninstall:
	rm -f "$(TARGET_BIN)"
	rm -rf "$(TARGET_LIB)"

.PHONY: all clean install uninstall
