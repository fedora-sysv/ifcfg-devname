NAME:=ifcfg-devname

all: production check

check:
	@cargo test

build:
	@cargo build

production:
	@cargo build --release

install:
	mkdir -p $(DESTDIR)/usr/lib/udev/rules.d
	install -p -m 0755 target/release/$(NAME) $(DESTDIR)/usr/lib/udev/
	install -p -m 644 rules/60-net.rules $(DESTDIR)/usr/lib/udev/rules.d/

uninstall:
	rm -f $(DESTDIR)/usr/lib/udev/$(NAME)
	rm -f $(DESTDIR)/usr/lib/udev/rules/60-net.rules

clean:
	@cargo clean

.PHONY: check build production install uninstall clean
