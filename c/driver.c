/* Differential driver for the bzip2 1.0.8 codec (BZ_NO_STDIO buffer API).
   stdin -> stdout, one command per line. The Rust port implements the identical protocol, so the two
   programs' output can be compared byte-for-byte over a shared corpus.

     R <level> <small> <hex|->   compress(level, verbosity=0, workFactor=30) -> print
                                 "C <rc> <destLen> <hex>", then decompress(small) the stream and print
                                 "R <rc> <outLen> OK|MISMATCH"
     D <level> <small> <hex|->   treat hex as a compressed stream; decompress(small) into a 1 MiB buffer;
                                 print "D <rc> <outLen> <hex up to 64 bytes>"
     P <level> <small> <hex|->   parameter probe: compress with the given (possibly invalid) level;
                                 print "P <rc>"

   '-' means zero-length data. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "bzlib.h"

/* required by the library in the BZ_NO_STDIO configuration */
void bz_internal_error(int errcode) {
    printf("INTERNAL %d\n", errcode);
    exit(3);
}

static int hexval(int c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    return -1;
}

static void print_hex(const unsigned char *p, unsigned n, unsigned maxbytes) {
    unsigned i, lim = n < maxbytes ? n : maxbytes;
    for (i = 0; i < lim; i++) printf("%02x", p[i]);
    if (n > maxbytes) printf("..");
}

int main(void) {
    static char line[600000];
    while (fgets(line, sizeof line, stdin)) {
        char op;
        int level, small;
        char *hex;
        unsigned srcLen = 0;
        static unsigned char src[262144];
        static unsigned char comp[300000];
        static unsigned char back[1048576];
        unsigned compLen, backLen;
        int rc, i;

        char *tok = strtok(line, " \t\r\n");
        if (!tok) continue;
        op = tok[0];
        tok = strtok(NULL, " \t\r\n"); if (!tok) continue; level = atoi(tok);
        tok = strtok(NULL, " \t\r\n"); if (!tok) continue; small = atoi(tok);
        hex = strtok(NULL, " \t\r\n"); if (!hex) continue;

        if (strcmp(hex, "-") != 0) {
            for (i = 0; hex[i] && hex[i + 1]; i += 2) {
                int hi = hexval(hex[i]), lo = hexval(hex[i + 1]);
                if (hi < 0 || lo < 0) break;
                src[srcLen++] = (unsigned char)((hi << 4) | lo);
            }
        }

        if (op == 'R') {
            compLen = sizeof comp;
            rc = BZ2_bzBuffToBuffCompress((char *)comp, &compLen, (char *)src, srcLen,
                                          level, 0, 30);
            printf("C %d %u ", rc, rc == BZ_OK ? compLen : 0);
            if (rc == BZ_OK) print_hex(comp, compLen, 100000); else printf("-");
            printf("\n");
            if (rc == BZ_OK) {
                backLen = sizeof back;
                rc = BZ2_bzBuffToBuffDecompress((char *)back, &backLen, (char *)comp,
                                                compLen, small, 0);
                printf("R %d %u %s\n", rc, rc == BZ_OK ? backLen : 0,
                       (rc == BZ_OK && backLen == srcLen &&
                        memcmp(back, src, srcLen) == 0) ? "OK" : "MISMATCH");
            }
        } else if (op == 'D') {
            backLen = sizeof back;
            rc = BZ2_bzBuffToBuffDecompress((char *)back, &backLen, (char *)src, srcLen,
                                            small, 0);
            printf("D %d %u ", rc, rc == BZ_OK ? backLen : 0);
            if (rc == BZ_OK) print_hex(back, backLen, 64); else printf("-");
            printf("\n");
        } else if (op == 'P') {
            compLen = sizeof comp;
            rc = BZ2_bzBuffToBuffCompress((char *)comp, &compLen, (char *)src, srcLen,
                                          level, 0, 30);
            printf("P %d\n", rc);
        }
    }
    return 0;
}
