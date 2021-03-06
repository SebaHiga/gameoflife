use std::io::{self, stdout, Write};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use termion::event::Key;
use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

mod conway;
use conway::GameOfLife;

fn generate_game_thread(play: &Arc<Mutex<bool>>, fps: &Arc<Mutex<f32>>) -> Arc<Mutex<GameOfLife>> {
    let gol = Arc::new(Mutex::new(GameOfLife::new()));
    let mut stdout = stdout().into_raw_mode().unwrap();

    let fps = fps.clone();
    let play = play.clone();
    let th_gol = Arc::clone(&gol);

    thread::spawn(move || loop {
        let time_wait = time::Duration::from_secs_f32(1.0 / *fps.lock().unwrap());

        write!(
            stdout,
            "{goto}Press 'e' to enter edit mode, 'q' to exit, 'c' to clean, 'r' to randomize and 1 to 5 for speed. Use the mouse button! | Edit mode: {playing} \n\r",
            goto = termion::cursor::Goto(1, 1),
            playing = !*play.lock().unwrap()
        )
        .unwrap();

        write!(stdout, "{goto}", goto = termion::cursor::Goto(1, 2),).unwrap();

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
    let fps = Arc::new(Mutex::new(10.0));
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    write!(stdout, "{}", termion::clear::All).unwrap();

    let gol = generate_game_thread(&play, &fps);
    let mut prev_row = 0;
    let mut prev_col = 0;

    for c in stdin.events() {
        match c.unwrap() {
            Event::Key(k) => match k {
                Key::Char('q') => {
                    break;
                }
                Key::Char('e') => {
                    let p = *play.lock().unwrap();
                    *play.lock().unwrap() = !p;
                }
                Key::Char('c') => gol.lock().unwrap().clean(),
                Key::Char('r') => gol.lock().unwrap().randomize(),
                Key::Char('1') => *fps.lock().unwrap() = 10.0,
                Key::Char('2') => *fps.lock().unwrap() = 20.0,
                Key::Char('3') => *fps.lock().unwrap() = 30.0,
                Key::Char('4') => *fps.lock().unwrap() = 40.0,
                Key::Char('5') => *fps.lock().unwrap() = 50.0,
                Key::Char('0') => *fps.lock().unwrap() = 10000.0,
                Key::Up => gol.lock().unwrap().shift_top(1),
                Key::Down => gol.lock().unwrap().shift_bottom(1),
                Key::Left => gol.lock().unwrap().shift_left(1),
                Key::Right => gol.lock().unwrap().shift_right(1),
                _ => {}
            },
            Event::Mouse(me) => match me {
                MouseEvent::Press(button, col, row) => match button {
                    MouseButton::Left => {
                        if !*play.lock().unwrap() {
                            gol.lock()
                                .unwrap()
                                .toggle_cell((row - 2) as usize, (col - 1) as usize);
                        } else {
                            prev_row = row;
                            prev_col = col;
                        }
                    }
                    _ => {}
                },
                MouseEvent::Hold(col, row) => {
                    if *play.lock().unwrap() {
                        let diff_col = col as i16 - prev_col as i16;
                        let diff_row = row as i16 - prev_row as i16;

                        prev_col = col;
                        prev_row = row;

                        gol.lock().unwrap().shift_vertical(diff_row);
                        gol.lock().unwrap().shift_horizontal(diff_col);
                    } else {
                        gol.lock()
                            .unwrap()
                            .toggle_cell((row - 2) as usize, (col - 1) as usize);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
