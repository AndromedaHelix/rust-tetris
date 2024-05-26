extern crate termion;
use std::io::Write;
use termion::clear;

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

use crate::Tetromino;
use crate::TetrominoCharacter;

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
    unrendered_tetrominoes: &mut Vec<Tetromino>,
    rendered_tetrominoes: &mut Vec<Tetromino>,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    built_tetroinoes: &[[TetrominoCharacter; WIDTH]; HEIGHT],
    score: u32,
) {
    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    writeln!(stdout, "{} Score: {}", termion::cursor::Goto(12,0), score).unwrap();

    write!(stdout, "\n\r").unwrap();

    for i in 0..HEIGHT {
        let mut j = 0;
        while j < WIDTH {
            let mut found_tetromino = false;
            let mut x = 0;

            if built_tetroinoes[i][j].x == j as i32
                && built_tetroinoes[i][j].y == i as i32
                && !built_tetroinoes[i][j].value.is_empty()
            {
                write!(stdout, "{}", built_tetroinoes[i][j].value).unwrap();
                j += 1;
                continue;
            }

            while !unrendered_tetrominoes.is_empty() && x < unrendered_tetrominoes.len() {
                let tetromino = &unrendered_tetrominoes[x];

                let skip_distance_first = (tetromino.first_line.characters.len()) as usize;
                let skip_distance_second = (tetromino.second_line.characters.len()) as usize;

                let skip_distance_third = (tetromino.third_line.characters.len()) as usize;

                let skip_distance_fourth = (tetromino.fourth_line.characters.len()) as usize;

                if tetromino.first_line.x as usize == j && tetromino.first_line.y as usize == i {
                    write!(stdout, "{}", tetromino.first_line).unwrap();
                    stdout.flush().unwrap();
                    j += skip_distance_first;
                    found_tetromino = true;
                    if tetromino.second_line.to_string().is_empty() {
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    }
                    break;
                } else if tetromino.second_line.x as usize == j
                    && tetromino.second_line.y as usize == i
                    && !tetromino.second_line.to_string().is_empty()
                {
                    found_tetromino = true;
                    j += skip_distance_second;
                    write!(stdout, "{}", tetromino.second_line).unwrap();
                    stdout.flush().unwrap();
                    if tetromino.third_line.to_string().is_empty() {
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    }
                    break;
                } else if tetromino.third_line.x as usize == j
                    && tetromino.third_line.y as usize == i
                    && !tetromino.third_line.to_string().is_empty()
                {
                    found_tetromino = true;
                    j += skip_distance_third;
                    write!(stdout, "{}", tetromino.third_line).unwrap();
                    stdout.flush().unwrap();
                    if tetromino.fourth_line.to_string().is_empty() {
                        rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    }
                    break;
                } else if tetromino.shape_type == 1
                    && tetromino.fourth_line.x as usize == j
                    && tetromino.fourth_line.y as usize == i
                    && !tetromino.fourth_line.to_string().is_empty()
                {
                    found_tetromino = true;
                    j += skip_distance_fourth;
                    write!(stdout, "{}", tetromino.fourth_line).unwrap();
                    stdout.flush().unwrap();
                    rendered_tetrominoes.push(unrendered_tetrominoes.remove(x));
                    break;
                }

                x += 1;
            }

            if !found_tetromino {
                write!(stdout, "{}", screen[i][j]).unwrap();
                j += 1;
            }
        }
        write!(stdout, "\n\r").unwrap();
    }

    unrendered_tetrominoes.append(rendered_tetrominoes);
}
