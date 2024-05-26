pub mod characters;
pub mod line;
pub mod tetromino;

use self::line::Line;
use self::tetromino::Tetromino;
use rand::Rng;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

pub fn create_tetronimo(tetrominoes_list: &mut Vec<Tetromino>) {
    let random_number: i32 = random_tetronimo();
    let x_position: i32 = random_tetromino_position();
    let mut tetromino_shape: Tetromino = Tetromino::blank_tetromino(x_position);

    match random_number {
        1 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 4);
            tetromino_shape.shape_type = 1;
        }
        2 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 3);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position + 2, tetromino_shape.first_line.y + 1, 1);
            tetromino_shape.second_line.x = x_position + 2;
            tetromino_shape.shape_type = 2;
        }
        3 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 2);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y + 1, 2);
            tetromino_shape.second_line.x = x_position;
            tetromino_shape.shape_type = 3;
        }
        4 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 2);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first_line.y + 1, 2);
            tetromino_shape.second_line.x = x_position + 1;
            tetromino_shape.shape_type = 4;
        }
        5 => {
            tetromino_shape.first_line.characters =
                Line::create_characters(x_position, tetromino_shape.first_line.y, 3);
            tetromino_shape.second_line.characters =
                Line::create_characters(x_position + 1, tetromino_shape.first_line.y + 1, 1);
            tetromino_shape.second_line.x = x_position + 1;
            tetromino_shape.shape_type = 5;
        }
        _ => panic!("Invalid tetromino shape"),
    }

    tetrominoes_list.push(tetromino_shape);
}

pub fn move_tetrmonioes(
    tetrominoes_list: &mut Vec<Tetromino>,
    x_movement: i32,
    y_movement: i32,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(x_movement, y_movement, game_borders);
    }
}

pub fn rotate_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    rotation: i32,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.rotate(rotation, game_borders);
    }
}

pub fn update_tetrominoes(
    tetrominoes_list: &mut Vec<Tetromino>,
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
) {
    for tetromino in tetrominoes_list {
        tetromino.move_tetromino(0, 1, game_borders);
    }
}

fn random_tetronimo() -> i32 {
    return rand::thread_rng().gen_range(1..=5) as i32;
}

fn random_tetromino_position() -> i32 {
    return rand::thread_rng().gen_range(2..WIDTH - 4) as i32;
}
