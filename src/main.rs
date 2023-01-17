mod domain;
mod presentation;
mod tetris;

use console::{Key, Term};
use domain::contract::*;
use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};
use tetris::Tetris;

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
fn run(game: &mut impl ConsoleGame, press_keys: &Mutex<Vec<Key>>) {
    game.init();
    'game: loop {
        let mut press_keys = press_keys.lock().unwrap();
        if press_keys.is_empty() {
            // 入力キーを取得できない場合の更新処理
            let status = game.update(&None);
            if !status.is_continue {
                break 'game;
            }
            game.draw();
        } else {
            // 入力キーを順次渡して更新処理
            while let Some(press_key) = press_keys.pop() {
                let status = game.update(&Some(press_key));
                if !status.is_continue {
                    break 'game;
                }
                game.draw();
            }
            *press_keys = Vec::new();
        }
        // 更新待機に入る前に、キー入力受付再開
        drop(press_keys);

        // 更新待機
        let dulation = game.get_status().update_duraltion_in_millis;
        sleep(Duration::from_millis(dulation));
    }
    println!("Game Over!");
}

// 【保留】「メモリを共有することでやり取りするな; 代わりにやり取りすることでメモリを共有しろ!」
// TODO: 画面領域からはみ出たブロックの位置調整
// TODO: ポイント対象行の削除
// TOOD: 点数の加算
// TOOD: ブロック落下スピード調整

fn main() {
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

    // ゲーム設定
    let config = Config {
        width: 10,
        height: 20,
    };

    // ゲームアプリ起動
    let mut game = Tetris::new(config);
    run(&mut game, &press_keys);
}
