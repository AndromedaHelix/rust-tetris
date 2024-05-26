/*

__          __   _ _   _               _                  _                     _____      _     _          _____       _   _
\ \        / /  (_) | | |             | |                | |                   |  __ \    | |   | |        / ____|     | | (_)
 \ \  /\  / / __ _| |_| |_ ___ _ __   | |__  _   _       | |_   _  __ _ _ __   | |__) |_ _| |__ | | ___   | |  __ _   _| |_ _  ___ _ __ _ __ ___ ____
  \ \/  \/ / '__| | __| __/ _ \ '_ \  | '_ \| | | |  _   | | | | |/ _` | '_ \  |  ___/ _` | '_ \| |/ _ \  | | |_ | | | | __| |/ _ \ '__| '__/ _ \_  /
   \  /\  /| |  | | |_| ||  __/ | | | | |_) | |_| | | |__| | |_| | (_| | | | | | |  | (_| | |_) | | (_) | | |__| | |_| | |_| |  __/ |  | | |  __// /
    \/  \/ |_|  |_|\__|\__\___|_| |_| |_.__/ \__, |  \____/ \__,_|\__,_|_| |_| |_|   \__,_|_.__/|_|\___/   \_____|\__,_|\__|_|\___|_|  |_|  \___/___|
                                              __/ |
                                             |___/
 */

use std::io::Read;

extern crate termion;
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear};

mod display;

use rust_tetris::tetromino;
use rust_tetris::tetromino::characters::TetrominoCharacter;
use rust_tetris::tetromino::tetromino::Tetromino;
use rust_tetris::{run, GameConfig};

const WIDTH: usize = 12; // 2 more to account for the borders
const HEIGHT: usize = 40;

/* Game loop */

fn main() {
    let mut screen: [[&str; WIDTH]; HEIGHT] = [[""; WIDTH]; HEIGHT];
    let rendered_tetrominoes_list: Vec<Tetromino> = Vec::new();
    let mut unrendered_tetrominoes_list: Vec<Tetromino> = Vec::new();

    let mut game_borders: [[bool; WIDTH]; HEIGHT + 1] = [[false; WIDTH]; HEIGHT + 1];
    for x in 0..WIDTH {
        game_borders[HEIGHT][x] = true;
    }

    // When the tetrominoes' stationary state is reached, they are added to the built_tetrominoes array
    // and removed from the unredered_tetrominoes_list
    //
    // The built_tetrominoes array is used to check for collisions with the tetrominoes that have already
    // been built. Also, the built_tetrominoes array is used to render the tetrominoes that have already
    // been built and to handle the scoring system.
    //
    // Also helps in reducing the number of computations per second needed to render moving tetrominoes as the unredered_tetrominoes_list reduces in size

    let built_tetrominoes: [[TetrominoCharacter; WIDTH]; HEIGHT] =
        [[TetrominoCharacter::default(); WIDTH]; HEIGHT];

    display::create_screen(&mut screen);
    tetromino::create_tetronimo(&mut unrendered_tetrominoes_list);

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = async_stdin().bytes();

    writeln!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();

    let game_config = GameConfig {
        screen,
        rendered_tetrominoes_list,
        unrendered_tetrominoes_list,
        game_borders,
        built_tetrominoes,
        stdout,
        stdin,
    };

    run(game_config);
}
