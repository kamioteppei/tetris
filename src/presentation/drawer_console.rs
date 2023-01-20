use crate::domain::{
    contract::{IDrawer, Status},
    draw::draw_info::DrawInfo,
};

pub struct DrawConsole {}

impl IDrawer for DrawConsole {
    fn draw(&self, draw_info: &DrawInfo, status: &Status) {
        // 上段から回す
        for i in (0..draw_info.ref_height()).rev() {
            let mut buf: String = String::from(" ");
            // 左端から回す
            for j in 0..draw_info.ref_width() {
                let cells = draw_info.ref_cells().as_ref();
                let cell = cells
                    .unwrap()
                    .get(i as usize)
                    .unwrap()
                    .get(j as usize)
                    .unwrap();
                buf += if cell.is_block { "■" } else { "□" };
            }
            println!("\x1B[{};1H{}", draw_info.ref_height() - i, buf);
        }
        println!("Score: {}", status.score);
    }
}
