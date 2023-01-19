use crate::domain::block_template::BlockTemplate;
use crate::domain::tetris::Tetris;
use crate::presentation::draw_info::DrawInfo;
use console::Key;

#[derive(Clone, Copy)]
pub struct Config {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone)]
pub struct Status {
    pub score: i32,
    pub update_duraltion_in_millis: u64,
}

pub enum TetrisError {
    StackOverFlowError,
}

pub trait IConsoleGame {
    fn init(&mut self);
    fn update(&mut self, press_key: &Option<Key>) -> Result<(), TetrisError>;
    fn draw(&self, drawer: &impl IDrawer);
    fn ref_config(&self) -> &Config;
    fn ref_status(&self) -> &Status;
    fn ref_draw_info(&self) -> &DrawInfo;
}

pub trait IDrawer {
    fn draw(&self, tetris: &Tetris);
}

pub trait IBlockTemplateRepository {
    fn choose_random(&self) -> BlockTemplate;
}
