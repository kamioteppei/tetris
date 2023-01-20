const BLOCK_ATOMS_SIZE: usize = 4;
const BLOCK_DIRECTION_ITEMS_SIZE: usize = 4;

pub type RGB = (i32, i32, i32);
pub type Point = (i32, i32);
pub type Points = [Point; BLOCK_ATOMS_SIZE];
pub type Shapes = [Points; BLOCK_DIRECTION_ITEMS_SIZE];
