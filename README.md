# bzip2-safe-rs

[![CI — differential vs bzip2 1.0.8](https://github.com/Kaizen-3C/bzip2-safe-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Kaizen-3C/bzip2-safe-rs/actions/workflows/ci.yml)
[![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](src/lib.rs)

**bzip2 1.0.8 codec, C → idiomatic safe Rust — proven byte-identical to the C reference.**

A whole-codec port of bzip2's buffer API (`BZ2_bzBuffToBuffCompress` / `Decompress`) to `#![forbid(unsafe_code)]`
Rust. The Rust output and upstream **bzip2 1.0.8** produce byte-for-byte identical results over a shared
differential corpus — compress and decompress, `small=0` and `small=1`, valid **and** malformed streams.

The module layout is upstream bzip2's own — one Rust file per C file — so you can read the port against the
source it came from: [`src/decompress.rs`](src/decompress.rs) against `c/src/decompress.c`,
[`src/blocksort.rs`](src/blocksort.rs) against `c/src/blocksort.c`, and so on. Any other port that keeps
upstream's split, [trifectatechfoundation/libbzip2-rs](https://github.com/trifectatechfoundation/libbzip2-rs)
among them, can be read module-for-module the same way.

## Verify it yourself

```bash
make check
```

It also runs on every push and pull request, and once a week on its own, against both
`clang` and `gcc` on x86-64 and arm64 — so a toolchain change that breaks the check
shows up here rather than on your machine.

Builds the C reference (upstream 1.0.8) and the Rust port, runs both over the corpus, and requires
byte-for-byte identical output:

```
PASS: Rust port byte-identical to bzip2 1.0.8 over 265 commands
      (compress + decompress, small=0 and small=1, valid + malformed streams; 0 unsafe)
```

For how the port was structured — the leaf-first dependency schedule, the integration envelope, and the two
defects the whole-project differential caught that per-module checking could not — see
[`docs/WHOLE-PROJECT-RUN.md`](docs/WHOLE-PROJECT-RUN.md).

## Layout

```
src/            the Rust codec — one file per module (mirrors libbz2-rs-sys/src)
  blocksort.rs  compress.rs  decompress.rs  huffman.rs
  crctable.rs   randtable.rs bzlib.rs       bzlib_private.rs
  lib.rs        crate root (#![forbid(unsafe_code)])
  main.rs       the R/D/P differential driver (bin `rdp`)
c/              pinned upstream bzip2 1.0.8 source + the C side of the differential driver
harness/        corpus generator + check runner
docs/           how the port was scheduled, and what the whole-project differential caught
```

## One subject, more than one destination

The same bzip2 can be taken to **safe Rust** (this repo) or to **hardened C** — and a drop-in that honors
the caller's allocator and bzip2's exact memory profile (the aliased working buffers, the C ABI) is a third
destination with a different trade-off. All are held to the same byte-identical standard against the C
reference.

This repo is the simplest of them: it owns its own memory and de-aliases bzip2's shared buffers, which reaches
zero `unsafe` and stays clean over multi-block inputs — but that *relaxes* what a drop-in `libbz2` caller
asked for. If you need the drop-in contract kept exactly, that is the third destination, not this one.

The module layout is the same across destinations — upstream's — so you can diff `decompress` between them,
and each of them against `decompress.c`.

## Honest scope

- This is an empirical **differential** proof (byte-identical over a broad corpus), not a formal proof of
  equivalence.
- Performance is **near** the C reference, not a claim to beat it.
- Scope is the **codec buffer API**; the full stream/file API is not part of this port.

## License

Ported from bzip2 1.0.8; the port carries the upstream bzip2 license (see [LICENSE](LICENSE) and
[THIRD_PARTY.md](THIRD_PARTY.md)).
