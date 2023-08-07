use std::collections::VecDeque;

use macroquad::{color, prelude::*};

use crate::{
    board::Board,
    piece::{Piece, Rotation},
};

pub const COLORS: [Color; 8] = [
    color::BLACK,
    color::BLUE,
    color::GREEN,
    color::YELLOW,
    color::ORANGE,
    color::PURPLE,
    color::WHITE,
    color::RED,
];

pub const COLORS_GHOST: [Color; 8] = [
    Color::new(0.0, 0.0, 0.0, 0.5),
    Color::new(0.0, 0.0, 1.0, 0.5),
    Color::new(0.0, 1.0, 0.0, 0.5),
    Color::new(1.0, 1.0, 0.0, 0.5),
    Color::new(1.0, 0.64, 0.0, 0.5),
    Color::new(1.0, 0.0, 1.0, 0.5),
    Color::new(1.0, 1.0, 1.0, 0.5),
    Color::new(1.0, 0.0, 0.0, 0.5),
];

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
}

pub struct Tetris {
    // Queue for next pieces
    pub next: VecDeque<Piece>,

    // Board
    pub board: Board,

    // Current piece
    pub piece: Piece,

    // Current State
    pub state: GameState,

    // Score
    pub score: i32,

    // Rows cleared
    pub rows_cleared: i32,

    //Frame count
    pub frame_count: i32,

    //Wait time
    pub wait_time: i32,

    //Move timer
    pub move_timer: i32,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut pieces = Piece::generate_pieces();
        pieces.append(Piece::generate_pieces().as_mut());
        let mut next = VecDeque::new();

        for piece in pieces.iter() {
            next.push_back(piece.clone());
        }

