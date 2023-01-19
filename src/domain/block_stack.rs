use crate::domain::{atom::Atom, block::Block, contract::Config};
use std::mem;

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

    // 浮遊ブロックが積載ブロックに接地したか判定
    pub fn is_on_stack_height(&self, float_block: &Block) -> bool {
        // 各列の最大行数配列
        let vec_stack_height_by_col = self.get_vec_stack_height_by_col();
        // 浮遊ブロックが列の最大行より下にあるか判定
        for (x, y) in float_block.points {
            if y <= vec_stack_height_by_col[x as usize] + 1 {
                return true;
            }
        }
        false
    }

    // 積載ブロックが最大行を超えたか判定
    pub fn is_stack_overflow(&self) -> bool {
        for atom in &self.atoms {
            let (_, y) = atom.point;
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

    fn get_vec_stack_height_by_col(&self) -> Vec<i32> {
        // 各列の最大行数配列を生成（ブロックがない列は-1行目とみなす）
        let mut vec_stack_height_by_col = vec![-1; self.config.width as usize];
        for atom in &self.atoms {
            let (x, mut y) = atom.point;
            if y > vec_stack_height_by_col[x as usize] {
                mem::swap(&mut vec_stack_height_by_col[x as usize], &mut y);
            }
        }
        vec_stack_height_by_col
    }
}

struct CompressHelper {}
impl CompressHelper {
    pub fn get_vec_is_fullfill_by_line(config: &Config, atoms: &Vec<Atom>) -> Vec<bool> {
        let mut vec_atom_counts_by_line = vec![0; config.height as usize];
        for atom in atoms {
            let (_, y) = atom.point;
            vec_atom_counts_by_line[y as usize] += 1;
        }
        let vec_is_fullfill_by_line: Vec<bool> = vec_atom_counts_by_line
            .iter()
            .map(|count| *count == config.width)
            .collect();
        vec_is_fullfill_by_line
    }

    pub fn get_vec_down_size_by_line(vec_is_fullfill_by_line: &Vec<bool>) -> Vec<i32> {
        let mut down_size_temp = 0;
        let vec_down_size_by_line = vec_is_fullfill_by_line
            .iter()
            .map(|is_delete| {
                if *is_delete {
                    {
                        down_size_temp += 1;
                        0
                    }
                } else {
                    down_size_temp
                }
            })
            .collect();
        vec_down_size_by_line
    }

    pub fn delete_atoms(atoms: &Vec<Atom>, vec_is_fullfill_by_line: &Vec<bool>) -> Vec<Atom> {
        let deleted_atoms = atoms
            .clone()
            .into_iter()
            .filter(|atom| {
                let (_, y) = atom.point;
                !vec_is_fullfill_by_line[y as usize]
            })
            .collect();
        deleted_atoms
    }

    pub fn compress_atoms(atoms: &Vec<Atom>, vec_down_size_by_line: &Vec<i32>) -> Vec<Atom> {
        let compressed_atoms = atoms
            .clone()
            .into_iter()
            .map(|mut atom| {
                let (x, y) = atom.point;
                atom.point = (x, y - vec_down_size_by_line[y as usize]);
                atom
            })
            .collect();
        compressed_atoms
    }

    pub fn get_delete_line_count(vec_is_fullfill_by_line: &Vec<bool>) -> i32 {
        let delete_line_count = vec_is_fullfill_by_line
            .iter()
            .filter(|is_delete| **is_delete)
            .count();
        delete_line_count as i32
    }
}
