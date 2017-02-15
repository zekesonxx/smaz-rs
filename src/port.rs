
/// Compression codebook
pub static SMAZ_CB: [&'static str; 254] = [
" ", "the", "e", "t", "a", "of", "o", "and", "i", "n", "s", "e ", "r", " th",
" t", "in", "he", "th", "h", "he ", "to", "\r\n", "l", "s ", "d", " a", "an",
"er", "c", " o", "d ", "on", " of", "re", "of ", "t ", ", ", "is", "u", "at",
"   ", "n ", "or", "which", "f", "m", "as", "it", "that", "\n", "was", "en",
"  ", " w", "es", " an", " i", "\r", "f ", "g", "p", "nd", " s", "nd ", "ed ",
"w", "ed", "http://", "for", "te", "ing", "y ", "The", " c", "ti", "r ", "his",
"st", " in", "ar", "nt", ",", " to", "y", "ng", " h", "with", "le", "al", "to ",
"b", "ou", "be", "were", " b", "se", "o ", "ent", "ha", "ng ", "their", "\"",
"hi", "from", " f", "in ", "de", "ion", "me", "v", ".", "ve", "all", "re ",
"ri", "ro", "is ", "co", "f t", "are", "ea", ". ", "her", " m", "er ", " p",
"es ", "by", "they", "di", "ra", "ic", "not", "s, ", "d t", "at ", "ce", "la",
"h ", "ne", "as ", "tio", "on ", "n t", "io", "we", " a ", "om", ", a", "s o",
"ur", "li", "ll", "ch", "had", "this", "e t", "g ", "e\r\n", " wh", "ere",
" co", "e o", "a ", "us", " d", "ss", "\n\r\n", "\r\n\r", "=\"", " be", " e",
"s a", "ma", "one", "t t", "or ", "but", "el", "so", "l ", "e s", "s,", "no",
"ter", " wa", "iv", "ho", "e a", " r", "hat", "s t", "ns", "ch ", "wh", "tr",
"ut", "/", "have", "ly ", "ta", " ha", " on", "tha", "-", " l", "ati", "en ",
"pe", " re", "there", "ass", "si", " fo", "wa", "ec", "our", "who", "its", "z",
"fo", "rs", ">", "ot", "un", "<", "im", "th ", "nc", "ate", "><", "ver", "ad",
" we", "ly", "ee", " n", "id", " cl", "ac", "il", "</", "rt", " wi", "div",
"e, ", " it", "whi", " ma", "ge", "x", "e c", "men", ".com"
];

pub const SMAZ_CB_LEN1: [u8; 33] = [0, 2, 3, 4, 6, 8, 9, 10, 12, 18, 22, 24, 28, 38, 44, 45, 49, 57, 59, 60, 65, 81, 83, 90, 101, 109, 110, 197, 204, 219, 222, 225, 250];
pub const SMAZ_CB_LEN2: [u8; 121] = [5, 11, 14, 15, 16, 17, 20, 21, 23, 25, 26, 27, 29, 30, 31, 33, 35, 36, 37, 39, 41, 42, 46, 47, 51, 52, 53, 54, 56, 58, 61, 62, 66, 69, 71, 73, 74, 75, 77, 79, 80, 84, 85, 87, 88, 91, 92, 94, 95, 96, 98, 102, 104, 106, 108, 111, 114, 115, 117, 120, 121, 123, 125, 127, 129, 130, 131, 136, 137, 138, 139, 144, 145, 147, 150, 151, 152, 153, 157, 163, 164, 165, 166, 169, 171, 173, 178, 179, 180, 182, 183, 186, 187, 189, 192, 194, 195, 196, 200, 205, 208, 212, 214, 215, 220, 221, 223, 224, 226, 228, 230, 232, 234, 235, 236, 237, 239, 240, 241, 242, 249];
pub const SMAZ_CB_LEN3: [u8; 88] = [1, 7, 13, 19, 32, 34, 40, 50, 55, 63, 64, 68, 70, 72, 76, 78, 82, 89, 97, 99, 105, 107, 112, 113, 116, 118, 119, 122, 124, 126, 132, 133, 134, 135, 140, 141, 142, 143, 146, 148, 149, 154, 156, 158, 159, 160, 161, 162, 167, 168, 170, 172, 174, 175, 176, 177, 181, 184, 185, 188, 190, 191, 193, 199, 201, 202, 203, 206, 207, 209, 211, 213, 216, 217, 218, 227, 229, 231, 233, 238, 243, 244, 245, 246, 247, 248, 251, 252];
pub const SMAZ_CB_LEN4: [u8; 8] = [48, 86, 93, 103, 128, 155, 198, 253];
pub const SMAZ_CB_LEN5: [u8; 3] = [43, 100, 210];



