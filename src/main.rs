mod block;
mod canvas;
mod tetris;

use console::{Key, Term};
use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::{Duration, SystemTime},
};
use tetris::ConsoleGame;
use tetris::{Config, Tetris};

// キー入力監視
fn listen(press_key: &Mutex<Key>) -> ! {
    let term = Term::stdout();
    loop {
        let key = match term.read_key() {
            Ok(key) => key,
            Err(_) => continue,
        };
        *press_key.lock().unwrap() = key;
    }
}

// 状態更新と画面更新
fn run(game: &mut impl ConsoleGame, press_key: &Mutex<Key>) {
    let mut time: SystemTime = SystemTime::now();
    game.init();
    loop {
        let is_continue = game.update(press_key);
        if !is_continue {
            break;
        }
        game.draw();
        time += Duration::from_millis(1000);
        if let Ok(dur) = time.duration_since(SystemTime::now()) {
            sleep(dur);
        }
    }
    println!("Game Over!");
}

fn main() {
    // コンソールクリア
    println!("\x1B[2J");

    let press_key: Arc<Mutex<Key>> = Arc::new(Mutex::new(Key::Unknown));
    {
        let press_key = press_key.clone();
        spawn(move || listen(&press_key));
    }

    let config = Config {
        width: 10,
        height: 15,
        speed: 0.1,
    };

    let mut game = Tetris::new(config);
    run(&mut game, &press_key);
}