        Tetris {
            next,
            board: Board::new(),
            piece: Piece::new(),
            state: GameState::Running,
            score: 0,
            rows_cleared: 0,
            frame_count: 0,
            wait_time: 0,
            move_timer: 0,
        }
    }
    pub async fn run(&mut self) {
        loop {
            self.update();
            self.draw();

            self.frame_count += 1;

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        }
    }

    pub fn update(&mut self) {
        if self.move_timer > 0 {
            self.move_timer -= 1;
        }

        // Wait Case
        if self.wait_time > 0 {
            self.wait_time -= 1;
            return;
        }

        // Reset Case
        if is_key_pressed(KeyCode::R) {
            self.state = GameState::Running;
            self.board = Board::new();
            self.piece = Piece::new();
            self.rows_cleared = 0;
            self.score = 0;
            self.wait_time = 0;
            return;
        }

        // GameOver Case
        if self.state == GameState::GameOver {
            return;
        }

        // Pause Case
        if is_key_pressed(KeyCode::P) && self.state == GameState::Running {
            self.state = GameState::Paused;
        } else if is_key_pressed(KeyCode::P) && self.state == GameState::Paused {
            self.state = GameState::Running;
        }

        if self.state == GameState::Paused {
            return;
        }

        // Instant drop
        if is_key_pressed(KeyCode::Space) {
            while self.piece_can_move(self.piece.clone(), Vec2 { x: 0.0, y: 1.0 }) {
                self.piece.y += 1;
            }
            self.place_piece();
            self.piece = self.next_piece();
            self.wait_time += 10;
        }

        // Handle piece horinzontal movement
        if is_key_down(KeyCode::Left)
            && self.piece_can_move(self.piece.clone(), Vec2 { x: -1.0, y: 0.0 })
            && self.move_timer == 0
        {
            self.piece.x -= 1;
            self.move_timer = 12;
        }
        if is_key_down(KeyCode::Right)
            && self.piece_can_move(self.piece.clone(), Vec2 { x: 1.0, y: 0.0 })
            && self.move_timer == 0
        {
            self.piece.x += 1;
            self.move_timer = 12;
        }

        // Handle piece rotation
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
            let rotation = match is_key_pressed(KeyCode::Up) {
                true => Rotation::Clockwise,
                false => Rotation::CounterClockwise,
            };
            let mut piece = self.piece.clone();
            piece.rotate(&rotation);

            if self.piece_can_move(piece, Vec2 { x: 0.0, y: 0.0 }) {
                self.piece.rotate(&rotation);
            }
        }

        // Chech top row
        if self.check_top_row() {
            self.state = GameState::GameOver;
        }

        // Handle piece vertical movement
        if self.frame_count % 40 == 0 {
            // Check if piece can move down further
            match self.piece_can_move(self.piece.clone(), Vec2 { x: 0.0, y: 1.0 }) {
                true => self.piece.y += 1,
                false => {
                    self.place_piece();
                    self.piece = self.next_piece();
                    self.wait_time += 10;
                }
            }

            // Check for full rows
            self.check_full_rows();
        }
    }

    fn next_piece(&mut self) -> Piece {
        let piece = self.next.pop_front().unwrap();
        if self.next.len() < 7 {
            let mut pieces = Piece::generate_pieces();
            pieces.append(Piece::generate_pieces().as_mut());
            for piece in pieces.iter() {
                self.next.push_back(piece.clone());
            }
        }
        piece
    }

    //Checks if the top row is reached for GameOver state
    fn check_top_row(&self) -> bool {
        for x in 0..self.board.width {
            if self.board.grid[0][x as usize] != 0 {
                return true;
            }
        }
        false
    }

    fn check_full_rows(&mut self) {
        let mut y = self.board.height - 1;

        let mut rows_cleared = 0;
        // Loop through rows
        'outer: while y > 0 {
            //Check if row is full
            let mut x = 0;
            while x < self.board.width {
                if self.board.grid[y as usize][x as usize] == 0 {
                    y -= 1;
                    continue 'outer;
                }
                x += 1;
            }

            // Move rows down
            let mut yy = y;
            while yy > 1 {
                let mut x = 0;
                while x < self.board.width {
                    self.board.grid[yy as usize][x as usize] =
                        self.board.grid[(yy - 1) as usize][x as usize];
                    x += 1;
                }
                yy -= 1;
            }

            //Clear top row
            self.board.clear_top_row();

            //increment rows rows_cleared
            self.rows_cleared += 1;
            rows_cleared += 1;
        }

        // Update score
        match rows_cleared {
            1 => self.score += 40,
            2 => self.score += 100,
            3 => self.score += 300,
            4 => self.score += 1200,
            _ => (),
        };
    }

    fn place_piece(&mut self) {
        self.board.place_piece(self.piece.clone());
    }

    fn piece_can_move(&self, piece: Piece, delta: Vec2) -> bool {
        self.board.piece_can_move(piece, delta)
    }

    fn draw(&mut self) {
        clear_background(BLACK);

        let width = screen_width();
        let height = screen_height();

        let block_size = height / self.board.height as f32;
        let origin = Vec2 {
            x: (width - self.board.width as f32 * block_size) / 2.0,
            y: (height - self.board.height as f32 * block_size) / 2.0,
        };

        // Draw board
        for y in 0..self.board.height {
            for x in 0..self.board.width {
                let color = COLORS[self.board.grid[y as usize][x as usize] as usize];
                draw_rectangle(
                    origin.x + x as f32 * block_size,
                    origin.y + y as f32 * block_size,
                    block_size,
                    block_size,
                    color,
                );
                draw_rectangle_lines(
                    origin.x + x as f32 * block_size,
                    origin.y + y as f32 * block_size,
                    block_size,
                    block_size,
                    2.0,
                    DARKGRAY,
                );
            }
        }

        // Draw piece
        for y in 0..self.piece.shape.len() as i32 {
            for x in 0..self.piece.shape[y as usize].len() as i32 {
                if self.piece.shape[y as usize][x as usize] != 0 {
                    let color = COLORS[self.piece.shape[y as usize][x as usize] as usize];
                    draw_rectangle(
                        origin.x + (self.piece.x + x) as f32 * block_size,
                        origin.y + (self.piece.y + y) as f32 * block_size,
                        block_size,
                        block_size,
                        color,
                    );
                    draw_rectangle_lines(
                        origin.x + (self.piece.x + x) as f32 * block_size,
                        origin.y + (self.piece.y + y) as f32 * block_size,
                        block_size,
                        block_size,
                        2.0,
                        DARKGRAY,
                    );
                }
            }
        }

        // Draw next pieces
        let mut next_origin = Vec2 {
            x: origin.x + self.board.width as f32 * block_size + 50.0,
            y: origin.y,
        };
        for piece in &self.next {
            for y in 0..piece.shape.len() as i32 {
                for x in 0..piece.shape[y as usize].len() as i32 {
                    if piece.shape[y as usize][x as usize] != 0 {
                        let color = COLORS[piece.shape[y as usize][x as usize] as usize];
                        draw_rectangle(
                            next_origin.x + x as f32 * block_size,
                            next_origin.y + y as f32 * block_size,
                            block_size,
                            block_size,
                            color,
                        );
                        draw_rectangle_lines(
                            next_origin.x + x as f32 * block_size,
                            next_origin.y + y as f32 * block_size,
                            block_size,
                            block_size,
                            2.0,
                            DARKGRAY,
                        );
                    }
                }
            }
            next_origin.y += 4.0 * block_size;
        }

        // Draw ghost piece
        let mut ghost_piece = self.piece.clone();
        while self.piece_can_move(ghost_piece.clone(), Vec2 { x: 0.0, y: 1.0 }) {
            ghost_piece.y += 1;
        }
        for y in 0..ghost_piece.shape.len() as i32 {
            for x in 0..ghost_piece.shape[y as usize].len() as i32 {
                if ghost_piece.shape[y as usize][x as usize] != 0 {
                    let color = COLORS_GHOST[ghost_piece.shape[y as usize][x as usize] as usize];
                    draw_rectangle(
                        origin.x + (ghost_piece.x + x) as f32 * block_size,
                        origin.y + (ghost_piece.y + y) as f32 * block_size,
                        block_size,
                        block_size,
                        color,
                    );
                    draw_rectangle_lines(
                        origin.x + (ghost_piece.x + x) as f32 * block_size,
                        origin.y + (ghost_piece.y + y) as f32 * block_size,
                        block_size,
                        block_size,
                        2.0,
                        DARKGRAY,
                    );
                }
            }
        }

        // Draw score
        let score_text = format!("Score: {}", self.score);
        draw_text(&score_text, 25.0, 25.0, 30.0, WHITE);

        // Draw rows cleared
        let rows_cleared_text = format!("Rows Cleared: {}", self.rows_cleared);
        draw_text(&rows_cleared_text, 25.0, 50.0, 30.0, WHITE);

        // Draw frame count
        let frame_count_text = format!("Frame Count: {}", self.frame_count);
        draw_text(&frame_count_text, 25.0, 75.0, 30.0, WHITE);

        // Draw game over
        if self.state == GameState::GameOver {
            let game_over_text = "Game Over";
            let text_width = measure_text(game_over_text, None, 40, 1.0);
            draw_text(
                game_over_text,
                width / 2.0 - text_width.width / 2.0,
                height / 2.0,
                40.0,
                WHITE,
            );
        }

        // Draw pause
        if self.state == GameState::Paused {
            let pause_text = "Paused";
            let text_width = measure_text(pause_text, None, 40, 1.0);
            draw_text(
                pause_text,
                width / 2.0 - text_width.width / 2.0,
                height / 2.0,
                40.0,
                WHITE,
            );
        }
    }
}
