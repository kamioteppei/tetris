use crate::domain::{
    block::{
        r#type::{BlockTemplates, BLOCK_TEMPLATE_ITEMS_SIZE},
        template::BlockTemplate,
    },
    contract::IBlockTemplateRepository,
};
use rand::Rng;

pub struct BlockTemplateRepository {
    templates: BlockTemplates,
}

impl BlockTemplateRepository {
    pub fn new() -> Self {
        Self {
            templates: [
                BlockTemplate::new(
                    [
                        [(0, -1), (1, -1), (-1, 0), (0, 0)],
                        [(0, -1), (0, 0), (1, 0), (1, 1)],
                        [(0, 0), (1, 0), (-1, 1), (0, 1)],
                        [(-1, -1), (-1, 0), (0, 0), (0, 1)],
                    ],
                    (0, 0, 0),
                ),
                BlockTemplate::new(
                    [
                        [(-1, -1), (0, -1), (0, 0), (1, 0)],
                        [(1, -1), (0, 0), (1, 0), (0, 1)],
                        [(-1, 0), (0, 0), (0, 1), (1, 1)],
                        [(0, -1), (0, 0), (-1, 0), (-1, 1)],
                    ],
                    (0, 0, 0),
                ),
                BlockTemplate::new(
                    [
                        [(-1, 0), (0, 0), (1, 0), (2, 0)],
                        [(1, -1), (1, 0), (1, 1), (1, 2)],
                        [(-1, 1), (0, 1), (1, 1), (2, 1)],
                        [(0, -1), (0, 0), (0, 1), (0, 2)],
                    ],
                    (0, 0, 0),
                ),
                BlockTemplate::new(
                    [
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                        [(0, -1), (1, -1), (0, 0), (1, 0)],
                    ],
                    (0, 0, 0),
                ),
                BlockTemplate::new(
                    [
                        [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                        [(1, -1), (0, -1), (0, 0), (0, 1)],
                        [(-1, 0), (0, 0), (1, 0), (1, 1)],
                        [(0, -1), (0, 0), (0, 1), (-1, 1)],
                    ],
                    (0, 0, 0),
                ),
                BlockTemplate::new(
                    [
                        [(1, -1), (-1, 0), (0, 0), (1, 0)],
                        [(0, -1), (0, 0), (0, 1), (1, 1)],
                        [(-1, 0), (0, 0), (1, 0), (-1, 1)],
                        [(-1, -1), (0, -1), (0, 0), (0, 1)],
                    ],
                    (0, 0, 0),
                ),
                BlockTemplate::new(
                    [
                        [(0, -1), (-1, 0), (0, 0), (1, 0)],
                        [(0, -1), (0, 0), (1, 0), (0, 1)],
                        [(-1, 0), (0, 0), (1, 0), (0, 1)],
                        [(0, -1), (-1, 0), (0, 0), (0, 1)],
                    ],
                    (0, 0, 0),
                ),
            ],
        }
    }
}

impl IBlockTemplateRepository for BlockTemplateRepository {
    fn choose_random(&self) -> BlockTemplate {
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(0, BLOCK_TEMPLATE_ITEMS_SIZE);
        let template: Option<&BlockTemplate> = self.templates.get(random_num);
        template.unwrap().clone()
    }
}
