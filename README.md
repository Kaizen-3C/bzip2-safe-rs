# libbzip2-rs

**bzip2 1.0.8 codec, C → idiomatic safe Rust — proven byte-identical to the C reference.**

A whole-codec port of bzip2's buffer API (`BZ2_bzBuffToBuffCompress` / `Decompress`) to `#![forbid(unsafe_code)]`
Rust. The Rust output and upstream **bzip2 1.0.8** produce byte-for-byte identical results over a shared
differential corpus — compress and decompress, `small=0` and `small=1`, valid **and** malformed streams.

The module layout deliberately mirrors
[trifectatechfoundation/libbzip2-rs](https://github.com/trifectatechfoundation/libbzip2-rs) (`libbz2-rs-sys/src`)
so you can diff module-for-module: our [`src/decompress.rs`](src/decompress.rs) against theirs,
[`src/blocksort.rs`](src/blocksort.rs) against theirs, and so on.

## Verify it yourself

```bash
make check
```

Builds the C reference (upstream 1.0.8) and the Rust port, runs both over the corpus, and requires
byte-for-byte identical output:

```
PASS: Rust port byte-identical to bzip2 1.0.8 over 265 commands
      (compress + decompress, small=0 and small=1, valid + malformed streams; 0 unsafe)
```

## Layout

```
src/            the Rust codec — one file per module (mirrors libbz2-rs-sys/src)
  blocksort.rs  compress.rs  decompress.rs  huffman.rs
  crctable.rs   randtable.rs bzlib.rs       bzlib_private.rs
  lib.rs        crate root (#![forbid(unsafe_code)])
  main.rs       the R/D/P differential driver (bin `rdp`)
c/              pinned upstream bzip2 1.0.8 source + the C side of the differential driver
harness/        corpus generator + check runner
```

## Four roads, one subject
> **Staged release:** sibling repos shown **without a link** aren't public yet — they roll out over the coming weeks of the series.


This repo is the **C → Rust** road. Sibling repos take the same bzip2 to **memory-safe C**, to a lifted-from-**c2rust**
safe Rust, and to a **contract-honored** drop-in — the same subject reached four ways:

| | mechanism | on the CVE-2019-12900 selector overflow |
|---|---|---|
| **C 1.0.8** (baseline) | hand-added bounds guard | graceful error return |
| **libbzip2-safe-c** (C → safe-C) | the bound travels with the write (structural) | byte-identical, refuses the OOB |
| **libbzip2-rs** (this repo, C → Rust) | the language refuses the write | typed error / panic — no OOB by construction |
| **libbzip2-safe-rust** (c2rust → safe Rust) | the language refuses the write | typed error / panic — no OOB by construction |
| **libbzip2-contract-honored-rs** (C → contract-honored) | the language refuses the write | typed error / panic — no OOB by construction |
| **libbzip2-contract-honored-lift** (c2rust → contract-honored) | the language refuses the write | typed error / panic — no OOB by construction |

They keep this module layout, so you can diff `decompress` across the Rust roads and against `decompress.c` — and
against [trifectatechfoundation/libbzip2-rs](https://github.com/trifectatechfoundation/libbzip2-rs), the drop-in
this layout mirrors.

**Which one did you ask for?** All are byte-identical safe ports. This repo (and libbzip2-safe-rust) reach
zero `unsafe` by owning their own memory and de-aliasing bzip2's shared buffers — simplest, and clean over
multi-block inputs — but that *relaxes* what a drop-in `libbz2` caller asked for. If you need a drop-in that
honors the **caller's allocator** and bzip2's exact **memory profile** (aliased working buffers, the C ABI),
that's **libbzip2-contract-honored-rs**: it keeps a bounded,
**Miri-proven-sound** boundary `unsafe` instead (contract-honored; single-block today, multi-block its growing
frontier).

## Honest scope

- This is an empirical **differential** proof (byte-identical over a broad corpus), not a formal proof of
  equivalence.
- Performance is **near** the C reference, not a claim to beat it.
- Scope is the **codec buffer API**; the full stream/file API is not part of this port.

## License

Ported from bzip2 1.0.8; the port carries the upstream bzip2 license (see [LICENSE](LICENSE) and
[THIRD_PARTY.md](THIRD_PARTY.md)).
