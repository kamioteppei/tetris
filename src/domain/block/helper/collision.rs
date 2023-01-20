use crate::domain::{
    block::{atom::Atom, block::Block},
    contract::Config,
};
use std::mem;

pub struct CollisionHelper {}
impl CollisionHelper {
    // 浮遊ブロックが積載ブロックに接地したか判定
    pub fn is_on_stack_height(config: &Config, float_block: &Block, atoms: &Vec<Atom>) -> bool {
        // 各列の最大行数配列
        let vec_stack_height_by_col = Self::get_vec_stack_height_by_col(config, atoms);
        // 浮遊ブロックが列の最大行より下にあるか判定
        for (x, y) in float_block.ref_points() {
            if y <= vec_stack_height_by_col[x as usize] + 1 {
                return true;
            }
        }
        false
    }

    fn get_vec_stack_height_by_col(config: &Config, atoms: &Vec<Atom>) -> Vec<i32> {
        // 各列の最大行数配列を生成（ブロックがない列は-1行目とみなす）
        let mut vec_stack_height_by_col = vec![-1; config.width as usize];
        for atom in atoms {
            let &(x, mut y) = atom.ref_point();
            if y > vec_stack_height_by_col[x as usize] {
                mem::swap(&mut vec_stack_height_by_col[x as usize], &mut y);
            }
        }
        vec_stack_height_by_col
    }
}
