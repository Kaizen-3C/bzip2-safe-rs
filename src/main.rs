#![forbid(unsafe_code)]
#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_parens, non_snake_case, non_upper_case_globals)]

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for line in reader.lines() {
        if let Ok(line) = line {
            process_line(&line);
        }
    }
}

fn process_line(line: &str) {
    let parts: Vec<&str> = line.split(|c: char| c == ' ' || c == '\t' || c == '\r' || c == '\n')
        .filter(|s| !s.is_empty())
        .collect();
    
    if parts.len() < 4 {
        return;
    }
    
    let op = parts[0].chars().next().unwrap_or(' ');
    let level: i32 = parts[1].parse().unwrap_or(0);
    let small: i32 = parts[2].parse().unwrap_or(0);
    let hex_str = parts[3];
    
    let mut src = Vec::new();
    if hex_str != "-" {
        let hex_bytes = hex_str.as_bytes();
        let mut i = 0;
        while i + 1 < hex_bytes.len() {
            let hi = hexval(hex_bytes[i] as i32);
            let lo = hexval(hex_bytes[i + 1] as i32);
            if hi < 0 || lo < 0 {
                break;
            }
            src.push(((hi << 4) | lo) as u8);
            i += 2;
        }
    }
    
    if op == 'R' {
        let mut comp = vec![0u8; 300000];
        let mut comp_len = comp.len() as u32;
        let rc = bzip2_safe_rs::bzlib::BZ2_bzBuffToBuffCompress(
            &mut comp,
            &mut comp_len,
            &src,
            src.len() as u32,
            level,
            0,
            30,
        );
        
        print!("C {} {} ", rc, if rc == bzip2_safe_rs::bzlib::BZ_OK { comp_len } else { 0 });
        if rc == bzip2_safe_rs::bzlib::BZ_OK {
            print_hex(&comp, comp_len as usize, 100000);
        } else {
            print!("-");
        }
        println!();
        
        if rc == bzip2_safe_rs::bzlib::BZ_OK {
            let mut back = vec![0u8; 1048576];
            let mut back_len = back.len() as u32;
            let rc2 = bzip2_safe_rs::bzlib::BZ2_bzBuffToBuffDecompress(
                &mut back,
                &mut back_len,
                &comp,
                comp_len,
                small,
                0,
            );
            
            let status = if rc2 == bzip2_safe_rs::bzlib::BZ_OK && (back_len as usize) == src.len() && back[..src.len()] == src[..] {
                "OK"
            } else {
                "MISMATCH"
            };
            
            println!("R {} {} {}", rc2, if rc2 == bzip2_safe_rs::bzlib::BZ_OK { back_len } else { 0 }, status);
        }
    } else if op == 'D' {
        let mut back = vec![0u8; 1048576];
        let mut back_len = back.len() as u32;
        let rc = bzip2_safe_rs::bzlib::BZ2_bzBuffToBuffDecompress(
            &mut back,
            &mut back_len,
            &src,
            src.len() as u32,
            small,
            0,
        );
        
        print!("D {} {} ", rc, if rc == bzip2_safe_rs::bzlib::BZ_OK { back_len } else { 0 });
        if rc == bzip2_safe_rs::bzlib::BZ_OK {
            print_hex(&back, back_len as usize, 64);
        } else {
            print!("-");
        }
        println!();
    } else if op == 'P' {
        let mut comp = vec![0u8; 300000];
        let mut comp_len = comp.len() as u32;
        let rc = bzip2_safe_rs::bzlib::BZ2_bzBuffToBuffCompress(
            &mut comp,
            &mut comp_len,
            &src,
            src.len() as u32,
            level,
            0,
            30,
        );
        
        println!("P {}", rc);
    }
}

fn hexval(c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        c - '0' as i32
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        c - 'a' as i32 + 10
    } else if c >= 'A' as i32 && c <= 'F' as i32 {
        c - 'A' as i32 + 10
    } else {
        -1
    }
}

fn print_hex(p: &[u8], n: usize, maxbytes: usize) {
    let lim = if n < maxbytes { n } else { maxbytes };
    for i in 0..lim {
        print!("{:02x}", p[i]);
    }
    if n > maxbytes {
        print!("..");
    }
}
