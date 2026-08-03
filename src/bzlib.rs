#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
use crate::bzlib_private::bz_stream;
use crate::bzlib_private::EState;
use crate::bzlib_private::DState;
use crate::bzlib_private::{
    True, False,
    BZ_M_IDLE, BZ_M_RUNNING, BZ_M_FLUSHING, BZ_M_FINISHING,
    BZ_S_OUTPUT, BZ_S_INPUT,
    BZ_N_OVERSHOOT,
    BZ_X_IDLE, BZ_X_OUTPUT, BZ_X_MAGIC_1, BZ_X_BLKHDR_1,
    BZ_VERSION,
};

pub const BZ_OK: i32 = 0;
pub const BZ_RUN_OK: i32 = 1;
pub const BZ_FLUSH_OK: i32 = 2;
pub const BZ_FINISH_OK: i32 = 3;
pub const BZ_STREAM_END: i32 = 4;
pub const BZ_SEQUENCE_ERROR: i32 = -1;
pub const BZ_PARAM_ERROR: i32 = -2;
pub const BZ_MEM_ERROR: i32 = -3;
pub const BZ_DATA_ERROR: i32 = -4;
pub const BZ_DATA_ERROR_MAGIC: i32 = -5;
pub const BZ_IO_ERROR: i32 = -6;
pub const BZ_UNEXPECTED_EOF: i32 = -7;
pub const BZ_OUTBUFF_FULL: i32 = -8;
pub const BZ_CONFIG_ERROR: i32 = -9;

pub const BZ_RUN: i32 = 0;
pub const BZ_FLUSH: i32 = 1;
pub const BZ_FINISH: i32 = 2;

/*---------------------------------------------------*/
pub static BZ2_CRC32TABLE: [u32; 256] = [
    0x00000000, 0x04c11db7, 0x09823b6e, 0x0d4326d9,
    0x130476dc, 0x17c56b6b, 0x1a864db2, 0x1e475005,
    0x2608edb8, 0x22c9f00f, 0x2f8ad6d6, 0x2b4bcb61,
    0x350c9b64, 0x31cd86d3, 0x3c8ea00a, 0x384fbdbd,
    0x4c11db70, 0x48d0c6c7, 0x4593e01e, 0x4152fda9,
    0x5f15adac, 0x5bd4b01b, 0x569796c2, 0x52568b75,
    0x6a1936c8, 0x6ed82b7f, 0x639b0da6, 0x675a1011,
    0x791d4014, 0x7ddc5da3, 0x709f7b7a, 0x745e66cd,
    0x9823b6e0, 0x9ce2ab57, 0x91a18d8e, 0x95609039,
    0x8b27c03c, 0x8fe6dd8b, 0x82a5fb52, 0x8664e6e5,
    0xbe2b5b58, 0xbaea46ef, 0xb7a96036, 0xb3687d81,
    0xad2f2d84, 0xa9ee3033, 0xa4ad16ea, 0xa06c0b5d,
    0xd4326d90, 0xd0f37027, 0xddb056fe, 0xd9714b49,
    0xc7361b4c, 0xc3f706fb, 0xceb42022, 0xca753d95,
    0xf23a8028, 0xf6fb9d9f, 0xfbb8bb46, 0xff79a6f1,
    0xe13ef6f4, 0xe5ffeb43, 0xe8bccd9a, 0xec7dd02d,
    0x34867077, 0x30476dc0, 0x3d044b19, 0x39c556ae,
    0x278206ab, 0x23431b1c, 0x2e003dc5, 0x2ac12072,
    0x128e9dcf, 0x164f8078, 0x1b0ca6a1, 0x1fcdbb16,
    0x018aeb13, 0x054bf6a4, 0x0808d07d, 0x0cc9cdca,
    0x7897ab07, 0x7c56b6b0, 0x71159069, 0x75d48dde,
    0x6b93dddb, 0x6f52c06c, 0x6211e6b5, 0x66d0fb02,
    0x5e9f46bf, 0x5a5e5b08, 0x571d7dd1, 0x53dc6066,
    0x4d9b3063, 0x495a2dd4, 0x44190b0d, 0x40d816ba,
    0xaca5c697, 0xa864db20, 0xa527fdf9, 0xa1e6e04e,
    0xbfa1b04b, 0xbb60adfc, 0xb6238b25, 0xb2e29692,
    0x8aad2b2f, 0x8e6c3698, 0x832f1041, 0x87ee0df6,
    0x99a95df3, 0x9d684044, 0x902b669d, 0x94ea7b2a,
    0xe0b41de7, 0xe4750050, 0xe9362689, 0xedf73b3e,
    0xf3b06b3b, 0xf771768c, 0xfa325055, 0xfef34de2,
    0xc6bcf05f, 0xc27dede8, 0xcf3ecb31, 0xcbffd686,
    0xd5b88683, 0xd1799b34, 0xdc3abded, 0xd8fba05a,
    0x690ce0ee, 0x6dcdfd59, 0x608edb80, 0x644fc637,
    0x7a089632, 0x7ec98b85, 0x738aad5c, 0x774bb0eb,
    0x4f040d56, 0x4bc510e1, 0x46863638, 0x42472b8f,
    0x5c007b8a, 0x58c1663d, 0x558240e4, 0x51435d53,
    0x251d3b9e, 0x21dc2629, 0x2c9f00f0, 0x285e1d47,
    0x36194d42, 0x32d850f5, 0x3f9b762c, 0x3b5a6b9b,
    0x0315d626, 0x07d4cb91, 0x0a97ed48, 0x0e56f0ff,
    0x1011a0fa, 0x14d0bd4d, 0x19939b94, 0x1d528623,
    0xf12f560e, 0xf5ee4bb9, 0xf8ad6d60, 0xfc6c70d7,
    0xe22b20d2, 0xe6ea3d65, 0xeba91bbc, 0xef68060b,
    0xd727bbb6, 0xd3e6a601, 0xdea580d8, 0xda649d6f,
    0xc423cd6a, 0xc0e2d0dd, 0xcda1f604, 0xc960ebb3,
    0xbd3e8d7e, 0xb9ff90c9, 0xb4bcb610, 0xb07daba7,
    0xae3afba2, 0xaafbe615, 0xa7b8c0cc, 0xa379dd7b,
    0x9b3660c6, 0x9ff77d71, 0x92b45ba8, 0x9675461f,
    0x8832161a, 0x8cf30bad, 0x81b02d74, 0x857130c3,
    0x5d8a9099, 0x594b8d2e, 0x5408abf7, 0x50c9b640,
    0x4e8ee645, 0x4a4ffbf2, 0x470cdd2b, 0x43cdc09c,
    0x7b827d21, 0x7f436096, 0x7200464f, 0x76c15bf8,
    0x68860bfd, 0x6c47164a, 0x61043093, 0x65c52d24,
    0x119b4be9, 0x155a565e, 0x18197087, 0x1cd86d30,
    0x029f3d35, 0x065e2082, 0x0b1d065b, 0x0fdc1bec,
    0x3793a651, 0x3352bbe6, 0x3e119d3f, 0x3ad08088,
    0x2497d08d, 0x2056cd3a, 0x2d15ebe3, 0x29d4f654,
    0xc5a92679, 0xc1683bce, 0xcc2b1d17, 0xc8ea00a0,
    0xd6ad50a5, 0xd26c4d12, 0xdf2f6bcb, 0xdbee767c,
    0xe3a1cbc1, 0xe760d676, 0xea23f0af, 0xeee2ed18,
    0xf0a5bd1d, 0xf464a0aa, 0xf9278673, 0xfde69bc4,
    0x89b8fd09, 0x8d79e0be, 0x803ac667, 0x84fbdbd0,
    0x9abc8bd5, 0x9e7d9662, 0x933eb0bb, 0x97ffad0c,
    0xafb010b1, 0xab710d06, 0xa6322bdf, 0xa2f33668,
    0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4,
];

