#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
use crate::bzlib_private::EState;

pub fn fallbackSimpleSort(fmap: &mut [u32], eclass: &[u32], lo: i32, hi: i32) {
    let mut i: i32;
    let mut j: i32;
    let mut tmp: i32;
    let mut ec_tmp: u32;

    if lo == hi {
        return;
    }

    if hi - lo > 3 {
        i = hi - 4;
        while i >= lo {
            tmp = fmap[i as usize] as i32;
            ec_tmp = eclass[tmp as usize];
            j = i + 4;
            while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
                fmap[(j - 4) as usize] = fmap[j as usize];
                j += 4;
            }
            fmap[(j - 4) as usize] = tmp as u32;
            i -= 1;
        }
    }

    i = hi - 1;
    while i >= lo {
        tmp = fmap[i as usize] as i32;
        ec_tmp = eclass[tmp as usize];
        j = i + 1;
        while j <= hi && ec_tmp > eclass[fmap[j as usize] as usize] {
            fmap[(j - 1) as usize] = fmap[j as usize];
            j += 1;
        }
        fmap[(j - 1) as usize] = tmp as u32;
        i -= 1;
    }
}

pub const FALLBACK_QSORT_SMALL_THRESH: i32 = 10;
pub const FALLBACK_QSORT_STACK_SIZE: usize = 100;

pub fn fallbackQSort3(fmap: &mut [u32], eclass: &[u32], loSt: i32, hiSt: i32) {
    let mut unLo: i32;
    let mut unHi: i32;
    let mut ltLo: i32;
    let mut gtHi: i32;
    let mut n: i32;
    let mut m: i32;
    let mut sp: usize;
    let mut lo: i32;
    let mut hi: i32;
    let mut med: u32;
    let mut r: u32;
    let mut r3: u32;

    let mut stackLo = [0i32; FALLBACK_QSORT_STACK_SIZE];
    let mut stackHi = [0i32; FALLBACK_QSORT_STACK_SIZE];

    r = 0;

    sp = 0;
    stackLo[sp] = loSt;
    stackHi[sp] = hiSt;
    sp += 1;

    while sp > 0 {
        sp -= 1;
        lo = stackLo[sp];
        hi = stackHi[sp];

        if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
            fallbackSimpleSort(fmap, eclass, lo, hi);
            continue;
        }

        r = ((r.wrapping_mul(7621)).wrapping_add(1)) % 32768;
        r3 = r % 3;
        if r3 == 0 {
            med = eclass[fmap[lo as usize] as usize];
        } else if r3 == 1 {
            med = eclass[fmap[((lo + hi) >> 1) as usize] as usize];
        } else {
            med = eclass[fmap[hi as usize] as usize];
        }

        unLo = lo;
        ltLo = lo;
        unHi = hi;
        gtHi = hi;

        loop {
            loop {
                if unLo > unHi {
                    break;
                }
                n = (eclass[fmap[unLo as usize] as usize] as i32) - (med as i32);
                if n == 0 {
                    fmap.swap(unLo as usize, ltLo as usize);
                    ltLo += 1;
                    unLo += 1;
                    continue;
                }
                if n > 0 {
                    break;
                }
                unLo += 1;
            }
            loop {
                if unLo > unHi {
                    break;
                }
                n = (eclass[fmap[unHi as usize] as usize] as i32) - (med as i32);
                if n == 0 {
                    fmap.swap(unHi as usize, gtHi as usize);
                    gtHi -= 1;
                    unHi -= 1;
                    continue;
                }
                if n < 0 {
                    break;
                }
                unHi -= 1;
            }
            if unLo > unHi {
                break;
            }
            fmap.swap(unLo as usize, unHi as usize);
            unLo += 1;
            unHi -= 1;
        }

        if gtHi < ltLo {
            continue;
        }

        n = if (ltLo - lo) < (unLo - ltLo) {
            ltLo - lo
        } else {
            unLo - ltLo
        };
        fvswap(fmap, lo, unLo - n, n);
        m = if (hi - gtHi) < (gtHi - unHi) {
            hi - gtHi
        } else {
            gtHi - unHi
        };
        fvswap(fmap, unLo, hi - m + 1, m);

        n = lo + unLo - ltLo - 1;
        m = hi - (gtHi - unHi) + 1;

        if n - lo > hi - m {
            stackLo[sp] = lo;
            stackHi[sp] = n;
            sp += 1;
            stackLo[sp] = m;
            stackHi[sp] = hi;
            sp += 1;
        } else {
            stackLo[sp] = m;
            stackHi[sp] = hi;
            sp += 1;
            stackLo[sp] = lo;
            stackHi[sp] = n;
            sp += 1;
        }
    }
}

