use std::io;

pub use libc::termios as Termios;

pub mod attr {
    use std::{
        io, mem,
        os::fd::{AsRawFd, BorrowedFd},
    };

    use super::{cvt, Termios};

    pub fn get_terminal_attr(fd: BorrowedFd) -> io::Result<Termios> {
        unsafe {
            let mut termios = mem::zeroed();
            cvt(libc::tcgetattr(fd.as_raw_fd(), &mut termios))?;
            Ok(termios)
        }
    }

    pub fn set_terminal_attr(fd: BorrowedFd, termios: &Termios) -> io::Result<()> {
        cvt(unsafe { libc::tcsetattr(fd.as_raw_fd(), libc::TCSANOW, termios) }).and(Ok(()))
    }

    pub fn raw_terminal_attr(termios: &mut Termios) {
        unsafe { libc::cfmakeraw(termios) }
    }
}

// Support functions for converting libc return values to io errors {
trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
        ($($t:ident)*) => ($(impl IsMinusOne for $t {
            fn is_minus_one(&self) -> bool {
                *self == -1
            }
        })*)
    }

impl_is_minus_one! { i8 i16 i32 i64 isize }

fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
