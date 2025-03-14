use std::io::Write;

#[derive(Debug, PartialEq)]
pub(crate) enum Background {
    Dark,
    Light,
}

pub(crate) fn probe() -> Option<Background> {
    let mut tty = std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/tty")
        .expect("We need a tty to do the check");
    let _ = tty.write(b"\x1b[?996n\x1b[c").expect("What happened");

    let term = console::Term::stdout();

    // We need to read char by char cause the message does not end in \n
    let mut state = 0 as char;
    loop {
        let c = term.read_char().expect("didn't read");
        if c == ';' && state == (0 as char) {
            state = term.read_char().expect("didn't read");
        }
        if c == 'c' {
            break;
        }
    }

    match state {
        '1' => Some(Background::Dark),
        '2' => Some(Background::Light),
        _ => None,
    }
}
