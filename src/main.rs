mod domain;
mod presentation;
mod repository;
mod service;
mod usecase;

use domain::contract::Config;
use usecase::play_console::play_console;

// 【保留】「メモリを共有することでやり取りするな; 代わりにやり取りすることでメモリを共有しろ!」
// 【保留】 ブロック落下スピード調整
// TODO: 画面領域からはみ出たブロックの位置調整

fn main() {
    // ゲーム設定
    let config = Config {
        width: 10,
        height: 20,
        score_one_line: 100,
        score_multiple_line_weight: 2,
    };

    // コンソールUIで実行(2D版ユースケース等の拡張を想定した構成)
    play_console(config);
}
