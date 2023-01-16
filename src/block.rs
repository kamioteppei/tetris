use rand::Rng;

const BLOCK_ATOMS_SIZE: usize = 4;
const BLOCK_DIRECTION_ITEMS_SIZE: usize = 4;
const BLOCK_TEMPLATE_ITEMS_SIZE: usize = 7;

#[derive(Clone)]
pub enum BlockDirection {
    Top,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
pub struct BlockTemplate {
    shapes: [[(i32, i32); BLOCK_ATOMS_SIZE]; BLOCK_DIRECTION_ITEMS_SIZE],
    pub color: (i32, i32, i32),
}

pub struct BlockTemplates {
    templates: [BlockTemplate; BLOCK_TEMPLATE_ITEMS_SIZE],
}

impl BlockTemplates {
    pub fn new() -> Self {
        Self {
            templates: [
                BlockTemplate {
                    shapes: [
                        [(0, -1), (1, -1), (-1, 0), (0, 0)],
                        [(0, -1), (0, 0), (1, 0), (1, 1)],
                        [(0, 0), (1, 0), (-1, 1), (0, 1)],
                        [(-1, -1), (-1, 0), (0, 0), (0, 1)],
                    ],
                    color: (0, 0, 0),
                },
                BlockTemplate {
                    shapes: [
                        [(-1, -1), (0, -1), (0, 0), (1, 0)],
                        [(1, -1), (0, 0), (1, 0), (0, 1)],
                        [(-1, 0), (0, 0), (0, 1), (1, 1)],
                        [(0, -1), (0, 0), (-1, 0), (-1, 1)],
                    ],
                    color: (0, 0, 0),
                },
                BlockTemplate {
                    shapes: [
                        [(-1, 0), (0, 0), (1, 0), (2, 0)],
                        [(1, -1), (1, 0), (1, 1), (1, 2)],
                        [(-1, 1), (0, 1), (1, 1), (2, 1)],
                        [(0, -1), (0, 0), (0, 1), (0, 2)],
                    ],
                    color: (0, 0, 0),
                },
                BlockTemplate {
                    shapes: [
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                    ],
                    color: (0, 0, 0),
                },
                BlockTemplate {
                    shapes: [
                        [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                        [(1, -1), (0, -1), (0, 0), (0, 1)],
                        [(-1, 0), (0, 0), (1, 0), (1, 1)],
                        [(0, -1), (0, 0), (0, 1), (-1, 1)],
                    ],
                    color: (0, 0, 0),
                },
                BlockTemplate {
                    shapes: [
                        [(1, -1), (-1, 0), (0, 0), (1, 0)],
                        [(0, -1), (0, 0), (0, 1), (1, 1)],
                        [(-1, 0), (0, 0), (1, 0), (-1, 1)],
                        [(-1, -1), (0, -1), (0, 0), (0, 1)],
                    ],
                    color: (0, 0, 0),
                },
                BlockTemplate {
                    shapes: [
                        [(0, -1), (-1, 0), (0, 0), (1, 0)],
                        [(0, -1), (0, 0), (1, 0), (0, 1)],
                        [(-1, 0), (0, 0), (1, 0), (0, 1)],
                        [(0, -1), (-1, 0), (0, 0), (0, 1)],
                    ],
                    color: (0, 0, 0),
                },
            ],
        }
    }

    pub fn choose_random(&self) -> BlockTemplate {
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(0, BLOCK_TEMPLATE_ITEMS_SIZE);
        let template: Option<&BlockTemplate> = self.templates.get(random_num);
        template.unwrap().clone()
    }
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
        let (center_point_x, center_point_y): &(i32, i32) = self.points.get(1).unwrap();
        let new_points = shape.map(|(x, y)| (x + center_point_x, y + center_point_y));
        self.direction = new_direction;
        self.points = new_points;
    }
}
