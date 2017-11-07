extern crate kernel32;
extern crate winapi;
use std::{io, mem};

use std::os::windows::prelude::*;
use self::kernel32::{GetStdHandle, ReadConsoleInputA};

use self::winapi::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use self::winapi::{HANDLE, INVALID_HANDLE_VALUE, KEY_EVENT, KEY_EVENT_RECORD, INPUT_RECORD, TRUE};

#[derive(Copy, Clone)]
pub enum StdStream {
    IN,
    OUT,
}

pub fn get_std_handle(strm: StdStream) -> io::Result<HANDLE> {
    let which_handle = match strm {
        StdStream::IN => STD_INPUT_HANDLE,
        StdStream::OUT => STD_OUTPUT_HANDLE,
    };

    unsafe {
        match GetStdHandle(which_handle) {
            x if x != INVALID_HANDLE_VALUE => Ok(x),
            _ => Err(io::Error::last_os_error()),
        }
    }
}

/// Is this stream a TTY?
pub fn is_tty<T: AsRawHandle>(_: &T) -> bool {
    true
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<Box<WindowsConsoleWrapper>> {
    Ok(Box::new(WindowsConsoleWrapper::new()))
}


pub struct WindowsConsoleWrapper {
    in_handle: HANDLE,
    out_handle: HANDLE,
}

pub struct WindowsConsoleBytesIterator {
    inner: WindowsConsoleWrapper
}

impl WindowsConsoleWrapper {
    pub fn new() -> WindowsConsoleWrapper {
        unsafe {
            WindowsConsoleWrapper {
                in_handle: GetStdHandle(STD_INPUT_HANDLE),
                out_handle: GetStdHandle(STD_OUTPUT_HANDLE)
            }
        }
    }

    pub fn bytes(self) -> WindowsConsoleBytesIterator {
        WindowsConsoleBytesIterator {
            inner: self
        }
    }
}

impl Iterator for WindowsConsoleBytesIterator {
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<io::Result<u8>> {
        unsafe {
            let mut buffer = mem::zeroed();
            let mut read_length = mem::zeroed();
            ReadConsoleInputA(self.inner.in_handle, &mut buffer, 1, &mut read_length);

            if buffer.EventType == KEY_EVENT {
                let key_event = mem::transmute_copy::<INPUT_RECORD, KEY_EVENT_RECORD>(&buffer);
                if key_event.bKeyDown == TRUE {
                    Option::from(Ok(key_event.wVirtualKeyCode as u8))
                } else {
                    Option::default()
                }
            } else {
                Option::default()
            }
        }
    }
}