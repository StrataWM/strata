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
TARGET_LIB = $(DESTDIR)$(LIBDIR)/$(LUA_LIB)

.PHONY: all clean install uninstall

ifeq ($(DEBUG),0)
	TARGET = release
	ARGS += --release
endif

all: $(BINARY)

$(BINARY): # Add dependencies here if needed
	cargo build $(ARGS)

clean:
	cargo clean

install: $(BINARY)
	install -Dm0755 "target/$(TARGET)/$(BINARY)" "$(TARGET_BIN)"
	cp -r "lua" "$(TARGET_LIB)"

uninstall:
	rm -f "$(TARGET_BIN)"

.PHONY: all clean install uninstall
