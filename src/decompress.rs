#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
use crate::bzlib_private::*;

pub fn make_maps_d(s: &mut DState) {
    s.nInUse = 0;
    for i in 0..256usize {
        if s.inUse[i] != 0 {
            s.seqToUnseq[s.nInUse as usize] = i as u8;
            s.nInUse += 1;
        }
    }
}

pub const BZ_OK: i32 = 0;
pub const BZ_STREAM_END: i32 = 4;
pub const BZ_DATA_ERROR: i32 = -4;
pub const BZ_DATA_ERROR_MAGIC: i32 = -5;
pub const BZ_MEM_ERROR: i32 = -3;

pub static BZ2_RNTABLE: [i32; 512] = [
    619, 720, 127, 481, 931, 816, 813, 233, 566, 247,
    985, 724, 205, 454, 863, 491, 741, 242, 949, 214,
    733, 859, 335, 708, 621, 574, 73, 654, 730, 472,
    419, 436, 278, 496, 867, 210, 399, 680, 480, 51,
    878, 465, 811, 169, 869, 675, 611, 697, 867, 561,
    862, 687, 507, 283, 482, 129, 807, 591, 733, 623,
    150, 238, 59, 379, 684, 877, 625, 169, 643, 105,
    170, 607, 520, 932, 727, 476, 693, 425, 174, 647,
    73, 122, 335, 530, 442, 853, 695, 249, 445, 515,
    909, 545, 703, 919, 874, 474, 882, 500, 594, 612,
    641, 801, 220, 162, 819, 984, 589, 513, 495, 799,
    161, 604, 958, 533, 221, 400, 386, 867, 600, 782,
    382, 596, 414, 171, 516, 375, 682, 485, 911, 276,
    98, 553, 163, 354, 666, 933, 424, 341, 533, 870,
    227, 730, 475, 186, 263, 647, 537, 686, 600, 224,
    469, 68, 770, 919, 190, 373, 294, 822, 808, 206,
    184, 943, 795, 384, 383, 461, 404, 758, 839, 887,
    715, 67, 618, 276, 204, 918, 873, 777, 604, 560,
    951, 160, 578, 722, 79, 804, 96, 409, 713, 940,
    652, 934, 970, 447, 318, 353, 859, 672, 112, 785,
    645, 863, 803, 350, 139, 93, 354, 99, 820, 908,
    609, 772, 154, 274, 580, 184, 79, 626, 630, 742,
    653, 282, 762, 623, 680, 81, 927, 626, 789, 125,
    411, 521, 938, 300, 821, 78, 343, 175, 128, 250,
    170, 774, 972, 275, 999, 639, 495, 78, 352, 126,
    857, 956, 358, 619, 580, 124, 737, 594, 701, 612,
    669, 112, 134, 694, 363, 992, 809, 743, 168, 974,
    944, 375, 748, 52, 600, 747, 642, 182, 862, 81,
    344, 805, 988, 739, 511, 655, 814, 334, 249, 515,
    897, 955, 664, 981, 649, 113, 974, 459, 893, 228,
    433, 837, 553, 268, 926, 240, 102, 654, 459, 51,
    686, 754, 806, 760, 493, 403, 415, 394, 687, 700,
    946, 670, 656, 610, 738, 392, 760, 799, 887, 653,
    978, 321, 576, 617, 626, 502, 894, 679, 243, 440,
    680, 879, 194, 572, 640, 724, 926, 56, 204, 700,
    707, 151, 457, 449, 797, 195, 791, 558, 945, 679,
    297, 59, 87, 824, 713, 663, 412, 693, 342, 606,
    134, 108, 571, 364, 631, 212, 174, 643, 304, 329,
    343, 97, 430, 751, 497, 314, 983, 374, 822, 928,
    140, 206, 73, 263, 980, 736, 876, 478, 430, 305,
    170, 514, 364, 692, 829, 82, 855, 953, 676, 246,
    369, 970, 294, 750, 807, 827, 150, 790, 288, 923,
    804, 378, 215, 828, 592, 281, 565, 555, 710, 82,
    896, 831, 547, 261, 524, 462, 293, 465, 502, 56,
    661, 821, 976, 991, 658, 869, 905, 758, 745, 193,
    768, 550, 608, 933, 378, 286, 215, 979, 792, 961,
    61, 688, 793, 644, 986, 403, 106, 366, 905, 644,
    372, 567, 466, 434, 645, 210, 389, 550, 919, 135,
    780, 773, 635, 389, 707, 100, 626, 958, 165, 504,
    920, 176, 193, 713, 857, 265, 203, 50, 668, 108,
    645, 990, 626, 197, 510, 357, 358, 850, 858, 364,
    936, 638,
];

