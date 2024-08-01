all: nri

nri:
	@cd nri && cargo build --release

install-nri:
	@install -p -m 644 nri/nri_plugin.h /usr/local/include/nri_plugin.h
	@install -p -m 550 nri/target/release/libisula_nri.so /usr/local/lib/

install: install-nri

clean:
	@cd nri && cargo clean

.PHONY: nri install-nri