fn flush_verbatim_buffer(output: &mut Vec<u8>, buffer: &mut Vec<u8>) {
    if buffer.is_empty() {
        return;
    } else if buffer.len() == 1 {
        output.push(254);
        output.append(buffer);
    } else {
        output.push(255);
        output.push((buffer.len()-1) as u8);
        output.append(buffer);
    }
}

#[allow(needless_range_loop)]
pub fn raw_compress(input: &[u8]) -> Vec<u8> {
    let mut inputoffset = 0usize;
    let mut output = Vec::with_capacity(input.len());
    let mut verbatim_buffer: Vec<u8> = Vec::with_capacity(32);

    while inputoffset < input.len() {

        // We're going to start by trying to find a code table entry
        // This works by starting with the longest entries (7 chars)
        // and then working down to looking for 1 char entries.
        let mut j = 7;
        let mut in_codetable = false;

        if j > input.len()-inputoffset {
            j = input.len()-inputoffset;
        }

        while j > 0 && !in_codetable {
            for index in 0..SMAZ_CB.len() {
                if *SMAZ_CB[index].as_bytes() == input[inputoffset..inputoffset+j] {
                    // We found a code table entry, so flush the verbatim buffer,
                    // add the byte for the code table entry we found,
                    // and finally remove as many chars from `input` as the
                    // entry was long.
                    flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
                    output.push(index as u8);
                    inputoffset += j;
                    in_codetable = true;
                    break;
                }
            }
            j -= 1;
        }


        // If we didn't find it anywhere in the code table
        // add it to the verbatim buffer
        if !in_codetable {
            verbatim_buffer.push(input[inputoffset]);
            inputoffset += 1;
        }


        // Flush the verbatim buffer if we've hit the 256 char limit
        // or if we've hit the end of the string.
        if verbatim_buffer.len() == 256 || input.len() == inputoffset {
            flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
        }
    }
    output.shrink_to_fit();
    output
}

#[allow(explicit_iter_loop)]
pub fn loopless_raw_compress(input: &[u8]) -> Vec<u8> {
    let mut inputoffset = 0usize;
    let mut output = Vec::with_capacity(input.len());
    let mut verbatim_buffer: Vec<u8> = Vec::with_capacity(32);

    macro_rules! input {
        ($pos: expr) => (input[inputoffset..inputoffset+$pos])
    }

    macro_rules! findlen {
        ($len: expr, $lenarr: expr, $maxlen: expr, $opcode: expr) => (
            if $maxlen >= $len && $opcode.is_none() {
                for i in $lenarr.iter() {
                    if input!($len) == (*SMAZ_CB[*i as usize].as_bytes()) {
                        $opcode = Some(*i);
                        inputoffset += SMAZ_CB[*i as usize].len();
                        break;
                    }
                }
            }
        )
    }

    while inputoffset < input.len() {

        let mut maxlen = 7;
        let mut opcode: Option<u8> = None;

        if maxlen > input.len()-inputoffset {
            maxlen = input.len()-inputoffset;
        }

        if maxlen == 7 && input!(7) == *b"http://" {
            opcode = Some(67);
            inputoffset += 7;
        }

        findlen!(5, SMAZ_CB_LEN5, maxlen, opcode);
        findlen!(4, SMAZ_CB_LEN4, maxlen, opcode);
        findlen!(3, SMAZ_CB_LEN3, maxlen, opcode);
        findlen!(2, SMAZ_CB_LEN2, maxlen, opcode);
        findlen!(1, SMAZ_CB_LEN1, maxlen, opcode);


        // If we didn't find it anywhere in the code table
        // add it to the verbatim buffer
        if let Some(opcode) = opcode {
            flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
            output.push(opcode);
        } else {
            verbatim_buffer.push(input[inputoffset]);
            inputoffset += 1;
        }


        // Flush the verbatim buffer if we've hit the 256 char limit
        // or if we've hit the end of the string.
        if input.len() == inputoffset || verbatim_buffer.len() == 256 {
            flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
        }
    }
    output.shrink_to_fit();
    output
}

pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut output = loopless_raw_compress(input);

    // Worst-case scenario resolution
    let worst_case = input.len()+(2*(input.len())/256);
    if worst_case < output.len() {
        output.clear();
        output.reserve(worst_case);
        for chunk in Vec::from(input).chunks(256) {
            if chunk.len() == 1 {
                output.push(254);
                output.push(chunk[0]);
            } else {
                output.push(255);
                output.push((chunk.len()-1) as u8);
                output.extend_from_slice(chunk);
            }
        }
        output.shrink_to_fit();
    }

    output
}

pub fn decompress(input: &[u8]) -> Vec<u8> {
    let input = Vec::from(input);
    // rough guess is 50% or worse compression,
    // just to try to minimize reallocations.
    let mut output = Vec::with_capacity(input.len()*2);

    let mut iter = input.iter();
    while let Some(c) = iter.next() {
        if *c == 254 {
            // Verbatim byte
            // This goes:
            // +------+----------+
            // | 0xFE | the byte |
            // +------+----------+

            output.push(*iter.next().unwrap());
        } else if *c == 255 {
            // Verbatim string
            // This goes:
            // +------+--------+------------+
            // | 0xFF | length | the string |
            // +------+--------+------------+

            let len = *iter.next().unwrap();

            // the length is off by one
            // because if it was length 1, it would use the verbatim byte.
            // a +1 is also in the original C decompressor code
            for _ in 0..len+1 {
                output.push(*iter.next().unwrap());
            }
        } else {
            // Codebook entry
            // all codebook entries are a single byte

            for k in SMAZ_CB[*c as usize].as_bytes() {
                output.push(*k);
            }
        }
    }
    output.shrink_to_fit();
    output
}

pub fn generate_opcode_len_arrays() {
    let mut one: Vec<u8> = Vec::with_capacity(34);
    let mut two: Vec<u8> = Vec::with_capacity(120);
    let mut three: Vec<u8> = Vec::with_capacity(87);
    let mut four: Vec<u8> = Vec::with_capacity(8);
    let mut five: Vec<u8> = Vec::with_capacity(3);

    for j in 0..SMAZ_CB.len() as u8 {
        match SMAZ_CB[j as usize].len() {
            1 => one.push(j),
            2 => two.push(j),
            3 => three.push(j),
            4 => four.push(j),
            5 => five.push(j),
            _ => {}
        }
    }
    println!("pub const SMAZ_CB_LEN1: [u8; {}] = {:?};", one.len(), one);
    println!("pub const SMAZ_CB_LEN2: [u8; {}] = {:?};", two.len(), two);
    println!("pub const SMAZ_CB_LEN3: [u8; {}] = {:?};", three.len(), three);
    println!("pub const SMAZ_CB_LEN4: [u8; {}] = {:?};", four.len(), four);
    println!("pub const SMAZ_CB_LEN5: [u8; {}] = {:?};", five.len(), five);
    println!("pub const SMAZ_CB_LEN7: [u8; 1] = [67];");

}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::{thread_rng, Rng};

    macro_rules! compress_check_decompress {
        ($input: expr, $correct: expr) => (
            let compressed = compress($input);
            assert_eq!(compressed.as_slice(), $correct);
            assert_eq!(decompress(compressed.as_slice()), $input);
        )
    }

    #[test]
    fn basic_tests() {
        // some generic ones
        compress_check_decompress!(b"a simple string", b"\xA3\xD4\x2D\x3C\x57\x3E\xC3\x46");
        compress_check_decompress!(b"http://google.com", b"\x43\x3B\x06\x06\x3B\x57\xFD");

        // some specially crafted ones

        // ends with a literal string
        compress_check_decompress!(b"there there DDD", b"\xD2\x0D\xA0\x00\xFF\x02\x44\x44\x44");
    }

    lazy_static! {
        static ref FIXTURE_LINES: Vec<&'static str> = include_str!("../test-fixture.txt").lines().collect();
    }

    #[bench]
    fn bench_rust_ver(b: &mut Bencher) {
        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = raw_compress(line);
            assert_eq!(decompress(compressed.as_slice()).as_slice(), line);
        });
    }

    #[bench]
    fn bench_new_rust_ver(b: &mut Bencher) {
        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = loopless_raw_compress(line);
            assert_eq!(decompress(compressed.as_slice()).as_slice(), line);
        });
    }

    #[bench]
    fn bench_c_ver(b: &mut Bencher) {
        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = super::super::smaz_compress_clean(line);
            assert_eq!(decompress(compressed.as_slice()).as_slice(), line);
        });
    }
}