pub fn compress_crc_table(i: u32) -> u32 {
    BZ2_CRC32TABLE[(i & 0xff) as usize]
}

/*---------------------------------------------------*/
pub fn bz_config_ok() -> i32 {
    1
}

/*---------------------------------------------------*/
pub fn make_estate(blockSize100k: i32) -> EState {
    let n = 100000 * blockSize100k;
    EState {
        mode: 0,
        state: 0,
        avail_in_expect: 0,
        arr1: vec![0u32; n as usize],
        arr2: vec![0u32; (n + BZ_N_OVERSHOOT) as usize],
        ftab: vec![0u32; 65537],
        origPtr: 0,
        ptr: Vec::new(),
        block: Vec::new(),
        mtfv: Vec::new(),
        zbits: Vec::new(),
        workFactor: 0,
        state_in_ch: 0,
        state_in_len: 0,
        rNToGo: 0,
        rTPos: 0,
        nblock: 0,
        nblockMAX: 0,
        numZ: 0,
        state_out_pos: 0,
        nInUse: 0,
        inUse: [0u8; 256],
        unseqToSeq: [0u8; 256],
        bsBuff: 0,
        bsLive: 0,
        blockCRC: 0,
        combinedCRC: 0,
        verbosity: 0,
        blockNo: 0,
        blockSize100k: 0,
        nMTF: 0,
        mtfFreq: [0i32; crate::bzlib_private::BZ_MAX_ALPHA_SIZE],
        selector: [0u8; crate::bzlib_private::BZ_MAX_SELECTORS],
        selectorMtf: [0u8; crate::bzlib_private::BZ_MAX_SELECTORS],
        len: [[0u8; crate::bzlib_private::BZ_MAX_ALPHA_SIZE]; crate::bzlib_private::BZ_N_GROUPS],
        code: [[0i32; crate::bzlib_private::BZ_MAX_ALPHA_SIZE]; crate::bzlib_private::BZ_N_GROUPS],
        rfreq: [[0i32; crate::bzlib_private::BZ_MAX_ALPHA_SIZE]; crate::bzlib_private::BZ_N_GROUPS],
        len_pack: [[0u32; 4]; crate::bzlib_private::BZ_MAX_ALPHA_SIZE],
    }
}

