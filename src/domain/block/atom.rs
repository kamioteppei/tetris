#[derive(Clone)]
pub struct Atom {
    point: (i32, i32),
    bgcolor: (i32, i32, i32),
}

impl Atom {
    pub fn new(point: (i32, i32), bgcolor: (i32, i32, i32)) -> Self {
        Self { point, bgcolor }
    }

    pub fn ref_point(&self) -> &(i32, i32) {
        &self.point
    }

    pub fn ref_bgcolor(&self) -> &(i32, i32, i32) {
        &self.bgcolor
    }
}
