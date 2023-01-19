use crate::domain::block::block::BLOCK_ATOMS_SIZE;
use crate::domain::block::block::BLOCK_DIRECTION_ITEMS_SIZE;

#[derive(Clone)]
pub struct BlockTemplate {
    shapes: [[(i32, i32); BLOCK_ATOMS_SIZE]; BLOCK_DIRECTION_ITEMS_SIZE],
    color: (i32, i32, i32),
}

impl BlockTemplate {
    pub fn new(
        shapes: [[(i32, i32); BLOCK_ATOMS_SIZE]; BLOCK_DIRECTION_ITEMS_SIZE],
        color: (i32, i32, i32),
    ) -> Self {
        Self { shapes, color }
    }

    pub fn ref_shapes(&self) -> &[[(i32, i32); BLOCK_ATOMS_SIZE]; BLOCK_DIRECTION_ITEMS_SIZE] {
        &self.shapes
    }

    pub fn ref_color(&self) -> &(i32, i32, i32) {
        &self.color
    }
}
