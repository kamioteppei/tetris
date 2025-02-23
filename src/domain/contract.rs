use crate::domain::block::template::BlockTemplate;

pub trait IBlockTemplateRepository {
    fn choose_random(&self) -> BlockTemplate;
}
