use crate::domain::{
    block::{atom::Atom, block::Block, helper::compress::CompressHelper},
    contract::Config,
};

pub struct BlockStack {
    config: Config,
    atoms: Vec<Atom>,
}

impl BlockStack {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            atoms: Vec::new(),
        }
    }

    pub fn ref_atoms(&self) -> &Vec<Atom> {
        &self.atoms
    }

    pub fn add_block(&mut self, block: Block) {
        self.atoms.append(&mut block.to_atoms());
    }

    // 積載ブロックが最大行を超えたか判定
    pub fn is_stack_overflow(&self) -> bool {
        for atom in &self.atoms {
            let &(_, y) = atom.ref_point();
            if y >= self.config.height - 1 {
                return true;
            }
        }
        false
    }

    // 埋まったブロック行削除 戻り値:削除行数
    pub fn compress(&mut self) -> i32 {
        let atoms = &self.atoms;
        let vec_is_fullfill_by_line =
            CompressHelper::get_vec_is_fullfill_by_line(&self.config, &atoms);
        let delete_line_count = CompressHelper::get_delete_line_count(&vec_is_fullfill_by_line);
        if delete_line_count > 0 {
            let deleted_atoms = CompressHelper::delete_atoms(&atoms, &vec_is_fullfill_by_line);
            let vec_down_size_by_line =
                CompressHelper::get_vec_down_size_by_line(&vec_is_fullfill_by_line);
            let compressed_atoms =
                CompressHelper::compress_atoms(&deleted_atoms, &vec_down_size_by_line);
            self.atoms = compressed_atoms;
        }
        delete_line_count
    }
}
