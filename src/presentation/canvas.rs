use crate::domain::block::Block;
use crate::presentation::cell::Cell;
use std::mem;

pub struct Canvas {
    width: i32,
    height: i32,
    bgcolor: (i32, i32, i32),
    pub cells: Option<Vec<Vec<Cell>>>,
}

impl Canvas {
    pub fn new(width: i32, height: i32, bgcolor: (i32, i32, i32)) -> Self {
        Self {
            width,
            height,
            bgcolor,
            cells: None,
        }
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

    pub fn update(&mut self, blocks: Vec<Block>) {
        self.init();

        let mut vec_cell = self.cells.clone().unwrap();
        for block in blocks {
            for (x, y) in block.points {
                let mut new_cell = Cell {
                    is_block: true,
                    bgcolor: block.template.color,
                };

                mem::swap(&mut vec_cell[y as usize][x as usize], &mut new_cell);
            }
        }
        self.cells = Some(vec_cell);
    }
}
