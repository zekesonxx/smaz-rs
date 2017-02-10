
#![allow(dead_code)]

extern crate libc;
use libc::c_int;

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
    let outlen = input.len()*3;
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
    //println!("compressing");
    let compressed = smaz_compress_clean(bytes);
    //println!("decompressing");
    let decompressed = smaz_decompress_clean(compressed.as_slice());

    if bytes != decompressed.as_slice() {
        println!("ERROR! Mismatch!");
        println!("attempting to compress: '{}'", input);
        println!("original: {:?}", bytes);
        println!("compressed: {:?}", compressed);
        println!("decompressed: {:?}", decompressed);
    }

    let ratio = 100isize-((100*compressed.len())/bytes.len()) as isize;
    if ratio > 0 {
        println!("'{}' compressed by {}%", input, ratio);
    } else {
        println!("'{}' enlarged by {}%", input, -ratio);
    }
}

fn main() {
    test_string("This is a small string");
    test_string("foobar");
    test_string("the end");
    test_string("not-a-g00d-Exampl333");
    test_string("Smaz is a simple compression library");
    test_string("Nothing is more difficult, and therefore more precious, than to be able to decide");
    test_string("this is an example of what works very well with smaz");
    test_string("1000 numbers 2000 will 10 20 30 compress very little");
    test_string("and now a few italian sentences:");
    test_string("Nel mezzo del cammin di nostra vita, mi ritrovai in una selva oscura");
    test_string("Mi illumino di immenso");
    test_string("L'autore di questa libreria vive in Sicilia");
    test_string("try it against urls");
    test_string("http://google.com");
    test_string("http://programming.reddit.com");
    test_string("http://github.com/antirez/smaz/tree/master");
    test_string("/media/hdb1/music/Alben/The Bla");
}
