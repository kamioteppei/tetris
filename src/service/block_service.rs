use crate::{
    domain::{block::Block, contract::IBlockTemplateRepository},
    repository::block_template_repository::BlockTemplateRepository,
};

pub struct BlockService {
    block_template_repository: BlockTemplateRepository,
}

impl BlockService {
    pub fn new() -> Self {
        Self {
            block_template_repository: BlockTemplateRepository::new(),
        }
    }

    // 新規ブロック生成
    pub fn create_block(&self) -> Block {
        let random_template = self.block_template_repository.choose_random();
        let new_block = Block::new(random_template);
        new_block
    }
}
