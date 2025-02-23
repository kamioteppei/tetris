use crate::domain::tetris::DrawModelContainer;

pub enum MonitorMessage {
    Display(usize, DrawModelContainer),
}
