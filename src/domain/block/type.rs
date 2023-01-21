use crate::domain::block::template::BlockTemplate;

const BLOCK_ATOMS_SIZE: usize = 4;
const BLOCK_DIRECTION_ITEMS_SIZE: usize = 4;
pub const BLOCK_TEMPLATE_ITEMS_SIZE: usize = 7;

pub type RGB = (i32, i32, i32);
pub type Point = (i32, i32);
pub type Points = [Point; BLOCK_ATOMS_SIZE];
pub type Shapes = [Points; BLOCK_DIRECTION_ITEMS_SIZE];
pub type BlockTemplates = [BlockTemplate; BLOCK_TEMPLATE_ITEMS_SIZE];
