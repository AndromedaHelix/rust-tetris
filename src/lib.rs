const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

use std::io::Bytes;

extern crate termion;
use std::io::Write;
use termion::AsyncReader;

use std::thread;
use std::time::Duration;

pub mod tetromino;

use crate::tetromino::characters::TetrominoCharacter;
use crate::tetromino::tetromino::Tetromino;

mod display;

pub struct GameConfig<'a> {
    pub screen: [[&'a str; WIDTH]; HEIGHT],
    pub rendered_tetrominoes_list: Vec<Tetromino>,
    pub unrendered_tetrominoes_list: Vec<Tetromino>,
    pub game_borders: [[bool; WIDTH]; HEIGHT + 1],
    pub built_tetrominoes: [[TetrominoCharacter; WIDTH]; HEIGHT],
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
    pub stdin: Bytes<AsyncReader>,
}

pub fn run(mut game_config: GameConfig) {
    let mut score: u32 = 0;
    let mut movement_counter = 1;

    display::display_screen(
        &game_config.screen,
        &mut game_config.unrendered_tetrominoes_list,
        &mut game_config.rendered_tetrominoes_list,
        &mut game_config.stdout,
        &game_config.built_tetrominoes,
        score,
    );

    loop {
        write!(game_config.stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = game_config.stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
        if let Some(Ok(b'a')) = b {
            tetromino::move_tetrmonioes(
                &mut game_config.unrendered_tetrominoes_list,
                -1,
                &mut game_config.game_borders,
            );

            display::display_screen(
                &game_config.screen,
                &mut game_config.unrendered_tetrominoes_list,
                &mut game_config.rendered_tetrominoes_list,
                &mut game_config.stdout,
                &game_config.built_tetrominoes,
                score,
            );
        }
        if let Some(Ok(b'd')) = b {
            tetromino::move_tetrmonioes(
                &mut game_config.unrendered_tetrominoes_list,
                1,
                &mut game_config.game_borders,
            );
            display::display_screen(
                &game_config.screen,
                &mut game_config.unrendered_tetrominoes_list,
                &mut game_config.rendered_tetrominoes_list,
                &mut game_config.stdout,
                &game_config.built_tetrominoes,
                score,
            );
        }
        if let Some(Ok(b'r')) = b {
            tetromino::rotate_tetrominoes(
                &mut game_config.unrendered_tetrominoes_list,
                90,
                &mut game_config.game_borders,
            );
            display::display_screen(
                &game_config.screen,
                &mut game_config.unrendered_tetrominoes_list,
                &mut game_config.rendered_tetrominoes_list,
                &mut game_config.stdout,
                &game_config.built_tetrominoes,
                score,
            );
        }

        thread::sleep(Duration::from_millis(100));

        let mut create: bool = true;

        for tetromino in &game_config.unrendered_tetrominoes_list {
            if !tetromino.stationary {
                create = false;
                break;
            }
        }

        if create {
            tetromino::create_tetronimo(&mut game_config.unrendered_tetrominoes_list);
        }

        if movement_counter % 2 == 0 {
            tetromino::update_tetrominoes(
                &mut game_config.unrendered_tetrominoes_list,
                &mut game_config.game_borders,
            );
        }
        movement_counter += 1;

        display::display_screen(
            &game_config.screen,
            &mut game_config.unrendered_tetrominoes_list,
            &mut game_config.rendered_tetrominoes_list,
            &mut game_config.stdout,
            &game_config.built_tetrominoes,
            score,
        );

        move_to_built(
            &mut game_config.unrendered_tetrominoes_list,
            &mut game_config.built_tetrominoes,
        );

        check_complete_line(&mut game_config.built_tetrominoes, &mut score);

        remake_gameborders(
            &mut game_config.game_borders,
            &mut game_config.built_tetrominoes,
        );

        game_config.stdout.flush().unwrap();
    }
}

fn move_to_built(
    tetrominoes_list: &mut Vec<Tetromino>,
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    let mut i = 0;
    while i < tetrominoes_list.len() {
        if tetrominoes_list[i].stationary {
            let tetromino = tetrominoes_list.remove(i);

            for character in tetromino.first_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.second_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.third_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
            for character in tetromino.fourth_line.characters {
                let x = character.x;
                let y = character.y;
                built_tetrominoes[y as usize][x as usize] = character;
            }
        } else {
            i += 1;
        }
    }
}

fn check_complete_line(
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
    score: &mut u32,
) -> bool {
    let mut complete_line = false;
    let mut lines_cleared = 0;

    for i in 0..HEIGHT {
        let mut line_is_complete = true;

        for j in 1..WIDTH - 1 {
            if built_tetrominoes[i][j].value.is_empty() {
                line_is_complete = false;
                break;
            }
        }

        if line_is_complete {
            complete_line = true;
            lines_cleared += 1;

            // Clear the current line
            for j in 0..WIDTH {
                built_tetrominoes[i][j] = TetrominoCharacter::default();
            }

            // Shift all rows above the current line down by one row
            for k in (1..=i).rev() {
                for j in 0..WIDTH {
                    built_tetrominoes[k][j] = built_tetrominoes[k - 1][j];
                    // Move character down by one row
                    built_tetrominoes[k][j].move_character(0, 1);
                }
            }

            // Clear the top row since it has been shifted down
            for j in 0..WIDTH {
                built_tetrominoes[0][j] = TetrominoCharacter::default();
            }
        }
    }

    // Update score based on the number of lines cleared
    *score += lines_cleared * 100;

    complete_line
}

fn remake_gameborders(
    game_borders: &mut [[bool; WIDTH]; HEIGHT + 1],
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if built_tetrominoes[i][j].value.is_empty() {
                game_borders[i][j] = false;
            } else {
                game_borders[i][j] = true;
            }
        }
    }
}
