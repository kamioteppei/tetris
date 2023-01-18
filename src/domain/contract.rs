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
    pub point: i32,
    pub is_continue: bool,
    pub update_duraltion_in_millis: u64,
}
pub trait IConsoleGame {
    fn init(&mut self);
    fn update(&mut self, press_key: &Option<Key>) -> &Status;
    fn draw(&self, drawer: &impl IDrawer);
    fn get_config(&self) -> &Config;
    fn get_status(&self) -> &Status;
    fn get_draw_info(&self) -> &DrawInfo;
}

pub trait IDrawer {
    fn draw(&self, tetris: &Tetris);
}

pub trait IBlockTemplateRepository {
    fn choose_random(&self) -> BlockTemplate;
}
