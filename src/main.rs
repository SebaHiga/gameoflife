use std::io::{stdin, stdout};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod conway;
use conway::GameOfLife;

fn generate_game_thread(play: &Arc<Mutex<bool>>) -> Arc<Mutex<GameOfLife>> {
    let fps = 10.0;
    let gol = Arc::new(Mutex::new(GameOfLife::new()));
    let time_wait = time::Duration::from_secs_f32(1.0 / fps);

    let play = play.clone();
    let th_gol = Arc::clone(&gol);

    thread::spawn(move || loop {
        th_gol.lock().unwrap().render();
        thread::sleep(time_wait);

        if *play.lock().unwrap() {
            th_gol.lock().unwrap().process();
        }
    });

    gol
}

fn main() {
    let stdin = stdin();
    let play = Arc::new(Mutex::new(true));
    let mut _stdout = stdout().into_raw_mode().unwrap();

    let gol = generate_game_thread(&play);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                break;
            }
            Key::Char('s') => {
                let p = *play.lock().unwrap();
                *play.lock().unwrap() = !p;
            }
            Key::Up => {
                gol.lock().unwrap().shift_top(1);
            }
            Key::Down => {
                gol.lock().unwrap().shift_bottom(1);
            }
            Key::Left => {
                gol.lock().unwrap().shift_left(1);
            }
            Key::Right => {
                gol.lock().unwrap().shift_right(1);
            }
            _ => println!("Other"),
        }
    }
}