pub fn fvswap(fmap: &mut [u32], zzp1: i32, zzp2: i32, zzn: i32) {
    let mut yyp1 = zzp1;
    let mut yyp2 = zzp2;
    let mut yyn = zzn;
    while yyn > 0 {
        fmap.swap(yyp1 as usize, yyp2 as usize);
        yyp1 += 1;
        yyp2 += 1;
        yyn -= 1;
    }
}

#[inline]
pub fn set_bh(bhtab: &mut [u32], zz: i32) {
    bhtab[(zz >> 5) as usize] |= (1u32) << (zz & 31);
}
#[inline]
pub fn clear_bh(bhtab: &mut [u32], zz: i32) {
    bhtab[(zz >> 5) as usize] &= !((1u32) << (zz & 31));
}
#[inline]
pub fn isset_bh(bhtab: &[u32], zz: i32) -> bool {
    (bhtab[(zz >> 5) as usize] & ((1u32) << (zz & 31))) != 0
}
#[inline]
pub fn word_bh(bhtab: &[u32], zz: i32) -> u32 {
    bhtab[(zz >> 5) as usize]
}
#[inline]
pub fn unaligned_bh(zz: i32) -> i32 {
    zz & 0x01f
}

pub fn fallbackSort(
    fmap: &mut [u32],
    eclass: &mut [u32],
    bhtab: &mut [u32],
    nblock: i32,
    verb: i32,
) {
    let mut ftab = [0i32; 257];
    let mut ftabCopy = [0i32; 256];
    let mut H: i32;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut l: i32;
    let mut r: i32;
    let mut cc: i32;
    let mut cc1: i32;
    let mut nNotDone: i32;
    let nBhtab: i32;

    for i in 0..257 {
        ftab[i] = 0;
    }
    for i in 0..(nblock as usize) {
        let b = eclass8_get(eclass, i) as usize;
        ftab[b] += 1;
    }
    for i in 0..256 {
        ftabCopy[i] = ftab[i];
    }
    for i in 1..257 {
        ftab[i] += ftab[i - 1];
    }

    for ii in 0..(nblock as usize) {
        let jj = eclass8_get(eclass, ii) as usize;
        let kk = ftab[jj] - 1;
        ftab[jj] = kk;
        fmap[kk as usize] = ii as u32;
    }

    nBhtab = 2 + (nblock / 32);
    for i in 0..(nBhtab as usize) {
        bhtab[i] = 0;
    }
    for i in 0..256 {
        set_bh(bhtab, ftab[i]);
    }

    for ii in 0..32i32 {
        set_bh(bhtab, nblock + 2 * ii);
        clear_bh(bhtab, nblock + 2 * ii + 1);
    }

    H = 1;
    loop {
        j = 0;
        i = 0;
        while i < nblock {
            if isset_bh(bhtab, i) {
                j = i;
            }
            k = fmap[i as usize] as i32 - H;
            if k < 0 {
                k += nblock;
            }
            eclass[k as usize] = j as u32;
            i += 1;
        }

        nNotDone = 0;
        r = -1;
        loop {
            k = r + 1;
            while isset_bh(bhtab, k) && unaligned_bh(k) != 0 {
                k += 1;
            }
            if isset_bh(bhtab, k) {
                while word_bh(bhtab, k) == 0xffffffff {
                    k += 32;
                }
                while isset_bh(bhtab, k) {
                    k += 1;
                }
            }
            l = k - 1;
            if l >= nblock {
                break;
            }
            while !isset_bh(bhtab, k) && unaligned_bh(k) != 0 {
                k += 1;
            }
            if !isset_bh(bhtab, k) {
                while word_bh(bhtab, k) == 0x00000000 {
                    k += 32;
                }
                while !isset_bh(bhtab, k) {
                    k += 1;
                }
            }
            r = k - 1;
            if r >= nblock {
                break;
            }

            if r > l {
                nNotDone += r - l + 1;
                fallbackQSort3(fmap, eclass, l, r);

                cc = -1;
                i = l;
                while i <= r {
                    cc1 = eclass[fmap[i as usize] as usize] as i32;
                    if cc != cc1 {
                        set_bh(bhtab, i);
                        cc = cc1;
                    }
                    i += 1;
                }
            }
        }

        H *= 2;
        if H > nblock || nNotDone == 0 {
            break;
        }
    }

    j = 0;
    for i in 0..(nblock as usize) {
        while ftabCopy[j as usize] == 0 {
            j += 1;
        }
        ftabCopy[j as usize] -= 1;
        eclass8_set(eclass, fmap[i] as usize, j as u8);
    }
}

