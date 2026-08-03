#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
use crate::bzlib_private::{
    EState, Bool,
    BZ_HDR_B, BZ_HDR_Z, BZ_HDR_h, BZ_HDR_0,
    BZ_MAX_ALPHA_SIZE, BZ_N_GROUPS, BZ_G_SIZE, BZ_N_ITERS, BZ_MAX_SELECTORS,
    BZ_RUNA, BZ_RUNB,
    True, False,
};

/*---------------------------------------------------*/
pub fn BZ2_bsInitWrite(s: &mut EState) {
    s.bsLive = 0;
    s.bsBuff = 0;
}

/*---------------------------------------------------*/
pub fn bs_finish_write(s: &mut EState) {
    while s.bsLive > 0 {
        s.zbits[s.numZ as usize] = (s.bsBuff >> 24) as u8;
        s.numZ += 1;
        s.bsBuff <<= 8;
        s.bsLive -= 8;
    }
}

/*---------------------------------------------------*/
#[inline]
pub fn bs_need_w(s: &mut EState) {
    while s.bsLive >= 8 {
        s.zbits[s.numZ as usize] = (s.bsBuff >> 24) as u8;
        s.numZ += 1;
        s.bsBuff <<= 8;
        s.bsLive -= 8;
    }
}

/*---------------------------------------------------*/
#[inline]
pub fn bs_w(s: &mut EState, n: i32, v: u32) {
    bs_need_w(s);
    s.bsBuff |= v << (32 - s.bsLive - n);
    s.bsLive += n;
}

/*---------------------------------------------------*/
pub fn bs_put_uint32(s: &mut EState, u: u32) {
    bs_w(s, 8, (u >> 24) & 0xff);
    bs_w(s, 8, (u >> 16) & 0xff);
    bs_w(s, 8, (u >>  8) & 0xff);
    bs_w(s, 8,  u        & 0xff);
}

/*---------------------------------------------------*/
pub fn bs_put_uchar(s: &mut EState, c: u8) {
    bs_w(s, 8, c as u32);
}

/*---------------------------------------------------*/
pub fn make_maps_e(s: &mut EState) {
    s.nInUse = 0;
    for i in 0..256 {
        if s.inUse[i] != 0 {
            s.unseqToSeq[i] = s.nInUse as u8;
            s.nInUse += 1;
        }
    }
}

/*---------------------------------------------------*/
pub fn generate_mtf_values(s: &mut EState) {
    let mut yy = [0u8; 256];
    let mut zPend: i32;
    let mut wr: i32;
    let EOB: i32;

    make_maps_e(s);
    EOB = s.nInUse + 1;

    for i in 0..=(EOB as usize) {
        s.mtfFreq[i] = 0;
    }

    wr = 0;
    zPend = 0;
    for i in 0..(s.nInUse as usize) {
        yy[i] = i as u8;
    }

    for i in 0..(s.nblock as usize) {
        let ll_i: u8;
        let mut j = s.ptr[i] as i32 - 1;
        if j < 0 { j += s.nblock; }
        ll_i = s.unseqToSeq[s.block[j as usize] as usize];

        if yy[0] == ll_i {
            zPend += 1;
        } else {
            if zPend > 0 {
                zPend -= 1;
                loop {
                    if zPend & 1 != 0 {
                        s.mtfv[wr as usize] = BZ_RUNB as u16;
                        wr += 1;
                        s.mtfFreq[BZ_RUNB as usize] += 1;
                    } else {
                        s.mtfv[wr as usize] = BZ_RUNA as u16;
                        wr += 1;
                        s.mtfFreq[BZ_RUNA as usize] += 1;
                    }
                    if zPend < 2 { break; }
                    zPend = (zPend - 2) / 2;
                }
                zPend = 0;
            }
            {
                let mut rtmp: u8 = yy[1];
                yy[1] = yy[0];
                let rll_i = ll_i;
                let mut ryy_j: usize = 1;
                while rll_i != rtmp {
                    ryy_j += 1;
                    let rtmp2 = rtmp;
                    rtmp = yy[ryy_j];
                    yy[ryy_j] = rtmp2;
                }
                yy[0] = rtmp;
                let j2 = ryy_j as i32;
                s.mtfv[wr as usize] = (j2 + 1) as u16;
                wr += 1;
                s.mtfFreq[(j2 + 1) as usize] += 1;
            }
        }
    }

    if zPend > 0 {
        zPend -= 1;
        loop {
            if zPend & 1 != 0 {
                s.mtfv[wr as usize] = BZ_RUNB as u16;
                wr += 1;
                s.mtfFreq[BZ_RUNB as usize] += 1;
            } else {
                s.mtfv[wr as usize] = BZ_RUNA as u16;
                wr += 1;
                s.mtfFreq[BZ_RUNA as usize] += 1;
            }
            if zPend < 2 { break; }
            zPend = (zPend - 2) / 2;
        }
    }

    s.mtfv[wr as usize] = EOB as u16;
    wr += 1;
    s.mtfFreq[EOB as usize] += 1;

    s.nMTF = wr;
}

