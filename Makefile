PREFIX?=/usr/local
DESTDIR?=
BINDIR=$(DESTDIR)$(PREFIX)/bin/

all:
	CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo b -r

clean:
	rm -rf b

install:
	mkdir -p $(BINDIR)
	cp -f b/target/release/radius2 $(BINDIR)/radius2

uninstall:
	rm -f $(BINDIR)/radius2

.PHONY: all clean install uninstall
