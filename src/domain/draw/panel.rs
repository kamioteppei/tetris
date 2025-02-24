use std::vec::Vec;

// 背景レイヤーを表す構造体
pub struct DrawPanel {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl DrawPanel {
    // 新しい背景レイヤーを作成
    pub fn new(width: usize, height: usize, default_char: char) -> Self {
        let data = vec![vec![default_char; width]; height];
        DrawPanel {
            width,
            height,
            data,
        }
    }

    // 1行の文字列を指定位置に上書き
    pub fn overlay_string(&mut self, s: &str, row: usize, col: usize) {
        let chars = string_to_chars(s);
        for (j, &ch) in chars.iter().enumerate() {
            if row < self.height && col + j < self.width {
                self.data[row][col + j] = ch;
            }
        }
    }

    // 複数行の文字列を指定位置に上書き
    pub fn overlay_strings(&mut self, lines: &Vec<String>, row: usize, col: usize) {
        for (i, line) in lines.iter().enumerate() {
            self.overlay_string(line, row + i, col);
        }
    }

    // 背景レイヤーを出力
    pub fn print(&self) {
        for row in &self.data {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

// 文字列をcharのベクターに変換する関数
fn string_to_chars(s: &str) -> Vec<char> {
    s.chars().collect()
}
