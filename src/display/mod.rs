extern crate termion;
use std::io::Write;
use termion::clear;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

use crate::move_to_built;
use crate::Tetromino;
use crate::TetrominoCharacter;

/// Adds the scrren elements to the screen array
///
/// # Arguments
///
/// * `screen` - An array representing the screen instelf
pub fn create_screen(screen: &mut [[&str; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        screen[i][0] = "<!";
        for j in 1..WIDTH - 1 {
            screen[i][j] = " . ";
        }
        screen[i][WIDTH - 1] = "!>";
    }
}

pub fn display_screen(
    screen: &[[&str; WIDTH]; HEIGHT],
    current_tetromino: &mut Tetromino,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    built_tetroinoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
    score: u32,
) {
    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    writeln!(stdout, "{} Score: {}", termion::cursor::Goto(12, 0), score).unwrap();

    write!(stdout, "\n\r").unwrap();

    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;

            if built_tetroinoes[i][j].x == j as i32
                && built_tetroinoes[i][j].y == i as i32
                && !built_tetroinoes[i][j].value.is_empty()
            {
                write!(stdout, "{}", built_tetroinoes[i][j].value).unwrap();
                j += 1;
                continue;
            }

            let skip_distance_first = (current_tetromino.first_line.characters.len()) as usize;
            let skip_distance_second = (current_tetromino.second_line.characters.len()) as usize;

            let skip_distance_third = (current_tetromino.third_line.characters.len()) as usize;

            let skip_distance_fourth = (current_tetromino.fourth_line.characters.len()) as usize;

            if current_tetromino.first_line.x as usize == j
                && current_tetromino.first_line.y as usize == i
            {
                write!(stdout, "{}", current_tetromino.first_line).unwrap();
                stdout.flush().unwrap();
                j += skip_distance_first;
                found_tetromino = true;
                if current_tetromino.second_line.to_string().is_empty() {
                    move_to_built(current_tetromino, built_tetroinoes);
                }
            } else if current_tetromino.second_line.x as usize == j
                && current_tetromino.second_line.y as usize == i
                && !current_tetromino.second_line.to_string().is_empty()
            {
                found_tetromino = true;
                j += skip_distance_second;
                write!(stdout, "{}", current_tetromino.second_line).unwrap();
                stdout.flush().unwrap();
                if current_tetromino.third_line.to_string().is_empty() {
                    move_to_built(current_tetromino, built_tetroinoes);
                }
            } else if current_tetromino.third_line.x as usize == j
                && current_tetromino.third_line.y as usize == i
                && !current_tetromino.third_line.to_string().is_empty()
            {
                found_tetromino = true;
                j += skip_distance_third;
                write!(stdout, "{}", current_tetromino.third_line).unwrap();
                stdout.flush().unwrap();
                if current_tetromino.fourth_line.to_string().is_empty() {
                    move_to_built(current_tetromino, built_tetroinoes);
                }
            } else if current_tetromino.shape_type == 1
                && current_tetromino.fourth_line.x as usize == j
                && current_tetromino.fourth_line.y as usize == i
                && !current_tetromino.fourth_line.to_string().is_empty()
            {
                found_tetromino = true;
                j += skip_distance_fourth;
                write!(stdout, "{}", current_tetromino.fourth_line).unwrap();
                stdout.flush().unwrap();
                move_to_built(current_tetromino, built_tetroinoes);
            }

            if !found_tetromino {
                write!(stdout, "{}", screen[i][j]).unwrap();
                j += 1;
            }
        }
        write!(stdout, "\n\r").unwrap();
    }
}
