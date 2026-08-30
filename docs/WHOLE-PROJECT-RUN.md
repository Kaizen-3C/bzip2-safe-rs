# Whole-project run — the schedule, the envelope, and what the differential caught

*Descriptive, never a correctness proof — `make check` is that. Companion to the module layout in
[`src/`](../src/) and the reference in [`c/`](../c/).*

## 1 · Scope and schedule

The port covers the bzip2 1.0.8 **library core** — eight translation units plus two shared headers —
together with the line-protocol driver in [`c/driver.c`](../c/driver.c) that the differential runs
against. The upstream command-line tools (`bzip2.c`, `bzip2recover.c`) are **not** in scope; see
[`THIRD_PARTY.md`](../THIRD_PARTY.md).

Units were ported **leaf-first**, in dependency order derived from the symbol graph rather than from file
names:

```
  1. bzlib_private   [header unit: shared types]   <- bzlib
  2. blocksort       <- bzlib_private
  3. crctable        <- bzlib_private
  4. huffman         <- bzlib_private
  5. randtable       <- bzlib_private
  6. compress        <- bzlib_private, blocksort, huffman
  7. decompress      <- bzlib_private, huffman
  8. bzlib           <- bzlib_private, compress, decompress
  9. driver          <- bzlib
```

`bzlib_private` and `bzlib` reference each other. The cycle is broken by promoting `bzlib_private` to a
**header unit** — shared types only — so the remaining graph is acyclic and every unit can be ported
against an already-settled dependency set.

## 2 · The envelope

**12 documents / 45 commands**, exercised through `c/driver.c`'s line protocol:

| Command | Covers |
|---|---|
| `R <level> <small> <hex>` | compress, then round-trip decompress — the compressed stream must match **byte for byte**, not merely round-trip |
| `D <level> <small> <hex>` | decompress arbitrary and corrupt streams — **error codes must match exactly** |
| `P <level> <small> <hex>` | parameter-error probes, including out-of-range levels |

Both `small` modes are exercised throughout. The malformed cases include empty input, bad magic, magic
only with the body truncated, garbage after valid magic under `small=1`, and raw binary garbage.

**Determinism gate.** The C reference is run twice over the whole envelope before any comparison is
accepted; the two passes must be identical. A reference that is not deterministic cannot be a reference.

*(The 265-command corpus behind `make check` is broader still — this envelope is the whole-project
integration set, not the ceiling.)*

## 3 · ⭐ What the whole-project differential caught

Both findings below were **invisible to per-module checking**: every module compiled, and each was
individually consistent with its own C. Only running the assembled program against the C reference
surfaced them.

### 3.1 An aliasing contract that lives *between* modules

Upstream `EState` deliberately aliases: `ptr`, `arr1`, `arr2` and `ftab` are **overlapping views of two
shared allocations** — `BZ2_bzCompressInit` in `bzlib.c` sets `ptr = (UInt32*)s->arr1`, and `blocksort.c`
then reads and writes through *both* names.

Translating each field to its own owned buffer is locally correct in every module and globally wrong.
`bzlib`'s initialisation sizes `arr1` and `arr2`; `blocksort` sorts through its own view; `ptr` is never
sized and stays empty. The failure surfaces as an out-of-bounds index in `BZ2_blockSort`'s tail, on the
very first compress command.

➜ **The contract is not stated in any single file.** It exists only in the relationship between
`bzlib.c`'s initialisation and `blocksort.c`'s use — which is precisely the class of property a
module-level test suite cannot express, and a whole-program differential catches on its first command.

### 3.2 A transposed index in the small-mode inverse-T construction

With compression already byte-exact, decompression under `small=1` still returned `BZ_DATA_ERROR` on
**valid** streams.

Upstream reads `uc = (UChar)(s->ll16[i])`, stores `SET_LL(i, cftabCopy[uc])`, increments
`cftabCopy[uc]`, and then builds the inverse T by pointer reversal. The port had the store's **index and
value transposed**.

➜ This is a one-argument error that **cannot** show up under `small=0`, produces no compile diagnostic,
and yields a plausible-looking error return rather than a crash. It is caught only by exercising the
small-mode path against the C on valid input and comparing bytes — which is why both `small` modes are in
the envelope rather than just the default.

## 4 · Upstream pin

Every file under `c/src/` is unmodified bzip2 1.0.8 (libarchive/bzip2, tag `bzip2-1.0.8`):

| File | sha256 (first 16) |
|---|---|
| `blocksort.c` | `4e48cd2ccff44699` |
| `bzlib.c` | `d06cf1bd991df1f2` |
| `compress.c` | `75995bd6e8c5f1e1` |
| `decompress.c` | `31a89f8bf408ef0e` |
| `huffman.c` | `bdeb45f3f535546a` |
| `crctable.c` | `2fb7a56462938645` |
| `randtable.c` | `407054ca6f54cd73` |
| `bzlib.h` | `6ac62e811669598e` |
| `bzlib_private.h` | `c0cda4f35ee1f2d5` |

Reproduce with `sha256sum c/src/*`.
