
#![allow(dead_code)]
#![allow(unknown_lints)]
#![feature(test)]
extern crate test;
extern crate rand;
extern crate libc;
#[macro_use] extern crate lazy_static;
extern crate itertools;
use libc::c_int;

mod port;

extern {
    fn smaz_compress(inbuf: *mut u8, inlen: c_int, out: *mut u8, outlen: c_int) -> c_int;
    fn smaz_decompress(inbuf: *mut u8, inlen: c_int, out: *mut u8, outlen: c_int) -> c_int;
}

fn smaz_compress_clean(input: &[u8]) -> Vec<u8> {
    let mut input = Vec::from(input);
    let outlen = input.len()*3;
    let mut output: Vec<u8> = Vec::with_capacity(outlen);
    unsafe {
        let n = smaz_compress(input.as_mut_ptr(),
                              input.capacity() as c_int,
                              output.as_mut_ptr(),
                              output.capacity() as c_int);
        if n>outlen as c_int {
            panic!("failed to compress, outlen wasn't big enough");
        }
        output.set_len(n as usize);
        output.shrink_to_fit();
    }
    output
}

fn smaz_decompress_clean(input: &[u8]) -> Vec<u8> {
    let mut input = Vec::from(input);
    let outlen = input.len()*9;
    let mut output: Vec<u8> = Vec::with_capacity(outlen);
    unsafe {
        let n = smaz_decompress(input.as_mut_ptr(),
                                input.capacity() as c_int,
                                output.as_mut_ptr(),
                                output.capacity() as c_int);
        if n>outlen as c_int {
            panic!("failed to decompress, outlen wasn't big enough");
        }
        output.set_len(n as usize);
        output.shrink_to_fit();
    }
    output
}

fn test_string(input: &str) {
    let bytes = input.as_bytes();
    let compressed = smaz_compress_clean(bytes);
    let decompressed = smaz_decompress_clean(compressed.as_slice());

    if bytes != decompressed.as_slice() {
        println!("ERROR! Mismatch!");
        println!("attempting to compress: '{}'", input);
        println!("original: {:?}", bytes);
        println!("compressed: {:?}", compressed);
        println!("decompressed: {:?}", decompressed);
        ::std::process::exit(1);
    }

    // Rust decompressor test
    let portdecompressed = port::decompress(compressed.as_slice());
    if bytes != portdecompressed.as_slice() {
        println!("Port didn't decompress right:");
        println!("attempting to compress: '{}'", input);
        println!("original: {:?}", bytes);
        println!("compressed: {:?}", compressed);
        println!("port decompressed: {:?}", portdecompressed);
        ::std::process::exit(1);
    }


    let ratio = 100isize-((100*compressed.len())/bytes.len()) as isize;
    if ratio > 0 {
        print!("'{}' compressed by {}%", input, ratio);
    } else {
        print!("'{}' enlarged by {}%", input, -ratio);
    }
    println!(" ({}B->{}B)", bytes.len(), compressed.len());
}

fn main() {
    use std::env;

    for argument in env::args() {
        println!("compressing: '{}'", argument);
        let cver = smaz_compress_clean(argument.as_bytes());
        let rustver = port::compress(argument.as_bytes());
        println!("----");
        println!("compressed:");
        println!("   cver: {:?}", cver);
        let ratio = 100isize-((100*cver.len())/argument.len()) as isize;
        println!("         {}% compression, ({}B->{}B)", ratio, argument.len(), cver.len());
        println!("rustver: {:?}", rustver);
        let ratio = 100isize-((100*rustver.len())/argument.len()) as isize;
        println!("         {}% compression ({}B->{}B)", ratio, argument.len(), rustver.len());
        let cver_de = smaz_decompress_clean(rustver.as_slice());
        let rustver_de = port::decompress(cver.as_slice());
        println!("----");
        println!("decompressed:");
        println!("   cver: {:?}", cver_de);
        println!("         {:?}", String::from_utf8(cver_de));
        println!("rustver: {:?}", rustver_de);
        println!("         {:?}", String::from_utf8(rustver_de));

    }
}
