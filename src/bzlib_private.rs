#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
/*-- General stuff. --*/

pub const BZ_VERSION: &str = "1.0.8, 13-Jul-2019";

pub type Char = i8;
pub type Bool = u8;
pub type UChar = u8;
pub type Int32 = i32;
pub type UInt32 = u32;
pub type Int16 = i16;
pub type UInt16 = u16;

pub const True: Bool = 1;
pub const False: Bool = 0;

/*-- Header bytes. --*/

pub const BZ_HDR_B: u8 = 0x42;
pub const BZ_HDR_Z: u8 = 0x5a;
pub const BZ_HDR_h: u8 = 0x68;
pub const BZ_HDR_0: u8 = 0x30;

/*-- Constants for the back end. --*/

pub const BZ_MAX_ALPHA_SIZE: usize = 258;
pub const BZ_MAX_CODE_LEN: i32 = 23;

pub const BZ_RUNA: i32 = 0;
pub const BZ_RUNB: i32 = 1;

pub const BZ_N_GROUPS: usize = 6;
pub const BZ_G_SIZE: i32 = 50;
pub const BZ_N_ITERS: i32 = 4;

pub const BZ_MAX_SELECTORS: usize = 2 + (900000 / 50);

/*-- States and modes for compression. --*/

pub const BZ_M_IDLE: i32 = 1;
pub const BZ_M_RUNNING: i32 = 2;
pub const BZ_M_FLUSHING: i32 = 3;
pub const BZ_M_FINISHING: i32 = 4;

pub const BZ_S_OUTPUT: i32 = 1;
pub const BZ_S_INPUT: i32 = 2;

pub const BZ_N_RADIX: i32 = 2;
pub const BZ_N_QSORT: i32 = 12;
pub const BZ_N_SHELL: i32 = 18;
pub const BZ_N_OVERSHOOT: i32 = BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2;

/*-- bz_stream (from bzlib.h, inlined here since no bzlib module is available) --*/

pub struct bz_stream {
    pub next_in: Vec<u8>,
    pub avail_in: u32,
    pub total_in_lo32: u32,
    pub total_in_hi32: u32,

    pub next_out: Vec<u8>,
    pub avail_out: u32,
    pub total_out_lo32: u32,
    pub total_out_hi32: u32,

    pub state: Option<Box<dyn core::any::Any>>,

    pub bzalloc: Option<fn(u32) -> Vec<u8>>,
    pub bzfree: Option<fn(Vec<u8>)>,
    pub opaque: Option<Box<dyn core::any::Any>>,
}

/*-- Structure holding all the compression-side stuff. --*/

pub struct EState {
    pub mode: i32,
    pub state: i32,

    pub avail_in_expect: u32,

    pub arr1: Vec<u32>,
    pub arr2: Vec<u32>,
    pub ftab: Vec<u32>,
    pub origPtr: i32,

    pub ptr: Vec<u32>,
    pub block: Vec<u8>,
    pub mtfv: Vec<u16>,
    pub zbits: Vec<u8>,

    pub workFactor: i32,

    pub state_in_ch: u32,
    pub state_in_len: i32,
    pub rNToGo: i32,
    pub rTPos: i32,

    pub nblock: i32,
    pub nblockMAX: i32,
    pub numZ: i32,
    pub state_out_pos: i32,

    pub nInUse: i32,
    pub inUse: [u8; 256],
    pub unseqToSeq: [u8; 256],

    pub bsBuff: u32,
    pub bsLive: i32,

    pub blockCRC: u32,
    pub combinedCRC: u32,

    pub verbosity: i32,
    pub blockNo: i32,
    pub blockSize100k: i32,

    pub nMTF: i32,
    pub mtfFreq: [i32; BZ_MAX_ALPHA_SIZE],
    pub selector: [u8; BZ_MAX_SELECTORS],
    pub selectorMtf: [u8; BZ_MAX_SELECTORS],

