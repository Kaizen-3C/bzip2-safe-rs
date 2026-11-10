# Status

**Built + proven.** `make check` passes: the Rust port is byte-identical to bzip2 1.0.8 over 265 differential
commands (compress + decompress, `small=0`/`small=1`, valid + malformed streams), with `#![forbid(unsafe_code)]`
across the whole codec.

- **Layout:** re-cut to one file per module, following upstream bzip2 1.0.8's own split (`c/src/*.c`), so the
  port reads module-for-module against the C it came from.
- **Scope:** the bzip2 codec buffer API (`BZ2_bzBuffToBuffCompress`/`Decompress`). Not the stream/file API.
- **Sibling road:** the same bzip2 taken to hardened C, held to the same byte-identical standard.
