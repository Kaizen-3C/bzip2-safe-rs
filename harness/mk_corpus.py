#!/usr/bin/env python3
"""Generate a shared differential corpus for the C reference and the Rust port.

Every line is a command (see c/driver.c). Coverage is deliberately broad — both the standard (small=0) and
low-memory (small=1) decode paths, a range of block sizes including multi-block inputs, varied entropy, plus
malformed streams and invalid parameters so error codes are compared too. Deterministic (seeded)."""
import random

r = random.Random(20260801)
out = []


def R(data, level=9, small=0):
    out.append(f"R {level} {small} {data.hex() if data else '-'}")


# boundary sizes + both small flags
for sm in (0, 1):
    for data in (b"", b"\x00", b"A", b"Hello, world!", b"banana", b"abracadabra",
                 b"mississippi" * 40, b"the quick brown fox\n" * 50):
        R(data, 9, sm)
        R(data, 1, sm)

# RLE1 run-length regimes (the >=4 escape encoding), both small flags
for n in (3, 4, 5, 255, 259, 260, 600):
    R(b"A" * n, 9, 0)
    R(b"A" * n, 1, 1)

# multi-block inputs (> 100k forces multiple blocks at low levels), both small flags
for n in (90_000, 150_000, 250_000):
    R(bytes(r.getrandbits(8) for _ in range(n)), 1, 0)
    R(bytes(r.getrandbits(8) for _ in range(n)), 1, 1)

# randomized: varied size / level / small / entropy
for _ in range(200):
    n = r.choice([1, 2, 7, 13, 100, 999, 5000, 20000, 60000])
    kind = r.choice(["rand", "runs", "text", "sparse", "zeros"])
    if kind == "rand":
        p = bytes(r.getrandbits(8) for _ in range(n))
    elif kind == "runs":
        p = bytes([r.getrandbits(8)]) * n
    elif kind == "text":
        p = (b"To be, or not to be. " * (n // 21 + 1))[:n]
    elif kind == "sparse":
        b = bytearray(n)
        for _ in range(n // 50):
            if n:
                b[r.randrange(n)] = r.getrandbits(8)
        p = bytes(b)
    else:
        p = b"\x00" * n
    R(p, r.choice([1, 3, 6, 9]), r.choice([0, 1]))

# D: malformed / truncated streams -> exact error codes (both decode paths)
for sm in (0, 1):
    out.append(f"D 0 {sm} -")                                   # empty
    out.append(f"D 0 {sm} " + b"NOTBZIP2".hex())               # bad magic
    out.append(f"D 0 {sm} " + b"BZh9".hex())                   # magic only
    out.append(f"D 0 {sm} " + b"BZh1\x00\x00\x00\x00".hex())   # garbage after magic
    out.append(f"D 0 {sm} " + bytes(range(64)).hex())          # binary garbage

# P: invalid parameters
out.append("P 0 0 " + b"abc".hex())
out.append("P 10 0 " + b"abc".hex())
out.append("P -1 0 -")

print("\n".join(out))
