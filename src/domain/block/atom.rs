use super::r#type::Point;
use super::r#type::RGB;

#[derive(Clone)]
pub struct Atom {
    point: Point,
    bgcolor: RGB,
}

impl Atom {
    pub fn new(point: Point, bgcolor: RGB) -> Self {
        Self { point, bgcolor }
    }

    pub fn ref_point(&self) -> &Point {
        &self.point
    }

    pub fn ref_bgcolor(&self) -> &RGB {
        &self.bgcolor
    }
}
