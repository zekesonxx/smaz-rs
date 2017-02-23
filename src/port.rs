

use std::cmp;

pub static SMAZ_COMPRESSION_CB: [&[u8]; 254] = [b"\x01 \x00", b"\x03the\x01",
b"\x01e\x02", b"\x01t\x03", b"\x01a\x04", b"\x02of\x05", b"\x01o\x06",
b"\x03and\x07", b"\x01i\x08", b"\x01n\x09", b"\x01s\x0A", b"\x02e \x0B",
b"\x01r\x0C", b"\x03 th\x0D", b"\x02 t\x0E", b"\x02in\x0F", b"\x02he\x10",
b"\x02th\x11", b"\x01h\x12", b"\x03he \x13", b"\x02to\x14", b"\x02\r\n\x15",
b"\x01l\x16", b"\x02s \x17", b"\x01d\x18", b"\x02 a\x19", b"\x02an\x1A",
b"\x02er\x1B", b"\x01c\x1C", b"\x02 o\x1D", b"\x02d \x1E", b"\x02on\x1F",
b"\x03 of\x20", b"\x02re\x21", b"\x03of \x22", b"\x02t \x23", b"\x02, \x24",
b"\x02is\x25", b"\x01u\x26", b"\x02at\x27", b"\x03   \x28", b"\x02n \x29",
b"\x02or\x2A", b"\x05which\x2B", b"\x01f\x2C", b"\x01m\x2D", b"\x02as\x2E",
b"\x02it\x2F", b"\x04that\x30", b"\x01\n\x31", b"\x03was\x32", b"\x02en\x33",
b"\x02  \x34", b"\x02 w\x35", b"\x02es\x36", b"\x03 an\x37", b"\x02 i\x38",
b"\x01\r\x39", b"\x02f \x3A", b"\x01g\x3B", b"\x01p\x3C", b"\x02nd\x3D",
b"\x02 s\x3E", b"\x03nd \x3F", b"\x03ed \x40", b"\x01w\x41", b"\x02ed\x42",
b"\x07http://\x43", b"\x03for\x44", b"\x02te\x45", b"\x03ing\x46",
b"\x02y \x47", b"\x03The\x48", b"\x02 c\x49", b"\x02ti\x4A", b"\x02r \x4B",
b"\x03his\x4C", b"\x02st\x4D", b"\x03 in\x4E", b"\x02ar\x4F", b"\x02nt\x50",
b"\x01,\x51", b"\x03 to\x52", b"\x01y\x53", b"\x02ng\x54", b"\x02 h\x55",
b"\x04with\x56", b"\x02le\x57", b"\x02al\x58", b"\x03to \x59", b"\x01b\x5A",
b"\x02ou\x5B", b"\x02be\x5C", b"\x04were\x5D", b"\x02 b\x5E", b"\x02se\x5F",
b"\x02o \x60", b"\x03ent\x61", b"\x02ha\x62", b"\x03ng \x63", b"\x05their\x64",
b"\x01\"\x65", b"\x02hi\x66", b"\x04from\x67", b"\x02 f\x68", b"\x03in \x69",
b"\x02de\x6A", b"\x03ion\x6B", b"\x02me\x6C", b"\x01v\x6D", b"\x01.\x6E",
b"\x02ve\x6F", b"\x03all\x70", b"\x03re \x71", b"\x02ri\x72", b"\x02ro\x73",
b"\x03is \x74", b"\x02co\x75", b"\x03f t\x76", b"\x03are\x77", b"\x02ea\x78",
b"\x02. \x79", b"\x03her\x7A", b"\x02 m\x7B", b"\x03er \x7C", b"\x02 p\x7D",
b"\x03es \x7E", b"\x02by\x7F", b"\x04they\x80", b"\x02di\x81", b"\x02ra\x82",
b"\x02ic\x83", b"\x03not\x84", b"\x03s, \x85", b"\x03d t\x86", b"\x03at \x87",
b"\x02ce\x88", b"\x02la\x89", b"\x02h \x8A", b"\x02ne\x8B", b"\x03as \x8C",
b"\x03tio\x8D", b"\x03on \x8E", b"\x03n t\x8F", b"\x02io\x90", b"\x02we\x91",
b"\x03 a \x92", b"\x02om\x93", b"\x03, a\x94", b"\x03s o\x95", b"\x02ur\x96",
b"\x02li\x97", b"\x02ll\x98", b"\x02ch\x99", b"\x03had\x9A", b"\x04this\x9B",
b"\x03e t\x9C", b"\x02g \x9D", b"\x03e\r\n\x9E", b"\x03 wh\x9F", b"\x03ere\xA0",
b"\x03 co\xA1", b"\x03e o\xA2", b"\x02a \xA3", b"\x02us\xA4", b"\x02 d\xA5",
b"\x02ss\xA6", b"\x03\n\r\n\xA7", b"\x03\r\n\r\xA8", b"\x02=\"\xA9",
b"\x03 be\xAA", b"\x02 e\xAB", b"\x03s a\xAC", b"\x02ma\xAD", b"\x03one\xAE",
b"\x03t t\xAF", b"\x03or \xB0", b"\x03but\xB1", b"\x02el\xB2", b"\x02so\xB3",
b"\x02l \xB4", b"\x03e s\xB5", b"\x02s,\xB6", b"\x02no\xB7", b"\x03ter\xB8",
b"\x03 wa\xB9", b"\x02iv\xBA", b"\x02ho\xBB", b"\x03e a\xBC", b"\x02 r\xBD",
b"\x03hat\xBE", b"\x03s t\xBF", b"\x02ns\xC0", b"\x03ch \xC1", b"\x02wh\xC2",
b"\x02tr\xC3", b"\x02ut\xC4", b"\x01/\xC5", b"\x04have\xC6", b"\x03ly \xC7",
b"\x02ta\xC8", b"\x03 ha\xC9", b"\x03 on\xCA", b"\x03tha\xCB", b"\x01-\xCC",
b"\x02 l\xCD", b"\x03ati\xCE", b"\x03en \xCF", b"\x02pe\xD0", b"\x03 re\xD1",
b"\x05there\xD2", b"\x03ass\xD3", b"\x02si\xD4", b"\x03 fo\xD5", b"\x02wa\xD6",
b"\x02ec\xD7", b"\x03our\xD8", b"\x03who\xD9", b"\x03its\xDA", b"\x01z\xDB",
b"\x02fo\xDC", b"\x02rs\xDD", b"\x01>\xDE", b"\x02ot\xDF", b"\x02un\xE0",
b"\x01<\xE1", b"\x02im\xE2", b"\x03th \xE3", b"\x02nc\xE4", b"\x03ate\xE5",
b"\x02><\xE6", b"\x03ver\xE7", b"\x02ad\xE8", b"\x03 we\xE9", b"\x02ly\xEA",
b"\x02ee\xEB", b"\x02 n\xEC", b"\x02id\xED", b"\x03 cl\xEE", b"\x02ac\xEF",
b"\x02il\xF0", b"\x02</\xF1", b"\x02rt\xF2", b"\x03 wi\xF3", b"\x03div\xF4",
b"\x03e, \xF5", b"\x03 it\xF6", b"\x03whi\xF7", b"\x03 ma\xF8", b"\x02ge\xF9",
b"\x01x\xFA", b"\x03e c\xFB", b"\x03men\xFC", b"\x04.com\xFD"];


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
    if buffer.len() == 1 {
        output.push(254);
        output.append(buffer);
    } else if !buffer.is_empty() {
        output.push(255);
        output.push((buffer.len()-1) as u8);
        output.append(buffer);
    }
}