#[inline]
pub fn get_ll(s: &DState, i: usize) -> i32 {
    let lo = s.ll16[i] as i32;
    let byte_idx = i >> 1;
    let shift = (i & 1) << 2;
    let hi = ((s.ll4[byte_idx] as i32) >> shift) & 0xf;
    (hi << 16) | lo
}

#[inline]
pub fn set_ll(s: &mut DState, i: usize, n: i32) {
    s.ll16[i] = n as u16;
    let byte_idx = i >> 1;
    let shift = (i & 1) << 2;
    s.ll4[byte_idx] = (s.ll4[byte_idx] & !(0xf << shift)) | (((n >> 16) as u8) << shift);
}

#[inline]
pub fn bz_get_fast(s: &mut DState) -> u8 {
    let tp = s.tPos as usize;
    s.tPos = s.tt[tp];
    let uc = (s.tPos & 0xff) as u8;
    s.tPos >>= 8;
    uc
}

#[inline]
pub fn bz_get_small(s: &mut DState) -> u8 {
    let tp = s.tPos as usize;
    let tpos = s.tPos as i32;
    let uc = crate::bzlib::BZ2_indexIntoF(tpos, &s.cftab);
    let new_tpos = get_ll(s, tp);
    s.tPos = new_tpos as u32;
    uc as u8
}

#[inline]
pub fn bz_rand_upd_mask(s: &mut DState) {
    if s.rNToGo == 0 {
        s.rNToGo = BZ2_RNTABLE[s.rTPos as usize];
        s.rTPos += 1;
        if s.rTPos == 512 { s.rTPos = 0; }
    }
    s.rNToGo -= 1;
}

#[inline]
pub fn bz_rand_mask(s: &DState) -> i32 {
    if s.rNToGo == 1 { 1 } else { 0 }
}