pub fn make_dstate() -> DState {
    DState {
        state: 0,
        state_out_ch: 0,
        state_out_len: 0,
        blockRandomised: 0,
        rNToGo: 0,
        rTPos: 0,
        bsBuff: 0,
        bsLive: 0,
        blockSize100k: 0,
        smallDecompress: 0,
        currBlockNo: 0,
        verbosity: 0,
        origPtr: 0,
        tPos: 0,
        k0: 0,
        unzftab: [0i32; 256],
        nblock_used: 0,
        cftab: [0i32; 257],
        cftabCopy: [0i32; 257],
        tt: Vec::new(),
        ll16: Vec::new(),
        ll4: Vec::new(),
        storedBlockCRC: 0,
        storedCombinedCRC: 0,
        calculatedBlockCRC: 0,
        calculatedCombinedCRC: 0,
        nInUse: 0,
        inUse: [0u8; 256],
        inUse16: [0u8; 16],
        seqToUnseq: [0u8; 256],
        mtfa: [0u8; crate::bzlib_private::MTFA_SIZE],
        mtfbase: [0i32; 256 / crate::bzlib_private::MTFL_SIZE],
        selector: [0u8; crate::bzlib_private::BZ_MAX_SELECTORS],
        selectorMtf: [0u8; crate::bzlib_private::BZ_MAX_SELECTORS],
        len: [[0u8; crate::bzlib_private::BZ_MAX_ALPHA_SIZE]; crate::bzlib_private::BZ_N_GROUPS],
        limit: [[0i32; crate::bzlib_private::BZ_MAX_ALPHA_SIZE]; crate::bzlib_private::BZ_N_GROUPS],
        base: [[0i32; crate::bzlib_private::BZ_MAX_ALPHA_SIZE]; crate::bzlib_private::BZ_N_GROUPS],
        perm: [[0i32; crate::bzlib_private::BZ_MAX_ALPHA_SIZE]; crate::bzlib_private::BZ_N_GROUPS],
        minLens: [0i32; crate::bzlib_private::BZ_N_GROUPS],
        save_i: 0,
        save_j: 0,
        save_t: 0,
        save_alphaSize: 0,
        save_nGroups: 0,
        save_nSelectors: 0,
        save_EOB: 0,
        save_groupNo: 0,
        save_groupPos: 0,
        save_nextSym: 0,
        save_nblockMAX: 0,
        save_nblock: 0,
        save_es: 0,
        save_N: 0,
        save_curr: 0,
        save_zt: 0,
        save_zn: 0,
        save_zvec: 0,
        save_zj: 0,
        save_gSel: 0,
        save_gMinlen: 0,
        save_gLimit: Vec::new(),
        save_gBase: Vec::new(),
        save_gPerm: Vec::new(),
    }
}

/*---------------------------------------------------*/
pub fn prepare_new_block(s: &mut EState) {
    s.nblock = 0;
    s.numZ = 0;
    s.state_out_pos = 0;
    s.blockCRC = 0xffffffff;
    for i in 0..256 {
        s.inUse[i] = False;
    }
    s.blockNo += 1;
}

/*---------------------------------------------------*/
pub fn init_RL(s: &mut EState) {
    s.state_in_ch = 256;
    s.state_in_len = 0;
}

pub fn isempty_RL(s: &EState) -> bool {
    if s.state_in_ch < 256 && s.state_in_len > 0 {
        false
    } else {
        true
    }
}

/*---------------------------------------------------*/
pub fn BZ2_bzCompressInit(
    strm: &mut bz_stream,
    blockSize100k: i32,
    verbosity: i32,
    workFactor: i32,
) -> i32 {
    if bz_config_ok() == 0 {
        return BZ_CONFIG_ERROR;
    }

    if blockSize100k < 1
        || blockSize100k > 9
        || workFactor < 0
        || workFactor > 250
    {
        return BZ_PARAM_ERROR;
    }

    let mut workFactor = workFactor;
    if workFactor == 0 {
        workFactor = 30;
    }

    let mut s = make_estate(blockSize100k);

    s.blockNo = 0;
    s.state = BZ_S_INPUT;
    s.mode = BZ_M_RUNNING;
    s.combinedCRC = 0;
    s.blockSize100k = blockSize100k;
    s.nblockMAX = 100000 * blockSize100k - 19;
    s.verbosity = verbosity;
    s.workFactor = workFactor;

    s.block = vec![0u8; s.arr2.len() * 4];
    s.mtfv = Vec::new();
    s.zbits = Vec::new();
    s.ptr = Vec::new();

    strm.total_in_lo32 = 0;
    strm.total_in_hi32 = 0;
    strm.total_out_lo32 = 0;
    strm.total_out_hi32 = 0;

    init_RL(&mut s);
    prepare_new_block(&mut s);

    strm.state = Some(Box::new(s));
    BZ_OK
}

/*---------------------------------------------------*/
pub fn add_pair_to_block(s: &mut EState) {
    let ch = (s.state_in_ch & 0xff) as u8;
    for _ in 0..s.state_in_len {
        let hi = (s.blockCRC >> 24) & 0xff;
        s.blockCRC = (s.blockCRC << 8) ^ compress_crc_table(hi ^ (ch as u32));
    }
    s.inUse[s.state_in_ch as usize] = True;
    match s.state_in_len {
        1 => {
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
        }
        2 => {
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
        }
        3 => {
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
        }
        _ => {
            s.inUse[(s.state_in_len - 4) as usize] = True;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = ch;
            s.nblock += 1;
            s.block[s.nblock as usize] = (s.state_in_len - 4) as u8;
            s.nblock += 1;
        }
    }
}

