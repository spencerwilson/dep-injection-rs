.PHONY = all test doc

all:
	$(MAKE) -C rust_core
	./build.py
	clang -bundle -undefined dynamic_lookup ./ext_module.o -L/usr/local/lib -L/usr/local/opt/openssl/lib -L/usr/local/opt/sqlite/lib -L. -lrust_core -o ./ext_module.so

test: all
	clear
	python test.py

doc:
	pushd rust_core; \
	cargo doc; \
	open target/doc/rust_core/index.html; \
	popd
