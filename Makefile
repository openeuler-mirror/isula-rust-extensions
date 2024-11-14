all: nri sandbox

nri:
	@cd nri && cargo build --release

sandbox:
	@cd sandbox && cargo build --release

install-nri:
	@install -p -m 644 nri/nri_plugin.h /usr/local/include/nri_plugin.h
	@install -p -m 755 nri/target/release/libisula_nri.so /usr/local/lib/

install-sandbox:
	@install -p -m 644 sandbox/isula_sandbox_api.h /usr/local/include/isula_sandbox_api.h
	@install -p -m 755 sandbox/target/release/libisula_sandbox.so /usr/local/lib/

install: install-nri install-sandbox

clean:
	@cd nri && cargo clean
	@cd sandbox && cargo clean

.PHONY: nri install-nri sandbox install-sandbox

