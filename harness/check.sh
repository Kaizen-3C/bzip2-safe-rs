#!/usr/bin/env bash
# Run the C reference and the Rust port over the same corpus; require byte-for-byte identical output.
# On Linux both emit LF, so a plain diff is exact — no normalization.
set -u
REF=_b/ref
PORT=target/release/rdp

python3 harness/mk_corpus.py > _b/corpus.txt
n=$(wc -l < _b/corpus.txt | tr -d ' ')

"$REF"  < _b/corpus.txt > _b/out_c.txt
"$PORT" < _b/corpus.txt > _b/out_rust.txt

if diff -q _b/out_c.txt _b/out_rust.txt >/dev/null; then
    echo "PASS: Rust port byte-identical to bzip2 1.0.8 over $n commands"
    echo "      (compress + decompress, small=0 and small=1, valid + malformed streams; 0 unsafe)"
    exit 0
fi
echo "FAIL: outputs differ"
diff _b/out_c.txt _b/out_rust.txt | head -30
exit 1
