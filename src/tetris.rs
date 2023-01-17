use crate::domain::block::Block;
use crate::domain::block_template::BlockTemplates;
use crate::domain::contract::*;
use crate::presentation::canvas::Canvas;
use console::Key;
use std::mem;

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
    block_templates: BlockTemplates,
    stack_blocks: Vec<Block>,
    float_block: Option<Block>,
    canvas: Canvas,
}

impl Tetris {
    pub fn new(config: Config) -> Self {
        let width = config.width; // プリミティブ型の値は代入時に自動で複製されるから所有権も排他
        let height = config.height; // プリミティブ型以外はcloneでコピー作成するか参照を渡すか
        let status = Status {
            point: 0_i32,
            is_continue: true,
            update_duraltion_in_millis: 1000,
        };
        Self {
            config,
            status,
            block_templates: BlockTemplates::new(),
            stack_blocks: Vec::new(),
            float_block: None,
            canvas: Canvas::new(width, height, (0, 0, 0)),
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

    // 浮遊ブロックが積載ブロックに接地したか判定
    fn is_on_stack_line(&self, float_block: &Block, stack_blocks: &Vec<Block>) -> bool {
        // 各列の最大行数配列を生成（ブロックがない列は-1行目とみなす）
        let mut stack_line = vec![-1; self.config.width as usize];
        for block in stack_blocks {
            for (x, mut y) in block.points {
                if y > stack_line[x as usize] {
                    mem::swap(&mut stack_line[x as usize], &mut y);
                }
            }
        }
        // 浮遊ブロックが列の最大行より下にあるか判定
        for (x, y) in float_block.points {
            if y <= stack_line[x as usize] + 1 {
                return true;
            }
        }
        false
    }

    // 積載ブロックが最大行を超えたか判定
    fn is_stack_overflow(&self, stack_blocks: &Vec<Block>) -> bool {
        for block in stack_blocks {
            for (_, y) in block.points {
                if y >= self.config.height - 1 {
                    return true;
                }
            }
        }
        false
    }

    // 新規ブロック生成
    fn create_block(&self) -> Block {
        let random_template = self.block_templates.choose_random();
        let mut new_block = Block::new(random_template);
        let start_pos_x = self.config.width / 2_i32 - 1;
        let start_pos_y = self.config.height - 1;
        new_block.init(&(start_pos_x, start_pos_y));
        new_block
    }

    fn update_draw_info(&mut self) {
        // 全ブロックの描画情報を描画用オブジェクトに編集
        let mut all_blocks = self.stack_blocks.clone();
        if let Some(float_block) = self.float_block.clone() {
            all_blocks.push(float_block);
        }
        self.canvas.update(all_blocks);
    }
}

impl ConsoleGame for Tetris {
    fn init(&mut self) {
        self.canvas.init();
    }

    fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn update(&mut self, press_key: &Option<Key>) -> Status {
        // 積載ブロックが最大行を超えたらゲーム終了
        if self.is_stack_overflow(&self.stack_blocks) {
            return Status {
                is_continue: false,
                ..self.status
            };
        }

        let event_type = self.get_event_type(press_key);

        // 浮遊ブロック操作
        let float_block: Block = match &self.float_block {
            Some(block) => {
                // 操作中のブロックを移動
                let mut block = block.clone();
                match &event_type {
                    EventType::BlockRotate => block.rotate(),
                    EventType::BlockMoveLeft => block.move_left(),
                    EventType::BlockMoveRight => block.move_right(),
                    EventType::BlockMoveDown => block.move_down(),
                    EventType::None => block.move_down(),
                }
                block
            }
            // 操作中のブロックがなければテンプレートからランダムに選び新規ブロック作成
            None => self.create_block(),
        };

        // ブロックが下部のブロックや地面に接したら、ブロックをスタックに移動し次のブロックを投下
        if self.is_on_stack_line(&float_block, &self.stack_blocks) {
            self.stack_blocks.push(float_block.clone());
            self.float_block = None;
        } else {
            self.float_block = Some(float_block);
        };

        // 描画情報更新
        self.update_draw_info();

        Status {
            is_continue: true,
            ..self.status
        }
    }

    fn draw(&self) {
        // 上段から回す
        for i in (0..self.config.height).rev() {
            let mut buf: String = String::from(" ");
            // 左端から回す
            for j in 0..self.config.width {
                let cells = self.canvas.cells.as_ref();
                let cell = cells
                    .unwrap()
                    .get(i as usize)
                    .unwrap()
                    .get(j as usize)
                    .unwrap();
                buf += if cell.is_block { "■" } else { "□" };
            }
            println!("\x1B[{};1H{}", self.config.height - i, buf);
        }
    }
}
