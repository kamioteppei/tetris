use crate::domain::draw::container::DrawContainer;
use crate::domain::draw::table::DrawTable;
use crate::domain::message::monitor::MonitorMessage;
use crate::domain::tetris::TetrisStatus;

use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::sync::Arc;
use tokio::sync::Mutex;

// Monitorアクターの構造体
pub struct Monitor {
    container_list: Arc<Mutex<Vec<DrawContainer>>>,
}

impl Monitor {
    pub fn new() -> Self {
        Monitor {
            container_list: Arc::new(Mutex::new(vec![
                DrawContainer {
                    id: 0,
                    status: TetrisStatus { score: 0 },
                    draw_table: DrawTable::new(0, 0, (0, 0, 0)),
                };
                2
            ])),
        }
    }

    pub async fn handle_message(&mut self, msg: MonitorMessage) {
        match msg {
            MonitorMessage::Display(id, container) => {
                let mut container_list = self.container_list.lock().await;
                container_list[id] = container;

                // コンソールをクリアして最新の内容を表示
                stdout().execute(Clear(ClearType::All)).unwrap();
                stdout().execute(MoveTo(0, 0)).unwrap();
                self.draw(&container_list);
            }
        }
    }

    fn draw(&self, container_list: &Vec<DrawContainer>) {
        for (_, c) in container_list.iter().enumerate() {
            let draw_table = &c.draw_table;
            let status = &c.status;
            // 上段から回す
            for i in (0..draw_table.ref_height()).rev() {
                let mut buf: String = String::from(" ");
                // 左端から回す
                for j in 0..draw_table.ref_width() {
                    let cells = draw_table.ref_cells().as_ref();
                    let cell = cells
                        .unwrap()
                        .get(i as usize)
                        .unwrap()
                        .get(j as usize)
                        .unwrap();
                    buf += if cell.is_block { "■" } else { "□" };
                }
                println!("\x1B[{};1H{}", draw_table.ref_height() - i, buf);
            }
            println!("Score: {}", status.score);
            println!("Press keys (a,s,d,w for PLAYER1, j,k,l,i for PLAYER2, q to quit):\n");
        }
    }
}
