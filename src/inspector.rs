use std::{borrow::Borrow, io::Write};

#[derive(Debug, PartialEq)]
pub(crate) enum Background {
    Dark,
    Light,
}
use termion::{event::Key, get_tty, input::TermRead, raw::IntoRawMode};

pub(crate) fn probe() -> Option<Background> {
    let mut tty = get_tty().expect("We need a tty to do the check");
    let _ = tty.write(b"\x1b[?996n\x1b[c").expect("What happened");
    let tty_raw = tty.borrow().into_raw_mode().expect("Can't use raw");

    // We need to read char by char cause the message does not end in \n
    let mut state = 0 as char;
    let mut is_this_one = false;

    for b in tty_raw.keys() {
        if let Key::Char(c) = b.expect("Error reading") {
            if is_this_one {
                is_this_one = false;
                state = c;
            }
            if c == ';' && state == 0u8 as char {
                is_this_one = true;
            }
            if c == 'c' {
                break;
            }
        }
    }

    match state {
        '1' => Some(Background::Dark),
        '2' => Some(Background::Light),
        _ => None,
    }
}
