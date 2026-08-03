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

## Three roads, one subject

This repo is the **C → Rust** road. A sibling repo takes the same bzip2 to **memory-safe C** —
keeping the language and ABI while removing the memory-unsafety by construction:

| | mechanism | on the CVE-2019-12900 selector overflow |
|---|---|---|
| **C 1.0.8** (baseline) | hand-added bounds guard | graceful error return |
| **[libbzip2-safe-c](https://github.com/Kaizen-3C/libbzip2-safe-c)** (C → safe-C) | the bound travels with the write (structural) | byte-identical, refuses the OOB |
| **libbzip2-rs** (this repo, C → Rust) | the language refuses the write | typed error / panic — no OOB by construction |

A third road — lifting an existing **c2rust** transliteration to safe Rust — is the active frontier and not
published here.

## Honest scope

- This is an empirical **differential** proof (byte-identical over a broad corpus), not a formal proof of
  equivalence.
- Performance is **near** the C reference, not a claim to beat it.
- Scope is the **codec buffer API**; the full stream/file API is not part of this port.

## License

Ported from bzip2 1.0.8; the port carries the upstream bzip2 license (see [LICENSE](LICENSE) and
[THIRD_PARTY.md](THIRD_PARTY.md)).
