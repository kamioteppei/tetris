use super::table::DrawTable;
use crate::domain::tetris::TetrisStatus;

#[derive(Clone)]
pub struct DrawContainer {
    pub id: usize,
    pub status: TetrisStatus,
    pub draw_table: DrawTable,
}
