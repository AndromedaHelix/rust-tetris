const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

use std::io::Bytes;

extern crate termion;
use std::io::Write;
use termion::AsyncReader;

use std::thread;
use std::time::Duration;

use std::sync::mpsc;

pub mod tetromino;

use crate::tetromino::characters::TetrominoCharacter;
use crate::tetromino::tetromino::Tetromino;

pub mod display;

pub struct GameConfig<'a> {
    pub screen: [[&'a str; WIDTH]; HEIGHT],
    pub current_tetromino: Tetromino,
    pub game_borders: [[bool; WIDTH]; HEIGHT + 1],
    pub built_tetrominoes: [[TetrominoCharacter; WIDTH]; HEIGHT],
    pub stdout: termion::raw::RawTerminal<std::io::Stdout>,
    pub stdin: Bytes<AsyncReader>,
}

pub fn run(mut game_config: GameConfig) {
    let mut score: u32 = 0;

    display::display_screen(
        &game_config.screen,
        &mut game_config.current_tetromino,
        &mut game_config.stdout,
        &mut game_config.built_tetrominoes,
        score,
    );

    let (tx, rx) = mpsc::channel();
    let mut movement_counter: i32 = 1;

    // Spanws a thread that handles updating movement_counters and sends
    // true when the tetromino's y position should go down by one following the game logic
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));

        if movement_counter % 2 == 0 {
            tx.send(true).unwrap();
        } else {
            tx.send(false).unwrap();
        }

        movement_counter += 1;
    });

    loop {
        write!(game_config.stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = game_config.stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }
        if let Some(Ok(b'a')) = b {
            game_config
                .current_tetromino
                .move_tetromino(-1, 0, &mut game_config.game_borders);

            display::display_screen(
                &game_config.screen,
                &mut game_config.current_tetromino,
                &mut game_config.stdout,
                &mut game_config.built_tetrominoes,
                score,
            );
        }
        if let Some(Ok(b'd')) = b {
            game_config
                .current_tetromino
                .move_tetromino(1, 0, &mut game_config.game_borders);

            display::display_screen(
                &game_config.screen,
                &mut game_config.current_tetromino,
                &mut game_config.stdout,
                &mut game_config.built_tetrominoes,
                score,
            );
        }
        if let Some(Ok(b'r')) = b {
            game_config
                .current_tetromino
                .rotate(90, &mut game_config.game_borders);

            display::display_screen(
                &game_config.screen,
                &mut game_config.current_tetromino,
                &mut game_config.stdout,
                &mut game_config.built_tetrominoes,
                score,
            );
        }
        if let Some(Ok(b's')) = b {
            game_config
                .current_tetromino
                .move_tetromino(0, 1, &mut game_config.game_borders);

            display::display_screen(
                &game_config.screen,
                &mut game_config.current_tetromino,
                &mut game_config.stdout,
                &mut game_config.built_tetrominoes,
                score,
            );
        }
        thread::sleep(Duration::from_millis(1));

        if game_config.current_tetromino.stationary {
            tetromino::create_tetronimo(&mut game_config.current_tetromino);
        }

        if rx.recv() == Ok(true) {
            game_config
                .current_tetromino
                .move_tetromino(0, 1, &mut game_config.game_borders);
        }

        display::display_screen(
            &game_config.screen,
            &mut game_config.current_tetromino,
            &mut game_config.stdout,
            &mut game_config.built_tetrominoes,
            score,
        );

        move_to_built(
            &mut game_config.current_tetromino,
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

pub fn move_to_built(
    tetromino: &mut Tetromino,
    built_tetrominoes: &mut [[TetrominoCharacter; WIDTH]; HEIGHT],
) {
    if tetromino.stationary {
        for character in &tetromino.first_line.characters {
            let x = character.x;
            let y = character.y;
            built_tetrominoes[y as usize][x as usize] = *character;
        }
        for character in &tetromino.second_line.characters {
            let x = character.x;
            let y = character.y;
            built_tetrominoes[y as usize][x as usize] = *character;
        }
        for character in &tetromino.third_line.characters {
            let x = character.x;
            let y = character.y;
            built_tetrominoes[y as usize][x as usize] = *character;
        }
        for character in &tetromino.fourth_line.characters {
            let x = character.x;
            let y = character.y;
            built_tetrominoes[y as usize][x as usize] = *character;
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
