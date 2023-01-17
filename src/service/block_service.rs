use crate::{
    domain::{
        block::Block,
        contract::{Config, IBlockTemplateRepository},
    },
    repository::block_template_repository::BlockTemplateRepository,
};

pub struct BlockService {
    config: Config,
    block_template_repository: BlockTemplateRepository,
}

impl BlockService {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            block_template_repository: BlockTemplateRepository::new(),
        }
    }

    // 新規ブロック生成
    pub fn create_block(&self) -> Block {
        let random_template = self.block_template_repository.choose_random();
        let mut new_block = Block::new(random_template);
        let start_pos_x = self.config.width / 2_i32 - 1;
        let start_pos_y = self.config.height - 1;
        new_block.init(&(start_pos_x, start_pos_y));
        new_block
    }
}