#[inline]
pub fn eclass8_get(eclass: &[u32], idx: usize) -> u8 {
    let word = eclass[idx >> 2];
    let byte = (idx & 3) * 8;
    ((word >> byte) & 0xff) as u8
}

#[inline]
pub fn eclass8_set(eclass: &mut [u32], idx: usize, val: u8) {
    let wi = idx >> 2;
    let byte = (idx & 3) * 8;
    let mut word = eclass[wi];
    word &= !(0xffu32 << byte);
    word |= (val as u32) << byte;
    eclass[wi] = word;
}

#[inline]
pub fn mainGtU(
    mut i1: u32,
    mut i2: u32,
    block: &[u8],
    quadrant: &[u16],
    nblock: u32,
    budget: &mut i32,
) -> bool {
    let mut k: i32;
    let mut c1: u8;
    let mut c2: u8;
    let mut s1: u16;
    let mut s2: u16;

    for _ in 0..12 {
        c1 = block[i1 as usize];
        c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
    }

    k = nblock as i32 + 8;

    loop {
        for _ in 0..8 {
            c1 = block[i1 as usize];
            c2 = block[i2 as usize];
            if c1 != c2 {
                return c1 > c2;
            }
            s1 = quadrant[i1 as usize];
            s2 = quadrant[i2 as usize];
            if s1 != s2 {
                return s1 > s2;
            }
            i1 = i1.wrapping_add(1);
            i2 = i2.wrapping_add(1);
        }

        if i1 >= nblock {
            i1 -= nblock;
        }
        if i2 >= nblock {
            i2 -= nblock;
        }

        k -= 8;
        *budget -= 1;

        if k < 0 {
            break;
        }
    }

    false
}

pub static incs: [i32; 14] = [
    1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524, 88573, 265720, 797161, 2391484,
];

