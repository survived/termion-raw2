//! Managing raw mode.
//!
//! The code in this library is slightly modified version of `raw` module of [`termion`](https://docs.rs/termion)
//! crate. Difference is that termion only supports raw mode for stdout, while this  modification
//! supports any terminal that implements [`AsRawFd`].
//!
//! Raw mode is a particular state a TTY can have. It signifies that:
//!
//! 1. No line buffering (the input is given byte-by-byte).
//! 2. The input is not written out, instead it has to be done manually by the programmer.
//! 3. The output is not canonicalized (for example, `\n` means "go one line down", not "line
//!    break").
//!
//! It is essential to design terminal programs.
//!
//! ### Example
//!
//! ```rust,no_run
//! use termion_raw2::IntoRawMode;
//! use std::io::{Write, stdout};
//!
//! let mut stdout = stdout().into_raw_mode()?;
//! write!(stdout, "Hey there.").unwrap();
//! # std::io::Result::Ok(())
//! ```

use std::{
    io::{self, Write},
    ops,
    os::fd::AsRawFd,
};

use sys::attr::{get_terminal_attr, raw_terminal_attr, set_terminal_attr};
use sys::Termios;

mod sys;

/// A terminal restorer, which keeps the previous state of the terminal, and restores it, when
/// dropped.
///
/// Restoring will entirely bring back the old TTY state.
pub struct RawTerminal<W: Write + AsRawFd> {
    prev_ios: Termios,
    output: W,
}

impl<W: Write + AsRawFd> Drop for RawTerminal<W> {
    fn drop(&mut self) {
        let _ = set_terminal_attr(self.output.as_raw_fd(), &self.prev_ios);
    }
}

impl<W: Write + AsRawFd> ops::Deref for RawTerminal<W> {
    type Target = W;

    fn deref(&self) -> &W {
        &self.output
    }
}

impl<W: Write + AsRawFd> ops::DerefMut for RawTerminal<W> {
    fn deref_mut(&mut self) -> &mut W {
        &mut self.output
    }
}

impl<W: Write + AsRawFd> Write for RawTerminal<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

#[cfg(unix)]
mod unix_impl {
    use super::*;
    use std::os::unix::io::{AsRawFd, RawFd};

    impl<W: Write + AsRawFd> AsRawFd for RawTerminal<W> {
        fn as_raw_fd(&self) -> RawFd {
            self.output.as_raw_fd()
        }
    }
}

/// Types which can be converted into "raw mode".
///
/// # Why is this type defined on writers and not readers?
///
/// TTYs has their state controlled by the writer, not the reader. You use the writer to clear the
/// screen, move the cursor and so on, so naturally you use the writer to change the mode as well.
pub trait IntoRawMode: Write + AsRawFd + Sized {
    /// Switch to raw mode.
    ///
    /// Raw mode means that stdin won't be printed (it will instead have to be written manually by
    /// the program). Furthermore, the input isn't canonicalised or buffered (that is, you can
    /// read from stdin one byte of a time). The output is neither modified in any way.
    fn into_raw_mode(self) -> io::Result<RawTerminal<Self>>;
}

impl<W: Write + AsRawFd> IntoRawMode for W {
    fn into_raw_mode(self) -> io::Result<RawTerminal<W>> {
        let mut ios = get_terminal_attr(self.as_raw_fd())?;
        let prev_ios = ios;

        raw_terminal_attr(&mut ios);

        set_terminal_attr(self.as_raw_fd(), &ios)?;

        Ok(RawTerminal {
            prev_ios,
            output: self,
        })
    }
}

impl<W: Write + AsRawFd> RawTerminal<W> {
    /// Temporarily switch to original mode
    pub fn suspend_raw_mode(&self) -> io::Result<()> {
        set_terminal_attr(self.as_raw_fd(), &self.prev_ios)?;
        Ok(())
    }

    /// Temporarily switch to raw mode
    pub fn activate_raw_mode(&self) -> io::Result<()> {
        let mut ios = get_terminal_attr(self.as_raw_fd())?;
        raw_terminal_attr(&mut ios);
        set_terminal_attr(self.as_raw_fd(), &ios)?;
        Ok(())
    }
}
