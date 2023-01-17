use crate::domain::block_template::BlockTemplate;

pub const BLOCK_ATOMS_SIZE: usize = 4;
pub const BLOCK_DIRECTION_ITEMS_SIZE: usize = 4;

#[derive(Clone)]
pub enum BlockDirection {
    Top,
    Right,
    Down,
    Left,
}
#[derive(Clone)]
pub struct Block {
    pub template: BlockTemplate,
    pub direction: BlockDirection,
    pub points: [(i32, i32); BLOCK_ATOMS_SIZE],
}

impl Block {
    pub fn new(template: BlockTemplate) -> Self {
        let direction = BlockDirection::Top;
        /*
        directionの値を下記で使用する方法
        ・参照用の変数を使用する
            let direction_ref = &direction;
            let points = template.shapes.get(*direction_ref as usize); // 参照外し
        ・明示的にクローンする
          (メモリを消費するので大きな構造体のクローンは注意)
            let points = template.shapes.get(direction.clone() as usize);
        ・Copyトレイトでコピーする
            メモリ自体をコピーするだけなので、メンバの型によってはシャローコピーになるので注意
            #[derive(Clone, Copy)]
            pub enum BlockDirection {

            let points = template.shapes.get(direction as usize);
        */
        let points = template.shapes.get(direction.clone() as usize);
        Self {
            template: template.clone(),
            direction,
            points: points.unwrap().clone(),
        }
    }

    pub fn init(&mut self, start_pos: &(i32, i32)) {
        self.r#move(start_pos);
    }

    pub fn move_left(&mut self) {
        self.r#move(&(-1, 0));
    }

    pub fn move_right(&mut self) {
        self.r#move(&(1, 0));
    }

    pub fn move_down(&mut self) {
        self.r#move(&(0, -1));
    }

    fn r#move(&mut self, move_size: &(i32, i32)) {
        let (move_size_x, move_size_y) = move_size;
        let points = self.points;
        let new_points = points.map(|(x, y)| (x + move_size_x, y + move_size_y));
        self.points = new_points;
    }

    pub fn rotate(&mut self) {
        let new_direction: BlockDirection = match self.direction {
            BlockDirection::Top => BlockDirection::Left,
            BlockDirection::Right => BlockDirection::Top,
            BlockDirection::Down => BlockDirection::Right,
            BlockDirection::Left => BlockDirection::Down,
        };
        let shape: &[(i32, i32); BLOCK_ATOMS_SIZE] = self
            .template
            .shapes
            .get(new_direction.clone() as usize)
            .unwrap();
        let (center_point_x, center_point_y): &(i32, i32) = self.points.get(2).unwrap();
        let new_points = shape.map(|(x, y)| (x + center_point_x, y + center_point_y));
        self.direction = new_direction;
        self.points = new_points;
    }
}
