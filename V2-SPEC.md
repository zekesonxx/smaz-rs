# smaz2: much small, very better

smaz2 is an **work-in-progress** idea/proposal/etc for improving the smaz small string compression protocol.
The new protocol would be almost entirely the same, but not backwards compatable.

## The opcode table changes
#### I, robot.
* Opcode `230`, currently `"><"`, would become `"I"`

smaz is largely designed for human-written, English text. Humans like referring to themselves. At the moment the letter I uses the literal character opcode and requires two bytes.

`<` and `>` both have opcodes, so it would only increase the compression of `><` by one byte.

#### URLs
* Opcode `67`, currently `"http://"`, would become `"://"`.
* Opcode `40`, currently `"   "` (3 spaces), would become `"http"`.

Compressing `http://` down to a single byte is certainly helpful, and back in 2008 when smez was written HTTPS wasn't very widespread in day-to-day use. In 2017, where HTTPS is prevalent everywhere, `https://` currently takes 9 bytes (which is an enlargement!). Under this change `http://` would take 2 bytes, and `https://` would take 3.

An alternative option would be to leave opcode `67` alone, and just change opcode `40` to `https://`. This would give identical compression numbers to HTTP and HTTPS URLs, but doesn't support other URLs.

Due to other, similar protocols using `://` (like `rtsp://` and `mumble://`), hardcoding to just http and https will reduce smaz2's versatility. However, the overwhelming prevalance of HTTP and HTTPS makes it worth keeping an `http` opcode.


## Implementation changes
#### More efficient looping
* Manually check for the three 5-byte opcodes (`there`, `their`, and `which`) before iterating over the whole table
* Only check strings up to length 4, instead of 7.

Existing smaz implementations are very inefficient in looping, since they waste three loops through the whole array just to check for four strings: `there`, `their`, `which`, and `http://`. `http://` is being removed by smaz2, and since there are only three 5-length strings it makes more sense to check for them manually then loop through the whole string.

Implementations may also want to manually check the (now 9) four-byte opcodes, to save yet another loop.

#### Minimizing uncompressable damage
* At the end of the loop, check the length and perform worst-case-scenario prevention if needed.

Worst-case-scenario prevention is done by checking the length of the compressed string. If it's longer than the worst-case-scenario length (`origlen+(2*(origlen/256))`), use the literal-string opcode to store the original text uncompressed.

This makes compression take slightly longer, but saves space and makes decompression much simpler.

**Example** The 12-byte string `e33e33e33e33` is enlarged to 20 bytes by the original smaz code. With worst-case-scenario prevention, it's only 14 bytes.

#### max_(de)compressed_size
* Where relevant (such as in a C implementation or interface), the functions `max_compressed_size` and `max_decompressed_size` should be available.

Calculating max compressed size is simple, as it's the worst-case-scenario length, `origlen+(2*(origlen/256))`.

To calculate max decompressed size, iterate over the compressed string, and for each byte:
* 0-253: add 5 to the counter
* 254: add 1 to the counter and skip forward a byte
* 255: add the length of the literal string to the counter, and skip forward by the length of the literal string.

For situations where speed is more important than memory efficiency, you can calculate a less accurate max decompressed size by simply multiplying the input length by 5 (if every opcode were a 5-byte opcode).
