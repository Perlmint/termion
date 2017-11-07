extern crate kernel32;
extern crate winapi;

use std::{io, mem};

use self::kernel32::GetConsoleScreenBufferInfo;
use self::winapi::TRUE;
use super::tty::{get_std_handle, StdStream};

use super::libc::{c_ushort};

#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    _x: c_ushort,
    _y: c_ushort,
}

pub const TIOCGWINSZ: usize = 0x40087468;

fn tiocgwinsz() -> i32 {
    TIOCGWINSZ as i32
}

/// Get the size of the terminal.
pub fn terminal_size() -> io::Result<(u16, u16)> {

    // Should this use "get_tty()"? As it is, it mirrors the
    // unix version in useing stdout.

    let stdout_handle = get_std_handle(StdStream::OUT)?;

    unsafe {
        let mut csbi = mem::zeroed();
        if GetConsoleScreenBufferInfo(stdout_handle, &mut csbi) == TRUE {
            Ok(((csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16,
                (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Unable to get the terminal size."))
        }
    }
}