pub fn mainSimpleSort(
    ptr: &mut [u32],
    block: &[u8],
    quadrant: &[u16],
    nblock: i32,
    lo: i32,
    hi: i32,
    d: i32,
    budget: &mut i32,
) {
    let mut i: i32;
    let mut j: i32;
    let mut h: i32;
    let bigN: i32;
    let mut hp: i32;
    let mut v: u32;

    bigN = hi - lo + 1;
    if bigN < 2 {
        return;
    }

    hp = 0;
    while incs[hp as usize] < bigN {
        hp += 1;
    }
    hp -= 1;

    while hp >= 0 {
        h = incs[hp as usize];

        i = lo + h;
        loop {
            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;
            while mainGtU(
                ptr[(j - h) as usize].wrapping_add(d as u32),
                v.wrapping_add(d as u32),
                block,
                quadrant,
                nblock as u32,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j = j - h;
                if j <= (lo + h - 1) {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;
            while mainGtU(
                ptr[(j - h) as usize].wrapping_add(d as u32),
                v.wrapping_add(d as u32),
                block,
                quadrant,
                nblock as u32,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j = j - h;
                if j <= (lo + h - 1) {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if i > hi {
                break;
            }
            v = ptr[i as usize];
            j = i;
            while mainGtU(
                ptr[(j - h) as usize].wrapping_add(d as u32),
                v.wrapping_add(d as u32),
                block,
                quadrant,
                nblock as u32,
                budget,
            ) {
                ptr[j as usize] = ptr[(j - h) as usize];
                j = j - h;
                if j <= (lo + h - 1) {
                    break;
                }
            }
            ptr[j as usize] = v;
            i += 1;

            if *budget < 0 {
                return;
            }
        }

        hp -= 1;
    }
}

#[inline]
pub fn mmed3(a: u8, b: u8, c: u8) -> u8 {
    let mut a = a;
    let mut b = b;
    let t: u8;
    if a > b {
        t = a;
        a = b;
        b = t;
    }
    if b > c {
        b = c;
        if a > b {
            b = a;
        }
    }
    b
}

pub fn mvswap(ptr: &mut [u32], zzp1: i32, zzp2: i32, zzn: i32) {
    let mut yyp1 = zzp1;
    let mut yyp2 = zzp2;
    let mut yyn = zzn;
    while yyn > 0 {
        ptr.swap(yyp1 as usize, yyp2 as usize);
        yyp1 += 1;
        yyp2 += 1;
        yyn -= 1;
    }
}

pub const MAIN_QSORT_SMALL_THRESH: i32 = 20;
pub const MAIN_QSORT_DEPTH_THRESH: i32 =
    crate::bzlib_private::BZ_N_RADIX + crate::bzlib_private::BZ_N_QSORT;
pub const MAIN_QSORT_STACK_SIZE: usize = 100;

pub fn mainQSort3(
    ptr: &mut [u32],
    block: &[u8],
    quadrant: &[u16],
    nblock: i32,
    loSt: i32,
    hiSt: i32,
    dSt: i32,
    budget: &mut i32,
) {
    let mut unLo: i32;
    let mut unHi: i32;
    let mut ltLo: i32;
    let mut gtHi: i32;
    let mut n: i32;
    let mut m: i32;
    let mut med: i32;
    let mut sp: usize;
    let mut lo: i32;
    let mut hi: i32;
    let mut d: i32;

    let mut stackLo = [0i32; MAIN_QSORT_STACK_SIZE];
    let mut stackHi = [0i32; MAIN_QSORT_STACK_SIZE];
    let mut stackD = [0i32; MAIN_QSORT_STACK_SIZE];

    let mut nextLo = [0i32; 3];
    let mut nextHi = [0i32; 3];
    let mut nextD = [0i32; 3];

    sp = 0;
    stackLo[sp] = loSt;
    stackHi[sp] = hiSt;
    stackD[sp] = dSt;
    sp += 1;

    while sp > 0 {
        sp -= 1;
        lo = stackLo[sp];
        hi = stackHi[sp];
        d = stackD[sp];

        if hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH {
            mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 {
                return;
            }
            continue;
        }

        med = mmed3(
            block[(ptr[lo as usize] as i32 + d) as usize],
            block[(ptr[hi as usize] as i32 + d) as usize],
            block[(ptr[((lo + hi) >> 1) as usize] as i32 + d) as usize],
        ) as i32;

        unLo = lo;
        ltLo = lo;
        unHi = hi;
        gtHi = hi;

        loop {
            loop {
                if unLo > unHi {
                    break;
                }
                n = (block[(ptr[unLo as usize] as i32 + d) as usize] as i32) - med;
                if n == 0 {
                    ptr.swap(unLo as usize, ltLo as usize);
                    ltLo += 1;
                    unLo += 1;
                    continue;
                }
                if n > 0 {
                    break;
                }
                unLo += 1;
            }
            loop {
                if unLo > unHi {
                    break;
                }
                n = (block[(ptr[unHi as usize] as i32 + d) as usize] as i32) - med;
                if n == 0 {
                    ptr.swap(unHi as usize, gtHi as usize);
                    gtHi -= 1;
                    unHi -= 1;
                    continue;
                }
                if n < 0 {
                    break;
                }
                unHi -= 1;
            }
            if unLo > unHi {
                break;
            }
            ptr.swap(unLo as usize, unHi as usize);
            unLo += 1;
            unHi -= 1;
        }

        if gtHi < ltLo {
            stackLo[sp] = lo;
            stackHi[sp] = hi;
            stackD[sp] = d + 1;
            sp += 1;
            continue;
        }

        n = if (ltLo - lo) < (unLo - ltLo) {
            ltLo - lo
        } else {
            unLo - ltLo
        };
        mvswap(ptr, lo, unLo - n, n);
        m = if (hi - gtHi) < (gtHi - unHi) {
            hi - gtHi
        } else {
            gtHi - unHi
        };
        mvswap(ptr, unLo, hi - m + 1, m);

        n = lo + unLo - ltLo - 1;
        m = hi - (gtHi - unHi) + 1;

        nextLo[0] = lo;
        nextHi[0] = n;
        nextD[0] = d;
        nextLo[1] = m;
        nextHi[1] = hi;
        nextD[1] = d;
        nextLo[2] = n + 1;
        nextHi[2] = m - 1;
        nextD[2] = d + 1;

        if mnextsize(&nextLo, &nextHi, 0) < mnextsize(&nextLo, &nextHi, 1) {
            mnextswap(&mut nextLo, &mut nextHi, &mut nextD, 0, 1);
        }
        if mnextsize(&nextLo, &nextHi, 1) < mnextsize(&nextLo, &nextHi, 2) {
            mnextswap(&mut nextLo, &mut nextHi, &mut nextD, 1, 2);
        }
        if mnextsize(&nextLo, &nextHi, 0) < mnextsize(&nextLo, &nextHi, 1) {
            mnextswap(&mut nextLo, &mut nextHi, &mut nextD, 0, 1);
        }

        stackLo[sp] = nextLo[0];
        stackHi[sp] = nextHi[0];
        stackD[sp] = nextD[0];
        sp += 1;
        stackLo[sp] = nextLo[1];
        stackHi[sp] = nextHi[1];
        stackD[sp] = nextD[1];
        sp += 1;
        stackLo[sp] = nextLo[2];
        stackHi[sp] = nextHi[2];
        stackD[sp] = nextD[2];
        sp += 1;
    }
}

#[inline]
pub fn mnextsize(nextLo: &[i32; 3], nextHi: &[i32; 3], az: usize) -> i32 {
    nextHi[az] - nextLo[az]
}

#[inline]
pub fn mnextswap(nextLo: &mut [i32; 3], nextHi: &mut [i32; 3], nextD: &mut [i32; 3], az: usize, bz: usize) {
    nextLo.swap(az, bz);
    nextHi.swap(az, bz);
    nextD.swap(az, bz);
}

pub const SETMASK: u32 = 1 << 21;
pub const CLEARMASK: u32 = !SETMASK;

#[inline]
pub fn bigfreq(ftab: &[u32], b: i32) -> i32 {
    ftab[((b + 1) << 8) as usize] as i32 - ftab[(b << 8) as usize] as i32
}

pub fn mainSort(
    ptr: &mut [u32],
    block: &mut [u8],
    quadrant: &mut [u16],
    ftab: &mut [u32],
    nblock: i32,
    verb: i32,
    budget: &mut i32,
) {
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut ss: i32;
    let mut sb: i32;
    let mut runningOrder = [0i32; 256];
    let mut bigDone = [false; 256];
    let mut copyStart = [0i32; 256];
    let mut copyEnd = [0i32; 256];
    let mut c1: u8;
    let mut numQSorted: i32;
    let mut s: u16;

    for ii in (0..=65536i32).rev() {
        ftab[ii as usize] = 0;
    }

    j = (block[0] as i32) << 8;
    i = nblock - 1;
    while i >= 3 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as u16 as i32) << 8);
        ftab[j as usize] += 1;
        quadrant[(i - 1) as usize] = 0;
        j = (j >> 8) | ((block[(i - 1) as usize] as u16 as i32) << 8);
        ftab[j as usize] += 1;
        quadrant[(i - 2) as usize] = 0;
        j = (j >> 8) | ((block[(i - 2) as usize] as u16 as i32) << 8);
        ftab[j as usize] += 1;
        quadrant[(i - 3) as usize] = 0;
        j = (j >> 8) | ((block[(i - 3) as usize] as u16 as i32) << 8);
        ftab[j as usize] += 1;
        i -= 4;
    }
    while i >= 0 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as u16 as i32) << 8);
        ftab[j as usize] += 1;
        i -= 1;
    }

    for ii in 0..(crate::bzlib_private::BZ_N_OVERSHOOT) {
        block[(nblock + ii) as usize] = block[ii as usize];
        quadrant[(nblock + ii) as usize] = 0;
    }

    for ii in 1..=65536usize {
        ftab[ii] += ftab[ii - 1];
    }

    s = (block[0] as u16) << 8;
    i = nblock - 1;
    while i >= 3 {
        s = (s >> 8) | ((block[i as usize] as u16) << 8);
        j = ftab[s as usize] as i32 - 1;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = i as u32;
        s = (s >> 8) | ((block[(i - 1) as usize] as u16) << 8);
        j = ftab[s as usize] as i32 - 1;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = (i - 1) as u32;
        s = (s >> 8) | ((block[(i - 2) as usize] as u16) << 8);
        j = ftab[s as usize] as i32 - 1;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = (i - 2) as u32;
        s = (s >> 8) | ((block[(i - 3) as usize] as u16) << 8);
        j = ftab[s as usize] as i32 - 1;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = (i - 3) as u32;
        i -= 4;
    }
    while i >= 0 {
        s = (s >> 8) | ((block[i as usize] as u16) << 8);
        j = ftab[s as usize] as i32 - 1;
        ftab[s as usize] = j as u32;
        ptr[j as usize] = i as u32;
        i -= 1;
    }

    for ii in 0..=255usize {
        bigDone[ii] = false;
        runningOrder[ii] = ii as i32;
    }

    {
        let mut vv: i32;
        let mut h: i32 = 1;
        loop {
            h = 3 * h + 1;
            if h > 256 {
                break;
            }
        }
        loop {
            h = h / 3;
            i = h;
            while i <= 255 {
                vv = runningOrder[i as usize];
                j = i;
                loop {
                    if !(bigfreq(ftab, runningOrder[(j - h) as usize]) > bigfreq(ftab, vv)) {
                        break;
                    }
                    runningOrder[j as usize] = runningOrder[(j - h) as usize];
                    j = j - h;
                    if j <= (h - 1) {
                        break;
                    }
                }
                runningOrder[j as usize] = vv;
                i += 1;
            }
            if h == 1 {
                break;
            }
        }
    }

    numQSorted = 0;

    for i_big in 0..=255i32 {
        ss = runningOrder[i_big as usize];

        j = 0;
        while j <= 255 {
            if j != ss {
                sb = (ss << 8) + j;
                if (ftab[sb as usize] & SETMASK) == 0 {
                    let lo = (ftab[sb as usize] & CLEARMASK) as i32;
                    let hi = (ftab[(sb + 1) as usize] & CLEARMASK) as i32 - 1;
                    if hi > lo {
                        mainQSort3(
                            ptr,
                            block,
                            quadrant,
                            nblock,
                            lo,
                            hi,
                            crate::bzlib_private::BZ_N_RADIX,
                            budget,
                        );
                        numQSorted += hi - lo + 1;
                        if *budget < 0 {
                            return;
                        }
                    }
                }
                ftab[sb as usize] |= SETMASK;
            }
            j += 1;
        }

        {
            for jj in 0..=255usize {
                copyStart[jj] = (ftab[(jj << 8) + ss as usize] & CLEARMASK) as i32;
                copyEnd[jj] = (ftab[(jj << 8) + ss as usize + 1] & CLEARMASK) as i32 - 1;
            }
            j = (ftab[(ss << 8) as usize] & CLEARMASK) as i32;
            while j < copyStart[ss as usize] {
                k = ptr[j as usize] as i32 - 1;
                if k < 0 {
                    k += nblock;
                }
                c1 = block[k as usize];
                if !bigDone[c1 as usize] {
                    ptr[copyStart[c1 as usize] as usize] = k as u32;
                    copyStart[c1 as usize] += 1;
                }
                j += 1;
            }
            j = (ftab[((ss + 1) << 8) as usize] & CLEARMASK) as i32 - 1;
            while j > copyEnd[ss as usize] {
                k = ptr[j as usize] as i32 - 1;
                if k < 0 {
                    k += nblock;
                }
                c1 = block[k as usize];
                if !bigDone[c1 as usize] {
                    ptr[copyEnd[c1 as usize] as usize] = k as u32;
                    copyEnd[c1 as usize] -= 1;
                }
                j -= 1;
            }
        }

        for jj in 0..=255usize {
            ftab[(jj << 8) + ss as usize] |= SETMASK;
        }

        bigDone[ss as usize] = true;

        if i_big < 255 {
            let bbStart = (ftab[(ss << 8) as usize] & CLEARMASK) as i32;
            let bbSize = (ftab[((ss + 1) << 8) as usize] & CLEARMASK) as i32 - bbStart;
            let mut shifts = 0i32;

            while (bbSize >> shifts) > 65534 {
                shifts += 1;
            }

            let mut jj = bbSize - 1;
            while jj >= 0 {
                let a2update = ptr[(bbStart + jj) as usize] as i32;
                let qVal = (jj >> shifts) as u16;
                quadrant[a2update as usize] = qVal;
                if a2update < crate::bzlib_private::BZ_N_OVERSHOOT {
                    quadrant[(a2update + nblock) as usize] = qVal;
                }
                jj -= 1;
            }
        }
    }

    let _ = numQSorted;
}