#[allow(explicit_iter_loop)]
pub fn raw_compress(input: &[u8]) -> Vec<u8> {
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
                        inputoffset += $len;
                        break;
                    }
                }
            }
        )
    }
    while inputoffset < input.len() {

        // length of the remainder of the string,
        // otherwise length 7 (the longest opcode)
        let maxlen = cmp::min(input.len()-inputoffset, 7);
        let mut opcode: Option<u8> = None;

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
            if !verbatim_buffer.is_empty() {
                flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
            }
            output.push(opcode);
        } else {
            verbatim_buffer.push(input[inputoffset]);
            inputoffset += 1;
        }


        // Flush the verbatim buffer if we've hit the 256 char limit
        // or if we've hit the end of the string.
        if !verbatim_buffer.is_empty() && (input.len() == inputoffset || verbatim_buffer.len() == 256) {
            flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
        }
    }
    output.shrink_to_fit();
    output
}


lazy_static! {
    static ref SORTED_CB: Vec<&'static [u8]> = {
        let mut k = Vec::from(SMAZ_COMPRESSION_CB.as_ref());
        k.sort();
        k
    };
}

pub fn lookup_table_compress(input: &[u8]) -> Vec<u8> {
    let mut inputoffset = 0usize;
    let mut output = Vec::with_capacity(input.len());
    let mut verbatim_buffer: Vec<u8> = Vec::with_capacity(32);

    macro_rules! input {
        ($pos: expr) => (input[inputoffset..inputoffset+$pos])
    }

    let mut opcode: Option<u8>;

    while inputoffset < input.len() {
        // length of the remainder of the string,
        // otherwise length 7 (the longest opcode)
        let maxlen = cmp::min(input.len()-inputoffset, 5);

        // Reset the opcode variable
        // We're resetting instead of just `let`ing to avoid an allocation
        opcode = None;

        // We do an inital check against the first character
        // to short-circuit the logic
        // and avoid the (comparatively) costly 7 char comparison
        // since, after all, 90+% of the time it won't be http://.
        if input[inputoffset] == b'h' && input.len()-inputoffset >= 7 && input!(7) == *b"http://" {
            opcode = Some(67);
            inputoffset += 7;
        } else {

            for len in (0..maxlen+1).rev() {
                let index = SORTED_CB.binary_search_by(|probe| {
                    use std::cmp::Ordering::*;
                    match probe[0].cmp(&(len as u8)) {
                        Less => Less,
                        Greater => Greater,
                        Equal => probe[1..len+1].cmp(&input!(len))
                    }
                });

                if let Ok(pos) = index {
                    // Found the opcode in question
                    // Add it to the array and continue
                    opcode = Some(SORTED_CB[pos][len+1]);
                    inputoffset += len;
                    break;
                }
            }
        }


        // If we didn't find it anywhere in the code table
        // add it to the verbatim buffer
        if let Some(opcode) = opcode {
            if !verbatim_buffer.is_empty() {
                flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
            }
            output.push(opcode);
        } else {
            verbatim_buffer.push(input[inputoffset]);
            inputoffset += 1;
        }


        // Flush the verbatim buffer if we've hit the 256 char limit
        // or if we've hit the end of the string.
        if (input.len() == inputoffset || verbatim_buffer.len() == 256) && !verbatim_buffer.is_empty() {
            flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
        }
    }

    output.shrink_to_fit();
    output
}

pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut output = lookup_table_compress(input);

    // Worst-case scenario resolution
    let worst_case = input.len()+(2*(input.len())/256);
    if worst_case < output.len() {
        output.clear();
        output.reserve(worst_case);
        for chunk in input.chunks(256) {
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

#[allow(needless_range_loop)]
pub fn generate_compression_codebook() {
    let mut book: Vec<String> = vec![];
    for j in 0..SMAZ_CB.len() {
        let element = SMAZ_CB[j];
        book.push(format!("b\"\\x{:02X}{}\\x{:02X}\"", element.len(), element.replace('\r', "\\r").replace('\n', "\\n").replace('"', "\\\""),
                                                       j));
    }
    println!("pub static SMAZ_COMPRESSION_CB: [&[u8]; {}] = [{}];", book.len(), book.join(", "));
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
    fn bench_compress_rust_ver(b: &mut Bencher) {
        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = raw_compress(line);
            assert_eq!(decompress(compressed.as_slice()).as_slice(), line);
        });
    }

    #[bench]
    fn bench_compress_rust_lookup_table_ver(b: &mut Bencher) {
        //to make sure the lazy_static'd codebook is sorted already
        let _ = lookup_table_compress(b"cats");

        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = lookup_table_compress(line);
            assert_eq!(decompress(compressed.as_slice()).as_slice(), line);
        });
    }

    #[bench]
    fn bench_compress_c_ver(b: &mut Bencher) {
        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = super::super::smaz_compress_clean(line);
            assert_eq!(decompress(compressed.as_slice()).as_slice(), line);
        });
    }


    #[bench]
    fn bench_decompress_rust(b: &mut Bencher) {
        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = super::super::smaz_compress_clean(line);
            assert_eq!(decompress(compressed.as_slice()).as_slice(), line);
        });
    }

    #[bench]
    fn bench_decompress_c(b: &mut Bencher) {
        let mut rng = thread_rng();
        b.iter(|| {
            let line = rng.choose(&FIXTURE_LINES).unwrap().as_bytes();
            let compressed = super::super::smaz_compress_clean(line);
            assert_eq!(super::super::smaz_decompress_clean(compressed.as_slice()).as_slice(), line);
        });
    }
}
