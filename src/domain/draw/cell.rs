use crate::domain::block::r#type::RGB;

#[derive(Clone)]
pub struct Cell {
    pub is_block: bool,
    pub bgcolor: RGB,
}
