use crate::domain::tetris::EventType;

#[derive(Clone)]
pub enum TetrisMessage {
    EventQueue(EventType),
}
