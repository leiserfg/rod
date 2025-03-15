use std::{borrow::Borrow, io::Read, io::Write};

#[derive(Debug, PartialEq)]
pub(crate) enum Background {
    Dark,
    Light,
}
use termion::{get_tty, raw::IntoRawMode};

pub(crate) fn probe() -> Option<Background> {
    let mut tty = get_tty().expect("We need a tty to do the check");
    let _ = tty.write(b"\x1b[?996n\x1b[c").expect("What happened");
    let tty_raw = tty.borrow().into_raw_mode().expect("Can't use raw");

    // We store only the first 8 characters, as they will contain the response
    // to 996 if the terminal suports the protocol correctly, then we keep reading until c
    // that marks the end of the response to [c
    let mut to_read = 8;
    let mut buff: Vec<u8> = Vec::with_capacity(to_read);

    for b in tty_raw.bytes() {
        let c = b.expect("Can't read");
        if to_read > 0 {
            buff.push(c);
            to_read -= 1;
        }
        if c == b'c' {
            break;
        }
    }

    match buff.as_slice() {
        b"\x1b[?997;1" => Some(Background::Dark),
        b"\x1b[?997;2" => Some(Background::Light),
        _ => None,
    }
}
