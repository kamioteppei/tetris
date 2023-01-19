use crate::domain::atom::Atom;
use crate::presentation::cell::Cell;
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

    pub fn ref_cells(&self) -> &Option<Vec<Vec<Cell>>> {
        &self.cells
    }

    pub fn init(&mut self) {
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
        self.init();

        let mut vec_cell = self.cells.clone().unwrap();
        for atom in atoms {
            let (x, y) = atom.point;
            // 画面上に隠れている部分は描画対象外
            if y >= self.height {
                continue;
            }
            let mut new_cell = Cell {
                is_block: true,
                bgcolor: atom.bgcolor,
            };

            mem::swap(&mut vec_cell[y as usize][x as usize], &mut new_cell);
        }
        self.cells = Some(vec_cell);
    }
}
