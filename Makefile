.PHONY = all build test doc

export OUT_DIR = build

all: build test

build:
	mkdir -p $(OUT_DIR)
	touch $(OUT_DIR)/__init__.py
	$(MAKE) -C rust_core
	./build.py

test:
	python test.py

clean:
	rm -rf $(OUT_DIR)

doc:
	pushd rust_core; \
	cargo doc; \
	open target/doc/rust_core/index.html; \
	popd
