use std::{io::Read, thread, time};

use termion::async_stdin;

mod conway;
use conway::GameOfLife;

fn main() {
    let mut stdin = async_stdin().bytes();
    let fps = 10.0;
    let ten_millis = time::Duration::from_secs_f32(1.0 / fps);

    let mut gol = GameOfLife::new();

    loop {
        let b = stdin.next();

        match b {
            Some(x) => match x {
                Ok(k) => match k {
                    b'q' => break,
                    b'a' => gol.shift_left(1),
                    b'd' => gol.shift_right(1),
                    b'w' => gol.shift_top(1),
                    b's' => gol.shift_bottom(1),
                    _ => {}
                },
                _ => {}
            },
            None => {}
        }

        // temporal manual next
        // V        if let Some(Ok(b'n')) = b {
        //         } else if let Some(Ok(b'q')) = b {
        //             break;
        //         }

        gol.process();
        gol.render();

        thread::sleep(ten_millis);
    }
}
