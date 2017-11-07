extern crate kernel32;
extern crate winapi;
use std::{io};

use super::Termios;
use super::tty::{get_std_handle, StdStream};
use self::kernel32::{SetConsoleMode};
use self::winapi::{ENABLE_ECHO_INPUT, ENABLE_MOUSE_INPUT};

pub fn get_terminal_attr() -> io::Result<Termios> {
    Ok(0 as Termios)
}

pub fn set_terminal_attr(_: &Termios) -> io::Result<()> {
    Ok(())
}

pub fn raw_terminal_attr(_: &mut Termios) {
    let handle = get_std_handle(StdStream::IN);
    unsafe {
        SetConsoleMode(handle.unwrap(), ENABLE_ECHO_INPUT & ENABLE_MOUSE_INPUT);
    }
}
