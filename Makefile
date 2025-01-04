PREFIX ?= /usr/local
MANDIR ?= $(PREFIX)/share/man

.PHONY: all install uninstall build

build:
	cargo build --release

all: build

install:
	# Install binary
	install -d $(DESTDIR)$(PREFIX)/bin
	install -m 755 target/release/flux $(DESTDIR)$(PREFIX)/bin/

	# Install man page
	install -d $(DESTDIR)$(MANDIR)/man1
	install -m 644 docs/flux.1 $(DESTDIR)$(MANDIR)/man1/

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