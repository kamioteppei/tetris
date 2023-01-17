use console::Key;

use super::block_template::BlockTemplate;

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
    fn update(&mut self, press_key: &Option<Key>) -> Status;
    fn draw(&self);
    fn get_status(&self) -> Status;
}

pub trait IBlockTemplateRepository {
    fn choose_random(&self) -> BlockTemplate;
}
