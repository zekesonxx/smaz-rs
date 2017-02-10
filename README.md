# smez-rs
Rust bindings (in the form of a binary) to [smez](https://github.com/antirez/smaz), a small-text compression library.

As far as I'm aware, I've implemented all of this correctly. For the moment, it's just a binary that produces identical output to the `smaz_test.c` program included in the original code.

Q. Why didn't you just use C to mess around with this library?  
A. Because I can't write C.  
Q. Why didn't you just rewrite this into Rust? It's not that long.  
A. Because I can't read C either.

## Things that would need to be done to make it into a proper Rust library
* The C code will probably bug out on non-ASCII text. This needs to be tested, and if it does, it'll need to be accounted for in the Rust code.
* Right now it makes a buffer with inputsize*3 for the output buffer. The library will panic if it turns out to be too small.
  * This can be fixed by dynamically changing the size if it turns out not to be big enough, and recalling `smez_(de)compress`
  * As far as I'm aware this can't be fixed without reimplementing smez into Rust so the output buffer can be resized during (de)compression as needed.
* If someone else can help me, taking the sane route and, you know, reimplementing it in Rust instead of calling out to the C version.
