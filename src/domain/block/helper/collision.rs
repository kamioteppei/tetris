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
        // TODO: バグ修正　最大行だけでは条件不足⇒地続きの中で最大行
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

    pub fn validate_block(config: &Config, float_block: &Block, atoms: &Vec<Atom>) -> bool {
        // 画面両外側にはみ出している場合不可
        let is_overlap_to_outline = float_block
            .ref_points()
            .into_iter()
            .any(|(x, _)| x < 0 || x >= config.width);
        if is_overlap_to_outline {
            return false;
        }
        // 積載ブロックと重複している場合不可
        for (bx, by) in float_block.ref_points() {
            let is_overlap_to_stack = atoms.into_iter().any(|atom| {
                let &(ax, ay) = atom.ref_point();
                ax == bx && ay == by
            });
            if is_overlap_to_stack {
                return false;
            }
        }
        true
    }
}