/*---------------------------------------------------*/
pub fn flush_RL(s: &mut EState) {
    if s.state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_RL(s);
}

/*---------------------------------------------------*/
pub fn add_char_to_block(s: &mut EState, zchh0: u32) {
    let zchh = zchh0;
    if zchh != s.state_in_ch && s.state_in_len == 1 {
        let ch = (s.state_in_ch & 0xff) as u8;
        let hi = (s.blockCRC >> 24) & 0xff;
        s.blockCRC = (s.blockCRC << 8) ^ compress_crc_table(hi ^ (ch as u32));
        s.inUse[s.state_in_ch as usize] = True;
        s.block[s.nblock as usize] = ch;
        s.nblock += 1;
        s.state_in_ch = zchh;
    } else if zchh != s.state_in_ch || s.state_in_len == 255 {
        if s.state_in_ch < 256 {
            add_pair_to_block(s);
        }
        s.state_in_ch = zchh;
        s.state_in_len = 1;
    } else {
        s.state_in_len += 1;
    }
}

/*---------------------------------------------------*/
pub fn copy_input_until_stop(s: &mut EState, strm: &mut bz_stream) -> bool {
    let mut progress_in = false;

    if s.mode == BZ_M_RUNNING {
        loop {
            if s.nblock >= s.nblockMAX {
                break;
            }
            if strm.avail_in == 0 {
                break;
            }
            progress_in = true;
            let idx = (strm.next_in.len() as u32 - strm.avail_in) as usize;
            let byte = strm.next_in[idx] as u32;
            add_char_to_block(s, byte);
            strm.avail_in -= 1;
            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
            }
        }
    } else {
        loop {
            if s.nblock >= s.nblockMAX {
                break;
            }
            if strm.avail_in == 0 {
                break;
            }
            if s.avail_in_expect == 0 {
                break;
            }
            progress_in = true;
            let idx = (strm.next_in.len() as u32 - strm.avail_in) as usize;
            let byte = strm.next_in[idx] as u32;
            add_char_to_block(s, byte);
            strm.avail_in -= 1;
            strm.total_in_lo32 = strm.total_in_lo32.wrapping_add(1);
            if strm.total_in_lo32 == 0 {
                strm.total_in_hi32 = strm.total_in_hi32.wrapping_add(1);
            }
            s.avail_in_expect -= 1;
        }
    }
    progress_in
}

/*---------------------------------------------------*/
pub fn copy_output_until_stop(s: &mut EState, strm: &mut bz_stream) -> bool {
    let mut progress_out = false;

    loop {
        if strm.avail_out == 0 {
            break;
        }
        if s.state_out_pos >= s.numZ {
            break;
        }
        progress_out = true;
        let out_idx = (strm.next_out.len() as u32 - strm.avail_out) as usize;
        strm.next_out[out_idx] = s.zbits[s.state_out_pos as usize];
        s.state_out_pos += 1;
        strm.avail_out -= 1;
        strm.total_out_lo32 = strm.total_out_lo32.wrapping_add(1);
        if strm.total_out_lo32 == 0 {
            strm.total_out_hi32 = strm.total_out_hi32.wrapping_add(1);
        }
    }
    progress_out
}

/*---------------------------------------------------*/
pub fn handle_compress(s: &mut EState, strm: &mut bz_stream) -> bool {
    let mut progress_in = false;
    let mut progress_out = false;

    loop {
        if s.state == BZ_S_OUTPUT {
            progress_out |= copy_output_until_stop(s, strm);
            if s.state_out_pos < s.numZ {
                break;
            }
            if s.mode == BZ_M_FINISHING && s.avail_in_expect == 0 && isempty_RL(s) {
                break;
            }
            prepare_new_block(s);
            s.state = BZ_S_INPUT;
            if s.mode == BZ_M_FLUSHING && s.avail_in_expect == 0 && isempty_RL(s) {
                break;
            }
        }

        if s.state == BZ_S_INPUT {
            progress_in |= copy_input_until_stop(s, strm);
            if s.mode != BZ_M_RUNNING && s.avail_in_expect == 0 {
                flush_RL(s);
                crate::compress::BZ2_compressBlock(s, (s.mode == BZ_M_FINISHING) as u8);
                s.state = BZ_S_OUTPUT;
            } else if s.nblock >= s.nblockMAX {
                crate::compress::BZ2_compressBlock(s, False);
                s.state = BZ_S_OUTPUT;
            } else if strm.avail_in == 0 {
                break;
            }
        }
    }

    progress_in || progress_out
}

/*---------------------------------------------------*/
pub fn BZ2_bzCompress(strm: &mut bz_stream, action: i32) -> i32 {
    let mut s = match strm.state.take() {
        None => return BZ_PARAM_ERROR,
        Some(b) => match b.downcast::<EState>() {
            Ok(es) => es,
            Err(b) => {
                strm.state = Some(b);
                return BZ_PARAM_ERROR;
            }
        },
    };

    let ret = bzCompress_inner(&mut s, strm, action);
    strm.state = Some(s);
    ret
}

