use macroquad::prelude::rand;

pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone)]
pub struct Piece {
    pub x: i32,
    pub y: i32,
    pub shape: [[u8; 4]; 4],
}

impl Piece {
    pub fn new() -> Piece {
        let piece_index = rand::gen_range::<usize>(0, 7);

        let shape = PIECES[piece_index];
        let x = 2;

        Piece { x, y: 0, shape }
    }

    pub fn rotate(&mut self, rotation: &Rotation) {
        let temp = self.shape;
        match rotation {
            Rotation::Clockwise => {
                for y in 0..self.shape.len() {
                    for x in 0..self.shape[y].len() {
                        self.shape[y][x] = temp[3 - x][y];
                    }
                }
            }
            Rotation::CounterClockwise => {
                for y in 0..self.shape.len() {
                    (0..self.shape[y].len()).for_each(|x| {
                        self.shape[y][x] = temp[x][3 - y];
                    });
                }
            }
        }
    }
}

pub const PIECES: [[[u8; 4]; 4]; 7] = [
    // I
    [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
    // J
    [[2, 0, 0, 0], [2, 2, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // L
    [[0, 0, 3, 0], [3, 3, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // O
    [[0, 0, 0, 0], [0, 4, 4, 0], [0, 4, 4, 0], [0, 0, 0, 0]],
    // S
    [[0, 5, 5, 0], [5, 5, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // T
    [[0, 6, 0, 0], [6, 6, 6, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    // Z
    [[7, 7, 0, 0], [0, 7, 7, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
];
