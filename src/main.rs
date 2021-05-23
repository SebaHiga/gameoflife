use std::{io::Read, thread, time};

use termion::async_stdin;

mod conway;
use conway::{Cell, GameOfLife, Status};

fn main() {
    let mut stdin = async_stdin().bytes();
    let fps = 10.0;
    let ten_millis = time::Duration::from_secs_f32(1.0 / fps);
    let mut matrix = Vec::new();

    for _ in 0..256 {
        let mut cells = Vec::new();

        for _ in 0..256 {
            cells.push(Cell {
                content: '█',
                status: Status::Dead,
            });
        }

        matrix.push(cells);
    }

    matrix[2][10].set_status(Status::Alive);
    matrix[3][11].set_status(Status::Alive);
    matrix[4][9].set_status(Status::Alive);
    matrix[4][10].set_status(Status::Alive);
    matrix[4][11].set_status(Status::Alive);

    let mut gol = GameOfLife { matrix: matrix };

    loop {
        let b = stdin.next();

        // temporal manual next
        if let Some(Ok(b'n')) = b {
            gol.process();
        } else if let Some(Ok(b'q')) = b {
            break;
        }

        gol.render();

        thread::sleep(ten_millis);
    }
}