pub fn bzCompress_inner(s: &mut EState, strm: &mut bz_stream, action: i32) -> i32 {
    let progress;
    loop {
        match s.mode {
            m if m == BZ_M_IDLE => {
                return BZ_SEQUENCE_ERROR;
            }
            m if m == BZ_M_RUNNING => {
                if action == BZ_RUN {
                    progress = handle_compress(s, strm);
                    return if progress { BZ_RUN_OK } else { BZ_PARAM_ERROR };
                } else if action == BZ_FLUSH {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = BZ_M_FLUSHING;
                    continue;
                } else if action == BZ_FINISH {
                    s.avail_in_expect = strm.avail_in;
                    s.mode = BZ_M_FINISHING;
                    continue;
                } else {
                    return BZ_PARAM_ERROR;
                }
            }
            m if m == BZ_M_FLUSHING => {
                if action != BZ_FLUSH {
                    return BZ_SEQUENCE_ERROR;
                }
                if s.avail_in_expect != strm.avail_in {
                    return BZ_SEQUENCE_ERROR;
                }
                progress = handle_compress(s, strm);
                let _ = progress;
                if s.avail_in_expect > 0 || !isempty_RL(s) || s.state_out_pos < s.numZ {
                    return BZ_FLUSH_OK;
                }
                s.mode = BZ_M_RUNNING;
                return BZ_RUN_OK;
            }
            m if m == BZ_M_FINISHING => {
                if action != BZ_FINISH {
                    return BZ_SEQUENCE_ERROR;
                }
                if s.avail_in_expect != strm.avail_in {
                    return BZ_SEQUENCE_ERROR;
                }
                progress = handle_compress(s, strm);
                if !progress {
                    return BZ_SEQUENCE_ERROR;
                }
                if s.avail_in_expect > 0 || !isempty_RL(s) || s.state_out_pos < s.numZ {
                    return BZ_FINISH_OK;
                }
                s.mode = BZ_M_IDLE;
                return BZ_STREAM_END;
            }
            _ => {
                return BZ_OK;
            }
        }
    }
}

/*---------------------------------------------------*/
pub fn BZ2_bzCompressEnd(strm: &mut bz_stream) -> i32 {
    match strm.state.take() {
        None => return BZ_PARAM_ERROR,
        Some(b) => {
            if b.downcast::<EState>().is_err() {
                return BZ_PARAM_ERROR;
            }
        }
    }
    strm.state = None;
    BZ_OK
}

/*---------------------------------------------------*/
pub fn BZ2_bzDecompressInit(strm: &mut bz_stream, verbosity: i32, small: i32) -> i32 {
    if bz_config_ok() == 0 {
        return BZ_CONFIG_ERROR;
    }
    if small != 0 && small != 1 {
        return BZ_PARAM_ERROR;
    }
    if verbosity < 0 || verbosity > 4 {
        return BZ_PARAM_ERROR;
    }

    let mut s = make_dstate();
    s.state = BZ_X_MAGIC_1;
    s.bsLive = 0;
    s.bsBuff = 0;
    s.calculatedCombinedCRC = 0;
    strm.total_in_lo32 = 0;
    strm.total_in_hi32 = 0;
    strm.total_out_lo32 = 0;
    strm.total_out_hi32 = 0;
    s.smallDecompress = small as u8;
    s.currBlockNo = 0;
    s.verbosity = verbosity;

    strm.state = Some(Box::new(s));
    BZ_OK
}