/*---------------------------------------------------*/
pub const BZ_LESSER_ICOST: u8  = 0;
pub const BZ_GREATER_ICOST: u8 = 15;

pub fn send_mtf_values(s: &mut EState) {
    let mut cost = [0u16; BZ_N_GROUPS];
    let mut fave = [0i32; BZ_N_GROUPS];

    let alpha_size: i32 = s.nInUse + 2;
    let n_groups: i32;
    let mut n_selectors: i32 = 0;
    let mut gs: i32;
    let mut ge: i32;
    let mut totc: i32;
    let mut bt: i32;
    let mut bc: i32;

    // Initialize len tables
    for t in 0..BZ_N_GROUPS {
        for v in 0..(alpha_size as usize) {
            s.len[t][v] = BZ_GREATER_ICOST;
        }
    }

    // Decide number of coding tables
    n_groups = if s.nMTF < 200 { 2 }
               else if s.nMTF < 600 { 3 }
               else if s.nMTF < 1200 { 4 }
               else if s.nMTF < 2400 { 5 }
               else { 6 };

    // Generate initial set of coding tables
    {
        let mut n_part = n_groups;
        let mut rem_f = s.nMTF;
        gs = 0;
        while n_part > 0 {
            let t_freq = rem_f / n_part;
            ge = gs - 1;
            let mut a_freq: i32 = 0;
            while a_freq < t_freq && ge < alpha_size - 1 {
                ge += 1;
                a_freq += s.mtfFreq[ge as usize];
            }

            if ge > gs
                && n_part != n_groups
                && n_part != 1
                && ((n_groups - n_part) % 2 == 1)
            {
                a_freq -= s.mtfFreq[ge as usize];
                ge -= 1;
            }

            for v in 0..(alpha_size as usize) {
                if v as i32 >= gs && v as i32 <= ge {
                    s.len[n_part as usize - 1][v] = BZ_LESSER_ICOST;
                } else {
                    s.len[n_part as usize - 1][v] = BZ_GREATER_ICOST;
                }
            }

            n_part -= 1;
            gs = ge + 1;
            rem_f -= a_freq;
        }
    }

    // Iterate BZ_N_ITERS times to improve tables
    for _iter in 0..BZ_N_ITERS {
        for t in 0..(n_groups as usize) { fave[t] = 0; }
        for t in 0..(n_groups as usize) {
            for v in 0..(alpha_size as usize) {
                s.rfreq[t][v] = 0;
            }
        }

        // Set up auxiliary length table for fast track (nGroups == 6)
        if n_groups == 6 {
            for v in 0..(alpha_size as usize) {
                s.len_pack[v][0] = ((s.len[1][v] as u32) << 16) | (s.len[0][v] as u32);
                s.len_pack[v][1] = ((s.len[3][v] as u32) << 16) | (s.len[2][v] as u32);
                s.len_pack[v][2] = ((s.len[5][v] as u32) << 16) | (s.len[4][v] as u32);
            }
        }

        n_selectors = 0;
        totc = 0;
        gs = 0;
        loop {
            if gs >= s.nMTF { break; }
            ge = gs + BZ_G_SIZE - 1;
            if ge >= s.nMTF { ge = s.nMTF - 1; }

            for t in 0..(n_groups as usize) { cost[t] = 0; }

            if n_groups == 6 && 50 == ge - gs + 1 {
                let mut cost01: u32 = 0;
                let mut cost23: u32 = 0;
                let mut cost45: u32 = 0;

                macro_rules! BZ_ITER {
                    ($nn:expr) => {{
                        let icv = s.mtfv[(gs + $nn) as usize] as usize;
                        cost01 = cost01.wrapping_add(s.len_pack[icv][0]);
                        cost23 = cost23.wrapping_add(s.len_pack[icv][1]);
                        cost45 = cost45.wrapping_add(s.len_pack[icv][2]);
                    }}
                }

                BZ_ITER!(0);  BZ_ITER!(1);  BZ_ITER!(2);  BZ_ITER!(3);  BZ_ITER!(4);
                BZ_ITER!(5);  BZ_ITER!(6);  BZ_ITER!(7);  BZ_ITER!(8);  BZ_ITER!(9);
                BZ_ITER!(10); BZ_ITER!(11); BZ_ITER!(12); BZ_ITER!(13); BZ_ITER!(14);
                BZ_ITER!(15); BZ_ITER!(16); BZ_ITER!(17); BZ_ITER!(18); BZ_ITER!(19);
                BZ_ITER!(20); BZ_ITER!(21); BZ_ITER!(22); BZ_ITER!(23); BZ_ITER!(24);
                BZ_ITER!(25); BZ_ITER!(26); BZ_ITER!(27); BZ_ITER!(28); BZ_ITER!(29);
                BZ_ITER!(30); BZ_ITER!(31); BZ_ITER!(32); BZ_ITER!(33); BZ_ITER!(34);
                BZ_ITER!(35); BZ_ITER!(36); BZ_ITER!(37); BZ_ITER!(38); BZ_ITER!(39);
                BZ_ITER!(40); BZ_ITER!(41); BZ_ITER!(42); BZ_ITER!(43); BZ_ITER!(44);
                BZ_ITER!(45); BZ_ITER!(46); BZ_ITER!(47); BZ_ITER!(48); BZ_ITER!(49);

                cost[0] = (cost01 & 0xffff) as u16;
                cost[1] = (cost01 >> 16) as u16;
                cost[2] = (cost23 & 0xffff) as u16;
                cost[3] = (cost23 >> 16) as u16;
                cost[4] = (cost45 & 0xffff) as u16;
                cost[5] = (cost45 >> 16) as u16;
            } else {
                for i in gs..=ge {
                    let icv = s.mtfv[i as usize] as usize;
                    for t in 0..(n_groups as usize) {
                        cost[t] = cost[t].wrapping_add(s.len[t][icv] as u16);
                    }
                }
            }

            bc = 999999999;
            bt = -1;
            for t in 0..(n_groups as usize) {
                if (cost[t] as i32) < bc {
                    bc = cost[t] as i32;
                    bt = t as i32;
                }
            }
            totc += bc;
            fave[bt as usize] += 1;
            s.selector[n_selectors as usize] = bt as u8;
            n_selectors += 1;

            if n_groups == 6 && 50 == ge - gs + 1 {
                macro_rules! BZ_ITUR {
                    ($nn:expr) => {{
                        s.rfreq[bt as usize][s.mtfv[(gs + $nn) as usize] as usize] += 1;
                    }}
                }

                BZ_ITUR!(0);  BZ_ITUR!(1);  BZ_ITUR!(2);  BZ_ITUR!(3);  BZ_ITUR!(4);
                BZ_ITUR!(5);  BZ_ITUR!(6);  BZ_ITUR!(7);  BZ_ITUR!(8);  BZ_ITUR!(9);
                BZ_ITUR!(10); BZ_ITUR!(11); BZ_ITUR!(12); BZ_ITUR!(13); BZ_ITUR!(14);
                BZ_ITUR!(15); BZ_ITUR!(16); BZ_ITUR!(17); BZ_ITUR!(18); BZ_ITUR!(19);
                BZ_ITUR!(20); BZ_ITUR!(21); BZ_ITUR!(22); BZ_ITUR!(23); BZ_ITUR!(24);
                BZ_ITUR!(25); BZ_ITUR!(26); BZ_ITUR!(27); BZ_ITUR!(28); BZ_ITUR!(29);
                BZ_ITUR!(30); BZ_ITUR!(31); BZ_ITUR!(32); BZ_ITUR!(33); BZ_ITUR!(34);
                BZ_ITUR!(35); BZ_ITUR!(36); BZ_ITUR!(37); BZ_ITUR!(38); BZ_ITUR!(39);
                BZ_ITUR!(40); BZ_ITUR!(41); BZ_ITUR!(42); BZ_ITUR!(43); BZ_ITUR!(44);
                BZ_ITUR!(45); BZ_ITUR!(46); BZ_ITUR!(47); BZ_ITUR!(48); BZ_ITUR!(49);
            } else {
                for i in gs..=ge {
                    s.rfreq[bt as usize][s.mtfv[i as usize] as usize] += 1;
                }
            }

            gs = ge + 1;
        }

        // Recompute tables
        for t in 0..(n_groups as usize) {
            crate::huffman::BZ2_hbMakeCodeLengths(
                &mut s.len[t][0..alpha_size as usize],
                &s.rfreq[t][0..alpha_size as usize],
                alpha_size,
                17,
            );
        }
    }

    // Compute MTF values for selectors
    {
        let mut pos = [0u8; BZ_N_GROUPS];
        for i in 0..(n_groups as usize) { pos[i] = i as u8; }
        for i in 0..(n_selectors as usize) {
            let ll_i = s.selector[i];
            let mut j: usize = 0;
            let mut tmp = pos[j];
            while ll_i != tmp {
                j += 1;
                let tmp2 = tmp;
                tmp = pos[j];
                pos[j] = tmp2;
            }
            pos[0] = tmp;
            s.selectorMtf[i] = j as u8;
        }
    }

    // Assign actual codes for the tables
    for t in 0..(n_groups as usize) {
        let mut min_len: i32 = 32;
        let mut max_len: i32 = 0;
        for i in 0..(alpha_size as usize) {
            if (s.len[t][i] as i32) > max_len { max_len = s.len[t][i] as i32; }
            if (s.len[t][i] as i32) < min_len { min_len = s.len[t][i] as i32; }
        }
        crate::huffman::BZ2_hbAssignCodes(
            &mut s.code[t][0..alpha_size as usize],
            &s.len[t][0..alpha_size as usize],
            min_len,
            max_len,
            alpha_size,
        );
    }

    // Transmit the mapping table
    {
        let mut in_use16 = [false; 16];
        for i in 0..16usize {
            in_use16[i] = false;
            for j in 0..16usize {
                if s.inUse[i * 16 + j] != 0 { in_use16[i] = true; }
            }
        }

        let _n_bytes = s.numZ;
        for i in 0..16 {
            if in_use16[i] { bs_w(s, 1, 1); } else { bs_w(s, 1, 0); }
        }
        for i in 0..16 {
            if in_use16[i] {
                for j in 0..16 {
                    if s.inUse[i * 16 + j] != 0 { bs_w(s, 1, 1); } else { bs_w(s, 1, 0); }
                }
            }
        }
    }

    // Now the selectors
    let _n_bytes = s.numZ;
    bs_w(s, 3, n_groups as u32);
    bs_w(s, 15, n_selectors as u32);
    for i in 0..(n_selectors as usize) {
        for _j in 0..(s.selectorMtf[i] as usize) {
            bs_w(s, 1, 1);
        }
        bs_w(s, 1, 0);
    }

    // Now the coding tables
    let _n_bytes2 = s.numZ;
    for t in 0..(n_groups as usize) {
        let mut curr = s.len[t][0] as i32;
        bs_w(s, 5, curr as u32);
        for i in 0..(alpha_size as usize) {
            while curr < s.len[t][i] as i32 { bs_w(s, 2, 2); curr += 1; }
            while curr > s.len[t][i] as i32 { bs_w(s, 2, 3); curr -= 1; }
            bs_w(s, 1, 0);
        }
    }

    // And finally, the block data proper
    let _n_bytes3 = s.numZ;
    let mut sel_ctr: usize = 0;
    gs = 0;
    loop {
        if gs >= s.nMTF { break; }
        ge = gs + BZ_G_SIZE - 1;
        if ge >= s.nMTF { ge = s.nMTF - 1; }

        if n_groups == 6 && 50 == ge - gs + 1 {
            let sel = s.selector[sel_ctr] as usize;
            macro_rules! BZ_ITAH {
                ($nn:expr) => {{
                    let mtfv_i = s.mtfv[(gs + $nn) as usize] as usize;
                    let l = s.len[sel][mtfv_i] as i32;
                    let c = s.code[sel][mtfv_i];
                    bs_w(s, l, c as u32);
                }}
            }

            BZ_ITAH!(0);  BZ_ITAH!(1);  BZ_ITAH!(2);  BZ_ITAH!(3);  BZ_ITAH!(4);
            BZ_ITAH!(5);  BZ_ITAH!(6);  BZ_ITAH!(7);  BZ_ITAH!(8);  BZ_ITAH!(9);
            BZ_ITAH!(10); BZ_ITAH!(11); BZ_ITAH!(12); BZ_ITAH!(13); BZ_ITAH!(14);
            BZ_ITAH!(15); BZ_ITAH!(16); BZ_ITAH!(17); BZ_ITAH!(18); BZ_ITAH!(19);
            BZ_ITAH!(20); BZ_ITAH!(21); BZ_ITAH!(22); BZ_ITAH!(23); BZ_ITAH!(24);
            BZ_ITAH!(25); BZ_ITAH!(26); BZ_ITAH!(27); BZ_ITAH!(28); BZ_ITAH!(29);
            BZ_ITAH!(30); BZ_ITAH!(31); BZ_ITAH!(32); BZ_ITAH!(33); BZ_ITAH!(34);
            BZ_ITAH!(35); BZ_ITAH!(36); BZ_ITAH!(37); BZ_ITAH!(38); BZ_ITAH!(39);
            BZ_ITAH!(40); BZ_ITAH!(41); BZ_ITAH!(42); BZ_ITAH!(43); BZ_ITAH!(44);
            BZ_ITAH!(45); BZ_ITAH!(46); BZ_ITAH!(47); BZ_ITAH!(48); BZ_ITAH!(49);
        } else {
            let sel = s.selector[sel_ctr] as usize;
            for i in gs..=ge {
                let mv = s.mtfv[i as usize] as usize;
                let l = s.len[sel][mv] as i32;
                let c = s.code[sel][mv];
                bs_w(s, l, c as u32);
            }
        }

        gs = ge + 1;
        sel_ctr += 1;
    }
}

