use crate::domain::{
    block::{block::Block, helper::collision::CollisionHelper, stack::BlockStack},
    draw::draw_model::DrawModel,
    message::monitor::MonitorMessage,
    message::tetris::TetrisMessage,
};
use crate::service::block_service::BlockService;
use tokio::sync::mpsc::Sender;

#[derive(Clone, PartialEq)]
pub enum EventType {
    BlockMoveLeft,
    BlockMoveRight,
    BlockMoveDown,
    BlockRotate,
    None,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub width: i32,
    pub height: i32,
    pub score_one_line: i32,
    pub score_multiple_line_weight: i32,
}

#[derive(Clone, Copy)]
pub struct Status {
    pub score: i32,
}

#[derive(Clone)]
pub struct DrawModelContainer {
    pub draw_model: DrawModel,
    pub status: Status,
}

pub enum TetrisError {
    StackOverFlowError,
}

pub struct Tetris {
    id: usize,
    config: Config,
    status: Status,
    float_block: Option<Block>,
    block_stack: BlockStack,
    block_service: BlockService,
    draw_model: DrawModel,
    monitor_tx: Sender<MonitorMessage>,
}

impl Tetris {
    pub fn new(id: usize, config: Config, monitor_tx: Sender<MonitorMessage>) -> Self {
        let width = config.width; // プリミティブ型の値は代入時に自動で複製されるから所有権も排他
        let height = config.height; // プリミティブ型以外はcloneでコピー作成するか参照を渡すか
        let status = Status { score: 0 };
        Self {
            id,
            config,
            status,
            float_block: None,
            block_stack: BlockStack::new(config.clone()),
            block_service: BlockService::new(),
            draw_model: DrawModel::new(width, height, (0, 0, 0)),
            monitor_tx,
        }
    }

    pub async fn handle_message(&mut self, msg: TetrisMessage) {
        match msg {
            TetrisMessage::EventQueue(event_type) => {
                if let Err(error) = self.update(event_type) {
                    match error {
                        TetrisError::StackOverFlowError => {}
                    }
                };

                // Monitorにログの内容を送信
                self.monitor_tx
                    .send(MonitorMessage::Display(
                        self.id,
                        DrawModelContainer {
                            draw_model: self.draw_model.clone(),
                            status: self.status.clone(),
                        },
                    ))
                    .await
                    .expect("Failed to send to monitor");
            }
        }
    }

    fn update(&mut self, event_type: EventType) -> Result<(), TetrisError> {
        // 積載ブロックが最大行を超えたらゲーム終了
        if self.block_stack.is_stack_overflow() {
            return Err(TetrisError::StackOverFlowError);
        }
        // 埋まったブロック行削除
        let delete_line_count = self.block_stack.compress();
        self.update_score(delete_line_count);

        // 浮遊ブロック操作
        let float_block: Block = match &self.float_block {
            Some(block) => {
                // 浮遊ブロックの操作
                let mut block = block.clone();
                match &event_type {
                    EventType::BlockRotate => self.block_rotate(&mut block),
                    EventType::BlockMoveLeft => self.block_move_left(&mut block),
                    EventType::BlockMoveRight => self.block_move_right(&mut block),
                    EventType::BlockMoveDown | EventType::None => block.move_down(),
                }
                block
            }
            // 新規ブロック作成(テンプレートからランダムに選択)
            None => {
                let start_pos_x = self.config.width / 2_i32 - 1;
                let start_pos_y = self.config.height - 1;
                let mut new_block = self.block_service.create_block();
                new_block.init(&(start_pos_x, start_pos_y));
                new_block
            }
        };

        // ブロックが下部のブロックや下面に接したら、浮遊ブロックをスタックに移動
        if CollisionHelper::is_on_stack_height(
            &self.config,
            &float_block,
            &self.block_stack.ref_atoms(),
        ) {
            self.block_stack.add_block(float_block.clone());
            self.float_block = None;
        } else {
            self.float_block = Some(float_block);
        };

        // 描画情報更新
        self.update_draw_model();

        Ok(())
    }

    fn update_draw_model(&mut self) {
        // 全ブロックの描画情報を描画用オブジェクトに編集
        let mut all_atoms = self.block_stack.ref_atoms().clone();
        if let Some(float_block) = self.float_block.clone() {
            all_atoms.append(&mut float_block.to_atoms());
        }
        self.draw_model.update(&all_atoms);
    }

    fn update_score(&mut self, delete_line_count: i32) {
        let score_one_line = self.config.score_one_line;
        let score_multiple_line_weight = self.config.score_multiple_line_weight;
        self.status.score +=
            score_one_line * delete_line_count.pow(score_multiple_line_weight as u32);
    }

    fn block_rotate(&self, block: &mut Block) {
        let mut clone = block.clone();
        clone.rotate();
        if CollisionHelper::validate_block(&self.config, &clone, &self.block_stack.ref_atoms()) {
            block.rotate();
        } else {
            block.move_down();
        }
    }

    fn block_move_left(&self, block: &mut Block) {
        let mut clone = block.clone();
        clone.move_left();
        if CollisionHelper::validate_block(&self.config, &clone, &self.block_stack.ref_atoms()) {
            block.move_left();
        } else {
            block.move_down();
        }
    }

    fn block_move_right(&self, block: &mut Block) {
        let mut clone = block.clone();
        clone.move_right();
        if CollisionHelper::validate_block(&self.config, &clone, &self.block_stack.ref_atoms()) {
            block.move_right();
        } else {
            block.move_down();
        }
    }
}
