use crate::domain::block::BLOCK_ATOMS_SIZE;
use crate::domain::block::BLOCK_DIRECTION_ITEMS_SIZE;

#[derive(Clone)]
pub struct BlockTemplate {
    pub shapes: [[(i32, i32); BLOCK_ATOMS_SIZE]; BLOCK_DIRECTION_ITEMS_SIZE],
    pub color: (i32, i32, i32),
}