    pub len: [[u8; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    pub code: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    pub rfreq: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    pub len_pack: [[u32; 4]; BZ_MAX_ALPHA_SIZE],
}

/*-- states for decompression. --*/

pub const BZ_X_IDLE: i32 = 1;
pub const BZ_X_OUTPUT: i32 = 2;

pub const BZ_X_MAGIC_1: i32 = 10;
pub const BZ_X_MAGIC_2: i32 = 11;
pub const BZ_X_MAGIC_3: i32 = 12;
pub const BZ_X_MAGIC_4: i32 = 13;
pub const BZ_X_BLKHDR_1: i32 = 14;
pub const BZ_X_BLKHDR_2: i32 = 15;
pub const BZ_X_BLKHDR_3: i32 = 16;
pub const BZ_X_BLKHDR_4: i32 = 17;
pub const BZ_X_BLKHDR_5: i32 = 18;
pub const BZ_X_BLKHDR_6: i32 = 19;
pub const BZ_X_BCRC_1: i32 = 20;
pub const BZ_X_BCRC_2: i32 = 21;
pub const BZ_X_BCRC_3: i32 = 22;
pub const BZ_X_BCRC_4: i32 = 23;
pub const BZ_X_RANDBIT: i32 = 24;
pub const BZ_X_ORIGPTR_1: i32 = 25;
pub const BZ_X_ORIGPTR_2: i32 = 26;
pub const BZ_X_ORIGPTR_3: i32 = 27;
pub const BZ_X_MAPPING_1: i32 = 28;
pub const BZ_X_MAPPING_2: i32 = 29;
pub const BZ_X_SELECTOR_1: i32 = 30;
pub const BZ_X_SELECTOR_2: i32 = 31;
pub const BZ_X_SELECTOR_3: i32 = 32;
pub const BZ_X_CODING_1: i32 = 33;
pub const BZ_X_CODING_2: i32 = 34;
pub const BZ_X_CODING_3: i32 = 35;
pub const BZ_X_MTF_1: i32 = 36;
pub const BZ_X_MTF_2: i32 = 37;
pub const BZ_X_MTF_3: i32 = 38;
pub const BZ_X_MTF_4: i32 = 39;
pub const BZ_X_MTF_5: i32 = 40;
pub const BZ_X_MTF_6: i32 = 41;
pub const BZ_X_ENDHDR_2: i32 = 42;
pub const BZ_X_ENDHDR_3: i32 = 43;
pub const BZ_X_ENDHDR_4: i32 = 44;
pub const BZ_X_ENDHDR_5: i32 = 45;
pub const BZ_X_ENDHDR_6: i32 = 46;
pub const BZ_X_CCRC_1: i32 = 47;
pub const BZ_X_CCRC_2: i32 = 48;
pub const BZ_X_CCRC_3: i32 = 49;
pub const BZ_X_CCRC_4: i32 = 50;

/*-- Constants for the fast MTF decoder. --*/

pub const MTFA_SIZE: usize = 4096;
pub const MTFL_SIZE: usize = 16;

/*-- Structure holding all the decompression-side stuff. --*/

pub struct DState {
    pub state: i32,

    pub state_out_ch: u8,
    pub state_out_len: i32,
    pub blockRandomised: u8,
    pub rNToGo: i32,
    pub rTPos: i32,

    pub bsBuff: u32,
    pub bsLive: i32,

    pub blockSize100k: i32,
    pub smallDecompress: u8,
    pub currBlockNo: i32,
    pub verbosity: i32,

    pub origPtr: i32,
    pub tPos: u32,
    pub k0: i32,
    pub unzftab: [i32; 256],
    pub nblock_used: i32,
    pub cftab: [i32; 257],
    pub cftabCopy: [i32; 257],

    pub tt: Vec<u32>,

    pub ll16: Vec<u16>,
    pub ll4: Vec<u8>,

    pub storedBlockCRC: u32,
    pub storedCombinedCRC: u32,
    pub calculatedBlockCRC: u32,
    pub calculatedCombinedCRC: u32,

    pub nInUse: i32,
    pub inUse: [u8; 256],
    pub inUse16: [u8; 16],
    pub seqToUnseq: [u8; 256],

    pub mtfa: [u8; MTFA_SIZE],
    pub mtfbase: [i32; 256 / MTFL_SIZE],
    pub selector: [u8; BZ_MAX_SELECTORS],
    pub selectorMtf: [u8; BZ_MAX_SELECTORS],
    pub len: [[u8; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],

    pub limit: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    pub base: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    pub perm: [[i32; BZ_MAX_ALPHA_SIZE]; BZ_N_GROUPS],
    pub minLens: [i32; BZ_N_GROUPS],

    pub save_i: i32,
    pub save_j: i32,
    pub save_t: i32,
    pub save_alphaSize: i32,
    pub save_nGroups: i32,
    pub save_nSelectors: i32,
    pub save_EOB: i32,
    pub save_groupNo: i32,
    pub save_groupPos: i32,
    pub save_nextSym: i32,
    pub save_nblockMAX: i32,
    pub save_nblock: i32,
    pub save_es: i32,
    pub save_N: i32,
    pub save_curr: i32,
    pub save_zt: i32,
    pub save_zn: i32,
    pub save_zvec: i32,
    pub save_zj: i32,
    pub save_gSel: i32,
    pub save_gMinlen: i32,
    pub save_gLimit: Vec<i32>,
    pub save_gBase: Vec<i32>,
    pub save_gPerm: Vec<i32>,
}