pub fn BZ2_decompress(s: &mut DState, strm: &mut crate::bzlib_private::bz_stream) -> i32 {
    let mut i = s.save_i;
    let mut j = s.save_j;
    let mut t = s.save_t;
    let mut alphaSize = s.save_alphaSize;
    let mut nGroups = s.save_nGroups;
    let mut nSelectors = s.save_nSelectors;
    let mut EOB = s.save_EOB;
    let mut groupNo = s.save_groupNo;
    let mut groupPos = s.save_groupPos;
    let mut nextSym = s.save_nextSym;
    let mut nblockMAX = s.save_nblockMAX;
    let mut nblock = s.save_nblock;
    let mut es = s.save_es;
    let mut N = s.save_N;
    let mut curr = s.save_curr;
    let mut zt = s.save_zt;
    let mut zn = s.save_zn;
    let mut zvec = s.save_zvec;
    let mut zj = s.save_zj;
    let mut gSel = s.save_gSel;
    let mut gMinlen = s.save_gMinlen;

    let mut uc: u8;

    let mut minLen: i32;
    let mut maxLen: i32;

    macro_rules! pull_byte {
        () => {{
            if strm.avail_in == 0 {
                false
            } else {
                let byte = strm.next_in.remove(0);
                s.bsBuff = (s.bsBuff << 8) | (byte as u32);
                s.bsLive += 8;
                strm.avail_in -= 1;
                strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
                if strm.total_in_lo32 == 0 { strm.total_in_hi32 += 1; }
                true
            }
        }};
    }

    macro_rules! save_and_return {
        ($rv:expr) => {{
            s.save_i = i; s.save_j = j; s.save_t = t;
            s.save_alphaSize = alphaSize; s.save_nGroups = nGroups;
            s.save_nSelectors = nSelectors; s.save_EOB = EOB;
            s.save_groupNo = groupNo; s.save_groupPos = groupPos;
            s.save_nextSym = nextSym; s.save_nblockMAX = nblockMAX;
            s.save_nblock = nblock; s.save_es = es; s.save_N = N;
            s.save_curr = curr; s.save_zt = zt; s.save_zn = zn;
            s.save_zvec = zvec; s.save_zj = zj; s.save_gSel = gSel;
            s.save_gMinlen = gMinlen;
            return $rv;
        }};
    }

    macro_rules! GB {
        ($nnn:expr) => {{
            let n = $nnn;
            while s.bsLive < n {
                if !pull_byte!() {
                    save_and_return!(BZ_OK);
                }
            }
            let v = (s.bsBuff >> (s.bsLive - n)) & ((1u32 << n) - 1);
            s.bsLive -= n;
            v as i32
        }};
    }

    if s.state == crate::bzlib_private::BZ_X_MAGIC_1 {
        uc = GB!(8) as u8;
        if uc != BZ_HDR_B { save_and_return!(BZ_DATA_ERROR_MAGIC); }
        uc = GB!(8) as u8;
        if uc != BZ_HDR_Z { save_and_return!(BZ_DATA_ERROR_MAGIC); }
        uc = GB!(8) as u8;
        if uc != BZ_HDR_h { save_and_return!(BZ_DATA_ERROR_MAGIC); }
        uc = GB!(8) as u8;
        if (uc as i32) < (BZ_HDR_0 as i32 + 1) || (uc as i32) > (BZ_HDR_0 as i32 + 9) {
            save_and_return!(BZ_DATA_ERROR_MAGIC);
        }
        s.blockSize100k = (uc as i32) - (BZ_HDR_0 as i32);

        if s.smallDecompress != 0 {
            s.ll16 = vec![0u16; (s.blockSize100k as usize) * 100000];
            s.ll4 = vec![0u8; ((1 + (s.blockSize100k as usize) * 100000) >> 1)];
        } else {
            s.tt = vec![0u32; (s.blockSize100k as usize) * 100000];
        }
    }

    'blocks: loop {
        uc = GB!(8) as u8;
        if uc == 0x17 {
            let u2 = GB!(8) as u8;
            if u2 != 0x72 { save_and_return!(BZ_DATA_ERROR); }
            let u3 = GB!(8) as u8;
            if u3 != 0x45 { save_and_return!(BZ_DATA_ERROR); }
            let u4 = GB!(8) as u8;
            if u4 != 0x38 { save_and_return!(BZ_DATA_ERROR); }
            let u5 = GB!(8) as u8;
            if u5 != 0x50 { save_and_return!(BZ_DATA_ERROR); }
            let u6 = GB!(8) as u8;
            if u6 != 0x90 { save_and_return!(BZ_DATA_ERROR); }
            s.storedCombinedCRC = GB!(8) as u32;
            s.storedCombinedCRC = (s.storedCombinedCRC << 8) | (GB!(8) as u32);
            s.storedCombinedCRC = (s.storedCombinedCRC << 8) | (GB!(8) as u32);
            s.storedCombinedCRC = (s.storedCombinedCRC << 8) | (GB!(8) as u32);
            s.state = crate::bzlib_private::BZ_X_IDLE;
            save_and_return!(BZ_STREAM_END);
        }
        if uc != 0x31 { save_and_return!(BZ_DATA_ERROR); }
        let u2 = GB!(8) as u8; if u2 != 0x41 { save_and_return!(BZ_DATA_ERROR); }
        let u3 = GB!(8) as u8; if u3 != 0x59 { save_and_return!(BZ_DATA_ERROR); }
        let u4 = GB!(8) as u8; if u4 != 0x26 { save_and_return!(BZ_DATA_ERROR); }
        let u5 = GB!(8) as u8; if u5 != 0x53 { save_and_return!(BZ_DATA_ERROR); }
        let u6 = GB!(8) as u8; if u6 != 0x59 { save_and_return!(BZ_DATA_ERROR); }

        s.currBlockNo += 1;

        s.storedBlockCRC = GB!(8) as u32;
        s.storedBlockCRC = (s.storedBlockCRC << 8) | (GB!(8) as u32);
        s.storedBlockCRC = (s.storedBlockCRC << 8) | (GB!(8) as u32);
        s.storedBlockCRC = (s.storedBlockCRC << 8) | (GB!(8) as u32);

        s.blockRandomised = GB!(1) as u8;

        s.origPtr = GB!(8);
        s.origPtr = (s.origPtr << 8) | GB!(8);
        s.origPtr = (s.origPtr << 8) | GB!(8);
        if s.origPtr < 0 { save_and_return!(BZ_DATA_ERROR); }
        if s.origPtr > 10 + 100000 * s.blockSize100k { save_and_return!(BZ_DATA_ERROR); }

        for ii in 0..16 {
            let bit = GB!(1);
            s.inUse16[ii] = if bit == 1 { 1 } else { 0 };
        }
        for ii in 0..256 { s.inUse[ii] = 0; }
        for ii in 0..16 {
            if s.inUse16[ii] != 0 {
                for jj in 0..16 {
                    let bit = GB!(1);
                    if bit == 1 { s.inUse[ii * 16 + jj] = 1; }
                }
            }
        }
        make_maps_d(s);
        if s.nInUse == 0 { save_and_return!(BZ_DATA_ERROR); }
        alphaSize = s.nInUse + 2;

        nGroups = GB!(3);
        if nGroups < 2 || nGroups > 6 { save_and_return!(BZ_DATA_ERROR); }
        nSelectors = GB!(15);
        if nSelectors < 1 { save_and_return!(BZ_DATA_ERROR); }
        {
            let mut pos = [0u8; BZ_N_GROUPS];
            for ii in 0..(nSelectors as usize) {
                let mut jj = 0i32;
                loop {
                    let bit = GB!(1);
                    if bit == 0 { break; }
                    jj += 1;
                    if jj >= nGroups { save_and_return!(BZ_DATA_ERROR); }
                }
                s.selectorMtf[ii] = jj as u8;
            }
            for v in 0..(nGroups as usize) { pos[v] = v as u8; }
            for ii in 0..(nSelectors as usize) {
                let mut v = s.selectorMtf[ii] as usize;
                let tmp = pos[v];
                while v > 0 { pos[v] = pos[v - 1]; v -= 1; }
                pos[0] = tmp;
                s.selector[ii] = tmp;
            }
        }

        for tt in 0..(nGroups as usize) {
            curr = GB!(5);
            for ii in 0..(alphaSize as usize) {
                loop {
                    if curr < 1 || curr > 20 { save_and_return!(BZ_DATA_ERROR); }
                    let bit = GB!(1);
                    if bit == 0 { break; }
                    let bit2 = GB!(1);
                    if bit2 == 0 { curr += 1; } else { curr -= 1; }
                }
                s.len[tt][ii] = curr as u8;
            }
        }

        for tt in 0..(nGroups as usize) {
            minLen = 32;
            maxLen = 0;
            for ii in 0..(alphaSize as usize) {
                if (s.len[tt][ii] as i32) > maxLen { maxLen = s.len[tt][ii] as i32; }
                if (s.len[tt][ii] as i32) < minLen { minLen = s.len[tt][ii] as i32; }
            }
            {
                let mut limit_arr = s.limit[tt];
                let mut base_arr = s.base[tt];
                let mut perm_arr = s.perm[tt];
                let len_arr = s.len[tt];
                crate::huffman::BZ2_hbCreateDecodeTables(
                    &mut limit_arr,
                    &mut base_arr,
                    &mut perm_arr,
                    &len_arr,
                    minLen,
                    maxLen,
                    alphaSize,
                );
                s.limit[tt] = limit_arr;
                s.base[tt] = base_arr;
                s.perm[tt] = perm_arr;
            }
            s.minLens[tt] = minLen;
        }

        EOB = s.nInUse + 1;
        nblockMAX = 100000 * s.blockSize100k;
        groupNo = -1;
        groupPos = 0;

        for ii in 0..256 { s.unzftab[ii] = 0; }

        {
            let mut kk: i32 = MTFA_SIZE as i32 - 1;
            for ii in (0..(256 / MTFL_SIZE) as i32).rev() {
                for jj in (0..MTFL_SIZE as i32).rev() {
                    s.mtfa[kk as usize] = (ii * (MTFL_SIZE as i32) + jj) as u8;
                    kk -= 1;
                }
                s.mtfbase[ii as usize] = kk + 1;
            }
        }

        nblock = 0;

        macro_rules! GET_MTF_VAL {
            () => {{
                if groupPos == 0 {
                    groupNo += 1;
                    if groupNo >= nSelectors { save_and_return!(BZ_DATA_ERROR); }
                    groupPos = BZ_G_SIZE;
                    gSel = s.selector[groupNo as usize] as i32;
                    gMinlen = s.minLens[gSel as usize];
                }
                groupPos -= 1;
                zn = gMinlen;
                zvec = GB!(zn);
                while zvec > s.limit[gSel as usize][zn as usize] {
                    zn += 1;
                    if zn > 20 { save_and_return!(BZ_DATA_ERROR); }
                    let bit = GB!(1);
                    zvec = (zvec << 1) | bit;
                }
                let idx = zvec - s.base[gSel as usize][zn as usize];
                if idx < 0 || idx >= alphaSize { save_and_return!(BZ_DATA_ERROR); }
                s.perm[gSel as usize][idx as usize]
            }};
        }

        nextSym = GET_MTF_VAL!();

        loop {
            if nextSym == EOB { break; }

            if nextSym == BZ_RUNA || nextSym == BZ_RUNB {
                es = -1;
                N = 1;
                loop {
                    if N < 0 { save_and_return!(BZ_DATA_ERROR); }
                    if nextSym == BZ_RUNA { es += N; }
                    else if nextSym == BZ_RUNB { es += 2 * N; }
                    N *= 2;
                    nextSym = GET_MTF_VAL!();
                    if nextSym != BZ_RUNA && nextSym != BZ_RUNB { break; }
                }

                es += 1;
                let seq = s.seqToUnseq[(s.mtfa[s.mtfbase[0] as usize]) as usize];
                s.unzftab[seq as usize] += es;

                if s.smallDecompress != 0 {
                    while es > 0 {
                        if nblock >= nblockMAX { save_and_return!(BZ_DATA_ERROR); }
                        set_ll(s, nblock as usize, seq as i32);
                        nblock += 1;
                        es -= 1;
                    }
                } else {
                    while es > 0 {
                        if nblock >= nblockMAX { save_and_return!(BZ_DATA_ERROR); }
                        s.tt[nblock as usize] = seq as u32;
                        nblock += 1;
                        es -= 1;
                    }
                }
                continue;
            } else {
                if nblock >= nblockMAX { save_and_return!(BZ_DATA_ERROR); }

                let nn = (nextSym - 1) as usize;
                let ucv: u8;
                if nn < MTFL_SIZE {
                    let mut nn2 = nn as i32;
                    let base0 = s.mtfbase[0] as usize;
                    let pp = base0 + nn2 as usize;
                    ucv = s.mtfa[pp];
                    while nn2 > 3 {
                        let z = base0 + nn2 as usize;
                        s.mtfa[z] = s.mtfa[z - 1];
                        s.mtfa[z - 1] = s.mtfa[z - 2];
                        s.mtfa[z - 2] = s.mtfa[z - 3];
                        s.mtfa[z - 3] = s.mtfa[z - 4];
                        nn2 -= 4;
                    }
                    while nn2 > 0 {
                        let z = base0 + nn2 as usize;
                        s.mtfa[z] = s.mtfa[z - 1];
                        nn2 -= 1;
                    }
                    s.mtfa[base0] = ucv;
                } else {
                    let mut nn2 = nn as i32;
                    let mut lno = nn2 / (MTFL_SIZE as i32);
                    let off = nn2 % (MTFL_SIZE as i32);
                    let pp = (s.mtfbase[lno as usize] + off) as usize;
                    ucv = s.mtfa[pp];
                    {
                        let mut p = pp;
                        let stop = s.mtfbase[lno as usize] as usize;
                        while p > stop {
                            s.mtfa[p] = s.mtfa[p - 1];
                            p -= 1;
                        }
                    }
                    s.mtfbase[lno as usize] += 1;
                    while lno > 0 {
                        s.mtfbase[lno as usize] -= 1;
                        let dst = s.mtfbase[lno as usize] as usize;
                        let src = (s.mtfbase[(lno - 1) as usize]
                            + (MTFL_SIZE as i32) - 1) as usize;
                        s.mtfa[dst] = s.mtfa[src];
                        lno -= 1;
                    }
                    s.mtfbase[0] -= 1;
                    s.mtfa[s.mtfbase[0] as usize] = ucv;
                    if s.mtfbase[0] == 0 {
                        let mut kk = MTFA_SIZE as i32 - 1;
                        for ii in (0..(256 / MTFL_SIZE) as i32).rev() {
                            for jj in (0..MTFL_SIZE as i32).rev() {
                                s.mtfa[kk as usize] = s.mtfa[(s.mtfbase[ii as usize] + jj) as usize];
                                kk -= 1;
                            }
                            s.mtfbase[ii as usize] = kk + 1;
                        }
                    }
                }

                let seq = s.seqToUnseq[ucv as usize];
                s.unzftab[seq as usize] += 1;
                if s.smallDecompress != 0 {
                    set_ll(s, nblock as usize, seq as i32);
                } else {
                    s.tt[nblock as usize] = seq as u32;
                }
                nblock += 1;

                nextSym = GET_MTF_VAL!();
                continue;
            }
        }

        if s.origPtr < 0 || s.origPtr >= nblock { save_and_return!(BZ_DATA_ERROR); }

        s.cftab[0] = 0;
        for ii in 1..=256usize { s.cftab[ii] = s.unzftab[ii - 1]; }
        for ii in 1..=256usize { s.cftab[ii] += s.cftab[ii - 1]; }
        for ii in 0..=256usize {
            if s.cftab[ii] < 0 || s.cftab[ii] > nblock { save_and_return!(BZ_DATA_ERROR); }
        }
        for ii in 1..=256usize {
            if s.cftab[ii - 1] > s.cftab[ii] { save_and_return!(BZ_DATA_ERROR); }
        }

        s.state_out_len = 0;
        s.state_out_ch = 0;
        s.calculatedBlockCRC = 0xffffffff;
        s.state = crate::bzlib_private::BZ_X_OUTPUT;

        if s.smallDecompress != 0 {
            // Make a copy of cftab, used in generation of T
            for ii in 0..=256usize { s.cftabCopy[ii] = s.cftab[ii]; }

            // compute the T vector
            for ii in 0..nblock {
                // C reads only the raw ll16 low byte: uc = (UChar)(s->ll16[i]).
                uc = (s.ll16[ii as usize] & 0xff) as u8;
                let cc = s.cftabCopy[uc as usize];
                set_ll(s, ii as usize, cc);
                s.cftabCopy[uc as usize] += 1;
            }

            // Compute T^(-1) by pointer reversal on T
            {
                let mut ir = s.origPtr;
                let mut jr = get_ll(s, ir as usize);
                loop {
                    let tmp = get_ll(s, jr as usize);
                    set_ll(s, jr as usize, ir);
                    ir = jr;
                    jr = tmp;
                    if ir == s.origPtr { break; }
                }
            }

            s.tPos = s.origPtr as u32;
            s.nblock_used = 0;
            if s.blockRandomised != 0 {
                s.rNToGo = 0; s.rTPos = 0;
                s.k0 = bz_get_small(s) as i32;
                bz_rand_upd_mask(s);
                s.k0 ^= bz_rand_mask(s);
                s.nblock_used += 1;
            } else {
                s.k0 = bz_get_small(s) as i32;
                s.nblock_used += 1;
            }
        } else {
            for ii in 0..nblock {
                uc = (s.tt[ii as usize] & 0xff) as u8;
                let cc = s.cftab[uc as usize] as usize;
                s.tt[cc] |= (ii as u32) << 8;
                s.cftab[uc as usize] += 1;
            }
            s.tPos = s.tt[s.origPtr as usize] >> 8;
            s.nblock_used = 0;
            if s.blockRandomised != 0 {
                s.rNToGo = 0; s.rTPos = 0;
                s.k0 = bz_get_fast(s) as i32;
                bz_rand_upd_mask(s);
                s.k0 ^= bz_rand_mask(s);
                s.nblock_used += 1;
            } else {
                s.k0 = bz_get_fast(s) as i32;
                s.nblock_used += 1;
            }
        }

        s.save_nblock = nblock;
        s.save_i = i; s.save_j = j; s.save_t = t;
        s.save_alphaSize = alphaSize; s.save_nGroups = nGroups;
        s.save_nSelectors = nSelectors; s.save_EOB = EOB;
        s.save_groupNo = groupNo; s.save_groupPos = groupPos;
        s.save_nextSym = nextSym; s.save_nblockMAX = nblockMAX;
        s.save_es = es; s.save_N = N;
        s.save_curr = curr; s.save_zt = zt; s.save_zn = zn;
        s.save_zvec = zvec; s.save_zj = zj; s.save_gSel = gSel;
        s.save_gMinlen = gMinlen;
        return BZ_OK;
    }
}
