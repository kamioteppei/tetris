use crate::domain::block::BLOCK_ATOMS_SIZE;
use crate::domain::block::BLOCK_DIRECTION_ITEMS_SIZE;
use rand::Rng;

pub const BLOCK_TEMPLATE_ITEMS_SIZE: usize = 7;

#[derive(Clone)]
pub struct BlockTemplate {
    pub shapes: [[(i32, i32); BLOCK_ATOMS_SIZE]; BLOCK_DIRECTION_ITEMS_SIZE],
    pub color: (i32, i32, i32),
}

pub struct BlockTemplates {
    pub templates: [BlockTemplate; BLOCK_TEMPLATE_ITEMS_SIZE],
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
