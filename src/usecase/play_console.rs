use crate::domain::contract::Config;
use crate::domain::tetris::Tetris;
use crate::{domain::contract::IConsoleGame, presentation::drawer_console::DrawConsole};
use console::{Key, Term};
use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

// キー入力監視(Key型のキューで保持)
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
fn run(tetris: &mut impl IConsoleGame, press_keys: &Mutex<Vec<Key>>) {
    tetris.init();
    let drawer: DrawConsole = DrawConsole {};
    'tetris: loop {
        let mut press_keys = press_keys.lock().unwrap();
        if press_keys.is_empty() {
            // 入力キーを取得できない場合の更新処理
            let status = tetris.update(&None);
            if !status.is_continue {
                break 'tetris;
            }
            tetris.draw(&drawer);
        } else {
            // 入力キーを順次渡して更新処理
            while let Some(press_key) = press_keys.pop() {
                let status = tetris.update(&Some(press_key));
                if !status.is_continue {
                    break 'tetris;
                }
                tetris.draw(&drawer);
            }
            *press_keys = Vec::new();
        }
        // 更新待機に入る前に、キー入力受付再開
        drop(press_keys);

        // 更新待機
        let dulation = tetris.get_status().update_duraltion_in_millis;
        sleep(Duration::from_millis(dulation));
    }
}

pub fn play_console(config: Config) {
    // コンソールクリア
    println!("\x1B[2J");

    // 入力キー配列
    let press_keys: Arc<Mutex<Vec<Key>>> = Arc::new(Mutex::new(Vec::new()));

    // サブスレッドでキー入力監視
    {
        let press_keys_linster = Arc::clone(&press_keys);
        spawn(move || listen(&press_keys_linster));
        // メインスレッド終了時にサブスレッドも終了させるのでhandle.join().unwrap();しない
    }

    // テトリス起動
    let mut tetris = Tetris::new(config);
    run(&mut tetris, &press_keys);

    // ゲーム終了
    println!("Game Over!");
}
