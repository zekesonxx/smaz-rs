# smaz: a short string text compression library

smaz is a very simple table-based compression format, designed for stretches of lowercase English strings under ~500 bytes.

In my (brief) testing, ~500 bytes is when gzip just starts to completely decimate smaz in comparison.

## The format
smaz works on a single-byte lookup table, spanning bytes 0-253.
You can find the lookup table as array `smaz_rcb` in the original `smaz.c` or as `SMAZ_CB` in my Rust port.

There are:
* 34 one-byte opcodes (ex "e")
* 120 two-byte opcodes (ex "he")
* 87 three-byte opcodes (ex "the")
* 8 four-byte opcodes (ex "this")
* 3 five-byte opcodes ("which", "there", and "their")
* 0 six-byte opcodes
* 1 seven-byte opcode ("http://")

Byte 254 is represents a literal byte following it:

```
+-----+------+
| 254 | byte |
+-----+------+
```

and byte 255 represents a literal string following it:
```
+-----+--------+------------+
| 255 | length | the string |
+-----+--------+------------+
```


## A couple of examples
```
+-----+-----+-----+-----+-----+------+
| 2   | 250 | 4   | 45  | 60  | 87   |
+-----+-----+-----+-----+-----+------+
| "e" | "x" | "a" | "m" | "p" | "le" |
+-----+-----+-----+-----+-----+------+
```

```
+---------+-------+-----+-----+-----+
| 210     | 233   | 0   | 59  | 6   |
+---------+-------+-----+-----+-----+
| "there" | " we" | " " | "g" | "o" |
+---------+-------+-----+-----+-----+
```

```
+-----+---+---+---+---+---+---+---+---+
| 255 | 6 | l | i | t | e | r | a | l |
+-----+---+---+---+---+---+---+---+---+
| "literal"                           |
+-------------------------------------+
```

```
+-----+---+-----+-----+-----+-----+-----+
| 255 | 1 | '3' | '3' | 28  | 254 | '3' |
+-----+---+-----+-----+-----+-----+-----+
| "33"                | "c" | "3"       |
+---------------------------------------+
```
This is an interesting edge case, since it takes the 4 byte input "33c3", and turns it into 7 bytes. Not really compression there.

```
+-----+---+-----+-----+-----+-----+---+-----+-----+-----+
| 255 | 1 | 'G' | 'L' | 4   | 255 | 2 | 'D' | 'O' | 'S' |
+-----+---+-----+-----+-----+-----+---+-----+-----+-----+
| "GL"                | "a" | "DOS"                     |
+-------------------------------------------------------+
```

Again, a compression failure, but smaz isn't designed to handle uppercase.


## What smaz is bad at
(from here on out I'm not doing the fancy box thing because it takes a while to do)
### https
Using one of the example strings:
```
input: "http://github.com/antirez/smaz/tree/master" (42 bytes)
compressed (23 bytes):
[67, 59, 47, 18, 38, 90, 253, 197, 26, 74, 33, 219, 197, 10, 173, 219, 197, 195, 235, 197, 173, 77, 27]
```
Wow, 46% compression. That's pretty insane. (for comparison, gzip gets -4.8%)
But, it's not 2008 anymore. It's 2017, and we need https on everything:
```
input: "https://github.com/antirez/smaz/tree/master" (43 bytes)
compressed (31 bytes):
[18, 3, 3, 60, 10, 254, 58, 197, 197, 59, 47, 18, 38, 90, 253, 197, 26, 74, 33, 219, 197, 10, 173, 219, 197, 195, 235, 197, 173, 77, 27]
```
We're down to 28%. What gives?

Well, you might've already guessed it, "http://" is a single opcode. And so is ".com".

Meaning:
* "http://google.com/" gets 56% compression.
* "https://google.com/" gets 16% compression.
* "https://google.net/" gets 6% compression.
the only savings there is "le" and "ne". Which gets undermined by the fact that ':' requires a literal byte opcode.
So that 6% compression that's left? Yeah, that's *one* byte of savings.





### Invoking Failure
If we really want to, we can craft strings that will fail really badly

```
input: "e3e3e3e3e3e3" (12 bytes)
compressed (18 bytes):
[2, 254, 51, 2, 254, 51, 2, 254, 51, 2, 254, 51, 2, 254, 51, 2, 254, 51]
```
enlarged by 50%. But, we can actually make it slightly worse:
```
input: "e33e33e33e33" (12 bytes)
compressed (20 bytes):
[2, 255, 1, 51, 51, 2, 255, 1, 51, 51, 2, 255, 1, 51, 51, 2, 255, 1, 51, 51]
```
This is alternating a single-character lookup-table string (the letter 'e') with a 2+ character non-lookup-table string ("33").
This turns 3 bytes ("e33") into 5, and is the *worst* case scenario for smaz, at 66% enlargement.

#### Damage control on worst-case scenarios
We can manually detect and deal with worst-case scenarios using the literal string opcode.

My Rust version handles this case:
```
input: "e33e33e33e33" (12 bytes)
compressed (14 bytes):
[255, 11, 101, 51, 51, 101, 51, 51, 101, 51, 51, 101, 51, 51]
```
Instead of allowing the worst case scenario through, it uses the literal string opcode to print out the entire input string.
Doing this makes the worst case scenario `len+(2*(len/256))` bytes long.

#### Invoking success
Similarly, we can craft strings that compress *really* well.
```
input: "he ate a hat" (12 bytes)
compressed (4 bytes):
[19, 229, 146, 190]
```
This garners us 67% compression, since "he ", "ate", " a ", and "hat" are all opcodes.

```
input: "theretheir" (10 bytes)
compressed (2 bytes):
[210, 100]
```
"there", "there", and "which" are the only 5 character opcodes, so putting two of them together gets us 80% compression.

```
input: "http://theretheir.com" (21 bytes)
compressed (4 bytes):
[67, 210, 100, 253]
```
81% compression. But, we can do just a bit better...

```
input: "http://http://http://http://" (28 bytes)
compressed (4 bytes):
[67, 67, 67, 67]
```
"http://" is the only opcode above 5 characters, and repeating it garners us 86% compression, the *best* case scenario for smaz.
Obviously you'd never have a real-world scenario where you just repeat "http://" over and over, but that doesn't mean it's not neat.