/*---------------------------------------------------*/
pub fn unRLE_obuf_to_output_FAST(s: &mut DState, strm: &mut bz_stream) -> bool {
    let mut k1: u8;

    if s.blockRandomised != 0 {
        loop {
            loop {
                if strm.avail_out == 0 {
                    return false;
                }
                if s.state_out_len == 0 {
                    break;
                }
                let out_idx = (strm.next_out.len() as u32 - strm.avail_out) as usize;
                strm.next_out[out_idx] = s.state_out_ch;
                let hi = (s.calculatedBlockCRC >> 24) & 0xff;
                s.calculatedBlockCRC = (s.calculatedBlockCRC << 8)
                    ^ compress_crc_table(hi ^ (s.state_out_ch as u32));
                s.state_out_len -= 1;
                strm.avail_out -= 1;
                strm.total_out_lo32 = strm.total_out_lo32.wrapping_add(1);
                if strm.total_out_lo32 == 0 {
                    strm.total_out_hi32 = strm.total_out_hi32.wrapping_add(1);
                }
            }

            if s.nblock_used == s.save_nblock + 1 {
                return false;
            }
            if s.nblock_used > s.save_nblock + 1 {
                return true;
            }

            s.state_out_len = 1;
            s.state_out_ch = s.k0 as u8;
            k1 = crate::decompress::bz_get_fast(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 2;
            k1 = crate::decompress::bz_get_fast(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 3;
            k1 = crate::decompress::bz_get_fast(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            k1 = crate::decompress::bz_get_fast(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            s.state_out_len = (k1 as i32) + 4;
            let k0v = crate::decompress::bz_get_fast(s);
            s.k0 = k0v as i32;
            crate::decompress::bz_rand_upd_mask(s);
            s.k0 ^= crate::decompress::bz_rand_mask(s);
            s.nblock_used += 1;
        }
    } else {
        let avail_out_init = strm.avail_out;
        let s_save_nblockpp = s.save_nblock + 1;

        #[derive(PartialEq)]
        enum St {
            Loop,
            EqOne,
            ReturnNotr,
        }
        let mut st = St::Loop;
        let mut c_state_out_len = s.state_out_len;
        let mut c_state_out_ch = s.state_out_ch;
        let mut c_nblock_used = s.nblock_used;
        let mut c_k0 = s.k0;
        let mut c_calc_crc = s.calculatedBlockCRC;
        let mut cs_avail_out = strm.avail_out;
        let mut cs_next_pos = (strm.next_out.len() as u32 - strm.avail_out) as usize;

        'outer: loop {
            match st {
                St::Loop => {
                    if c_state_out_len > 0 {
                        loop {
                            if cs_avail_out == 0 {
                                st = St::ReturnNotr;
                                continue 'outer;
                            }
                            if c_state_out_len == 1 {
                                break;
                            }
                            strm.next_out[cs_next_pos] = c_state_out_ch;
                            let hi = (c_calc_crc >> 24) & 0xff;
                            c_calc_crc = (c_calc_crc << 8)
                                ^ compress_crc_table(hi ^ (c_state_out_ch as u32));
                            c_state_out_len -= 1;
                            cs_next_pos += 1;
                            cs_avail_out -= 1;
                        }
                        st = St::EqOne;
                        continue 'outer;
                    }

                    if c_nblock_used > s_save_nblockpp {
                        return true;
                    }
                    if c_nblock_used == s_save_nblockpp {
                        c_state_out_len = 0;
                        st = St::ReturnNotr;
                        continue 'outer;
                    }
                    c_state_out_ch = c_k0 as u8;
                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    if (k1v as i32) != c_k0 {
                        c_k0 = k1v as i32;
                        st = St::EqOne;
                        continue 'outer;
                    }
                    if c_nblock_used == s_save_nblockpp {
                        st = St::EqOne;
                        continue 'outer;
                    }

                    c_state_out_len = 2;
                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    if c_nblock_used == s_save_nblockpp {
                        st = St::Loop;
                        continue 'outer;
                    }
                    if (k1v as i32) != c_k0 {
                        c_k0 = k1v as i32;
                        st = St::Loop;
                        continue 'outer;
                    }

                    c_state_out_len = 3;
                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    if c_nblock_used == s_save_nblockpp {
                        st = St::Loop;
                        continue 'outer;
                    }
                    if (k1v as i32) != c_k0 {
                        c_k0 = k1v as i32;
                        st = St::Loop;
                        continue 'outer;
                    }

                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    c_state_out_len = (k1v as i32) + 4;
                    let k0v = crate::decompress::bz_get_fast(s);
                    c_k0 = k0v as i32;
                    c_nblock_used += 1;
                    st = St::Loop;
                    continue 'outer;
                }
                St::EqOne => {
                    if cs_avail_out == 0 {
                        c_state_out_len = 1;
                        st = St::ReturnNotr;
                        continue 'outer;
                    }
                    strm.next_out[cs_next_pos] = c_state_out_ch;
                    let hi = (c_calc_crc >> 24) & 0xff;
                    c_calc_crc = (c_calc_crc << 8)
                        ^ compress_crc_table(hi ^ (c_state_out_ch as u32));
                    cs_next_pos += 1;
                    cs_avail_out -= 1;
                    if c_nblock_used > s_save_nblockpp {
                        return true;
                    }
                    if c_nblock_used == s_save_nblockpp {
                        c_state_out_len = 0;
                        st = St::ReturnNotr;
                        continue 'outer;
                    }
                    c_state_out_ch = c_k0 as u8;
                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    if (k1v as i32) != c_k0 {
                        c_k0 = k1v as i32;
                        st = St::EqOne;
                        continue 'outer;
                    }
                    if c_nblock_used == s_save_nblockpp {
                        st = St::EqOne;
                        continue 'outer;
                    }

                    c_state_out_len = 2;
                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    if c_nblock_used == s_save_nblockpp {
                        st = St::Loop;
                        continue 'outer;
                    }
                    if (k1v as i32) != c_k0 {
                        c_k0 = k1v as i32;
                        st = St::Loop;
                        continue 'outer;
                    }

                    c_state_out_len = 3;
                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    if c_nblock_used == s_save_nblockpp {
                        st = St::Loop;
                        continue 'outer;
                    }
                    if (k1v as i32) != c_k0 {
                        c_k0 = k1v as i32;
                        st = St::Loop;
                        continue 'outer;
                    }

                    let k1v = crate::decompress::bz_get_fast(s);
                    c_nblock_used += 1;
                    c_state_out_len = (k1v as i32) + 4;
                    let k0v = crate::decompress::bz_get_fast(s);
                    c_k0 = k0v as i32;
                    c_nblock_used += 1;
                    st = St::Loop;
                    continue 'outer;
                }
                St::ReturnNotr => {
                    break 'outer;
                }
            }
        }

        let total_out_lo32_old = strm.total_out_lo32;
        strm.total_out_lo32 = strm
            .total_out_lo32
            .wrapping_add(avail_out_init - cs_avail_out);
        if strm.total_out_lo32 < total_out_lo32_old {
            strm.total_out_hi32 = strm.total_out_hi32.wrapping_add(1);
        }

        s.calculatedBlockCRC = c_calc_crc;
        s.state_out_ch = c_state_out_ch;
        s.state_out_len = c_state_out_len;
        s.nblock_used = c_nblock_used;
        s.k0 = c_k0;
        strm.avail_out = cs_avail_out;
    }
    false
}

/*---------------------------------------------------*/
pub fn BZ2_indexIntoF(indx: i32, cftab: &[i32]) -> i32 {
    let mut nb: i32 = 0;
    let mut na: i32 = 256;
    loop {
        let mid = (nb + na) >> 1;
        if indx >= cftab[mid as usize] {
            nb = mid;
        } else {
            na = mid;
        }
        if na - nb == 1 {
            break;
        }
    }
    nb
}

/*---------------------------------------------------*/
pub fn unRLE_obuf_to_output_SMALL(s: &mut DState, strm: &mut bz_stream) -> bool {
    let mut k1: u8;

    if s.blockRandomised != 0 {
        loop {
            loop {
                if strm.avail_out == 0 {
                    return false;
                }
                if s.state_out_len == 0 {
                    break;
                }
                let out_idx = (strm.next_out.len() as u32 - strm.avail_out) as usize;
                strm.next_out[out_idx] = s.state_out_ch;
                let hi = (s.calculatedBlockCRC >> 24) & 0xff;
                s.calculatedBlockCRC = (s.calculatedBlockCRC << 8)
                    ^ compress_crc_table(hi ^ (s.state_out_ch as u32));
                s.state_out_len -= 1;
                strm.avail_out -= 1;
                strm.total_out_lo32 = strm.total_out_lo32.wrapping_add(1);
                if strm.total_out_lo32 == 0 {
                    strm.total_out_hi32 = strm.total_out_hi32.wrapping_add(1);
                }
            }
            if s.nblock_used == s.save_nblock + 1 {
                return false;
            }
            if s.nblock_used > s.save_nblock + 1 {
                return true;
            }

            s.state_out_len = 1;
            s.state_out_ch = s.k0 as u8;
            k1 = crate::decompress::bz_get_small(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 2;
            k1 = crate::decompress::bz_get_small(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 3;
            k1 = crate::decompress::bz_get_small(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            k1 = crate::decompress::bz_get_small(s);
            crate::decompress::bz_rand_upd_mask(s);
            k1 ^= crate::decompress::bz_rand_mask(s) as u8;
            s.nblock_used += 1;
            s.state_out_len = (k1 as i32) + 4;
            let k0v = crate::decompress::bz_get_small(s);
            s.k0 = k0v as i32;
            crate::decompress::bz_rand_upd_mask(s);
            s.k0 ^= crate::decompress::bz_rand_mask(s);
            s.nblock_used += 1;
        }
    } else {
        loop {
            loop {
                if strm.avail_out == 0 {
                    return false;
                }
                if s.state_out_len == 0 {
                    break;
                }
                let out_idx = (strm.next_out.len() as u32 - strm.avail_out) as usize;
                strm.next_out[out_idx] = s.state_out_ch;
                let hi = (s.calculatedBlockCRC >> 24) & 0xff;
                s.calculatedBlockCRC = (s.calculatedBlockCRC << 8)
                    ^ compress_crc_table(hi ^ (s.state_out_ch as u32));
                s.state_out_len -= 1;
                strm.avail_out -= 1;
                strm.total_out_lo32 = strm.total_out_lo32.wrapping_add(1);
                if strm.total_out_lo32 == 0 {
                    strm.total_out_hi32 = strm.total_out_hi32.wrapping_add(1);
                }
            }
            if s.nblock_used == s.save_nblock + 1 {
                return false;
            }
            if s.nblock_used > s.save_nblock + 1 {
                return true;
            }

            s.state_out_len = 1;
            s.state_out_ch = s.k0 as u8;
            k1 = crate::decompress::bz_get_small(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 2;
            k1 = crate::decompress::bz_get_small(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            s.state_out_len = 3;
            k1 = crate::decompress::bz_get_small(s);
            s.nblock_used += 1;
            if s.nblock_used == s.save_nblock + 1 {
                continue;
            }
            if (k1 as i32) != s.k0 {
                s.k0 = k1 as i32;
                continue;
            }

            k1 = crate::decompress::bz_get_small(s);
            s.nblock_used += 1;
            s.state_out_len = (k1 as i32) + 4;
            let k0v = crate::decompress::bz_get_small(s);
            s.k0 = k0v as i32;
            s.nblock_used += 1;
        }
    }
}

/*---------------------------------------------------*/
pub fn BZ2_bzDecompress(strm: &mut bz_stream) -> i32 {
    let mut s = match strm.state.take() {
        None => return BZ_PARAM_ERROR,
        Some(b) => match b.downcast::<DState>() {
            Ok(ds) => ds,
            Err(b) => {
                strm.state = Some(b);
                return BZ_PARAM_ERROR;
            }
        },
    };

    let ret = bzDecompress_inner(&mut s, strm);
    strm.state = Some(s);
    ret
}

pub fn bzDecompress_inner(s: &mut DState, strm: &mut bz_stream) -> i32 {
    loop {
        if s.state == BZ_X_IDLE {
            return BZ_SEQUENCE_ERROR;
        }
        if s.state == BZ_X_OUTPUT {
            let corrupt = if s.smallDecompress != 0 {
                unRLE_obuf_to_output_SMALL(s, strm)
            } else {
                unRLE_obuf_to_output_FAST(s, strm)
            };
            if corrupt {
                return BZ_DATA_ERROR;
            }
            if s.nblock_used == s.save_nblock + 1 && s.state_out_len == 0 {
                s.calculatedBlockCRC = !s.calculatedBlockCRC;
                if s.calculatedBlockCRC != s.storedBlockCRC {
                    return BZ_DATA_ERROR;
                }
                s.calculatedCombinedCRC =
                    (s.calculatedCombinedCRC << 1) | (s.calculatedCombinedCRC >> 31);
                s.calculatedCombinedCRC ^= s.calculatedBlockCRC;
                s.state = BZ_X_BLKHDR_1;
            } else {
                return BZ_OK;
            }
        }
        if s.state >= BZ_X_MAGIC_1 {
            let r = crate::decompress::BZ2_decompress(s, strm);
            if r == BZ_STREAM_END {
                if s.calculatedCombinedCRC != s.storedCombinedCRC {
                    return BZ_DATA_ERROR;
                }
                return r;
            }
            if s.state != BZ_X_OUTPUT {
                return r;
            }
        }
    }
}

/*---------------------------------------------------*/
pub fn BZ2_bzDecompressEnd(strm: &mut bz_stream) -> i32 {
    match strm.state.take() {
        None => return BZ_PARAM_ERROR,
        Some(b) => {
            if b.downcast::<DState>().is_err() {
                return BZ_PARAM_ERROR;
            }
        }
    }
    strm.state = None;
    BZ_OK
}

/*---------------------------------------------------*/
pub fn BZ2_bzBuffToBuffCompress(
    dest: &mut [u8],
    destLen: &mut u32,
    source: &[u8],
    sourceLen: u32,
    blockSize100k: i32,
    verbosity: i32,
    workFactor: i32,
) -> i32 {
    if blockSize100k < 1
        || blockSize100k > 9
        || verbosity < 0
        || verbosity > 4
        || workFactor < 0
        || workFactor > 250
    {
        return BZ_PARAM_ERROR;
    }

    let mut workFactor = workFactor;
    if workFactor == 0 {
        workFactor = 30;
    }

    let mut strm = new_stream();
    let ret = BZ2_bzCompressInit(&mut strm, blockSize100k, verbosity, workFactor);
    if ret != BZ_OK {
        return ret;
    }

    strm.next_in = source.to_vec();
    strm.next_out = dest.to_vec();
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;

    let ret = BZ2_bzCompress(&mut strm, BZ_FINISH);
    if ret == BZ_FINISH_OK {
        BZ2_bzCompressEnd(&mut strm);
        return BZ_OUTBUFF_FULL;
    }
    if ret != BZ_STREAM_END {
        BZ2_bzCompressEnd(&mut strm);
        return ret;
    }

    let produced = (*destLen - strm.avail_out) as usize;
    for i in 0..produced {
        dest[i] = strm.next_out[i];
    }
    *destLen -= strm.avail_out;
    BZ2_bzCompressEnd(&mut strm);
    BZ_OK
}

/*---------------------------------------------------*/
pub fn BZ2_bzBuffToBuffDecompress(
    dest: &mut [u8],
    destLen: &mut u32,
    source: &[u8],
    sourceLen: u32,
    small: i32,
    verbosity: i32,
) -> i32 {
    if (small != 0 && small != 1) || verbosity < 0 || verbosity > 4 {
        return BZ_PARAM_ERROR;
    }

    let mut strm = new_stream();
    let ret = BZ2_bzDecompressInit(&mut strm, verbosity, small);
    if ret != BZ_OK {
        return ret;
    }

    strm.next_in = source.to_vec();
    strm.next_out = dest.to_vec();
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;

    let ret = BZ2_bzDecompress(&mut strm);
    if ret == BZ_OK {
        if strm.avail_out > 0 {
            BZ2_bzDecompressEnd(&mut strm);
            return BZ_UNEXPECTED_EOF;
        } else {
            BZ2_bzDecompressEnd(&mut strm);
            return BZ_OUTBUFF_FULL;
        }
    }
    if ret != BZ_STREAM_END {
        BZ2_bzDecompressEnd(&mut strm);
        return ret;
    }

    let produced = (*destLen - strm.avail_out) as usize;
    for i in 0..produced {
        dest[i] = strm.next_out[i];
    }
    *destLen -= strm.avail_out;
    BZ2_bzDecompressEnd(&mut strm);
    BZ_OK
}

/*---------------------------------------------------*/
pub fn BZ2_bzlibVersion() -> &'static str {
    BZ_VERSION
}

/*---------------------------------------------------*/
pub fn new_stream() -> bz_stream {
    bz_stream {
        next_in: Vec::new(),
        avail_in: 0,
        total_in_lo32: 0,
        total_in_hi32: 0,
        next_out: Vec::new(),
        avail_out: 0,
        total_out_lo32: 0,
        total_out_hi32: 0,
        state: None,
        bzalloc: None,
        bzfree: None,
        opaque: None,
    }
}
