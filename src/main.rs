
#![allow(dead_code)]
#![allow(unknown_lints)]
#![feature(test)]
extern crate test;

#[macro_use] extern crate lazy_static;

#[cfg(feature = "cbinding")]
extern crate rand;
#[cfg(feature = "cbinding")]
extern crate libc;


mod port;
#[cfg(feature = "cbinding")]
mod cbinding;


//fn test_string(input: &str) {
//    let bytes = input.as_bytes();
//    let compressed = cbinding::smaz_compress_clean(bytes);
//    let decompressed = cbinding::smaz_decompress_clean(compressed.as_slice());
//
//    if bytes != decompressed.as_slice() {
//        println!("ERROR! Mismatch!");
//        println!("attempting to compress: '{}'", input);
//        println!("original: {:?}", bytes);
//        println!("compressed: {:?}", compressed);
//        println!("decompressed: {:?}", decompressed);
//        ::std::process::exit(1);
//    }
//
//    // Rust decompressor test
//    let portdecompressed = port::decompress(compressed.as_slice());
//    if bytes != portdecompressed.as_slice() {
//        println!("Port didn't decompress right:");
//        println!("attempting to compress: '{}'", input);
//        println!("original: {:?}", bytes);
//        println!("compressed: {:?}", compressed);
//        println!("port decompressed: {:?}", portdecompressed);
//        ::std::process::exit(1);
//    }
//
//
//    let ratio = 100isize-((100*compressed.len())/bytes.len()) as isize;
//    if ratio > 0 {
//        print!("'{}' compressed by {}%", input, ratio);
//    } else {
//        print!("'{}' enlarged by {}%", input, -ratio);
//    }
//    println!(" ({}B->{}B)", bytes.len(), compressed.len());
//}

#[cfg(feature = "cbinding")]
fn main() {
    use std::env;

    for argument in env::args() {
        println!("compressing: '{}'", argument);
        let cver = cbinding::smaz_compress_clean(argument.as_bytes());
        let rustver = port::compress(argument.as_bytes());
        println!("----");
        println!("compressed:");
        println!("   cver: {:?}", cver);
        let ratio = 100isize-((100*cver.len())/argument.len()) as isize;
        println!("         {}% compression, ({}B->{}B)", ratio, argument.len(), cver.len());
        println!("rustver: {:?}", rustver);
        let ratio = 100isize-((100*rustver.len())/argument.len()) as isize;
        println!("         {}% compression ({}B->{}B)", ratio, argument.len(), rustver.len());
        let cver_de = cbinding::smaz_decompress_clean(rustver.as_slice());
        let rustver_de = port::decompress(cver.as_slice());
        println!("----");
        println!("decompressed:");
        println!("   cver: {:?}", cver_de);
        println!("         {:?}", String::from_utf8(cver_de));
        println!("rustver: {:?}", rustver_de);
        println!("         {:?}", String::from_utf8(rustver_de));

    }
}

#[cfg(not(feature = "cbinding"))]
fn main() {
    use std::env;

    for argument in env::args() {
        println!("compressing: '{}'", argument);
        let rustver = port::compress(argument.as_bytes());
        println!("----");
        println!("compressed:");
        println!("rustver: {:?}", rustver);
        let ratio = 100isize-((100*rustver.len())/argument.len()) as isize;
        println!("         {}% compression ({}B->{}B)", ratio, argument.len(), rustver.len());
        let rustver_de = port::decompress(rustver.as_slice());
        println!("----");
        println!("decompressed:");
        println!("rustver: {:?}", rustver_de);
        println!("         {:?}", String::from_utf8(rustver_de));

    }
}
