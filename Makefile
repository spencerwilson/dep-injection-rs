.PHONY = all test

all:
	make rust_core
	./build.py
	clang -bundle -undefined dynamic_lookup ./ext_module.o -L/usr/local/lib -L/usr/local/opt/openssl/lib -L/usr/local/opt/sqlite/lib -L. -lrust_core -o ./ext_module.so

test: all
	python test.py
