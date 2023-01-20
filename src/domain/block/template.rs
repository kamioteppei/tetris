use super::r#type::{Shapes, RGB};

#[derive(Clone)]
pub struct BlockTemplate {
    shapes: Shapes,
    color: RGB,
}

impl BlockTemplate {
    pub fn new(shapes: Shapes, color: RGB) -> Self {
        Self { shapes, color }
    }

    pub fn ref_shapes(&self) -> &Shapes {
        &self.shapes
    }

    pub fn ref_color(&self) -> &RGB {
        &self.color
    }
}
