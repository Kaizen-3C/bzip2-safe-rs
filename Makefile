# bzip2-safe-rs — bzip2 1.0.8 codec, C -> safe Rust. Differential proof: byte-identical to upstream 1.0.8.
#   make check   build the C reference + the Rust port, run both over a shared corpus, require identical
#   make clean
CC      ?= clang
CFLAGS   = -O2 -w -DBZ_NO_STDIO -Ic/src
RUST_BIN = target/release/rdp

.PHONY: check clean
check: _b/ref $(RUST_BIN) ; @bash harness/check.sh

_b/ref: c/driver.c $(wildcard c/src/*.c)
	@mkdir -p _b
	$(CC) $(CFLAGS) $(wildcard c/src/*.c) c/driver.c -o $@

$(RUST_BIN): $(wildcard src/*.rs) Cargo.toml
	cargo build --release

clean: ; rm -rf _b target