pub fn BZ2_blockSort(s: &mut EState) {
    let nblock: i32 = s.nblock;
    let verb: i32 = s.verbosity;
    let mut wfact: i32 = s.workFactor;
    let mut budget: i32;
    let budgetInit: i32;
    let mut i: i32;

    if nblock < 10000 {
        // In C, arr2 and block alias the same memory: eclass8 == (UChar*)eclass
        // == s->block. Here they are separate buffers, so we must copy the
        // block bytes into arr2 (as packed bytes) before fallbackSort reads
        // them via eclass8_get, and copy the reconstructed block back after.
        let mut arr1 = std::mem::take(&mut s.arr1);
        let mut arr2 = std::mem::take(&mut s.arr2);
        let mut ftab = std::mem::take(&mut s.ftab);
        let block = std::mem::take(&mut s.block);

        // seed arr2's low nblock bytes from block (little-endian packing)
        for idx in 0..(nblock as usize) {
            let wi = idx >> 2;
            let byte = (idx & 3) * 8;
            let mut word = arr2[wi];
            word &= !(0xffu32 << byte);
            word |= (block[idx] as u32) << byte;
            arr2[wi] = word;
        }

        fallbackSort(&mut arr1, &mut arr2, &mut ftab, nblock, verb);

        // reconstruct block from arr2's low nblock bytes
        let mut block = block;
        for idx in 0..(nblock as usize) {
            let wi = idx >> 2;
            let byte = (idx & 3) * 8;
            block[idx] = ((arr2[wi] >> byte) & 0xff) as u8;
        }

        // ptr aliases arr1 in C; expose sorted order via s.ptr
        s.ptr = arr1.clone();
        s.arr1 = arr1;
        s.arr2 = arr2;
        s.ftab = ftab;
        s.block = block;
    } else {
        i = nblock + crate::bzlib_private::BZ_N_OVERSHOOT;
        if i & 1 != 0 {
            i += 1;
        }
        let quad_off = i as usize;

        if wfact < 1 {
            wfact = 1;
        }
        if wfact > 100 {
            wfact = 100;
        }
        budgetInit = nblock * ((wfact - 1) / 3);
        budget = budgetInit;

        let mut ptr = std::mem::take(&mut s.ptr);
        if (ptr.len() as i32) < nblock {
            ptr = vec![0u32; nblock as usize];
        }
        let mut block = std::mem::take(&mut s.block);
        let mut ftab = std::mem::take(&mut s.ftab);

        let quad_len = (nblock + crate::bzlib_private::BZ_N_OVERSHOOT + 1) as usize;
        let mut quadrant = vec![0u16; quad_len];

        mainSort(
            &mut ptr,
            &mut block,
            &mut quadrant,
            &mut ftab,
            nblock,
            verb,
            &mut budget,
        );

        s.ptr = ptr;
        s.block = block;
        s.ftab = ftab;

        if budget < 0 {
            let mut arr1 = std::mem::take(&mut s.arr1);
            let mut arr2 = std::mem::take(&mut s.arr2);
            let mut ftab = std::mem::take(&mut s.ftab);
            let block = std::mem::take(&mut s.block);

            for idx in 0..(nblock as usize) {
                let wi = idx >> 2;
                let byte = (idx & 3) * 8;
                let mut word = arr2[wi];
                word &= !(0xffu32 << byte);
                word |= (block[idx] as u32) << byte;
                arr2[wi] = word;
            }

            fallbackSort(&mut arr1, &mut arr2, &mut ftab, nblock, verb);

            let mut block = block;
            for idx in 0..(nblock as usize) {
                let wi = idx >> 2;
                let byte = (idx & 3) * 8;
                block[idx] = ((arr2[wi] >> byte) & 0xff) as u8;
            }

            s.ptr = arr1.clone();
            s.arr1 = arr1;
            s.arr2 = arr2;
            s.ftab = ftab;
            s.block = block;
        }
    }

    s.origPtr = -1;
    i = 0;
    while i < s.nblock {
        if s.ptr[i as usize] == 0 {
            s.origPtr = i;
            break;
        }
        i += 1;
    }
}
