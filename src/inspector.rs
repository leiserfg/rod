use std::{fs::File, io::Read};

#[derive(Debug, PartialEq)]
pub(crate) enum Background {
    Dark,
    Light,
}
use termion::{get_tty, raw::IntoRawMode};

#[cfg(target_os = "linux")]
fn is_linux_console(tty: &File) -> bool {
    use std::os::fd::AsRawFd;

    let mode = 0usize; // We won't use the mode but we still need to pass it to ioctl

    #[cfg(not(target_env = "musl"))]
    const KDGETMODE: u64 = 0x4B3B;

    #[cfg(target_env = "musl")]
    const KDGETMODE: i32 = 0x4B3B;

    unsafe { libc::ioctl(tty.as_raw_fd(), KDGETMODE, &mode) == 0 }
}

#[cfg(not(target_os = "linux"))]
fn is_linux_console(_tty: &File) -> bool {
    false
}

fn is_ghostty<Tty>(tty: &mut Tty) -> bool
where
    Tty: IntoRawMode + Read,
{
    let _ = tty.write(b"\x1b[>0q\x1b[c").expect("Din't allow the write");

    let ghostty_fingerprint = b"\x1bP>|ghostty";

    let mut buff: Vec<u8> = Vec::with_capacity(ghostty_fingerprint.len());

    {
        let mut tty_raw = tty.into_raw_mode().expect("Can't use raw");

        for b in tty_raw.bytes() {
            let c = b.expect("Can't read");
            if c == b'[' {
                break;
            }
            if buff.len() < ghostty_fingerprint.len() {
                buff.push(c);
            }
        }
        for b in tty_raw.bytes() {
            let c = b.expect("Can't read");
            if c == b'c' {
                break;
            }
        }
    };
    buff.as_slice() == ghostty_fingerprint
}

fn get_bg_ghostty<Tty>(tty: &mut Tty) -> Option<Background>
where
    Tty: IntoRawMode + Read,
{
    let _ = tty.write(b"\x1b[?996n").expect("What happened");

    // Similar to the case when using the CSI [c guard, but here we know we are in ghostty that is
    // able to report dark/light but has an issue with the order ofthe csi messages
    // So we can workarounding by sending only one message and reading til `n`
    let mut to_read = 8;
    let mut buff: Vec<u8> = Vec::with_capacity(to_read);

    {
        let mut tty_raw = tty.into_raw_mode().expect("Can't use raw");
        for b in tty_raw.bytes() {
            let c = b.expect("Can't read");
            if to_read > 0 {
                buff.push(c);
                to_read -= 1;
            }
            if c == b'n' {
                break;
            }
        }
    }

    match buff.as_slice() {
        b"\x1b[?997;1" => Some(Background::Dark),
        b"\x1b[?997;2" => Some(Background::Light),
        _ => None,
    }
}
fn get_bg_using_guard<Tty>(tty: &mut Tty) -> Option<Background>
where
    Tty: IntoRawMode + Read,
{
    let _ = tty.write(b"\x1b[?996n\x1b[c").expect("What happened");

    // We store only the first 8 characters, as they will contain the response
    // to 996 if the terminal suports the protocol correctly, then we keep reading until c
    // that marks the end of the response to [c
    let mut to_read = 8;
    let mut buff: Vec<u8> = Vec::with_capacity(to_read);

    {
        let mut tty_raw = tty.into_raw_mode().expect("Can't use raw");
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
    }

    match buff.as_slice() {
        b"\x1b[?997;1" => Some(Background::Dark),
        b"\x1b[?997;2" => Some(Background::Light),
        _ => None,
    }
}

pub(crate) fn probe() -> Option<Background> {
    let mut tty = get_tty().expect("Can't open tty, windows maybe?");

    if is_linux_console(&tty) {
        return None;
    }
    if is_ghostty(&mut tty) {
        return get_bg_ghostty(&mut tty);
    }
    get_bg_using_guard(&mut tty)
}
