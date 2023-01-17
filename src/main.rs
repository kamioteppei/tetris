mod block;
mod canvas;
mod tetris;

use console::{Key, Term};
use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};
use tetris::{Config, ConsoleGame, EventType, Tetris};

// キー入力監視
fn listen(press_keys: &Mutex<Vec<Key>>) -> ! {
    let term = Term::stdout();
    loop {
        let key = match term.read_key() {
            Ok(key) => key,
            Err(_) => {
                continue;
            }
        };
        let mut press_keys = press_keys.lock().unwrap();
        press_keys.push(key);
    }
}

// 状態更新と画面更新
fn run(game: &mut impl ConsoleGame, press_keys: &Mutex<Vec<Key>>) {
    game.init();
    'game: loop {
        // 入力キーからイベントタイプを取得し、排他制御変数を早期に解放
        let mut press_keys = press_keys.lock().unwrap();
        if press_keys.is_empty() {
            let event_type = EventType::BlockMoveDown;
            let is_continue = game.update(&event_type);
            if !is_continue {
                break 'game;
            }
            game.draw();
        } else {
            while let Some(press_key) = press_keys.pop() {
                let event_type = game.get_event_type(&press_key);
                let is_continue = game.update(&event_type);
                if !is_continue {
                    break 'game;
                }
                game.draw();
            }
            *press_keys = Vec::new();
        }
        drop(press_keys);

        let duraltion = Duration::from_millis(1000) + Duration::from_millis(500);
        sleep(duraltion);
    }
    println!("Game Over!");
}

// TODO: 「メモリを共有することでやり取りするな; 代わりにやり取りすることでメモリを共有しろ!」
// TODO: 画面領域からはみ出たブロックの位置調整
// TODO: ポイント対象行の削除
// TOOD: 点数の加算
// TOOD: ブロック落下スピード調整

fn main() {
    // コンソールクリア
    println!("\x1B[2J");

    let press_keys: Arc<Mutex<Vec<Key>>> = Arc::new(Mutex::new(Vec::new()));
    let press_keys_linster = Arc::clone(&press_keys);
    /* let handle = */
    spawn(move || listen(&press_keys_linster));

    let config = Config {
        width: 10,
        height: 15,
        speed: 0.1,
    };

    let mut game = Tetris::new(config);
    run(&mut game, &press_keys);

    // メインスレッド終了時にサブスレッドも終了させるので下行コメントアウト
    // handle.join().unwrap();
}
