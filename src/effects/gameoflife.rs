use std::{thread, time};
use rand::{Rng, SeedableRng};

use crate::vector::{draw_points_float, Point};
use crate::util::{remove_random_points, remove_random_samples};

const WIDTH: usize = crate::SIZE as usize;
const HEIGHT: usize = crate::SIZE as usize;

pub fn game_of_life(t: usize) -> Vec<(f32, f32)> {
    let mut board = initialize_board();
    let mut display: Vec<(f32, f32)> = Vec::new();
    for _ in 0..t {
        let print = print_board_4x4(&board);
        for p in print {display.push(p)};
        update_board(&mut board);
    }
    display
}

fn initialize_board() -> [[bool; WIDTH]; HEIGHT] {
    // Initialize the board with random initial state or any desired pattern.
    let mut board = [[false; WIDTH]; HEIGHT];
    
    // Example: Glider pattern
    board[2][4] = true;
    board[3][5] = true;
    board[4][3] = true;
    board[4][4] = true;
    board[4][5] = true;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(5);
    for _ in 1..700 { 
        let x: usize = rng.gen_range(WIDTH/2..WIDTH);
        let y: usize = rng.gen_range(HEIGHT/2..HEIGHT);
        board[x][y] = true; 
    }
    
    board
}

fn print_board(board: &[[bool; WIDTH]; HEIGHT]) -> Vec<(f32, f32)> {
    let mut display: Vec<(f32, f32)> = Vec::new();
    let mut cells: Vec<Point> = Vec::new();
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell { cells.push(Point { 
                x: row_index as f32 - crate::SIZE_F/2.,y: col_index as f32 - crate::SIZE_F/2.}) 
            }
        }
    }
    let new_cells = remove_random_points(cells, 1./50.);
    let cli = draw_points_float(1. / 50., new_cells, 3);
    for cl in cli {
        display.push(cl);
    }
    display
}

fn print_board_4x4(board: &[[bool; WIDTH]; HEIGHT]) -> Vec<(f32, f32)> {
    let mut display: Vec<(f32, f32)> = Vec::new();
    let mut cells: Vec<Point> = Vec::new();
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell { 

                cells.push(Point { 
                    x: row_index as f32,
                    y: col_index as f32}
                );
            }
        }
    }
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell { 

                cells.push(Point { 
                    x: row_index as f32*-1.,
                    y: col_index as f32}
                );
            }
        }
    }
    let new_cells = remove_random_points(cells, 1./50.);
    //println!("{}", new_cells.len());
    let cli = draw_points_float(1. / 50., new_cells, 1);
    for cl in cli {
        display.push(cl);
    }
    display
}

fn update_board(board: &mut [[bool; WIDTH]; HEIGHT]) {
    let mut new_board = [[false; WIDTH]; HEIGHT];
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let live_neighbors = count_live_neighbors(board, i, j);
            
            new_board[i][j] = match (board[i][j], live_neighbors) {
                (true, 2..=3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    
    *board = new_board;
}

fn count_live_neighbors(board: &[[bool; WIDTH]; HEIGHT], row: usize, col: usize) -> usize {
    let mut count = 0;
    
    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if i >= 0 && i < HEIGHT as isize && j >= 0 && j < WIDTH as isize {
                if (i as usize != row || j as usize != col) && board[i as usize][j as usize] {
                    count += 1;
                }
            }
        }
    }
    
    count
}
