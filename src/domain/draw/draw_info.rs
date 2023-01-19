use crate::domain::block::atom::Atom;
use crate::domain::draw::cell::Cell;
use std::mem;

pub struct DrawInfo {
    width: i32,
    height: i32,
    bgcolor: (i32, i32, i32),
    cells: Option<Vec<Vec<Cell>>>,
}

impl DrawInfo {
    pub fn new(width: i32, height: i32, bgcolor: (i32, i32, i32)) -> Self {
        Self {
            width,
            height,
            bgcolor,
            cells: None,
        }
    }

    pub fn ref_width(&self) -> i32 {
        self.width
    }

    pub fn ref_height(&self) -> i32 {
        self.height
    }

    pub fn ref_cells(&self) -> &Option<Vec<Vec<Cell>>> {
        &self.cells
    }

    pub fn clear(&mut self) {
        self.cells = Some(vec![
            vec![
                Cell {
                    is_block: false,
                    bgcolor: self.bgcolor,
                };
                self.width as usize
            ];
            self.height as usize
        ]);
    }

    pub fn update(&mut self, atoms: &Vec<Atom>) {
        self.clear();

        let mut vec_cell = self.cells.clone().unwrap();
        for atom in atoms {
            let &(x, y) = atom.ref_point();
            // 画面上に隠れている部分は描画対象外
            if y >= self.height {
                continue;
            }
            let mut new_cell = Cell {
                is_block: true,
                bgcolor: atom.ref_bgcolor().clone(),
            };

            mem::swap(&mut vec_cell[y as usize][x as usize], &mut new_cell);
        }
        self.cells = Some(vec_cell);
    }
}
