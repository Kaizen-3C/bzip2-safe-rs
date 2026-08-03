# Third-party code & provenance

This repository contains **bzip2 1.0.8** (© 1996–2019 Julian R Seward), under the bzip2 license in
[`LICENSE`](LICENSE), and a Rust translation of it. Provenance is recorded here so it is verifiable, not
merely asserted.

## `c/src/` — upstream bzip2 1.0.8, unmodified

Every file is byte-for-byte identical to the corresponding file in the upstream bzip2 1.0.8 release. SHA-256:

| file | sha256 |
|------|--------|
| c/src/blocksort.c      | 4e48cd2ccff44699e67a7c949b0e9576c05b8dcbe20f863475c4fcc8db11a409 |
| c/src/bzlib.c          | d06cf1bd991df1f2dc8ef4f7713d186eb636767111cbd4807ef5fc4a54ca6838 |
| c/src/bzlib.h          | 6ac62e811669598ee30c9e1c379b9e627f6ff17a5a3dc1e0b4fa8b8ea75e580d |
| c/src/bzlib_private.h  | c0cda4f35ee1f2d54c9beacd524f8d28e0dbf8494aca30d854af3f143af4341b |
| c/src/compress.c       | 75995bd6e8c5f1e1dad05178f3cf53137df99ce860a1984324f78591f28deed3 |
| c/src/crctable.c       | 2fb7a564629386456e731f431a5cf4f5026747bace4cd10be8f5ecf082066a92 |
| c/src/decompress.c     | 31a89f8bf408ef0e4acae83e8be60a8eb4edece6c866d6e32b8f7e557ca54bc6 |
| c/src/huffman.c        | bdeb45f3f535546a672811b68aa87cc58fd395b28ecebc34fa3566a656a4d1d1 |
| c/src/randtable.c      | 407054ca6f54cd737dbc26ceb6b7874b55a0fcff86c2eb23cbec2fbdbb884815 |

The upstream `bzip2.c` command-line tool is intentionally not included — the scope is the codec (the
buffer-to-buffer API).

## `rust/src/main.rs` — a Rust translation of the bzip2 1.0.8 codec

This is a derivative work: a translation of the bzip2 1.0.8 sources above into Rust, implementing the same
buffer-API codec plus the differential line protocol. It is `#![forbid(unsafe_code)]`. As a derivative of
bzip2 it is distributed under the bzip2 license.

## Original work (© 2025 the authors, bzip2 license)

`c/driver.c`, everything under `harness/`, the `Makefile`, and `README.md` are original to this repository.

## Toolchain

`clang` (or any C compiler), a Rust toolchain (`cargo`), and `python3`, on Linux/WSL.
