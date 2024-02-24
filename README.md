<!-- PROJECT NAME -->
# Fiter

A simple char-by-char iterator for files in rust.  
For that it converts an iterator over bytes into an iterator of UTF-8 decoded chars.  
The UTF-8 decoder is implemented as specified in [RFC 3629](https://datatracker.ietf.org/doc/html/rfc3629).



<!-- USAGE EXAMPLES -->
## Usage

The following example uses a buffered `Fiter` to print the license char-by-char to stdout:
```rust
use fiter::*;

fn main() {
  let mut fiter = Fiter::new_buffered("LICENSE").unwrap();

  for ch in fiter {
    print!("{}", ch.unwrap());
  }
}
```

For an unbuffered iterator `Fiter::new_unbuffered(...)` can be used, but it is much slower and should therefore not be used.



<!-- BENCHMARK -->
## Benchmark

The benches are not very detailed but should give an rough overview.  
The source code for the benches are in the module `benches` at the bottom of the `src/lib.rs` file.  
Both input files have around 1MB each.  
Results come from a MacBook Pro (M1 Pro).  

| input       | std      | [utf8-chars](github.com/A1-Triard/utf8-chars) | fiter 1k | fiter 100k |
|-------------|----------|--------------|---------|-----------|
| large_ascii | 31.68ms  | 48.88ms      | 34.48ms | 25.13ms   |
| large_utf8  | 16.70ms  | 40.27ms      | 22.56ms | 15.72ms   |

So the `Fiter` implementation, with a buffer of 1k bytes, is slightly slower than the implementation that only uses functions from the std library.
But the problem of the std implementation is, that it allocates as much memory as the file is big, where instead the `Fiter` implementation uses 1k bytes at most.
This doesn't matter for small files, but for big files it can make a huge different.



<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
