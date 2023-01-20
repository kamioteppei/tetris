use crate::domain::draw::draw_info::DrawInfo;
use crate::domain::{
    block::{block::Block, helper::collision::CollisionHelper, stack::BlockStack},
    contract::{Config, IConsoleGame, IDrawer, Status, TetrisError},
};
use crate::service::block_service::BlockService;
use console::Key;

#[derive(Clone, PartialEq)]
pub enum EventType {
    BlockMoveLeft,
    BlockMoveRight,
    BlockMoveDown,
    BlockRotate,
    None,
}

pub struct Tetris {
    config: Config,
    status: Status,
    float_block: Option<Block>,
    block_stack: BlockStack,
    block_service: BlockService,
    draw_info: DrawInfo,
}

impl Tetris {
    pub fn new(config: Config) -> Self {
        let width = config.width; // プリミティブ型の値は代入時に自動で複製されるから所有権も排他
        let height = config.height; // プリミティブ型以外はcloneでコピー作成するか参照を渡すか
        let status = Status {
            score: 0,
            update_duraltion_in_millis: 1000,
        };
        Self {
            config,
            status,
            float_block: None,
            block_stack: BlockStack::new(config.clone()),
            block_service: BlockService::new(),
            draw_info: DrawInfo::new(width, height, (0, 0, 0)),
        }
    }

    fn get_event_type(&self, press_key: &Option<Key>) -> EventType {
        let press_key = match press_key {
            Some(key) => key,
            None => return EventType::None,
        };
        let event_type: EventType = match *press_key {
            Key::ArrowUp => EventType::BlockRotate,
            Key::ArrowLeft => EventType::BlockMoveLeft,
            Key::ArrowRight => EventType::BlockMoveRight,
            Key::ArrowDown => EventType::BlockMoveDown,
            _ => EventType::None,
        };
        event_type
    }

    fn update_draw_info(&mut self) {
        // 全ブロックの描画情報を描画用オブジェクトに編集
        let mut all_atoms = self.block_stack.ref_atoms().clone();
        if let Some(float_block) = self.float_block.clone() {
            all_atoms.append(&mut float_block.to_atoms());
        }
        self.draw_info.update(&all_atoms);
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

impl IConsoleGame for Tetris {
    fn init(&mut self) {
        self.draw_info.clear();
    }

    fn ref_config(&self) -> &Config {
        &self.config
    }

    fn ref_status(&self) -> &Status {
        &self.status
    }

    fn ref_draw_info(&self) -> &DrawInfo {
        &self.draw_info
    }

    fn update(&mut self, press_key: &Option<Key>) -> Result<(), TetrisError> {
        // 積載ブロックが最大行を超えたらゲーム終了
        if self.block_stack.is_stack_overflow() {
            return Err(TetrisError::StackOverFlowError);
        }
        // 埋まったブロック行削除
        let delete_line_count = self.block_stack.compress();
        self.update_score(delete_line_count);

        // イベント種別判定
        let event_type = self.get_event_type(press_key);
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
        self.update_draw_info();

        Ok(())
    }

    fn draw(&self, drawer: &impl IDrawer) {
        drawer.draw(&self.draw_info, &self.status);
    }
}
