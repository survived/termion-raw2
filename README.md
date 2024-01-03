<!-- cargo-rdme start -->

Managing raw mode.

The code in this library is slightly modified version of `raw` module of [`termion`](https://docs.rs/termion)
crate. Difference is that termion only supports raw mode for stdout, while this  modification
supports any terminal that implements [`AsFd`].

Raw mode is a particular state a TTY can have. It signifies that:

1. No line buffering (the input is given byte-by-byte).
2. The input is not written out, instead it has to be done manually by the programmer.
3. The output is not canonicalized (for example, `\n` means "go one line down", not "line
   break").

It is essential to design terminal programs.

### Example

```rust
use termion_raw2::IntoRawMode;
use std::io::{Write, stdout};

let mut stdout = stdout().into_raw_mode()?;
write!(stdout, "Hey there.").unwrap();
```

<!-- cargo-rdme end -->
