use crate::domain::tetris::TetrisEventType;

#[derive(Clone)]
pub enum TetrisMessage {
    EventQueue(TetrisEventType),
}
