use std::io::{self, stdout, Write};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use termion::event::Key;
use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::terminal_size;

mod conway;
use conway::GameOfLife;

fn generate_game_thread(play: &Arc<Mutex<bool>>) -> Arc<Mutex<GameOfLife>> {
    let fps = 10.0;
    let gol = Arc::new(Mutex::new(GameOfLife::new()));
    let time_wait = time::Duration::from_secs_f32(1.0 / fps);
    let mut stdout = stdout().into_raw_mode().unwrap();

    let play = play.clone();
    let th_gol = Arc::clone(&gol);

    thread::spawn(move || loop {
        let (col, row) = terminal_size().unwrap();

        write!(
            stdout,
            "{clear}{goto}{col}x{row} Playing: {playing}\n\r",
            clear = termion::clear::All,
            goto = termion::cursor::Goto(1, 1),
            col = col,
            row = row,
            playing = *&play.lock().unwrap()
        )
        .unwrap();

        th_gol.lock().unwrap().render();
        thread::sleep(time_wait);

        if *play.lock().unwrap() {
            th_gol.lock().unwrap().process();
        }
    });

    gol
}

fn main() {
    let stdin = io::stdin();
    let play = Arc::new(Mutex::new(true));
    let mut _stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    let gol = generate_game_thread(&play);

    for c in stdin.events() {
        match c.unwrap() {
            Event::Key(k) => match k {
                Key::Char('q') => {
                    break;
                }
                Key::Char('s') => {
                    let p = *play.lock().unwrap();
                    *play.lock().unwrap() = !p;
                }
                Key::Up => gol.lock().unwrap().shift_top(1),
                Key::Down => gol.lock().unwrap().shift_bottom(1),
                Key::Left => gol.lock().unwrap().shift_left(1),
                Key::Right => gol.lock().unwrap().shift_right(1),
                _ => {}
            },
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, col, row) => {
                    gol.lock()
                        .unwrap()
                        .toggle_cell((row - 2) as usize, (col - 1) as usize);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
