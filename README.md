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



<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
