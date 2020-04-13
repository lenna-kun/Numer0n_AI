extern crate termion;

use std::io::{stdin, stdout, Write};

use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type Pair<'a, 'b> = (&'a mut usize, &'b str);

pub enum Error {
    KeyboardInterrupt,
    NotEnoughInput,
}

pub fn hide_cursor() {
    print!("{}", cursor::Hide);
}

pub fn read_pair(var1: Pair, var2: Pair) -> std::result::Result<(), Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut v1: usize = 0;
    let mut v2: usize = 0;

    write!(stdout, "\x1b[1m\x1b[{}G{}:   {}:  {}{}\x1b[91m",
        3, var1.1, var2.1, 
        cursor::Left((5+var2.1.len()) as u16), cursor::Show).unwrap();
    stdout.flush().unwrap();
    let mut input_cnt: i8 = 0;
    for evt in stdin.events() {
        match evt.unwrap() {
            Event::Key(Key::Ctrl('c')) => return Err(Error::KeyboardInterrupt),
            Event::Key(Key::Char('\n')) => {
                if input_cnt >= 2 { break; } else {
                    write!(stdout, "\r").unwrap();
                    stdout.flush().unwrap();
                    return Err(Error::NotEnoughInput);
                }
            },
            Event::Key(Key::Backspace) => {
                if input_cnt == 1 {
                    write!(stdout, "\x1b[{}G {}", 5 + var1.1.len(), cursor::Left(1)).unwrap();
                } else if input_cnt == 2 {
                    write!(stdout, "\x1b[{}G {}", 9 + var1.1.len() + var2.1.len(), cursor::Left(1)).unwrap();
                }
                input_cnt = std::cmp::max(input_cnt-1, 0);
                stdout.flush().unwrap();
            },
            Event::Key(Key::Char(c)) => {
                let num: usize = if c as u8 >= b'0' && c as u8 <= b'9' {
                    (c as u8 - b'0') as usize
                } else { continue };
                if input_cnt == 0 { 
                    v1 = num;
                    input_cnt += 1;
                    write!(stdout, "\x1b[{}G{}{}", 5 + var1.1.len(), v1, cursor::Right((3+var2.1.len()) as u16)).unwrap();
                } else if input_cnt == 1 {
                    v2 = num;
                    input_cnt += 1;
                    write!(stdout, "\x1b[{}G{}", 9 + var1.1.len() + var2.1.len(), v2).unwrap();
                }
                stdout.flush().unwrap();
            },
            _ => (),
        }
    }
    write!(stdout, "\x1b[0m\n\r").unwrap();
    stdout.flush().unwrap();
    *var1.0 = v1;
    *var2.0 = v2;
    Ok(())
}