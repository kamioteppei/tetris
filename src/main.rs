mod domain;
mod presentation;
mod repository;
mod service;
mod usecase;

use domain::tetris::{TetrisConfig, TetrisEventType};
use domain::{message::tetris::TetrisMessage, tetris::Tetris};
use presentation::monitor::Monitor;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{self};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> io::Result<()> {
    // ゲーム設定
    let config = TetrisConfig {
        width: 10,
        height: 20,
        score_one_line: 100,
        score_multiple_line_weight: 2,
        initial_duration: 1500,
    };

    // チャネルの作成
    let (monitor_tx, mut monitor_rx) = mpsc::channel(32);
    let (tetris1_tx, mut tetris1_rx) = mpsc::channel(32);
    let (tetris2_tx, mut tetris2_rx) = mpsc::channel(32);

    // broadcast チャネルの作成
    let (timer_tx, _) = broadcast::channel(16);
    let timer_rx1 = timer_tx.subscribe();
    let timer_rx2 = timer_tx.subscribe();

    // Monitorアクターの起動
    let mut monitor = Monitor::new();
    tokio::spawn(async move {
        while let Some(msg) = monitor_rx.recv().await {
            monitor.handle_message(msg).await;
        }
    });

    // Tetrisアクターの起動（PLAYER1）
    let monitor_tx_clone = monitor_tx.clone();
    let mut timer_rx1 = timer_rx1;
    tokio::spawn(async move {
        let mut tetris1 = Tetris::new(0, config, monitor_tx_clone);
        tetris1.init();
        loop {
            tokio::select! {
                Some(msg) = tetris1_rx.recv() => {
                    tetris1.handle_message(msg).await;
                }
                Ok(msg) = timer_rx1.recv() => {
                    tetris1.handle_message(msg).await;
                }
            }
        }
    });

    // Tetrisアクターの起動（PLAYER2）
    let monitor_tx_clone = monitor_tx.clone();
    let mut timer_rx2 = timer_rx2;
    tokio::spawn(async move {
        let mut tetris2 = Tetris::new(1, config, monitor_tx_clone);
        tetris2.init();
        loop {
            tokio::select! {
                Some(msg) = tetris2_rx.recv() => {
                    tetris2.handle_message(msg).await;
                }
                Ok(msg) = timer_rx2.recv() => {
                    tetris2.handle_message(msg).await;
                }
            }
        }
    });

    // 定期的な更新処理の送信（broadcastを使用）
    let timer_tx_clone = timer_tx.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(config.initial_duration)).await;
            let _ = timer_tx_clone.send(TetrisMessage::EventQueue(TetrisEventType::None));
        }
    });

    // ターミナルをRawモードに設定
    enable_raw_mode()?;

    loop {
        // キー入力の待機
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                // キー押下時のみ処理
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            match c {
                                'a' | 's' | 'd' | 'w' => {
                                    tetris1_tx
                                        .send(TetrisMessage::EventQueue(char_to_event_type(c)))
                                        .await
                                        .expect("Failed to send to tetris1");
                                }
                                'j' | 'k' | 'l' | 'i' => {
                                    tetris2_tx
                                        .send(TetrisMessage::EventQueue(char_to_event_type(c)))
                                        .await
                                        .expect("Failed to send to tetris2");
                                }
                                'q' => break, // qで終了
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // ターミナルの設定を元に戻す
    disable_raw_mode()?;
    Ok(())
}

fn char_to_event_type(c: char) -> TetrisEventType {
    let event_type: TetrisEventType = match c {
        'w' | 'i' => TetrisEventType::BlockRotate,
        'a' | 'j' => TetrisEventType::BlockMoveLeft,
        'd' | 'l' => TetrisEventType::BlockMoveRight,
        's' | 'k' => TetrisEventType::BlockMoveDown,
        _ => TetrisEventType::None,
    };
    event_type
}
