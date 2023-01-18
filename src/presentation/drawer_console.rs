use crate::domain::{
    contract::{IConsoleGame, IDrawer},
    tetris::Tetris,
};

pub struct DrawConsole {}
impl IDrawer for DrawConsole {
    fn draw(&self, tetris: &Tetris) {
        // 上段から回す
        for i in (0..tetris.get_config().height).rev() {
            let mut buf: String = String::from(" ");
            // 左端から回す
            for j in 0..tetris.get_config().width {
                let cells = tetris.get_draw_info().cells.as_ref();
                let cell = cells
                    .unwrap()
                    .get(i as usize)
                    .unwrap()
                    .get(j as usize)
                    .unwrap();
                buf += if cell.is_block { "■" } else { "□" };
            }
            println!("\x1B[{};1H{}", tetris.get_config().height - i, buf);
        }
    }
}
