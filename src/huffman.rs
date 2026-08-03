#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case)]
use crate::bzlib_private::{BZ_MAX_ALPHA_SIZE, BZ_MAX_CODE_LEN};

#[inline]
pub fn weightof(zz: i32) -> i32 {
    zz & (0xffffff00u32 as i32)
}

#[inline]
pub fn depthof(zz: i32) -> i32 {
    zz & 0x000000ff
}

#[inline]
pub fn mymax(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

#[inline]
pub fn addweights(zw1: i32, zw2: i32) -> i32 {
    (weightof(zw1) + weightof(zw2)) | (1 + mymax(depthof(zw1), depthof(zw2)))
}

pub fn BZ2_hbMakeCodeLengths(
    len: &mut [u8],
    freq: &[i32],
    alpha_size: i32,
    max_len: i32,
) {
    let alpha_size = alpha_size as usize;

    let mut heap = [0i32; BZ_MAX_ALPHA_SIZE + 2];
    let mut weight = [0i32; BZ_MAX_ALPHA_SIZE * 2];
    let mut parent = [0i32; BZ_MAX_ALPHA_SIZE * 2];

    for i in 0..alpha_size {
        weight[i + 1] = (if freq[i] == 0 { 1 } else { freq[i] }) << 8;
    }

    loop {
        let mut n_nodes = alpha_size as i32;
        let mut n_heap: i32 = 0;

        heap[0] = 0;
        weight[0] = 0;
        parent[0] = -2;

        for i in 1..=(alpha_size as i32) {
            parent[i as usize] = -1;
            n_heap += 1;
            heap[n_heap as usize] = i;
            // UPHEAP(n_heap)
            {
                let mut zz = n_heap;
                let tmp = heap[zz as usize];
                while weight[tmp as usize] < weight[heap[(zz >> 1) as usize] as usize] {
                    heap[zz as usize] = heap[(zz >> 1) as usize];
                    zz >>= 1;
                }
                heap[zz as usize] = tmp;
            }
        }

        while n_heap > 1 {
            let n1 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            // DOWNHEAP(1)
            {
                let mut zz: i32 = 1;
                let tmp = heap[zz as usize];
                loop {
                    let mut yy = zz << 1;
                    if yy > n_heap { break; }
                    if yy < n_heap
                        && weight[heap[(yy + 1) as usize] as usize]
                            < weight[heap[yy as usize] as usize]
                    {
                        yy += 1;
                    }
                    if weight[tmp as usize] < weight[heap[yy as usize] as usize] { break; }
                    heap[zz as usize] = heap[yy as usize];
                    zz = yy;
                }
                heap[zz as usize] = tmp;
            }

            let n2 = heap[1];
            heap[1] = heap[n_heap as usize];
            n_heap -= 1;
            // DOWNHEAP(1)
            {
                let mut zz: i32 = 1;
                let tmp = heap[zz as usize];
                loop {
                    let mut yy = zz << 1;
                    if yy > n_heap { break; }
                    if yy < n_heap
                        && weight[heap[(yy + 1) as usize] as usize]
                            < weight[heap[yy as usize] as usize]
                    {
                        yy += 1;
                    }
                    if weight[tmp as usize] < weight[heap[yy as usize] as usize] { break; }
                    heap[zz as usize] = heap[yy as usize];
                    zz = yy;
                }
                heap[zz as usize] = tmp;
            }

            n_nodes += 1;
            parent[n1 as usize] = n_nodes;
            parent[n2 as usize] = n_nodes;
            weight[n_nodes as usize] = addweights(weight[n1 as usize], weight[n2 as usize]);
            parent[n_nodes as usize] = -1;
            n_heap += 1;
            heap[n_heap as usize] = n_nodes;
            // UPHEAP(n_heap)
            {
                let mut zz = n_heap;
                let tmp = heap[zz as usize];
                while weight[tmp as usize] < weight[heap[(zz >> 1) as usize] as usize] {
                    heap[zz as usize] = heap[(zz >> 1) as usize];
                    zz >>= 1;
                }
                heap[zz as usize] = tmp;
            }
        }

        let mut too_long = false;
        for i in 1..=(alpha_size as i32) {
            let mut j: i32 = 0;
            let mut k = i;
            while parent[k as usize] >= 0 {
                k = parent[k as usize];
                j += 1;
            }
            len[(i - 1) as usize] = j as u8;
            if j > max_len {
                too_long = true;
            }
        }

        if !too_long { break; }

        for i in 1..=(alpha_size as i32) {
            let mut j = weight[i as usize] >> 8;
            j = 1 + (j / 2);
            weight[i as usize] = j << 8;
        }
    }
}

pub fn BZ2_hbAssignCodes(
    code: &mut [i32],
    length: &[u8],
    min_len: i32,
    max_len: i32,
    alpha_size: i32,
) {
    let mut vec: i32 = 0;
    for n in min_len..=max_len {
        for i in 0..(alpha_size as usize) {
            if length[i] as i32 == n {
                code[i] = vec;
                vec += 1;
            }
        }
        vec <<= 1;
    }
}

pub fn BZ2_hbCreateDecodeTables(
    limit: &mut [i32],
    base: &mut [i32],
    perm: &mut [i32],
    length: &[u8],
    min_len: i32,
    max_len: i32,
    alpha_size: i32,
) {
    let mut pp: i32 = 0;
    for i in min_len..=max_len {
        for j in 0..(alpha_size as usize) {
            if length[j] as i32 == i {
                perm[pp as usize] = j as i32;
                pp += 1;
            }
        }
    }

    for i in 0..(BZ_MAX_CODE_LEN as usize) {
        base[i] = 0;
    }
    for i in 0..(alpha_size as usize) {
        base[length[i] as usize + 1] += 1;
    }
    for i in 1..(BZ_MAX_CODE_LEN as usize) {
        base[i] += base[i - 1];
    }

    for i in 0..(BZ_MAX_CODE_LEN as usize) {
        limit[i] = 0;
    }
    let mut vec: i32 = 0;

    for i in min_len..=max_len {
        vec += base[(i + 1) as usize] - base[i as usize];
        limit[i as usize] = vec - 1;
        vec <<= 1;
    }
    for i in (min_len + 1)..=max_len {
        base[i as usize] = ((limit[(i - 1) as usize] + 1) << 1) - base[i as usize];
    }
}
