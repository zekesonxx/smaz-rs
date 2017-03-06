use libc::c_int;

extern {
    fn smaz_compress(inbuf: *mut u8, inlen: c_int, out: *mut u8, outlen: c_int) -> c_int;
    fn smaz_decompress(inbuf: *mut u8, inlen: c_int, out: *mut u8, outlen: c_int) -> c_int;
}

pub fn smaz_compress_clean(input: &[u8]) -> Vec<u8> {
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

pub fn smaz_decompress_clean(input: &[u8]) -> Vec<u8> {
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
