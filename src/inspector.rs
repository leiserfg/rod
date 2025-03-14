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
    let mut buff = String::new();

    let mut to_read = 8usize;
    buff.reserve(to_read);

    for b in tty_raw.keys() {
        if let Key::Char(c) = b.expect("Error reading") {
            if c == 'c' {
                break;
            }
            if to_read > 0 {
                buff.push(c);
                to_read -= 1;
            }
        }
    }

    match buff.as_str() {
        "997;1n62" => Some(Background::Dark),
        "997;2n62" => Some(Background::Light),
        _ => None,
    }
}
