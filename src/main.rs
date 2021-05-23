use std::io::{stdout, Read, Write};
use std::{thread, time};
use termion::{async_stdin, raw::IntoRawMode, terminal_size};

fn main() {
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let ten_millis = time::Duration::from_millis(100);

    loop {
        let (row, col) = terminal_size().unwrap();
        let b = stdin.next();

        write!(
            stdout,
            "{clear}{goto}{x}, {y}",
            clear = termion::clear::All,
            goto = termion::cursor::Goto(1, 1),
            x = row,
            y = col
        )
        .unwrap();

        if let Some(Ok(b'q')) = b {
            break;
        }

        stdout.flush().unwrap();
        thread::sleep(ten_millis);
    }
}
