PREFIX ?= /usr/local
MANDIR ?= $(PREFIX)/share/man
TARGET ?= x86_64-unknown-linux-gnu

.PHONY: all install uninstall build build-arm64 build-armv7

build:
	cargo build --release --target $(TARGET)

build-arm64:
	RUSTFLAGS='-C linker=aarch64-linux-gnu-gcc' cargo build --release --target aarch64-unknown-linux-gnu

build-armv7:
	cargo build --release --target armv7-unknown-linux-gnueabihf

all: build

install:
	# Install binary
	install -d $(DESTDIR)$(PREFIX)/bin
	install -m 755 target/$(TARGET)/release/flux $(DESTDIR)$(PREFIX)/bin/

	# Install man page
	install -d $(DESTDIR)$(MANDIR)/man1
	install -m 644 resources/flux.1 $(DESTDIR)$(MANDIR)/man1/

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/flux
	rm -f $(DESTDIR)$(MANDIR)/man1/flux.1 

clean:
	cargo clean

distclean:
	cargo clean
	rm -rf target

format:
	cargo fmt