use macroquad::window::Conf;

use crate::tetris::Tetris;

mod board;
mod piece;
mod tetris;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut tetris = Tetris::new();
    tetris.run().await;
}
