use macroquad::prelude::Vec2;

use crate::piece::Piece;

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub grid: Vec<Vec<u8>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            width: 10,
            height: 20,
            grid: vec![vec![0; 10]; 20],
        }
    }

    pub fn clear_top_row(&mut self) {
        for x in 0..self.width {
            self.grid[0][x as usize] = 0;
        }
    }

    pub fn place_piece(&mut self, piece: Piece) {
        for y in 0..piece.shape.len() {
            for x in 0..piece.shape[y].len() {
                if piece.shape[y][x] != 0 {
                    self.grid[(piece.y + y as i32) as usize][(piece.x + x as i32) as usize] =
                        piece.shape[y][x];
                }
            }
        }
    }

    pub fn piece_can_move(&self, piece: Piece, delta: Vec2) -> bool {
        for y in 0..piece.shape.len() as i32 {
            for x in 0..piece.shape[y as usize].len() as i32 {
                if piece.shape[y as usize][x as usize] != 0 {
                    if ((piece.x + x) as f32 + delta.x) < 0.0
                        || (piece.x + x + delta.x as i32) >= self.width
                    {
                        return false;
                    }

                    if ((piece.y + y) as f32 + delta.y) < 0.0
                        || (piece.y + y + delta.y as i32) >= self.height
                    {
                        return false;
                    }

                    if self.grid[(piece.y + y + delta.y as i32) as usize]
                        [(piece.x + x + delta.x as i32) as usize]
                        != 0
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}
