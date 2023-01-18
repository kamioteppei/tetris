mod domain;
mod presentation;
mod repository;
mod service;
mod usecase;

use domain::contract::Config;
use usecase::play_console::play_console;

// 【保留】「メモリを共有することでやり取りするな; 代わりにやり取りすることでメモリを共有しろ!」
// TODO: 画面領域からはみ出たブロックの位置調整
// TODO: ポイント対象行の削除
// TOOD: 点数の加算
// TOOD: ブロック落下スピード調整

fn main() {
    // ゲーム設定
    let config = Config {
        width: 10,
        height: 20,
    };

    // コンソールUIで実行(2D版ユースケース等の拡張を想定した構成)
    play_console(config);
}
