use std::{
    io::{stdout, Read, Write},
    u16,
};

use std::{thread, time};
use termion::{
    async_stdin,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

use cell::{Cell, Status};
pub mod cell;

fn render(stdout: &mut RawTerminal<std::io::Stdout>, matrix: &Vec<Vec<Cell>>, row: u16, col: u16) {
    for index_row in 0..matrix.len() {
        for index_col in 0..matrix[index_row].len() {
            let cell = matrix[index_row][index_col].clone();

            if cell.get_status() == Status::Alive {
                write!(stdout, "{}", matrix[index_row][index_col].get_content()).unwrap();
            } else {
                write!(stdout, "{}", ' ').unwrap();
            }

            stdout.flush().unwrap();

            if index_col >= (row - 1) as usize {
                break;
            }
        }
        if index_row >= (col - 2) as usize {
            break;
        }
    }
}

fn main() {
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let fps = 12.0;
    let ten_millis = time::Duration::from_secs_f32(1.0 / fps);
    let mut matrix = Vec::new();

    for _ in 0..64 {
        let mut cells = Vec::new();

        for _ in 0..256 {
            cells.push(Cell {
                row: 0,
                col: 0,
                content: '█',
                status: Status::get_rand(),
            });
        }

        matrix.push(cells);
    }

    loop {
        let (row, col) = terminal_size().unwrap();
        let b = stdin.next();

        write!(
            stdout,
            "{clear}{goto}{x}, {y}\n\r",
            clear = termion::clear::All,
            goto = termion::cursor::Goto(1, 1),
            x = row,
            y = col
        )
        .unwrap();
        stdout.flush().unwrap();

        if let Some(Ok(b'q')) = b {
            break;
        }

        render(&mut stdout, &matrix, row, col);

        thread::sleep(ten_millis);
    }
}
