
/// Compression codebook
pub const SMAZ_CB: [&'static str; 254] = [
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
    let mut input = Vec::from(input);
    let mut output = Vec::with_capacity(input.len());
    let mut verbatim_buffer: Vec<u8> = vec![];

    while !input.is_empty() {

        // We're going to start by trying to find a code table entry
        // This works by starting with the longest entries (7 chars)
        // and then working down to looking for 1 char entries.
        let mut j = 7;
        let mut in_codetable = false;
        if j > input.len() {
            j = input.len();
        }
        while j > 0 {
            if in_codetable {
                break;
            }
            for index in 0..SMAZ_CB.len() {
                if *SMAZ_CB[index].as_bytes() == input[..j] {
                    // We found a code table entry, so flush the verbatim buffer,
                    // add the byte for the code table entry we found,
                    // and finally remove as many chars from `input` as the
                    // entry was long.
                    flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
                    output.push(index as u8);
                    input.drain(0..j);
                    in_codetable = true;
                    break;
                }
            }
            j -= 1;
        }


        // If we didn't find it anywhere in the code table
        // add it to the verbatim buffer
        if !in_codetable {
            verbatim_buffer.push(input.remove(0));
        }


        // Flush the verbatim buffer if we've hit the 256 char limit
        // or if we've hit the end of the string.
        if verbatim_buffer.len() == 256 || input.is_empty() {
            flush_verbatim_buffer(&mut output, &mut verbatim_buffer);
        }
    }
    output.shrink_to_fit();
    output
}

pub fn compress(input: &[u8]) -> Vec<u8> {
    let mut output = raw_compress(input);

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
