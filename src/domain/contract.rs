use console::Key;

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
pub trait ConsoleGame {
    fn init(&mut self);
    fn update(&mut self, press_key: &Option<Key>) -> Status;
    fn draw(&self);
    fn get_status(&self) -> Status;
}