/*---------------------------------------------------*/
pub fn BZ2_compressBlock(s: &mut EState, is_last_block: Bool) {
    if s.nblock > 0 {
        // BZ_FINALISE_CRC macro: s->blockCRC = ~s->blockCRC
        s.blockCRC = !s.blockCRC;

        s.combinedCRC = (s.combinedCRC << 1) | (s.combinedCRC >> 31);
        s.combinedCRC ^= s.blockCRC;
        if s.blockNo > 1 { s.numZ = 0; }

        crate::blocksort::BZ2_blockSort(s);
    }

    /* In C, zbits = (UChar*)(&((UChar*)s->arr2)[s->nblock]) and
       mtfv = (UInt16*)s->arr1.  In this port arr1/arr2 are held as
       separate Vecs, so we allocate the aliased output buffers here.
       zbits must be large enough for the compressed output; mtfv must
       hold up to nblock+1 MTF symbols. */
    {
        // arr2 is Vec<u32> of length (n + BZ_N_OVERSHOOT); as bytes that is 4x.
        // zbits aliases the region starting at byte offset nblock, so its
        // available length is (arr2.len()*4 - nblock).
        let zcap = s.arr2.len() * 4;
        let zlen = if zcap > s.nblock as usize {
            zcap - s.nblock as usize
        } else {
            0
        };
        // Ensure a generous minimum so the bit-stream writer never overflows
        // for tiny blocks (header/trailer bytes for empty streams etc).
        let needed = if zlen < 1024 { 1024 } else { zlen };
        if s.zbits.len() < needed {
            s.zbits = vec![0u8; needed];
        }
        // mtfv holds ptr-sized number of symbols (nblock + 1 for EOB).
        let mneeded = (s.nblock as usize) + 2;
        if s.mtfv.len() < mneeded {
            s.mtfv = vec![0u16; mneeded];
        }
    }

    if s.blockNo == 1 {
        BZ2_bsInitWrite(s);
        bs_put_uchar(s, BZ_HDR_B);
        bs_put_uchar(s, BZ_HDR_Z);
        bs_put_uchar(s, BZ_HDR_h);
        bs_put_uchar(s, BZ_HDR_0 + s.blockSize100k as u8);
    }

    if s.nblock > 0 {
        bs_put_uchar(s, 0x31); bs_put_uchar(s, 0x41);
        bs_put_uchar(s, 0x59); bs_put_uchar(s, 0x26);
        bs_put_uchar(s, 0x53); bs_put_uchar(s, 0x59);

        bs_put_uint32(s, s.blockCRC);

        bs_w(s, 1, 0);

        bs_w(s, 24, s.origPtr as u32);
        generate_mtf_values(s);
        send_mtf_values(s);
    }

    if is_last_block != 0 {
        bs_put_uchar(s, 0x17); bs_put_uchar(s, 0x72);
        bs_put_uchar(s, 0x45); bs_put_uchar(s, 0x38);
        bs_put_uchar(s, 0x50); bs_put_uchar(s, 0x90);
        bs_put_uint32(s, s.combinedCRC);
        bs_finish_write(s);
    }
}
